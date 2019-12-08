use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut data = load_data(&input);
    run_program(&mut data);
}

/// Read an input string and return a vector of integers.
fn load_data(input_lines: &str) -> Vec<i64> {
    let mut data = Vec::new();
    for line in input_lines.split(",") {
        let parsed_line = line.trim().parse::<i64>().unwrap();
        data.push(parsed_line);
    }
    data
}

fn run_program(program: &mut Vec<i64>) {
    for (index, code) in program.iter_mut().enumerate() {
        match *code {
            1 => add(
                program[index + 1],
                program[index + 2],
                program.get_mut(index + 3).unwrap(),
            ),
            2 => multiply(
                program[index + 1],
                program[index + 2],
                program.get_mut(index + 3).unwrap(),
            ),
            99 => return,
            _ => panic!("Unexpected opcode"),
        }
    }
}

fn add(a: i64, b: i64, res: &mut i64) {
    *res += a + b;
}

fn multiply(a: i64, b: i64, res: &mut i64) {
    *res += a + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut data = vec![1, 0, 0, 0, 99];
        let expected = vec![2, 0, 0, 0, 99];

        run_program(&mut data);

        assert_eq!(data, expected);
    }

    #[test]
    fn test_example_2() {
        let mut data = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];

        run_program(&mut data);

        assert_eq!(data, expected);
    }

    #[test]
    fn test_example_3() {
        let mut data = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];

        run_program(&mut data);

        assert_eq!(data, expected);
    }

    #[test]
    fn test_example_4() {
        let mut data = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        run_program(&mut data);

        assert_eq!(data, expected);
    }
}
