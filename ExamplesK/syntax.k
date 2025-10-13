static string hello = "Hello World!\n"; # in the ROM as raw data #
const short outAddr = 0xFFFE; # replaces this symbol in the program with the number #
heap uint x = 0; # in the RAM at a given address #
heap uint y = 0; 

#puts raw bytecode into the ROM for stuff like textures#
LABEL NULLSTRING
raw
0 0 0 0 0 0 0 0
end 

proc:void printPos()
    _A = x + 48;
    store(_A, outAddr);
    _A = y + 48;
    store(_A, outAddr);
    ret;
end 

proc:uint main()
    sys(0x0A, hello);
    _X = 0;
    uint x = 0;
    while(_X < 11)
        if(_X = 10)
            store('1', outAddr);
            store('0', outAddr);
        else
            _A = _X+48;
            store(_A, outAddr);
        end 
        store(10, outAddr);
        _X++;
    end
    printPos();
    ret 0;
end