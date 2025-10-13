#pragma once
#include <vector>

struct PaletteB2{uint8_t data[4*3]; uint8_t& operator[](int i){return data[i];}};
struct PaletteB4{uint8_t data[16*3]; uint8_t& operator[](int i){return data[i];}};
struct PaletteB8{uint8_t data[256*3]; uint8_t& operator[](int i){return data[i];}};

struct PalettePointer{
    enum PaletteType:uint8_t{
        NONE = 0,
        PT_B2, PT_B4, PT_B8
    };
    void* palette;
    PaletteType type;
    PalettePointer(){}
    PalettePointer(PaletteType _type){init(_type);}
    void init(PaletteType _type){
        type = _type;
        switch(type){
            case PaletteType::PT_B2: palette = new PaletteB2; 
            break;
            case PaletteType::PT_B4: palette = new PaletteB4; 
            break;
            case PaletteType::PT_B8: palette = new PaletteB8; 
            break;
            default:
            break;
        }
    }
    uint8_t& operator[](int i){
        switch(type){
            case PaletteType::PT_B2: return (*(PaletteB2*)palette)[i]; 
            break;
            case PaletteType::PT_B4: return (*(PaletteB4*)palette)[i]; 
            break;
            case PaletteType::PT_B8: return (*(PaletteB8*)palette)[i]; 
            break;
            default:
            std::cerr << "PALETTE WITH NO TYPE BEING READ" << std::endl;
            exit(-1);
            break;
        }
    }
    ~PalettePointer(){
        switch(type){
            case PaletteType::PT_B2: delete (PaletteB2*) palette; 
            break;
            case PaletteType::PT_B4: delete (PaletteB4*) palette; 
            break;
            case PaletteType::PT_B8: delete (PaletteB8*) palette; 
            break;
            default:
            break;
        }
    }
};

class Texture{
    public:
    SDL_Texture* texture;
    uint8_t w,h;
    uint8_t size = 1;
    uint8_t colorFormat;
    uint8_t* data = nullptr;
    uint8_t* processedData = nullptr;
    int currentPalette = -1;
    Texture(){}
    Texture(SDL_Renderer* renderer , uint8_t _dWidth, uint8_t _dHeight, uint8_t _formatSize, uint8_t _colorFormat):
        w(_dWidth), h(_dHeight), size(_formatSize), colorFormat(_colorFormat){
        data = (uint8_t*)malloc(w*h);
        processedData = (uint8_t*)malloc(w*h*3);
        texture = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_RGB24, SDL_TEXTUREACCESS_STREAMING, w, h);
    }
    uint8_t& operator[](int i){return data[i];}
    void update(int palette, std::vector<PalettePointer> &palettes){
        if(palette==currentPalette)return;
        for(int i = 0; i < w*h; i++){
            processedData[i*3]   = palettes[palette][data[i]*3];
            processedData[i*3+1] = palettes[palette][data[i]*3+1];
            processedData[i*3+2] = palettes[palette][data[i]*3+2];
        }
        currentPalette=palette;
        SDL_UpdateTexture(texture, NULL, processedData, w*3);
    }
    void freeTex(){if(data!=nullptr){free(data); data=nullptr;} if(processedData!=nullptr){free(processedData); processedData=nullptr;} SDL_DestroyTexture(texture);}
};

class K_PPU{
    public:
    kg::KWINDOW window;
    int scale = 4; //256x240 to 1024x960
    int tileSize = 8; //or 16 for big tile mode
    std::vector<PalettePointer> palettes;
    std::vector<Texture> textures;

    K_PPU(){}

    // loads
    int loadPalette(uint8_t paletteType, std::vector<uint8_t> data){
        PalettePointer::PaletteType type;
        switch(paletteType){
            case 4: type = PalettePointer::PaletteType::PT_B2; break; case 16: type = PalettePointer::PaletteType::PT_B4; break; case 255: type = PalettePointer::PaletteType::PT_B8; break;
        }
        palettes.push_back({});
        palettes[palettes.size()-1].init(type);
        for(int i = 0; i < data.size(); i++){
            palettes[palettes.size()-1][i] = data[i];
        }
        return palettes.size()-1;
    }

    int loadTexture(uint8_t dWidth, uint8_t dHeight, uint8_t formatSize, uint8_t colorFormat, std::vector<uint8_t> data){
        textures.push_back({window.gRenderer, dWidth, dHeight, formatSize, colorFormat});
        for(int y = 0; y < dHeight; y++){
            for(int x = 0; x < dWidth; x++){
                textures[textures.size()-1][(y*dWidth)+x] = data[(y*dWidth)+x];
            }
        }
        return textures.size()-1;
    }

    // draws
    void drawRectTile(uint8_t x, uint8_t y, Vector3<uint8_t> color){
        window.rect(realPos(x*tileSize, (y*tileSize)+tileSize), scale*tileSize, scale*tileSize, color);
    }
    void drawPixel(uint8_t x, uint8_t y, Vector3<uint8_t> color){
        window.rect(realPos(x, y+1), scale, scale, color);
    } 
    void drawTexture(uint8_t x, uint8_t y, uint8_t tex, uint8_t palette){
        if(tex>textures.size()-1){ 
            std::cerr << "INVALID TEXTURE ID: " << (int)tex << std::endl;
            return;
        }
        textures[tex].update(palette, palettes);
        Vector2<int> pos = realPos(x, y);
        SDL_Rect dest_rect = {pos.x, pos.y, (int)(textures[tex].w)*scale*textures[tex].size, (int)(textures[tex].h)*scale*textures[tex].size};
        SDL_RenderCopy(window.gRenderer, textures[tex].texture, nullptr, &dest_rect);
    }

    // conversion 
    Vector2<int> realPos(int x, int y){
        return {x*scale, (240-y)*scale};
    }

    ~K_PPU(){
        for(Texture t : textures){
            t.freeTex();
        }
    }
};