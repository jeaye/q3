DEBUG=--cfg debug_shader
RELEASE=--cfg release_shader

LIBS=-L lib/glfw-rs/lib -L lib/rust-opengles -L /opt/local/lib

.SILENT:

all: setup debug
	echo "Finished building Q3"

setup:
	mkdir -p bin

debug: clean
	rustc src/main.rs -o bin/q3 ${LIBS} ${DEBUG}

release: clean
	rustc src/main.rs -o bin/q3 ${LIBS} ${RELEASE}

pretty:
	rustc src/main.rs -o bin/q3 ${LIBS} --pretty normal

clean:
	-rm -f bin/q3

