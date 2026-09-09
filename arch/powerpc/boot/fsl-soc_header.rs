/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: `types.h` supplies the C integer type aliases used by this header.

extern "C" {
    pub fn fsl_get_immr() -> *mut u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
