defmodule Part1 do
  defp to_integer_list(binary) do
    binary |> String.graphemes() |> Enum.map(&String.to_integer/1)
  end

  defp load(filepath) do
    {:ok, contents} = File.read(filepath)
    contents |> String.split("\n", trim: true) |> Enum.map(&to_integer_list/1)
  end

  defp most_common(binary, output) do
    binary |> Enum.zip(output) |> Enum.map(fn {i, j} -> i + j end)
  end

  defp iterate([head | tail], output) do
    iterate(tail, most_common(head, output))
  end

  defp iterate([], output) do
    output
  end

  defp iterate(input) do
    width = length(hd(input))
    acc = List.duplicate(0, width)
    iterate(input, acc)
  end

  defp gamma_rate(input, length) do
    half = length / 2

    input
    |> Enum.map(fn x ->
      if x > half do
        1
      else
        0
      end
    end)
  end

  defp epsilon_rate(input, length) do
    half = length / 2

    input
    |> Enum.map(fn x ->
      if x < half do
        1
      else
        0
      end
    end)
  end

  defp solve(input) do
    nr_of_items = length(input)
    width = length(hd(input))

    output = iterate(input)
    gamma = Integer.undigits(gamma_rate(output, nr_of_items), 2)
    epsilon = Integer.undigits(epsilon_rate(output, nr_of_items), 2)

    gamma * epsilon
  end

  def main(args) do
    input = load(args)
    result = solve(input)
    IO.puts(result)
  end
end
