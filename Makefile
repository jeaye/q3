DEBUG_CONFIGS=--cfg debug_shader --cfg check_gl
DEBUG_OPTIMIZATION=0 # Change for debug info

RELEASE_CONFIGS=--cfg release_shader
RELEASE_OPTIMIZATION=3

# Determine system
UNAME=$(shell uname)

# Get git HEAD
export COMMIT=$(shell git rev-parse --short HEAD)
export VERSION=0.1

# Colors
COLOR_OFF=$(shell tput sgr0)
COLOR_RED=$(shell tput setaf 1)
COLOR_YELLOW=$(shell tput setaf 3)
COLOR_GREEN=$(shell tput setaf 2)

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
	${ECHO} "Building ${VERSION_NAME_DEBUG} Q^3 [commit: ${COMMIT}](this can take a while)"
	rustc src/main.rs -o bin/q3 ${LIBS} ${DEBUG_CONFIGS} --opt-level ${DEBUG_OPTIMIZATION} 2>&1 | sed 's/^/\t/'
	${ECHO} "Finished building ${VERSION_NAME_DEBUG} Q^3"
	echo

release: clean
	${ECHO} "Building ${VERSION_NAME_RELEASE} Q^3 [commit: ${COMMIT}](this can take a while)"
	rustc src/main.rs -o bin/q3 ${LIBS} ${RELEASE_CONFIGS} --opt-level ${RELEASE_OPTIMIZATION} 2>&1 | sed 's/^/\t/'
	${ECHO} "Finished building ${VERSION_NAME_RELEASE} Q^3"
	echo

pretty:
	rustc src/main.rs -o bin/q3 ${LIBS} --pretty normal

clean:
	-rm -f bin/q3

