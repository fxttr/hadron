/*
 * Copyright (c) 2022, Florian Büstgens
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
 * THIS SOFTWARE IS PROVIDED BY Florian Büstgens ''AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL Florian Büstgens BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/*     ____      __               
 *    / __/___ _/ /________  ____ 
 *   / /_/ __ `/ / ___/ __ \/ __ \
 *  / __/ /_/ / / /__/ /_/ / / / /
 * /_/  \__,_/_/\___/\____/_/ /_/                                
 *
 *
 * Please do not expect too much from this project. 
 *
 * "But do not ask me where I am going, As I travel in this limitless world, Where every step I take is my home."
 * - Dogen Zenji
 */

#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod api;

#[no_mangle]
pub extern "C" fn _kmain() -> ! {
    kprintln(b"Starting up...");
    
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn kprintln(msg: &[u8]) {
    let vga_buffer = 0xb8000 as *mut u8;

    for(i, &byte) in msg.iter().enumerate() {
	unsafe {
	    let offset = i as isize * 2;
	    
	    *vga_buffer.offset(offset) = byte;
	    *vga_buffer.offset(offset + 1) = 0xb;
	}
    }
}
