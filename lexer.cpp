#include <string>
#include <vector>
#include <iostream>
#include <utility>
#include "lexer.h"

std::vector<Lexer::Token *> *Lexer::lex()
{
    std::vector<Token *> *tokens = new std::vector<Token *>;
    for (current = 0; current < toLex.length(); current++)
    {
        switch (toLex[current])
        {
        case '(':
            tokens->push_back(new Token(LeftParenthesis, "("));
            break;
        case ')':
            tokens->push_back(new Token(RightParenthesis, ")"));
            break;
        case ',':
            tokens->push_back(new Token(Comma, ","));
            break;
        case '`':
            if (toLex[++current] == '(') {
            } else {
                tokens->push_back(new Token(BackTick, lexIdentifier()->value));
            }
            break;
        default:
        {
            if (isdigit(toLex[current]))
            {
                tokens->push_back(lexNumber());
            }
            else if (!notIdentifierAble.contains(toLex[current]))
            {
                tokens->push_back(lexIdentifier());
            }
        }
        break;
        }
    }
    return tokens;
}

Lexer::Token *Lexer::lexNumber()
{
    if (toLex[current] == '0')
    {
        switch (toLex[current + 1])
        {
        case 'b':
            return lexBinary();
            break;
        case 'x':
            return lexHex();
            break;
        }
    }
    return lexDecimal();
}

class ExceptionNotImplemented : std::exception {

};

Lexer::Token *Lexer::lexBinary()
{
    throw ExceptionNotImplemented();
}

Lexer::Token *Lexer::lexHex()
{
    throw ExceptionNotImplemented();
}

Lexer::Token *Lexer::lexDecimal()
{
    std::string number = std::string() + toLex[current];
    while (isdigit(toLex[current + 1]) || toLex[current + 1] == '.') {
        number.push_back(toLex[current + 1]);
        current++;
    }
    return new Token(Number, number);
}

Lexer::Token *Lexer::lexIdentifier()
{
    std::string identifier = std::string() + toLex[current];
    while (!notIdentifierAble.contains(toLex[current + 1]) && current < toLex.length()) {
        identifier.push_back(toLex[current + 1]);
        current++;
    }
    return new Token(Identifier, identifier);
}