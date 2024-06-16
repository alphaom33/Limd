#include "parser.h"

Parser::Parser(std::vector<Lexer::Token *> toParse)
{
    this->toParse = toParse;
}

std::vector<ASTN *> Parser::Parse()
{
    auto out = std::vector<ASTN *>();

    for (current = 0; current < toParse.size(); current++)
    {
        switch (toParse[current]->character)
        {
        case LeftParenthesis:
        {
            std::string name = toParse[++current]->value;

            auto a = countParenthesis();
            out.push_back(new FuncCallN{
                FuncCall,
                new VarNameN{
                    VarName,
                    name},
                (new Parser(std::vector(a.begin() + 1, a.end())))->Parse()});
        }
        break;
        case BackTickList:
        {
            auto a = countParenthesis();
            out.push_back(new ListN{
                ListImmediate,
                (new Parser(std::vector(a.begin() + 1, a.end())))->Parse()});
        }
        break;
        case Number:
        {
            if (!toParse[current]->value.contains("."))
            {
                out.push_back(new IntN{
                    IntImmediate,
                    std::stoi(toParse[current]->value, 0, 10)});
            }
        }
        break;
        case Identifier:
            out.push_back(new VarNameN{
                VarName,
                toParse[current]->value});
            break;
        case BackTick:
            out.push_back(new UnevaluatedN{
                Unevaluated,
                toParse[current]->value});
            break;
        case CharList:
            out.push_back(new StringN{
                StringImmediate,
                toParse[current]->value});
            break;
        }
    }
    return out;
}

std::vector<Lexer::Token *> Parser::countParenthesis()
{
    auto out = std::vector<Lexer::Token *>();
    out.push_back(toParse[current]);

    int numParens = 1;
    for (current++; numParens > 0; current++)
    {
        switch (toParse[current]->character)
        {
        case BackTickList:
            numParens++;
            break;
        case LeftParenthesis:
            numParens++;
            break;
        case RightParenthesis:
            numParens--;
            break;
        }
        out.push_back(toParse[current]);
    }
    current--;

    return out;
}