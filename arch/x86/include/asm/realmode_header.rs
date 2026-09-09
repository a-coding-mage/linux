/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Flag bit definitions for use with the flags field of the trampoline header
 * in the CONFIG_X86_64 variant.
 */
pub const TH_FLAGS_SME_ACTIVE_BIT: u32 = 0;
pub const TH_FLAGS_SME_ACTIVE: u32 = 1u32 << TH_FLAGS_SME_ACTIVE_BIT;

/* This must match data at realmode/rm/header.S */
#[repr(C)]
pub struct real_mode_header {
    pub text_start: u32,
    pub ro_end: u32,
    /* SMP trampoline */
    pub trampoline_start: u32,
    pub trampoline_header: u32,
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    pub sev_es_trampoline_start: u32,
    #[cfg(CONFIG_X86_64)]
    pub trampoline_start64: u32,
    #[cfg(CONFIG_X86_64)]
    pub trampoline_pgd: u32,
    /* ACPI S3 wakeup */
    #[cfg(CONFIG_ACPI_SLEEP)]
    pub wakeup_start: u32,
    #[cfg(CONFIG_ACPI_SLEEP)]
    pub wakeup_header: u32,
    /* APM/BIOS reboot */
    pub machine_real_restart_asm: u32,
    #[cfg(CONFIG_X86_64)]
    pub machine_real_restart_seg: u32,
}

/* This must match data at realmode/rm/trampoline_{32,64}.S */
#[repr(C)]
pub struct trampoline_header {
    #[cfg(CONFIG_X86_32)]
    pub start: u32,
    #[cfg(CONFIG_X86_32)]
    pub gdt_pad: u16,
    #[cfg(CONFIG_X86_32)]
    pub gdt_limit: u16,
    #[cfg(CONFIG_X86_32)]
    pub gdt_base: u32,
    #[cfg(not(CONFIG_X86_32))]
    pub start: u64,
    #[cfg(not(CONFIG_X86_32))]
    pub efer: u64,
    #[cfg(not(CONFIG_X86_32))]
    pub cr4: u32,
    #[cfg(not(CONFIG_X86_32))]
    pub flags: u32,
    #[cfg(not(CONFIG_X86_32))]
    pub lock: u32,
}

unsafe extern "C" {
    pub static mut real_mode_header: *mut real_mode_header;
    pub static mut real_mode_blob_end: u8;

    pub static mut initial_code: core::ffi::c_ulong;
    pub static mut initial_stack: core::ffi::c_ulong;
    #[cfg(CONFIG_AMD_MEM_ENCRYPT)]
    pub static mut initial_vc_handler: core::ffi::c_ulong;

    pub static mut trampoline_lock: *mut u32;

    pub static mut real_mode_blob: u8;
    pub static mut real_mode_relocs: u8;

    #[cfg(CONFIG_X86_32)]
    pub static mut startup_32_smp: u8;
    #[cfg(CONFIG_X86_32)]
    pub static mut boot_gdt: u8;
    #[cfg(not(CONFIG_X86_32))]
    pub static mut secondary_startup_64: u8;
    #[cfg(not(CONFIG_X86_32))]
    pub static mut secondary_startup_64_no_verify: u8;

    pub fn reserve_real_mode();
    pub fn load_trampoline_pgtable();
    pub fn init_real_mode();
}

#[inline(always)]
pub unsafe fn real_mode_size_needed() -> usize {
    if !real_mode_header.is_null() {
        return 0; /* already allocated. */
    }

    ALIGN(
        (&raw const real_mode_blob_end as usize)
            .wrapping_sub(&raw const real_mode_blob as usize),
        PAGE_SIZE,
    )
}

#[inline]
pub unsafe fn set_real_mode_mem(mem: phys_addr_t) {
    real_mode_header = __va(mem) as *mut real_mode_header;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
