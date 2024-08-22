#include "readFile.h"
#include "filesystem"
#include "fstream"

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