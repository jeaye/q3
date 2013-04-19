.SILENT:

all: clean
	rustc src/main.rs -o bin/q3 -L lib 

pretty:
	rustc src/main.rs -o bin/q3 -L lib --pretty normal

clean:
	-rm -f bin/q3

