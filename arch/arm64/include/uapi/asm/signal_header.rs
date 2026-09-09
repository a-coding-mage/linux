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

/* Required for AArch32 compatibility. */
pub const SA_RESTORER: u32 = 0x04000000;

pub const MINSIGSTKSZ: u32 = 5120;
pub const SIGSTKSZ: u32 = 16384;

/* Declarations from <asm-generic/signal.h> are supplied by the surrounding translation. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
