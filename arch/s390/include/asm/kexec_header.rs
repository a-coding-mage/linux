/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 2005
 *
 * Author(s): Rolf Adelsberger <adelsberger@de.ibm.com>
 *
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/module.h, asm/processor.h, asm/page.h, and asm/setup.h.

/*
 * KEXEC_SOURCE_MEMORY_LIMIT maximum page get_free_page can return.
 * I.e. Maximum page that is mapped directly into kernel memory,
 * and kmap is not required.
 */

/* Maximum physical address we can use pages from */
pub const KEXEC_SOURCE_MEMORY_LIMIT: usize = usize::MAX;

/* Maximum address we can reach in physical address mode */
pub const KEXEC_DESTINATION_MEMORY_LIMIT: usize = usize::MAX;

/* Maximum address we can use for the control pages */
/* Not more than 2GB */
pub const KEXEC_CONTROL_MEMORY_LIMIT: usize = 1usize << 31;

/* Allocate control page with GFP_DMA */
pub const KEXEC_CONTROL_MEMORY_GFP: usize = GFP_DMA | __GFP_NORETRY;

/* Maximum address we can use for the crash control pages */
pub const KEXEC_CRASH_CONTROL_MEMORY_LIMIT: usize = usize::MAX;

/* Allocate one page for the pdp and the second for the code */
pub const KEXEC_CONTROL_PAGE_SIZE: usize = 4096;

/* Alignment of crashkernel memory */
pub const KEXEC_CRASH_MEM_ALIGN: usize = HPAGE_SIZE;

/* The native architecture */
pub const KEXEC_ARCH: u32 = KEXEC_ARCH_S390;

/* Allow kexec_file to load a segment to 0 */
pub const KEXEC_BUF_MEM_UNKNOWN: i32 = -1;

/* Provide a dummy definition to avoid build failures. */
#[inline]
pub unsafe fn crash_setup_regs(_newregs: *mut pt_regs, _oldregs: *mut pt_regs) {}

pub struct kimage;

#[repr(C)]
pub struct s390_load_data {
    /* Pointer to the kernel buffer. Used to register cmdline etc.. */
    pub kernel_buf: *mut core::ffi::c_void,

    /* Load address of the kernel_buf. */
    pub kernel_mem: usize,

    /* Parmarea in the kernel buffer. */
    pub parm: *mut parmarea,

    /* Total size of loaded segments in memory. Used as an offset. */
    pub memsz: usize,

    pub report: *mut ipl_report,
}

extern "C" {
    pub fn s390_verify_sig(kernel: *const core::ffi::c_char, kernel_len: usize) -> i32;
    pub fn kexec_file_add_components(
        image: *mut kimage,
        add_kernel: Option<unsafe extern "C" fn(*mut kimage, *mut s390_load_data) -> i32>,
    ) -> *mut core::ffi::c_void;
    pub fn arch_kexec_do_relocs(
        r_type: i32,
        loc: *mut core::ffi::c_void,
        val: usize,
        addr: usize,
    ) -> i32;
}

/* ARCH_HAS_KIMAGE_ARCH */

#[repr(C)]
pub struct kimage_arch {
    pub ipl_buf: *mut core::ffi::c_void,
}

extern "C" {
    pub static s390_kexec_image_ops: kexec_file_ops;
    pub static s390_kexec_elf_ops: kexec_file_ops;
}

// The following declarations and aliases are present only when CONFIG_CRASH_DUMP is enabled.
#[cfg(CONFIG_CRASH_DUMP)]
extern "C" {
    pub fn crash_free_reserved_phys_range(begin: usize, end: usize);
    pub fn arch_kexec_protect_crashkres();
    pub fn arch_kexec_unprotect_crashkres();
    pub fn is_kdump_kernel() -> bool;
}

// The following declarations and aliases are present only when CONFIG_KEXEC_FILE is enabled.
#[cfg(CONFIG_KEXEC_FILE)]
pub struct purgatory_info;

#[cfg(CONFIG_KEXEC_FILE)]
extern "C" {
    pub fn arch_kexec_apply_relocations_add(
        pi: *mut purgatory_info,
        section: *mut Elf_Shdr,
        relsec: *const Elf_Shdr,
        symtab: *const Elf_Shdr,
    ) -> i32;
    pub fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
