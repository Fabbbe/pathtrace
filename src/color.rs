use std::io;

use crate::vec3;

pub fn color_print<T: io::Write>(mut w: T, color: vec3::Vec3) -> io::Result<()> {
    writeln!(w, "{} {} {}", 
           (color.x.clamp(0.0, 1.0)*255.0) as u8,
           (color.y.clamp(0.0, 1.0)*255.0) as u8,
           (color.z.clamp(0.0, 1.0)*255.0) as u8,
    )?;
    Ok(())
}
