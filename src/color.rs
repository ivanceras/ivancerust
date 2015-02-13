//color.rs
pub struct Color{
	pub r:u8,
	pub g:u8,
	pub b:u8,
}

impl Clone for Color {
    fn clone(&self) -> Color { Color{r:self.r, g:self.g, b:self.b} }
}
