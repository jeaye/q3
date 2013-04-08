.SILENT:

all: clean
	rustc src/main.rs -o bin/q3 -L lib -L lib/libglfw -Z debug-info 

clean:
	-rm -f bin/q3

