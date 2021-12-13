using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

public class Part1
{
    static Dictionary<string, List<string>> ReadFile(string path)
    {
        using (var reader = new StreamReader(path))
        {
            var contents = reader.ReadToEnd().Split("\n");

            var nodes = new Dictionary<string, List<string>>();
            foreach (var line in contents)
            {
                var input = line.Split('-');

                if (!nodes.ContainsKey(input[0]))
                {
                    nodes[input[0]] = new List<string>();
                }
                if (!nodes.ContainsKey(input[1]))
                {
                    nodes[input[1]] = new List<string>();
                }

                if (input[1] != "start")
                {
                    nodes[input[0]].Add(input[1]);
                }
                if (input[0] != "start")
                {
                    nodes[input[1]].Add(input[0]);
                }
            }
            return nodes;
        }
    }

    static int Solve(Dictionary<string, List<string>> input)
    {
        Queue<Tuple<List<string>, bool>> to_visit = new Queue<Tuple<List<string>, bool>>();
        to_visit.Enqueue(new Tuple<List<string>, bool>(new List<string>() { "start" }, false));

        int paths = 0;
        while (to_visit.Count > 0)
        {
            var elem = to_visit.Dequeue();
            var path = elem.Item1;
            var was_small_cave_visited_twice = elem.Item2;
            var node = path.Last();

            foreach (var next_node in input[node])
            {
                if (next_node == "end")
                {
                    paths += 1;
                    continue;
                }

                var is_small_cave = Char.IsLower(next_node[0]);
                var is_already_visited = path.Contains(next_node);
                if (is_small_cave && is_already_visited && was_small_cave_visited_twice)
                {
                    continue;
                }

                var new_path = path.ToList();
                new_path.Add(next_node);

                var can_still_visit_small_cave_twice =
                    (is_small_cave && is_already_visited) || was_small_cave_visited_twice;

                to_visit.Enqueue(new Tuple<List<string>, bool>(new_path, can_still_visit_small_cave_twice));
            }
        }

        return paths;
    }

    public static void Main(string[] args)
    {
        var input = ReadFile(args[0]);
        var result = Solve(input);
        Console.WriteLine(result);
    }
}
