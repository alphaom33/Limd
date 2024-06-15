#include "interpreter.h"
#pragma once

TypeT *Plus(std::vector<TypeT *> params)
{
    int sum = 0;
    for (TypeT *t : params)
    {
        sum += ((IntT *)t)->value;
    }
    std::cout << sum;
    return new IntT{
        Int,
        sum};
}

TypeT *Minus(std::vector<TypeT *> params)
{
    int difference = ((IntT *)params[0])->value;
    for (int i = 1; i < params.size(); i++)
    {
        difference -= ((IntT *)params[i])->value;
    }
    std::cout << difference;
    return new IntT{
        Int,
        difference};
}

TypeT *Multiply(std::vector<TypeT *> params)
{
    int difference = ((IntT *)params[0])->value;
    for (int i = 1; i < params.size(); i++)
    {
        difference *= ((IntT *)params[i])->value;
    }
    std::cout << difference;
    return new IntT{
        Int,
        difference};
}

TypeT *Divide(std::vector<TypeT *> params)
{
    int difference = ((IntT *)params[0])->value;
    for (int i = 1; i < params.size(); i++)
    {
        difference /= ((IntT *)params[i])->value;
    }
    std::cout << difference;
    return new IntT{
        Int,
        difference};
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
            };