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

            auto parameters = std::vector<ASTN *>();
            while (toParse[current]->character != RightParenthesis)
            {
                auto tokens = std::vector<Lexer::Token *>();
                for (current++; toParse[current]->character != Comma && toParse[current]->character != RightParenthesis; current++)
                {
                    if (toParse[current]->character == LeftParenthesis)
                    {
                        auto a = countParenthesis();
                        tokens.insert(tokens.end(), a.begin(), a.end());
                    }
                    tokens.push_back(toParse[current]);
                }
                parameters.push_back((new Parser(tokens))->Parse()[0]);
            }

            out.push_back(new FuncCallN{
                FuncCall,
                new VarNameN{
                    VarName,
                    name},
                parameters});
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