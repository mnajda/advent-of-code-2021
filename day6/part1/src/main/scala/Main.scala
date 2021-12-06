import scala.io.Source

def parse(line: String): List[Int] =
  line.split(",").map(_.toInt).toList

def loadFile(path: String): List[Int] =
  val file = Source.fromFile(path)
  val output = file.getLines().flatMap(parse).toList

  file.close

  output

def iterate(fish: List[Int]): Int =
  def iterate(fish: List[Int], step: Int): Int = {
    if (step == 80) {
      return fish.size
    }

    val new_fish = fish.map(fish => if (fish == 0) { 6 } else { fish - 1 }).toList;
    val how_many_born = fish.count(_ == 0)
    iterate(new_fish ++ List.fill(how_many_born)(8), step + 1)
  }
  
  iterate(fish, 0)

@main def main(path: String, others: String*): Unit = 
  val input = loadFile(path)
  val result = iterate(input)

  println(result)
