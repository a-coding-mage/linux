// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 * Copyright (C) 2008-2009 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

use core::ffi::{c_char, c_void};

// Linux kernel headers supply these declarations and constants.

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct Resource {
    pub start: usize,
    pub name: *const c_char,
}

#[repr(C)]
pub struct DeviceNode {
    pub name: *const c_char,
}

#[repr(C)]
pub struct SocInfo {
    pub compatible: *const c_char,
    pub mem_detect: Option<unsafe extern "C" fn()>,
    pub mem_size: usize,
    pub mem_base: usize,
    pub mem_size_min: usize,
    pub mem_size_max: usize,
}

pub static mut rt_sysc_membase: *mut c_void = core::ptr::null_mut();
pub static mut rt_memc_membase: *mut c_void = core::ptr::null_mut();

static MTMIPS_MEMC_MATCH: [OfDeviceId; 6] = [
    OfDeviceId { compatible: b"mediatek,mt7621-memc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,mt7620a-memc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,rt2880-memc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,rt3050-memc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,rt3883-memc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: core::ptr::null() },
];

static MTMIPS_SYSC_MATCH: [OfDeviceId; 11] = [
    OfDeviceId { compatible: b"mediatek,mt7621-sysc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,mt7620-sysc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,mt7628-sysc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,mt7688-sysc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,rt2880-sysc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,rt3050-sysc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,rt3052-sysc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,rt3352-sysc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,rt3883-sysc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: b"ralink,rt5350-sysc\0".as_ptr() as *const c_char },
    OfDeviceId { compatible: core::ptr::null() },
];

extern "C" {
    static mut soc_info: SocInfo;
    fn of_find_matching_node(from: *mut DeviceNode, matches: *const OfDeviceId) -> *mut DeviceNode;
    fn of_address_to_resource(np: *mut DeviceNode, index: i32, res: *mut Resource) -> i32;
    fn request_mem_region(start: usize, size: usize, name: *const c_char) -> *mut c_void;
    fn of_node_put(np: *mut DeviceNode);
    fn ioremap(start: usize, size: usize) -> *mut c_void;
    fn panic(format: *const c_char, ... ) -> !;
    fn set_io_port_base(base: usize);
    fn get_fdt() -> *mut c_void;
    fn __dt_setup_arch(dtb: *mut c_void);
    fn early_init_dt_scan_memory() -> i32;
    fn memblock_add(base: usize, size: usize);
    fn detect_memory_region(base: usize, min: usize, max: usize);
    fn __dt_register_buses(compatible: *const c_char, bus: *const c_char);
}

unsafe fn mtmips_of_remap_node(
    mtch: *const OfDeviceId,
    type_: *const c_char,
) -> *mut c_void {
    let mut res = Resource { start: 0, name: core::ptr::null() };
    let np = of_find_matching_node(core::ptr::null_mut(), mtch);
    if np.is_null() {
        panic(b"Failed to find %s controller node\0".as_ptr() as *const c_char, type_);
    }
    if of_address_to_resource(np, 0, &mut res) != 0 {
        panic(b"Failed to get resource for %s node\0".as_ptr() as *const c_char, (*np).name);
    }
    let size = resource_size(&res);
    if request_mem_region(res.start, size, res.name).is_null() {
        panic(b"Failed to request resources for %s node\0".as_ptr() as *const c_char, (*np).name);
    }
    of_node_put(np);
    ioremap(res.start, size)
}

unsafe fn resource_size(res: *const Resource) -> usize {
    (*res).start
}

pub unsafe fn ralink_of_remap() {
    rt_sysc_membase = mtmips_of_remap_node(MTMIPS_SYSC_MATCH.as_ptr(), b"system\0".as_ptr() as *const c_char);
    rt_memc_membase = mtmips_of_remap_node(MTMIPS_MEMC_MATCH.as_ptr(), b"memory\0".as_ptr() as *const c_char);
    if rt_sysc_membase.is_null() || rt_memc_membase.is_null() {
        panic(b"Failed to remap core resources\0".as_ptr() as *const c_char);
    }
}

pub unsafe fn plat_mem_setup() {
    set_io_port_base(0xa000_0000);
    let dtb = get_fdt();
    __dt_setup_arch(dtb);
    if early_init_dt_scan_memory() != 0 { return; }
    if let Some(mem_detect) = soc_info.mem_detect {
        mem_detect();
    } else if soc_info.mem_size != 0 {
        memblock_add(soc_info.mem_base, soc_info.mem_size.wrapping_mul(1 << 20));
    } else {
        detect_memory_region(
            soc_info.mem_base,
            soc_info.mem_size_min.wrapping_mul(1 << 20),
            soc_info.mem_size_max.wrapping_mul(1 << 20),
        );
    }
}

unsafe fn plat_of_setup() -> i32 {
    __dt_register_buses(soc_info.compatible, b"palmbus\0".as_ptr() as *const c_char);
    0
}

// arch_initcall(plat_of_setup)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
