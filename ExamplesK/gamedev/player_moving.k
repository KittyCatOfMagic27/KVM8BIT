static buffer testPallet = [
    4
    255 0 0
    0 255 0
    0 0 255
    255 255 255
];

static buffer texture01 = [
    4 4
    4 1
    75 75 75 75
];

heap uint playerX = 40;
heap uint playerY = 40;

static string winClsMsg = "Why close window :(\n";
proc:void windowClosed()
    sys(0x0A, winClsMsg);
    exit(0);
    ret;
end

proc:void loadTestPallet()
    sys(0x08, 0x06, testPallet);
    ret;
end

proc:void loadTestTexture()
    sys(0x08, 0x04, texture01);
    ret;
end

proc:void drawCheckers()
    uint x = 0;
    uint y = 0;
    uint display = 48;
    uint bOrG = 0;
    while (x < 32)
        y = 0;
        while (y < 33)
            if (bOrG == 0) 
                bOrG = 128;
            else
                bOrG = 0;
            end
            sys(0x08, 0x02, x, y, bOrG, bOrG, bOrG);
            y = y + 1;
        end
        x = x + 1;
    end
    ret;
end

#inconsistent seg fault on draw#
proc:void drawPlayer()
    sys(0x08, 0x05, playerX, playerY, 0, 0);
    ret;
end 

proc:void wait()
    #bytes to the milisecond in reverse order, segfaults#
    sys(0x02, 13, 0, 0, 0);
    ret;
end

proc:void updatePos()
    sys(0x0B, 'w');
    if(_A==1)
        playerY = playerY + 1;
    end
    sys(0x0B, 's');
    if(_A==1)
        playerY = playerY - 1;
    end

    sys(0x0B, 'd');
    if(_A==1)
        playerX = playerX + 1;
    end
    sys(0x0B, 'a');
    if(_A==1)
        playerX = playerX - 1;
    end
    ret;
end

proc:uint main()
    loadTestPallet();
    loadTestTexture();
    drawPlayer();
    sys(0x0D, windowClosed);

    while(1)
        sys(0x08, 0x01, 0, 0, 0); #set background to black#

        sys(0x0B); #load events#
        updatePos();

        drawCheckers();
        drawPlayer();
        sys(0x07); #window update#

        wait();
    end
    ret 0;
end