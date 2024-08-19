#include <string>
#include <vector>
#include <iostream>
#include <utility>

#define OPERATION_DEF_LIST(E) \
    E(FuncCall, 0x1)          \
    E(Lambda, 0x2)            \
    E(VarName, 0x3)           \
    E(IntImmediate, 0x4)      \
    E(BoolImmediate, 0x5)     \
    E(CharImmediate, 0x6)     \
    E(StringImmediate, 0x7)   \
    E(ListImmediate, 0x8)     \
    E(RangeList, 0x9)         \
    E(Unevaluated, 0xa)

enum Operation
{
#define OPERATION_DEF_ENUM_DEF(name, value) name = value,
    OPERATION_DEF_LIST(OPERATION_DEF_ENUM_DEF)
};

static inline std::ostream &operator<<(std::ostream &o, Operation e)
{
    switch (e)
    {
#define OPERATION_DEF_ENUM_CASE(name, value) \
    case name:                               \
        return o << #name;
        OPERATION_DEF_LIST(OPERATION_DEF_ENUM_CASE);

    default:
        return o << "unknown";
    }
}