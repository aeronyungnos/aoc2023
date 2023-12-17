use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

const INF: i32 = std::i32::MAX;

fn revised_dijkstra(grid: Vec<&str>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut dist_dir: HashMap<(i32, i32, i32), Vec<Vec<i32>>> = HashMap::new();
    for i in 0..11 {
        dist_dir.insert((0, 1, i), vec![vec![INF; cols]; rows]);
        dist_dir.insert((0, -1, i), vec![vec![INF; cols]; rows]);
        dist_dir.insert((1, 0, i), vec![vec![INF; cols]; rows]);
        dist_dir.insert((-1, 0, i), vec![vec![INF; cols]; rows]);
    }

    let start = (0, 0, (0, 0), 0, Vec::new());
    let mut heap: BinaryHeap<Reverse<(i32, (usize, usize, (i32, i32), i32, Vec<i32>))>> =
        BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((cost, (row, col, dir, conseq, track)))) = heap.pop() {
        if row == rows - 1 && col == cols - 1 && conseq > 2 {
            return cost;
        }

        for &(dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;

            let direc = (dr, dc);

            let mut new_conseq = conseq;

            if direc == dir {
                new_conseq += 1;
                if new_conseq == 10 {
                    continue;
                }
            } else if direc == (-dir.0, -dir.1) {
                continue;
            } else if direc != dir {
                if dir == (0, 0) || conseq > 2 {
                    new_conseq = 0;
                } else {
                    continue;
                }
            }

            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let new_cost = cost + grid[nr as usize].as_bytes()[nc as usize] as i32 - '0' as i32;

                if new_cost < dist_dir[&(direc.0, direc.1, new_conseq)][nr as usize][nc as usize] {
                    dist_dir.get_mut(&(direc.0, direc.1, new_conseq)).unwrap()[nr as usize]
                        [nc as usize] = new_cost;

                    heap.push(Reverse((
                        new_cost,
                        (
                            nr as usize,
                            nc as usize,
                            direc,
                            new_conseq,
                            track.iter().cloned().collect(),
                        ),
                    )));
                }
            }
        }
    }

    -1
}

fn main() {
    let input: &str = include_str!("../input/17.txt");
    let grid: Vec<&str> = input.lines().collect();

    let lowest_heath_path: i32 = revised_dijkstra(grid);

    println!("{}", lowest_heath_path);
}
