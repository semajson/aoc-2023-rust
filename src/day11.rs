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
        let expanded_image = add_expansion(raw_image);

        let galaxies = get_galaxies(&expanded_image);

        let difference_sum = galaxies_difference_sum(&galaxies);

        difference_sum.to_string()
    }

    fn part_two(raw_image: &mut Self::ParsedInput) -> String {
        let image = raw_image.clone();
        let galaxies = get_galaxies(&image);

        // Find the indexes of the blank rows/columns
        let empty_row_indexes = get_empty_row_indexes(&image);
        let transposed_image = transpose(&image);
        let empty_column_indexes = get_empty_row_indexes(&transposed_image);

        let galaxies =
            get_galaxies_when_expanded_quick(&galaxies, &empty_row_indexes, &empty_column_indexes);

        let difference_sum = galaxies_difference_sum(&galaxies);

        difference_sum.to_string()
    }
}

fn get_galaxies_when_expanded_quick(
    galaxies: &[Galaxy],
    empty_row_indexes: &[isize],
    empty_column_indexes: &[isize],
) -> Vec<Galaxy> {
    // NGL, this is pretty horrible :(
    // Lots of errors hit during writing this...
    let mut new_galaxies = vec![];
    for galaxy in galaxies {
        let num_x_expansions = empty_column_indexes
            .iter()
            .filter(|x| **x < galaxy.x)
            .collect::<Vec<&isize>>()
            .len() as isize;
        let num_y_expansions = empty_row_indexes
            .iter()
            .filter(|y| **y < galaxy.y)
            .collect::<Vec<&isize>>()
            .len() as isize;
        let multiplier: isize = 1000000;

        let new_x = galaxy.x + num_x_expansions * (multiplier - 1);
        let new_y = galaxy.y + num_y_expansions * (multiplier - 1);

        let new_galaxy = Galaxy { x: new_x, y: new_y };
        new_galaxies.push(new_galaxy);
    }

    new_galaxies
}

fn get_empty_row_indexes(image: &[Vec<char>]) -> Vec<isize> {
    let mut indexes = vec![];

    for (index, row) in image.iter().enumerate() {
        if !row.contains(&'#') {
            indexes.push(index as isize);
        }
    }
    indexes
}

fn get_galaxies(image: &[Vec<char>]) -> Vec<Galaxy> {
    let mut galaxies = vec![];

    for (y, row) in image.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if *point == '#' {
                galaxies.push(Galaxy {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    galaxies
}

fn galaxies_difference_sum(galaxies: &[Galaxy]) -> isize {
    let mut sum = 0;
    for galaxy_a in galaxies.iter() {
        for galaxy_b in galaxies.iter() {
            sum += (galaxy_a.x - galaxy_b.x).abs() + (galaxy_a.y - galaxy_b.y).abs();
        }
    }
    sum /= 2;
    sum
}

pub struct Galaxy {
    x: isize,
    y: isize,
}

fn add_expansion(raw_image: &[Vec<char>]) -> Vec<Vec<char>> {
    fn add_row_expansion(raw_image: &[Vec<char>]) -> Vec<Vec<char>> {
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
    let mut new_image = add_row_expansion(raw_image);

    // Do columns
    new_image = transpose(&new_image);
    new_image = add_row_expansion(&new_image);
    new_image = transpose(&new_image);

    new_image
}

#[allow(clippy::needless_range_loop)]
fn transpose<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
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

#[cfg(test)]
mod tests {
    use super::*;

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
            "1030".to_string()
        )
    }

    #[test]
    fn check_day11_both_case1() {
        assert_eq!(Day11::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
