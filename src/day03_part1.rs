use std::fs;

//const INPUT_PATH: &str = "input/03/example.txt";
const INPUT_PATH: &str = "input/03/input.txt";

pub fn main() {
    let content = fs::read_to_string(INPUT_PATH).unwrap();

    let mut total = 0;
    for line in content.lines() {
        let max_joltage = max_joltage(line);
        total += max_joltage;
    }

    println!();
    println!("Total: {}", total);
    println!("Lines: {}", content.lines().collect::<Vec<&str>>().len());
}

fn max_joltage(line: &str) -> u32 {
    println!("--- Line: {}", line);

    let mut largest: Vec<u8> = vec![];
    let batteries: Vec<u8> = line.chars().map(|c| -> u8 {
        c.to_string().parse().unwrap()
    }).collect();
    //println!("{:?}", batteries);

    for index in 0..batteries.len() {
        let joltage = batteries[index];
        if largest.len() == 0 {
            largest.push(joltage);
        } else if largest.len() == 1 {
            if largest[0] < joltage {
                largest[0] = joltage;
                largest.push(0);
            } else {
                largest.push(joltage);
            }
            largest.push(joltage);
        } else {
            if largest[0] < joltage && index != batteries.len() - 1 {
                println!("    DEBUG: {} < {}", largest[0], joltage);
                largest[0] = joltage;
                largest[1] = 0;
            } else if largest[1] < joltage {
                println!("    DEBUG2: {} < {}", largest[1], joltage);
                if largest[0] < largest[1] {
                    println!("    DEBUG3: {} < {}", largest[0], largest[1]);
                    largest[0] = largest[1];
                }
                largest[1] = joltage;
            } 
        }
    }

    let total_joltage = largest[0] * 10 + largest[1];
    println!("    ==> {}", total_joltage);

    return total_joltage as u32;
}

#[test]
fn test_examples() {
    assert_eq!(98, max_joltage("987654321111111"));
    assert_eq!(89, max_joltage("811111111111119"));
    assert_eq!(78, max_joltage("234234234234278"));
    assert_eq!(92, max_joltage("818181911112111"));
}

#[test]
fn test_own() {
    assert_eq!(99, max_joltage("887654321111199"));
    assert_eq!(87, max_joltage("4732321333332463233337712234322122322247222252423773321362313613333336333732233372323328332333322777"));
    assert_eq!(96, max_joltage("1222244327222414232233422425356322526513215232625252222344534141424312492321632634253122343124442433"));
    assert_eq!(75, max_joltage("5112454424222122431441522442342425334525244254332453524214472412223221252222352512235352425214223422"));
    assert_eq!(95, max_joltage("2222227321754726673633524322456382771445438343537513235332426476332523246251452543731252361447292252"));
    assert_eq!(99, max_joltage("5754818635363378216255676242211353265462243358231497363334444343516236326731322232642636742327379332"));
    assert_eq!(97, max_joltage("3122221222112312312232222262521312223131232212392237234221112213215321242221222223122722223242231132"));
    assert_eq!(86, max_joltage("3332221732222313632231112123312334132423312333223376521312228322222132232231322233613312223323133232"));
    assert_eq!(66, max_joltage("1562433324432523322122322243553245224632262522233262336333252143334323134333211324443413323134412552"));
    assert_eq!(99, max_joltage("8678575554446875757776855398756854855887628657665566385684847784545935875964615788576788674444588668"));
    assert_eq!(86, max_joltage("2224132122282231232225222122213512115222212242223112232222423212262221252215213222224122242222522212"));
    assert_eq!(77, max_joltage("6526643435244533412553645532433133432515342223625213345252545334225321221733354423152716334415342425"));
    assert_eq!(94, max_joltage("2422232382217254223228342222122345525213271313333542273324232313233312223721222223231521342942232232"));
    assert_eq!(99, max_joltage("3353245432332434653526334135283437653612538722536244512718328332636432532363993842768352552662374632"));
    assert_eq!(77, max_joltage("4445544245563555645464242235261443573553343522346445663441531676424463555542134364566541335553566255"));
    assert_eq!(66, max_joltage("2121222322122122263322222225421233422333333332456223223233212322332322122141232523222223222321222232"));
    assert_eq!(99, max_joltage("2126212253329254635558385225472857739325164231817544393536578564312324625232214378474322225725423441"));

    assert_eq!(91, max_joltage("891"));
}