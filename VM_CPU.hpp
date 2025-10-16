#pragma once

#include "./libs/KGraphics.hpp"

#include <thread>
#include <chrono>

#define PAGE_SIZE 256
#define IO_CONSOLE_OUT_ADDRESS 0xFFFF
#define IO_CONSOLE_BUFFERED_OUT_ADDRESS 0xFFFE

// #define __DEBUG__
// #define __DEBUG_GRAPHICS__
// #define __XRAY_STACK__
// #define __PRINT_FPS__

#include "./tables/OPcodes.hpp"

static char CURRENT_INSTRUCTION = 0;

void printOp(uint8_t op){
    for (auto& p: OPCodetable) {
        uint16_t x = p.second;
        if(((uint8_t*)&x)[0]==op||((uint8_t*)&x)[1]==op){
            std::cout << p.first;
            return;
        }
    }
}

#include "./components/VM_RAM.hpp"
#include "./components/VM_PPU.hpp"

class K_CPU{
    public:
    //RETURN TICK PKG
    struct ReturnPackage{
        bool programEnd = false;
        int returnValue = -1;
    };
    //DEBUG
    #ifdef __PRINT_FPS__
    int windowUpdateCount = 0;
    int lastWindowUpdateCount = 0;
    #endif
    //BUSSES
    uint8_t A = 0;
    uint8_t X = 0;
    uint8_t Y = 0;
    uint8_t P = 0;
    uint8_t S = 0xFF;
    uint16_t PC = 0;
    uint8_t ABH = 0;
    uint8_t ABL = 0; //ADDRESS BUSSES (HIGH PAGE, LOW ADDR)
    //INFO
    size_t ramSize;
    //COMPONENTS
    K_RAM RAM;
    uint8_t* ROM;
    //GRAPHICS
    K_PPU PPU;
    //MISC.
    uint16_t endingProc = 0xFFFF;
    uint16_t windowProc = 0xFFFF;

    K_CPU(){}
    K_CPU(size_t _ramSize, uint8_t* _ROM):ramSize(_ramSize), ROM(_ROM){
        secureInit();
    }

    void secureInit(){
        RAM.INIT_RAM(ramSize);
    }

    void init(size_t _ramSize, uint8_t* _ROM){
        ramSize=_ramSize;
        ROM=_ROM;
        RAM.INIT_RAM(ramSize);
    }

    void start(void (*displayLoop)(void)){
        PPU.window.init({});
        PPU.window.provideDisplayLoop(displayLoop);
        PPU.window.start();
    }

    void end(){
        PC = endingProc;
        while(endingProc!=0xFFFF){
            ReturnPackage ret = executeProgramTick(nullptr);
            if(ret.programEnd)break;
        }
        PPU.window.end();
    }

    void windowClosed(){
        PC = windowProc;
        while(windowProc!=0xFFFF){
            ReturnPackage ret = executeProgramTick(nullptr);
            if(ret.programEnd)break;
        }
        PC = endingProc;
        while(endingProc!=0xFFFF){
            ReturnPackage ret = executeProgramTick(nullptr);
            if(ret.programEnd)break;
        }
        PPU.window.end();
    }

