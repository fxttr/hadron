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
}

impl Write for VgaBuffer {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }

    fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> core::fmt::Result {
        core::fmt::write(&mut self, args)
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
	Ok(for(i, &byte) in s.as_bytes().iter().enumerate() {
	    unsafe {
		let offset = i as isize * 2;
		
		*self.start.offset(offset) = byte;
		*self.start.offset(offset + 1) = 0xb;
	    }
	})
    }
}
