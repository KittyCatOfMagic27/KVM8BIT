cd ./KCompilerRust
cargo run
cd ../
g++ -o kasmCompiler kasmCompiler.cpp
g++ -o vm_run vm_run.cpp -lSDL2main -lSDL2
./kasmCompiler
./vm_run