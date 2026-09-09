/* SPDX-License-Identifier: GPL-2.0 */

// Translated from asm/cpuid/api.h.  Types and symbols supplied by the
// included kernel headers are intentionally left as external dependencies.

#[cfg(CONFIG_X86_32)]
extern "C" {
    pub fn cpuid_feature() -> bool;
}

#[cfg(not(CONFIG_X86_32))]
#[inline]
pub fn cpuid_feature() -> bool { true }

#[inline]
pub unsafe fn native_cpuid(eax: *mut u32, ebx: *mut u32, ecx: *mut u32, edx: *mut u32) {
    // The C implementation uses volatile inline `cpuid`, with EAX and ECX
    // as both inputs and outputs and a memory clobber.
    core::arch::asm!(
        "cpuid",
        inout("eax") *eax,
        lateout("ebx") *ebx,
        inout("ecx") *ecx,
        lateout("edx") *edx,
        options(nostack, preserves_flags),
    );
}

#[inline]
pub unsafe fn native_cpuid_eax(op: u32) -> u32 {
    let mut eax = op; let mut ebx; let mut ecx = 0; let mut edx;
    native_cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx); eax
}
#[inline]
pub unsafe fn native_cpuid_ebx(op: u32) -> u32 {
    let mut eax = op; let mut ebx; let mut ecx = 0; let mut edx;
    native_cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx); ebx
}
#[inline]
pub unsafe fn native_cpuid_ecx(op: u32) -> u32 {
    let mut eax = op; let mut ebx; let mut ecx = 0; let mut edx;
    native_cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx); ecx
}
#[inline]
pub unsafe fn native_cpuid_edx(op: u32) -> u32 {
    let mut eax = op; let mut ebx; let mut ecx = 0; let mut edx;
    native_cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx); edx
}

#[cfg(CONFIG_PARAVIRT_XXL)]
extern "C" {
    pub fn __cpuid(eax: *mut u32, ebx: *mut u32, ecx: *mut u32, edx: *mut u32);
}
#[cfg(not(CONFIG_PARAVIRT_XXL))]
#[inline]
pub unsafe fn __cpuid(eax: *mut u32, ebx: *mut u32, ecx: *mut u32, edx: *mut u32) {
    native_cpuid(eax, ebx, ecx, edx)
}

#[inline]
pub unsafe fn cpuid(op: u32, eax: *mut u32, ebx: *mut u32, ecx: *mut u32, edx: *mut u32) {
    *eax = op; *ecx = 0; __cpuid(eax, ebx, ecx, edx);
}

#[inline]
pub unsafe fn cpuid_count(op: u32, count: i32, eax: *mut u32, ebx: *mut u32, ecx: *mut u32, edx: *mut u32) {
    *eax = op; *ecx = count as u32; __cpuid(eax, ebx, ecx, edx);
}

#[inline] pub unsafe fn cpuid_eax(op: u32) -> u32 { let (mut a, mut b, mut c, mut d)=(0,0,0,0); cpuid(op,&mut a,&mut b,&mut c,&mut d); a }
#[inline] pub unsafe fn cpuid_ebx(op: u32) -> u32 { let (mut a, mut b, mut c, mut d)=(0,0,0,0); cpuid(op,&mut a,&mut b,&mut c,&mut d); b }
#[inline] pub unsafe fn cpuid_ecx(op: u32) -> u32 { let (mut a, mut b, mut c, mut d)=(0,0,0,0); cpuid(op,&mut a,&mut b,&mut c,&mut d); c }
#[inline] pub unsafe fn cpuid_edx(op: u32) -> u32 { let (mut a, mut b, mut c, mut d)=(0,0,0,0); cpuid(op,&mut a,&mut b,&mut c,&mut d); d }

