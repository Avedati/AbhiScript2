RUSTC=rustc
TARGET=src/main.rs
OUT=bin/abhiscript_inner

all:
	$(RUSTC) $(TARGET) -o $(OUT)
