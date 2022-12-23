use anyhow::Result;
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::Array2;

type Point = (usize, usize);
#[derive(Debug)]
struct Shape {
    coordinates: Vec<Point>,
}
#[derive(Debug, Clone)]
enum Material {
    Air,
    Solid,
    // true means falling
    Sand,
}

struct CaveSystem {
    map: Array2<Material>,
    spawn: Point,
}
enum Direction {
    Left,
    Right,
    Stay,
}
impl CaveSystem {
    fn new(shape: (usize, usize), spawn: Point) -> Self {
        let map = Array2::<Material>::from_shape_simple_fn(shape, || Material::Air);
        Self { map, spawn }
    }
    fn draw_structures(&mut self, shapes: Vec<Shape>) {
        for shape in shapes {
            for segment in shape.coordinates.windows(2) {
                let leftx = std::cmp::min(segment[0].0, segment[1].0);
                let rightx = std::cmp::max(segment[0].0, segment[1].0);
                let lefty = std::cmp::min(segment[0].1, segment[1].1);
                let righty = std::cmp::max(segment[0].1, segment[1].1);
                self.map
                    .slice_mut(s![leftx..=rightx, lefty..=righty])
                    .fill(Material::Solid);
            }
        }
    }
    fn check_left_right(&self, pos: Point) -> Option<Direction> {
        let left = self.map.get(((pos.0) + 1, (pos.1).saturating_sub(1)))?;
        let right = self.map.get(((pos.0) + 1, (pos.1) + 1))?;

        match (left, right) {
            (Material::Air, _) => Some(Direction::Left),
            (Material::Sand | Material::Solid, Material::Air) => Some(Direction::Right),
            (Material::Sand | Material::Solid, Material::Sand | Material::Solid) => {
                Some(Direction::Stay)
            }
        }
    }
    fn simulate_sand(&mut self) -> usize {
        let (spwn_x, spwn_y) = self.spawn;
        let mut count = 0;
        loop {
            //spawn sand
            self.map[[spwn_x, spwn_y]] = Material::Sand;
            'inner: loop {
                let maybe_new_pos1 = self.sand_fall_down(self.spawn);
                let Some(new_pos1) = maybe_new_pos1 else { return count};
                let maybe_dir = self.check_left_right(new_pos1);
                let Some(dir) = maybe_dir else { return count};
                let new_pos2 = self.fall_to(new_pos1, dir);
                if new_pos1 == new_pos2 {
                    //sand came to rest
                    count += 1;
                    break 'inner;
                }
            }
        }
    }
    // returns new pos after falling straigt down as far as possible
    fn sand_fall_down(&mut self, pos: Point) -> Option<Point> {
        let next_pos = self.find_free_from(pos)?;
        self.map[[pos.0, pos.1]] = Material::Air;
        self.map[[pos.0, next_pos]] = Material::Sand;
        Some((pos.0, next_pos))
    }
    fn fall_to(&mut self, pos: Point, dir: Direction) -> Point {
        self.map[[pos.0, pos.1]] = Material::Air;
        match dir {
            Direction::Left => {
                self.map[[(pos.0) + 1, (pos.1).saturating_sub(1)]] = Material::Sand;
                ((pos.0) + 1, (pos.1).saturating_sub(1))
            }
            Direction::Right => {
                self.map[[(pos.0) + 1, (pos.1) + 1]] = Material::Sand;
                ((pos.0) + 1, (pos.1) + 1)
            }
            Direction::Stay => pos,
        }
    }
    fn find_free_from(&self, pos: Point) -> Option<usize> {
        self.map
            .slice(s![pos.0, pos.1..])
            .iter()
            .position(|x| match x {
                Material::Sand | Material::Solid => true,
                _ => false,
            })
    }
}

fn parse_input(path: &str) -> Result<Vec<Shape>> {
    let input = std::fs::read_to_string(path)?;
    // split into lines
    let lines: Vec<&str> = input.lines().collect();
    // split into single points as &str
    let splits: Vec<Vec<&str>> = lines
        .into_iter()
        .map(|line| line.split("->").collect::<Vec<&str>>())
        .collect();
    // trim whitespace
    let trimmed: Vec<Vec<&str>> = splits
        .into_iter()
        .map(|x| x.into_iter().map(|x| x.trim()).collect())
        .collect();
    let tuples: Vec<Shape> = trimmed
        .into_iter()
        //create Shapes from Vec<Vec<&str>>
        .map(|shape| Shape {
            coordinates: shape
                .into_iter()
                .map(|item| {
                    // split and parse into isize
                    item.split(",")
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .map(|item| item.parse::<usize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect::<Vec<Point>>(),
        })
        .collect();
    Ok(tuples)
}

fn main() -> Result<()> {
    let parsed_input = parse_input("input.txt")?;

    let max_x = parsed_input
        .iter()
        .map(|x| x.coordinates.iter().map(|x| x.0).max().unwrap())
        .max()
        .unwrap()
        + 1;
    let max_y = parsed_input
        .iter()
        .map(|x| x.coordinates.iter().map(|x| x.1).max().unwrap())
        .max()
        .unwrap()
        + 1;
    println!("maxx: {}, maxy: {}", max_x, max_y);

    let spawn = (500, 0);
    let mut cave = CaveSystem::new((max_x as usize, max_y as usize), spawn);
    cave.draw_structures(parsed_input);
    let count = cave.simulate_sand();
    println!("Count:{}", count);
    //dbg!(cave.map);
    Ok(())
}
