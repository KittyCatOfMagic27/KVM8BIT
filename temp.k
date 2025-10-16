static string finalMessage = "Goodbye World!\n"; # in the ROM as raw data #
static string hello = "Hello World!\n";
heap uint x = 1; 
heap uint y = 1; 

proc:uint main()
    uint i = 0;
    while (i < 255)
        sys(0x0A, hello); 
        x = 0;
        i = i + 1;
    end
    ret 0;
end