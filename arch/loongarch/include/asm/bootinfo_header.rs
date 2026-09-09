/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, asm/setup.h

use core::ffi::{c_char, c_int, c_ulong};

extern "C" {
    pub fn get_system_type() -> *const c_char;

    pub fn init_environ();
    pub fn memblock_init();
    pub fn init_numa_memory() -> c_int;
}

#[repr(C)]
pub struct loongson_board_info {
    pub bios_size: c_int,
    pub bios_vendor: *const c_char,
    pub bios_version: *const c_char,
    pub bios_release_date: *const c_char,
    pub board_name: *const c_char,
    pub board_vendor: *const c_char,
}

// NR_CPUS and BITS_PER_LONG are supplied by the surrounding kernel translation.
pub const NR_WORDS: usize = (NR_CPUS + BITS_PER_LONG - 1) / BITS_PER_LONG;

/*
 * The "core" of cores_per_node and cores_per_package stands for a
 * logical core, which means in a SMT system it stands for a thread.
 */
#[repr(C)]
pub struct loongson_system_configuration {
    pub nr_cpus: c_int,
    pub nr_nodes: c_int,
    pub boot_cpu_id: c_int,
    pub cores_per_node: c_int,
    pub cores_per_package: c_int,
    pub cores_io_master: [c_ulong; NR_WORDS],
    pub suspend_addr: c_ulong,
    pub cpuname: *const c_char,
}

extern "C" {
    pub static mut efi_system_table: u64;
    pub static mut fw_arg0: c_ulong;
    pub static mut fw_arg1: c_ulong;
    pub static mut fw_arg2: c_ulong;
    pub static mut b_info: loongson_board_info;
    pub static mut loongson_sysconf: loongson_system_configuration;

    fn test_bit(bit: c_ulong, addr: *const c_ulong) -> bool;
}

pub unsafe fn io_master(cpu: c_int) -> bool {
    test_bit(cpu as c_ulong, loongson_sysconf.cores_io_master.as_ptr())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
