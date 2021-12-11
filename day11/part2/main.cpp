#include <algorithm>
#include <array>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <queue>
#include <vector>

constexpr auto steps = std::array<std::pair<std::int64_t, std::int64_t>, 8>{
    std::pair{0, 1},
    std::pair{1, 0},
    std::pair{0, -1},
    std::pair{-1, 0},
    std::pair{1, 1},
    std::pair{-1, 1},
    std::pair{1, -1},
    std::pair{-1, -1}
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

std::vector<std::pair<std::int64_t, std::int64_t>> get_adjacent_octopuses(
    const std::vector<std::vector<std::int64_t>>& input,
    const std::size_t column,
    const std::size_t row)
{
    auto adjacent_octopuses = std::vector<std::pair<std::int64_t, std::int64_t>>{};
    for (const auto& step: steps)
    {
        const auto y = column + step.first;
        const auto x = row + step.second;
        if ((x >= 0 && x < input[column].size()) && (y >= 0 && y < input.size()))
        {
            adjacent_octopuses.emplace_back(y, x);
        }
    }

    return adjacent_octopuses;
}

void flash(
    const std::pair<std::int64_t, std::int64_t>& octopus,
    std::vector<std::vector<std::int64_t>>& input,
    std::vector<std::vector<bool>>& have_flashed)
{
    if (have_flashed[octopus.first][octopus.second])
    {
        return;
    }

    auto to_flash = std::queue<std::pair<std::int64_t, std::int64_t>>{};
    to_flash.push(octopus);
    have_flashed[octopus.first][octopus.second] = true;

    while (!to_flash.empty())
    {
        const auto flash = to_flash.front();
        to_flash.pop();

        const auto adjacent_octopuses = get_adjacent_octopuses(input, flash.first, flash.second);
        for (const auto& adjacent_octopus : adjacent_octopuses)
        {
            ++input[adjacent_octopus.first][adjacent_octopus.second];
            if (input[adjacent_octopus.first][adjacent_octopus.second] > 9 &&
                !have_flashed[adjacent_octopus.first][adjacent_octopus.second])
            {
                have_flashed[adjacent_octopus.first][adjacent_octopus.second] = true;
                to_flash.push(adjacent_octopus);
            }
        }
    }
}

bool step(std::vector<std::vector<std::int64_t>>& input)
{
    auto to_flash = std::vector<std::pair<std::int64_t, std::int64_t>>{};
    auto have_flashed = std::vector<std::vector<bool>>(input.size(), std::vector<bool>(input.front().size(), false));

    for (auto column = 0U; column < input.size(); ++column)
    {
        for (auto row = 0U; row < input[column].size(); ++row)
        {
            ++input[column][row];
            if (input[column][row] > 9)
            {
                to_flash.emplace_back(column, row);
            }
        }
    }

    for (const auto& octopus : to_flash)
    {
        flash(octopus, input, have_flashed);
    }

    for (auto column = 0U; column < input.size(); ++column)
    {
        for (auto row = 0U; row < input[column].size(); ++row)
        {
            if (have_flashed[column][row])
            {
                input[column][row] = 0;
            }
        }
    }

    const auto have_all_flashed =
        std::all_of(have_flashed.begin(),
                    have_flashed.end(),
                    [](const auto& row) {
                        return std::all_of(row.begin(), row.end(), [](const auto flashed) { return flashed; });
                    });

    return have_all_flashed;
}

std::int64_t simulate(std::vector<std::vector<std::int64_t>> input)
{
    auto step_number = std::int64_t{1};
    while (!step(input))
    {
        ++step_number;
    }

    return step_number;
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        throw std::runtime_error("Provide filepath");
    }

    auto input = load_file(argv[1]);
    const auto result = simulate(std::move(input));
    std::cout << result << std::endl;
}
