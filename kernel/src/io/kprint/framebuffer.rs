/* 
 * This file is part of the Zen distribution (https://github.com/fxttr/zen).
 * Copyright (c) 2023 Florian Marrero Liestmann.
 * 
 * This program is free software: you can redistribute it and/or modify  
 * it under the terms of the GNU General Public License as published by  
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but 
 * WITHOUT ANY WARRANTY; without even the implied warranty of 
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU 
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License 
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use core::fmt::{Arguments, Write};
use lazy_static::lazy_static;
use spin::Mutex;

const WIDTH: usize = 80;
const HEIGHT: usize = 25;


lazy_static! {
    pub static ref BUFFER: Mutex<VgaWriter> = Mutex::new(VgaWriter {
	memory: unsafe { &mut *(0xb8000 as *mut Memory) },
	cursor: 0
    });
}

#[repr(transparent)]
struct Memory {
    chars: [[u8; WIDTH]; HEIGHT],
}

pub struct VgaWriter {
    memory: &'static mut Memory,
    cursor: usize
}

impl VgaWriter {
    #[inline]
    fn write_byte(&mut self, byte: u8) {
	match byte {
	    b'\n' => self.new_line(),
	    byte => {
		if self.cursor >= WIDTH {
		    self.new_line();
		}

		let row = HEIGHT - 1;
		let col = self.cursor;

		self.memory.chars[row][col] = byte;
		self.cursor += 1;
	    }
	}
    }

    #[inline]
    fn new_line(&mut self) {
	for row in 1..HEIGHT {
	    for col in 0..WIDTH {
		let character = self.memory.chars[row][col];
		self.memory.chars[row - 1][col] = character;
	    }
	}

	self.clear_row(HEIGHT - 1);
	self.cursor = 0;
    }


    #[inline]
    fn clear_row(&mut self, row: usize) {
        for col in 0..WIDTH {
            self.memory.chars[row][col] = b' ';
        }
    }
    
}

impl Write for VgaWriter {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write_str(c.encode_utf8(&mut [0; 4]))
    }

    fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> core::fmt::Result {
        core::fmt::write(&mut self, args)
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
	Ok(for byte in s.bytes() {
		self.write_byte(byte);
	})
    }
}
