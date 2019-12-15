use std::fs::File;
use std::io::{BufRead, BufReader};
use either::Either;
use std::collections::HashSet;

const GRID_SIZE : usize = 30000;
const START_POINT : i32 = 15000;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Markers {
    Empty = 1,
    Start,
    Intersection,
    LongitudinalMove,
    LateralMove,
    PathsCrossed
}

type Grid = Vec<Vec<Markers>>;
type Point = (i32, i32);

fn manhattan_distance(p1: Point, p2: Point) -> usize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    ((x1-x2).abs() + (y1-y2).abs()) as usize
}

fn find_closest_intersection(start: Point, paths_crossed: Vec<Point>) -> usize {
    let mut leader = manhattan_distance(start, paths_crossed[0]);

    for point in paths_crossed {
        let distance = manhattan_distance(start, point);
        if distance < leader {
            leader = distance;
        }
    }

    leader
}

fn parse_instruction(instruction: String) -> (char, i32) {
    let steps = &instruction[1..];
    let steps = steps.parse::<i32>().unwrap();
    (instruction.chars().next().unwrap(), steps)
}

fn move_to(grid: &mut Grid, paths_crossed: &mut Vec<Point>, start: Point, end: Point) -> Point {
   let (x1, y1) = start;
   let (x2, y2) = end;
   
   if x1 == x2 { // moving long
       let range = if y1 < y2 { Either::Left((y1+1)..=y2) } else { Either::Right((y2..y1).rev( ))};
       
       for i in range {
           grid[i as usize][x1 as usize] = if grid[i as usize][x1 as usize] == Markers::LateralMove { 
               paths_crossed.push((x1, i));
               Markers::PathsCrossed
           } else if grid[i as usize][x1 as usize] == Markers::Empty { 
               Markers::LongitudinalMove
           } else {
               grid[i as usize][x1 as usize]
           }
       }

       // this will not be an intersection definitionally some of the time
       grid[y2 as usize][x1 as usize] = Markers::Intersection;

       (x1, y2)
   } else { // moving lat
       let range = if x1 < x2 { Either::Left((x1+1)..=x2) } else { Either::Right((x2..x1).rev( ))};
       for i in range {
           grid[y1 as usize][i as usize] = if grid[y1 as usize][i as usize] == Markers::LongitudinalMove { 
               paths_crossed.push((i, y1));
               Markers::PathsCrossed
           } else if grid[y1 as usize][i as usize] == Markers::Empty { 
               Markers::LateralMove
           } else {
               grid[y1 as usize][i as usize]
           }
       }

       // this will not be an intersection definitionally some of the time
       grid[y1 as usize][x2 as usize] = Markers::Intersection;

       (x2, y1)
   }
}

fn instruction_to_destination(start: Point, direction: char, steps: i32) -> Point {
    let (x, y) = start;
    match direction {
        'U' => (x, y-steps),
        'D' => (x, y+steps),
        'L' => (x-steps, y),
        'R' => (x+steps, y),
        _ => panic!("Command not found")
    }
}

fn build_wires(instruction_sets: Vec<Vec<String>>) -> (Grid, Vec<Point>) {
    let mut grid: Grid = vec![vec![Markers::Empty; GRID_SIZE] ; GRID_SIZE];
    grid[START_POINT as usize][START_POINT as usize] = Markers::Start;
    let mut paths_crossed: Vec<Point> = Vec::new();

    for instructions in instruction_sets {
        let mut start: Point = (START_POINT, START_POINT);
        for instruction in instructions {
            let (direction, steps) = parse_instruction(instruction);
            let end = instruction_to_destination(start, direction, steps);

            start = move_to(&mut grid, &mut paths_crossed, start, end);
        }
    }

    (grid, paths_crossed)

}

fn bfs(grid: Grid) -> usize {
    let mut queue = vec!((START_POINT, START_POINT, 0));
    let possible_moves = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut seen: HashSet<Point> =  HashSet::new();


    while queue.len() > 0 {
        let current_point = queue.remove(0);
        let (x, y, steps) = current_point;
        seen.insert((x, y));

        for (x1, y1) in possible_moves.iter() {
            let new_x = x + x1;
            let new_y = y + y1;
            let marker = grid[new_y as usize][new_x as usize];
            if marker == Markers::PathsCrossed {
                return steps + 1;
            }
            if marker != Markers::Empty && marker != Markers::Start && !seen.contains(&(new_x, new_y)) {
                queue.push((new_x, new_y, steps + 1));
            }
        }
    }

    0
}

