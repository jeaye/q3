Q³
===
![Screenshot](pics/012_1_no_wire.png)

### What is Q³?
Q^3 is a project using Mozilla's Rust language and OpenGL to create a Quake 3 like game that takes 
Quake 3 and QuakeLive maps, voxelizes them, and allows groups of players to blow the shit out 
of everything in a fast-paced Quake-esque first person shooter with 100% destructible environments.

### What's the current state of Q³?
Q³ is not a game yet! It's still a side project that I'm working on in my spare time.  
* Multithreaded OpenGL rendering
* Half-baked BSP renderer (Quake 3 and Quake Live)
  * Quake Live map rendering is... buggy
* TTF renderer
* Arbitrary mesh voxelizer (for BSP maps)
  * Using Separating Axis Theorem and instance rendering (it's not that fast)
* Basic UI with drop-down console that provides in-game tweaking/debugging
  * See [Console](https://github.com/Jeaye/q3/wiki/Console)
* Tested on Linux and Mac OS X
* [Documentation on a wiki](https://github.com/Jeaye/q3/wiki)

### How do I get Q³ running on my system?
I run on the (nearly) latest Rust master; I generally pull every few days. Q³ currently has 
[glfw3](https://github.com/glfw/glfw), 
[glfw-rs](https://github.com/Jeaye/glfw-rs), 
[rust-opengles](https://github.com/Jeaye/rust-opengles), and 
[rust-stb-image](https://github.com/mozilla-servo/rust-stb-image), 
as submodules. To configure, simply run (in source and out of source builds are acceptable):  
```bash
./configure
```
From there, you should be able to compile and run a release build with:  
```bash
make release && ./bin/q3
```
**NOTE:** Ensure that you have Freetype2 installed.  
**NOTE:** I don't have access to a Windows machine at the moment, so I'm not sure yet what will go into building 
this under something like MinGW. If you're interested in looking into it, please feel free to send me a pull
request!


### Who're you?
Hi! I'm Jeaye, a C++ software engineer.  
* **Email:**
  * jeaye (at) arrownext (dot) com  
* **LinkedIn:**
  * http://www.linkedin.com/in/jeaye  
* **IRC:**
  * #rust on irc.mozilla.org:6697 
  * #q3 on irc.freenode.net:6667 

