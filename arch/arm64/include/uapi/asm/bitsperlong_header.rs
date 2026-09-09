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

// When building the kernel for a non-AArch64 target, this is 32 bits
// (used by the compat vDSO); otherwise it is 64 bits.
#[cfg(all(feature = "__KERNEL__", not(target_arch = "aarch64")))]
pub const __BITS_PER_LONG: usize = 32;

#[cfg(not(all(feature = "__KERNEL__", not(target_arch = "aarch64"))))]
pub const __BITS_PER_LONG: usize = 64;

// Dependency corresponding to: #include <asm-generic/bitsperlong.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
