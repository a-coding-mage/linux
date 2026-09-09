/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// `PMD_SHIFT`, `PAGE_SHIFT`, `THREAD_SIZE_ORDER`, `PAGE_SIZE`, and
// `CONFIG_PHYSICAL_ALIGN`.

/* Minimum kernel alignment, as a power of two */
#[cfg(feature = "CONFIG_X86_64")]
pub const MIN_KERNEL_ALIGN_LG2: usize = PMD_SHIFT;
#[cfg(not(feature = "CONFIG_X86_64"))]
pub const MIN_KERNEL_ALIGN_LG2: usize = PAGE_SHIFT + THREAD_SIZE_ORDER;
pub const MIN_KERNEL_ALIGN: usize = 1usize << MIN_KERNEL_ALIGN_LG2;

// C build-time validation:
// CONFIG_PHYSICAL_ALIGN must be a power of two and must be at least
// MIN_KERNEL_ALIGN.

#[cfg(feature = "CONFIG_KERNEL_BZIP2")]
pub const BOOT_HEAP_SIZE: usize = 0x400000;
#[cfg(all(
    not(feature = "CONFIG_KERNEL_BZIP2"),
    feature = "CONFIG_KERNEL_ZSTD"
))]
/*
 * Zstd needs to allocate the ZSTD_DCtx in order to decompress the kernel.
 * The ZSTD_DCtx is ~160KB, so set the heap size to 192KB because it is a
 * round number and to allow some slack.
 */
pub const BOOT_HEAP_SIZE: usize = 0x30000;
#[cfg(all(
    not(feature = "CONFIG_KERNEL_BZIP2"),
    not(feature = "CONFIG_KERNEL_ZSTD")
))]
pub const BOOT_HEAP_SIZE: usize = 0x10000;

#[cfg(feature = "CONFIG_X86_64")]
pub const BOOT_STACK_SIZE: usize = 0x4000;

/*
 * Used by decompressor's startup_32() to allocate page tables for identity
 * mapping of the 4G of RAM in 4-level paging mode:
 * - 1 level4 table;
 * - 1 level3 table;
 * - 4 level2 table that maps everything with 2M pages;
 *
 * The additional level5 table needed for 5-level paging is allocated from
 * trampoline_32bit memory.
 */
#[cfg(feature = "CONFIG_X86_64")]
pub const BOOT_INIT_PGT_SIZE: usize = 6 * 4096;

/*
 * Total number of page tables kernel_add_identity_map() can allocate,
 * including page tables consumed by startup_32().
 *
 * Worst-case scenario:
 *  - 5-level paging needs 1 level5 table;
 *  - KASLR needs to map kernel, boot_params, cmdline and randomized kernel,
 *    assuming all of them cross 256T boundary:
 *    + 4*2 level4 table;
 *    + 4*2 level3 table;
 *    + 4*2 level2 table;
 *  - X86_VERBOSE_BOOTUP needs to map the first 2M (video RAM):
 *    + 1 level4 table;
 *    + 1 level3 table;
 *    + 1 level2 table;
 * Total: 28 tables
 *
 * Add 4 spare table in case decompressor touches anything beyond what is
 * accounted above. Warn if it happens.
 */
#[cfg(feature = "CONFIG_X86_64")]
pub const BOOT_PGT_SIZE_WARN: usize = 28 * 4096;
#[cfg(feature = "CONFIG_X86_64")]
pub const BOOT_PGT_SIZE: usize = 32 * 4096;

#[cfg(not(feature = "CONFIG_X86_64"))]
pub const BOOT_STACK_SIZE: usize = 0x1000;

pub const TRAMPOLINE_32BIT_SIZE: usize = 2 * PAGE_SIZE;
pub const TRAMPOLINE_32BIT_CODE_OFFSET: usize = PAGE_SIZE;
pub const TRAMPOLINE_32BIT_CODE_SIZE: usize = 0xA0;

#[allow(improper_ctypes)]
extern "C" {
    pub static mut output_len: u32;
    pub static kernel_text_size: usize;
    pub static kernel_inittext_offset: usize;
    pub static kernel_inittext_size: usize;
    pub static kernel_total_size: usize;

    pub fn decompress_kernel(
        outbuf: *mut u8,
        virt_addr: usize,
        error: Option<unsafe extern "C" fn(x: *mut i8)>,
    ) -> usize;

    pub static mut boot_params_ptr: *mut boot_params;
    pub static mut trampoline_32bit: *mut usize;
    pub static trampoline_ljmp_imm_offset: u16;

    pub fn trampoline_32bit_src(trampoline: *mut core::ffi::c_void, enable_5lvl: bool);
}

#[repr(C)]
pub struct boot_params {
    _opaque: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
