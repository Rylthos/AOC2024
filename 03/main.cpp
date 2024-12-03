#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <iterator>
#include <regex>
#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

typedef int puzzle_input;

std::string readLines(const std::string filename)
{
    std::ifstream file(filename, std::ios::in);
    if (!file.is_open())
    {
        std::cerr << "Failed to open file\n";
        exit(-1);
    }

    std::vector<std::string> lines;
    std::string line;
    while (std::getline(file, line))
    {
        lines.push_back(line);
    }

    file.close();

    std::stringstream ss;
    for (auto s : lines)
    {
        ss << s;
    }

    return ss.str();
}

// puzzle_input parseInput(std::vector<std::string> lines) {}

int64_t part1(std::string input)
{
    std::regex regex("mul\\(\\d+,\\d+\\)");
    auto word_begin = std::sregex_iterator(input.begin(), input.end(), regex);
    auto word_end = std::sregex_iterator();

    int64_t sum = 0;
    for (auto i = word_begin; i != word_end; i++)
    {
        std::string instr = i->str();
        std::string start = instr.substr(4);
        auto split = start.find(",");
        // int left = std::atoi(split.)
        int left = std::atoi(start.substr(0, split).c_str());
        int right = std::atoi(start.substr(split + 1).c_str());
        sum += left * right;
        // std::cout << left << " " << right << "\n";
    }

    return sum;
}

int64_t part2(std::string input)
{
    // std::cout << input << "\n";
    std::regex regex("(mul\\(\\d+,\\d+\\))|(do\\(\\))|(don't\\(\\))");
    // std::regex regex("do\\(\\)");
    auto word_begin = std::sregex_iterator(input.begin(), input.end(), regex);
    auto word_end = std::sregex_iterator();

    bool enabled = true;

    int64_t sum = 0;
    for (auto i = word_begin; i != word_end; i++)
    {
        std::string instr = i->str();
        // std::cout << instr << std::endl;
        bool isMul = instr[0] == 'm';

        if (isMul && enabled)
        {
            std::string start = instr.substr(4);
            auto split = start.find(",");
            // int left = std::atoi(split.)
            int left = std::atoi(start.substr(0, split).c_str());
            int right = std::atoi(start.substr(split + 1).c_str());
            sum += left * right;
            // std::cout << left << " " << right << "\n";
        }
        else
        {
            if (instr.length() == 4) // Do
            {
                enabled = true;
            }
            else
            {
                enabled = false;
            }
        }
    }

    return sum;
}

int main()
{
    // std::string input = readLines("Example.txt");
    // std::string input = readLines("Example2.txt");
    // std::cout << input << "\n";
    // std::string input = readLines("Input.txt");
    // auto parsed = parseInput(lines);

    std::cout << "Part 1: " << part1(input) << "\n";
    std::cout << "Part 2: " << part2(input) << "\n";

    std::cout << std::endl;

    return 0;
}