#[inline]
pub unsafe fn __cpuid_read(leaf: u32, subleaf: u32, regs: *mut u32) {
    *regs.add(CPUID_EAX as usize) = leaf;
    *regs.add(CPUID_ECX as usize) = subleaf;
    __cpuid(regs.add(CPUID_EAX as usize), regs.add(CPUID_EBX as usize), regs.add(CPUID_ECX as usize), regs.add(CPUID_EDX as usize));
}

// CPUID register indices are supplied by asm/cpuid/types.h.
extern "C" {
    static cpuid_0x2_table: *const leaf_0x2_table;
}

#[inline]
pub fn cpuid_function_is_indexed(function: u32) -> bool {
    matches!(function, 4 | 7 | 0xb | 0xd | 0xf | 0x10 | 0x12 | 0x14 | 0x17 | 0x18 | 0x1d | 0x1e | 0x1f | 0x24 | 0x8000001d)
}

#[inline]
pub unsafe fn cpuid_base_hypervisor(sig: *const core::ffi::c_char, leaves: u32) -> u32 {
    let mut base = 0x40000000u32;
    while base < 0x40010000 {
        let (mut eax, mut s0, mut s1, mut s2) = (0, 0, 0, 0);
        cpuid(base, &mut eax, &mut s0, &mut s1, &mut s2);
        if libc_memcmp(sig as *const _, (&s0 as *const u32).cast(), 12) == 0 &&
           (leaves == 0 || eax.wrapping_sub(base) >= leaves) { return base; }
        base += 0x100;
    }
    0
}

extern "C" { fn libc_memcmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void, n: usize) -> i32; }

// The remaining C macros are retained as Rust macro interfaces; their
// token-pasted leaf types and parser structures come from the included headers.
#[macro_export] macro_rules! cpuid_read_subleaf { ($leaf:expr, $subleaf:expr, $regs:expr) => {{ unsafe { $crate::__cpuid_read($leaf, $subleaf, ($regs as *mut u32)); } }} }
#[macro_export] macro_rules! cpuid_read { ($leaf:expr, $regs:expr) => {{ $crate::cpuid_read_subleaf!($leaf, 0, $regs) }} }
#[macro_export] macro_rules! for_each_possible_cpuid_base_hypervisor { ($function:ident) => { for $function in (0x40000000u32..0x40010000u32).step_by(0x100) } }
#[macro_export] macro_rules! cpuid_subleaf { ($cpuinfo:expr, $leaf:tt, $subleaf:tt) => { $crate::__cpuid_table_subleaf!($cpuinfo, $leaf, $subleaf) } }
#[macro_export] macro_rules! cpuid_leaf { ($cpuinfo:expr, $leaf:tt) => { $crate::cpuid_subleaf!($cpuinfo, $leaf, 0) } }
#[macro_export] macro_rules! cpuid_leaf_raw { ($cpuinfo:expr, $leaf:tt) => { $crate::cpuid_leaf!($cpuinfo, $leaf) as *const cpuid_regs } }
#[macro_export] macro_rules! cpuid_subleaf_n { ($cpuinfo:expr, $leaf:tt, $subleaf:expr) => { $crate::__cpuid_table_subleaf_n!($cpuinfo, $leaf, $subleaf) } }
#[macro_export] macro_rules! cpuid_subleaf_n_raw { ($cpuinfo:expr, $leaf:tt, $subleaf:expr) => { $crate::cpuid_subleaf_n!($cpuinfo, $leaf, $subleaf) as *const cpuid_regs } }
#[macro_export] macro_rules! cpuid_subleaf_count { ($cpuinfo:expr, $leaf:tt) => { $crate::__cpuid_table_nr_filled_subleaves!($cpuinfo, $leaf, n) } }

// External parser API declarations.
extern "C" {
    pub fn cpuid_scan_cpu(c: *mut cpuinfo_x86);
    pub fn cpuid_refresh_leaf(c: *mut cpuinfo_x86, leaf: u32);
    pub fn cpuid_refresh_range(c: *mut cpuinfo_x86, start: u32, end: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
