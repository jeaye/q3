DEBUG=--cfg debug_shader
RELEASE=--cfg release_shader

.SILENT:

all: clean
	rustc src/main.rs -o bin/q3 -L lib ${DEBUG}

release: clean
	rustc src/main.rs -o bin/q3 -L lib ${RELEASE}

pretty:
	rustc src/main.rs -o bin/q3 -L lib --pretty normal

clean:
	-rm -f bin/q3

