/*
 * LumOS::rL4
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
 *  _                     ___  ____
 * | |   _   _ _ __ ___  / _ \/ ___|
 * | |  | | | | '_ ` _ \| | | \___ \
 * | |__| |_| | | | | | | |_| |___) |
 * |_____\__,_|_| |_| |_|\___/|____/
*/

#![no_std]
#![no_main]
#![feature(allow_internal_unstable)]

use core::panic::PanicInfo;

pub mod api;

#[macro_use]
pub mod io;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kprintln!("Copyright (C) 2022 Florian Buestgens\n  _____     _                          _    _  _\n |  ___|_ _| | ___ ___  _ __  _ _ _ __| |  | || |\n | |_ / _` | |/ __/ _ \\| '_ \\(_|_) '__| |  | || |_\n |  _| (_| | | (_| (_) | | | |_ _| |  | |__|__   _|\n |_|  \\__,_|_|\\___\\___/|_| |_(_|_)_|  |_____| |_|");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
