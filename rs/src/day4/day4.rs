use std::collections::HashMap;

fn count_neighbors(row: usize, col: usize, matrix: &Vec<Vec<char>>) -> u32 {
    let neighbor_offsets: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    neighbor_offsets
        .iter()
        .map(|(r_off, c_off)| (row as i32 + r_off, col as i32 + c_off))
        .filter(|(r, c)| {
            if (*r >= 0)
                && ((*r as usize) < matrix.len())
                && (*c >= 0)
                && ((*c as usize) < matrix[*r as usize].len())
            {
                true
            } else {
                false
            }
        })
        .map(|(r, c)| (r as usize, c as usize))
        .fold(0, |a, e| if matrix[e.0][e.1] == '@' { a + 1 } else { a })
}

fn build_matrix() -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    let data = include_str!("input.txt");
    for line in data.trim().split("\n") {
        let mut curr = Vec::new();
        for c in line.chars() {
            curr.push(c);
        }
        matrix.push(curr);
    }
    matrix
}

struct Node {
    row: u32,
    col: u32,
}

impl Node {
    fn get_key(&self) -> (u32, u32) {
        (self.row, self.col)
    }

    fn get_neighbors(&self, maxrow: u32, maxcol: u32) -> Vec<(u32, u32)> {
        let (row, col) = (self.row, self.col);
        [
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ]
        .into_iter()
        .filter(|&(r, c)| r >= 0 && r < maxrow && c >= 0 && c < maxcol)
        .collect()
    }
}

fn build_graph() -> HashMap<(u32, u32), Vec<(u32, u32)>> {
    let mut mygraph = HashMap::new();
    let data = include_str!("sample.txt");
    for (row, line) in data.trim().split("\n").enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '@' {
                let node = Node {
                    row: row as u32,
                    col: col as u32,
                };
                mygraph.insert(node.get_key(), Vec::new());
                for neigh in node.get_neighbors(data.len() as u32, line.len() as u32) {
                    if mygraph.contains_key(&neigh) {
                        mygraph[&neigh].push(node.get_key());
                        mygraph[&node.get_key()].push(neigh);
                    }
                }
            }
        }
    }
    mygraph
}

pub fn run() {
    let graph = build_graph();
    println!("{:?}", output);
    let mut matrix = build_matrix();
    let mut output = 0;
    let mut removed;
    loop {
        removed = false;
        for row in 0..matrix.len() {
            for col in 0..matrix[row].len() {
                if matrix[row][col] == '@' {
                    let ct = count_neighbors(row, col, &matrix);
                    if ct < 4 {
                        output += 1;
                        matrix[row][col] = '.';
                        removed = true;
                    }
                }
            }
        }
        if !removed {
            break;
        }
    }
    println!("{:?}", output);
}
