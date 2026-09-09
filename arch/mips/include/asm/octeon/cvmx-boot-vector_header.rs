/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003-2017 Cavium, Inc.
 */

// Dependency: <asm/octeon/octeon.h>

/*
 * The boot vector table is made up of an array of 1024 elements of
 * struct cvmx_boot_vector_element.  There is one entry for each
 * possible MIPS CPUNum, indexed by the CPUNum.
 *
 * Once cvmx_boot_vector_get() returns a non-NULL value (indicating
 * success), NMI to a core will cause execution to transfer to the
 * target_ptr location for that core's entry in the vector table.
 *
 * The struct cvmx_boot_vector_element fields app0, app1, and app2 can
 * be used by the application that has set the target_ptr in any
 * application specific manner, they are not touched by the vectoring
 * code.
 *
 * The boot vector code clobbers the CP0_DESAVE register, and on
 * OCTEON II and later CPUs also clobbers CP0_KScratch2.  All GP
 * registers are preserved, except on pre-OCTEON II CPUs, where k1 is
 * clobbered.
 */

/*
 * Applications install the boot bus code in cvmx-boot-vector.c, which
 * uses this magic:
 */
pub const OCTEON_BOOT_MOVEABLE_MAGIC1: u64 = 0xdb00110ad358eacd_u64;

#[repr(C)]
pub struct cvmx_boot_vector_element {
    /* kseg0 or xkphys address of target code. */
    pub target_ptr: u64,
    /* Three application specific arguments. */
    pub app0: u64,
    pub app1: u64,
    pub app2: u64,
}

unsafe extern "C" {
    pub fn cvmx_boot_vector_get() -> *mut cvmx_boot_vector_element;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
