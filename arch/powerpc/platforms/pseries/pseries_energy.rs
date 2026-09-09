// SPDX-License-Identifier: GPL-2.0-only
/*
 * POWER platform energy management driver
 * Copyright (C) 2010 IBM Corporation
 *
 * This pseries platform device driver provides access to
 * platform energy management capabilities.
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_ulong, c_void};

const MODULE_VERS: &str = "1.0";
const MODULE_NAME: &str = "pseries_energy";
const FLAGS_MODE1: c_ulong = 0x004E200000080E01;
const FLAGS_MODE2: c_ulong = 0x004E200000080401;
const FLAGS_ACTIVATE: c_ulong = 0x100;

static mut SYSFS_ENTRIES: c_int = 0;

#[repr(C)]
pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)]
pub struct Property { _private: [u8; 0] }
#[repr(C)]
pub struct Device { pub id: c_int, pub kobj: Kobject }
#[repr(C)]
pub struct Kobject { _private: [u8; 0] }
#[repr(C)]
pub struct DeviceAttribute { pub attr: Attribute }
#[repr(C)]
pub struct Attribute { _private: [u8; 0] }
#[repr(C)]
pub struct Subsystem { _private: [u8; 0] }
#[repr(C)]
pub struct OfDrcInfo {
    pub drc_type: [c_char; 8], pub drc_index_start: u32,
    pub drc_name: u32, pub drc_type_len: u32, pub drc_num_sequential_elems: u32,
    pub sequential_inc: u32, pub num_sequential_elems: u32, pub last_drc_index: u32,
}

extern "C" {
    static mut cpu_subsys: Subsystem;
    fn of_find_node_by_path(path: *const c_char) -> *mut DeviceNode;
    fn of_node_put(node: *mut DeviceNode);
    fn of_find_property(node: *mut DeviceNode, name: *const c_char, len: *mut c_int) -> *mut Property;
    fn of_prop_next_u32(prop: *mut Property, prev: *const u32, length: *mut u32) -> *const u32;
    fn of_read_drc_info_cell(prop: *mut *mut Property, value: *mut *const u32, info: *mut OfDrcInfo);
    fn of_property_read_u32_index(node: *mut DeviceNode, name: *const c_char, index: c_int, out: *mut u32) -> c_int;
    fn of_get_property(node: *mut DeviceNode, name: *const c_char, len: *mut c_int) -> *const c_int;
    fn cpu_core_index_of_thread(cpu: c_int) -> c_int;
    fn cpu_first_thread_of_core(cpu: c_int) -> c_int;
    fn plpar_hcall9(h: c_ulong, retbuf: *mut c_ulong, flags: c_ulong, a3: c_ulong, a4: c_ulong, a5: c_ulong, a6: c_ulong, a7: c_ulong, a8: c_ulong, a9: c_ulong, a10: c_ulong) -> c_int;
    fn get_zeroed_page(flags: c_ulong) -> *mut c_void;
    fn free_page(addr: c_ulong);
    fn __pa(addr: *mut c_void) -> c_ulong;
    fn cpu_online(cpu: c_int) -> bool;
    fn sysfs_emit(page: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn sprintf(dst: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn firmware_has_feature(feature: c_ulong) -> bool;
    fn bus_get_dev_root(subsys: *mut Subsystem) -> *mut Device;
    fn put_device(dev: *mut Device);
    fn device_create_file(dev: *mut Device, attr: *mut DeviceAttribute) -> c_int;
    fn device_remove_file(dev: *mut Device, attr: *mut DeviceAttribute);
    fn get_cpu_device(cpu: c_int) -> *mut Device;
    fn sysfs_remove_file(kobj: *mut Kobject, attr: *mut Attribute);
}

unsafe fn cpu_to_drc_index(cpu: c_int) -> u32 {
    let mut dn = of_find_node_by_path(b"/cpus\0".as_ptr() as *const c_char);
    let mut rc = 1; let mut ret = 0; if dn.is_null() { return ret; }
    let thread_index = cpu_core_index_of_thread(cpu);
    let mut info = of_find_property(dn, b"ibm,drc-info\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if !info.is_null() {
        let mut n = 0; let mut value = of_prop_next_u32(info, core::ptr::null(), &mut n); if value.is_null() { of_node_put(dn); return ret; } value = value.add(1);
        let mut drc = core::mem::zeroed::<OfDrcInfo>();
        for _ in 0..n { of_read_drc_info_cell(&mut info, &mut value, &mut drc); if drc.drc_type[0] != b'C' as c_char || drc.drc_type[1] != b'P' as c_char || drc.drc_type[2] != b'U' as c_char { of_node_put(dn); return ret; } if thread_index < drc.last_drc_index { break; } }
        ret = drc.drc_index_start + (thread_index as u32) * drc.sequential_inc;
    } else { let mut nr = 0; if of_property_read_u32_index(dn, b"ibm,drc-indexes\0".as_ptr() as *const c_char, 0, &mut nr) != 0 { of_node_put(dn); return ret; } let mut index = 0; if of_property_read_u32_index(dn, b"ibm,drc-indexes\0".as_ptr() as *const c_char, thread_index + 1, &mut index) != 0 { of_node_put(dn); return ret; } ret = index; }
    rc = 0; of_node_put(dn); let _ = rc; ret
}

unsafe fn drc_index_to_cpu(drc_index: u32) -> c_int {
    let dn = of_find_node_by_path(b"/cpus\0".as_ptr() as *const c_char); if dn.is_null() { return 0; }
    let info = of_find_property(dn, b"ibm,drc-info\0".as_ptr() as *const c_char, core::ptr::null_mut()); let mut cpu = 0;
    if !info.is_null() { let mut n=0; let mut value=of_prop_next_u32(info,core::ptr::null(),&mut n); if !value.is_null(){value=value.add(1); let mut drc=core::mem::zeroed::<OfDrcInfo>(); for _ in 0..n { of_read_drc_info_cell(&mut (info as *mut Property),&mut value,&mut drc); if drc.drc_index_start <= drc_index && drc_index <= drc.last_drc_index { cpu += ((drc_index-drc.drc_index_start)/drc.sequential_inc) as c_int; break; } cpu += drc.num_sequential_elems as c_int; } } } else { let indexes=of_get_property(dn,b"ibm,drc-indexes\0".as_ptr() as *const c_char,core::ptr::null_mut()); if !indexes.is_null(){ let mut i=0; while i<*indexes as usize && *indexes.add(i+1) as u32 != drc_index {i+=1;} cpu=i as c_int; } }
    of_node_put(dn); cpu_first_thread_of_core(cpu)
}

unsafe fn get_best_energy_list(page: *mut c_char, activate: c_int) -> isize {
    let mut retbuf = [0 as c_ulong; 9]; let buf = get_zeroed_page(0); if buf.is_null() { return -12; }
    let mut flags = FLAGS_MODE1; if activate != 0 { flags |= FLAGS_ACTIVATE; }
    if plpar_hcall9(0, retbuf.as_mut_ptr(), flags, 0, __pa(buf), 0,0,0,0,0,0) != 0 { free_page(buf as c_ulong); return -22; }
    let mut s = page; let count = retbuf[0] as usize;
    for i in 0..count { let cpu=drc_index_to_cpu(*(buf as *const u32).add(2*i+1)); if (cpu_online(cpu)&&activate==0)||(!cpu_online(cpu)&&activate!=0) { s=s.add(sprintf(s,b"%d,\0".as_ptr() as *const c_char) as usize); } }
    if s != page { s=s.sub(1); s=s.add(sprintf(s,b"\n\0".as_ptr() as *const c_char) as usize); } free_page(buf as c_ulong); s.offset_from(page)
}

unsafe fn get_best_energy_data(dev:*mut Device,page:*mut c_char,activate:c_int)->isize { let mut retbuf=[0 as c_ulong;9]; let mut flags=FLAGS_MODE2; if activate!=0 {flags|=FLAGS_ACTIVATE;} if plpar_hcall9(0,retbuf.as_mut_ptr(),flags,cpu_to_drc_index((*dev).id),0,0,0,0,0,0)!=0{return -22;} sysfs_emit(page,b"%lu\n\0".as_ptr() as *const c_char,retbuf[1]>>32) }
unsafe fn cpu_activate_hint_list_show(_: *mut Device,_:*mut DeviceAttribute,p:*mut c_char)->isize{get_best_energy_list(p,1)}
unsafe fn cpu_deactivate_hint_list_show(_: *mut Device,_:*mut DeviceAttribute,p:*mut c_char)->isize{get_best_energy_list(p,0)}
unsafe fn percpu_activate_hint_show(d:*mut Device,_:*mut DeviceAttribute,p:*mut c_char)->isize{get_best_energy_data(d,p,1)}
unsafe fn percpu_deactivate_hint_show(d:*mut Device,_:*mut DeviceAttribute,p:*mut c_char)->isize{get_best_energy_data(d,p,0)}

static mut ATTR_CPU_ACTIVATE_HINT_LIST: DeviceAttribute=DeviceAttribute{attr:Attribute{_private:[]}};
static mut ATTR_CPU_DEACTIVATE_HINT_LIST: DeviceAttribute=DeviceAttribute{attr:Attribute{_private:[]}};
static mut ATTR_PERCPU_ACTIVATE_HINT: DeviceAttribute=DeviceAttribute{attr:Attribute{_private:[]}};
static mut ATTR_PERCPU_DEACTIVATE_HINT: DeviceAttribute=DeviceAttribute{attr:Attribute{_private:[]}};

unsafe fn pseries_energy_init() -> c_int { SYSFS_ENTRIES=1; 0 }
unsafe fn pseries_energy_cleanup() { if SYSFS_ENTRIES==0{return;} SYSFS_ENTRIES=0; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
