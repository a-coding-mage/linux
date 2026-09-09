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

// C dependency: <asm-generic/statfs.h> supplies the statfs declarations.

// C macro: ARCH_PACK_COMPAT_STATFS64 expands to __attribute__((packed, aligned(4))).
// Apply #[repr(C, packed(4))] to the corresponding translated structure.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
