use std::env;

/*
	LIST = "[" [EXPR "," EXPR "," ... EXPR] "]"
	DICTIONARY = "{" [LITERAL ":" EXPR "," LITERAL ":" EXPR "," ... LITERAL ":" EXPR] "}"
	UNIT = NUM or STR or BOOL or LITERAL or LIST or DICTIONARY or FUNCTIONCALL
	ATOM = UNIT ("+" or "-") UNIT
	EXPR = ATOM ("*" or "/" or "%") ATOM
	STATEMENT = (FUNCTIONCALL | STRUCT | ASSIGNMENT | FUNCTION) ";"
	FUNCTIONCALL = LITERAL "(" [EXPR "," EXPR "," EXPR "," ... EXPR] ")" ";"
	STRUCT = "struct" LITERAL "{" [TYPE LITERAL ";" TYPE LITERAL ";" ... TYPE LITERAL] ";" "}" ";"
	ASSIGNMENT = TYPE LITERAL "=" EXPR
	FUNCTION = "fn" ":" TYPE LITERAL "(" [TYPE LITERAL "," TYPE LITERAL "," ... TYPE LITERAL] "=" "{" [STATEMENT STATEMENT ...] "}" ";"

-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

	fps(30);

	c.code "
		#include <stdio.h>
	"

	num x = 0;
	num y = 0;

	struct Rectangle {
		num x;
		num y;
		num w;
		num h;
		num r;
		num g;
		num b;
		num a;
	};

	list.str test = ['string1', 'string2', 'string3'];

	fn:num draw(num a, num b) = {
		object.Rectangle r = { x: 0, y: 0, w: 100, h: 100, r: 255, g: 255, b: 255, a: 255 };
		fill(r.x, r.y, r.w, r.h, r.r, r.g, r.b, r.a);
		flip();
	};
*/

#[derive(Clone)]
struct Token {
	line_number: i32,
	token_type: String,
	lexeme: String
}

struct Tokenizer {
	input: String,
	i: usize
}

struct Parser {
	tokens: Vec<Token>,
	i: usize
}

impl Tokenizer {
	fn tokenize(&mut self, keywords: Vec<String>, punctuation: Vec<char>, operators: Vec<char>) -> Vec<Token> {
		let mut tokens: Vec<Token> = Vec::new();
		let mut line_no: i32 = 1;
		while self.i < self.input.len() {
			// https://doc.rust-lang.org/std/string/struct.String.html
			if self.input.chars().nth(self.i).unwrap().is_alphabetic() || self.input.chars().nth(self.i).unwrap() == '_' {
				let mut lexeme: String = "".to_string();
				while self.i < self.input.len() && (self.input.chars().nth(self.i).unwrap().is_alphabetic() || self.input.chars().nth(self.i).unwrap() == '_' || self.input.chars().nth(self.i).unwrap() == '.') {
					let char_str = self.input.chars().nth(self.i).unwrap().to_string();
					lexeme.push_str(&char_str);
					self.i += 1;
				}
				// https://stackoverflow.com/questions/58368801/how-do-i-check-if-a-thing-is-in-a-vector
				if keywords.iter().any(|g| *g==lexeme) {
					tokens.push(Token{line_number: line_no, token_type: "keyword".to_string(), lexeme: lexeme.to_string()});
				}
				else if lexeme.len() >= 5 && lexeme.chars().take(5).collect::<String>() == "list." {
					tokens.push(Token{line_number: line_no, token_type: "keyword".to_string(), lexeme: lexeme.to_string()});
				}
				else if lexeme.len() >= 7 && lexeme.chars().take(7).collect::<String>() == "object." {
					tokens.push(Token{line_number: line_no, token_type: "keyword".to_string(), lexeme: lexeme.to_string()});
				}
				else if lexeme == "true" || lexeme == "false" {
					tokens.push(Token{line_number: line_no, token_type: "boolean".to_string(), lexeme: lexeme.to_string()});
				}
				else {
					tokens.push(Token{line_number: line_no, token_type: "identifier".to_string(), lexeme: lexeme.to_string()});
				}
			}
			else if self.input.chars().nth(self.i).unwrap() == '"' || self.input.chars().nth(self.i).unwrap() == '\'' {
				let str_start = self.input.chars().nth(self.i).unwrap();
				self.i += 1;
				let mut lexeme: String = "".to_string();
				while self.i < self.input.len() && (self.input.chars().nth(self.i).unwrap() != str_start) {
					if self.input.chars().nth(self.i).unwrap() == '\\' {
						let char_str = self.input.chars().nth(self.i).unwrap().to_string();
						lexeme.push_str(&char_str);
						self.i += 1;
					}
					let cs = self.input.chars().nth(self.i).unwrap().to_string();
					lexeme.push_str(&cs);
					self.i += 1;
				}
				self.i += 1;
				tokens.push(Token{line_number: line_no, token_type: "string".to_string(), lexeme: lexeme.to_string()});
			}
			else if self.input.chars().nth(self.i).unwrap().is_digit(10) || self.input.chars().nth(self.i).unwrap() == '.' || self.input.chars().nth(self.i).unwrap() == 'e' || self.input.chars().nth(self.i).unwrap() == 'E' {
				let mut lexeme: String = "".to_string();
				while self.i < self.input.len() && (self.input.chars().nth(self.i).unwrap().is_digit(10) || self.input.chars().nth(self.i).unwrap() == '.' || self.input.chars().nth(self.i).unwrap() == 'e' || self.input.chars().nth(self.i).unwrap() == 'E') {
					let char_str = self.input.chars().nth(self.i).unwrap().to_string();
					lexeme.push_str(&char_str);
					self.i += 1;
				}
				tokens.push(Token{line_number: line_no, token_type: "number".to_string(), lexeme: lexeme.to_string()});
			}
			else if punctuation.iter().any(|&g| g==self.input.chars().nth(self.i).unwrap()) {
				let lexeme: String = self.input.chars().nth(self.i).unwrap().to_string();
				self.i += 1;
				tokens.push(Token{line_number: line_no, token_type: "punctuation".to_string(), lexeme: lexeme.to_string()})
			}
			else if operators.iter().any(|&g| g==self.input.chars().nth(self.i).unwrap()) {
				let lexeme: String = self.input.chars().nth(self.i).unwrap().to_string();
				self.i += 1;
				tokens.push(Token{line_number: line_no, token_type: "operator".to_string(), lexeme: lexeme.to_string()});
			}
			else {
				if self.input.chars().nth(self.i).unwrap() == '\n' {
					line_no += 1;
				}
				self.i += 1;
			}
		}
		return tokens
	}
}

