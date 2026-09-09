/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 2018
 *
 * Author(s): Philipp Rudo <prudo@linux.vnet.ibm.com>
 */

// C header guard: _S390_PURGATORY_H_
// The declaration is omitted when compiling as assembler (__ASSEMBLER__).
// Dependency corresponding to: #include <linux/purgatory.h>

unsafe extern "C" {
    pub fn verify_sha256_digest() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
