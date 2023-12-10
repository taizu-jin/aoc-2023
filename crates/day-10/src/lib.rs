use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum PartKind {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Start,
}

impl TryFrom<&char> for PartKind {
    type Error = ();

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        let kind = match value {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            'S' => Self::Start,
            _ => return Err(()),
        };

        Ok(kind)
    }
}

impl Display for PartKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PartKind::NS => write!(f, "|"),
            PartKind::EW => write!(f, "-"),
            PartKind::NE => write!(f, "L"),
            PartKind::NW => write!(f, "J"),
            PartKind::SW => write!(f, "7"),
            PartKind::SE => write!(f, "F"),
            PartKind::Start => write!(f, "S"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: i32,
    y: i32,
    kind: PartKind,
}

impl Part {
    fn new(x: i32, y: i32, kind: impl TryInto<PartKind>) -> Result<Self, ()> {
        let kind = match kind.try_into() {
            Ok(kind) => kind,
            Err(_) => return Err(()),
        };
        Ok(Self { x, y, kind })
    }

    fn is_connecting(&self, other: &Part) -> bool {
        match (self.kind, other.kind) {
            (PartKind::NS, PartKind::NS) if self.x == other.x && self.y.abs_diff(other.y) == 1 => {
                true
            }
            (PartKind::EW, PartKind::EW) if self.x.abs_diff(other.x) == 1 && self.y == other.y => {
                true
            }
            (PartKind::NS, PartKind::NE)
            | (PartKind::NS, PartKind::NW)
            | (PartKind::SW, PartKind::NE)
            | (PartKind::SW, PartKind::NS)
            | (PartKind::SW, PartKind::NW)
            | (PartKind::SE, PartKind::NS)
            | (PartKind::SE, PartKind::NE)
            | (PartKind::SE, PartKind::NW)
            | (PartKind::Start, PartKind::NW)
            | (PartKind::Start, PartKind::NE)
            | (PartKind::Start, PartKind::NS)
            | (PartKind::SE, PartKind::Start)
            | (PartKind::SW, PartKind::Start)
            | (PartKind::NS, PartKind::Start)
                if self.x == other.x && self.y + 1 == other.y =>
            {
                true
            }
            (PartKind::NS, PartKind::SW)
            | (PartKind::NS, PartKind::SE)
            | (PartKind::NE, PartKind::NS)
            | (PartKind::NE, PartKind::SW)
            | (PartKind::NE, PartKind::SE)
            | (PartKind::NW, PartKind::NS)
            | (PartKind::NW, PartKind::SE)
            | (PartKind::NW, PartKind::SW)
            | (PartKind::Start, PartKind::SE)
            | (PartKind::Start, PartKind::SW)
            | (PartKind::Start, PartKind::NS)
            | (PartKind::NS, PartKind::Start)
            | (PartKind::NE, PartKind::Start)
            | (PartKind::NW, PartKind::Start)
                if self.x == other.x && self.y - 1 == other.y =>
            {
                true
            }
            (PartKind::EW, PartKind::NE)
            | (PartKind::EW, PartKind::SE)
            | (PartKind::NW, PartKind::EW)
            | (PartKind::NW, PartKind::NE)
            | (PartKind::NW, PartKind::SE)
            | (PartKind::SW, PartKind::EW)
            | (PartKind::SW, PartKind::NE)
            | (PartKind::SW, PartKind::SE)
            | (PartKind::Start, PartKind::EW)
            | (PartKind::Start, PartKind::SE)
            | (PartKind::Start, PartKind::NE)
            | (PartKind::EW, PartKind::Start)
            | (PartKind::NW, PartKind::Start)
            | (PartKind::SW, PartKind::Start)
                if self.x - 1 == other.x && self.y == other.y =>
            {
                true
            }
            (PartKind::EW, PartKind::NW)
            | (PartKind::EW, PartKind::SW)
            | (PartKind::NE, PartKind::EW)
            | (PartKind::NE, PartKind::NW)
            | (PartKind::NE, PartKind::SW)
            | (PartKind::SE, PartKind::EW)
            | (PartKind::SE, PartKind::NW)
            | (PartKind::SE, PartKind::SW)
            | (PartKind::Start, PartKind::EW)
            | (PartKind::Start, PartKind::NW)
            | (PartKind::Start, PartKind::SW)
            | (PartKind::EW, PartKind::Start)
            | (PartKind::NE, PartKind::Start)
            | (PartKind::SE, PartKind::Start)
                if self.x + 1 == other.x && self.y == other.y =>
            {
                true
            }
            _ => false,
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug)]
struct Pipe {
    start: Part,
    end: Part,
    has_start: bool,
    parts: VecDeque<Part>,
}

impl Pipe {
    fn new(part: Part) -> Self {
        Self {
            start: part,
            end: part,
            has_start: PartKind::Start == part.kind,
            parts: vec![part].into(),
        }
    }