    ReturnPackage executeProgramTick(std::vector<SDL_Event> *events){
        #define GET_CHAR (*(ROM+PC))
        #define GET_NEXT_CHARI (*(ROM+(++PC)))
        #define GET_LAST_CHAR (*(ROM+(PC-1)))

        std::string instructionHistory = "";
        while(true){
            #ifdef __XRAY_STACK__
            CURRENT_INSTRUCTION = GET_CHAR;
            #endif
            instructionHistory+=(char)GET_CHAR;
            switch(GET_CHAR){
                //------------SYSTEM------------
                //SYS
                case 0xE2:{
                    switch(GET_NEXT_CHARI){
                        //DUMP
                        case 0x01: std::cout << RAM.OUT_BUFFER; RAM.OUT_BUFFER = "";
                        break;
                        //WAIT 
                        case 0x02:{
                            int x = *(int*)RAM.OUT_BUFFER.c_str();
                            RAM.OUT_BUFFER = "";
                            std::this_thread::sleep_for(std::chrono::milliseconds(x));
                        }
                        break;
                        //UPDATE WINDOW & POLL EVENTS
                        case 0x07:{
                            #ifdef __PRINT_FPS__
                            windowUpdateCount++;
                            #endif
                            PC++;
                            return {};
                        }
                        break;
                        //GRAPHICS FUNCTION
                        case 0x08:{
                            #ifdef __DEBUG_GRAPHICS__
                            std::cout << "INPUTS TO GRAPHICS CALL: ";
                            for(int i = 0; i < RAM.OUT_BUFFER.size(); i++){
                                std::cout << (int)(uint8_t)RAM.OUT_BUFFER[i] << " ";
                            }
                            std::cout << std::endl;
                            #endif 
                            switch(RAM.OUT_BUFFER[0]){
                                //CHANGE BACKGROUND COLOR
                                case 0x01:{
                                    #ifdef __DEBUG_GRAPHICS__
                                    std::cout << "CHANGING BACKGROUND COLOR" << std::endl;
                                    #endif 
                                    PPU.window.colorBackground(RAM.OUT_BUFFER[1],RAM.OUT_BUFFER[2],RAM.OUT_BUFFER[3]);
                                }
                                break;
                                //DRAW TILE RGB
                                case 0x02:{
                                    #ifdef __DEBUG_GRAPHICS__
                                    std::cout << "DRAWING RGB TILE" << std::endl;
                                    #endif 
                                    PPU.drawRectTile((uint8_t)(RAM.OUT_BUFFER[1]), (uint8_t)(RAM.OUT_BUFFER[2]), {(uint8_t)RAM.OUT_BUFFER[3], (uint8_t)RAM.OUT_BUFFER[4], (uint8_t)RAM.OUT_BUFFER[5]});
                                }
                                break;
                                //DRAW PIXEL RBG
                                case 0x03:{
                                    #ifdef __DEBUG_GRAPHICS__
                                    std::cout << "DRAWING RGB PIXEL" << std::endl;
                                    #endif 
                                    PPU.drawPixel((uint8_t)(RAM.OUT_BUFFER[1]), (uint8_t)(RAM.OUT_BUFFER[2]), {(uint8_t)RAM.OUT_BUFFER[3], (uint8_t)RAM.OUT_BUFFER[4], (uint8_t)RAM.OUT_BUFFER[5]});
                                }
                                break;
                                //LOAD TEXTURE
                                case 0x04:{
                                    #ifdef __DEBUG_GRAPHICS__
                                    std::cout << "LOADING TEXTURE" << std::endl;
                                    #endif 
                                    uint16_t address = (uint8_t)RAM.OUT_BUFFER[1];
                                    address <<= 8;
                                    address += (uint8_t)RAM.OUT_BUFFER[2];
                                    std::vector<uint8_t> data;
                                    //use mode to determine size
                                    switch((ROM+address)[3]){
                                        //2 bit color
                                        case 1:
                                        for(int i = 0; i < ((ROM+address)[0]*(ROM+address)[1])/4; i++){
                                            data.push_back((ROM+address)[4+i]>>6);
                                            data.push_back(((ROM+address)[4+i]&0b00110000)>>4);
                                            data.push_back(((ROM+address)[4+i]&0b00001100)>>2);
                                            data.push_back(((ROM+address)[4+i]&0b00000011));
                                        }
                                        break;
                                        default:
                                        std::cerr << "Invalid Texture Color Mode: " << (int)((ROM+address)[3]) << std::endl;
                                        exit(-1);
                                    }
                                    PPU.loadTexture((ROM+address)[0], (ROM+address)[1], (ROM+address)[2], (ROM+address)[3], data);
                                }
                                break;
                                //DRAW TEXTURE
                                case 0x05:{
                                    #ifdef __DEBUG_GRAPHICS__
                                    std::cout << "DRAWING TEXTURE" << std::endl;
                                    #endif 
                                    PPU.drawTexture(RAM.OUT_BUFFER[1], RAM.OUT_BUFFER[2], RAM.OUT_BUFFER[3], RAM.OUT_BUFFER[4]);
                                }
                                break;
                                //LOAD PALLET
                                case 0x06:{
                                    #ifdef __DEBUG_GRAPHICS__
                                    std::cout << "LOADING PALLETE" << std::endl;
                                    #endif 
                                    uint16_t address = (uint8_t)RAM.OUT_BUFFER[1];
                                    address <<= 8;
                                    address += (uint8_t)RAM.OUT_BUFFER[2];
                                    int count = ROM[address];
                                    std::vector<uint8_t> data;
                                    for(int i = 1; i < (3*count)+1; i++){
                                        data.push_back(ROM[address+i]);
                                    }
                                    A = PPU.loadPalette(count, data);
                                }
                                break;
                                default:
                                std::cerr << "Invalid Graphics Call: ";
                                printf ("0x%02x", RAM.OUT_BUFFER[0]);
                                std::cerr << std::endl;
                                break;
                            }
                            RAM.OUT_BUFFER = "";
                        }
                        break;
                        //DECLARE ENDING PROC
                        case 0x09:{
                            uint16_t address = (uint8_t)RAM.OUT_BUFFER[0];
                            address <<= 8;
                            address += (uint8_t)RAM.OUT_BUFFER[1];
                            endingProc = address;
                            RAM.OUT_BUFFER = "";
                        }
                        break;
                        //DECLARE WINDOW PROC
                        case 0x0D:{
                            uint16_t address = (uint8_t)RAM.OUT_BUFFER[0];
                            address <<= 8;
                            address += (uint8_t)RAM.OUT_BUFFER[1];
                            windowProc = address;
                            RAM.OUT_BUFFER = "";
                        }
                        break;
                        //WRITE ROM ADDRESS
                        case 0x0A:{
                            uint16_t address = (uint8_t)RAM.OUT_BUFFER[0];
                            address <<= 8;
                            address += (uint8_t)RAM.OUT_BUFFER[1];
                            while(ROM[address]!=0) std::cout << ROM[address++];
                            RAM.OUT_BUFFER = "";
                        }
                        break;
                        //WRITE STACK ADDRESS
                        case 0x0C:{
                            uint16_t address = (uint8_t)RAM.OUT_BUFFER[0];
                            address <<= 8;
                            address += S+(uint8_t)RAM.OUT_BUFFER[1];
                            while(*RAM.getRAddress(address)!=0) std::cout << *RAM.getRAddress(address--);
                            RAM.OUT_BUFFER = "";
                        }
                        break;
                        //IS KEY PRESSED
                        case 0x0B:{
                            char key = RAM.OUT_BUFFER[0];
                            if(key>='a'&&key<='z') key-=93;
                            else if(key>='A'&&key<='Z') key-=61;
                            bool match = false;
                            for(int i = 0; i < events->size(); i++){
                                if((*events)[i].type == SDL_KEYDOWN){
                                    if((*events)[i].key.keysym.scancode==key){match = true; break;}
                                }
                            }
                            if(match) A=1;
                            else A=0;
                            RAM.OUT_BUFFER = "";
                        }
                        break;
                        default:
                        std::cerr << "Invalid System Call: ";
                        printf ("0x%02x", GET_CHAR);
                        std::cerr << std::endl;
                        break;
                    }
                }
                break;

                //------------STACK ALLOCS------------
                //SAL
                case 0x1A:{S -= GET_NEXT_CHARI;} break;
                //DAL
                case 0x3A:{S += GET_NEXT_CHARI;} break;

                //------------STORES------------
                //STRC
                case 0x89:
                {
                    uint8_t pageDest = GET_NEXT_CHARI;
                    uint8_t addrDest = GET_NEXT_CHARI;
                    uint8_t byte1 = GET_NEXT_CHARI;
                    uint8_t byte2 = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(pageDest, addrDest);
                    #ifdef __DEBUG__
                    std::cout << "Stored Literal "<< (int)byte1 << ":" << (int)byte2 <<" to:" << (int)pageDest << ":" << (int)addrDest << std::endl;
                    #endif
                    uint16_t rel = ptr-RAM.MEMORY;
                    if(rel==IO_CONSOLE_OUT_ADDRESS){std::cout << byte1 << byte2;}
                    if(rel==IO_CONSOLE_BUFFERED_OUT_ADDRESS){RAM.OUT_BUFFER.push_back((char)byte1); RAM.OUT_BUFFER.push_back((char)byte2);}
                    else RAM.assignToAddress(ptr, Y);
                }
                break;
                //STCS
                case 0xC2:
                {
                    uint8_t pageDest = 0x01;
                    uint8_t addrDest = S+GET_NEXT_CHARI;
                    uint8_t byte1 = GET_NEXT_CHARI;
                    uint8_t byte2 = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(pageDest, addrDest);
                    #ifdef __DEBUG__
                    std::cout << "Stored Literal "<< (int)byte1 << ":" << (int)byte2 <<" at stack pos:" << (int)addrDest << std::endl;
                    #endif
                    uint16_t rel = ptr-RAM.MEMORY;
                    if(rel==IO_CONSOLE_OUT_ADDRESS){std::cout << byte1 << byte2;}
                    if(rel==IO_CONSOLE_BUFFERED_OUT_ADDRESS){RAM.OUT_BUFFER.push_back((char)byte1); RAM.OUT_BUFFER.push_back((char)byte2);}
                    else RAM.assignToAddress(ptr, Y);
                }
                break;
                //STY 
                case 0x80: 
                {
                    PC++;
                    uint8_t* ptr = RAM.getRAddress(ABH, GET_CHAR);
                    #ifdef __DEBUG__
                    std::cout << "Stored Y to:" << (int)GET_CHAR << std::endl;
                    #endif
                    RAM.assignToAddress(ptr, Y);
                }
                break;
                case 0x8C: 
                {
                    uint8_t page = GET_NEXT_CHARI;
                    uint8_t addr = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    #ifdef __DEBUG__
                    std::cout << "Stored Y to:" << (int)GET_LAST_CHAR << ":" << (int)GET_CHAR << std::endl;
                    #endif
                    RAM.assignToAddress(ptr, Y);
                }
                break;
                //STYS
                case 0xFC: 
                {   
                    uint8_t page = 0x01;
                    uint8_t addr = S+GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    #ifdef __DEBUG__
                    std::cout << "Stored Y to stack position:" << (int)addr << std::endl;
                    #endif
                    RAM.assignToAddress(ptr, Y);
                }
                break;
                //STA 
                case 0x81:
                {
                    PC++;
                    uint8_t* ptr = RAM.getRAddress(ABH, GET_CHAR);
                    #ifdef __DEBUG__
                    std::cout << "Stored A to:" << (int)GET_CHAR << std::endl;
                    #endif
                    RAM.assignToAddress(ptr, A);
                }
                break;
                case 0x8D:
                {
                    uint8_t page = GET_NEXT_CHARI;
                    uint8_t addr = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    #ifdef __DEBUG__
                    std::cout << "Stored A to:" << (int)GET_LAST_CHAR << ":" << (int)GET_CHAR << std::endl;
                    #endif
                    RAM.assignToAddress(ptr, A);
                }
                break;
                //STAS
                case 0x1C: 
                {   
                    uint8_t page = 0x01;
                    uint8_t addr = S+GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    #ifdef __DEBUG__
                    std::cout << "Stored A to stack position:" << (int)addr << std::endl;
                    #endif
                    RAM.assignToAddress(ptr, A);
                }
                break;
                //STX 
                case 0x82: 
                {
                    PC++;
                    uint8_t* ptr = RAM.getRAddress(ABH, GET_CHAR);
                    #ifdef __DEBUG__
                    std::cout << "Stored X to:" << (int)GET_CHAR << std::endl;
                    #endif
                    RAM.assignToAddress(ptr, X);
                }
                break;
                case 0x8E:
                {
                    uint8_t page = GET_NEXT_CHARI;
                    uint8_t addr = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    #ifdef __DEBUG__
                    std::cout << "Stored X to:" << (int)GET_LAST_CHAR << ":" << (int)GET_CHAR << std::endl;
                    #endif
                    RAM.assignToAddress(ptr, X);
                }
                break;
                //STXS
                case 0x3C: 
                {   
                    uint8_t page = 0x01;
                    uint8_t addr = S+GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    #ifdef __DEBUG__
                    std::cout << "Stored X to stack position:" << (int)addr << std::endl;
                    #endif
                    RAM.assignToAddress(ptr, X);
                }
                break;

                //------------LOADS------------
                //LDY
                case 0xB4:
                {
                    PC++;
                    uint8_t* ptr = RAM.getRAddress(ABH, GET_CHAR);
                    #ifdef __DEBUG__
                    std::cout << "Loaded to Y:" << (int)*ptr << std::endl;
                    #endif
                    Y = *ptr;
                }
                break;
                case 0xAC:
                {
                    uint8_t page = GET_NEXT_CHARI;
                    uint8_t addr = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    #ifdef __DEBUG__
                    std::cout << "Loaded to Y:" << (int)GET_LAST_CHAR << ":" << (int)GET_CHAR << std::endl;
                    #endif
                    Y = *ptr;
                }
                break;
                case 0xA0:
                {
                    Y = GET_NEXT_CHARI;
                    #ifdef __DEBUG__
                    std::cout << "Loaded to Y: (literal) " << (int)GET_CHAR << std::endl;
                    #endif
                }
                break;
                //LDYS
                case 0x5C:
                {
                    uint8_t page = 0x01;
                    uint8_t addr = S+GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    #ifdef __DEBUG__
                    std::cout << "Loaded to Y: byte at stack index:" << (int)addr << std::endl;
                    #endif
                    Y = *ptr;
                }
                break;
                //LDA
                case 0xA1:
                {
                    PC++;
                    uint8_t* ptr = RAM.getRAddress(ABH, GET_CHAR);
                    #ifdef __DEBUG__
                    std::cout << "Loaded to A:" << (int)*ptr << std::endl;
                    #endif
                    A = *ptr;
                }
                break;
                case 0xAD:
                {
                    uint8_t page = GET_NEXT_CHARI;
                    uint8_t addr = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    #ifdef __DEBUG__
                    std::cout << "Loaded to A:" << (int)GET_LAST_CHAR << ":" << (int)GET_CHAR << std::endl;
                    #endif
                    A = *ptr;
                }
                break;
                case 0xA9:
                {
                    A = GET_NEXT_CHARI;
                    #ifdef __DEBUG__
                    std::cout << "Loaded to A: (literal) " << (int)GET_CHAR << std::endl;
                    #endif
                }
                break;
                //LDAS
                case 0x7C:
                {
                    uint8_t page = 0x01;
                    uint8_t addr = S+GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    #ifdef __DEBUG__
                    std::cout << "Loaded to A: byte at stack index:" << (int)addr << std::endl;
                    #endif
                    A = *ptr;
                }
                break;
                //LDX
                case 0xA2:
                {
                    PC++;
                    uint8_t* ptr = RAM.getRAddress(ABH, GET_CHAR);
                    #ifdef __DEBUG__
                    std::cout << "Loaded to X:" << (int)*ptr << std::endl;
                    #endif
                    X = *ptr;
                }
                break;
                case 0xAE:
                {
                    uint8_t page = GET_NEXT_CHARI;
                    uint8_t addr = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    #ifdef __DEBUG__
                    std::cout << "Loaded to X:" << (int)GET_LAST_CHAR << ":" << (int)GET_CHAR << std::endl;
                    #endif
                    X = *ptr;
                }
                break;
                case 0xA6:
                {
                    X = GET_NEXT_CHARI;
                    #ifdef __DEBUG__
                    std::cout << "Loaded to X: (literal) " << (int)GET_CHAR << std::endl;
                    #endif
                }
                break;
                //LDXS
                case 0xDC:
                {
                    uint8_t page = 0x01;
                    uint8_t addr = S+GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    #ifdef __DEBUG__
                    std::cout << "Loaded to X: byte at stack index:" << (int)addr << std::endl;
                    #endif
                    X = *ptr;
                }
                break;

                //------------TRANSFERS------------
                case 0xAA: X = A; break; //TAX
                case 0x8A: A = X; break; //TXA
                case 0xA8: Y = A; break; //TAY
                case 0x98: A = Y; break; //TYA
                case 0xBA: X = S; break; //TSX
                case 0x9A: S = X; break; //TXS 

                //------------ARITHMETIC------------
                //ADCC (ADD CARRY AND FLAGS)
                case 0x69:{
                    int x = A + GET_NEXT_CHARI;
                    #ifdef __DEBUG__
                    std::cout << "ADD A OUT " << x << std::endl;
                    #endif
                    if(x < 0){P &= 0b10111110; P += 0b00000001; x=0;}
                    else if(x==0){P &= 0b10111110; P += 0b01000000;}
                    else{P &= 0b10111110;}
                    A = x;
                } break;
                //ADC $$$$
                case 0x6D: {
                    uint8_t page = GET_NEXT_CHARI;
                    uint8_t addr = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    int x = A + *ptr;
                    #ifdef __DEBUG__
                    std::cout << "Added A and " << page << ":" << addr << " = " << x << std::endl;
                    #endif
                    if(x < 0){P &= 0b10111110; P += 0b00000001; x=0;}
                    else if(x==0){P &= 0b10111110; P += 0b01000000;}
                    else{P &= 0b10111110;}
                    A = x;
                }
                //SBCC (ADD CARRY AND FLAGS)
                case 0xE9:{ 
                    int x = A - GET_NEXT_CHARI;
                    #ifdef __DEBUG__
                    std::cout << "MINUS A OUT " << x << std::endl;
                    #endif
                    if(x < 0){P &= 0b10111110; P += 0b00000001; x=0;}
                    else if(x==0){P &= 0b10111110; P += 0b01000000;}
                    else{P &= 0b10111110;}
                    A = x;
                } break;

                //------------DEC/INC------------
                //DEC
                case 0xCE:{
                    uint8_t page = GET_NEXT_CHARI;
                    uint8_t addr = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr); *ptr=(*ptr)-1;} break;
                case 0xCA:{X--;} break;
                case 0x88:{Y--;} break;
                //INC
                case 0xEE:{
                    uint8_t page = GET_NEXT_CHARI;
                    uint8_t addr = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr); *ptr=(*ptr)+1;} break;
                case 0xE8:{X++;} break;
                case 0xC8:{Y++;} break;

                //------------JUMP------------
                //JMPA
                case 0x4C:{uint16_t addr = GET_NEXT_CHARI; addr = addr << 8; addr+=GET_NEXT_CHARI; PC = addr-1;} break;
                //JMPR (non-implemented, don't know what JMP ind means in MOS)
                // case 0x6C:
                //JSR
                case 0x20:{
                    uint16_t addr = GET_NEXT_CHARI; 
                    addr = addr << 8; 
                    addr+=GET_NEXT_CHARI; 
                    uint16_t ret_addr = (uint16_t)(PC+1);
                    addToStack(ret_addr);
                    PC = addr-1;
                } break;
                //RTS
                case 0x60:{
                    PC=popStackWord()-1;
                } break;

                //------------COMPARE------------
                //CPXC
                case 0xE0:{int x = X-GET_NEXT_CHARI; if(x < 0){P &= 0b10111110; P += 0b00000001;}else if(x==0){P &= 0b10111110; P += 0b01000000;}else{P &= 0b10111110;}} break;
                //CPX 
                case 0xEC:{
                    uint8_t page = GET_NEXT_CHARI;
                    uint8_t addr = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    int x = X-*ptr; if(x < 0){P &= 0b10111110; P += 0b00000001;}else if(x==0){P &= 0b10111110; P += 0b01000000;}else{P &= 0b10111110;}} break;
                //CPYC
                case 0xC0:{int x = Y-GET_NEXT_CHARI; if(x < 0){P &= 0b10111110; P += 0b00000001;}else if(x==0){P &= 0b10111110; P += 0b01000000;}else{P &= 0b10111110;}} break;
                //CPY 
                case 0xCC:{
                    uint8_t page = GET_NEXT_CHARI;
                    uint8_t addr = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    int x = Y-*ptr; if(x < 0){P &= 0b10111110; P += 0b00000001;}else if(x==0){P &= 0b10111110; P += 0b01000000;}else{P &= 0b10111110;}} break;
                //CMPC
                case 0xC9:{int x = A-GET_NEXT_CHARI; if(x < 0){P &= 0b10111110; P += 0b00000001;}else if(x==0){P &= 0b10111110; P += 0b01000000;}else{P &= 0b10111110;}} break;
                //CMP
                case 0xCD:{
                    uint8_t page = GET_NEXT_CHARI;
                    uint8_t addr = GET_NEXT_CHARI;
                    uint8_t* ptr = RAM.getRAddress(page, addr);
                    int x = A-*ptr; if(x < 0){P &= 0b10111110; P += 0b00000001;}else if(x==0){P &= 0b10111110; P += 0b01000000;}else{P &= 0b10111110;}} break;

                //------------BRANCH------------
                //BEQ
                case 0xF0:{int bytes = GET_NEXT_CHARI; if((P&0b01000000)!=0)PC+=bytes;} break; //equal
                //BNE 
                case 0xD0:{int bytes = GET_NEXT_CHARI; if((P&0b01000000)==0)PC+=bytes;} break; //not equal
                //BMI
                case 0x30:{int bytes = GET_NEXT_CHARI; if((P&0b00000001)==1)PC+=bytes;} break; //less than
                //BPL
                case 0x10:{int bytes = GET_NEXT_CHARI; if((P&0b00000001)!=1)PC+=bytes;} break; //greater

                //------------BREAK------------
                case 0x04:{std::cout << "Program returned with: " << (int)A << std::endl; return {true, A};}break;

                case 0xFF:
                    std::cerr << "Following OP or OP arg combo is not implemented:" << std::endl;
                //Catch Invalid Ops
                default:
                    std::cerr << "Invalid OPCode for ";
                    printOp(GET_CHAR);
                    std::cerr <<":";
                    printf ("0x%02x", GET_CHAR);
                    std::cerr << std::endl;
                    // std::cerr << "Printing Instruction History:" << std::endl;
                    // for(uint8_t x:instructionHistory){printOp(x); std::cout << std::endl;}
                    return {true, -1};
                break;
            }
            PC++;
        }
        return {true, -1};
    }

    uint16_t popStackWord(){
        S+=2;
        uint8_t* ptr = RAM.getRAddress(0x01, S);
        ptr--;
        uint16_t x = ptr[0];
        x <<= 8;
        x+=ptr[1];
        #ifdef __DEBUG__
        printf ("Popped word '0x%04x' from stack at S(0x%02x)\n", x, S);
        #endif
        return x;
    }
    uint8_t popStackByte(){
        S++;
        uint8_t* ptr = RAM.getRAddress(0x01, S);
        #ifdef __DEBUG__
        printf ("Popped byte '0x%02x' from stack at S(0x%02x)\n", *ptr, S);
        #endif
        return *ptr;
    }

    void addToStack(uint8_t x){
        RAM.assignToAddress(0x01, S, x);
        S--;
        #ifdef __DEBUG__
        printf ("Added byte '0x%02x' to stack at S(0x%02x)\n", x, S+1);
        #endif
    }
    void addToStack(uint16_t x){
        RAM.assignToAddress(0x01, S, ((uint8_t*)&x)[0]);
        S--;
        RAM.assignToAddress(0x01, S, ((uint8_t*)&x)[1]);
        S--;
        #ifdef __DEBUG__
        printf ("Added word '0x%04x' to stack at S(0x%02x)\n", x, S+2);
        #endif
    }
};