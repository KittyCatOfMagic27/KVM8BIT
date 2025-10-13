proc:void printPos()
    const short outAddr = 0xFFFE; # replaces this symbol in the program with the number #
    uint x = 0; # in the RAM at a given address #
    uint y = 0; 
    _A = x + 48;
    store(_A, outAddr);
    _A = y + 48;
    store(_A, outAddr);
    ret;
end 

proc:uint main()
    printPos();
    ret 0;
end