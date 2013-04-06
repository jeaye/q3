.SILENT:

all: clean
	rustc src/main.rs -o bin/a.out -L lib -L lib/libglfw -Z debug-info 

clean:
	-rm -f bin/.out

