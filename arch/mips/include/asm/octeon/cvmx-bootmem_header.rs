/*
 * Simple allocate only memory allocator. Used to allocate memory at
 * application start time.
 *
 * This is a source-level Rust translation of cvmx-bootmem.h.
 */

/* Must be multiple of 8, changing breaks ABI */
pub const CVMX_BOOTMEM_NAME_LEN: usize = 128;

/* Can change without breaking ABI */
pub const CVMX_BOOTMEM_NUM_NAMED_BLOCKS: usize = 64;

/* minimum alignment of bootmem alloced blocks */
pub const CVMX_BOOTMEM_ALIGNMENT_SIZE: u64 = 16u64;

/* Flags for cvmx_bootmem_phy_mem* functions */
/* Allocate from end of block instead of beginning */
pub const CVMX_BOOTMEM_FLAG_END_ALLOC: u32 = 1 << 0;

/* Don't do any locking. */
pub const CVMX_BOOTMEM_FLAG_NO_LOCKING: u32 = 1 << 1;

/* First bytes of each free physical block of memory contain this structure,
 * which is used to maintain the free memory list. Since the bootloader is
 * only 32 bits, there is a union providing 64 and 32 bit versions. The
 * application init code converts addresses to 64 bit addresses before the
 * application starts.
 */
#[repr(C)]
pub struct cvmx_bootmem_block_header {
    /* Referenced from assembly routines in the bootloader; do not change
     * without changing those routines as well. */
    pub next_block_addr: u64,
    pub size: u64,
}

/* Structure for named memory blocks. */
#[repr(C)]
pub struct cvmx_bootmem_named_block_desc {
    /* Base address of named block */
    pub base_addr: u64,
    /* Size actually allocated for named block (may differ from requested). */
    pub size: u64,
    /* name of named block */
    pub name: [std::ffi::c_char; CVMX_BOOTMEM_NAME_LEN],
}

/* Current descriptor versions */
/* CVMX bootmem descriptor major version */
pub const CVMX_BOOTMEM_DESC_MAJ_VER: u32 = 3;
/* CVMX bootmem descriptor minor version */
pub const CVMX_BOOTMEM_DESC_MIN_VER: u32 = 0;

/* First three members of cvmx_bootmem_desc_t are left in original positions
 * for backwards compatibility. */
#[repr(C)]
pub struct cvmx_bootmem_desc {
    #[cfg(any(target_endian = "big", feature = "cvmx_build_for_linux_host"))]
    pub lock: u32,
    #[cfg(any(target_endian = "big", feature = "cvmx_build_for_linux_host"))]
    pub flags: u32,
    #[cfg(any(target_endian = "big", feature = "cvmx_build_for_linux_host"))]
    pub head_addr: u64,
    #[cfg(any(target_endian = "big", feature = "cvmx_build_for_linux_host"))]
    pub major_version: u32,
    #[cfg(any(target_endian = "big", feature = "cvmx_build_for_linux_host"))]
    pub minor_version: u32,
    #[cfg(not(any(target_endian = "big", feature = "cvmx_build_for_linux_host")))]
    pub flags: u32,
    #[cfg(not(any(target_endian = "big", feature = "cvmx_build_for_linux_host")))]
    pub lock: u32,
    #[cfg(not(any(target_endian = "big", feature = "cvmx_build_for_linux_host")))]
    pub head_addr: u64,
    #[cfg(not(any(target_endian = "big", feature = "cvmx_build_for_linux_host")))]
    pub minor_version: u32,
    #[cfg(not(any(target_endian = "big", feature = "cvmx_build_for_linux_host")))]
    pub major_version: u32,
    pub app_data_addr: u64,
    pub app_data_size: u64,
    #[cfg(any(target_endian = "big", feature = "cvmx_build_for_linux_host"))]
    pub named_block_num_blocks: u32,
    #[cfg(any(target_endian = "big", feature = "cvmx_build_for_linux_host"))]
    pub named_block_name_len: u32,
    #[cfg(not(any(target_endian = "big", feature = "cvmx_build_for_linux_host")))]
    pub named_block_name_len: u32,
    #[cfg(not(any(target_endian = "big", feature = "cvmx_build_for_linux_host")))]
    pub named_block_num_blocks: u32,
    pub named_block_array_addr: u64,
}

unsafe extern "C" {
    pub fn cvmx_bootmem_init(mem_desc_ptr: *mut std::ffi::c_void) -> std::ffi::c_int;
    pub fn cvmx_bootmem_alloc_address(size: u64, address: u64, alignment: u64) -> *mut std::ffi::c_void;
    pub fn cvmx_bootmem_alloc_named(size: u64, alignment: u64, name: *mut std::ffi::c_char) -> *mut std::ffi::c_void;
    pub fn cvmx_bootmem_alloc_named_range(size: u64, min_addr: u64, max_addr: u64, align: u64, name: *mut std::ffi::c_char) -> *mut std::ffi::c_void;
    pub fn cvmx_bootmem_alloc_named_range_once(size: u64, min_addr: u64, max_addr: u64, align: u64, name: *mut std::ffi::c_char, init: Option<unsafe extern "C" fn(*mut std::ffi::c_void)> ) -> *mut std::ffi::c_void;
    pub fn cvmx_bootmem_free_named(name: *mut std::ffi::c_char) -> std::ffi::c_int;
    pub fn cvmx_bootmem_find_named_block(name: *mut std::ffi::c_char) -> *mut cvmx_bootmem_named_block_desc;
    pub fn cvmx_bootmem_phy_alloc(req_size: u64, address_min: u64, address_max: u64, alignment: u64, flags: u32) -> i64;
    pub fn cvmx_bootmem_phy_named_block_alloc(size: u64, min_addr: u64, max_addr: u64, alignment: u64, name: *mut std::ffi::c_char, flags: u32) -> i64;
    pub fn __cvmx_bootmem_phy_free(phy_addr: u64, size: u64, flags: u32) -> std::ffi::c_int;
    pub fn cvmx_bootmem_lock();
    pub fn cvmx_bootmem_unlock();
    pub fn cvmx_bootmem_get_desc() -> *mut cvmx_bootmem_desc;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
