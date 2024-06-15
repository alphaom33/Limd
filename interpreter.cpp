#include "interpreter.h"
#include "stdlib.h"

Interpreter::Interpreter(std::vector<ASTN *> toInterpret)
{
    this->toInterpret = toInterpret;
}

void Interpreter::Interpret()
{
    Scope global = Scope{
        new Scope{
            nullptr,
            initialScope
        },
        std::map<std::string, TypeT *>()};
    for (current = 0; current < toInterpret.size(); current++)
    {
        switch (toInterpret[current]->op)
        {
        case FuncCall:
        {
            Evaluate(toInterpret[current], &global);
        }
        break;
        }
    }
}

TypeT *Interpreter::Evaluate(ASTN *yep, Scope *currentScope)
{
    switch (yep->op)
    {
    case VarName:
        return GetVar(currentScope, ((VarNameN *)yep)->name);
    case IntImmediate:
        return new IntT{
            Int,
            ((IntN *)yep)->value
            };
    case Function:
            FuncCallN *a = (FuncCallN *)yep;
            auto params = std::vector<TypeT *>();
            for (ASTN *a : a->parameters) {
                params.push_back(Evaluate(a, currentScope));
            }

            VarNameN *b = (VarNameN *)a->name;
            FuncT * c = (FuncT *)initialScope[b->name];

            return c->body(params);
            break;
    }
    return nullptr;
}

TypeT *Interpreter::GetVar(Scope *current, std::string name)
{
    while (current != nullptr)
    {
        if (current->vars.contains(name))
        {
            return current->vars[name];
        }
        current = current->parent;
    }
    return nullptr;
}
