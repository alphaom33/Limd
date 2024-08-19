#include "iostream"
#include "filesystem"
#include "fstream"
#include "string"
#include "lexer.h"
#include "parser.h"
#include "ast.h"
#include "interpreter.h"

std::string readFile(std::string name);

int main()
{
    std::string file = readFile("utils.limd");

    Lexer *lexer = new Lexer(file);

    for (Lexer::Token *t : *lexer->lex())
    {
        std::cout << "{" << t->character << ", \"" << t->value << "\"}" << ", ";
    }

    std::cout << "\n";
    std::cout << std::endl;

    Parser *parser = new Parser(*lexer->lex());
    for (ASTN *a : parser->Parse()) {
        std::cout << a << std::endl;
    }

    std::cout << std::endl;

    Interpreter *interpreter = new Interpreter(parser->Parse());
    for (std::pair p : interpreter->Interpret().vars) {
        std::cout << p.first << (TypeT*)p.second << ", ";
    }

    std::cout << std::endl;
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