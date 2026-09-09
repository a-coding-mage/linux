/* SPDX-License-Identifier: GPL-2.0-or-later */

// The H_HOME_NODE_ASSOCIATIVITY h_call returns 6 64-bit registers.
pub const VPHN_REGISTER_COUNT: usize = 6;

/*
 * 6 64-bit registers unpacked into up to 24 be32 associativity values. To
 * form the complete property we have to add the length in the first cell.
 */
pub const VPHN_ASSOC_BUFSIZE: usize =
    VPHN_REGISTER_COUNT * core::mem::size_of::<u64>() / core::mem::size_of::<u16>() + 1;

/*
 * The H_HOME_NODE_ASSOCIATIVITY hcall takes two values for flags:
 * 1 for retrieving associativity information for a guest cpu
 * 2 for retrieving associativity information for a host/hypervisor cpu
 */
pub const VPHN_FLAG_VCPU: u64 = 1;
pub const VPHN_FLAG_PCPU: u64 = 2;

unsafe extern "C" {
    pub fn hcall_vphn(
        cpu: ::core::ffi::c_ulong,
        flags: u64,
        associativity: *mut __be32,
    ) -> ::core::ffi::c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
