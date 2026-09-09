/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Ptrace definitions for the Hexagon architecture
 *
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

// Dependency supplied by the corresponding registers definitions.

macro_rules! instruction_pointer {
    ($regs:expr) => {
        pt_elr($regs)
    };
}

macro_rules! user_stack_pointer {
    ($regs:expr) => {
        unsafe { (*$regs).r29 }
    };
}

macro_rules! profile_pc {
    ($regs:expr) => {
        instruction_pointer!($regs)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
