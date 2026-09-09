/* SPDX-License-Identifier: GPL-2.0 */
/*
 * SGI PCI Xtalk Bridge
 */

// Original dependency: <asm/sn/types.h>

#[repr(C)]
pub struct xtalk_bridge_platform_data {
    pub mem: resource,
    pub io: resource,
    pub bridge_addr: libc::c_ulong,
    pub intr_addr: libc::c_ulong,
    pub mem_offset: libc::c_ulong,
    pub io_offset: libc::c_ulong,
    pub nasid: nasid_t,
    pub masterwid: libc::c_int,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
