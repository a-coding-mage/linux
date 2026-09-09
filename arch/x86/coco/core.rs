// SPDX-License-Identifier: GPL-2.0-only
/*
 * Confidential Computing Platform Capability checks
 *
 * Copyright (C) 2021 Advanced Micro Devices, Inc.
 * Copyright (C) 2024 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 *
 * Author: Tom Lendacky <thomas.lendacky@amd.com>
 */

// C dependencies supplied by other translation units are intentionally left as external Rust symbols.

extern "C" {
    static mut sev_status: u64;
    static mut sme_me_mask: u64;
    fn arch_get_random_longs(buf: *mut c_ulong, num: usize) -> usize;
    fn add_device_randomness(buf: *const c_void, len: usize);
    fn memzero_explicit(buf: *mut c_void, len: usize);
}

#[allow(non_camel_case_types)]
type c_ulong = usize;
#[allow(non_camel_case_types)]
type c_void = core::ffi::c_void;

// These constants and the enum are provided by the corresponding kernel headers.
// CONFIG_AMD_MEM_ENCRYPT controls the conditional implementation below.

#[no_mangle]
pub static mut cc_vendor: cc_vendor = CC_VENDOR_NONE;
#[no_mangle]
pub static mut cc_mask: u64 = 0;

#[repr(C)]
struct cc_attr_flags {
    host_sev_snp: u64,
}

static mut cc_flags: cc_attr_flags = cc_attr_flags { host_sev_snp: 0 };

unsafe fn intel_cc_platform_has(attr: cc_attr) -> bool {
    match attr {
        CC_ATTR_GUEST_UNROLL_STRING_IO
        | CC_ATTR_GUEST_MEM_ENCRYPT
        | CC_ATTR_MEM_ENCRYPT => true,
        _ => false,
    }
}

#[inline(always)]
unsafe fn amd_cc_platform_vtom(attr: cc_attr) -> bool {
    match attr {
        CC_ATTR_GUEST_MEM_ENCRYPT | CC_ATTR_MEM_ENCRYPT => true,
        _ => false,
    }
}

unsafe fn amd_cc_platform_has(attr: cc_attr) -> bool {
    // The body is guarded by CONFIG_AMD_MEM_ENCRYPT in the C source.
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    {
        if sev_status & MSR_AMD64_SNP_VTOM != 0 {
            return amd_cc_platform_vtom(attr);
        }

        return match attr {
            CC_ATTR_MEM_ENCRYPT => sme_me_mask != 0,
            CC_ATTR_HOST_MEM_ENCRYPT => {
                sme_me_mask != 0 && (sev_status & MSR_AMD64_SEV_ENABLED) == 0
            }
            CC_ATTR_GUEST_MEM_ENCRYPT => sev_status & MSR_AMD64_SEV_ENABLED != 0,
            CC_ATTR_GUEST_STATE_ENCRYPT => sev_status & MSR_AMD64_SEV_ES_ENABLED != 0,
            CC_ATTR_GUEST_UNROLL_STRING_IO => {
                (sev_status & MSR_AMD64_SEV_ENABLED != 0)
                    && (sev_status & MSR_AMD64_SEV_ES_ENABLED == 0)
            }
            CC_ATTR_GUEST_SEV_SNP => sev_status & MSR_AMD64_SEV_SNP_ENABLED != 0,
            CC_ATTR_GUEST_SNP_SECURE_TSC => sev_status & MSR_AMD64_SNP_SECURE_TSC != 0,
            CC_ATTR_HOST_SEV_SNP => cc_flags.host_sev_snp != 0,
            CC_ATTR_SNP_SECURE_AVIC => sev_status & MSR_AMD64_SNP_SECURE_AVIC != 0,
            _ => false,
        };
    }
    #[cfg(not(CONFIG_AMD_MEM_ENCRYPT))]
    {
        let _ = attr;
        false
    }
}

#[no_mangle]
pub unsafe fn cc_platform_has(attr: cc_attr) -> bool {
    match cc_vendor {
        CC_VENDOR_AMD => amd_cc_platform_has(attr),
        CC_VENDOR_INTEL => intel_cc_platform_has(attr),
        _ => false,
    }
}

#[no_mangle]
pub unsafe fn cc_mkenc(val: u64) -> u64 {
    match cc_vendor {
        CC_VENDOR_AMD => {
            if sev_status & MSR_AMD64_SNP_VTOM != 0 {
                val & !cc_mask
            } else {
                val | cc_mask
            }
        }
        CC_VENDOR_INTEL => val & !cc_mask,
        _ => val,
    }
}

#[no_mangle]
pub unsafe fn cc_mkdec(val: u64) -> u64 {
    match cc_vendor {
        CC_VENDOR_AMD => {
            if sev_status & MSR_AMD64_SNP_VTOM != 0 {
                val | cc_mask
            } else {
                val & !cc_mask
            }
        }
        CC_VENDOR_INTEL => val | cc_mask,
        _ => val,
    }
}

unsafe fn amd_cc_platform_clear(attr: cc_attr) {
    match attr {
        CC_ATTR_HOST_SEV_SNP => cc_flags.host_sev_snp = 0,
        _ => {}
    }
}

#[no_mangle]
pub unsafe fn cc_platform_clear(attr: cc_attr) {
    match cc_vendor {
        CC_VENDOR_AMD => amd_cc_platform_clear(attr),
        _ => {}
    }
}

unsafe fn amd_cc_platform_set(attr: cc_attr) {
    match attr {
        CC_ATTR_HOST_SEV_SNP => cc_flags.host_sev_snp = 1,
        _ => {}
    }
}

#[no_mangle]
pub unsafe fn cc_platform_set(attr: cc_attr) {
    match cc_vendor {
        CC_VENDOR_AMD => amd_cc_platform_set(attr),
        _ => {}
    }
}

#[no_mangle]
pub unsafe fn cc_random_init() {
    // The seed is 32 bytes (in units of longs), which is 256 bits, which
    // is the security level that the RNG is targeting.
    let mut rng_seed: [c_ulong; 32 / core::mem::size_of::<c_ulong>()] = [0; 32 / core::mem::size_of::<c_ulong>()];
    let mut i: usize = 0;
    let mut longs: usize;

    if !cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) {
        return;
    }

    // Since the CoCo threat model includes the host, the only reliable
    // source of entropy that can be neither observed nor manipulated is
    // RDRAND. Usually, RDRAND failure is considered tolerable, but since
    // CoCo guests have no other unobservable source of entropy, it's
    // important to at least ensure the RNG gets some initial random seeds.
    while i < rng_seed.len() {
        longs = arch_get_random_longs(rng_seed.as_mut_ptr().add(i), rng_seed.len() - i);

        // A zero return value means that the guest doesn't have RDRAND
        // or the CPU is physically broken, and in both cases that
        // means most crypto inside of the CoCo instance will be
        // broken, defeating the purpose of CoCo in the first place. So
        // just panic here because it's absolutely unsafe to continue
        // executing.
        if longs == 0 {
            panic!("RDRAND is defective.");
        }
        i += longs;
    }
    add_device_randomness(rng_seed.as_ptr() as *const c_void, core::mem::size_of_val(&rng_seed));
    memzero_explicit(rng_seed.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&rng_seed));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
