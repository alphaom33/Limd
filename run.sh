if [[ "$OSTYPE" == "msys" ]]; then
    rm a.exe
    g++ main.cpp lexer.cpp parser.cpp interpreter.cpp -std=c++23
    ./a.exe
else
    rm a.out
    g++-13 main.cpp lexer.cpp parser.cpp interpreter.cpp readFile.cpp -std=c++23
    ./a.out
fi