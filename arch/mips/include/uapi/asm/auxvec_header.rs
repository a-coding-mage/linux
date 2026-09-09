/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright (C) 2015 Imagination Technologies
 * Author: Alex Smith <alex.smith@imgtec.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 */

// C header guard: __ASM_AUXVEC_H

/* Location of VDSO image. */
pub const AT_SYSINFO_EHDR: usize = 33;

pub const AT_VECTOR_SIZE_ARCH: usize = 1; /* entries in ARCH_DLINFO */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
