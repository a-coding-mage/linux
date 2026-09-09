/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding architecture and topology modules
// are intentionally left external to this translation.

#[repr(C)]
pub struct cpu_dev {
    pub c_vendor: *const ::core::ffi::c_char,

    /* some have two possibilities for cpuid string */
    pub c_ident: [*const ::core::ffi::c_char; 2],

    pub c_early_init: Option<unsafe extern "C" fn(*mut cpuinfo_x86)>,
    pub c_bsp_init: Option<unsafe extern "C" fn(*mut cpuinfo_x86)>,
    pub c_init: Option<unsafe extern "C" fn(*mut cpuinfo_x86)>,
    pub c_identify: Option<unsafe extern "C" fn(*mut cpuinfo_x86)>,
    pub c_detect_tlb: Option<unsafe extern "C" fn(*mut cpuinfo_x86)>,
    pub c_x86_vendor: ::core::ffi::c_int,

    // CONFIG_X86_32
    // Optional vendor specific routine to obtain the cache size.
    #[cfg(CONFIG_X86_32)]
    pub legacy_cache_size:
        Option<unsafe extern "C" fn(*mut cpuinfo_x86, ::core::ffi::c_uint) -> ::core::ffi::c_uint>,

    // Family/stepping-based lookup table for model names.
    #[cfg(CONFIG_X86_32)]
    pub legacy_models: [legacy_cpu_model_info; 5],
}

#[cfg(CONFIG_X86_32)]
#[repr(C)]
pub struct legacy_cpu_model_info {
    pub family: ::core::ffi::c_int,
    pub model_names: [*const ::core::ffi::c_char; 16],
}

// #define cpu_dev_register(cpu_devX) ...
// Registration is emitted by the platform/linker integration using the
// .x86_cpu_dev.init section.

unsafe extern "C" {
    pub static __x86_cpu_dev_start: *const *const cpu_dev;
    pub static __x86_cpu_dev_end: *const *const cpu_dev;

    pub fn init_spectral_chicken(c: *mut cpuinfo_x86);
    pub fn get_cpu_cap(c: *mut cpuinfo_x86);
    pub fn get_cpu_address_sizes(c: *mut cpuinfo_x86);
    pub fn cpu_detect_cache_sizes(c: *mut cpuinfo_x86);
    pub fn init_scattered_cpuid_features(c: *mut cpuinfo_x86);
    pub fn init_intel_cacheinfo(c: *mut cpuinfo_x86);
    pub fn init_amd_cacheinfo(c: *mut cpuinfo_x86);
    pub fn init_hygon_cacheinfo(c: *mut cpuinfo_x86);
    pub fn check_null_seg_clears_base(c: *mut cpuinfo_x86);

    pub fn cacheinfo_amd_init_llc_id(c: *mut cpuinfo_x86, die_id: u16);
    pub fn cacheinfo_hygon_init_llc_id(c: *mut cpuinfo_x86);

    pub fn cpu_select_mitigations();
    pub fn x86_spec_ctrl_setup_ap();
    pub fn update_srbds_msr();
    pub fn update_gds_msr();

    pub static mut spectre_v2_enabled: spectre_v2_mitigation;
}

// CONFIG_CPU_SUP_INTEL
#[cfg(CONFIG_CPU_SUP_INTEL)]
unsafe extern "C" {
    pub fn tsx_init();
    pub fn tsx_ap_init();
    pub fn intel_unlock_cpuid_leafs(c: *mut cpuinfo_x86);
}

#[cfg(not(CONFIG_CPU_SUP_INTEL))]
#[inline]
pub unsafe fn tsx_init() {}

#[cfg(not(CONFIG_CPU_SUP_INTEL))]
#[inline]
pub unsafe fn tsx_ap_init() {}

#[cfg(not(CONFIG_CPU_SUP_INTEL))]
#[inline]
pub unsafe fn intel_unlock_cpuid_leafs(_c: *mut cpuinfo_x86) {}

// CONFIG_AMD_NB && CONFIG_SYSFS
#[cfg(all(CONFIG_AMD_NB, CONFIG_SYSFS))]
unsafe extern "C" {
    pub fn amd_init_l3_cache(index: ::core::ffi::c_int) -> *mut amd_northbridge;
}

#[cfg(not(all(CONFIG_AMD_NB, CONFIG_SYSFS)))]
#[inline]
pub unsafe fn amd_init_l3_cache(_index: ::core::ffi::c_int) -> *mut amd_northbridge {
    ::core::ptr::null_mut()
}

#[inline]
pub fn spectre_v2_in_eibrs_mode(mode: spectre_v2_mitigation) -> bool {
    mode == SPECTRE_V2_EIBRS
        || mode == SPECTRE_V2_EIBRS_RETPOLINE
        || mode == SPECTRE_V2_EIBRS_LFENCE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
