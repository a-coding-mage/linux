/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent from C: #include <linux/kernel.h> */

#[repr(C)]
pub struct cpumask {
    pub bits: [::std::os::raw::c_ulong; 1],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
