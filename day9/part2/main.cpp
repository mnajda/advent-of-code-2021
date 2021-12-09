#include <algorithm>
#include <array>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <queue>
#include <vector>

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

std::vector<std::pair<std::int64_t, std::int64_t>> get_adjacent_points(
    const std::vector<std::vector<std::int64_t>>& input,
    const std::size_t column,
    const std::size_t row)
{
    auto adjacent_points = std::vector<std::pair<std::int64_t, std::int64_t>>{};
    for (const auto& step: steps)
    {
        const auto y = column + step.first;
        const auto x = row + step.second;
        if ((x >= 0 and x < input[column].size()) and (y >= 0 and y < input.size()))
        {
            adjacent_points.emplace_back(y, x);
        }
    }

    return adjacent_points;
}

std::vector<std::int64_t> get_adjacent_values(
    const std::vector<std::vector<std::int64_t>>& input,
    const std::vector<std::pair<std::int64_t, std::int64_t>>& adjacent_points)
{
    auto adjacent_values = std::vector<std::int64_t>{};
    for (const auto& point : adjacent_points)
    {
        adjacent_values.push_back(input[point.first][point.second]);
    }

    return adjacent_values;
}

std::vector<std::int64_t> create_basin(
    const std::vector<std::vector<std::int64_t>>& input,
    const std::pair<std::int64_t, std::int64_t>& low_point)
{
    auto basin = std::vector<std::int64_t>{};
    auto to_visit = std::queue<std::pair<std::int64_t, std::int64_t>>{};
    auto visited = std::vector<std::vector<bool>>(input.size(), std::vector<bool>(input.front().size(), false));
    to_visit.push(low_point);
    visited[low_point.first][low_point.second] = true;

    while (not to_visit.empty())
    {
        const auto point = to_visit.front();
        to_visit.pop();

        const auto height = input[point.first][point.second];
        if (height < 9)
        {
            basin.push_back(height);
            const auto adjacent_points = get_adjacent_points(input, point.first, point.second);
            for (const auto& adjacent_point : adjacent_points)
            {
                if (not visited[adjacent_point.first][adjacent_point.second])
                {
                    visited[adjacent_point.first][adjacent_point.second] = true;
                    to_visit.push(adjacent_point);
                }
            }
        }
    }

    return basin;
}

std::size_t solve(std::vector<std::vector<std::int64_t>> input)
{
    auto low_points = std::vector<std::pair<std::int64_t, std::int64_t>>{};
    for (auto column = 0U; column < input.size(); ++column)
    {
        for (auto row = 0U; row < input[column].size(); ++row)
        {
            const auto height = input[column][row];
            const auto adjacent_points = get_adjacent_points(input, column, row);
            const auto adjacent_values = get_adjacent_values(input, adjacent_points);

            const auto is_low_point =
                std::all_of(adjacent_values.begin(), adjacent_values.end(),
                            [height](const auto adjacent) { return height < adjacent; });
            if (is_low_point)
            {
                low_points.emplace_back(column, row);
            }
        }
    }

    auto basin_sizes = std::vector<std::size_t>{};
    std::transform(
        low_points.begin(), low_points.end(), std::back_inserter(basin_sizes),
        [&input](const auto& low_point) { return create_basin(input, low_point).size(); });
    std::sort(basin_sizes.begin(), basin_sizes.end(), std::greater<int>());

    return basin_sizes.at(0) * basin_sizes.at(1) * basin_sizes.at(2);
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        throw std::runtime_error("Provide filepath");
    }

    auto input = load_file(argv[1]);
    const auto result = solve(std::move(input));
    std::cout << result << std::endl;
}
