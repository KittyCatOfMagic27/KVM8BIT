#pragma once

#include "../KCompilerHeaders/types.hpp"

using enum Operation::OP_FORMAT;
using enum AnonValueType::VALUE;
using enum TYPE;

static std::unordered_map<std::string,Operation> operationsTable;

void PROCESS_store(){}
void PROCESS_ret(){}

void initializeOperationsTable(){
    operationsTable["store"] = {
        "store", 
        OP_FUNCXY,
        VT_ERROR, T_BYTE,
        VT_ERROR, T_BYTE,
        T_VOID,
        0, nullptr
    };
    operationsTable["store"].typeReq1VT = VT_REGISTER;
    operationsTable["store"].typeReq2VT = VT_EXPR|VT_LITERAL|VT_STACKVAR|VT_HEAPVAR;
    operationsTable["store"].processingFunction = &PROCESS_store;
    operationsTable["ret"] = {
        "ret", 
        OP_OX,
        VT_ERROR, T_BYTE,
        VT_ERROR, T_BYTE,
        T_VOID,
        0, nullptr
    };
    operationsTable["ret"].typeReq1VT = VT_REGISTER|VT_EXPR|VT_LITERAL|VT_STACKVAR|VT_HEAPVAR;
    operationsTable["ret"].processingFunction = &PROCESS_ret;
}
