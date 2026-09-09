/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than implemented in this header translation.

#[repr(i32)]
pub enum UcodeState {
    UCODE_OK = 0,
    UCODE_NEW,
    UCODE_NEW_SAFE,
    UCODE_UPDATED,
    UCODE_NFOUND,
    UCODE_ERROR,
    UCODE_TIMEOUT,
    UCODE_OFFLINE,
}

#[repr(C)]
pub struct Device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MicrocodeOps {
    pub request_microcode_fw: Option<unsafe extern "C" fn(cpu: i32, dev: *mut Device) -> UcodeState>,
    pub microcode_fini_cpu: Option<unsafe extern "C" fn(cpu: i32)>,

    /*
     * The generic 'microcode_core' part guarantees that the callbacks
     * below run on a target CPU when they are being called.
     * See also the "Synchronization" section in microcode_core.c.
     */
    pub apply_microcode: Option<unsafe extern "C" fn(cpu: i32) -> UcodeState>,
    pub stage_microcode: Option<unsafe extern "C" fn()>,
    pub collect_cpu_info:
        Option<unsafe extern "C" fn(cpu: i32, csig: *mut CpuSignature) -> i32>,
    pub finalize_late_load: Option<unsafe extern "C" fn(result: i32)>,
    // C bit-fields: nmi_safe:1, use_nmi:1, use_staging:1.
    pub flags: u32,
}

#[repr(C)]
pub struct EarlyLoadData {
    pub old_rev: u32,
    pub new_rev: u32,
}

extern "C" {
    pub static mut early_data: EarlyLoadData;
    pub static mut ucode_cpu_info: UcodeCpuInfo;
    pub static mut microcode_rev: [u32; NR_CPUS];
    pub static mut base_rev: u32;

    pub fn find_microcode_in_initrd(path: *const core::ffi::c_char) -> CpioData;
}

pub const MAX_UCODE_COUNT: u32 = 128;

#[inline]
pub const fn qchar(a: u32, b: u32, c: u32, d: u32) -> u32 {
    a.wrapping_add(b << 8).wrapping_add(c << 16).wrapping_add(d << 24)
}

pub const CPUID_INTEL1: u32 = qchar(b'G' as u32, b'e' as u32, b'n' as u32, b'u' as u32);
pub const CPUID_INTEL2: u32 = qchar(b'i' as u32, b'n' as u32, b'e' as u32, b'I' as u32);
pub const CPUID_INTEL3: u32 = qchar(b'n' as u32, b't' as u32, b'e' as u32, b'l' as u32);
pub const CPUID_AMD1: u32 = qchar(b'A' as u32, b'u' as u32, b't' as u32, b'h' as u32);
pub const CPUID_AMD2: u32 = qchar(b'e' as u32, b'n' as u32, b't' as u32, b'i' as u32);
pub const CPUID_AMD3: u32 = qchar(b'c' as u32, b'A' as u32, b'M' as u32, b'D' as u32);

#[inline]
pub const fn cpuid_is(a: u32, b: u32, c: u32, ebx: u32, ecx: u32, edx: u32) -> bool {
    ((ebx ^ a) | (edx ^ b) | (ecx ^ c)) == 0
}

#[inline]
pub unsafe fn x86_cpuid_vendor() -> i32 {
    let mut eax: u32 = 0;
    let mut ebx: u32;
    let mut ecx: u32 = 0;
    let mut edx: u32;
    native_cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx);

    if cpuid_is(CPUID_INTEL1, CPUID_INTEL2, CPUID_INTEL3, ebx, ecx, edx) {
        return X86_VENDOR_INTEL;
    }
    if cpuid_is(CPUID_AMD1, CPUID_AMD2, CPUID_AMD3, ebx, ecx, edx) {
        return X86_VENDOR_AMD;
    }
    X86_VENDOR_UNKNOWN
}

#[inline]
pub unsafe fn x86_cpuid_family() -> u32 {
    let mut eax: u32 = 1;
    let mut ebx: u32;
    let mut ecx: u32 = 0;
    let mut edx: u32;
    native_cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx);
    x86_family(eax)
}

extern "C" {
    pub static mut force_minrev: bool;

    #[cfg(feature = "CONFIG_CPU_SUP_AMD")]
    pub fn load_ucode_amd_bsp(ed: *mut EarlyLoadData, family: u32);
    #[cfg(feature = "CONFIG_CPU_SUP_AMD")]
    pub fn load_ucode_amd_ap(family: u32);
    #[cfg(feature = "CONFIG_CPU_SUP_AMD")]
    pub fn reload_ucode_amd(cpu: u32);
    #[cfg(feature = "CONFIG_CPU_SUP_AMD")]
    pub fn init_amd_microcode() -> *mut MicrocodeOps;
    #[cfg(feature = "CONFIG_CPU_SUP_AMD")]
    pub fn exit_amd_microcode();

    #[cfg(feature = "CONFIG_CPU_SUP_INTEL")]
    pub fn load_ucode_intel_bsp(ed: *mut EarlyLoadData);
    #[cfg(feature = "CONFIG_CPU_SUP_INTEL")]
    pub fn load_ucode_intel_ap();
    #[cfg(feature = "CONFIG_CPU_SUP_INTEL")]
    pub fn reload_ucode_intel();
    #[cfg(feature = "CONFIG_CPU_SUP_INTEL")]
    pub fn init_intel_microcode() -> *mut MicrocodeOps;
}

// CONFIG_MICROCODE_DBG controls the original ucode_dbg variadic logging macro.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
