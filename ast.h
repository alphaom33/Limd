#include "Operation.h"

#pragma once
struct ASTN
{
    Operation op;
};

struct VarNameN : public ASTN
{
    std::string name;
};

struct FuncCallN : public ASTN
{
    VarNameN *name;
    std::vector<ASTN *> parameters;
};

struct IntN : public ASTN
{
    int value;
};

static inline std::ostream &operator<<(std::ostream &o, ASTN *e);
static inline std::ostream &operator<<(std::ostream &o, VarNameN *e);
static inline std::ostream &operator<<(std::ostream &o, FuncCallN *e);
static inline std::ostream &operator<<(std::ostream &o, IntN *e);

static inline std::ostream &operator<<(std::ostream &o, ASTN *e)
{
    o << "{ " << e->op;
    switch (e->op)
    {
    case FuncCall:
    {
        FuncCallN *a = static_cast<FuncCallN *>(e);
        o << a;
    }
    break;
    case VarName:
    {
        VarNameN *a = static_cast<VarNameN *>(e);
        o << a;
    }
    break;
    case IntImmediate:
        IntN *a = static_cast<IntN *>(e);
        o << a;
        break;
    }
    return o << "}";
}

static inline std::ostream &operator<<(std::ostream &o, VarNameN *e)
{
    return o << ", \"" << e->name << "\"";
}

static inline std::ostream &operator<<(std::ostream &o, FuncCallN *e)
{
    o << ", ";

    ASTN *a = e->name;
    o << a;
    o << ", ";

    o << "[ ";
    for (ASTN *a : e->parameters)
    {
        o << a << ", ";
    }
    return o << " ]";
}

static inline std::ostream &operator<<(std::ostream &o, IntN *e)
{
    return o << ", " << e->value;
}