#include "ast.h"
#include "typeT.h"
#pragma once
class Interpreter
{
public:
    Interpreter(std::vector<ASTN *> toInterpret);

    Scope Interpret(Scope *current = nullptr);
    TypeT *Evaluate(ASTN *yep, Scope *current);

private:
    TypeT *GetVar(Scope *current, std::string name);

    std::vector<ASTN *> toInterpret;
    int current;
};