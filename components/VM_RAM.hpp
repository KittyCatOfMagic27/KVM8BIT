#pragma once

class K_RAM{
    public:
    size_t SIZE;
    uint8_t* MEMORY;
    std::string OUT_BUFFER="";

    K_RAM(){}
    K_RAM(size_t _SIZE): SIZE(_SIZE){}

    void INIT_RAM(size_t _SIZE){
        SIZE = _SIZE;
        MEMORY = (uint8_t*)malloc(SIZE);
    }

    // Addressing
    uint8_t* getRAddress(uint8_t page, uint8_t addr){
        uint16_t offset = page;
        offset = offset << 8;
        offset += addr;
        return MEMORY+offset;
    }

    uint8_t* getRAddress(uint16_t addr){
        return MEMORY+addr;
    }

    void assignToAddress(uint8_t* ptr, uint8_t data){
        uint16_t rel = ptr-MEMORY;
        if(rel==IO_CONSOLE_OUT_ADDRESS){std::cout << data; return;}
        if(rel==IO_CONSOLE_BUFFERED_OUT_ADDRESS){OUT_BUFFER.push_back((char)data); return;}
        if(rel>=SIZE){printf("Address '0x%04x' out of range.\n", rel); exit(-1);}
        *ptr = data;
        #ifdef __XRAY_STACK__
        if(rel>=0x0100&&rel<=0x01FF){
            printf("\033[2J\033[H");
            printOp(CURRENT_INSTRUCTION);
            std::cout << std::endl;
            printStack();
            std::string BUFFER = "";
            std::cout << "Any to Continue, q to quit." << std::endl;
            std::cin >> BUFFER;
            if(BUFFER == "q") exit(0);
        }
        #endif
    }

    void assignToAddress(uint8_t page, uint8_t addr, uint8_t data){
        assignToAddress(getRAddress(page, addr), data);
    }

    void printStack(){
        for(uint8_t i = 0; i < 16; i++){
            for(uint8_t j = 0; j < 16; j++){
                uint8_t addr = i;
                addr <<= 4;
                addr +=j;
                printf ("%02x ", *getRAddress(0x01, addr));
            } 
            std::cout << std::endl;
        }
    }

    ~K_RAM(){free(MEMORY);}
};