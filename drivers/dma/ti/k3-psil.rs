// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 2019 Texas Instruments Incorporated - http://www.ti.com
 *  Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// Linux kernel dependencies and `k3-psil-priv.h` are supplied by other files.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut ep_map_mutex: c_void;
    static mut soc_ep_map: *const psil_ep_map;

    static am654_ep_map: psil_ep_map;
    static j721e_ep_map: psil_ep_map;
    static j7200_ep_map: psil_ep_map;
    static am64_ep_map: psil_ep_map;
    static j721s2_ep_map: psil_ep_map;
    static am62_ep_map: psil_ep_map;
    static am62a_ep_map: psil_ep_map;
    static j784s4_ep_map: psil_ep_map;
    static am62p_ep_map: psil_ep_map;

    fn mutex_lock(mutex: *mut c_void);
    fn mutex_unlock(mutex: *mut c_void);
    fn soc_device_match(matches: *const soc_device_attribute) -> *const soc_device_attribute;
    fn of_property_match_string(node: *mut device_node, propname: *const c_char, string: *const c_char) -> c_int;
    fn of_parse_phandle_with_args(
        node: *mut device_node,
        phandle_name: *const c_char,
        cells_name: *const c_char,
        index: c_int,
        out_args: *mut of_phandle_args,
    ) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node;

#[repr(C)]
pub struct of_phandle_args {
    pub np: *mut device_node,
    pub args_count: u32,
    pub args: [u32; 16],
}

#[repr(C)]
pub struct psil_endpoint_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct psil_ep_map_entry {
    pub thread_id: u32,
    pub ep_config: psil_endpoint_config,
}

#[repr(C)]
pub struct psil_ep_map {
    pub name: *const c_char,
    pub src: *const psil_ep_map_entry,
    pub src_count: c_int,
    pub dst: *const psil_ep_map_entry,
    pub dst_count: c_int,
}

#[repr(C)]
pub struct soc_device_attribute {
    pub family: *const c_char,
    pub data: *const c_void,
}

const K3_PSIL_DST_THREAD_ID_OFFSET: u32 = 0x8000_0000;
const ENOTSUPP: isize = 524;
const ENOENT: isize = 2;
const EINVAL: isize = 22;

static K3_SOC_DEVICES: &[soc_device_attribute] = &[
    soc_device_attribute { family: b"AM65X\0".as_ptr() as *const c_char, data: unsafe { &am654_ep_map as *const _ as *const c_void } },
    soc_device_attribute { family: b"J721E\0".as_ptr() as *const c_char, data: unsafe { &j721e_ep_map as *const _ as *const c_void } },
    soc_device_attribute { family: b"J7200\0".as_ptr() as *const c_char, data: unsafe { &j7200_ep_map as *const _ as *const c_void } },
    soc_device_attribute { family: b"AM64X\0".as_ptr() as *const c_char, data: unsafe { &am64_ep_map as *const _ as *const c_void } },
    soc_device_attribute { family: b"J721S2\0".as_ptr() as *const c_char, data: unsafe { &j721s2_ep_map as *const _ as *const c_void } },
    soc_device_attribute { family: b"AM62X\0".as_ptr() as *const c_char, data: unsafe { &am62_ep_map as *const _ as *const c_void } },
    soc_device_attribute { family: b"AM62AX\0".as_ptr() as *const c_char, data: unsafe { &am62a_ep_map as *const _ as *const c_void } },
    soc_device_attribute { family: b"J784S4\0".as_ptr() as *const c_char, data: unsafe { &j784s4_ep_map as *const _ as *const c_void } },
    soc_device_attribute { family: b"AM62PX\0".as_ptr() as *const c_char, data: unsafe { &am62p_ep_map as *const _ as *const c_void } },
    soc_device_attribute { family: b"J722S\0".as_ptr() as *const c_char, data: unsafe { &am62p_ep_map as *const _ as *const c_void } },
    soc_device_attribute { family: core::ptr::null(), data: core::ptr::null() },
];

pub unsafe fn psil_get_ep_config(thread_id: u32) -> *mut psil_endpoint_config {
    mutex_lock(&mut ep_map_mutex);
    if soc_ep_map.is_null() {
        let soc = soc_device_match(K3_SOC_DEVICES.as_ptr());
        if soc.is_null() {
            mutex_unlock(&mut ep_map_mutex);
            return (-ENOTSUPP) as *mut psil_endpoint_config;
        }
        soc_ep_map = (*soc).data as *const psil_ep_map;
    }
    mutex_unlock(&mut ep_map_mutex);

    let map = &*soc_ep_map;
    if thread_id & K3_PSIL_DST_THREAD_ID_OFFSET != 0 && !map.dst.is_null() {
        for i in 0..map.dst_count {
            let entry = &*map.dst.add(i as usize);
            if entry.thread_id == thread_id {
                return &entry.ep_config as *const _ as *mut _;
            }
        }
    }
    let thread_id = thread_id & !K3_PSIL_DST_THREAD_ID_OFFSET;
    if !map.src.is_null() {
        for i in 0..map.src_count {
            let entry = &*map.src.add(i as usize);
            if entry.thread_id == thread_id {
                return &entry.ep_config as *const _ as *mut _;
            }
        }
    }
    (-ENOENT) as *mut psil_endpoint_config
}

pub unsafe fn psil_set_new_ep_config(
    dev: *mut device,
    name: *const c_char,
    ep_config: *mut psil_endpoint_config,
) -> c_int {
    if dev.is_null() || (*dev).of_node.is_null() {
        return -EINVAL as c_int;
    }
    let index = of_property_match_string((*dev).of_node, b"dma-names\0".as_ptr() as *const c_char, name);
    if index < 0 {
        return index;
    }
    let mut dma_spec = of_phandle_args { np: core::ptr::null_mut(), args_count: 0, args: [0; 16] };
    if of_parse_phandle_with_args((*dev).of_node, b"dmas\0".as_ptr() as *const c_char, b"#dma-cells\0".as_ptr() as *const c_char, index, &mut dma_spec) != 0 {
        return -ENOENT as c_int;
    }
    let dst_ep_config = psil_get_ep_config(dma_spec.args[0]);
    if (dst_ep_config as isize) < 0 {
        of_node_put(dma_spec.np);
        return dst_ep_config as c_int;
    }
    memcpy(dst_ep_config as *mut c_void, ep_config as *const c_void, core::mem::size_of::<psil_endpoint_config>());
    of_node_put(dma_spec.np);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