fn main() {
    let filename = "src/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let instruction_sets: Vec<Vec<String>> = reader.lines().into_iter().map({ |line: Result<String, std::io::Error> |
        line.unwrap().split(",").into_iter().map({|s|
            s.trim().to_string()
        }).collect()
    }).collect();

    let (_grid, paths_crossed) = build_wires(instruction_sets);

    println!("{}", find_closest_intersection((START_POINT, START_POINT), paths_crossed));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_manhattan_distance() {
        assert_eq!(manhattan_distance((0, 0), (2, 2)), 4);
        assert_eq!(manhattan_distance((10, 2), (3, 20)), 25);
        assert_eq!(manhattan_distance((1, 3), (2, 20)), 18);
        assert_eq!(manhattan_distance((302020, 9329), (3, 50303939)), 50596627);
    }

    #[test]
    fn test_closest_interaction() {
        assert_eq!(find_closest_intersection((0, 0), vec!((10, 10), (2, 2), (4, 4))), 4);
        assert_eq!(find_closest_intersection((0, 0), vec!((0, 0), (2, 2), (4, 4))), 0);
        assert_eq!(find_closest_intersection((30, 30), vec!((10, 10), (2, 2), (4, 4))), 40);
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(parse_instruction("R40".to_string()), ('R', 40));
        assert_eq!(parse_instruction("L0".to_string()), ('L', 0));
        assert_eq!(parse_instruction("U300202".to_string()), ('U', 300202));
        assert_eq!(parse_instruction("D4".to_string()), ('D', 4));
    }

    #[test]
    fn test_move_to() {
        let mut grid: Grid = vec![vec![Markers::Empty; 10] ; 10];
        let mut paths_crossed: Vec<Point> = Vec::new();
        move_to(&mut grid, &mut paths_crossed, (0, 0), (9, 0));
        let mut lateral_moves = vec![Markers::LateralMove; 10];
        lateral_moves[0] = Markers::Empty;
        lateral_moves[9] = Markers::Intersection;
            
        assert_eq!(grid[0], lateral_moves);
        move_to(&mut grid, &mut paths_crossed, (0, 0), (0, 9));
        for (i, col) in grid.iter().enumerate() {
            if i == 0 {
                assert_eq!(col[0], Markers::Empty);
            } else if i == 9 {
                assert_eq!(col[0], Markers::Intersection);
            } else {
                assert_eq!(col[0], Markers::LongitudinalMove);
            }
        }

        for (i, col) in grid.iter().enumerate() {
            for (j, row) in col.iter().enumerate() {
                if i == 0 || j == 0 {
                    continue;
                }
                assert_eq!(*row, Markers::Empty);
            }
        }
        assert_eq!(paths_crossed.len(), 0);
        let mut grid: Grid = vec![vec![Markers::Empty; 10] ; 10];
        let mut paths_crossed: Vec<Point> = Vec::new();
        move_to(&mut grid, &mut paths_crossed, (0, 4), (9, 4));
        move_to(&mut grid, &mut paths_crossed, (4, 0), (4, 9));
        assert_eq!(grid[3][4], Markers::LongitudinalMove);
        assert_eq!(grid[4][4], Markers::PathsCrossed);

        assert_eq!(paths_crossed.len(), 1);
    }

    #[test]
    fn test_instructions_to_destination() {
        assert_eq!(instruction_to_destination((100, 100), 'U', 100), (100, 0));
        assert_eq!(instruction_to_destination((100, 100), 'D', 100), (100, 200));
        assert_eq!(instruction_to_destination((100, 100), 'L', 100), (0, 100));
        assert_eq!(instruction_to_destination((100, 100), 'R', 100), (200, 100));
    }

    #[test]
    fn test_build_wires() {
        let instruction_sets = vec!(vec!("D2".to_string(), "R3".to_string()), vec!("R2".to_string(), "D3".to_string()));

        let (_grid, paths_crossed) = build_wires(instruction_sets);
        assert_eq!(paths_crossed, vec!((START_POINT + 2, START_POINT + 2)))
    }

    #[test]
    fn test_bfs() {
        let wire1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(",").into_iter().map({|s|
            s.trim().to_string()
        }).collect();

        let wire2 = "U62,R66,U55,R34,D71,R55,D58,R83".split(",").into_iter().map({|s|
            s.trim().to_string()
        }).collect();

        let instruction_sets = vec!(wire1, wire2);

        let (grid, _paths_crossed) = build_wires(instruction_sets);
        assert_eq!(bfs(grid), 610);
        // assert_eq!(paths_crossed, vec!((START_POINT + 2, START_POINT + 2)))
    }
}
