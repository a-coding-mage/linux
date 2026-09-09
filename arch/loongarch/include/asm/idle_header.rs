/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: linux/linkage.h (asmlinkage).

pub unsafe extern "C" {
    pub fn __arch_cpu_idle();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
