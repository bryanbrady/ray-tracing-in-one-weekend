use std::io::{self, Write};

#[derive(Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

pub fn color(x: f64, y: f64, z: f64) -> Color {
    Color {
        r: x,
        g: y,
        b: z
    }
}

pub fn write_color(_out: &mut impl Write, _color: Color) -> io::Result<()> {
    let ir = (255.999 * _color.r) as u64;
    let ig = (255.999 * _color.g) as u64;
    let ib = (255.999 * _color.b) as u64;
    let out = format!("{} {} {}\n", ir, ig, ib);
    _out.write_all(out.as_bytes())?;
    Ok(())
}
