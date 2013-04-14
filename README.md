Q^3
===
### What is Q^3?
Q^3 is a very new project using Mozilla's Rust language and OpenGL 3.3 to create a Quake 3 like game.
The goal of this project is not to be Quake 3, but to be its own game. Currently testing the tech available
to me, I'm still assessing what's going to become of Q^3. However, my goal is to take Quake 3 and QuakeLive
maps, voxelize them, allow groups of players to join (for free), and blow the shit out of everything in a fast-paced
Quake-esque first person shooter. 

### What's the current state of Q^3?
Q^3 is not a game yet! It's still a hackjob affair that I fondle every day. As of writing, it's merely a half-baked
BSP renderer and half of a half-baked TTF renderer. 

### How do I get Q^3 running on my system?
Garwsh, I don't know! I run on the (nearly) latest Rust incoming; I generally pull every few days. Q^3 currently has glfw3, rust-opengles, and glfw-rs as submodules. To configure, simply run:  
```bash
./configure
```
From there, you should be able to compile and run with:  
```bash
make && ./bin/q3
```
**NOTE:** Ensure that you have Freetype2 installed.  
**NOTE:** I don't have access to a Windows machine at the moment, so I'm not sure yet what will go into building 
this in something like Visual Studio. If you're interested in looking into it, please feel free to send me a pull
request!


### Who're you?
Hi! I'm Jeaye, a professional C++ game developer.  
**Email:** jeaye (at) arrownext (dot) com  
**LinkedIn:** http://www.linkedin.com/in/jeaye  
**IRC:** #rust on irc.mozilla.org:6697 

![Screenshot](pics/7_camera.png)

