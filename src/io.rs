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
use core::fmt::{Arguments, self, Write};

use self::vga_buffer::VgaBuffer;

mod vga_buffer;

#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::io::_kprint(format_args!($($arg)*)));
}

#[allow_internal_unstable(print_internals, format_args_nl)]
macro_rules! kprintln {
    () => (kprint!("\n"));
    ($($arg:tt)*) => ({
	$crate::io::_kprint(format_args_nl!($($arg)*));
    })
}

pub fn _kprint(args: Arguments<'_>) {
    _print_to(args, VgaBuffer::new, "kernel")
}

fn _print_to<D>(args: fmt::Arguments<'_>, func: fn() -> D, label: &str)
where
    D: Write
{
    let mut driver: D = func();
    let _ = driver.write_fmt(args);
}
