/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/include/asm/setup.h
 *
 *  Copyright (C) 1997-1999 Russell King
 *
 *  Structure passed to kernel to tell it about the
 *  hardware it's running on.  See Documentation/arch/arm/setup.rst
 *  for more info.
 */

// C dependencies: <linux/screen_info.h> and <uapi/asm/setup.h>.

// The C __tag macro applies __used and places an object in ".taglist.init".
// Those linker-section/build attributes are preserved here as declaration intent.
#[allow(unused_macros)]
macro_rules! __tagtable {
    ($tag:expr, $fn:ident) => {
        #[used]
        #[link_section = ".taglist.init"]
        static __tagtable_$fn: TagTable = TagTable { tag: $tag, fn_: $fn };
    };
}

extern "C" {
    pub fn arm_add_memory(start: u64, size: u64) -> i32;
    pub fn early_print(str_: *const core::ffi::c_char, ...);
    pub fn dump_machine_table();
}

#[cfg(feature = "CONFIG_ATAGS_PROC")]
extern "C" {
    pub fn save_atags(tags: *const Tag);
}

#[cfg(not(feature = "CONFIG_ATAGS_PROC"))]
#[inline]
pub unsafe fn save_atags(_tags: *const Tag) {}

#[repr(C)]
pub struct TagTable {
    pub tag: u32,
    pub fn_: unsafe extern "C" fn(),
}

#[repr(C)]
pub struct Tag {
    _private: [u8; 0],
}

#[repr(C)]
pub struct MachineDesc {
    _private: [u8; 0],
}

extern "C" {
    pub fn init_default_cache_policy(policy: c_ulong);
    pub fn paging_init(desc: *const MachineDesc);
    pub fn early_mm_init(desc: *const MachineDesc);
    pub fn adjust_lowmem_bounds();
    pub fn setup_dma_zone(desc: *const MachineDesc);
}

#[cfg(feature = "CONFIG_VGA_CONSOLE")]
extern "C" {
    pub static mut vgacon_screen_info: ScreenInfo;
}

// External declaration supplied by <linux/screen_info.h>.
#[repr(C)]
pub struct ScreenInfo {
    _private: [u8; 0],
}

type c_ulong = core::ffi::c_ulong;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
