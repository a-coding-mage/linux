/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <linux/purgatory.h> when not assembling. Its
// declarations are supplied by the corresponding Rust translation.

// C equivalent: #ifndef __ASSEMBLER__
unsafe extern "C" {
    pub fn purgatory();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
