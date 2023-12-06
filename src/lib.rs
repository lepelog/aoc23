pub struct AocGrid<'a> {
    input: &'a [u8],
    pub width: usize,
    pub height: usize,
}

impl<'a> AocGrid<'a> {
    pub fn from_input(input: &'a [u8]) -> Self {
        let width = input
            .iter()
            .position(|c| *c == b'\n')
            .unwrap_or(input.len());
        let height = input.len() / (width + 1);
        Self {
            input,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        if y >= self.height || x >= self.width {
            return None;
        }
        let pos = y * (self.width + 1) + x;
        self.input.get(pos).cloned()
    }

    pub fn get_with_offset(&self, x: usize, xd: isize, y: usize, yd: isize) -> Option<u8> {
        let new_x = x.checked_add_signed(xd)?;
        let new_y = y.checked_add_signed(yd)?;
        self.get(new_x, new_y)
    }
}

pub fn chr_to_num(chr: u8) -> Option<u8> {
    matches!(chr, b'0'..=b'9').then(|| (chr - b'0'))
}
