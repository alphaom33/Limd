#include "ast.h"
#include "lexer.h"
#pragma once
class Parser {
public:
    Parser(std::vector<Lexer::Token*> toParse);
    std::vector<ASTN*> Parse();
    std::vector<Lexer::Token*> countParenthesis();

private:
    std::vector<Lexer::Token*> toParse;
    int current;
};