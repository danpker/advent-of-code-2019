#[aoc_generator(day1)]
fn parse_input_day1(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part_one(input: &[i32]) -> i32 {
    input.iter().map(|x| fuel_requirement(*x)).sum()
}

#[aoc(day1, part2)]
fn part_two(input: &[i32]) -> i32 {
    input
        .iter()
        .map(|x| fuel_requirement_including_fuel(*x))
        .sum()
}

/// Fuel required to launch a given module is based on its mass. Specifically, to find the fuel
/// required for a module, take its mass, divide by three, round down, and subtract 2.
fn fuel_requirement(mass: i32) -> i32 {
    (mass / 3) - 2
}

/// Calculate the fuel requirement including additional fuel to carry the initial
/// fuel load.
/// This is calcualted by finding out the fuel amount for the starting mass and then
/// finding out the fuel required for that amount using the same calculation.
/// This happens recursively until zero or negative fuel is required for the previous
/// fuel amount.
fn fuel_requirement_including_fuel(mass: i32) -> i32 {
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
