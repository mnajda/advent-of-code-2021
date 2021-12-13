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
        Queue<List<string>> to_visit = new Queue<List<string>>();
        to_visit.Enqueue(new List<string>() { "start" });

        int paths = 0;
        while (to_visit.Count > 0)
        {
            var path = to_visit.Dequeue();
            var node = path.Last();

            foreach (var next_node in input[node])
            {
                if (next_node == "end")
                {
                    paths += 1;
                    continue;
                }

                if (Char.IsLower(next_node[0]) && path.Contains(next_node))
                {
                    continue;
                }

                var new_path = path.ToList();
                new_path.Add(next_node);
                to_visit.Enqueue(new_path);
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
