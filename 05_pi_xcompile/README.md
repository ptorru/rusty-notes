# Cross-compiling rust for Raspberry Pi (running linux)
In this entry I will leave some notes behind on x-compiling Rust for Raspberry Pi.

What this means is using your laptop/pc to compile your project, this will then generate a binary file that we can push to the Raspberry Pi and ultimately run it there.

## First time set-up
I have followed the instructions in the (Chacin.dev website)[https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/]. I decided to manually download and locally install the GCC-10 Arm tool chain. However (Hackermoon)[https://hackernoon.com/compiling-rust-for-the-raspberry-pi-49fdcd7df658] reminds us that this is also available over apt. You may want to change the version in the command, as of today we are in version 10.3, they have 4.7 on the example command.

## Creating the project

I kept on following the instructions from Chacin, however had to tweak their config.toml file as the linker string was not quite right:

```bash
# Original:
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"

# Had to tweak to:
linker = "arm-none-linux-gnueabihf-gcc"
```

## Compiling
When compiling one has to use cargo's *--target* option. I created a makefile here just for the sake of having a quick shortcut for this; also to have a shortcut for the command to push the code into the Pi.

You will see in my Makefile I don't use sshpass or anything because I already have configured ssh-keys with my Pi.