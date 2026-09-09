/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: linux/types.h, linux/moduleloader.h, linux/cleanup.h

// When CONFIG_KASAN_GENERIC or CONFIG_KASAN_SW_TAGS is enabled without
// CONFIG_KASAN_VMALLOC, MODULE_ALIGN is PAGE_SIZE shifted by
// KASAN_SHADOW_SCALE_SHIFT; otherwise it is PAGE_SIZE.
#[cfg(all(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS), not(CONFIG_KASAN_VMALLOC)))]
pub const MODULE_ALIGN: usize = PAGE_SIZE << KASAN_SHADOW_SCALE_SHIFT;
#[cfg(not(all(any(CONFIG_KASAN_GENERIC, CONFIG_KASAN_SW_TAGS), not(CONFIG_KASAN_VMALLOC))))]
pub const MODULE_ALIGN: usize = PAGE_SIZE;

/**
 * enum execmem_type - types of executable memory ranges
 *
 * There are several subsystems that allocate executable memory.
 * Architectures define different restrictions on placement,
 * permissions, alignment and other parameters for memory that can be used
 * by these subsystems.
 * Types in this enum identify subsystems that allocate executable memory
 * and let architectures define parameters for ranges suitable
 * for allocations by each subsystem.
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum execmem_type {
    EXECMEM_DEFAULT = 0,
    EXECMEM_MODULE_TEXT = 0,
    EXECMEM_KPROBES = 1,
    EXECMEM_FTRACE = 2,
    EXECMEM_BPF = 3,
    EXECMEM_MODULE_DATA = 4,
    EXECMEM_TYPE_MAX = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum execmem_range_flags {
    EXECMEM_KASAN_SHADOW = 1 << 0,
    EXECMEM_ROX_CACHE = 1 << 1,
}

#[cfg(CONFIG_ARCH_HAS_EXECMEM_ROX)]
extern "C" {
    pub fn execmem_fill_trapping_insns(ptr: *mut core::ffi::c_void, size: usize);
    pub fn execmem_restore_rox(ptr: *mut core::ffi::c_void, size: usize) -> i32;
}

#[cfg(not(CONFIG_ARCH_HAS_EXECMEM_ROX))]
#[inline]
pub unsafe fn execmem_restore_rox(_ptr: *mut core::ffi::c_void, _size: usize) -> i32 {
    0
}

#[repr(C)]
pub struct execmem_range {
    pub start: usize,
    pub end: usize,
    pub fallback_start: usize,
    pub fallback_end: usize,
    pub pgprot: pgprot_t,
    pub alignment: u32,
    pub flags: execmem_range_flags,
}

#[repr(C)]
pub struct execmem_info {
    pub ranges: [execmem_range; execmem_type::EXECMEM_TYPE_MAX as usize],
}

extern "C" {
    pub fn execmem_arch_setup() -> *mut execmem_info;
    pub fn execmem_alloc(type_: execmem_type, size: usize) -> *mut core::ffi::c_void;
    pub fn execmem_alloc_rw(type_: execmem_type, size: usize) -> *mut core::ffi::c_void;
    pub fn execmem_free(ptr: *mut core::ffi::c_void);
}

// DEFINE_FREE(execmem, void *, if (_T) execmem_free(_T));

#[cfg(CONFIG_MMU)]
extern "C" {
    pub fn execmem_vmap(size: usize) -> *mut vm_struct;
}

extern "C" {
    pub fn execmem_is_rox(type_: execmem_type) -> bool;
}

#[cfg(all(CONFIG_EXECMEM, not(CONFIG_ARCH_WANTS_EXECMEM_LATE)))]
extern "C" {
    pub fn execmem_init();
}

#[cfg(not(all(CONFIG_EXECMEM, not(CONFIG_ARCH_WANTS_EXECMEM_LATE))))]
#[inline]
pub fn execmem_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
