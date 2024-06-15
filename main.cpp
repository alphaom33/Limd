#include "iostream"
#include "filesystem"
#include "fstream"
#include "string"
#include "lexer.h"
#include "parser.h"
#include "ast.h"

std::string readFile(std::string name);

int main()
{
    std::string file = readFile("basic.limd");

    Lexer *lexer = new Lexer(file);

    for (Lexer::Token *t : *lexer->lex())
    {
        std::cout << "{" << t->character << ", \"" << t->value << "\"}" << ", ";
    }

    std::cout << "\n";

    Parser *parser = new Parser(*lexer->lex());
    for (ASTN *a : parser->Parse()) {
        std::cout << a << "\n";
    }
}

std::string readFile(std::string name)
{
    std::ifstream mainFile(name);
    std::string tmpString;
    std::string outString;
    while (std::getline(mainFile, tmpString))
    {
        outString += tmpString;
    }
    return outString;
}