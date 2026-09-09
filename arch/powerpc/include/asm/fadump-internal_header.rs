/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Firmware-Assisted Dump internal code. */

use core::ffi::{c_char, c_int, c_long, c_ulong};

pub const FADUMP_MAX_MEM_REGS: usize = 128;

#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
pub const MAX_BOOT_MEM_RATIO: c_int = 4;

/* C macro: (memblock.memblock_type.cnt). */
#[macro_export]
macro_rules! memblock_num_regions {
    ($memblock_type:ident) => { memblock.$memblock_type.cnt };
}

#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
pub const FADUMP_REGISTER: c_int = 1;
#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
pub const FADUMP_UNREGISTER: c_int = 2;
#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
pub const FADUMP_INVALIDATE: c_int = 3;

#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
#[inline]
pub unsafe fn fadump_str_to_u64(mut str_: *const c_char) -> u64 {
    let mut val: u64 = 0;
    let mut i = 0;
    while i < core::mem::size_of::<u64>() {
        let ch = *str_;
        val = if ch != 0 {
            str_ = str_.add(1);
            (val << 8) | (ch as u8 as u64)
        } else {
            val << 8
        };
        i += 1;
    }
    val
}

#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
pub const FADUMP_CPU_UNKNOWN: u32 = !0u32;
#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
pub const FADUMP_HEADER_VERSION: u32 = 1;
pub const RNG_NAME_SZ: usize = 16;

/* The C macros below invoke fadump_str_to_u64 on string literals. */
#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
pub const FADUMP_CRASH_INFO_MAGIC_OLD: u64 = 0x4641_444d_5049_4e46;
#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
pub const FADUMP_CRASH_INFO_MAGIC: u64 = 0x4641_444d_5053_4947;

#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
#[repr(C)]
pub struct fadump_crash_info_header {
    pub magic_number: u64,
    pub version: u32,
    pub crashing_cpu: u32,
    pub vmcoreinfo_raddr: u64,
    pub vmcoreinfo_size: u64,
    pub pt_regs_sz: u32,
    pub cpu_mask_sz: u32,
    pub regs: pt_regs,
    pub cpu_mask: cpumask,
}

#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
#[repr(C)]
pub struct fadump_memory_range {
    pub base: u64,
    pub size: u64,
}

#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
#[repr(C)]
pub struct fadump_mrange_info {
    pub name: [c_char; RNG_NAME_SZ],
    pub mem_ranges: *mut fadump_memory_range,
    pub mem_ranges_sz: u32,
    pub mem_range_cnt: u32,
    pub max_mem_ranges: u32,
    pub is_static: bool,
}

#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
#[repr(C)]
pub struct fw_dump {
    pub reserve_dump_area_start: c_ulong,
    pub reserve_dump_area_size: c_ulong,
    pub reserve_bootvar: c_ulong,
    pub cpu_state_data_size: c_ulong,
    pub cpu_state_dest_vaddr: u64,
    pub cpu_state_data_version: u32,
    pub cpu_state_entry_size: u32,
    pub hpte_region_size: c_ulong,
    pub boot_memory_size: c_ulong,
    pub boot_mem_dest_addr: u64,
    pub boot_mem_addr: [u64; FADUMP_MAX_MEM_REGS],
    pub boot_mem_sz: [u64; FADUMP_MAX_MEM_REGS],
    pub boot_mem_top: u64,
    pub boot_mem_regs_cnt: u64,
    pub fadumphdr_addr: c_ulong,
    pub elfcorehdr_addr: u64,
    pub elfcorehdr_size: u64,
    pub cpu_notes_buf_vaddr: c_ulong,
    pub cpu_notes_buf_size: c_ulong,
    pub param_area: c_ulong,
    pub max_copy_size: u64,
    pub kernel_metadata: u64,
    pub ibm_configure_kernel_dump: c_int,
    pub fadump_enabled: c_ulong,
    pub fadump_supported: c_ulong,
    pub dump_active: c_ulong,
    pub dump_registered: c_ulong,
    pub nocma: c_ulong,
    pub param_area_supported: c_ulong,
    pub ops: *mut fadump_ops,
}

/* C function pointer declarations are represented as nullable C ABI pointers. */
#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
#[repr(C)]
pub struct fadump_ops {
    pub fadump_init_mem_struct: Option<unsafe extern "C" fn(*mut fw_dump) -> u64>,
    pub fadump_get_metadata_size: Option<unsafe extern "C" fn() -> u64>,
    pub fadump_setup_metadata: Option<unsafe extern "C" fn(*mut fw_dump) -> c_int>,
    pub fadump_get_bootmem_min: Option<unsafe extern "C" fn() -> u64>,
    pub fadump_register: Option<unsafe extern "C" fn(*mut fw_dump) -> c_int>,
    pub fadump_unregister: Option<unsafe extern "C" fn(*mut fw_dump) -> c_int>,
    pub fadump_invalidate: Option<unsafe extern "C" fn(*mut fw_dump) -> c_int>,
    pub fadump_cleanup: Option<unsafe extern "C" fn(*mut fw_dump)>,
    pub fadump_process: Option<unsafe extern "C" fn(*mut fw_dump) -> c_int>,
    pub fadump_region_show: Option<unsafe extern "C" fn(*mut fw_dump, *mut seq_file)>,
    pub fadump_trigger: Option<unsafe extern "C" fn(*mut fadump_crash_info_header, *const c_char)>,
    pub fadump_max_boot_mem_rgns: Option<unsafe extern "C" fn() -> c_int>,
}

#[cfg(not(feature = "CONFIG_PRESERVE_FA_DUMP"))]
extern "C" {
    pub fn fadump_setup_cpu_notes_buf(num_cpus: u32) -> i32;
    pub fn fadump_free_cpu_notes_buf();
    pub fn fadump_regs_to_elf_notes(buf: *mut u32, regs: *mut pt_regs) -> *mut u32;
    pub fn fadump_update_elfcore_header(bufp: *mut c_char);
    pub fn is_fadump_reserved_mem_contiguous() -> bool;
}

#[cfg(feature = "CONFIG_PRESERVE_FA_DUMP")]
#[repr(C)]
pub struct fw_dump {
    pub boot_mem_top: u64,
    pub dump_active: u64,
}

#[cfg(feature = "CONFIG_PPC_PSERIES")]
extern "C" { pub fn rtas_fadump_dt_scan(fadump_conf: *mut fw_dump, node: u64); }
#[cfg(not(feature = "CONFIG_PPC_PSERIES"))]
#[inline] pub unsafe fn rtas_fadump_dt_scan(_: *mut fw_dump, _: u64) {}

#[cfg(feature = "CONFIG_PPC_POWERNV")]
extern "C" { pub fn opal_fadump_dt_scan(fadump_conf: *mut fw_dump, node: u64); }
#[cfg(not(feature = "CONFIG_PPC_POWERNV"))]
#[inline] pub unsafe fn opal_fadump_dt_scan(_: *mut fw_dump, _: u64) {}

/* External types supplied by other headers. */
#[allow(non_camel_case_types)]
pub type s32 = i32;
#[allow(non_camel_case_types)]
pub type pt_regs = crate::pt_regs;
#[allow(non_camel_case_types)]
pub type cpumask = crate::cpumask;
#[allow(non_camel_case_types)]
pub type seq_file = crate::seq_file;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
