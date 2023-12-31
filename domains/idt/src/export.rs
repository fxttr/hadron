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

use crate::internal::{
    IDTEntry, IDTHandlerFunction, IDTHandlerFunctionWithErrCode, PageFaultIDTHandlerFunction,
};

// Table to handle interrupts and exceptions. Affects x86 and x86_64.
// See: https://www.logix.cz/michal/doc/i386/chp09-00.htm
#[repr(C)]
pub struct InterruptDescriptorTable {
    // A device error (#DE) occurs when dividing by 0 or when the calculation is too large.
    pub divide_error: IDTEntry<IDTHandlerFunction>,

    pub debug: IDTEntry<IDTHandlerFunction>,
    pub non_maskable_interrupt: IDTEntry<IDTHandlerFunction>,
    pub breakpoint: IDTEntry<IDTHandlerFunction>,
    pub overflow: IDTEntry<IDTHandlerFunction>,
    pub bound_range_exceeded: IDTEntry<IDTHandlerFunction>,
    pub invalid_opcode: IDTEntry<IDTHandlerFunction>,
    pub device_not_available: IDTEntry<IDTHandlerFunction>,
    pub double_fault: IDTEntry<IDTHandlerFunctionWithErrCode>,
    pub invalid_tss: IDTEntry<IDTHandlerFunctionWithErrCode>,
    pub segment_not_present: IDTEntry<IDTHandlerFunctionWithErrCode>,
    pub stack_segment_fault: IDTEntry<IDTHandlerFunctionWithErrCode>,
    pub general_protection_fault: IDTEntry<IDTHandlerFunctionWithErrCode>,
    pub page_fault: IDTEntry<PageFaultIDTHandlerFunction>,
    pub x87_floating_point: IDTEntry<IDTHandlerFunction>,
    pub alignment_check: IDTEntry<IDTHandlerFunctionWithErrCode>,
    pub machine_check: IDTEntry<IDTHandlerFunction>,
    pub simd_floating_point: IDTEntry<IDTHandlerFunction>,
    pub virtualization: IDTEntry<IDTHandlerFunction>,
    pub security_exception: IDTEntry<IDTHandlerFunctionWithErrCode>,
    // some fields omitted
}
