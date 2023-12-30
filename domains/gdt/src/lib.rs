/*
 * This file is part of the hadron distribution (https://github.com/fxttr/hadron).
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
pub mod export;
mod internal;

use lazy_static::lazy_static;
use security::core::x86_64::segmentation::{Descriptor, Segment, TaskStateSegment};
use uio::kprintln;
use x86_64::structures::memory::VirtualAddress;

use crate::{export::GlobalDescriptorTable, internal::SegmentSelectors};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const STACK_SIZE: usize = 4096 * 5;

lazy_static! {
    static ref TASK_STATE_SEGMENT: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtualAddress::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };

        tss
    };
}

lazy_static! {
    static ref GLOBAL_DESCRIPTOR_TABLE: (GlobalDescriptorTable, SegmentSelectors) = {
        let mut global_descriptor_table = GlobalDescriptorTable::new();
        let code_segment_selector = global_descriptor_table.add(Descriptor::kernel_code_segment());
        let tss_segment_selector =
            global_descriptor_table.add(Descriptor::tss_segment(&TASK_STATE_SEGMENT));

        (
            global_descriptor_table,
            SegmentSelectors {
                code_segment_selector,
                tss_segment_selector,
            },
        )
    };
}

pub fn init() {
    use core::arch::asm;
    use security::core::x86_64::segmentation::CodeSegment;

    GLOBAL_DESCRIPTOR_TABLE.0.init();

    #[cfg(debug_assertions)]
    kprintln!(
        "GDT intialized.\nCS: {}\nTSS: {}",
        GLOBAL_DESCRIPTOR_TABLE.1.code_segment_selector.0,
        GLOBAL_DESCRIPTOR_TABLE.1.tss_segment_selector.0
    );

    unsafe {
        CodeSegment::set_reg(GLOBAL_DESCRIPTOR_TABLE.1.code_segment_selector);

        asm!(
            "ltr {0:x}",
            in(reg) GLOBAL_DESCRIPTOR_TABLE.1.tss_segment_selector.0,
            options(nostack, preserves_flags)
        );
    }
}
