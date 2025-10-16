#include <iostream>
#include <fstream>
#include <filesystem>

#define RAM_SIZE 2000
#define ROM_FILE_NAME "ROM.bin"

#include "VM_CPU.hpp"

static K_CPU CPU;

void programLoop(){
    SDL_Event event;
    std::vector<SDL_Event> events;
    while (SDL_PollEvent(&event)) {  // poll until all events are handled!
       switch (event.type) {
            case SDL_KEYDOWN:
                switch (event.key.keysym.sym) {
                    // case SDLK_m:                 // enter menu state
                    // case SDLK_ESCAPE:
                    //     machine->popState();
                    //     break;
                    // default:
                    //     break;
                }
                break;
            case SDL_WINDOWEVENT:
                switch (event.window.event) {
                    case SDL_WINDOWEVENT_CLOSE:   // exit game
                        CPU.windowClosed();
                        return;
                        break;
                    default:
                        break;
                }
                break;
            default:
                // logFileStderr("TutorialState unknown event...\n");
                break;
        }
        events.push_back(event);
    }
    K_CPU::ReturnPackage ret = CPU.executeProgramTick(&events);
    if(ret.programEnd){
        CPU.end();
    }
    CPU.PPU.window.next_tick();
}

#ifdef __PRINT_FPS__
void fpsCount() 
{
    std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    std::cout << "\033[H";
    std::cout << "\033[J";
    std::cout << "FPS: " << CPU.windowUpdateCount-CPU.lastWindowUpdateCount << std::endl;
    CPU.lastWindowUpdateCount = CPU.windowUpdateCount;
}
#endif

int main(int argc, char** args){
    //Fetch ROM
    std::fstream ROM_FILEFS;

    ROM_FILEFS.open(ROM_FILE_NAME, std::ios_base::in|std::ios_base::binary);
    size_t ROM_SIZE = std::filesystem::file_size(ROM_FILE_NAME);
    uint8_t* ROM_BIN = (uint8_t*)malloc(ROM_SIZE+1);
    ROM_FILEFS.read((char*)ROM_BIN, ROM_SIZE);
    ROM_FILEFS.close();
    ROM_BIN[ROM_SIZE] = 0;

    //INIT CPU
    CPU.init(RAM_SIZE, (uint8_t*)ROM_BIN);

    #ifdef __PRINT_FPS__
    auto exec_fpsCount = [](){ while (true) fpsCount(); };
    std::thread fpst(exec_fpsCount);
    fpst.detach();
    #endif

    CPU.start(programLoop);

    return 0;
}