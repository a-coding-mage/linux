/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2018 David Abdurachmanov <david.abdurachmanov@gmail.com>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 as published by
 * the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

// C dependency: <asm/bitsperlong.h>

// When the target word size is 64 bits, this header selects <asm/unistd_64.h>.
// Otherwise, it selects <asm/unistd_32.h>.  Those architecture-specific
// declarations are supplied by their corresponding Rust translation units.
// Dependency selected by the original C preprocessor: <asm/unistd_64.h>

// Dependency selected by the original C preprocessor: <asm/unistd_32.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
