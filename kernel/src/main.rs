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

#![no_std]
#![no_main]
#![feature(allow_internal_unstable)]

use core::panic::PanicInfo;
use core::arch::asm;

#[macro_use]
pub mod io;
pub mod api;


#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    kprintln!("Copyright (C) 2023 Florian Marrero Liestmann\n");

    // hang for now
    hcf()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hcf()
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
