DEBUG_CONFIGS=--cfg debug_shader --cfg no_check_gl
DEBUG_OPTIMIZATION=3 # Change for debug info

RELEASE_CONFIGS=--cfg release_shader --cfg no_check_gl
RELEASE_OPTIMIZATION=3

# Determine system
UNAME=$(shell uname)

# Colors
COLOR_OFF="\e[0m"
COLOR_RED="\e[0;31m"
COLOR_YELLOW="\e[0;33m"
COLOR_GREEN="\e[0;32m"

# Output colorizing
ECHO_PREFIX="${COLOR_RED}»»»${COLOR_OFF}"
ifeq ($(UNAME), Linux)
	ECHO=echo -e "${ECHO_PREFIX}"
else
	ECHO=echo "${ECHO_PREFIX}"
endif

# Version names of the build types we can attempt
VERSION_NAME_DEBUG="${COLOR_YELLOW}DEBUG${COLOR_OFF}"
VERSION_NAME_RELEASE="${COLOR_GREEN}RELEASE${COLOR_OFF}"

LIBS=-L lib/glfw/src -L lib/glfw-rs/lib -L lib/rust-opengles -L lib/stb-image -L /opt/local/lib

.SILENT:

all: setup debug

setup:
	mkdir -p bin

debug: clean
	${ECHO} "Building ${VERSION_NAME_DEBUG} Q^3 (this can take a while)"
	rustc src/main.rs -o bin/q3 ${LIBS} ${DEBUG_CONFIGS} --opt-level ${DEBUG_OPTIMIZATION}
	${ECHO} "Finished building ${VERSION_NAME_DEBUG} Q^3"
	echo

release: clean
	${ECHO} "Building ${VERSION_NAME_RELEASE} Q^3 (this can take a while)"
	rustc src/main.rs -o bin/q3 ${LIBS} ${RELEASE_CONFIGS} --opt-level ${RELEASE_OPTIMIZATION} 
	${ECHO} "Finished building ${VERSION_NAME_RELEASE} Q^3"
	echo

pretty:
	rustc src/main.rs -o bin/q3 ${LIBS} --pretty normal

clean:
	-rm -f bin/q3

