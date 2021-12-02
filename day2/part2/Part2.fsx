open System
open System.IO

let args : string array = fsi.CommandLineArgs |> Array.tail
let path = args[0]

let makeCommand (cmd : string) =
   let command = cmd.Split(" ")
   (command[0], command[1] |> int)

let commands =
    File.ReadAllText(path).Split("\n")
    |> Array.map(fun cmd -> makeCommand(cmd))
    |> Seq.toList

let executeCommand (command : string * int, position : int * int * int) : int * int * int =
    let (cmd, value) = command
    let (horizontal, depth, aim) = position

    match cmd with
    | "forward" -> (horizontal + value, depth + (aim * value), aim)
    | "down" -> (horizontal, depth, aim + value)
    | "up" -> (horizontal, depth, aim - value)
    | _ -> (horizontal, depth, aim)

let executeCommands commands =
    let rec execute (commands : _ list, position : int * int * int) : int * int * int =
        match commands with
        | []              -> position
        | command :: []   -> executeCommand(command, position)
        | command :: tail -> execute(tail, executeCommand(command, position))
    execute(commands, (0, 0, 0))

let (horizontal, depth, _) = executeCommands commands
printfn "%A" (horizontal * depth)
