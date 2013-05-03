DEBUG_CONFIGS=--cfg debug_shader
DEBUG_OPTIMIZATION=0

RELEASE_CONFIGS=--cfg release_shader
RELEASE_OPTIMIZATION=3


LIBS=-L lib/glfw-rs/lib -L lib/rust-opengles -L /opt/local/lib

.SILENT:

all: setup debug
	echo "Finished building Q3"

setup:
	mkdir -p bin

debug: clean
	rustc src/main.rs -o bin/q3 ${LIBS} ${DEBUG_CONFIGS} --opt-level ${DEBUG_OPTIMIZATION}

release: clean
	rustc src/main.rs -o bin/q3 ${LIBS} ${RELEASE_CONFIGS} --opt-level ${RELEASE_OPTIMIZATION} 

pretty:
	rustc src/main.rs -o bin/q3 ${LIBS} --pretty normal

clean:
	-rm -f bin/q3

