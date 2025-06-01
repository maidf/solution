use std::collections::{HashSet, VecDeque};

pub fn shortest_path_binary_matori(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
        return -1;
    }

    let d = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    let mut queue = VecDeque::<(i32, i32, i32)>::new();
    queue.push_back((0, 0, 1));
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    while !queue.is_empty() {
        let (x, y, dist) = queue.pop_front().unwrap();

        if x as usize == n - 1 && y as usize == n - 1 {
            return dist;
        }

        for (i, j) in d {
            let (dx, dy) = (i+x, j+y);
            if dx < 0 || n as i32 <= dx || dy < 0 || n as i32 <= dy {
                continue;
            }
            
            if grid[dx as usize][dy as usize] == 0 && !visited.contains(&(dx, dy)) {
                queue.push_back((dx, dy, dist + 1));
                visited.insert((dx, dy));
            }
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::shortest_path_binary_matori;

    #[test]
    fn test() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        let res = shortest_path_binary_matori(grid);
        assert_eq!(2, res);
    }

    #[test]
    fn test2() {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let res = shortest_path_binary_matori(grid);
        assert_eq!(4, res);
    }
}
