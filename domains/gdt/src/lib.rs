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

 pub struct Gdt {
    table: [u64; 10],
    next: usize,
}

impl Gdt {
    pub const fn new() -> Self {
        Self {
            table: [0; 10],
            next: 1, // entry 0 must be NULL
        }
    }
}
