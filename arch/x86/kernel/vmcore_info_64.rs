// SPDX-License-Identifier: GPL-2.0-only

// Declarations supplied by the Linux vmcore, page-table, and x86 setup
// interfaces included by the original source.
unsafe extern "C" {
    static sme_me_mask: u64;
    static phys_base: u64;
    static init_top_pgt: u8;
    static pgtable_l5_enabled: unsafe extern "C" fn() -> bool;
    static kaslr_offset: unsafe extern "C" fn() -> usize;
    static KERNEL_IMAGE_SIZE: u64;
    unsafe fn vmcoreinfo_append_str(format: *const core::ffi::c_char, ...);

    // Rust declarations corresponding to the VMCOREINFO_* macros.
    unsafe fn VMCOREINFO_NUMBER(value: *const u8);
    unsafe fn VMCOREINFO_SYMBOL(value: *const u8);
    unsafe fn VMCOREINFO_LENGTH(value: *const u8, length: usize);
}

#[cfg(feature = "CONFIG_NUMA")]
unsafe extern "C" {
    static node_data: u8;
    static MAX_NUMNODES: usize;
}

pub unsafe fn arch_crash_save_vmcoreinfo() {
    let sme_mask: u64 = sme_me_mask;

    VMCOREINFO_NUMBER((&raw const phys_base).cast::<u8>());
    VMCOREINFO_SYMBOL((&raw const init_top_pgt).cast::<u8>());
    vmcoreinfo_append_str(
        c"NUMBER(pgtable_l5_enabled)=%d\n".as_ptr(),
        pgtable_l5_enabled(),
    );

    #[cfg(feature = "CONFIG_NUMA")]
    {
        VMCOREINFO_SYMBOL((&raw const node_data).cast::<u8>());
        VMCOREINFO_LENGTH((&raw const node_data).cast::<u8>(), MAX_NUMNODES);
    }

    vmcoreinfo_append_str(c"KERNELOFFSET=%lx\n".as_ptr(), kaslr_offset());
    VMCOREINFO_NUMBER((&raw const KERNEL_IMAGE_SIZE).cast::<u8>());
    VMCOREINFO_NUMBER((&raw const sme_mask).cast::<u8>());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
