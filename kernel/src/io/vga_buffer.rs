/*
 * Copyright (c) 2022, Florian Marrero Liestmann
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     1. Redistributions of source code must retain the above copyright
 *        notice, this list of conditions and the following disclaimer.
 *
 *     2. Redistributions in binary form must reproduce the above copyright notice,
 *        this list of conditions and the following disclaimer in the
 *        documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY Florian Marrero Liestmann ''AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL Florian Marrero Liestmann BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
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
