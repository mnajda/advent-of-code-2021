#include <algorithm>
#include <array>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
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

std::vector<std::int64_t> get_adjacent(
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

    auto adjacent_values = std::vector<std::int64_t>{};
    for (const auto& point : adjacent_points)
    {
        adjacent_values.push_back(input[point.first][point.second]);
    }

    return adjacent_values;
}

std::int64_t solve(std::vector<std::vector<std::int64_t>> input)
{
    auto result = std::int64_t{};
    for (auto column = 0U; column < input.size(); ++column)
    {
        for (auto row = 0U; row < input[column].size(); ++row)
        {
            const auto height = input[column][row];
            const auto adjacent_values = get_adjacent(input, column, row);

            const auto is_low_point =
                std::all_of(adjacent_values.begin(), adjacent_values.end(),
                            [height](const auto adjacent) { return height < adjacent; });
            if (is_low_point)
            {
                result += (height + 1);
            }
        }
    }

    return result;
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        throw std::runtime_error("Provide filepath");
    }

    const auto input = load_file(argv[1]);
    const auto result = solve(std::move(input));
    std::cout << result << std::endl;
}
