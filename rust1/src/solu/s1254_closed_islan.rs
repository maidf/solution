pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut grid = grid;

    fn dfs(r: usize, c: usize, grid: &mut Vec<Vec<i32>>) -> bool {
        if r >= grid.len() || c >= grid[0].len() {
            return false;
        }
        if grid[r][c] == 1 {
            return true;
        }

        grid[r][c] = 1;

        let up = dfs(r - 1, c, grid);
        let down = dfs(r + 1, c, grid);
        let left = dfs(r, c - 1, grid);
        let right = dfs(r, c + 1, grid);

        up && down && left && right
    }

    let mut res = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 0 {
                if dfs(i, j, &mut grid) {
                    res += 1;
                }
            }
        }
    }
    res
}
