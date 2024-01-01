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

use core::arch::asm;
use core::marker::PhantomData;

use x86_64::structures::memory::VirtualAddress;
use x86_64::structures::table::DescriptorTablePointer;
use x86_64::types::paging::PageFaultErrorCode;

use crate::export::InterruptDescriptorTable;

pub type InterruptHandlerFunction = extern "x86-interrupt" fn(&mut InterruptStackFrame);
pub type InterruptHandlerFunctionWithErrorCode =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
pub type PageFaultInterruptHandlerFunction =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: PageFaultErrorCode);

#[repr(C)]
pub struct InterruptDescriptorTableEntry<T> {
    low_ptr: u16,
    gdt_selector: u16,
    options: InterruptDescriptorTableEntryOptions,
    middle_ptr: u16,
    high_ptr: u32,
    reserved: u32,
    phantom: PhantomData<T>,
}

#[repr(transparent)]
struct InterruptDescriptorTableEntryOptions(u16);

#[repr(transparent)]
pub struct InterruptStackFrame {
    value: InterruptStackFrameValue,
}

impl core::fmt::Debug for InterruptStackFrame {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl core::fmt::Debug for InterruptStackFrameValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        struct Hex(u64);

        impl core::fmt::Debug for Hex {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        let mut s = f.debug_struct("InterruptStackFrame");
        s.field("instruction_pointer", &self.instruction_pointer);
        s.field("code_segment", &self.code_segment);
        s.field("cpu_flags", &Hex(self.cpu_flags));
        s.field("stack_pointer", &self.stack_pointer);
        s.field("stack_segment", &self.stack_segment);
        s.finish()
    }
}

#[repr(C)]
struct InterruptStackFrameValue {
    instruction_pointer: VirtualAddress,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: VirtualAddress,
    stack_segment: u64,
}

impl InterruptDescriptorTable {
    pub fn new() -> Self {
        Self {
            divide_error: InterruptDescriptorTableEntry::null(),
            debug: InterruptDescriptorTableEntry::null(),
            non_maskable_interrupt: InterruptDescriptorTableEntry::null(),
            breakpoint: InterruptDescriptorTableEntry::null(),
            overflow: InterruptDescriptorTableEntry::null(),
            bound_range_exceeded: InterruptDescriptorTableEntry::null(),
            invalid_opcode: InterruptDescriptorTableEntry::null(),
            device_not_available: InterruptDescriptorTableEntry::null(),
            double_fault: InterruptDescriptorTableEntry::null(),
            invalid_tss: InterruptDescriptorTableEntry::null(),
            segment_not_present: InterruptDescriptorTableEntry::null(),
            stack_fault: InterruptDescriptorTableEntry::null(),
            general_protection: InterruptDescriptorTableEntry::null(),
            page_fault: InterruptDescriptorTableEntry::null(),
            x87_fpu_floating_point: InterruptDescriptorTableEntry::null(),
            alignment_check: InterruptDescriptorTableEntry::null(),
            machine_check: InterruptDescriptorTableEntry::null(),
            simd_floating_point: InterruptDescriptorTableEntry::null(),
            virtualization: InterruptDescriptorTableEntry::null(),
            control_protection: InterruptDescriptorTableEntry::null(),
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    fn pointer(&self) -> DescriptorTablePointer {
        use core::mem::size_of;

        DescriptorTablePointer {
            base: VirtualAddress::new(self as *const _ as u64),
            limit: (size_of::<Self>() - 1) as u16,
        }
    }

    #[inline]
    pub fn load(&'static self) {
        unsafe {
            asm!(
                "lidt [{}]",
                in(reg) &self.pointer(),
                options(readonly, nostack, preserves_flags)
            );
        }
    }
}

impl<T> InterruptDescriptorTableEntry<T> {
    #[inline]
    pub fn null() -> Self {
        Self {
            low_ptr: 0,
            gdt_selector: 0,
            options: InterruptDescriptorTableEntryOptions::default(),
            middle_ptr: 0,
            high_ptr: 0,
            reserved: 0,
            phantom: PhantomData,
        }
    }
}

impl Default for InterruptDescriptorTableEntryOptions {
    fn default() -> Self {
        Self(0b1110_0000_0000)
    }
}
