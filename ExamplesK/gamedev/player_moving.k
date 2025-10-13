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

proc:void loadTestPallet()
    sys(0x08, 0x06, testPallet);
    ret;
end

proc:void loadTestTexture()
    sys(0x08, 0x04, texture01);
    ret;
end

#inconsistent seg fault on draw#
proc:void drawPlayer()
    sys(0x08, 0x05, playerX, playerY, 0, 0);
    ret;
end 

proc:void wait()
    #bytes to the milisecond in reverse order, segfaults#
    sys(0x02, 0, 4, 0, 0);
    ret;
end

proc:uint main()
    loadTestPallet();
    loadTestTexture();
    drawPlayer();
    sys(0x07); #window update#
    wait();
    ret 0;
end