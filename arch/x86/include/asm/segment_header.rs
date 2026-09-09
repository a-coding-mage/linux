/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: <linux/const.h>, <asm/alternative.h>, <asm/ibt.h>, and
// (on 32-bit/other builds) <asm/cache.h> provide build-time architecture data.

/* Constructor for a conventional segment GDT (or LDT) entry. */
#[macro_export]
macro_rules! GDT_ENTRY {
    ($flags:expr, $base:expr, $limit:expr) => {
        ((($base as u64 & 0xff000000u64) << (56 - 24)) |
         (($flags as u64 & 0x0000f0ffu64) << 40) |
         (($limit as u64 & 0x000f0000u64) << (48 - 16)) |
         (($base as u64 & 0x00ffffffu64) << 16) |
         ($limit as u64 & 0x0000ffffu64))
    };
}

pub const GDT_ENTRY_BOOT_CS: usize = 2;
pub const GDT_ENTRY_BOOT_DS: usize = 3;
pub const GDT_ENTRY_BOOT_TSS: usize = 4;
pub const __BOOT_CS: usize = GDT_ENTRY_BOOT_CS * 8;
pub const __BOOT_DS: usize = GDT_ENTRY_BOOT_DS * 8;
pub const __BOOT_TSS: usize = GDT_ENTRY_BOOT_TSS * 8;
pub const SEGMENT_RPL_MASK: usize = 0x3;
pub const USER_SEGMENT_RPL_MASK: usize = 0x2;
pub const USER_RPL: usize = 0x3;
pub const SEGMENT_TI_MASK: usize = 0x4;
pub const SEGMENT_LDT: usize = 0x4;
pub const SEGMENT_GDT: usize = 0x0;
pub const GDT_ENTRY_INVALID_SEG: usize = 0;

// The following layout is selected by CONFIG_X86_32 && !BUILD_VDSO32_64.
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_TLS_MIN: usize = 6;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_TLS_MAX: usize = GDT_ENTRY_TLS_MIN + GDT_ENTRY_TLS_ENTRIES - 1;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_KERNEL_CS: usize = 12;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_KERNEL_DS: usize = 13;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_DEFAULT_USER_CS: usize = 14;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_DEFAULT_USER_DS: usize = 15;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_TSS: usize = 16;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_LDT: usize = 17;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_PNPBIOS_CS32: usize = 18;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_PNPBIOS_CS16: usize = 19;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_PNPBIOS_DS: usize = 20;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_PNPBIOS_TS1: usize = 21;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_PNPBIOS_TS2: usize = 22;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_APMBIOS_BASE: usize = 23;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_ESPFIX_SS: usize = 26;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_PERCPU: usize = 27;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_CPUNODE: usize = 28;
#[cfg(all(target_arch = "x86", not(feature = "build_vdso32_64")))]
pub const GDT_ENTRY_DOUBLEFAULT_TSS: usize = 31;

// 64-bit layout (the alternative branch in the C header).
#[cfg(any(target_arch = "x86_64", feature = "build_vdso32_64"))]
pub const GDT_ENTRY_KERNEL32_CS: usize = 1;
#[cfg(any(target_arch = "x86_64", feature = "build_vdso32_64"))]
pub const GDT_ENTRY_KERNEL_CS: usize = 2;
#[cfg(any(target_arch = "x86_64", feature = "build_vdso32_64"))]
pub const GDT_ENTRY_KERNEL_DS: usize = 3;
#[cfg(any(target_arch = "x86_64", feature = "build_vdso32_64"))]
pub const GDT_ENTRY_DEFAULT_USER32_CS: usize = 4;
#[cfg(any(target_arch = "x86_64", feature = "build_vdso32_64"))]
pub const GDT_ENTRY_DEFAULT_USER_DS: usize = 5;
#[cfg(any(target_arch = "x86_64", feature = "build_vdso32_64"))]
pub const GDT_ENTRY_DEFAULT_USER_CS: usize = 6;
#[cfg(any(target_arch = "x86_64", feature = "build_vdso32_64"))]
pub const GDT_ENTRY_TSS: usize = 8;
#[cfg(any(target_arch = "x86_64", feature = "build_vdso32_64"))]
pub const GDT_ENTRY_LDT: usize = 10;
#[cfg(any(target_arch = "x86_64", feature = "build_vdso32_64"))]
pub const GDT_ENTRY_TLS_MIN: usize = 12;
#[cfg(any(target_arch = "x86_64", feature = "build_vdso32_64"))]
pub const GDT_ENTRY_TLS_MAX: usize = 14;
#[cfg(any(target_arch = "x86_64", feature = "build_vdso32_64"))]
pub const GDT_ENTRY_CPUNODE: usize = 15;

