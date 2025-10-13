static string finalMessage = "Goodbye World!\n"; # in the ROM as raw data #
static string hello = "Hello World!\n";
const short outAddr = 0xFFFF; # replaces this symbol in the program with the number #
heap uint x = 0; # in the RAM at a given address #
heap uint y = 0; 

static buffer texture = [ #a block of preset ROM memory#
    1 2 3 4 5
];

proc:void finishingProc()
    sys(0x0A, finalMessage);
    exit(0); #cannot return since it'd be a RTS instead of BRK#
end

proc:void printPos()
    _A = x + 48;
    store(_A, outAddr);
    _A = y + 48;
    store(_A, outAddr);
    buffer[4] b; # buffer is like an array #
    b[0] = 'h';
    b[1] = 'i';
    b[2] = 10;
    b[3] = 0;
    sys(0x0C, b);
    ret;
end 

proc:uint main()
    sys(0x09, finishingProc);
    sys(0x0A, hello);
    printPos();
    ret 0;
end