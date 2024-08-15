#include <string>
#include <vector>
#include <iostream>
#include <utility>
#include "character.h"
#include <array>

#pragma once
class Lexer {
public:
    struct Token {
        Token(Character character, std::string value) {
            this->character = character;
            this->value = value;
        }

        Character character;
        std::string value;
    };

    Lexer(std::string toLex) {
        this->toLex = toLex;
    }

    std::vector<Token*> *lex();

private:
    Token *lexNumber();
    Token *lexBinary();
    Token *lexHex();
    Token *lexDecimal();
    Token *lexIdentifier();

    int current;
    std::string toLex;
    std::string notIdentifierAble = " ,()[]";
    std::array<std::string, 2> keywords = {"true", "false"};
    
};