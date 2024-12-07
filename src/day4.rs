use super::read_grid_bytes;

const fn check_m_and_s(a: u8, b: u8) -> bool { a == b'M' && b == b'S' || a == b'S' && b == b'M' }

pub fn day4_b() {
    let grid = read_grid_bytes(4);
    let mut result = 0;
    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            result += usize::from(
                grid[y][x] == b'A'
                    && check_m_and_s(grid[y - 1][x - 1], grid[y + 1][x + 1])
                    && check_m_and_s(grid[y - 1][x + 1], grid[y + 1][x - 1]),
            );
        }
    }
    println!("{result}");
}

pub fn day4_a() {
    let grid = read_grid_bytes(4);
    let mut result = 0;
    for (y, r) in grid.iter().enumerate() {
        for (x, &v) in r.iter().enumerate() {
            if v == b'X' {
                let left = x >= 3;
                let right = x < r.len() - 3;
                let up = y >= 3;
                let down = y < grid.len() - 3;
                result += usize::from(left && &r[x - 3..x] == b"SAM");
                result += usize::from(right && &r[x + 1..=x + 3] == b"MAS");
                result += usize::from(
                    up && grid[y - 1][x] == b'M'
                        && grid[y - 2][x] == b'A'
                        && grid[y - 3][x] == b'S',
                );
                result += usize::from(
                    down && grid[y + 1][x] == b'M'
                        && grid[y + 2][x] == b'A'
                        && grid[y + 3][x] == b'S',
                );
                result += usize::from(
                    left & up
                        && grid[y - 1][x - 1] == b'M'
                        && grid[y - 2][x - 2] == b'A'
                        && grid[y - 3][x - 3] == b'S',
                );
                result += usize::from(
                    left & down
                        && grid[y + 1][x - 1] == b'M'
                        && grid[y + 2][x - 2] == b'A'
                        && grid[y + 3][x - 3] == b'S',
                );
                result += usize::from(
                    right & up
                        && grid[y - 1][x + 1] == b'M'
                        && grid[y - 2][x + 2] == b'A'
                        && grid[y - 3][x + 3] == b'S',
                );
                result += usize::from(
                    right & down
                        && grid[y + 1][x + 1] == b'M'
                        && grid[y + 2][x + 2] == b'A'
                        && grid[y + 3][x + 3] == b'S',
                );
            }
        }
    }
    println!("{result}");
}
