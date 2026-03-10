use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn generate_grid() -> Vec<Vec<i32>> {
    let grid:Vec<Vec<i32>> = vec![
        vec![9,9,9,0,0],
        vec![9,9,0,0,9],
        vec![9,0,0,9,9],
        vec![0,0,9,9,9],
        vec![1,9,9,9,9],
    ];
    grid
}

fn dijkstra(grid: &Vec<Vec<i32>>) {
    let rows = grid.len();
    let columns = grid[0].len();

    let mut dist = vec![vec![i32::MAX; columns]; rows];

    dist[0][0] = grid[0][0];

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((grid[0][0], 0usize, 0usize)));
    
    let directions: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

    while let Some(Reverse((cost, r, c))) = heap.pop() {
        if r == rows-1 && c == columns-1 {
            println!("Menor custo total: {}", cost);
            return;
        }

        for (dr, dc) in directions {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            let inside_grid = nr >= 0
                && nc >= 0
                && nr < rows as i32
                && nc < columns as i32;

            if inside_grid {
                let nr = nr as usize;
                let nc = nc as usize;

                let new_cost = cost + grid[nr][nc];

                if new_cost < dist[nr][nc] {
                    dist[nr][nc] = new_cost;
                    heap.push(Reverse((new_cost, nr, nc)));
                }
            }   
        }
    }
}

fn main() {
    let grid:Vec<Vec<i32>> = generate_grid();
    
    dijkstra(&grid);
}
