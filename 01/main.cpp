#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <unordered_map>
#include <vector>

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

std::pair<std::vector<int>, std::vector<int>> parseInput(std::vector<std::string> lines)
{
    std::vector<int> left;
    std::vector<int> right;
    for (std::string line : lines)
    {
        int a, b;
        sscanf(line.c_str(), "%d   %d", &a, &b);
        left.push_back(a);
        right.push_back(b);
    }

    return std::make_pair(left, right);
}

uint64_t part1(std::pair<std::vector<int>, std::vector<int>> input)
{
    std::make_heap(input.first.begin(), input.first.end(), std::greater<>{});
    std::make_heap(input.second.begin(), input.second.end(), std::greater<>{});

    uint64_t sum = 0;
    while (input.first.size() > 0)
    {
        int minLeft, minRight;
        std::pop_heap(input.first.begin(), input.first.end(), std::greater<>{});
        std::pop_heap(input.second.begin(), input.second.end(), std::greater<>{});
        minLeft = input.first.back();
        minRight = input.second.back();
        input.first.pop_back();
        input.second.pop_back();

        int diff = std::abs(minLeft - minRight);
        sum += diff;
    }

    return sum;
}

uint64_t part2(std::pair<std::vector<int>, std::vector<int>> input)
{
    std::unordered_map<int, int> occurences;

    for (int i : input.second)
    {
        int currentCount = 0;
        if (occurences.count(i)) currentCount = occurences.at(i);

        occurences.insert_or_assign(i, currentCount += 1);
    }

    uint64_t sum = 0;
    for (int j : input.first)
    {
        int count = occurences.count(j) ? occurences.at(j) : 0;
        sum += count * j;
    }

    return sum;
}

int main()
{
    // std::vector<std::string> lines = readLines("Example.txt");
    std::vector<std::string> lines = readLines("Input.txt");
    auto parsed = parseInput(lines);

    std::cout << "Part 1: " << part1(parsed) << "\n";
    std::cout << "Part 2: " << part2(parsed) << "\n";

    std::cout << std::endl;

    return 0;
}
