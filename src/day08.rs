use std::collections::HashMap;

const INPUT: &str = include_str!("../data/Day08.txt");

/* Danger will robinson */
fn digit(c: char) -> i8 {
    (c as i8) - ('0' as i8)
}

fn check_visible_row(row: &Vec<i8>, width: usize, result: &mut Vec<bool>) {
    let mut vis_height_l: i8 = -1;
    let mut vis_height_r: i8 = -1;
    for x in 0..width {
        // left to right
        if row[x] > vis_height_l {
            vis_height_l = row[x];
            result[x] = true;
        };
        let r = width - x - 1;
        // right to left
        if row[r] > vis_height_r {
            vis_height_r = row[r];
            result[r] = true;
        };
    }
}

fn check_visible_col(grid: &Vec<Vec<i8>>, x: usize, height: usize, visible: &mut Vec<Vec<bool>>) {
    let mut vis_height_t: i8 = -1;
    let mut vis_height_b: i8 = -1;
    for y in 0..height {
        // down
        if grid[y][x] > vis_height_t {
            vis_height_t = grid[y][x];
            visible[y][x] = true;
        };
        let r = height - y - 1;
        // up
        if grid[r][x] > vis_height_b {
            vis_height_b = grid[r][x];
            visible[r][x] = true;
        };
    }
}

fn part1(input: &str) -> usize {
    let grid = input
        .split("\r\n")
        .into_iter()
        .map(|s| s.chars().map(digit).collect::<Vec<_>>())
        .filter(|s| s.len() > 0)
        .collect::<Vec<_>>();
    let height = grid.len();
    let width = grid[0].len();
    let mut visible: Vec<Vec<bool>> = Vec::with_capacity(height);
    for _ in 0..height {
        let mut row = Vec::with_capacity(width);
        for _ in 0..width {
            row.push(false);
        }
        visible.push(row);
    }
    for y in 0..height {
        check_visible_row(&grid[y], width, &mut visible[y])
    }
    for x in 0..width {
        check_visible_col(&grid, x, height, &mut visible)
    }
    visible
        .into_iter()
        .map(|r| r.into_iter().filter(|b| *b).count())
        .sum()
}

fn count_setup() -> HashMap<i8, usize> {
    let mut r: HashMap<i8, usize> = HashMap::new();
    for i in 0..(10 as i8) {
        r.insert(i,0);
    }
    r
}

fn update_counts(c: &mut HashMap<i8, usize>, h: i8) {
    // anything smaller than h can only see 1
    for i in 0..10 {
        let current = c.get(&i).unwrap();
        if i <= h {
            c.insert(i, 1);
        } else {
            c.insert(i, *current+1);
        }
    }
}

fn calc_scenic_col(grid: &Vec<Vec<i8>>, y: usize, width: usize, scenic: &mut Vec<Vec<i32>>) {
    let mut lcounts = count_setup();
    let mut rcounts = count_setup();

    for x in 0..width {
        // left to right
        scenic[y][x] *= *(lcounts.get(&(*grid)[y][x]).unwrap()) as i32;
        update_counts(&mut lcounts, (*grid)[y][x]);

        // right to left
        let r = width - x - 1;
        scenic[y][r] *= *(rcounts.get(&(*grid)[y][r]).unwrap()) as i32;
        update_counts(&mut rcounts, (*grid)[y][r]);
    }
}

fn calc_scenic_row(grid: &Vec<Vec<i8>>, x: usize, height: usize, scenic: &mut Vec<Vec<i32>>) {
    let mut tcounts = count_setup();
    let mut bcounts = count_setup();

    for y in 0..height {
        // down
        scenic[y][x] *= *(tcounts.get(&(*grid)[y][x]).unwrap()) as i32;
        update_counts(&mut tcounts, (*grid)[y][x]);

        // up
        let r = height - y - 1;
        scenic[r][x] *= *(bcounts.get(&(*grid)[r][x]).unwrap()) as i32;
        update_counts(&mut bcounts, (*grid)[r][x]);
    }
}

fn part2(input: &str) -> i32 {
    let grid = input
        .split("\r\n")
        .into_iter()
        .map(|s| s.chars().map(digit).collect::<Vec<_>>())
        .filter(|s| s.len() > 0)
        .collect::<Vec<_>>();
    let height = grid.len();
    let width = grid[0].len();
    let mut scenic: Vec<Vec<i32>> = Vec::with_capacity(height);
    for _ in 0..height {
        let mut row = Vec::with_capacity(width);
        for _ in 0..width {
            row.push(1);
        }
        scenic.push(row);
    }
    for y in 0..height {
        calc_scenic_row(&grid, y, width, &mut scenic)
    }
    for x in 0..width {
        calc_scenic_col(&grid, x, height, &mut scenic)
    }
    scenic
        .into_iter()
        .map(|r| r.into_iter().max().unwrap())
        .max()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../data/Day08_test.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 8);
    }
}
