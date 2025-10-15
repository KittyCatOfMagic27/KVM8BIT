#!/bin/bash
# PLEASE LOAD RUNK BEFORE USE 'source ./run.sh'

echo "Loaded runk!"
function runk() {
    first_flag=$1
    first_flag=${first_flag:="--help"}
    file=""

    if [ $first_flag == "-fc" ]; then
        cd ./KCompilerRust
        if [ -n "${2}" ]; then 
            file="../$2"
        fi
        cargo run -- $file
        cd ../
        g++ -o kasmCompiler kasmCompiler.cpp
        g++ -o vm_run vm_run.cpp -lSDL2main -lSDL2
        ./kasmCompiler
        ./vm_run
    elif [ $first_flag == "-c" ]; then
        ./KCompilerRust/target/debug/KCompilerRust $2
        ./kasmCompiler
        ./vm_run
    elif [ $first_flag == "-co" ]; then
        cd ./KCompilerRust
        if [ -n "${2}" ]; then 
            file="../$2"
        fi
        cargo run -- $file
        cd ../
        g++ -o kasmCompiler kasmCompiler.cpp
        g++ -o vm_run vm_run.cpp -lSDL2main -lSDL2
        ./kasmCompiler
    elif [ $first_flag == "-r" ]; then
        ./vm_run
    elif [ $first_flag == "--help" ]; then
        echo "NOTE: RUNK ONLY SUPPORTS SIMPLE FLAGS"
        echo "INPUT FILE NAME AFTER FLAG. IF NONE IS PROVIDED, 'main.k' IS USED."
        echo "-fc -= full compile of compiler and program, runs after"
        echo "-c -= compiles program"
        echo "-r -= emulator runs ROM.bin"
    else 
        echo "type the --help flag for help"
    fi 
    
}