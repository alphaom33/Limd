#include "Type.h"
#include "functional"
#pragma once
struct TypeT {
    Type type;
};

struct FuncT : public TypeT {
    std::function<TypeT *(std::vector<TypeT *> params)> body;
};

struct IntT : public TypeT {
    int value;
};