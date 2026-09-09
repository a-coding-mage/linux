// SPDX-License-Identifier: GPL-2.0-only
//
// Dependencies supplied by the corresponding Linux headers are intentionally
// left external to this translation.

#[repr(C)]
pub struct leaf_0x1_0 {
    pub base_family_id: u32,
    pub ext_family: u32,
    pub base_model: u32,
    pub ext_model: u32,
}

fn __x86_family(mut base_fam: u32, ext_fam: u32) -> u32 {
    if base_fam == 0xf {
        base_fam = base_fam.wrapping_add(ext_fam);
    }

    base_fam
}

fn __x86_model(family: u32, mut base_model: u32, ext_model: u32) -> u32 {
    if family >= 0x6 {
        base_model |= ext_model << 4;
    }

    base_model
}

pub fn x86_family(sig: u32) -> u32 {
    __x86_family((sig >> 8) & 0xf, (sig >> 20) & 0xff)
}

// EXPORT_SYMBOL_GPL(x86_family)

pub fn x86_model(sig: u32) -> u32 {
    __x86_model(x86_family(sig), (sig >> 4) & 0xf, (sig >> 16) & 0xf)
}

// EXPORT_SYMBOL_GPL(x86_model)

pub fn x86_stepping(sig: u32) -> u32 {
    sig & 0xf
}

// EXPORT_SYMBOL_GPL(x86_stepping)

pub unsafe fn cpuid_family(l: *const leaf_0x1_0) -> u32 {
    __x86_family((*l).base_family_id, (*l).ext_family)
}

pub unsafe fn cpuid_model(l: *const leaf_0x1_0) -> u32 {
    __x86_model(cpuid_family(l), (*l).base_model, (*l).ext_model)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
