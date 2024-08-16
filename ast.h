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

struct LambdaN : public ASTN
{
    std::vector<ASTN *> parameters;
    std::vector<ASTN *> toRun;
};

struct IntN : public ASTN
{
    int value;
};

struct BoolN : public ASTN
{
    bool value;
};

struct StringN : public ASTN
{
    std::string value;
};

struct ListN : public ASTN
{
    std::vector<ASTN *> values;
};

struct RangeN : public ASTN
{
    ASTN *start;
    ASTN *end;
};

struct UnevaluatedN : public ASTN
{
    std::string value;
};

static inline std::ostream &operator<<(std::ostream &o, ASTN *e);
static inline std::ostream &operator<<(std::ostream &o, VarNameN *e);
static inline std::ostream &operator<<(std::ostream &o, FuncCallN *e);
static inline std::ostream &operator<<(std::ostream &o, IntN *e);
static inline std::ostream &operator<<(std::ostream &o, BoolN *e);
static inline std::ostream &operator<<(std::ostream &o, StringN *e);
static inline std::ostream &operator<<(std::ostream &o, ListN *e);
static inline std::ostream &operator<<(std::ostream &o, RangeN *e);
static inline std::ostream &operator<<(std::ostream &o, UnevaluatedN *e);
static inline std::ostream &operator<<(std::ostream &o, LambdaN *e);

static inline std::ostream &operator<<(std::ostream &o, ASTN *e)
{
    o << "{ " << e->op << std::flush;
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
    {
        IntN *a = static_cast<IntN *>(e);
        o << a;
    }
    break;
    case BoolImmediate:
    {
        BoolN *a = static_cast<BoolN *>(e);
        o << a;
    }
    break;
    case Unevaluated:
    {
        UnevaluatedN *a = static_cast<UnevaluatedN *>(e);
        o << a;
    }
    break;
    case StringImmediate:
    {
        StringN *a = static_cast<StringN *>(e);
        o << a;
    }
    break;
    case ListImmediate:
    {
        ListN *a = static_cast<ListN *>(e);
        o << a;
    }
    break;
    case RangeList:
    {
        RangeN *a = static_cast<RangeN *>(e);
        o << a;
    }
    break;
    case Lambda:
    {
        LambdaN *a = static_cast<LambdaN *>(e);
        o << a;
    }
    break;
    }
    return o << " }" << std::flush;
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
    return o << "]";
}

static inline std::ostream &operator<<(std::ostream &o, IntN *e)
{
    return o << ", " << e->value;
}

static inline std::ostream &operator<<(std::ostream &o, BoolN *e)
{
    return o << ", " << (e->value ? "true" : "false");
}

static inline std::ostream &operator<<(std::ostream &o, StringN *e)
{
    return o << ", \"" << e->value << "\"";
}

static inline std::ostream &operator<<(std::ostream &o, UnevaluatedN *e)
{
    return o << ", " << e->value;
}

static inline std::ostream &operator<<(std::ostream &o, ListN *e)
{
    o << ", ";

    o << "[ ";
    for (ASTN *a : e->values) {
        o << a << ", ";
    }
    return o << " ]";
}

static inline std::ostream &operator<<(std::ostream &o, LambdaN *e)
{
    o << " [ ";
    for (ASTN *a : e->parameters) {
        o << a << ", ";
    }
    o << "], ";

    o << "[ ";
    for (ASTN *a : e->toRun) {
        o << a << ", ";
    }
    return o << "]";
}

static inline std::ostream &operator<<(std::ostream &o, RangeN *e)
{
    return o << " start: " << e->start << " .. end: " << e->end << std::flush;
}