#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>
#include <unordered_map>
#include <vector>

typedef std::pair<std::multimap<int, int>, std::vector<std::vector<int>>> puzzle_input;

std::vector<std::string> readLines(const std::string filename)
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

    return lines;
}

puzzle_input parseInput(std::vector<std::string> lines)
{
    bool parsingRules = true;
    std::multimap<int, int> rules;
    std::vector<std::vector<int>> pages;

    for (std::string line : lines)
    {
        if (line.length() == 0)
        {
            parsingRules = false;
            continue;
        }

        if (parsingRules)
        {
            int a = std::atoi(line.substr(0, 2).c_str());
            int b = std::atoi(line.substr(3).c_str());
            rules.emplace(a, b);
        }
        else
        {
            size_t previous = 0;
            auto comma = line.find(",");
            std::vector<int> input;
            while (comma != std::string::npos)
            {
                input.push_back(std::atoi(line.substr(previous, comma - previous).c_str()));
                previous = comma + 1;
                comma = line.find(",", previous);
            }
            input.push_back(std::atoi(line.substr(previous).c_str()));
            pages.push_back(input);
        }
    }

    // for (auto a : rules)
    // {
    //     std::cout << a.first << "|" << a.second << "\n";
    // }
    //
    // for (auto input : pages)
    // {
    //     for (int i : input)
    //         std::cout << i << ", ";
    //
    //     std::cout << "\n";
    // }

    return { rules, pages };
}

bool isValid(const std::vector<int>& input, const std::multimap<int, int>& rules,
             std::vector<std::pair<int, int>>* incorrect = nullptr, bool shouldFix = false)
{
    bool isValid = true;
    for (int i = 0; i < input.size() - 1; i++)
    {
        int value = input[i];
        for (int j = i + 1; j < input.size(); j++)
        {
            int check_value = input[j];
            auto begin = rules.lower_bound(check_value);
            auto end = rules.upper_bound(check_value);

            while (begin != end)
            {
                if (begin->second == value)
                {
                    isValid = false;

                    if (shouldFix)
                    {
                        incorrect->emplace_back(j, i);
                    }
                    else
                    {
                        return isValid;
                    }
                }
                begin++;
            }
        }
    }

    return isValid;
}

int64_t part1(puzzle_input input)
{
    int64_t sum = 0;
    for (auto line : input.second)
    {
        if (isValid(line, input.first))
        {
            // for (int i : line)
            //     std::cout << i << ", ";
            // std::cout << "\n";

            int midPoint = (line.size() - 1) / 2;
            sum += line.at(midPoint);
        }
    }
    return sum;
}

int64_t part2(puzzle_input input)
{
    int64_t sum = 0;
    for (auto line : input.second)
    {
        std::vector<std::pair<int, int>> fixed;
        if (!isValid(line, input.first, &fixed, true))
        {
            int a = line[fixed[0].first];
            int b = line[fixed[0].second];
            line[fixed[0].first] = b;
            line[fixed[0].second] = a;

            fixed.clear();
            while (!isValid(line, input.first, &fixed, true))
            {
                int a = line[fixed[0].first];
                int b = line[fixed[0].second];
                line[fixed[0].first] = b;
                line[fixed[0].second] = a;
                fixed.clear();
            }

            int midPoint = (line.size() - 1) / 2;
            sum += line.at(midPoint);
        }
    }

    return sum;
}

int main()
{
    // std::vector<std::string> lines = readLines("Example.txt");
    std::vector<std::string> lines = readLines("Input.txt");
    auto parsed = parseInput(lines);

    int64_t output1 = part1(parsed);
    int64_t output2 = part2(parsed);
    std::cout << "Part 1: " << output1 << "\n";
    std::cout << "Part 2: " << output2 << "\n";

    std::cout << std::endl;

    return 0;
}
