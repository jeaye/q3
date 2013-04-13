#.SILENT:

all: clean
	rustc src/main.rs -o bin/q3 -L lib 

# Performs a submodule update and rebuilds/installs submodules
update:
	git submodule update --recursive --init
	#
	# GLFW-3
	cd lib/glfw-rs
	make clean
	git pull
	make && cp lib/*.so ../
	cd ../../
	#
	# OpenGL ES
	cd lib/rust-opengles
	make clean
	git pull origin master # Pull from my fork
	git pull git://github.com/mozilla-servo/rust-opengles.git # Pull from parent
	make && cp *.so ../
	cd ../../
	
clean:
	-rm -f bin/q3

