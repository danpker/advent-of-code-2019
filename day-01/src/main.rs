use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    part_one(&input);
    part_two(&input);
}

fn part_one(input_lines: &str) {
    let mut total = 0;
    for line in input_lines.lines() {
        let parsed_line = line.parse::<i64>().unwrap();
        total += fuel_requirement(parsed_line);
    }

    println!("Part One: {}", total);
}

fn part_two(input_lines: &str) {
    let mut total = 0;
    for line in input_lines.lines() {
        let parsed_line = line.parse::<i64>().unwrap();
        total += fuel_requirement_including_fuel(parsed_line);
    }

    println!("Part Two: {}", total);
}

/// Fuel required to launch a given module is based on its mass. Specifically, to find the fuel
/// required for a module, take its mass, divide by three, round down, and subtract 2.
fn fuel_requirement(mass: i64) -> i64 {
    (mass / 3) - 2
}

/// Calculate the fuel requirement including additional fuel to carry the initial
/// fuel load.
/// This is calcualted by finding out the fuel amount for the starting mass and then
/// finding out the fuel required for that amount using the same calculation.
/// This happens recursively until zero or negative fuel is required for the previous
/// fuel amount.
fn fuel_requirement_including_fuel(mass: i64) -> i64 {
    let starting_fuel = fuel_requirement(mass);

    if starting_fuel <= 0 {
        return 0;
    } else {
        starting_fuel + fuel_requirement_including_fuel(starting_fuel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_of_12_will_be_2() {
        assert_eq!(fuel_requirement(12), 2);
    }

    #[test]
    fn test_mass_of_14_will_be_2() {
        assert_eq!(fuel_requirement(14), 2);
    }

    #[test]
    fn test_mass_of_1969_will_be_654() {
        assert_eq!(fuel_requirement(1969), 654);
    }

    #[test]
    fn test_mass_of_100756_will_be_33583() {
        assert_eq!(fuel_requirement(100756), 33583);
    }

    #[test]
    fn test_mass_of_12_will_be_2_including_fuel() {
        assert_eq!(fuel_requirement_including_fuel(12), 2);
    }

    #[test]
    fn test_mass_of_1969_will_be_966_including_fuel() {
        assert_eq!(fuel_requirement_including_fuel(1969), 966);
    }

    #[test]
    fn test_mass_of_100756_will_be_50346_including_fuel() {
        assert_eq!(fuel_requirement_including_fuel(100756), 50346);
    }
}
