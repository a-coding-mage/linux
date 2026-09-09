/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of s390/include/asm/sysinfo.h. */

use core::ffi::c_void;

/* External kernel-provided types and functions are supplied by other units. */
extern "C" {
    pub static mut topology_max_mnest: core::ffi::c_int;
    pub fn register_service_level(level: *mut service_level) -> core::ffi::c_int;
    pub fn unregister_service_level(level: *mut service_level) -> core::ffi::c_int;
    pub fn sthyi_fill(dst: *mut c_void, rc: *mut u64) -> core::ffi::c_int;
}

/* stsi is implemented by the s390 inline assembly in the original header. */
#[inline]
pub unsafe fn stsi(sysinfo: *mut c_void, fc: core::ffi::c_int,
                   sel1: core::ffi::c_int, sel2: core::ffi::c_int) -> core::ffi::c_int {
    let _ = (sysinfo, fc, sel1, sel2);
    /* The CC_IPM/CC_OUT/CC_CLOBBER_LIST kernel assembly interface is external. */
    unimplemented!("s390 stsi inline assembly")
}

#[repr(C)]
pub struct sysinfo_1_1_1 {
    pub flags0: u8, /* p:1, reserved:6, t:1 */
    pub reserved0: u8,
    pub ccr: u8,
    pub cai: u8,
    pub reserved_0: [i8; 20],
    pub lic: core::ffi::c_ulong,
    pub manufacturer: [i8; 16],
    pub type_: [i8; 4],
    pub reserved_1: [i8; 12],
    pub model_capacity: [i8; 16],
    pub sequence: [i8; 16],
    pub plant: [i8; 4],
    pub model: [i8; 16],
    pub model_perm_cap: [i8; 16],
    pub model_temp_cap: [i8; 16],
    pub model_cap_rating: u32,
    pub model_perm_cap_rating: u32,
    pub model_temp_cap_rating: u32,
    pub typepct: [u8; 5],
    pub reserved_2: [u8; 3],
    pub ncr: u32,
    pub npr: u32,
    pub ntr: u32,
    pub reserved_3: [i8; 4],
    pub model_var_cap: [i8; 16],
    pub model_var_cap_rating: u32,
    pub nvr: u32,
}

#[repr(C)]
pub struct sysinfo_1_2_1 { pub reserved_0: [i8; 80], pub sequence: [i8; 16], pub plant: [i8; 4], pub reserved_1: [i8; 2], pub cpu_address: u16 }

#[repr(C)]
pub struct sysinfo_1_2_2 {
    pub format: i8, pub reserved_0: [i8; 1], pub acc_offset: u16,
    pub mt_flags0: u8, /* mt_installed:1, reserved:2, mt_stid:5 */
    pub mt_gtid: u8, /* reserved:3, mt_gtid:5 */
    pub reserved_1: [i8; 18], pub nominal_cap: u32, pub secondary_cap: u32,
    pub capability: u32, pub cpus_total: u16, pub cpus_configured: u16,
    pub cpus_standby: u16, pub cpus_reserved: u16, pub adjustment: [u16; 0],
}
#[repr(C)] pub struct sysinfo_1_2_2_extension { pub alt_capability: u32, pub alt_adjustment: [u16; 0] }

#[repr(C)] pub struct sysinfo_2_2_1 { pub reserved_0: [i8; 80], pub sequence: [i8; 16], pub plant: [i8; 4], pub cpu_id: u16, pub cpu_address: u16 }

#[repr(C)]
pub struct sysinfo_2_2_2 {
    pub reserved_0: [i8; 32], pub lpar_number: u16, pub reserved_1: i8,
    pub characteristics: u8, pub cpus_total: u16, pub cpus_configured: u16,
    pub cpus_standby: u16, pub cpus_reserved: u16, pub name: [i8; 8], pub caf: u32,
    pub reserved_2: [i8; 8], pub mt_flags0: u8, pub mt_gtid_flags: u8,
    pub mt_psmtid_flags: u8, pub reserved_3: [i8; 5], pub cpus_dedicated: u16,
    pub cpus_shared: u16, pub reserved_4: [i8; 3], pub vsne: u8, pub uuid: uuid_t,
    pub reserved_5: [i8; 160], pub ext_name: [i8; 256],
}

pub const LPAR_CHAR_DEDICATED: u8 = 1 << 7;
pub const LPAR_CHAR_SHARED: u8 = 1 << 6;
pub const LPAR_CHAR_LIMITED: u8 = 1 << 5;

#[repr(C)]
pub struct sysinfo_3_2_2 {
    pub reserved_0: [i8; 31], pub count_flags: u8,
    pub vm: [sysinfo_3_2_2_vm; 8], pub reserved_3: [i8; 1504], pub ext_names: [[i8; 256]; 8],
}
#[repr(C)] pub struct sysinfo_3_2_2_vm {
    pub reserved_0: [i8; 4], pub cpus_total: u16, pub cpus_configured: u16, pub cpus_standby: u16,
    pub cpus_reserved: u16, pub name: [i8; 8], pub caf: u32, pub cpi: [i8; 16],
    pub reserved_1: [i8; 3], pub evmne: u8, pub reserved_2: u32, pub uuid: uuid_t,
}

#[inline] pub unsafe fn topology_mnest_limit() -> u8 { core::cmp::min(topology_max_mnest as u8, 4) }
pub const TOPOLOGY_NR_MAG: usize = 6;
#[repr(C)] pub struct topology_core { pub nl: u8, pub reserved0: [u8; 3], pub flags: u8, pub reserved1: u8, pub origin: u16, pub mask: core::ffi::c_ulong }
#[repr(C)] pub struct topology_container { pub nl: u8, pub reserved: [u8; 6], pub id: u8 }
#[repr(C)] pub union topology_entry { pub nl: u8, pub cpu: topology_core, pub container: topology_container }
#[repr(C)] pub struct sysinfo_15_1_x { pub reserved0: [u8; 2], pub length: u16, pub mag: [u8; TOPOLOGY_NR_MAG], pub reserved1: u8, pub mnest: u8, pub reserved2: [u8; 4], pub tle: [topology_entry; 0] }

#[repr(C)] pub struct service_level { pub list: list_head, pub seq_print: Option<unsafe extern "C" fn(*mut seq_file, *mut service_level)> }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
