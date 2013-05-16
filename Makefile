DEBUG_CONFIGS=--cfg debug_shader --cfg no_check_gl
DEBUG_OPTIMIZATION=3 # TODO: Change for debug info

RELEASE_CONFIGS=--cfg release_shader --cfg no_check_gl
RELEASE_OPTIMIZATION=3


LIBS=-L lib/glfw/src -L lib/glfw-rs/lib -L lib/rust-opengles -L lib/stb-image -L /opt/local/lib

.SILENT:

all: setup debug

setup:
	mkdir -p bin

debug: clean
	rustc src/main.rs -o bin/q3 ${LIBS} ${DEBUG_CONFIGS} --opt-level ${DEBUG_OPTIMIZATION}
	echo "Finished building DEBUG Q3"

release: clean
	rustc src/main.rs -o bin/q3 ${LIBS} ${RELEASE_CONFIGS} --opt-level ${RELEASE_OPTIMIZATION} 
	echo "Finished building RELEASE Q3"

pretty:
	rustc src/main.rs -o bin/q3 ${LIBS} --pretty normal

clean:
	-rm -f bin/q3

