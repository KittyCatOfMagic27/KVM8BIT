static string finalMessage = "Goodbye World!\n"; # in the ROM as raw data #
static string hello = "Hello World!\n";
heap uint x = 0; 

proc:uint main()
    if(x==1)
        sys(0x0A, hello);
    end
    ret 0;
end