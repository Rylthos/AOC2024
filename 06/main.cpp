#include <algorithm>
#include <cassert>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <vector>

struct Vec2 {
    int x;
    int y;
    int direction = 0;
};
bool operator==(const Vec2& lhs, const Vec2& rhs)
{
    return (lhs.x == rhs.x) && (lhs.y == rhs.y) && (lhs.direction == rhs.direction);
}

struct Hash {
    size_t operator()(const Vec2& x) const
    {
        size_t hash = (std::hash<int>()(x.x) << 8) ^ (std::hash<int>()(x.y));
        return hash;
    }
};
;

typedef std::pair<Vec2, std::vector<std::string>> puzzle_input;

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
    Vec2 origin;
    size_t y = 0;
    for (std::string line : lines)
    {
        auto pos = line.find("^");
        if (pos != std::string::npos)
        {
            origin.y = y;
            origin.x = pos;
            break;
        }
        y++;
    }

    return { origin, lines };
}

Vec2 getDirection(int direction)
{
    Vec2 offset;
    offset.x = (direction == 1) ? 1 : ((direction == 3) ? -1 : 0);
    offset.y = (direction == 0) ? -1 : ((direction == 2) ? 1 : 0);
    return offset;
}

std::pair<bool, std::unordered_set<Vec2, Hash>> getVisited(puzzle_input input, bool getPath = false)
{
    int direction = 0;
    Vec2 currentPos = input.first;
    int width = input.second.at(0).size();
    int height = input.second.size();
    std::unordered_set<Vec2, Hash> visited;
    std::unordered_set<Vec2, Hash> path;

    visited.insert(currentPos);

    // if (getPath)
    // {
    //     currentPos.direction = direction;
    //     path.insert(currentPos);
    // }
    currentPos.direction = 0;
    while (true)
    {
        Vec2 offset = getDirection(direction);

        Vec2 nextPos;
        nextPos.x = currentPos.x + offset.x;
        nextPos.y = currentPos.y + offset.y;

        if (nextPos.x < 0 || nextPos.x >= width || nextPos.y < 0 || nextPos.y >= height) break;

        char character = input.second.at(nextPos.y).at(nextPos.x);
        bool encodeDirection = true;
        switch (character)
        {
        case '.':
        case '^':
            break;
        case '#':
        case 'O':
            {
                nextPos = currentPos;

                direction = (direction + 1) % 4;
                encodeDirection = false;
                break;
            }
        default:
            assert(false);
        }

        visited.insert(nextPos);

        if (getPath && encodeDirection)
        {
            currentPos.direction = direction;
            // printf("X: %d, Y: %d, D: %d\n", currentPos.x, currentPos.y, currentPos.direction);
            if (path.contains(currentPos))
                return { true, path };
            else
                path.insert(currentPos);
        }

        currentPos = nextPos;
    }
    if (getPath)
        return { false, path };
    else
        return { false, visited };
}

int64_t part1(puzzle_input input) { return getVisited(input).second.size(); }

int64_t part2(puzzle_input input)
{
    auto path = getVisited(input, true);

    int width = input.second.at(0).size();
    int height = input.second.size();

    std::unordered_set<Vec2, Hash> values;

    for (auto v : path.second)
    {
        Vec2 direction = getDirection(v.direction);
        Vec2 nextPos;
        nextPos.x = v.x + direction.x;
        nextPos.y = v.y + direction.y;

        if (nextPos.x < 0 || nextPos.x >= width || nextPos.y < 0 || nextPos.y >= height) continue;

        char saved = input.second.at(nextPos.y).at(nextPos.x);
        if (saved != '.') continue;

        input.second.at(nextPos.y).at(nextPos.x) = 'O';

        if (!values.contains(nextPos))
        {
            if (getVisited(input, true).first)
            {
                values.insert(nextPos);
            }
        }

        input.second.at(nextPos.y).at(nextPos.x) = saved;
    }
    return values.size();
}

int main()
{
    // std::vector<std::string> lines = readLines("Example.txt");
    std::vector<std::string> lines = readLines("Input.txt");
    auto parsed = parseInput(lines);

    int64_t p1 = part1(parsed);
    int64_t p2 = part2(parsed);

    std::cout << "Part 1: " << p1 << "\n";
    std::cout << "Part 2: " << p2 << "\n";

    std::cout << std::endl;

    return 0;
}
