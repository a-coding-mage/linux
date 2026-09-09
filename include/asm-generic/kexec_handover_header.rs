/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the translated Linux types.

#[repr(C)]
pub struct kho_scratch {
    pub addr: phys_addr_t,
    pub size: phys_addr_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
