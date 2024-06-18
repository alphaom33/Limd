#include "interpreter.h"
#pragma once

TypeT *Plus(Scope *current, std::vector<TypeT *> params)
{
    int sum = 0;
    for (TypeT *t : params)
    {
        sum += ((IntT *)t)->value;
    }
    return new IntT{
        Int,
        sum};
}

TypeT *Minus(Scope *current, std::vector<TypeT *> params)
{
    int difference = ((IntT *)params[0])->value;
    for (int i = 1; i < params.size(); i++)
    {
        difference -= ((IntT *)params[i])->value;
    }
    return new IntT{
        Int,
        difference};
}

TypeT *Multiply(Scope *current, std::vector<TypeT *> params)
{
    int difference = ((IntT *)params[0])->value;
    for (int i = 1; i < params.size(); i++)
    {
        difference *= ((IntT *)params[i])->value;
    }
    return new IntT{
        Int,
        difference};
}

TypeT *Divide(Scope *current, std::vector<TypeT *> params)
{
    int difference = ((IntT *)params[0])->value;
    for (int i = 1; i < params.size(); i++)
    {
        difference /= ((IntT *)params[i])->value;
    }
    return new IntT{
        Int,
        difference};
}

TypeT *Set(Scope *current, std::vector<TypeT *> params)
{
    current->vars[((StringT *)params[0])->value] = params[1];
    return params[1];
}

TypeT *Print(Scope *current, std::vector<TypeT *> params)
{
    for (TypeT *t : params)
    {
        std::cout << t;
    }
    return new TypeT{};
}

std::map<std::string, TypeT *> initialScope = {
    std::pair<std::string, TypeT *>(
        std::string("+"),
        new FuncT{
            Function,
            Plus}),
    std::pair<std::string, TypeT *>(
        std::string("-"),
        new FuncT{
            Function,
            Minus}),
    std::pair<std::string, TypeT *>(
        std::string("*"),
        new FuncT{
            Function,
            Multiply}),
    std::pair<std::string, TypeT *>(
        std::string("/"),
        new FuncT{
            Function,
            Divide}),
    std::pair<std::string, TypeT *>(
        std::string("="),
        new FuncT{
            Function,
            Set}),
    std::pair<std::string, TypeT *>(
        std::string("print"),
        new FuncT{
            Function,
            Print}),
};