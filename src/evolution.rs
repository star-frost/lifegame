use crate::GRID_SIZE;

pub fn next_generation_bounded(
    current: &[[bool; GRID_SIZE]; GRID_SIZE],
) -> [[bool; GRID_SIZE]; GRID_SIZE] {
    let mut next = [[false; GRID_SIZE]; GRID_SIZE];

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let mut neighbors = 0i32;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < GRID_SIZE as isize && ny >= 0 && ny < GRID_SIZE as isize {
                        if current[ny as usize][nx as usize] {
                            neighbors += 1;
                        }
                    }
                }
            }
            next[y][x] = if current[y][x] {
                neighbors == 2 || neighbors == 3
            } else {
                neighbors == 3
            };
        }
    }

    next
}

pub fn next_generation_torus(
    current: &[[bool; GRID_SIZE]; GRID_SIZE],
) -> [[bool; GRID_SIZE]; GRID_SIZE] {
    let mut next = [[false; GRID_SIZE]; GRID_SIZE];

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let mut neighbors = 0i32;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = ((x as isize + dx).rem_euclid(GRID_SIZE as isize)) as usize;
                    let ny = ((y as isize + dy).rem_euclid(GRID_SIZE as isize)) as usize;
                    if current[ny][nx] {
                        neighbors += 1;
                    }
                }
            }
            next[y][x] = if current[y][x] {
                neighbors == 2 || neighbors == 3
            } else {
                neighbors == 3
            };
        }
    }

    next
}

pub fn next_generation(current: &[[bool; GRID_SIZE]; GRID_SIZE]) -> [[bool; GRID_SIZE]; GRID_SIZE] {
    debug_assert_eq!(current.len(), GRID_SIZE);
    next_generation_bounded(current)
}
