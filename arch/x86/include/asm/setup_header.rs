/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of the x86 setup header. */

pub const COMMAND_LINE_SIZE: usize = 2048;

/* __i386__: MAXMEM_PFN = PFN_DOWN(MAXMEM), MAX_NONPAE_PFN = (1 << 20). */
#[cfg(target_arch = "x86")]
pub const MAX_NONPAE_PFN: usize = 1usize << 20;

pub const PARAM_SIZE: usize = 4096; /* sizeof(struct boot_params) */

pub const OLD_CL_MAGIC: u16 = 0xA33F;
pub const OLD_CL_ADDRESS: usize = 0x020; /* Relative to real mode data */
pub const NEW_CL_POINTER: usize = 0x228; /* Relative to real mode data */

/* Interrupt control for vSMPowered x86_64 systems. */
#[cfg(feature = "CONFIG_X86_64")]
extern "C" {
    pub fn vsmp_init();
}
#[cfg(not(feature = "CONFIG_X86_64"))]
#[inline]
pub fn vsmp_init() {}

pub struct pt_regs;

extern "C" {
    pub fn setup_bios_corruption_check();
    pub fn early_platform_quirks();

    pub static mut saved_video_mode: ::core::ffi::c_ulong;
    pub static mut acpi_realmode_flags: ::core::ffi::c_ulong;

    pub fn reserve_standard_io_resources();
    pub fn i386_reserve_resources();
    pub fn __startup_64(
        p2v_offset: ::core::ffi::c_ulong,
        bp: *mut boot_params,
    ) -> ::core::ffi::c_ulong;
    pub fn startup_64_setup_gdt_idt();
    pub fn startup_64_load_idt(vc_handler: *mut ::core::ffi::c_void);
    pub fn __pi_startup_64_load_idt(vc_handler: *mut ::core::ffi::c_void);
    pub fn early_setup_idt();
    pub fn do_early_exception(regs: *mut pt_regs, trapnr: ::core::ffi::c_int);
}

/* CONFIG_X86_INTEL_MID: declare x86_intel_mid_early_setup; otherwise an empty inline. */
#[cfg(feature = "CONFIG_X86_INTEL_MID")]
extern "C" {
    pub fn x86_intel_mid_early_setup();
}
#[cfg(not(feature = "CONFIG_X86_INTEL_MID"))]
#[inline]
pub fn x86_intel_mid_early_setup() {}

/* CONFIG_X86_INTEL_CE: declare x86_ce4100_early_setup; otherwise an empty inline. */
#[cfg(feature = "CONFIG_X86_INTEL_CE")]
extern "C" {
    pub fn x86_ce4100_early_setup();
}
#[cfg(not(feature = "CONFIG_X86_INTEL_CE"))]
#[inline]
pub fn x86_ce4100_early_setup() {}

/* Declarations below are omitted from assembler builds in the C header. */
extern "C" {
    pub static mut boot_params: boot_params;
    pub static mut _text: [::core::ffi::c_char; 0];
    pub static mut _brk_end: ::core::ffi::c_ulong;
    pub fn extend_brk(size: usize, align: usize) -> *mut ::core::ffi::c_void;
    pub fn probe_roms();
    pub fn clear_bss();
}

#[inline]
pub unsafe fn kaslr_enabled() -> bool {
    /* IS_ENABLED(CONFIG_RANDOMIZE_MEMORY) && !!(boot_params.hdr.loadflags & KASLR_FLAG) */
    cfg!(feature = "CONFIG_RANDOMIZE_MEMORY")
        && ((boot_params.hdr.loadflags & KASLR_FLAG) != 0)
}

#[inline]
pub unsafe fn kaslr_memory_enabled() -> bool {
    kaslr_enabled() && !cfg!(feature = "CONFIG_KASAN")
}

#[inline]
pub unsafe fn kaslr_offset() -> ::core::ffi::c_ulong {
    (_text.as_ptr() as usize as ::core::ffi::c_ulong).wrapping_sub(__START_KERNEL)
}

/* Do NOT EVER look at the BIOS memory size location. */
pub const LOWMEMSIZE: usize = 0x9f000;

/* RESERVE_BRK(name, size) declares a used, one-byte-aligned .bss..brk object. */

#[cfg(target_arch = "x86")]
extern "C" {
    pub fn i386_start_kernel() -> !;
    pub fn mk_early_pgtbl_32();
}

#[cfg(not(target_arch = "x86"))]
extern "C" {
    pub fn x86_64_start_kernel(real_mode: *mut ::core::ffi::c_char) -> !;
    pub fn x86_64_start_reservations(real_mode_data: *mut ::core::ffi::c_char) -> !;
}

#[cfg(feature = "CONFIG_CMDLINE_BOOL")]
extern "C" {
    pub static mut builtin_cmdline_added: bool;
}
#[cfg(not(feature = "CONFIG_CMDLINE_BOOL"))]
pub const builtin_cmdline_added: bool = false;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
