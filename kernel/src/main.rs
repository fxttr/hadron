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


static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        if framebuffer_response.framebuffer_count < 1 {
            hcf();
        }

        kprintln!("Copyright (C) 2023 Florian Marrero Liestmann\n");

        // Get the first framebuffer's information.
        let framebuffer = &framebuffer_response.framebuffers()[0];

        for i in 0..100_usize {
            // Calculate the pixel offset using the framebuffer information we obtained above.
            // We skip `i` scanlines (pitch is provided in bytes) and add `i * 4` to skip `i` pixels forward.
            let pixel_offset = i * framebuffer.pitch as usize + i * 4;

            // Write 0xFFFFFFFF to the provided pixel offset to fill it white.
            // We can safely unwrap the result of `as_ptr()` because the framebuffer address is
            // guaranteed to be provided by the bootloader.
            unsafe {
                *(framebuffer.address.as_ptr().unwrap().offset(pixel_offset as isize) as *mut u32) = 0xFFFFFFFF;
            }
        }
    }

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