    fn is_loop(&self) -> bool {
        self.end.is_connecting(&self.start) || self.start.is_connecting(&self.end)
    }

    fn is_part_connecting(&self, part: &Part) -> bool {
        self.start.is_connecting(part) || self.end.is_connecting(part)
    }

    fn connect_part(&mut self, part: Part) -> Option<(Part, Part)> {
        if self.start.is_connecting(&part) {
            let old = self.start;

            self.start = part;
            self.parts.push_back(part);

            if part.kind == PartKind::Start {
                self.has_start = true;
            }

            Some((self.start, old))
        } else if self.end.is_connecting(&part) {
            let old = self.end;

            self.end = part;
            self.parts.push_front(part);

            if part.kind == PartKind::Start {
                self.has_start = true;
            }

            Some((self.end, old))
        } else {
            None
        }
    }

    fn connect_pipe(&mut self, pipe: &Pipe) -> Option<(Part, Part)> {
        if self.start.is_connecting(&pipe.end) {
            let old = self.start;
            self.start = pipe.start;

            for part in &pipe.parts {
                self.parts.push_back(*part);
            }

            if pipe.has_start {
                self.has_start = true;
            }

            Some((self.start, old))
        } else if self.end.is_connecting(&pipe.start) {
            let old = self.end;
            self.end = pipe.end;

            for part in pipe.parts.iter().rev() {
                self.parts.push_front(*part);
            }

            if pipe.has_start {
                self.has_start = true;
            }

            Some((self.end, old))
        } else if self.start.is_connecting(&pipe.start) {
            let old = self.start;
            self.start = pipe.end;

            for part in pipe.parts.iter().rev() {
                self.parts.push_back(*part);
            }

            if pipe.has_start {
                self.has_start = true;
            }

            Some((self.start, old))
        } else if self.end.is_connecting(&pipe.end) {
            let old = self.end;
            self.end = pipe.start;

            for part in &pipe.parts {
                self.parts.push_front(*part);
            }

            if pipe.has_start {
                self.has_start = true;
            }

            Some((self.end, old))
        } else {
            None
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for p in &self.parts {
            write!(f, "{}", p)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Maze {
    pipes: Vec<Pipe>,
    pipe_map: HashMap<(i32, i32), usize>,
    maze: Vec<char>,
    width: usize,
    height: usize,
}

impl Maze {
    fn new() -> Self {
        Self {
            pipes: Vec::new(),
            pipe_map: HashMap::new(),
            maze: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn connect_part(&mut self, part: Part) {
        let mut pipes = Vec::new();
        let mut indexes = Vec::new();

        if part.x != 0 {
            if let Some(index) = self.pipe_map.get(&(part.x - 1, part.y)) {
                indexes.push(*index);
            }
        }

        if part.y != 0 {
            if let Some(index) = self.pipe_map.get(&(part.x, part.y - 1)) {
                indexes.push(*index);
            }
        }

        if let Some(index) = self.pipe_map.get(&(part.x + 1, part.y)) {
            indexes.push(*index);
        }

        if let Some(index) = self.pipe_map.get(&(part.x, part.y + 1)) {
            indexes.push(*index);
        }

        indexes.sort_unstable();
        indexes.dedup();

        let mut iter = self.pipes.iter_mut();
        let mut offest = 0;
        for i in indexes.into_iter() {
            if let Some(pipe) = iter.nth(i - offest) {
                if pipe.is_part_connecting(&part) {
                    pipes.push((i, pipe));
                }
            }
            offest += i + 1;
        }

        let remove_pipe = match &mut pipes[..] {
            [] => {
                let pipe = Pipe::new(part);
                self.pipes.push(pipe);
                self.pipe_map.insert((part.x, part.y), self.pipes.len() - 1);
                None
            }
            [(i, pipe)] => {
                let (new, old) = pipe
                    .connect_part(part)
                    .expect("should return an old end part");
                if pipe.parts.len() > 2 {
                    self.pipe_map
                        .remove(&(old.x, old.y))
                        .expect("old end should still be in the map");
                }
                self.pipe_map.insert((new.x, new.y), *i);
                None
            }
            [(i1, p1), (i2, p2)] => {
                let (new, old) = p1
                    .connect_part(part)
                    .expect("should return an old end part");
                if p1.parts.len() > 2 {
                    self.pipe_map
                        .remove(&(old.x, old.y))
                        .expect("old end should still be in the map");
                }
                self.pipe_map.insert((new.x, new.y), *i1);

                let (new, old) = p1.connect_pipe(p2).expect("failed to connect pipes");

                self.pipe_map
                    .remove(&(old.x, old.y))
                    .expect("removing old end");

                self.pipe_map.remove(&(p2.start.x, p2.start.y));
                self.pipe_map.remove(&(p2.end.x, p2.end.y));

                self.pipe_map.insert((new.x, new.y), *i1);

                Some(*i2)
            }
            _ => unreachable!("part can't connect to more than 2 pipes"),
        };

        if let Some(index) = remove_pipe {
            self.pipes.remove(index);
            for value in self.pipe_map.values_mut().filter(|v| **v > index) {
                *value -= 1;
            }
        }
    }

    fn fill_loop(&mut self) {
        let pipe = self
            .pipes
            .iter()
            .find(|p| p.is_loop() && p.has_start)
            .unwrap();
        let pipe = pipe.parts.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>();

        let mut is_crossed = false;

        for y in 0..self.height {
            for x in 0..self.width {
                if pipe.contains(&(x as i32, y as i32)) {
                    let tile = PartKind::try_from(&self.maze[x + y * self.width]).unwrap();
                    match tile {
                        PartKind::NS | PartKind::SW | PartKind::SE | PartKind::Start => {
                            is_crossed = !is_crossed
                        }
                        _ => (),
                    }
                } else if is_crossed {
                    self.maze[x + y * self.width] = 'I';
                } else {
                    self.maze[x + y * self.width] = 'O';
                }
            }
        }
    }
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut maze = Maze::new();

        let mut is_init = false;
        maze.height = value.lines().count();

        for (y, line) in value.lines().enumerate() {
            if !is_init {
                maze.width = line.chars().count();
                maze.maze.resize_with(maze.width * maze.height, || '.');
                is_init = true;
            }
            for (x, char) in line.chars().enumerate() {
                maze.maze[x + y * maze.width] = char;
                match char {
                    '.' => continue,
                    c => {
                        let part = Part::new(x as i32, y as i32, &c).unwrap();
                        maze.connect_part(part);
                    }
                }
            }
        }

        maze
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pipe = self
            .pipes
            .iter()
            .find(|p| p.is_loop() && p.has_start)
            .unwrap();
        let pipe = pipe.parts.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>();

        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.maze[x + y * self.width];
                let is_part_of_pipe = pipe.contains(&(x as i32, y as i32));

                match (tile, is_part_of_pipe) {
                    ('S', true) => write!(f, "\x1B[31m{}\x1B[37m", tile)?,
                    ('I', _) => write!(f, "\x1B[32m{}\x1B[37m", tile)?,
                    ('O', _) => write!(f, "\x1B[90m{}\x1B[37m", tile)?,
                    (tile, true) => write!(f, "\x1B[34m{}\x1B[37m", tile)?,
                    (_, _) => write!(f, "{}", tile)?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn solve_part_1(input: &str) -> u32 {
    let maze = Maze::from(input);
    let pipe = maze
        .pipes
        .iter()
        .find(|p| p.is_loop() && p.has_start)
        .unwrap();
    (pipe.parts.len() / 2) as u32
}

pub fn solve_part_2(input: &str) -> u32 {
    let mut maze = Maze::from(input);
    maze.fill_loop();
    maze.maze.iter().filter(|c| **c == 'I').count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_connection() {
        //.F.
        //.J.
        let f = Part::new(1, 0, &'F').unwrap();
        let j = Part::new(1, 1, &'J').unwrap();

        assert!(f.is_connecting(&j));
        //.FJ.
        let f = Part::new(1, 0, &'F').unwrap();
        let j = Part::new(2, 0, &'J').unwrap();

        assert!(f.is_connecting(&j));

        //7
        //J
        let s = Part::new(57, 4, &'7').unwrap();
        let j = Part::new(57, 5, &'J').unwrap();
        assert!(j.is_connecting(&s));
    }

    #[test]
    fn pipe_connection() {
        //..F7
        //.FJ
        let mut p1 = Pipe::new(Part::new(2, 0, &'F').unwrap());
        p1.connect_part(Part::new(3, 0, &'7').unwrap());

        let mut p2 = Pipe::new(Part::new(1, 1, &'F').unwrap());
        p2.connect_part(Part::new(2, 1, &'J').unwrap());

        p1.connect_pipe(&p2);
        assert_eq!(p1.end.x, p2.end.x);

        assert_eq!(p1.end.y, p2.end.y);
    }

    #[test]
    fn part_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(solve_part_1(input), 4);

        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(solve_part_1(input), 8);
    }

    #[test]
    fn part_2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(solve_part_2(input), 4);

        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        assert_eq!(solve_part_2(input), 4);

        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        assert_eq!(solve_part_2(input), 8);

        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(solve_part_2(input), 10);
    }
}
