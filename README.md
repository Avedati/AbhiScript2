# AbhiScript2

This is the second version of my programming language, AbhiScript. The syntax of the language has changed, and the language is now compiled, rather than interpreted.

## Grammar

The grammar of AbhiScript2 is as follows:

	C_CODE = "c.code" STR ";"
	LIST = "[" [EXPR "," EXPR "," ... EXPR] "]"
	DICTIONARY = "{" [LITERAL ":" EXPR "," LITERAL ":" EXPR "," ... LITERAL ":" EXPR] "}"
	UNIT = NUM or STR or BOOL or LITERAL or LIST or DICTIONARY or FUNCTIONCALL
	ATOM = UNIT ("+" or "-") UNIT
	EXPR = ATOM ("*" or "/" or "%") ATOM
	STATEMENT = (FUNCTIONCALL | STRUCT | ASSIGNMENT | FUNCTION | C_CODE) ";"
	FUNCTIONCALL = LITERAL "(" [EXPR "," EXPR "," EXPR "," ... EXPR] ")" ";"
	STRUCT = "struct" LITERAL "{" [TYPE LITERAL ";" TYPE LITERAL ";" ... TYPE LITERAL] ";" "}" ";"
	ASSIGNMENT = TYPE LITERAL "=" EXPR
	FUNCTION = "fn" ":" TYPE LITERAL "(" [TYPE LITERAL "," TYPE LITERAL "," ... TYPE LITERAL] "=" "{" [STATEMENT STATEMENT ...] "}" ";"

## Example

Here is a hello world program written in AbhiScript2:

```
c.code "#include <stdio.h>"

fn:bool main() = {
	str message = "Hello, world!";
	printf("%s\n", message);
	c.code "return 0;";
}
```

## Getting Started
### Mac OS and Linux

To install AbhiScript2 on Mac OS or Linux, navigate to this directory in your terminal and type the following commands:
```bash
mkdir ~/bin
cp ./bin/* ~/bin
cd ~
echo "export PATH=$(pwd)/bin:$PATH" >> ~/.bashrc
echo "export PATH=$(pwd)/bin:$PATH" >> ~/.zshrc
```

### Windows

To install AbhiScript2 on Windows, you can use the commands above in the Windows Subsystem for Linux (WSL).
If you do not have WSL, you will need to build AbhiScript2 from source. Make sure that you have rust installed on your system, navigate to this directory on your system, and type the following commands into your command prompt: 

```bash
make
```

This will provide you with an AbhiScript2 compiler, which you can find in bin/abhiscriptc.
