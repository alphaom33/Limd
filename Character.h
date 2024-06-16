#include <string>
#include <vector>
#include <iostream>
#include <utility>

#define TOKEN_DEF_LIST(E)    \
    E(LeftParenthesis, 0x1)  \
    E(RightParenthesis, 0x2) \
    E(Comma, 0x3)            \
    E(Number, 0x4)           \
    E(Identifier, 0x6)       \
    E(BackTick, 0x7)

enum Character
{
#define TOKEN_DEF_ENUM_DEF(name, value) name = value,
    TOKEN_DEF_LIST(TOKEN_DEF_ENUM_DEF)
};

static inline std::ostream &operator<<(std::ostream &o, Character e)
{
    switch (e)
    {
#define TOKEN_DEF_ENUM_CASE(name, value) \
    case name:                           \
        return o << #name;
        TOKEN_DEF_LIST(TOKEN_DEF_ENUM_CASE);

    default:
        return o << "unknown";
    }
}