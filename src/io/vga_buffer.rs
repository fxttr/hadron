/*
 * Falcon_rL4
 * Copyright (C) 2022 Florian Büstgens
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use core::fmt::{Arguments, Write};

pub struct VgaBuffer {
    start: *mut u8
}

impl VgaBuffer {
    pub fn new() -> Self {
	Self {
	    start: 0xb8000 as *mut u8
	}
    }

    fn set_byte_at(&mut self, byte: u8, x: isize, y: isize) {
	unsafe {
	    let attribute: u16 = (0 << 4) | (7 & 0x0F);
	    let offset = ((y * 80) + x) * 2;
	    
	    *self.start.offset(offset) = byte | (attribute << 8) as u8;
	    *self.start.offset(offset + 1) = 0xb;
	}
    }
}

impl Write for VgaBuffer {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }

    fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> core::fmt::Result {
        core::fmt::write(&mut self, args)
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
	let mut line_cursor: isize = 0;
	let mut line_multiplier: isize = 0;
	
	Ok(for(_ , &byte) in s.as_bytes().iter().enumerate() {
	    line_cursor += 1;
	    
	    if byte as char == '\n' {
		line_multiplier += 1;
		line_cursor = 0;
		continue;
	    }

	    self.set_byte_at(byte, line_cursor, line_multiplier);

	})
    }
}
/*

0 = 0 * 79 - 0
1 = 0 + 1
2 = 0 + 2 
3 = 0 + 3
4 = 0 + 4
74 = 1 * 79 - 5
80 = 74 + 6
81 = 74 + 7
82 = 74 + 8
149 = 2 * 79 - 9
159 = 149 + 10

1234\n123\n123

0 1 2 3 4 5 6 7 8 9
....................
10 11 12 13 14 15 16 
....................
20 
....................
....................
....................
....................
*/
