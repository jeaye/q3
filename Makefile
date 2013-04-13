.SILENT:

all: clean
	rustc src/main.rs -o bin/q3 -L lib 

clean:
	-rm -f bin/q3

