open System
open System.IO

let args = fsi.CommandLineArgs |> Array.tail
let path = args[0]

let targets =
    File.ReadAllText(path).Split([|"x="; "y="; ".."; ", "|], StringSplitOptions.None)
    |> Array.filter (fun elem -> Int32.TryParse elem |> fst )
    |> Array.map int

let minY = targets[2]
let maxVy = abs minY - 1
let maxY = (maxVy * (maxVy + 1)) / 2

printfn "%A" (maxY)
