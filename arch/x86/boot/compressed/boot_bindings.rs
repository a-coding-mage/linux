// SPDX-License-Identifier: GPL-2.0
//! Native declarations for the compressed-kernel C ABI.
// Bindgen output uses C names and contains declarations only.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, missing_docs, unreachable_pub)]
type __kernel_size_t = usize;
type __kernel_ssize_t = isize;
type __kernel_ptrdiff_t = isize;
include!(concat!(
    env!("LUPOS_BOOT_OBJ"),
    "/boot_bindings_generated.rs"
));
include!(concat!(env!("LUPOS_BOOT_OBJ"), "/boot_build.rs"));
pub const LOAD_PHYSICAL_ADDR: usize = LUPOS_BOOT_LOAD_PHYSICAL_ADDR as usize;
pub const BOOT_HEAP_SIZE: usize = LUPOS_BOOT_BOOT_HEAP_SIZE as usize;
pub const MIN_KERNEL_ALIGN: usize = LUPOS_BOOT_MIN_KERNEL_ALIGN as usize;
pub const KERNEL_IMAGE_SIZE: usize = LUPOS_BOOT_KERNEL_IMAGE_SIZE as usize;
pub const __START_KERNEL_map: usize = LUPOS_BOOT_START_KERNEL_MAP as usize;
pub const VO___start_rodata: usize = LUPOS_BOOT_VO_START_RODATA as usize;
pub const VO__text: usize = LUPOS_BOOT_VO_TEXT as usize;
pub const VO__sinittext: usize = LUPOS_BOOT_VO_SINITTEXT as usize;
pub const VO___inittext_end: usize = LUPOS_BOOT_VO_INITTEXT_END as usize;
pub const VO__end: usize = LUPOS_BOOT_VO_END as usize;
pub const VO___bss_start: usize = LUPOS_BOOT_VO_BSS_START as usize;
pub const PUD_SIZE: usize = LUPOS_BOOT_PUD_SIZE as usize;
pub const PUD_SHIFT: u32 = LUPOS_BOOT_PUD_SHIFT as u32;
pub const MSR_AMD64_SEV_ES_ENABLED: u64 = LUPOS_BOOT_SEV_ES_ENABLED as u64;
pub const EFI_MEMORY_MORE_RELIABLE: u64 = LUPOS_BOOT_EFI_MEMORY_MORE_RELIABLE as u64;
pub const EFI_MEMORY_SP: u64 = LUPOS_BOOT_EFI_MEMORY_SP as u64;
pub const E820_TYPE_RAM: u32 = LUPOS_BOOT_E820_TYPE_RAM as u32;