impl Parser {
	fn match_type(&mut self, token_type: &str) -> bool {
		if self.i >= self.tokens.len() {
			panic!("Error in match_type, reached EOF.");
		}
		self.tokens[self.i].token_type == token_type
	}

	fn match_token(&mut self, token_type: &str, lexeme: &str) -> bool {
		if self.i >= self.tokens.len() {
			panic!("Error in match_token, reached EOF.");
		}
		self.match_type(token_type) && self.tokens[self.i].lexeme == lexeme
	}

	fn expect_type(&mut self, token_type: &str) -> String {
		if self.match_type(token_type) {
			self.i += 1;
			return self.tokens[self.i - 1].lexeme.clone();
		}
		panic!("Error on line {}, expected type {}, got type {}", self.tokens[self.i].line_number, token_type, self.tokens[self.i].token_type);
	}

	fn expect_token(&mut self, token_type: &str, lexeme: &str) {
		if self.match_token(token_type, lexeme) {
			self.i += 1;	
		}
		else {
			panic!("Error on line {}, expected type `{}` and lexeme `{}`, got type `{}` and lexeme `{}`",
				self.tokens[self.i].line_number,
				token_type, lexeme,
				self.tokens[self.i].token_type, self.tokens[self.i].lexeme
			);
		}
	}

	fn convert_type(&mut self, lexeme: String) -> String {
		match &lexeme[..] {
			"num" => "double ".to_string(),
			"str" => "char* ".to_string(),
			"bool" => "int ".to_string(),
			_ => {
				if lexeme.len() > 5 && lexeme.chars().take(5).collect::<String>() == "list.".to_string() {
					let mut internal_type: String = self.convert_type(lexeme.chars().skip(5).take(lexeme.len() - 5).collect());
					internal_type.push_str("*");
					internal_type
				}
				else if lexeme.len() > 7 && lexeme.chars().take(7).collect::<String>() == "object.".to_string() {
					let mut converted: String = "struct ".to_string();
					converted.push_str(&(lexeme.chars().skip(7).take(lexeme.len() - 7).collect::<String>().clone()));
					converted.push_str(" ");
					converted
				}
				else {
					panic!("Error, could not convert type {}.", lexeme);
				}
			}
		}
	}

