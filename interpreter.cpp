#include "interpreter.h"
#include "stdlib.h"

Interpreter::Interpreter(std::vector<ASTN *> toInterpret)
{
    this->toInterpret = toInterpret;
}

Scope Interpreter::Interpret(Scope *global)
{
    if (global == nullptr)
    {
        global = new Scope{
            new Scope{
                nullptr,
                initialScope},
            std::map<std::string, TypeT *>()};
    }
    for (current = 0; current < toInterpret.size(); current++)
    {
        Evaluate(toInterpret[current], global);
    }
    return *global;
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
    case BoolImmediate:
        return new BoolT{
            Bool,
            ((BoolN *)toEvaluate)->value};
    case CharImmediate:
        return new CharT{
            Char,
            ((CharN *)toEvaluate)->value};
    case Lambda:
    {
        LambdaN *lambda = (LambdaN *)toEvaluate;
        return new FuncT{
            Function,
            [lambda, currentScope](Scope *scope, std::vector<TypeT *> params)
            {
                auto paramNames = std::map<std::string, TypeT *>();
                for (int i = 0; i < lambda->parameters.size() && i < params.size(); i++)
                {
                    paramNames[((VarNameN *)lambda->parameters[i])->name] = params[i];
                }

                Scope *current = new Scope{
                    currentScope,
                    paramNames};
                TypeT *out = (new Interpreter(lambda->toRun))->Interpret(current).vars["return"];
                free(current);
                return out;
            }};
    }
    case Function:
    {
        FuncCallN *called = (FuncCallN *)toEvaluate;
        std::string name = called->name->name;

        auto params = std::vector<TypeT *>();
        for (int i = 0; i < called->parameters.size(); i++)
        {
            params.push_back(Evaluate(called->parameters[i], currentScope));
        }

        FuncT *function = (FuncT *)GetVar(currentScope, name);

        currentScope->vars["return"] = function->body(currentScope, params);
        return currentScope->vars["return"];
    }
    case Unevaluated:
    {
        auto out = std::vector<TypeT *>();
        for (char c : ((UnevaluatedN *)toEvaluate)->value)
        {
            out.push_back(new CharT{
                Char,
                c});
        };
        return new ListT{
            String,
            out};
    }
    case StringImmediate:
    {
        auto out = std::vector<TypeT *>();
        for (char c : ((UnevaluatedN *)toEvaluate)->value)
        {
            out.push_back(new CharT{
                Char,
                c});
        };
        return new ListT{
            String,
            out};
    }
    case ListImmediate:
    {
        Interpreter *tmp = new Interpreter(((ListN *)toEvaluate)->values);
        auto out = std::vector<TypeT *>();
        for (ASTN *a : tmp->toInterpret)
        {
            out.push_back(tmp->Evaluate(a, currentScope));
        }
        return new ListT{
            List,
            out};
    }
    case RangeList:
    {
        auto out = std::vector<TypeT *>();
        IntT *start = (IntT *)Evaluate(((RangeN *)toEvaluate)->start, currentScope);
        IntT *end = (IntT *)Evaluate(((RangeN *)toEvaluate)->end, currentScope);
        for (int i = start->value; i < end->value; i++) {
            out.push_back(new IntT{
                Int,
                i
            });
        }
        return new ListT{
            List,
            out };
    }
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
    std::cout << "namy" << name << "asdf" << std::endl;
    throw VarNotFound();
}
