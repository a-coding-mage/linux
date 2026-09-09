/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This file contains prototypes provided by each m68k machine
 * to parse bootinfo data structures and to configure the machine
 */

/* The C header guard is omitted; Rust modules provide equivalent scoping. */

use core::ffi::c_int;

/* Supplied by the bootinfo definitions in another translation unit. */
pub enum bi_record {}

extern "C" {
    pub fn amiga_parse_bootinfo(record: *const bi_record) -> c_int;
    pub fn apollo_parse_bootinfo(record: *const bi_record) -> c_int;
    pub fn atari_parse_bootinfo(record: *const bi_record) -> c_int;
    pub fn bvme6000_parse_bootinfo(record: *const bi_record) -> c_int;
    pub fn hp300_parse_bootinfo(record: *const bi_record) -> c_int;
    pub fn mac_parse_bootinfo(record: *const bi_record) -> c_int;
    pub fn mvme147_parse_bootinfo(record: *const bi_record) -> c_int;
    pub fn mvme16x_parse_bootinfo(record: *const bi_record) -> c_int;
    pub fn q40_parse_bootinfo(record: *const bi_record) -> c_int;
    pub fn virt_parse_bootinfo(record: *const bi_record) -> c_int;

    pub fn config_amiga();
    pub fn config_apollo();
    pub fn config_atari();
    pub fn config_bvme6000();
    pub fn config_hp300();
    pub fn config_mac();
    pub fn config_mvme147();
    pub fn config_mvme16x();
    pub fn config_q40();
    pub fn config_sun3();
    pub fn config_sun3x();
    pub fn config_virt();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