	fn parse_delimited(&mut self, start: Token, end: Token, start_result: String, end_result: String, element_function: impl Fn(&mut Parser) -> String, delimiter_function: impl Fn(&mut Parser) -> String) -> String {
		let mut result: String = "".to_string();
		self.expect_token(&(start.token_type), &(start.lexeme));
		result.push_str(&start_result);
		if self.match_token(&(end.token_type), &(end.lexeme)) {
			self.expect_token(&(end.token_type), &(end.lexeme));
			result.push_str(&end_result);
			return result;
		}
		else {
			result.push_str(&(element_function(self)))
		}
		while self.i < self.tokens.len() && !self.match_token(&(end.token_type), &(end.lexeme)) {
			result.push_str(&(delimiter_function(self)));
			result.push_str(&(element_function(self)));
		}
		self.expect_token(&(end.token_type), &(end.lexeme));
		result.push_str(&end_result);
		return result;
	}

	fn parse_unit(&mut self) -> String {
		if self.match_type("identifier") {
			if self.i + 1 < self.tokens.len() && self.tokens[self.i + 1].token_type == "punctuation" && self.tokens[self.i + 1].lexeme == "(" {
				self.parse_function_call()
			}
			else {
				self.expect_type("identifier")
			}
		}
		else if self.match_type("keyword") {
			let r1 = self.expect_type("keyword").clone();
			self.convert_type(r1)
		}
		else if self.match_type("number") { self.expect_type("number") }
		else if self.match_type("string") {
			let mut result: String = "\"".to_string();
			result.push_str(&(self.expect_type("string")));
			result.push_str("\"");
			result
		}
		else if self.match_type("boolean") {
			if self.match_token("boolean", "true") {
				"1".to_string()
			}
			else {
				"0".to_string()
			}
		}
		else if self.match_token("punctuation", "{") {
			self.parse_delimited(Token{line_number:0, token_type:"punctuation".to_string(), lexeme:"{".to_string()}, Token{line_number:0, token_type:"punctuation".to_string(), lexeme:"}".to_string()}, "{".to_string(), "}".to_string(),
				|parser: &mut Parser| -> String {
				let mut result: String = ".".to_string();
				result.push_str(&(parser.expect_type("identifier")));
				parser.expect_token("punctuation", ":");
				result.push_str(":");
				result.push_str(&(parser.parse_expr()));
				result
			},
				|parser: &mut Parser| -> String {
				parser.expect_token("punctuation", ",");
				",".to_string()
			})
		}
		else if self.match_token("punctuation", "[") {
			self.parse_delimited(Token{line_number:0, token_type:"punctuation".to_string(), lexeme:"[".to_string()}, Token{line_number:0, token_type:"punctuation".to_string(), lexeme:"]".to_string()}, "{".to_string(), "}".to_string(),
				|parser: &mut Parser| -> String {
				parser.parse_expr()
			},
				|parser: &mut Parser| -> String {
				parser.expect_token("punctuation", ",");
				",".to_string()
			})
		}
		else {
			panic!("Error on line {}, could not parse {} as unit.", self.tokens[self.i].line_number, self.tokens[self.i].lexeme)
		}
	}

	fn parse_atom(&mut self) -> String {
		let mut result: String = self.parse_unit();
		while self.match_token("operator", "+") || self.match_token("operator", "-") {
			result.push_str(&(self.expect_type("operator")));
			result.push_str(&(self.parse_unit()));
		}
		return result;
	}

	fn parse_expr(&mut self) -> String {
		let mut result: String = self.parse_atom();
		while self.match_token("operator", "*") || self.match_token("operator", "/") || self.match_token("operator", "%") {
			result.push_str(&(self.expect_type("operator")));
			result.push_str(&(self.parse_atom()));
		}
		return result;
	}

	fn parse_function(&mut self) -> String {
		self.expect_token("keyword", "fn");
		self.expect_token("punctuation", ":");
		let mut result: String = "".to_string();
		let r1 = self.expect_type("keyword").clone();
		let function_type: String = self.convert_type(r1);
		result.push_str(&function_type);
		let function_name: String = self.expect_type("identifier");
		result.push_str(&function_name);
		let args: String = self.parse_delimited(Token{line_number:0, token_type:"punctuation".to_string(), lexeme:"(".to_string()}, Token{line_number: 0, token_type:"punctuation".to_string(), lexeme:")".to_string()}, "(".to_string(), ")".to_string(),
			|parser: &mut Parser| -> String {
			let arg_type = parser.expect_type("keyword");
			let arg_name = parser.expect_type("identifier");
			let mut arg: String = parser.convert_type(arg_type);
			arg.push_str(&arg_name);
			return arg;
		},
			|parser: &mut Parser| -> String {
			parser.expect_token("punctuation", ",");
			return ",".to_string();
		});
		result.push_str(&args);
		self.expect_token("punctuation", "=");
		self.expect_token("punctuation", "{");
		result.push_str("{");
		while !self.match_token("punctuation", "}") {
			result.push_str(&(self.parse_statement()));
			result.push_str("\n");
		}
		self.expect_token("punctuation", "}");
		result.push_str("}");
		self.expect_token("punctuation", ";");
		return result;
	}

