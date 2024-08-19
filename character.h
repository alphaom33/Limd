#include <string>
#include <vector>
#include <iostream>
#include <utility>

#define TOKEN_DEF_LIST(E)    \
    E(LeftParenthesis, 0x1)  \
    E(RightParenthesis, 0x2) \
    E(Comma, 0x3)            \
    E(Number, 0x4)           \
    E(CharList, 0x5)         \
    E(SingleChar, 0x6)       \
    E(BackTickList, 0x7)     \
    E(EndBackTickList, 0x8)  \
    E(Identifier, 0x9)       \
    E(BackTick, 0xa)         \
    E(LeftBracket, 0xb)      \
    E(RightBracket, 0xc)     \
    E(Range, 0xd)

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