#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <unordered_map>
#include <vector>

typedef std::vector<std::string> puzzle_input;

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

void search_string(puzzle_input input, int x, int y, int& count, char search = 'X',
                   int searchDirX = -2, int searchDirY = -2)
{
    if (input[y][x] != search) return;

    char newSearch;
    switch (search)
    {
    case 'X':
        newSearch = 'M';
        break;
    case 'M':
        newSearch = 'A';
        break;
    case 'A':
        newSearch = 'S';
        break;
    case 'S':
        count += 1;
        return;
    }

    if (searchDirX == -2 && searchDirY == -2)
    {
        for (int i = -1; i <= 1; i++)
        {
            for (int j = -1; j <= 1; j++)
            {
                if (i == j && i == 0) continue;

                int newX = x + j;
                int newY = y + i;

                if (newY >= 0 && newY < input.size() && newX >= 0 && newX < input[newY].size())
                {
                    search_string(input, newX, newY, count, newSearch, j, i);
                }
            }
        }
    }
    else
    {
        int newX = x + searchDirX;
        int newY = y + searchDirY;

        if (newY >= 0 && newY < input.size() && newX >= 0 && newX < input[newY].size())
            search_string(input, newX, newY, count, newSearch, searchDirX, searchDirY);
    }
}

int64_t part1(puzzle_input input)
{
    int64_t count = 0;
    for (int i = 0; i < input.size(); i++)
    {
        for (int j = 0; j < input[i].size(); j++)
        {
            int sub_count = 0;
            search_string(input, j, i, sub_count);
            count += sub_count;
        }
    }
    return count;
}

int64_t part2(puzzle_input input)
{
    int64_t count = 0;
    for (int i = 1; i < input.size() - 1; i++)
    {
        for (int j = 1; j < input[i].size() - 1; j++)
        {
            char c = input[i][j];

            char topLeft = input[i - 1][j - 1];
            char topRight = input[i - 1][j + 1];
            char bottomLeft = input[i + 1][j - 1];
            char bottomRight = input[i + 1][j + 1];

            if (c == 'A')
            {
                if ((topLeft == 'M' && bottomRight == 'S') ||
                    (topLeft == 'S' && bottomRight == 'M'))
                {
                    if ((topRight == 'M' && bottomLeft == 'S') ||
                        (topRight == 'S' && bottomLeft == 'M'))
                        count += 1;
                }
            }
        }
    }
    return count;
}

int main()
{
    // std::vector<std::string> lines = readLines("Example.txt");
    std::vector<std::string> lines = readLines("Input.txt");

    int64_t output = part1(lines);
    std::cout << "Part 1: " << output << "\n";
    std::cout << "Part 2: " << part2(lines) << "\n";

    std::cout << std::endl;

    return 0;
}
