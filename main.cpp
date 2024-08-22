#include "iostream"
#include "string"
#include "lexer.h"
#include "parser.h"
#include "ast.h"
#include "interpreter.h"
#include "readFile.h"


int main()
{
    std::string file = readFile("basic.limd");

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
