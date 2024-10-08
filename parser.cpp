#include "parser.h"
#include "memory"
#include "lexer.h"
#include "ranges"
#include "readFile.h"

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
        case LeftBracket:
        {
            auto parameters = std::vector<Lexer::Token *>();
            for (current++; toParse[current]->character != RightBracket; current++)
            {
                parameters.push_back(toParse[current]);
            }
            
            current++;
            auto functions = countParenthesis();
            out.push_back(new LambdaN{
                Lambda,
                (new Parser(parameters))->Parse(),
                (new Parser(std::vector(functions.begin() + 1, functions.end())))->Parse()
            });
        }
        break;
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
            auto b = (new Parser(std::vector(a.begin() + 1, a.end())))->Parse();
            if (b[0]->op == RangeList) {
                out.push_back(b[0]);
            } else {
                out.push_back(new ListN{
                    ListImmediate,
                    b});
            }
        }
        break;
        case Range:
        {
            auto tmp = *(out.end() - 1);
            auto a = countParenthesis();
            out.pop_back();
            out.push_back(new RangeN{
                RangeList,
                tmp,
                (new Parser(std::vector(a.begin() + 1, a.end())))->Parse()[0]
            });
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
            if (toParse[current]->value == "true" || toParse[current]->value == "false") {
                out.push_back(new BoolN{
                    BoolImmediate,
                    toParse[current]->value == "true"
                });
                break;
            } else if (toParse[current]->value == "import") {
                auto a = (new Lexer(readFile(toParse[++current]->value + ".limd")))->lex();
                toParse.insert(toParse.begin() + current + 1, a->begin(), a->end());
                break;
            }

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
        case SingleChar:
            out.push_back(new CharN{
                CharImmediate,
                *toParse[current]->value.begin()});
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