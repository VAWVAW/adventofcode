use std::{collections::VecDeque, fs};

fn one(input: String) {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum MirrorTile {
        TopLeft,
        TopRight,
    }
    use MirrorTile::{TopLeft, TopRight};
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum SplitterTile {
        Horizontal,
        Vertical,
    }
    use SplitterTile::{Horizontal, Vertical};
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Tile {
        Empty,
        Mirror(MirrorTile),
        Splitter(SplitterTile),
    }
    use Tile::{Empty, Mirror, Splitter};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TileData {
        tile: Tile,

        /// lshifted by Direction as u8
        hit_by: u8,
    }
    impl From<Tile> for TileData {
        fn from(value: Tile) -> Self {
            Self {
                tile: value,
                hit_by: 0,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Direction {
        Top,
        Right,
        Bottom,
        Left,
    }
    use Direction::{Bottom, Left, Right, Top};
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Ray {
        x: usize,
        y: usize,
        direction: Direction,
    }
    impl Ray {
        fn step(self, rays: &mut VecDeque<Self>, max_x: usize, max_y: usize) {
            let (x, y) = match self.direction {
                Top => {
                    if self.y == 0 {
                        return;
                    };
                    (self.x, self.y - 1)
                }
                Bottom => {
                    if self.y == max_y {
                        return;
                    };
                    (self.x, self.y + 1)
                }
                Right => {
                    if self.x == max_x {
                        return;
                    };
                    (self.x + 1, self.y)
                }
                Left => {
                    if self.x == 0 {
                        return;
                    };
                    (self.x - 1, self.y)
                }
            };
            rays.push_back(Self {
                x,
                y,
                direction: self.direction,
            });
        }
    }

    let mut tiles: Vec<Vec<TileData>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Empty.into(),
                    '/' => Mirror(TopRight).into(),
                    '\\' => Mirror(TopLeft).into(),
                    '-' => Splitter(Horizontal).into(),
                    '|' => Splitter(Vertical).into(),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut rays: VecDeque<Ray> = VecDeque::from([Ray {
        x: 0,
        y: 0,
        direction: Right,
    }]);

    let max_y = tiles.len() - 1;
    let max_x = tiles[0].len() - 1;

    while let Some(ray) = rays.pop_front() {
        let tile = &mut tiles[ray.y][ray.x];

        // check if tile was hit by a ray in this direction
        if ((tile.hit_by >> ray.direction as u8) & 1) == 1 {
            continue;
        }
        tile.hit_by |= 1 << ray.direction as u8;

        match tile.tile {
            Empty => ray.step(&mut rays, max_x, max_y),
            Mirror(mirror) => {
                if TopLeft == mirror {
                    let mut new_ray = ray;
                    new_ray.direction = match ray.direction {
                        Top => Left,
                        Right => Bottom,
                        Bottom => Right,
                        Left => Top,
                    };
                    new_ray.step(&mut rays, max_x, max_y);
                } else {
                    let mut new_ray = ray;
                    new_ray.direction = match ray.direction {
                        Top => Right,
                        Right => Top,
                        Bottom => Left,
                        Left => Bottom,
                    };
                    new_ray.step(&mut rays, max_x, max_y);
                }
            }
            Splitter(splitter) => {
                if Horizontal == splitter {
                    if Right == ray.direction || Left == ray.direction {
                        ray.step(&mut rays, max_x, max_y)
                    } else {
                        let mut ray_right = ray;
                        let mut ray_left = ray;
                        ray_right.direction = Right;
                        ray_left.direction = Left;

                        ray_right.step(&mut rays, max_x, max_y);
                        ray_left.step(&mut rays, max_x, max_y);
                    }
                } else {
                    if Top == ray.direction || Bottom == ray.direction {
                        ray.step(&mut rays, max_x, max_y)
                    } else {
                        let mut ray_top = ray;
                        let mut ray_bottom = ray;
                        ray_top.direction = Top;
                        ray_bottom.direction = Bottom;

                        ray_top.step(&mut rays, max_x, max_y);
                        ray_bottom.step(&mut rays, max_x, max_y);
                    }
                }
            }
        }
    }

    let energized: u32 = tiles
        .iter()
        .map(|line| {
            line.iter()
                .map(|tile| if tile.hit_by == 0 { 0 } else { 1 })
                .sum::<u32>()
        })
        .sum();

    println!("{energized}");
}

fn two(input: String) {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum MirrorTile {
        TopLeft,
        TopRight,
    }
    use MirrorTile::{TopLeft, TopRight};
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum SplitterTile {
        Horizontal,
        Vertical,
    }
    use SplitterTile::{Horizontal, Vertical};
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Tile {
        Empty,
        Mirror(MirrorTile),
        Splitter(SplitterTile),
    }
    use Tile::{Empty, Mirror, Splitter};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct TileData {
        tile: Tile,

        /// lshifted by Direction as u8
        hit_by: u8,
    }
    impl From<Tile> for TileData {
        fn from(value: Tile) -> Self {
            Self {
                tile: value,
                hit_by: 0,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Direction {
        Top,
        Right,
        Bottom,
        Left,
    }
    use Direction::{Bottom, Left, Right, Top};
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Ray {
        x: usize,
        y: usize,
        direction: Direction,
    }
    impl Ray {
        fn step(self, rays: &mut VecDeque<Self>, max_x: usize, max_y: usize) {
            let (x, y) = match self.direction {
                Top => {
                    if self.y == 0 {
                        return;
                    };
                    (self.x, self.y - 1)
                }
                Bottom => {
                    if self.y == max_y {
                        return;
                    };
                    (self.x, self.y + 1)
                }
                Right => {
                    if self.x == max_x {
                        return;
                    };
                    (self.x + 1, self.y)
                }
                Left => {
                    if self.x == 0 {
                        return;
                    };
                    (self.x - 1, self.y)
                }
            };
            rays.push_back(Self {
                x,
                y,
                direction: self.direction,
            });
        }
    }

    fn calculate_energized(tiles: &Vec<Vec<TileData>>, initial: Ray) -> u32 {
        let mut tiles = tiles.clone();
        let mut rays: VecDeque<Ray> = VecDeque::from([initial]);

        let max_y = tiles.len() - 1;
        let max_x = tiles[0].len() - 1;

        while let Some(ray) = rays.pop_front() {
            let tile = &mut tiles[ray.y][ray.x];

            // check if tile was hit by a ray in this direction
            if ((tile.hit_by >> ray.direction as u8) & 1) == 1 {
                continue;
            }
            tile.hit_by |= 1 << ray.direction as u8;

            match tile.tile {
                Empty => ray.step(&mut rays, max_x, max_y),
                Mirror(mirror) => {
                    if TopLeft == mirror {
                        let mut new_ray = ray;
                        new_ray.direction = match ray.direction {
                            Top => Left,
                            Right => Bottom,
                            Bottom => Right,
                            Left => Top,
                        };
                        new_ray.step(&mut rays, max_x, max_y);
                    } else {
                        let mut new_ray = ray;
                        new_ray.direction = match ray.direction {
                            Top => Right,
                            Right => Top,
                            Bottom => Left,
                            Left => Bottom,
                        };
                        new_ray.step(&mut rays, max_x, max_y);
                    }
                }
                Splitter(splitter) => {
                    if Horizontal == splitter {
                        if Right == ray.direction || Left == ray.direction {
                            ray.step(&mut rays, max_x, max_y)
                        } else {
                            let mut ray_right = ray;
                            let mut ray_left = ray;
                            ray_right.direction = Right;
                            ray_left.direction = Left;

                            ray_right.step(&mut rays, max_x, max_y);
                            ray_left.step(&mut rays, max_x, max_y);
                        }
                    } else {
                        if Top == ray.direction || Bottom == ray.direction {
                            ray.step(&mut rays, max_x, max_y)
                        } else {
                            let mut ray_top = ray;
                            let mut ray_bottom = ray;
                            ray_top.direction = Top;
                            ray_bottom.direction = Bottom;

                            ray_top.step(&mut rays, max_x, max_y);
                            ray_bottom.step(&mut rays, max_x, max_y);
                        }
                    }
                }
            }
        }

        let energized: u32 = tiles
            .iter()
            .map(|line| {
                line.iter()
                    .map(|tile| if tile.hit_by == 0 { 0 } else { 1 })
                    .sum::<u32>()
            })
            .sum();
        energized
    }

    let tiles: Vec<Vec<TileData>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Empty.into(),
                    '/' => Mirror(TopRight).into(),
                    '\\' => Mirror(TopLeft).into(),
                    '-' => Splitter(Horizontal).into(),
                    '|' => Splitter(Vertical).into(),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let max_y = tiles.len() - 1;
    let max_x = tiles[0].len() - 1;

    let max_energized = (0..4u8)
        .map(|i| match i {
            0 => Right,
            1 => Top,
            2 => Left,
            3 => Bottom,
            _ => unreachable!(),
        })
        .map(|start_direction| match start_direction {
            Top => (0..=max_x)
                .map(|x| Ray {
                    x,
                    y: max_y,
                    direction: start_direction,
                })
                .map(|start_ray| calculate_energized(&tiles, start_ray))
                .max()
                .unwrap(),
            Right => (0..=max_y)
                .map(|y| Ray {
                    x: 0,
                    y,
                    direction: start_direction,
                })
                .map(|start_ray| calculate_energized(&tiles, start_ray))
                .max()
                .unwrap(),
            Bottom => (0..=max_x)
                .map(|x| Ray {
                    x,
                    y: 0,
                    direction: start_direction,
                })
                .map(|start_ray| calculate_energized(&tiles, start_ray))
                .max()
                .unwrap(),
            Left => (0..=max_y)
                .map(|y| Ray {
                    x: max_x,
                    y,
                    direction: start_direction,
                })
                .map(|start_ray| calculate_energized(&tiles, start_ray))
                .max()
                .unwrap(),
        })
        .max()
        .unwrap();

    println!("{max_energized}");
}

fn main() {
    let (execute_first, file_name) = match std::env::args()
        .nth(1)
        .unwrap_or("test1".to_owned())
        .as_str()
    {
        "1" => (true, "data.txt"),
        "2" => (false, "data.txt"),
        "t1" => (true, "data_test1.txt"),
        "t2" => (false, "data_test2.txt"),
        _ => (true, "data_test1.txt"),
    };
    let input = fs::read_to_string(file_name).unwrap();

    if execute_first {
        one(input);
    } else {
        two(input);
    }
}
