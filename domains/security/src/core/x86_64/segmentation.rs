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

use super::privileges::PLevel;
use bit_field::BitField;

pub enum Descriptor {
    UserSegment(u64),
    SystemSegment(u64, u64),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SegmentSelector(pub u16);

pub struct CodeSegment;

pub struct StackSegment;

pub struct DataSegment;

pub struct FSegment;

pub struct GSegment;

impl SegmentSelector {
    pub const NULL: Self = Self::new(0, PLevel::Ring0);

    #[inline]
    pub const fn new(index: u16, rpl: PLevel) -> Self {
        Self(index << 3 | (rpl as u16))
    }

    #[inline]
    pub fn index(self) -> u16 {
        self.0 >> 3
    }

    // Requested Privilege Level ()
    #[inline]
    pub fn rpl(self) -> PLevel {
        PLevel::from_u16(self.0.get_bits(0..2))
    }

    /// Set the privilege level for this Segment selector.
    #[inline]
    pub fn set_rpl(&mut self, rpl: PLevel) {
        self.0.set_bits(0..2, rpl as u16);
    }
}
