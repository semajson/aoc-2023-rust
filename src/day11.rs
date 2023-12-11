use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day11;

impl Solution for Day11 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input: &str) -> Self::ParsedInput {
        let input_lines = input
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        input_lines
            .iter()
            .map(|x| x.chars().collect())
            .collect::<Vec<Vec<char>>>()
    }

    fn part_one(raw_image: &mut Self::ParsedInput) -> String {
        // TODO: implement part one
        let expanded_image = add_expansion(raw_image);

        let mut galaxies = vec![];

        for (y, row) in expanded_image.iter().enumerate() {
            for (x, point) in row.iter().enumerate() {
                if *point == '#' {
                    galaxies.push(Galaxy {
                        x: x.clone() as isize,
                        y: y.clone() as isize,
                    });
                }
            }
        }

        let mut sum = 0;
        for galaxy_a in galaxies.iter() {
            for galaxy_b in galaxies.iter() {
                sum += (galaxy_a.x - galaxy_b.x).abs() + (galaxy_a.y - galaxy_b.y).abs();
            }
        }
        println!("test");
        sum = sum / 2;

        sum.to_string()
    }

    fn part_two(raw_image: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

pub struct Galaxy {
    x: isize,
    y: isize,
}

fn add_expansion(raw_image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    fn add_row_expansion(raw_image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut new_image = vec![];
        for row in raw_image.iter() {
            if !row.contains(&'#') {
                new_image.push(row.clone());
                new_image.push(row.clone());
            } else {
                new_image.push(row.clone());
            }
        }
        new_image
    }

    // Do rows first
    let mut new_image = add_row_expansion(&raw_image);

    // Do columns
    new_image = transpose(new_image);
    new_image = add_row_expansion(&new_image);
    new_image = transpose(new_image);

    new_image
}

fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let mut new_matrix = vec![];
    for x in 0..matrix[0].len() {
        let mut new_row = vec![];
        for y in 0..matrix.len() {
            new_row.push(matrix[y][x].clone())
        }
        new_matrix.push(new_row)
    }
    new_matrix
}

// fn transpose_internet<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
// where
//     T: Clone,
// {
//     assert!(!v.is_empty());
//     (0..v[0].len())
//         .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
//         .collect()
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn get_transpose() {
    //     let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
    //     assert!(transpose(matrix.clone()) == transpose_internet(matrix))
    // }

    #[test]
    fn check_day11_part1_case1() {
        assert_eq!(
            Day11::solve_part_one(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            ),
            "374".to_string()
        )
    }

    #[test]
    fn check_day11_part2_case1() {
        assert_eq!(
            Day11::solve_part_two(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            ),
            "0".to_string()
        )
    }

    #[test]
    fn check_day11_both_case1() {
        assert_eq!(Day11::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
