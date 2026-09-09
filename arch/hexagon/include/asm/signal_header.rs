/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 and
 * only version 2 as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301, USA.
 */

// C header guard: _ASM_SIGNAL_H

// Supplied by the architecture's register definitions.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub static mut __rt_sigtramp_template: [usize; 2];

    pub fn do_signal(regs: *mut pt_regs);
}

// Dependency from <asm-generic/signal.h>; declarations are supplied elsewhere.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
