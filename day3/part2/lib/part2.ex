defmodule Part2 do
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

  defp rate([result], _, _, _) do
    result
  end

  defp rate(input, commons, transformation, step) do
    to_keep = transformation.(hd(commons), length(input) / 2)
    filtered = input |> Enum.filter(fn number -> number |> Enum.at(step) == to_keep end)
    new_commons = iterate(filtered)
    rate(filtered, Enum.drop(new_commons, step + 1), transformation, step + 1)
  end

  defp solve(input) do
    most_common = iterate(input)

    oxygen_rating_filter = fn x, half ->
      if(x >= half) do
        1
      else
        0
      end
    end

    co2_rating_filter = fn x, half ->
      if(x < half) do
        1
      else
        0
      end
    end

    oxygen_rating = rate(input, most_common, oxygen_rating_filter, 0)
    co2_rating = rate(input, most_common, co2_rating_filter, 0)

    Integer.undigits(oxygen_rating, 2) * Integer.undigits(co2_rating, 2)
  end

  def main(args) do
    input = load(args)
    result = solve(input)
    IO.puts(result)
  end
end
