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

use core::mem::size_of;

#[cfg(target_arch = "x86_64")]
use security::core::x86_64::segmentation::{Descriptor, SegmentSelector};
use x86_64::structures::{memory::VirtualAddress, table::DescriptorTablePointer};

pub struct GlobalDescriptorTable {
    table: [u64; 8],
    len: usize,
}

impl GlobalDescriptorTable {
    pub const fn new() -> Self {
        Self {
            table: [0; 8],
            len: 1, // entry 0 must be NULL
        }
    }

    #[inline]
    pub fn as_raw_slice(&self) -> &[u64] {
        &self.table[..self.len]
    }

    #[inline]
    pub const unsafe fn u_from_raw_slice(slice: &[u64]) -> GlobalDescriptorTable {
        let len: usize = slice.len();
        let mut table: [u64; 8] = [0; 8];
        let mut i: usize = 0;

        assert!(len <= 8, "A GDT cannot be longer than 8 elements.");

        while len > i {
            table[i] = slice[i];
            i += 1;
        }

        GlobalDescriptorTable { table, len }
    }

    #[inline]
    fn push(&mut self, value: u64) -> usize {
        let i = self.len;

        self.table[i] = value;
        self.len += 1;

        i
    }

    #[inline]
    pub fn add(&mut self, entry: Descriptor) -> SegmentSelector {
        let i = match entry {
            Descriptor::SystemSegment(low, high) => {
                if self.len > self.table.len().saturating_sub(2) {
                    panic!("Not enough space in GDT for holding a SystemSegment.")
                }

                let i = self.push(low);
                self.push(high);

                i
            }

            Descriptor::UserSegment(value) => {
                if self.len > self.table.len().saturating_sub(1) {
                    panic!("Not enough space in GDT for holding a UserSegment.")
                }

                self.push(value)
            }
        };

        SegmentSelector::new(i as u16, entry.dpl())
    }

    #[inline]
    pub fn init(&self) {
        unsafe {
            core::arch::asm!(
                "lgdt [{}]",
                in(reg) &self.as_descriptor_table_pointer(),
                options(readonly, nostack, preserves_flags)
            );
        }
    }

    #[inline]
    fn as_descriptor_table_pointer(&self) -> DescriptorTablePointer {
        DescriptorTablePointer {
            base: VirtualAddress::new(self.table.as_ptr() as u64),
            limit: (self.len * size_of::<u64>() - 1) as u16,
        }
    }
}
