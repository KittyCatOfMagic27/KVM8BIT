#include <iostream>
#include <fstream>
#include <filesystem>
#include <unordered_map>
#include <vector>

#include "./tables/OPcodes.hpp"

#define PROGRAM_FILE "program.kasm"
#define ROM_FILE_NAME "ROM.bin"

bool isNumber(const std::string& str){
  for (char const &c : str) {
    if ((c<'0')||(c>'9')) return false;
  }
  return true;
}

struct Label_Bundle{
    std::string label;
    uint16_t bytesToReplace;
};

int main(){
    std::fstream programIn;
    programIn.open(PROGRAM_FILE, std::ios_base::in);

    std::fstream binaryOut;
    binaryOut.open(ROM_FILE_NAME, std::ios_base::out|std::ios_base::binary);
    binaryOut.clear();

    std::vector<Label_Bundle> labelBundles;
    std::unordered_map<std::string,uint16_t> Labels;
    uint16_t writtenBytes = 0;
    uint16_t mainAddress = 0x0003;
    uint16_t jumpStartAddress = 0x0000;

    std::string nextToken = "";
    programIn >> nextToken;

    if(nextToken!="__START_HEADER__"){
        binaryOut << (uint8_t)0x4C; 
        binaryOut << (uint8_t)0x00; 
        binaryOut << (uint8_t)0x03; 
        writtenBytes = 3;
    }

    bool keepWhile0 = true;
    while(keepWhile0){
        keepWhile0 = !programIn.eof();
        if(nextToken=="LABEL"){
            programIn >> nextToken;
            Labels[nextToken] = writtenBytes;
            if(nextToken=="__MAIN__") mainAddress = writtenBytes;
            programIn >> nextToken;
        }
        else if(nextToken=="__START_HEADER__"){
            programIn >> nextToken;
        }
        else if(nextToken=="__END_HEADER__"){
            jumpStartAddress = writtenBytes;
            binaryOut << (uint8_t)0x4C; 
            binaryOut << (uint8_t)0x00; 
            binaryOut << (uint8_t)0x03; 
            writtenBytes += 3;
            programIn >> nextToken;
        }
        else if(nextToken[0]=='#'){
            while(nextToken[nextToken.size()-1]!='#') programIn >> nextToken;
            programIn >> nextToken;
        }
        else if(nextToken=="RAW"){
            while(nextToken!="END"){
                if(nextToken[0]=='\"'&&nextToken[nextToken.size()-1]=='\"'){
                    for(int i=1; i<nextToken.size()-1; i++){
                        binaryOut << nextToken[i];
                        writtenBytes++;
                    }
                }
                else if(nextToken[0]=='\"'){
                    for(int i=1; i<nextToken.size(); i++){
                        binaryOut << nextToken[i];
                        writtenBytes++;
                    }
                    programIn >> nextToken;
                    while (nextToken[nextToken.size()-1] != '"') {
                        binaryOut << ' ';
                        writtenBytes++;
                        for(int i=0; i<nextToken.size(); i++){
                            binaryOut << nextToken[i];
                            writtenBytes++;
                        }
                        programIn >> nextToken;
                    }
                    binaryOut << ' ';
                    writtenBytes++;
                    for(int i=0; i<nextToken.size()-1; i++){
                        binaryOut << nextToken[i];
                        writtenBytes++;
                    }
                }
                else if(isNumber(nextToken)){
                    binaryOut << (char)stoi(nextToken);
                    writtenBytes++;
                }
                programIn >> nextToken;
            }
            programIn >> nextToken;
        }
        else{
            std::string op;
            uint16_t codex;
            if(nextToken[nextToken.size()-1]==';'){
                op = nextToken;
                op.erase(nextToken.size()-1);
                codex = OPCodetable[op];
            }
            else{
                op = nextToken;
                codex = OPCodetable[op];
            }
            std::string output = "";

            //count args
            int arg_count = 0;
            if(op=="STRC" | op=="STCS" | op == "STSH") arg_count = -2;
            if(nextToken[nextToken.size()-1]!=';'){
                bool keepWhile1 = true;
                programIn >> nextToken;
                while(keepWhile1){
                    keepWhile1 = !programIn.eof();
                    bool identified = false;

                    if(nextToken[nextToken.size()-1]==';'){keepWhile1 = false; nextToken.pop_back();}
                    //hex number
                    if(nextToken.size()>2){
                        if((nextToken[0]=='0')&&(nextToken[1]=='x')){
                            //add hex parsing
                            std::string temp = nextToken;
                            temp.erase(0,2);
                            uint16_t value = (uint16_t)stoi(temp, 0, 16);
                            if(temp.size()==2) output += ((char*)&value)[0];
                            else if(temp.size()==4) {output += ((char*)&value)[1]; output += ((char*)&value)[0]; arg_count++;}
                            else {std::cerr << "Invalid length hex number: " << nextToken << std::endl; exit(-1);}
                            identified = true;
                        }
                        else if(nextToken[0]=='\''&&nextToken[nextToken.size()-1]=='\''&&nextToken.size()==3){
                            output += nextToken[1];
                            identified = true;
                        }
                    }
                    if(isNumber(nextToken)){
                        if(stoi(nextToken)<256){
                            output += (char)stoi(nextToken);
                            identified = true;
                        }
                        else {
                            uint16_t x = (uint16_t)stoi(nextToken);                
                            output += ((char*)&x)[1];
                            output += ((char*)&x)[0];
                            identified = true;
                            arg_count++;
                        }
                    }
                    else if(!identified){
                        if(Labels.find(nextToken) != Labels.end()){
                            uint16_t x = Labels[nextToken];                
                            output += ((char*)&x)[1];
                            output += ((char*)&x)[0];
                        }
                        else{
                            uint16_t x = 0x0000;                
                            output += ((char*)&x)[1];
                            output += ((char*)&x)[0];
                            labelBundles.push_back({nextToken, (uint16_t)(writtenBytes+1)});
                        }
                        arg_count++;
                    }
                    //loop counter and featch next token
                    arg_count++;
                    programIn >> nextToken;
                }
            } else programIn >> nextToken;

            if(arg_count == 0) arg_count = 1;
            uint8_t opcode = ((uint8_t*)&codex)[arg_count-1] ;
            if(opcode == 0xFF) std::cerr << "INVALID OP COMBINATION: " << op << " " << arg_count << " args" << std::endl;

            output = (char)opcode + output;
            for(int i = 0; i < output.size(); i++) binaryOut << output[i];
            writtenBytes += output.size();
        }
        binaryOut.flush();
    }

    binaryOut << (uint8_t)0x00;
    
    if(mainAddress!=3){
        binaryOut.seekp(jumpStartAddress+1);
        binaryOut << ((char*)&mainAddress)[1];
        binaryOut.seekp(jumpStartAddress+2);
        binaryOut << ((char*)&mainAddress)[0];
    }

    for(Label_Bundle lb : labelBundles){
        uint16_t x = Labels[lb.label];
        binaryOut.seekp(lb.bytesToReplace);
        binaryOut << ((char*)&x)[1];
        binaryOut.seekp(lb.bytesToReplace+1);
        binaryOut << ((char*)&x)[0];
    }

    programIn.close();
    binaryOut.close();
    return 0;
}