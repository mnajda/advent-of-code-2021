open System
open System.IO

let args : string array = fsi.CommandLineArgs |> Array.tail
let path = args[0]

let positions =
    File.ReadAllText(path).Split(",")
    |> Array.map(int)
    |> Seq.toList

let calculateCost positions dest =
    List.fold (fun acc pos -> acc + abs (pos - dest)) 0 positions

let calculateCosts positions =
    let minimum = List.reduce min positions
    let maximum = List.reduce max positions

    List.reduce min [for dest in minimum .. maximum -> calculateCost positions dest]

let result = calculateCosts positions
printfn "%A" (result)
