#pragma once
/*
DEPENDENCIES:
    SDL2
*/

#include <SDL2/SDL.h>

#include "./math/vectors.hpp"

#include <string>
#include <cstring>
#include <iostream>

namespace kg{
    struct WINDOW_INFO{
        int width = 1024;
        int height = 960;
        int xpos = -1;
        int ypos = -1;
        std::string window_name = "";
    };

    class KWINDOW{
        public:
        // Pointers to our window and surface
        // SDL_Surface* winSurface = NULL;
        SDL_Window* window = NULL;
        SDL_Renderer* gRenderer = NULL;
        WINDOW_INFO gInfo;
        bool closeWindow = false;
        void(*loopFunc)(void);

        KWINDOW(){}

        KWINDOW(WINDOW_INFO _gInfo){
            init(_gInfo);
        }

        // MEMORY LEAKS IN HERE, ALL SDL2 CODE, IGNORE
        void init(WINDOW_INFO _gInfo){
            // std::cout << "WINDOW_INFO {\n   width: "
            // << _gInfo.width << "\n  height: "
            // << _gInfo.height << "\n  xpos: "
            // << _gInfo.xpos << "\n  ypos: "
            // << _gInfo.ypos << "\n  window_name: "
            // << _gInfo.window_name << "\n}"
            // << std::endl;

            gInfo = _gInfo;
            // Initialize SDL. SDL_Init will return -1 if it fails.
            if ( SDL_Init( SDL_INIT_EVERYTHING ) < 0 ) {
                std::cerr << "Error initializing SDL: " << SDL_GetError() << std::endl;
                // End the program
                exit(-1);
            }

            // Create our window
            window = SDL_CreateWindow( gInfo.window_name.c_str(), gInfo.xpos, gInfo.ypos, gInfo.width, gInfo.height, SDL_WINDOW_SHOWN );

            // Make sure creating the window succeeded
            if ( !window ) {
                std::cerr << "Error creating window: " << SDL_GetError()  << std::endl;
                // End the program
                exit(-1);
            }

            // // Get the surface from the window
            // winSurface = SDL_GetWindowSurface( window );

            // // Make sure getting the surface succeeded
            // if ( !winSurface ) {
            //     std::cerr << "Error getting surface: " << SDL_GetError() << std::endl;
            //     // End the program
            //     exit(-1);
            // }

            gRenderer = SDL_CreateRenderer( window, -1, SDL_RENDERER_ACCELERATED );
			if(!gRenderer)
			{
				std::cerr << "Renderer could not be created! SDL Error: " << SDL_GetError() << std::endl;
				exit(-1);
			}
        }

        void end(){
            SDL_DestroyWindow(window);
            SDL_DestroyRenderer(gRenderer);
            SDL_Quit();
            closeWindow=true;
        }

        inline void provideDisplayLoop(void (*func)(void)){
            loopFunc = func;
        }

        void start(){
            while(true){
                if(closeWindow) return;
                loopFunc();
            }
        }

        inline void swap(){
            SDL_RenderPresent( gRenderer );
        }

        inline void next_tick(){
            swap();
        }

        //GL Functions
        inline void colorBackground(int r, int g, int b){
            SDL_SetRenderDrawColor( gRenderer, r, g, b, 0xFF );
            SDL_RenderClear( gRenderer );
        }

        inline void rect(Vector2<int> pos, int w, int h, Vector3<uint8_t> color){
            SDL_Rect fillRect = {pos.x, pos.y, w, h};
            SDL_SetRenderDrawColor(gRenderer, color.x, color.y, color.z, 0xFF);
            SDL_RenderFillRect(gRenderer, &fillRect);
        }
    };
}