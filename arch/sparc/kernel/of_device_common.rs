// SPDX-License-Identifier: GPL-2.0-only
// Translated from of_device_common.c. Linux headers provide the concrete
// definitions and external declarations used below.

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub const EINVAL: c_int = 22;
pub const IORESOURCE_MEM: c_ulong = 0x0000_0200;
pub const OF_MAX_ADDR_CELLS: usize = 4;

#[repr(C)]
pub struct device_node {
    pub child: *mut device_node,
    pub sibling: *mut device_node,
    pub parent: *mut device_node,
}

#[repr(C)]
pub struct resource {
    pub name: *const c_char,
    pub _opaque: [u8; 40],
}

#[repr(C)]
pub struct dev_archdata {
    pub iommu: *mut c_void,
    pub stc: *mut c_void,
    pub host_controller: *mut c_void,
    pub numa_node: c_int,
}

#[repr(C)]
pub struct device {
    pub archdata: dev_archdata,
    pub of_node: *mut device_node,
    pub dma_ops: *mut c_void,
}

#[repr(C)]
pub struct platform_archdata {
    pub num_irqs: c_int,
    pub irqs: *mut u32,
    pub resource: *mut resource,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub archdata: platform_archdata,
    pub num_resources: c_int,
}

unsafe extern "C" {
    fn of_find_device_by_node(node: *mut device_node) -> *mut platform_device;
    fn of_ioremap(r: *mut resource, offset: u64, size: u64, name: *mut c_char) -> *mut c_void;
    fn resource_size(r: *const resource) -> u64;
    fn of_n_addr_cells(dp: *mut device_node) -> c_int;
    fn of_n_size_cells(dp: *mut device_node) -> c_int;
    fn of_read_addr(addr: *const u32, cells: c_int) -> u64;
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn of_node_name_eq(np: *mut device_node, name: *const c_char) -> bool;
    fn of_property_present(np: *mut device_node, name: *const c_char) -> bool;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn irq_of_parse_and_map(node: *mut device_node, index: c_int) -> u32 {
    let op = unsafe { of_find_device_by_node(node) };
    if op.is_null() || index >= unsafe { (*op).archdata.num_irqs } {
        return 0;
    }
    unsafe { *(*op).archdata.irqs.add(index as usize) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_address_to_resource(
    node: *mut device_node,
    index: c_int,
    r: *mut resource,
) -> c_int {
    let op = unsafe { of_find_device_by_node(node) };
    if op.is_null() || index >= unsafe { (*op).num_resources } {
        return -EINVAL;
    }
    unsafe {
        core::ptr::copy_nonoverlapping((*op).archdata.resource.add(index as usize), r, 1);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_iomap(node: *mut device_node, index: c_int) -> *mut c_void {
    let op = unsafe { of_find_device_by_node(node) };
    if op.is_null() || index >= unsafe { (*op).num_resources } {
        return core::ptr::null_mut();
    }
    let r = unsafe { (*op).archdata.resource.add(index as usize) };
    unsafe { of_ioremap(r, 0, resource_size(r), (*r).name as *mut c_char) }
}

/* Take the archdata values for IOMMU, STC, and HOSTDATA found in
 * BUS and propagate to all child platform_device objects.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn of_propagate_archdata(bus: *mut platform_device) {
    let bus_sd = unsafe { &(*bus).dev.archdata };
    let bus_dp = unsafe { (*bus).dev.of_node };
    let mut dp = unsafe { (*bus_dp).child };

    while !dp.is_null() {
        let op = unsafe { of_find_device_by_node(dp) };
        unsafe {
            (*op).dev.archdata.iommu = bus_sd.iommu;
            (*op).dev.archdata.stc = bus_sd.stc;
            (*op).dev.archdata.host_controller = bus_sd.host_controller;
            (*op).dev.archdata.numa_node = bus_sd.numa_node;
            (*op).dev.dma_ops = (*bus).dev.dma_ops;
            if !(*dp).child.is_null() {
                of_propagate_archdata(op);
            }
            dp = (*dp).sibling;
        }
    }
}

unsafe fn get_cells(dp: *mut device_node, addrc: *mut c_int, sizec: *mut c_int) {
    if !addrc.is_null() {
        unsafe { *addrc = of_n_addr_cells(dp) };
    }
    if !sizec.is_null() {
        unsafe { *sizec = of_n_size_cells(dp) };
    }
}

pub unsafe extern "C" fn of_bus_default_count_cells(
    dev: *mut device_node,
    addrc: *mut c_int,
    sizec: *mut c_int,
) {
    unsafe { get_cells(dev, addrc, sizec) };
}

pub unsafe extern "C" fn of_out_of_range(
    addr: *const u32,
    base: *const u32,
    size: *const u32,
    na: c_int,
    ns: c_int,
) -> c_int {
    let a = unsafe { of_read_addr(addr, na) };
    let mut b = unsafe { of_read_addr(base, na) };
    if a < b { return 1; }
    b = b.wrapping_add(unsafe { of_read_addr(size, ns) });
    if a >= b { return 1; }
    0
}

pub unsafe extern "C" fn of_bus_default_map(
    addr: *mut u32, range: *const u32, na: c_int, ns: c_int, pna: c_int,
) -> c_int {
    let mut result = [0u32; OF_MAX_ADDR_CELLS];
    if ns > 2 {
        unsafe { printk(c"of_device: Cannot handle size cells (%d) > 2.".as_ptr(), ns) };
        return -EINVAL;
    }
    if unsafe { of_out_of_range(addr, range, range.add((na + pna) as usize), na, ns) } != 0 {
        return -EINVAL;
    }
    unsafe { core::ptr::copy_nonoverlapping(range.add(na as usize), result.as_mut_ptr(), pna as usize); }
    for i in 0..na as usize {
        let j = pna as usize - 1 - i;
        unsafe { *result.as_mut_ptr().add(j) = (*result.as_ptr().add(j)).wrapping_add((*addr.add(na as usize - 1 - i)).wrapping_sub(*range.add(na as usize - 1 - i))); }
    }
    unsafe { core::ptr::copy_nonoverlapping(result.as_ptr(), addr, pna as usize); }
    0
}

pub unsafe extern "C" fn of_bus_default_get_flags(_addr: *const u32, flags: c_ulong) -> c_ulong {
    if flags != 0 { flags } else { IORESOURCE_MEM }
}

pub unsafe extern "C" fn of_bus_sbus_match(np: *mut device_node) -> c_int {
    let mut dp = np;
    while !dp.is_null() {
        if unsafe { of_node_name_eq(dp, c"sbus".as_ptr()) || of_node_name_eq(dp, c"sbi".as_ptr()) } { return 1; }
        if unsafe { of_property_present(dp, c"ranges".as_ptr()) } { break; }
        dp = unsafe { (*dp).parent };
    }
    0
}

pub unsafe extern "C" fn of_bus_sbus_count_cells(_child: *mut device_node, addrc: *mut c_int, sizec: *mut c_int) {
    if !addrc.is_null() { unsafe { *addrc = 2; } }
    if !sizec.is_null() { unsafe { *sizec = 1; } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
