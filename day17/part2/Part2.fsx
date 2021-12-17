open System
open System.IO

let args = fsi.CommandLineArgs |> Array.tail
let path = args[0]

let targets =
    File.ReadAllText(path).Split([|"x="; "y="; ".."; ", "|], StringSplitOptions.None)
    |> Array.filter (fun elem -> Int32.TryParse elem |> fst )
    |> Array.map int

let (minX, maxX) = targets[0], targets[1]
let (minY, maxY) = targets[2], targets[3]

let shoot xv yv =
    let rec shoot (x: int, y: int, xv: int, yv: int) : bool =
        if (x <= maxX) && (y >= minY) then
            let nextX = x + xv
            let nextY = y + yv
            
            if (x >= minX) && (x <= maxX) && (y <= maxY) && (y >= minY) then true
            elif xv > 0 then shoot(nextX, nextY, xv - 1, yv - 1)
            elif xv < 0 then shoot(nextX, nextY, xv + 1, yv - 1)
            else shoot(nextX, nextY, xv, yv - 1)

        else false
    
    shoot(0, 0, xv, yv)

let iterate xv = [for yv in -200 .. 200 -> shoot xv yv] |> Seq.filter id |> Seq.length

let trajectories = [for xv in 0 .. maxX -> iterate xv] |> List.sum

printfn "%A" (trajectories)
