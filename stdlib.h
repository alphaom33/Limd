#include "interpreter.h"
#pragma once

class VarNotFound : public std::exception
{
    public:
    VarNotFound() {
        std::cout << "asdf";
    }
};

class TooManyArgs : public std::exception
{
};

TypeT *Equals(Scope *current, std::vector<TypeT *> params) {
    bool out = false;
    if (params[0]->type == Bool) {
        for (TypeT *t : params) {
            out = out == ((BoolT *)t)->value;
        }
    } else if (params[0]->type == Char) {
        out = ((CharT *)params[0])->value == ((CharT *)params[1])->value;
    } else if (params[0]->type == Int) {
        out = ((IntT *)params[0])->value == ((IntT *)params[1])->value;
    }
    return new BoolT{
        Bool,
        out
    };
}

TypeT *And(Scope *current, std::vector<TypeT *> params) {
    bool out = false;
    for (TypeT *t : params) {
        out = out && ((BoolT *)t)->value;
    }
    return new BoolT{
        Bool,
        out
    };
}

TypeT *LessThan(Scope *current, std::vector<TypeT *> params) {
    if (params.size() > 2) {
        throw TooManyArgs();
    }

    return new BoolT{
        Bool,
        ((IntT *)params[0])->value < ((IntT *)params[1])->value
    };
}

TypeT *GreaterThan(Scope *current, std::vector<TypeT *> params) {
    if (params.size() > 2) {
        throw TooManyArgs();
    }

    return new BoolT{
        Bool,
        ((IntT *)params[0])->value > ((IntT *)params[1])->value
    };
}

TypeT *If(Scope *current, std::vector<TypeT *> params) {
    for (int i = 0; i < params.size(); i += 2) {
        if (params[i]->type != Bool) {
            return ((FuncT *)params[i])->body(current, std::vector<TypeT *>());
        }

        if (((BoolT*)params[i])->value) {
            return ((FuncT *)params[i + 1])->body(current, std::vector<TypeT *>());
        }
    }

    return new TypeT{};
}

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

Scope *GetThing(Scope *start, std::string ack) {
        Scope *currenty = start;
        while (currenty != nullptr) {
            if (currenty->vars.contains(ack)) {
                return currenty;
            }
            currenty = currenty->parent;
        }
        return nullptr;
}

TypeT *Set(Scope *current, std::vector<TypeT *> params)
{
    std::string ack = "";
    for (TypeT *t : ((ListT *)params[0])->values) {
        ack += ((CharT *)t)->value;
    }

    Scope *maybe = GetThing(current, ack);
    if (maybe != nullptr) {
        maybe->vars[ack] = params[1];
    } else {
        current->vars[ack] = params[1];
    }
    return params[1];
}

TypeT *Print(Scope *current, std::vector<TypeT *> params)
{
    for (TypeT *t : params)
    {
        std::cout << t;
    }
    std::cout << std::flush;
    return new TypeT{};
}

TypeT *Println(Scope *current, std::vector<TypeT *> params)
{
    for (TypeT *t : params)
    {
        std::cout << t;
    }
    std::cout << std::endl;
    return new TypeT{};
}


TypeT *In(Scope* current, std::vector<TypeT *> params) {
    std::string a;
    std::cin >> a;
    auto out = std::vector<TypeT *>();
    for (const char c : a) {
        out.push_back(new CharT{
            Char,
            c
        });
    }
    return new ListT{
        String,
        out
    };
}

TypeT *Len(Scope *current, std::vector<TypeT *> params) {
    return new IntT{
        Int,
        (int)((ListT *)params[0])->values.size()
    };
}

TypeT *Pass(Scope *current, std::vector<TypeT *> params) {
    return current->vars["return"];
}

TypeT *Nth(Scope *current, std::vector<TypeT *> params) {
    return ((ListT *)params[0])->values[((IntT *)params[1])->value];
}

TypeT *SetNth(Scope *current, std::vector<TypeT *> params) {
    ((ListT *)params[0])->values[((IntT *)params[1])->value] = params[2];
    return params[0];
}

std::map<std::string, TypeT *> initialScope = {
    std::pair<std::string, TypeT *>(
        std::string("=="),
        new FuncT{
            Function,
            Equals}),
    std::pair<std::string, TypeT *>(
        std::string("&&"),
        new FuncT{
            Function,
            And}),
    std::pair<std::string, TypeT *>(
        std::string("<"),
        new FuncT{
            Function,
            LessThan}),
    std::pair<std::string, TypeT *>(
        std::string(">"),
        new FuncT{
            Function,
            GreaterThan}),
    std::pair<std::string, TypeT *>(
        std::string("if"),
        new FuncT{
            Function,
            If}),
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
    std::pair<std::string, TypeT *>(
        std::string("println"),
        new FuncT{
            Function,
            Println}),
    std::pair<std::string, TypeT*>(
        std::string("in"),
        new FuncT{
            Function,
            In}),
    std::pair<std::string, TypeT*>(
        std::string("len"),
        new FuncT{
            Function,
            Len}),
    std::pair<std::string, TypeT*>(
        std::string("pass"),
        new FuncT{
            Function,
            Pass}),
    std::pair<std::string, TypeT*>(
        std::string("nth"),
        new FuncT{
            Function,
            Nth}),
    std::pair<std::string, TypeT*>(
        std::string("setNth"),
        new FuncT{
            Function,
            SetNth}),
};