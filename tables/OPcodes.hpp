#pragma once

#include <string>
#include <unordered_map>

//opcodes stored by amount of args right to left (0x2211)
//if no args, slot 1 is taken
//empty opcode is 0xFF
static std::unordered_map<std::string,uint16_t> OPCodetable({
    {"SPT",  0xFF82},
    {"SYS",  0xFFE2},
    {"SAL",  0xFF1A},
    {"DAL",  0xFF3A},

    {"LDY",  0xACB4},
    {"LDYC", 0xFFA0},
    {"LDYS", 0xFF5C},
    {"LDA",  0xADA1},
    {"LDAC", 0xFFA9},
    {"LDAS", 0xFF7C},
    {"LDX",  0xAEA2},
    {"LDXC", 0xFFA6},
    {"LDXS", 0xFFDC},
    
    {"TAX",  0xFFAA},
    {"TXA",  0xFF8A},
    {"TAY",  0xFFA8},
    {"TYA",  0xFF98},
    {"TSX",  0xFFBA},
    {"TXS",  0xFF9A},

    {"STRC", 0x89FF},
    {"STCS", 0xC2FF},
    {"STSH", 0xFF04},
    {"STY",  0x8C80},
    {"STYS", 0xFFFC},
    {"STA",  0x8D81},
    {"STAS", 0xFF1C},
    {"STX",  0x8E82},
    {"STXS", 0xFF3C},

    {"ADCC", 0xFF69},
    {"ADC",  0xFF6D},
    {"SBCC", 0x00E9},
    {"SBC",  0xEDE5},

    {"DEC",  0xCEFF},
    {"DEX",  0xFFCA},
    {"DEY",  0xFF88},

    {"INC",  0xEEFF},
    {"INX",  0xFFE8},
    {"INY",  0xFFC8},

    {"ANDC", 0xFF29},
    {"AND",  0x2D25},
    {"XORC", 0xFF49},
    {"XOR",  0x4D45},
    {"ORAC", 0xFF09},
    {"ORA",  0x0D05},

    {"CMPC", 0xFFC9},
    {"CMP",  0xCDC5},
    {"CPXC", 0xFFE0},
    {"CPX",  0xECE4},
    {"CPYC", 0xFFC0},
    {"CPY",  0xCCC4},

    {"JMPA", 0x4CFF},
    {"JMPR", 0x6CFF},
    {"JSR",  0x20FF},
    {"RTS",  0xFF60},

    {"BPL",  0xFF10},
    {"BMI",  0xFF30},
    {"BVC",  0xFF50},
    {"BVS",  0xFF70},
    {"BCC",  0xFF90},
    {"BCS",  0xFFB0},
    {"BNE",  0xFFD0},
    {"BEQ",  0xFFF0},

    {"BRK", 0xFF04},
});