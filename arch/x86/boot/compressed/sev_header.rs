/* SPDX-License-Identifier: GPL-2.0 */
/*
 * AMD SEV header for early boot related functions.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

// CONFIG_AMD_MEM_ENCRYPT controls which declarations and implementations are
// available in the original header.

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
use crate::{msr, phys_addr_t};

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
extern "C" {
    pub fn snp_accept_memory(start: phys_addr_t, end: phys_addr_t);
    pub fn sev_get_status() -> u64;
    pub fn early_is_sevsnp_guest() -> bool;
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
#[inline]
pub unsafe fn sev_es_rd_ghcb_msr() -> u64 {
    let mut m: msr = core::mem::zeroed();

    raw_rdmsr(MSR_AMD64_SEV_ES_GHCB, &mut m);

    m.q
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
#[inline]
pub unsafe fn sev_es_wr_ghcb_msr(val: u64) {
    let mut m: msr = core::mem::zeroed();

    m.q = val;
    raw_wrmsr(MSR_AMD64_SEV_ES_GHCB, &mut m);
}

#[cfg(CONFIG_AMD_MEM_ENCRYPT)]
extern "C" {
    fn raw_rdmsr(msr: u32, m: *mut msr);
    fn raw_wrmsr(msr: u32, m: *mut msr);
}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub fn snp_accept_memory(_start: crate::phys_addr_t, _end: crate::phys_addr_t) {}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub fn sev_get_status() -> u64 {
    0
}

#[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
#[inline]
pub fn early_is_sevsnp_guest() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
