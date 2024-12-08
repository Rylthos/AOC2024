#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <unordered_map>
#include <vector>

typedef std::vector<std::pair<int64_t, std::vector<int64_t>>> puzzle_input;

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
    puzzle_input returnValue;

    for (auto line : lines)
    {
        auto pos = line.find(":");
        int64_t target = std::atol(line.substr(0, pos).c_str());

        std::vector<int64_t> values;

        pos = line.find(" ", pos);
        while (pos != std::string::npos)
        {
            auto next = line.find(" ", pos + 1);
            int64_t value = std::atol(line.substr(pos, pos - next).c_str());
            values.push_back(value);

            pos = next;
        }

        returnValue.push_back({ target, values });
    }

    return returnValue;
}

bool is_contained(int64_t original, int64_t check, int64_t& shortened)
{
    while (original > 0 && check > 0)
    {
        int64_t check_original = original % 10;
        int64_t check_check = check % 10;

        if (check_original != check_check) return false;

        original /= 10;
        check /= 10;
    }
    shortened = original;

    return true;
}

bool valid(int64_t target, std::vector<int64_t> values, bool concatenation = false)
{
    if (values.size() == 1) return target == values[0];
    if (values.size() == 0) return false;

    int64_t last = values[values.size() - 1];

    // Addition
    {
        int64_t new_target = target - last;
        auto new_values = values;
        new_values.erase(new_values.end() - 1);

        if (new_target >= 0)
        {

            if (valid(new_target, new_values, concatenation)) return true;
        }
    }

    // Multiplication
    {
        if (target % last == 0)
        {

            int64_t new_target = target / last;
            auto new_values = values;
            new_values.erase(new_values.end() - 1);

            if (valid(new_target, new_values, concatenation)) return true;
        }
    }

    // Concatenation
    if (concatenation)
    {
        int64_t new_target;
        if (is_contained(target, last, new_target))
        {

            auto new_values = values;
            new_values.erase(new_values.end() - 1);

            if (valid(new_target, new_values, concatenation)) return true;
        }
    }

    return false;
}

int64_t puzzle(puzzle_input input, bool concatenation)
{
    int64_t sum = 0;
    for (auto i : input)
    {
        if (valid(i.first, i.second, concatenation))
        {
            // std::cout << i.first << "\n";
            sum += i.first;
        }
    }
    return sum;
}

int main()
{
    // std::vector<std::string> lines = readLines("Example.txt");
    std::vector<std::string> lines = readLines("Input.txt");
    auto parsed = parseInput(lines);

    std::cout << "Part 1: " << puzzle(parsed, false) << "\n";
    std::cout << "Part 2: " << puzzle(parsed, true) << "\n";

    std::cout << std::endl;

    return 0;
}
