use anyhow::Result;
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::Array2;

type Point = (isize, isize);
#[derive(Debug)]
struct Shape {
    coordinates: Vec<Point>,
}
#[derive(Debug, Clone)]
enum Material {
    Air,
    Solid,
    Sand,
}

struct CaveSystem {
    map: Array2<Material>,
    spawn: Point,
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
    fn simulate_sand(&mut self) {}
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
        .map(|x| x.into_iter().map(|y| y.trim()).collect())
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
                        .map(|item| item.parse::<isize>().unwrap())
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
    println!("{:?}", cave.map);
    //dbg!(cave.map);
    Ok(())
}
