#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <optional>
#include <unordered_map>
#include <vector>

typedef std::vector<std::vector<int>> puzzle_input;

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
    std::vector<std::vector<int>> returnValue;
    for (std::string line : lines)
    {
        std::vector<int> values;

        size_t pos = line.find(" ");
        size_t initial = 0;
        while (pos != std::string::npos)
        {
            int value = std::atoi(line.substr(initial, pos - initial).c_str());
            values.push_back(std::atoi(line.substr(initial, pos).c_str()));
            initial = pos + 1;
            pos = line.find(" ", initial);
        }
        values.push_back(
            std::atoi(line.substr(initial, std::min(pos, line.size()) - initial).c_str()));
        returnValue.push_back(values);
    }
    return returnValue;
}

int64_t part1(puzzle_input input)
{
    size_t index = 0;
    int64_t sum = 0;
    for (auto level : input)
    {
        int current = level[0];
        int value = level[1];
        bool isIncreasing = value > current;
        int maxDiff = std::abs(value - current);
        bool isValid = true;
        if (maxDiff == 0 || maxDiff > 3) isValid = false;
        current = value;
        for (int i = 2; i < level.size() && isValid; i++)
        {
            value = level[i];
            int diff = value - current;
            maxDiff = std::max(std::abs(diff), maxDiff);

            if (std::abs(diff) == 0 || std::abs(diff) > 3)
            {
                isValid = false;
                break;
            }
            else if (isIncreasing && diff <= 0)
            {
                isValid = false;
                break;
            }
            else if (!isIncreasing && diff >= 0)
            {
                isValid = false;
                break;
            }

            current = value;
        }

        if (isValid && maxDiff > 0 && maxDiff <= 3)
        {
            sum += 1;
            // printf("%ld\n", index);
        }
        index += 1;
    }

    return sum;
}

bool isSafe(std::vector<int> values, bool hasRemoved = false)
{
    int current = values[0];
    int value = values[1];

    bool isIncreasing = value > current;
    int maxDiff = -1;
    bool isValid = true;

    for (int i = 1; i < values.size() && isValid; i++)
    {
        value = values[i];
        int diff = value - current;

        if (diff == 0 || std::abs(diff) > 3)
        {
            if (hasRemoved)
                isValid = false;
            else
            {
                for (int j = 0; j <= i; j++)
                {
                    std::vector<int> copy = values;
                    copy.erase(copy.begin() + j);
                    if (isSafe(copy, true)) return true;
                }
                return false;
            }
        }
        else if (isIncreasing && diff < 0)
        {
            if (hasRemoved)
                isValid = false;
            else
            {
                for (int j = 0; j <= i; j++)
                {
                    std::vector<int> copy = values;
                    copy.erase(copy.begin() + j);
                    if (isSafe(copy, true)) return true;
                }
                return false;
            }
        }
        else if (!isIncreasing && diff > 0)
        {
            if (hasRemoved)
                isValid = false;
            else
            {
                for (int j = 0; j <= i; j++)
                {
                    std::vector<int> copy = values;
                    copy.erase(copy.begin() + j);
                    if (isSafe(copy, true)) return true;
                }
                return false;
            }
        }

        maxDiff = std::max(std::abs(diff), maxDiff);

        current = value;
    }

    if (isValid && maxDiff >= 1 && maxDiff <= 3)
    {
        return true;
    }
    return false;
}

int64_t part2(puzzle_input input)
{
    int64_t sum = 0;
    int64_t index = 0;
    for (auto line : input)
    {
        if (isSafe(line)) sum += 1;
        index += 1;
    }

    return sum;
}

int main()
{
    // std::vector<std::string> lines = readLines("Example.txt");
    std::vector<std::string> lines = readLines("Input.txt");
    auto parsed = parseInput(lines);

    // for (auto i : parsed)
    // {
    //     for (auto j : i)
    //         printf("%d ", j);
    //     printf("\n");
    // }

    int64_t value = part1(parsed);
    std::cout << "Part 1: " << value << "\n";
    std::cout << "Part 2: " << part2(parsed) << "\n";

    std::cout << std::endl;

    return 0;
}
