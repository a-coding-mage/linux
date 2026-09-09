// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/device-id/of.h> and <linux/module.h>

use core::ffi::c_char;

// Supplied by the Linux device-id compatibility layer.
use crate::of_device_id;

/* This is a dummy device table linked into all of the crypto
 * opcode drivers.  It serves to trigger the module autoloading
 * mechanisms in userspace which scan the OF device tree and
 * load any modules which have device table entries that
 * match OF device nodes.
 */
static CRYPTO_OPCODE_MATCH: [of_device_id; 2] = [
    of_device_id {
        name: b"cpu\0".as_ptr() as *const c_char,
        compatible: b"sun4v\0".as_ptr() as *const c_char,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

// Equivalent of MODULE_DEVICE_TABLE(of, crypto_opcode_match): the
// module metadata/export is provided by the module build integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
