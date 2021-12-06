import scala.io.Source

def parse(line: String): List[Int] =
  line.split(",").map(_.toInt).toList

def loadFile(path: String): List[Int] =
  val file = Source.fromFile(path)
  val output = file.getLines().flatMap(parse).toList

  file.close

  output

def iterate(input: List[Int]): Long =
  def iterate(fish: Array[Long], step: Int): Long = {
    if (step == 256) {
      return fish.map(_.toLong).sum
    }

    val how_many_born = fish(0)
    for i <- 0 to 7 do fish(i) = fish(i + 1)
    fish(6) += how_many_born
    fish(8) = how_many_born

    iterate(fish, step + 1)
  }
  
  var fish = Array.fill(9)(0L)
  for times <- input do fish(times) += 1
  
  iterate(fish, 0)

@main def main(path: String, others: String*): Unit = 
  val input = loadFile(path)
  val result = iterate(input)

  println(result)