	fn parse_struct(&mut self) -> String {
		let mut result: String = "struct ".to_string();
		self.expect_token("keyword", "struct");
		result.push_str(&(self.expect_type("identifier")));
		self.expect_token("punctuation", "{");
		result.push_str("{");
		while !self.match_token("punctuation", "}") {
			let r1 = self.expect_type("keyword").clone();
			result.push_str(&(self.convert_type(r1)));
			let r2 = self.expect_type("identifier").clone();
			result.push_str(&r2);
			self.expect_token("punctuation", ";");
			result.push_str(";");
		}
		self.expect_token("punctuation", "}");
		result.push_str("}");
		self.expect_token("punctuation", ";");
		result.push_str(";");
		return result;
	}

	fn parse_assignment(&mut self) -> String {
		let mut result: String = "".to_string();
		let r1 = self.expect_type("keyword").clone();
		result.push_str(&(self.convert_type(r1)));
		result.push_str(&(self.expect_type("identifier")));
		self.expect_token("punctuation", "=");
		result.push_str("=");
		result.push_str(&(self.parse_expr()));
		self.expect_token("punctuation", ";");
		result.push_str(";");
		return result;
	}

	fn parse_function_call(&mut self) -> String {
		let mut result: String = "".to_string();
		result.push_str(&(self.expect_type("identifier")));
		result.push_str(&(self.parse_delimited(Token{line_number:0, token_type:"punctuation".to_string(),lexeme:"(".to_string()}, Token{line_number:0, token_type:"punctuation".to_string(),lexeme:")".to_string()}, "(".to_string(), ")".to_string(),
			|parser: &mut Parser| -> String {
				return parser.parse_expr();
			},
			|parser: &mut Parser| -> String {
				parser.expect_token("punctuation", ",");
				return ",".to_string();
			}
		)));
		self.expect_token("punctuation", ";");
		result.push_str(";");
		return result;
	}

	fn parse_c_code(&mut self) -> String {
		self.expect_token("identifier", "c.code");
		let code: String = self.expect_type("string");
		self.expect_token("punctuation", ";");
		return code.clone();
	}

	fn parse_statement(&mut self) -> String {
		if self.match_token("keyword", "fn") {
			return self.parse_function();
		}
		else if self.match_token("keyword", "struct") {
			return self.parse_struct();
		}
		else if self.match_type("keyword") {
			return self.parse_assignment();
		}
		else if self.match_type("identifier") {
			if self.match_token("identifier", "c.code") {
				return self.parse_c_code();
			}
			return self.parse_function_call();
		}
		panic!("Error on line {}, must parse function, struct, assignment, or identifier.", self.tokens[self.i].line_number);
	}

	fn parse(&mut self) -> String {
		let mut result: String = "".to_string();
		while self.i < self.tokens.len() {
			result.push_str(&(self.parse_statement()));
			result.push_str("\n");
		}
		return result;
	}
}

fn main() {
	let keywords: Vec<String> = vec!["struct".to_string(), "fn".to_string(), "num".to_string(), "str".to_string(), "bool".to_string()];
	let punctuation = vec![';', ',', '(', ')', '{', '}', ':', '=', '.','[',']'];
	let operators = vec!['+', '-', '*', '/', '%']; // TODO: implement functions for relational operators (less than, less than or equal to, equal to, greater than, etc.).
	if env::args().len() >= 2 {
		let filename: String = env::args().nth(1).unwrap();
		let contents: String = std::fs::read_to_string(&filename).unwrap();
		let mut tokenizer = Tokenizer{input: contents, i: 0};
		let tokens = tokenizer.tokenize(keywords, punctuation, operators);
		// TODO: implement this as a command-line argument.
		// for token in tokens { println!("Token: type: {}, lexeme: {}, line number: {}", token.token_type, token.lexeme, token.line_number); }
		let mut parser = Parser{tokens: tokens, i: 0};
		let result: String = parser.parse();
		println!("{}", result);
	}
	/*
	let mut tokenizer = Tokenizer{input: input, i: 0};
	let tokens = tokenizer.tokenize(keywords, punctuation, operators);
	// for token in tokens { println!("Token: type: {}, lexeme: {}, line number: {}", token.token_type, token.lexeme, token.line_number); }
	let mut parser = Parser{tokens: tokens, i: 0};
	let result: String = parser.parse();
	println!("{}", result);
	*/
}
