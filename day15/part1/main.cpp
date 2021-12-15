#include <algorithm>
#include <array>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <queue>
#include <vector>

using PositionWithRisk = std::pair<std::int64_t, std::pair<std::int64_t, std::int64_t>>;

constexpr auto steps = std::array<std::pair<std::int64_t, std::int64_t>, 4>{
    std::pair{0, 1},
    std::pair{1, 0},
    std::pair{0, -1},
    std::pair{-1, 0}
};

std::vector<std::vector<std::int64_t>> load_file(const char* filepath)
{
    auto output = std::vector<std::vector<std::int64_t>>{};
    auto file = std::ifstream{filepath};

    auto input = std::string{};
    while (file >> input)
    {
        auto numbers = std::vector<std::int64_t>{};
        std::transform(input.begin(), input.end(), std::back_inserter(numbers),
                       [](const auto num) { return num - '0'; });
        output.emplace_back(numbers);
    }

    return output;
}

std::vector<std::pair<std::pair<std::int64_t, std::int64_t>, std::int64_t>> get_adjacent_costs(
    const std::vector<std::vector<std::int64_t>>& input,
    const std::vector<std::vector<bool>>& visited,
    const std::size_t column,
    const std::size_t row)
{
    auto adjacent_points = std::vector<std::pair<std::pair<std::int64_t, std::int64_t>, std::int64_t>>{};
    for (const auto& step : steps)
    {
        const auto y = column + step.first;
        const auto x = row + step.second;
        if ((x >= 0 && x < input[column].size()) && (y >= 0 && y < input.size()))
        {
            if (!visited[y][x])
            {
                adjacent_points.push_back({{y, x}, input[y][x]});
            }
        }
    }

    return adjacent_points;
}

std::int64_t find_shortest_path(std::vector<std::vector<std::int64_t>> input)
{
    auto to_visit =
        std::priority_queue<PositionWithRisk, std::vector<PositionWithRisk>, std::greater<PositionWithRisk>>{};
    auto visited = std::vector<std::vector<bool>>(input.size(), std::vector<bool>(input.front().size(), false));

    const auto start = PositionWithRisk{0, {0, 0}};
    const auto target = std::pair<std::int64_t, std::int64_t>{input.size() - 1, input.front().size() - 1};

    to_visit.push(start);

    while (!to_visit.empty())
    {
        const auto [current_cost, point] = to_visit.top();
        to_visit.pop();

        if (point == target)
        {
            return current_cost;
        }

        if (visited[point.first][point.second])
        {
            continue;
        }
        visited[point.first][point.second] = true;

        const auto adjacent_points = get_adjacent_costs(input, visited, point.first, point.second);
        for (const auto& adjacent_point : adjacent_points)
        {
            const auto& [neighbour, cost] = adjacent_point;

            const auto new_cost = current_cost + cost;
            to_visit.push({new_cost, neighbour});
        }
    }

    throw std::runtime_error("No path found");
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        throw std::runtime_error("Provide filepath");
    }

    auto input = load_file(argv[1]);
    const auto result = find_shortest_path(std::move(input));
    std::cout << result << std::endl;
}
