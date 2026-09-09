// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2022
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(u32)]
enum CpuFeatureType {
    TypeHwcap,
    TypeFacility,
    TypeMachine,
}

#[repr(C)]
struct S390CpuFeature {
    // C bit-fields: type occupies bits 0..4 and num occupies bits 4..32.
    bits: u32,
}

const TYPE_HWCAP: u32 = 0;
const TYPE_FACILITY: u32 = 1;
const TYPE_MACHINE: u32 = 2;

// These constants and symbols are provided by the corresponding kernel headers.
extern "C" {
    static elf_hwcap: usize;
    fn test_facility(num: u32) -> i32;
    fn test_machine_feature(num: u32) -> i32;
}

extern "Rust" {
    fn WARN_ON_ONCE(condition: bool) -> bool;
}

const fn cpu_feature(ty: u32, num: u32) -> S390CpuFeature {
    S390CpuFeature { bits: (ty & 0x0f) | ((num & 0x0fff_ffff) << 4) }
}

static mut s390_cpu_features: [S390CpuFeature; MAX_CPU_FEATURES] = [
    cpu_feature(TYPE_HWCAP, HWCAP_NR_MSA),
    cpu_feature(TYPE_HWCAP, HWCAP_NR_VXRS),
    cpu_feature(TYPE_FACILITY, 158),
    cpu_feature(TYPE_MACHINE, MFEATURE_DIAG288),
];

/*
 * cpu_have_feature - Test CPU features on module initialization
 */
#[no_mangle]
pub unsafe extern "C" fn cpu_have_feature(num: u32) -> i32 {
    if WARN_ON_ONCE(num >= MAX_CPU_FEATURES as u32) {
        return 0;
    }
    let feature = &s390_cpu_features[num as usize];
    let feature_type = feature.bits & 0x0f;
    let feature_num = feature.bits >> 4;
    match feature_type {
        TYPE_HWCAP => {
            ((*elf_hwcap & (1usize << feature_num)) != 0) as i32
        }
        TYPE_FACILITY => test_facility(feature_num),
        TYPE_MACHINE => test_machine_feature(feature_num),
        _ => {
            WARN_ON_ONCE(true);
            0
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
