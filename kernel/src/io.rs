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

use limine::{NonNullPtr, Framebuffer};

use crate::except::hcf;

#[macro_use]
pub mod kprint;

static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);

pub fn init() -> &'static NonNullPtr<Framebuffer> {
    let framebuffer: &NonNullPtr<Framebuffer>;

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        if framebuffer_response.framebuffer_count < 1 {
            hcf();
        }

        // Get the first framebuffer's information.
        framebuffer = &framebuffer_response.framebuffers()[0];
    } else {
        hcf();
    }

    framebuffer
}