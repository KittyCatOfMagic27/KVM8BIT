# KVM8BIT
UTILIZES THE FOLLOWING: SDL2, Cargo, g++
KVM8BIT is a project to create an 8 bit enumlator inspired by the MOS 6502 processor! The end goal is to have a fully functioning arcade style game to be written for it! The whole project is very work in progress at the moment.
The project has 3 stages: .K, KASM, and the emulator itself!.
You can see where these are in 'run.sh'.

## How to Run ##
`usr@penguin:~/.../KVM_8BIT$ bash run.sh`
This runs ALL stages from .K to emulator, building the .K compiler as well. Change it... seriously please, this is for debug.
The .K code is read from 'main.k.'

## Stages ##
The project has 3 stages: .K, KASM, and the emulator itself!.
The .K stage takes the given .K code and compiles it into KASM.
KASM is then directly translated into bytecode, or a ROM, in 'ROM.bin' that the emulator reads.
The emulator then iterates over the program and opens a window, only updating the window when the system call is given.
The emulator then returns whatever is left in the A register at the end of the program.

## KASM Syntax ##
KASM has a very simple syntax and very small compiler as a result. All operations are in the file 'documentation/opcodes.txt.'
The basic structure of a KASM program is as such:
```
LABEL __MAIN__
...
LDA 0;
BRK;
```
Some tips! ALWAYS put the \_\_MAIN\_\_ label at the end of a program, as the compiler is only a single pass (nothing can be used before it is defined).

## .K Syntax ##
.K is very much similar to a much more explicit C99 with limited functionality. .K is nowhere near finalized yet, so the desired syntax is currently outlined in 'ExamplesK/syntax.k'.