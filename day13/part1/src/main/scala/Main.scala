import scala.io.Source

type Coords = (Int, Int)
type Instruction = (Boolean, Int)

def makeCoord(line: String): Coords =
  val coords = line.split(",").map(_.toInt).toArray

  (coords(0), coords(1))

def makeInstruction(line: String): Instruction =
  val instructions = line.split("=").toArray
  val is_horizontal = if instructions(0).last == 'y' then { true } else { false }
  val value = instructions(1).toInt

  (is_horizontal, value)

def loadFile(path: String): (List[Coords], List[Instruction]) =
  val file = Source.fromFile(path)
  val output = file.mkString.split("\n\n").toArray;

  file.close

  val coordinates = output(0).split("\n").map(makeCoord).toList
  val instructions = output(1).split("\n").map(makeInstruction).toList

  (coordinates, instructions)

def fold(paper: Set[Coords], instruction: Instruction): Set[Coords] =
  val (horizontal, line) = instruction

  paper.map((x, y) => if horizontal then {
    if y < line then{ (x, y) } else { ((x, (line * 2) - y)) }
  } else {
    if x < line then { (x, y) } else { ((line * 2) - x, y) }
  })

def makeOrigami(input: (List[Coords], List[Instruction])): Int =
  val (paper, instructions) = input

  val folded = fold(paper.toSet, instructions.head)

  folded.size

@main def main(path: String): Unit = 
  val input = loadFile(path)
  val result = makeOrigami(input)
  println(result)