pub const GDT_ENTRIES: usize = if cfg!(target_arch = "x86_64") { 16 } else { 32 };
pub const IDT_ENTRIES: usize = 256;
pub const NUM_EXCEPTION_VECTORS: usize = 32;
pub const EXCEPTION_ERRCODE_MASK: u32 = 0x20027d00;
pub const GDT_SIZE: usize = GDT_ENTRIES * 8;
pub const GDT_ENTRY_TLS_ENTRIES: usize = 3;
pub const TLS_SIZE: usize = GDT_ENTRY_TLS_ENTRIES * 8;
pub const VDSO_CPUNODE_BITS: usize = 12;
pub const VDSO_CPUNODE_MASK: usize = 0xfff;

#[cfg(not(target_arch = "x86_64"))]
pub const __KERNEL_CS: usize = GDT_ENTRY_KERNEL_CS * 8;
#[cfg(target_arch = "x86_64")]
pub const __KERNEL_CS: usize = GDT_ENTRY_KERNEL_CS * 8;
pub const __KERNEL_DS: usize = GDT_ENTRY_KERNEL_DS * 8;

pub unsafe fn vdso_encode_cpunode(cpu: i32, node: usize) -> usize {
    (node << VDSO_CPUNODE_BITS) | cpu as usize
}

pub unsafe fn vdso_read_cpunode(cpu: *mut u32, node: *mut u32) {
    let mut p: usize;
    // C uses alternative_io (LSL, or RDPID when available). The instruction
    // selection is supplied by the architecture alternative machinery.
    core::arch::asm!("lsl {0:e}, {1:e}", out(reg) p, in(reg) __CPUNODE_SEG, options(nostack));
    if !cpu.is_null() { *cpu = (p & VDSO_CPUNODE_MASK) as u32; }
    if !node.is_null() { *node = (p >> VDSO_CPUNODE_BITS) as u32; }
}

// Kernel-only declarations and constants.
pub const EARLY_IDT_HANDLER_SIZE: usize = 9 + 4; // ENDBR_INSN_SIZE
pub const XEN_EARLY_IDT_HANDLER_SIZE: usize = 8 + 4; // ENDBR_INSN_SIZE
extern "C" {
    pub static early_idt_handler_array: [[core::ffi::c_char; EARLY_IDT_HANDLER_SIZE]; NUM_EXCEPTION_VECTORS];
    pub fn early_ignore_irq();
    #[cfg(feature = "config_xen_pv")]
    pub static xen_early_idt_handler_array: [[core::ffi::c_char; XEN_EARLY_IDT_HANDLER_SIZE]; NUM_EXCEPTION_VECTORS];
}

#[inline(always)]
pub unsafe fn __savesegment_cs() -> usize { let v: usize; core::arch::asm!("mov {0}, cs", out(reg) v); v }
#[inline(always)]
pub unsafe fn __savesegment_ss() -> usize { let v: usize; core::arch::asm!("mov {0}, ss", out(reg) v); v }
#[inline(always)]
pub unsafe fn __savesegment_ds() -> usize { let v: usize; core::arch::asm!("mov {0}, ds", out(reg) v); v }
#[inline(always)]
pub unsafe fn __savesegment_es() -> usize { let v: usize; core::arch::asm!("mov {0}, es", out(reg) v); v }
#[inline(always)]
pub unsafe fn __savesegment_fs() -> usize { let v: usize; core::arch::asm!("mov {0}, fs", out(reg) v); v }
#[inline(always)]
pub unsafe fn __savesegment_gs() -> usize { let v: usize; core::arch::asm!("mov {0}, gs", out(reg) v); v }

// Segment loading uses the C exception-table machinery; its exact fixup is an
// external architecture dependency and is retained here as inline assembly.
#[inline(always)] pub unsafe fn __loadsegment_ss(value: u16) { core::arch::asm!("mov ss, {0:x}", in(reg) value, options(nostack)); }
#[inline(always)] pub unsafe fn __loadsegment_ds(value: u16) { core::arch::asm!("mov ds, {0:x}", in(reg) value, options(nostack)); }
#[inline(always)] pub unsafe fn __loadsegment_es(value: u16) { core::arch::asm!("mov es, {0:x}", in(reg) value, options(nostack)); }
#[cfg(target_arch = "x86")]
#[inline(always)] pub unsafe fn __loadsegment_fs(value: u16) { core::arch::asm!("mov fs, {0:x}", in(reg) value, options(nostack)); }
#[cfg(target_arch = "x86")]
#[inline(always)] pub unsafe fn __loadsegment_gs(value: u16) { core::arch::asm!("mov gs, {0:x}", in(reg) value, options(nostack)); }

#[macro_export]
macro_rules! loadsegment { ($seg:ident, $val:expr) => { $crate::__loadsegment_$seg($val) }; }
#[macro_export]
macro_rules! savesegment { ($seg:ident, $var:ident) => { $var = $crate::__savesegment_$seg() }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
