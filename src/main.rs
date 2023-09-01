use std::{
    io::{self, Write},
    time::Duration,
};

const WIDTH: usize = 50;
const HEIGHT: usize = 20;
pub struct Grid([[bool; HEIGHT + 2]; WIDTH + 2]);

impl Default for Grid {
    fn default() -> Self {
        Self([[false; HEIGHT + 2]; WIDTH + 2])
    }
}

impl Grid {
    #[inline]
    fn neighbors(&self, x: usize, y: usize) -> u8 {
        let x = x + 1;
        let y = y + 1;
        macro_rules! n {
            ($(($sx: tt $x: expr, $sy: tt $y: expr))+) => {{
                let mut n = 0;
                $(if self.0[n!(@ $sx $x + x)][n!(@ $sy $y + y)] {
                    n += 1;
                })+
                n
            }};
            (@ -$n:literal + $v: ident) => { $v - $n };
            (@ +$n:literal + $v: ident) => { $v + $n };
        }
        n![(-1, -1)(+0, -1)(+1, -1)(-1, +0)(+1, +0)(-1, +1)(+0, +1)(+1, +1)]
    }

    #[inline]
    fn kill(&mut self, x: usize, y: usize) {
        self.set(x, y, false);
    }

    #[inline]
    fn spawn(&mut self, x: usize, y: usize) {
        self.set(x, y, true);
    }

    #[inline]
    fn set(&mut self, x: usize, y: usize, to: bool) {
        self.0[x + 1][y + 1] = to
    }

    #[inline]
    fn cell(&self, x: usize, y: usize) -> bool {
        self.0[x + 1][y + 1]
    }

    fn iterate(&mut self) {
        let mut neighbors = [[0u8; HEIGHT + 2]; WIDTH + 2];

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                neighbors[x + 1][y + 1] = self.neighbors(x, y);
            }
        }

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let cell = self.cell(x, y);
                let n = neighbors[x + 1][y + 1];
                match n {
                    2 | 3 if cell => continue,
                    3 if !cell => self.spawn(x, y),
                    _ => self.kill(x, y),
                }
            }
        }
    }

    fn dead(&mut self) -> bool {
        for row in self.0 {
            for col in row {
                if col {
                    return false;
                };
            }
        }
        true
    }

    fn print(&self, mut w: impl Write) -> io::Result<()> {
        let tab = ["░░", "██"];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                w.write_all(tab[self.cell(x, y) as usize].as_bytes())?;
            }
            writeln!(w)?;
        }
        Ok(())
    }
}

fn main() {
    let mut grid = Grid::default();
    let img = image::open("src/seed.png").unwrap().to_luma8();
    assert!(img.width() == WIDTH as u32);
    assert!(img.height() == HEIGHT as u32);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let on = img.get_pixel(x as u32, y as u32)[0] <= 128;
            grid.set(x, y, on);
        }
    }

    while !grid.dead() {
        print!("[H[2J[3J");
        grid.print(std::io::stdout().lock()).unwrap();
        grid.iterate();
        std::thread::sleep(Duration::from_millis(100));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn neb() {
        let mut g = Grid::default();
        assert_eq!(g.neighbors(0, 0), 0);
        assert_eq!(g.neighbors(5, 5), 0);
        g.spawn(5, 5);
        assert_eq!(g.neighbors(4, 4), 1);
        g.spawn(4, 5);
        assert_eq!(g.neighbors(4, 4), 2);
    }
}
