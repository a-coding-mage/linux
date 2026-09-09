/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2012 ARM Ltd.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

// Dependency supplied by the corresponding Linux types definitions.

#[repr(C)]
pub struct ucontext {
    pub uc_flags: ::core::ffi::c_ulong,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_sigmask: sigset_t,
    /* glibc uses a 1024-bit sigset_t */
    pub __unused: [u8; 1024 / 8 - ::core::mem::size_of::<sigset_t>()],
    /* last for future expansion */
    pub uc_mcontext: sigcontext,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
