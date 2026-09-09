/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* atmapi.h - ATM API user space/kernel compatibility */

/* Written 1999,2000 by Werner Almesberger, EPFL ICA */

/*
 * Such alignment is not required on 32 bit sparcs, but we can't
 * figure that we are on a sparc64 while compiling user-space programs.
 *
 * The C header applies aligned(8) on sparc and ia64; preserve that
 * target-dependent layout here.
 */
#[cfg_attr(any(target_arch = "sparc", target_arch = "ia64"), repr(align(8)))]
#[repr(C)]
pub struct atm_kptr_t {
    /*
     * Opaque type for kernel pointers. Note that _ is never accessed. We need
     * the struct in order hide the array, so that we can make simple
     * assignments instead of being forced to use memcpy. It also improves
     * error reporting for code that still assumes that we're passing
     * unsigned longs.
     *
     * Convention: NULL pointers are passed as a field of all zeroes.
     */
    pub _:
        [u8; 8],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
