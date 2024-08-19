#include <string>
#include <vector>
#include <iostream>
#include <utility>

#define TYPE_DEF_LIST(E) \
    E(Function, 0x1)     \
    E(Int, 0x2)          \
    E(Bool, 0x3)          \
    E(String, 0x4)       \
    E(Char, 0x5)       \
    E(List, 0x6)

enum Type
{
#define TYPE_DEF_ENUM_DEF(name, value) name = value,
    TYPE_DEF_LIST(TYPE_DEF_ENUM_DEF)
};

static inline std::ostream &operator<<(std::ostream &o, Type e)
{
    switch (e)
    {
#define TYPE_DEF_ENUM_CASE(name, value) \
    case name:                          \
        return o << #name;
        TYPE_DEF_LIST(TYPE_DEF_ENUM_CASE);

    default:
        return o << "unknown";
    }
}