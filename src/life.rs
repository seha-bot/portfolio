use crate::raw_draw::{RawColor, RawDraw};

pub struct Life {
    width: u32,
    height: u32,
    grid: Vec<u8>,
    buf: Vec<u8>,
}

impl<'buf> RawDraw<'buf> for Life {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn buf(&'buf self) -> &'buf Vec<u8> {
        &self.buf
    }

    fn buf_mut(&'buf mut self) -> &'buf mut Vec<u8> {
        &mut self.buf
    }
}

impl Life {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            grid: vec![0; (width * height) as usize],
            buf: vec![0; 4 * (width * height) as usize],
        }
    }

    #[inline]
    const fn index(&self, x: u32, y: u32) -> usize {
        (x + y * self.width) as usize
    }

    const LIFE_MASK: u8 = 0x80;

    #[inline]
    const fn is_set(x: u8) -> bool {
        x & Self::LIFE_MASK == Self::LIFE_MASK
    }

    #[inline]
    const fn set_life(x: u8) -> u8 {
        x | Self::LIFE_MASK
    }

    #[inline]
    const fn reset_life(x: u8) -> u8 {
        x & !Self::LIFE_MASK
    }

    #[inline]
    pub fn set(&mut self, x: u32, y: u32) {
        let i = self.index(x, y);
        let w = self.width as usize;
        self.grid[i] = Self::set_life(self.grid[i]);
        self.grid[i - 1] += 1;
        self.grid[i + 1] += 1;
        self.grid[i - w] += 1;
        self.grid[i + w] += 1;
        self.grid[i - w - 1] += 1;
        self.grid[i - w + 1] += 1;
        self.grid[i + w - 1] += 1;
        self.grid[i + w + 1] += 1;

        self.put_buf(x, y, RawColor::from_single(255));
    }

    #[inline]
    fn reset(&mut self, x: u32, y: u32) {
        let i = self.index(x, y);
        let w = self.width as usize;
        self.grid[i] = Self::reset_life(self.grid[i]);
        self.grid[i - 1] -= 1;
        self.grid[i + 1] -= 1;
        self.grid[i - w] -= 1;
        self.grid[i + w] -= 1;
        self.grid[i - w - 1] -= 1;
        self.grid[i - w + 1] -= 1;
        self.grid[i + w - 1] -= 1;
        self.grid[i + w + 1] -= 1;

        self.put_buf(x, y, RawColor::from_single(0));
    }

    pub fn tick(&mut self) {
        let grid = self.grid.clone();
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let i = self.index(x, y);
                let cnt = Self::reset_life(grid[i]);
                if Self::is_set(grid[i]) {
                    if cnt != 2 && cnt != 3 {
                        self.reset(x, y);
                    }
                } else if cnt == 3 {
                    self.set(x, y);
                }
            }
        }
    }
}
