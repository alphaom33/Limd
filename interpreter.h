#include "ast.h"
#include "TypeT.h"
#include "map"
#pragma once
class Interpreter
{
public:
    Interpreter(std::vector<ASTN *> toInterpret);

    typedef struct Scope
    {
        Scope *parent;
        std::map<std::string, TypeT *> vars;
    } Scope;

    void Interpret();
    TypeT *Evaluate(ASTN *yep, Scope *current);

private:
    TypeT *GetVar(Scope *current, std::string name);

    std::vector<ASTN *> toInterpret;
    int current;
};