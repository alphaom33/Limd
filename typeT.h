#include "Type.h"
#include "functional"
#include "map"
#pragma once
struct TypeT {
    Type type;
};

typedef struct Scope
{
    Scope *parent;
    std::map<std::string, TypeT *> vars;
} Scope;

struct FuncT : public TypeT {
    std::function<TypeT *(Scope *scope, std::vector<TypeT *> params)> body;
};

struct IntT : public TypeT {
    int value;
};

struct StringT : public TypeT {
    std::string value;
};

struct ListT : public TypeT {
    std::vector<TypeT *> values;
};

static inline std::ostream &operator<<(std::ostream &o, TypeT *e) {
    o << "{ " << e->type << ", ";
    switch (e->type)
    {
    case Int:
        o << ((IntT *)e)->value;
        break;
    case String:
        o << ((StringT *)e)->value;
        break; 
    case List:
        o << "[ ";
        for (TypeT *t : ((ListT *)e)->values) {
            o << t << ", ";
        }
        o << "]";
        break;
    }
    return o << " }";
}