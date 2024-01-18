#[derive(PartialEq, Eq)]
pub struct RawColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RawColor {
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn from_single(val: u8) -> Self {
        Self {
            r: val,
            g: val,
            b: val,
            a: 255,
        }
    }
}

pub trait RawDraw<'buf> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn buf(&'buf self) -> &'buf Vec<u8>;
    fn buf_mut(&'buf mut self) -> &'buf mut Vec<u8>;

    fn get_buf(&'buf self, x: u32, y: u32) -> RawColor {
        let i = 4 * (x + y * self.width()) as usize;
        let buf = self.buf();
        RawColor::from_rgba(buf[i], buf[i + 1], buf[i + 2], buf[i + 3])
    }

    fn put_buf(&'buf mut self, x: u32, y: u32, color: RawColor) {
        let i = 4 * (x + y * self.width()) as usize;
        let buf = self.buf_mut();
        buf[i] = color.r;
        buf[i + 1] = color.g;
        buf[i + 2] = color.b;
        buf[i + 3] = color.a;
    }
}
