// This one is ugly, I made a brut force solution. 

#[derive(PartialEq, Clone, Copy)]
pub enum Visibility {
    Visible,
    Hidden,
    Unset,
}

pub struct VisibilityTable {
    left: Visibility,
    right: Visibility,
    top: Visibility,
    down: Visibility,
}

impl VisibilityTable {
    pub fn as_vec(&self) -> Vec<Visibility> {
        return vec![self.left, self.right, self.top, self.down];
    }
}

impl Default for VisibilityTable {
    fn default() -> Self {
        Self {
            left: Visibility::Unset,
            right: Visibility::Unset,
            top: Visibility::Unset,
            down: Visibility::Unset,
        }
    }
}

pub struct Map {
    trees: Vec<Vec<(u32, VisibilityTable)>>,
    height: usize,
    width: usize,
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for line in &self.trees {
            for (c, t) in line {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn parse(input: &str) -> Self {
        return Self {
            trees: input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| (c.to_digit(10).unwrap(), VisibilityTable::default()))
                        .collect::<Vec<(u32, VisibilityTable)>>()
                })
                .collect::<Vec<Vec<(u32, VisibilityTable)>>>(),
            height: input.lines().count(),
            width: input
                .lines()
                .map(|line| line.chars().count())
                .next()
                .unwrap(),
        };
    }
}

pub fn set_visibility(mut map: Map) -> Map {
    // left
    for i in 0..map.height {
        let mut max_left = 0;
        for j in 0..map.width {
            let (val, v_table) = &mut map.trees[i][j];
            if j == 0 {
                v_table.left = Visibility::Visible;
                max_left = *val;
            } else if *val > max_left {
                v_table.left = Visibility::Visible;
                max_left = *val;
            } else {
                v_table.left = Visibility::Hidden
            }
        }
    }

    // right
    for i in 0..map.height {
        let mut max_right = 0;
        for j in (0..map.width).rev() {
            let (val, v_table) = &mut map.trees[i][j];
            if j == map.width - 1 {
                v_table.right = Visibility::Visible;
                max_right = *val;
            } else if *val > max_right {
                v_table.right = Visibility::Visible;
                max_right = *val;
            } else {
                v_table.right = Visibility::Hidden
            }
        }
    }

    //top
    for j in 0..map.width {
        let mut max_top = 0;
        for i in 0..map.height {
            let (val, v_table) = &mut map.trees[i][j];
            if i == 0 {
                v_table.top = Visibility::Visible;
                max_top = *val;
            } else if *val > max_top {
                v_table.top = Visibility::Visible;
                max_top = *val;
            } else {
                v_table.top = Visibility::Hidden
            }
        }
    }

    //down
    for j in 0..map.width {
        let mut max_down = 0;
        for i in (0..map.height).rev() {
            let (val, v_table) = &mut map.trees[i][j];
            if i == map.height - 1 {
                v_table.down = Visibility::Visible;
                max_down = *val;
            } else if *val > max_down {
                v_table.down = Visibility::Visible;
                max_down = *val;
            } else {
                v_table.down = Visibility::Hidden
            }
        }
    }

    return map;
}

pub fn compute_max_scenic_score(map: &Map) -> u32 {
    let mut max_scenic_score: u32 = 0;
    for i in 0..map.height {
        for j in 0..map.width {
            let current_tree = map.trees[i][j].0;

            //left
            let mut left_score = 0;
            if j > 0 {
                let mut left_r = j - 1;
                while left_r >= 1 && (map.trees[i][left_r].0 < current_tree) {
                    left_score += 1;
                    left_r -= 1;
                }
                left_score += 1;               
            }

            // right
            let mut right_score = 0;
            if j < map.width - 1 {
                let mut right_r = j + 1;
                while right_r < map.width - 1 && (map.trees[i][right_r].0 < current_tree) {
                    right_score += 1;
                    right_r += 1;
                }
                right_score += 1;
            }

            // top
            let mut top_score = 0;
            if i > 0 {
                let mut top_r = i - 1;
                while top_r >= 1 && (map.trees[top_r][j].0 < current_tree) {
                    top_score += 1;
                    top_r -= 1;
                }
                top_score += 1;
            }

            // down
            let mut down_score = 0;
            if i < map.height - 1 {
                let mut down_r = i + 1;
                while down_r < map.height - 1 && (map.trees[down_r][j].0 < current_tree) {
                    down_score += 1;
                    down_r += 1;
                }
                down_score += 1; 
            }
            let scenic_score = left_score * right_score * down_score * top_score;
            
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    return max_scenic_score;
}
fn main() {
    let input = include_str!(".\\real_input.txt");
    let mut map = Map::parse(input);
    dbg!(&map);

    map = set_visibility(map);

    let part_one_answer = map
        .trees
        .iter()
        .map(|line| {
            line.iter()
                .map(|(_, v_table)| v_table.as_vec().iter().any(|&v| v == Visibility::Visible))
                .filter(|&b| b == true)
                .count()
        })
        .sum::<usize>();

    dbg!(part_one_answer);

    let part_two_answer = compute_max_scenic_score(&map);
    dbg!(part_two_answer);
}
