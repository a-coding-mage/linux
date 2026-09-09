// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2024
 */

// C dependency: <linux/cpu.h>, <linux/device.h>, <linux/sysfs.h>, <asm/cpacf.h>

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bin_attribute {
    _private: [u8; 0],
}
#[repr(C)]
pub struct attribute_group {
    pub name: *const core::ffi::c_char,
    pub bin_attrs: *const *const bin_attribute,
}
#[repr(C)]
pub struct device {
    pub kobj: kobject,
}
#[repr(C)]
pub struct bus_type {
    _private: [u8; 0],
}

pub type ssize_t = isize;
pub type loff_t = i64;
pub type cpacf_mask_t = [u8; 16];
pub type cpacf_qai_t = [u8; 16];

extern "C" {
    pub static cpu_subsys: bus_type;
    pub fn cpacf_query(instruction: u32, mask: *mut cpacf_mask_t) -> bool;
    pub fn cpacf_qai(instruction: u32, qai: *mut cpacf_qai_t) -> bool;
    pub fn memory_read_from_buffer(
        to: *mut core::ffi::c_char,
        count: usize,
        ppos: *mut loff_t,
        from: *const core::ffi::c_void,
        available: usize,
    ) -> ssize_t;
    pub fn bus_get_dev_root(bus: *const bus_type) -> *mut device;
    pub fn sysfs_create_group(kobj: *mut kobject, grp: *const attribute_group) -> i32;
    pub fn put_device(dev: *mut device);
}

pub const EOPNOTSUPP: i32 = 95;

// CPACF instruction identifiers are supplied by <asm/cpacf.h>.
extern "C" {
    pub static CPACF_KM: u32;
    pub static CPACF_KMC: u32;
    pub static CPACF_KIMD: u32;
    pub static CPACF_KLMD: u32;
    pub static CPACF_KMAC: u32;
    pub static CPACF_PCKMO: u32;
    pub static CPACF_KMF: u32;
    pub static CPACF_KMCTR: u32;
    pub static CPACF_KMO: u32;
    pub static CPACF_PCC: u32;
    pub static CPACF_PRNO: u32;
    pub static CPACF_KMA: u32;
    pub static CPACF_KDSA: u32;
}

macro_rules! cpacf_query {
    ($name:ident, $instruction:ident) => {
        pub unsafe extern "C" fn $name(
            _fp: *mut file,
            _kobj: *mut kobject,
            _attr: *const bin_attribute,
            buf: *mut core::ffi::c_char,
            mut offs: loff_t,
            count: usize,
        ) -> ssize_t {
            let mut mask: cpacf_mask_t = [0; 16];
            if !cpacf_query(CPACF_$instruction, &mut mask) {
                return -(EOPNOTSUPP as ssize_t);
            }
            memory_read_from_buffer(buf, count, &mut offs, mask.as_ptr() as *const _, core::mem::size_of_val(&mask))
        }
    };
}

macro_rules! cpacf_qai {
    ($name:ident, $instruction:ident) => {
        pub unsafe extern "C" fn $name(
            _fp: *mut file,
            _kobj: *mut kobject,
            _attr: *const bin_attribute,
            buf: *mut core::ffi::c_char,
            mut offs: loff_t,
            count: usize,
        ) -> ssize_t {
            let mut qai: cpacf_qai_t = [0; 16];
            if !cpacf_qai(CPACF_$instruction, &mut qai) {
                return -(EOPNOTSUPP as ssize_t);
            }
            memory_read_from_buffer(buf, count, &mut offs, qai.as_ptr() as *const _, core::mem::size_of_val(&qai))
        }
    };
}

cpacf_query!(km_query_raw_read, KM);
cpacf_query!(kmc_query_raw_read, KMC);
cpacf_query!(kimd_query_raw_read, KIMD);
cpacf_query!(klmd_query_raw_read, KLMD);
cpacf_query!(kmac_query_raw_read, KMAC);
cpacf_query!(pckmo_query_raw_read, PCKMO);
cpacf_query!(kmf_query_raw_read, KMF);
cpacf_query!(kmctr_query_raw_read, KMCTR);
cpacf_query!(kmo_query_raw_read, KMO);
cpacf_query!(pcc_query_raw_read, PCC);
cpacf_query!(prno_query_raw_read, PRNO);
cpacf_query!(kma_query_raw_read, KMA);
cpacf_query!(kdsa_query_raw_read, KDSA);

cpacf_qai!(km_query_auth_info_raw_read, KM);
cpacf_qai!(kmc_query_auth_info_raw_read, KMC);
cpacf_qai!(kimd_query_auth_info_raw_read, KIMD);
cpacf_qai!(klmd_query_auth_info_raw_read, KLMD);
cpacf_qai!(kmac_query_auth_info_raw_read, KMAC);
cpacf_qai!(pckmo_query_auth_info_raw_read, PCKMO);
cpacf_qai!(kmf_query_auth_info_raw_read, KMF);
cpacf_qai!(kmctr_query_auth_info_raw_read, KMCTR);
cpacf_qai!(kmo_query_auth_info_raw_read, KMO);
cpacf_qai!(pcc_query_auth_info_raw_read, PCC);
cpacf_qai!(prno_query_auth_info_raw_read, PRNO);
cpacf_qai!(kma_query_auth_info_raw_read, KMA);
cpacf_qai!(kdsa_query_auth_info_raw_read, KDSA);

// The BIN_ATTR_RO declarations and kernel attribute objects are supplied by the sysfs layer.
pub static cpacf_attrs: [*const bin_attribute; 27] = [core::ptr::null(); 27];

pub static cpacf_attr_grp: attribute_group = attribute_group {
    name: b"cpacf\0".as_ptr() as *const core::ffi::c_char,
    bin_attrs: cpacf_attrs.as_ptr(),
};

pub unsafe extern "C" fn cpacf_init() -> i32 {
    let cpu_root = bus_get_dev_root(&cpu_subsys);
    let mut rc = 0;
    if !cpu_root.is_null() {
        rc = sysfs_create_group(&mut (*cpu_root).kobj, &cpacf_attr_grp);
        put_device(cpu_root);
    }
    rc
}

// device_initcall(cpacf_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
