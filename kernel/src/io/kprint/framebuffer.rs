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
use limine::{NonNullPtr, Framebuffer};
use spin::Mutex;

use crate::io::init;
use crate::io::kprint::font::{FONT, FONT_DIMENSIONS};

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

lazy_static! {
    pub static ref WRITER: Mutex<FramebufferWriter> = Mutex::new(FramebufferWriter::new());
}

pub enum Colors {
    Red,
    Green,
    White,
    Black
}

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

pub struct FramebufferWriter {
    framebuffer: &'static NonNullPtr<Framebuffer>,
    row: u16,
    col: u16,
    cursor: usize,
}

impl Pixel {
	pub fn new(r: u8, g: u8, b: u8) -> Self {
		Self { r, g, b }
	}

	pub fn set(&mut self, to: impl Into<Pixel>) {
		let to: Pixel = to.into();
		self.r = to.r;
		self.g = to.g;
		self.b = to.b;
	}

	pub fn reset(&mut self) {
		self.r = 255;
		self.g = 255;
		self.b = 255;
	}

    fn as_bits(&self) -> u32 {
		(self.r as u32) << 16 | (self.g as u32) << 8 | self.b as u32
	}
}

impl Default for Pixel {
	fn default() -> Self {
		Self { r: 0, g: 0, b: 0 }
	}
}

impl From<Colors> for Pixel {
    fn from(c: Colors) -> Self {
		match c {
			Colors::Red => Self::new(255, 0, 0),
			Colors::Green => Self::new(0, 255, 0),
			Colors::White => Self::new(255, 255, 255),
            Colors::Black => Self::new(0, 0, 0)
		}
	}
}

impl FramebufferWriter {
    pub fn new() -> Self {
        Self {
            framebuffer: init(),
            row: 0,
            col: 0,
            cursor: 0
        }
    }

    #[inline]
    fn write_char(&mut self, char: char) {
        match char {
            '\n' => self.new_line(),
            '\t' => self.tab(),
            _ => {
                let offset = (char as u8 - 32) as usize * 16;
            }
        }
    }

    #[inline]
    fn new_line(&mut self) {
        self.row += FONT_DIMENSIONS.1 as u16;
        self.col = 0;
    }

    #[inline]
    fn tab(&mut self) {
        for _ in 0..12 {
            self.write_char(' ');
        }
    } 

    #[inline]
    fn clear_row(&mut self, row: usize) {
    
    }
}

impl Write for FramebufferWriter {
    fn write_fmt(mut self: &mut Self, args: Arguments<'_>) -> core::fmt::Result {
        core::fmt::write(&mut self, args)
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(for char in s.chars() {
            self.write_char(char);
        })
    }
}
