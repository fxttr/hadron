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

use core::marker::PhantomData;

use x86_64::structures::memory::VirtualAddress;
use x86_64::types::paging::PageFaultErrorCode;

pub type IDTHandlerFunction = extern "x86-interrupt" fn(&mut InterruptStackFrame);
pub type IDTHandlerFunctionWithErrCode =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: u64);
pub type PageFaultIDTHandlerFunction =
    extern "x86-interrupt" fn(InterruptStackFrame, error_code: PageFaultErrorCode);

/// An Interrupt Descriptor Table entry.
#[repr(C)]
pub struct IDTEntry<T> {
    pointer_low: u16,
    gdt_selector: u16,
    options: IDTEntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
    phantom: PhantomData<T>,
}

#[repr(transparent)]
struct IDTEntryOptions(u16);

#[repr(transparent)]
pub struct InterruptStackFrame {
    value: InterruptStackFrameValue,
}

#[repr(C)]
struct InterruptStackFrameValue {
    instruction_pointer: VirtualAddress,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: VirtualAddress,
    stack_segment: u64,
}
