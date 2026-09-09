// SPDX-License-Identifier: GPL-2.0
/*
 * FreeBSD Bhyve guest enlightenments
 *
 * Copyright © 2025 Amazon.com, Inc. or its affiliates.
 *
 * Author: David Woodhouse <dwmw2@infradead.org>
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn cpu_feature_enabled(feature: u32) -> bool;
    fn cpuid_base_hypervisor(signature: *const u8, leaves: u32) -> u32;
    fn cpuid_eax(leaf: u32) -> u32;
    fn x86_init_noop();
}

extern "C" {
    static X86_FEATURE_HYPERVISOR: u32;
}

static mut bhyve_cpuid_base: u32 = 0;
static mut bhyve_cpuid_max: u32 = 0;

const BHYVE_SIGNATURE: &[u8] = b"bhyve bhyve \0";

const CPUID_BHYVE_FEATURES: u32 = 0x40000001;

/* Features advertised in CPUID_BHYVE_FEATURES %eax */

/* MSI Extended Dest ID */
const CPUID_BHYVE_FEAT_EXT_DEST_ID: u32 = 1u32 << 0;

unsafe fn bhyve_detect() -> u32
{
    if !cpu_feature_enabled(X86_FEATURE_HYPERVISOR) {
        return 0;
    }

    bhyve_cpuid_base = cpuid_base_hypervisor(BHYVE_SIGNATURE.as_ptr(), 0);
    if bhyve_cpuid_base == 0 {
        return 0;
    }

    bhyve_cpuid_max = cpuid_eax(bhyve_cpuid_base);
    bhyve_cpuid_max
}

unsafe fn bhyve_features() -> u32
{
    let cpuid_leaf: u32 = bhyve_cpuid_base | CPUID_BHYVE_FEATURES;

    if bhyve_cpuid_max < cpuid_leaf {
        return 0;
    }

    cpuid_eax(cpuid_leaf)
}

unsafe fn bhyve_ext_dest_id() -> bool
{
    (bhyve_features() & CPUID_BHYVE_FEAT_EXT_DEST_ID) != 0
}

unsafe fn bhyve_x2apic_available() -> bool
{
    true
}

pub static x86_hyper_bhyve: hypervisor_x86 = hypervisor_x86 {
    name: b"Bhyve\0".as_ptr() as *const i8,
    detect: bhyve_detect,
    init: hypervisor_x86_init {
        init_platform: x86_init_noop,
        x2apic_available: bhyve_x2apic_available,
        msi_ext_dest_id: bhyve_ext_dest_id,
    },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
