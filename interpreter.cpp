#include "interpreter.h"
#include "stdlib.h"

Interpreter::Interpreter(std::vector<ASTN *> toInterpret)
{
    this->toInterpret = toInterpret;
}

Scope Interpreter::Interpret()
{
    Scope global = Scope{
        new Scope{
            nullptr,
            initialScope},
        std::map<std::string, TypeT *>()};
    for (current = 0; current < toInterpret.size(); current++)
    {
        Evaluate(toInterpret[current], &global);
    }
    return global;
}

TypeT *Interpreter::Evaluate(ASTN *toEvaluate, Scope *currentScope)
{
    switch (toEvaluate->op)
    {
    case VarName:
        return GetVar(currentScope, ((VarNameN *)toEvaluate)->name);
    case IntImmediate:
        return new IntT{
            Int,
            ((IntN *)toEvaluate)->value};
    case Function:
    {
        FuncCallN *called = (FuncCallN *)toEvaluate;
        std::string name = called->name->name;

        auto params = std::vector<TypeT *>();
        for (int i = 0; i < called->parameters.size(); i++)
        {
            params.push_back(Evaluate(called->parameters[i], currentScope));
        }

        FuncT *function = (FuncT *)initialScope[name];

        return function->body(currentScope, params);
    }
    break;
    case Unevaluated:
        return new StringT{
            String,
            ((UnevaluatedN *)toEvaluate)->value
        };
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
