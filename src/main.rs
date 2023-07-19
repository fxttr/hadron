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

pub mod api;

#[macro_use]
pub mod io;

#[no_mangle]
fn _start() -> ! {
    use bootboot::*;

    let bootboot_r = unsafe { & (*(BOOTBOOT_INFO as *const BOOTBOOT)) };

    if bootboot_r.fb_scanline > 0 {

        let fb = BOOTBOOT_FB as u64;

        for y in 0..bootboot_r.fb_height {
            let addr = fb
                + bootboot_r.fb_scanline as u64 * y as u64
                + bootboot_r.fb_width as u64 * 2;
            unsafe { *(addr as *mut u64) = 0x00FFFFFF };
        }

        for x in 0..bootboot_r.fb_width {
            let addr = fb
                + bootboot_r.fb_scanline as u64 * (bootboot_r.fb_height / 2) as u64 + (x * 4) as u64;
            unsafe { *(addr as *mut u64) = 0x00FFFFFF };
        }

        for y in 0..20 {
            for x in 0..20 {
                let addr = fb
                    + bootboot_r.fb_scanline as u64 * (y + 20) as u64
                    + (x + 20) * 4;
                unsafe { *(addr as *mut u64) = 0x00FF0000 };
            }
        }

        for y in 0..20 {
            for x in 0..20 {
                let addr = fb
                    + bootboot_r.fb_scanline as u64 * (y + 20) as u64
                    + (x + 50) * 4;
                unsafe { *(addr as *mut u64) = 0x0000FF00 };
            }
        }

        for y in 0..20 {
            for x in 0..20 {
                let addr = fb
                    + bootboot_r.fb_scanline as u64 * (y + 20) as u64
                    + (x + 80) * 4;
                unsafe { *(addr as *mut u64) = 0x000000FF };
            }
        }

        kprintln!("Copyright (C) 2023 Florian Marrero Liestmann\n");
    }

    // hang for now
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
