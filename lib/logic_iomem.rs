// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2021 Intel Corporation
 * Author: Johannes Berg <johannes@sipsolutions.net>
 */

#[repr(C)]
pub struct logic_iomem_region {
    pub res: *const resource,
    pub ops: *const logic_iomem_region_ops,
    pub list: list_head,
}

#[repr(C)]
pub struct logic_iomem_area {
    pub ops: *const logic_iomem_ops,
    pub priv_: *mut core::ffi::c_void,
}

const AREA_SHIFT: usize = 24;
const MAX_AREA_SIZE: usize = 1usize << AREA_SHIFT;
const MAX_AREAS: usize = ((1u64 << 31) as usize) / MAX_AREA_SIZE;
const AREA_BITS: usize = (MAX_AREAS - 1) << AREA_SHIFT;
const AREA_MASK: usize = MAX_AREA_SIZE - 1;

#[cfg(target_pointer_width = "64")]
const IOREMAP_BIAS: usize = 0xDEAD000000000000usize;
#[cfg(target_pointer_width = "64")]
const IOREMAP_MASK: usize = 0xFFFFFFFF00000000usize;
#[cfg(not(target_pointer_width = "64"))]
const IOREMAP_BIAS: usize = 0x80000000usize;
#[cfg(not(target_pointer_width = "64"))]
const IOREMAP_MASK: usize = 0x80000000usize;

static mut regions_mtx: mutex = DEFINE_MUTEX!();
static mut regions_list: list_head = LIST_HEAD!();
static mut mapped_areas: [logic_iomem_area; MAX_AREAS] = [
    logic_iomem_area { ops: core::ptr::null(), priv_: core::ptr::null_mut() };
    MAX_AREAS
];

pub unsafe fn logic_iomem_add_region(
    resource: *mut resource,
    ops: *const logic_iomem_region_ops,
) -> i32 {
    let rreg: *mut logic_iomem_region;
    let mut err: i32;

    if WARN_ON(resource.is_null() || ops.is_null()) {
        return -EINVAL;
    }
    if WARN_ON(((*resource).flags & IORESOURCE_TYPE_BITS) != IORESOURCE_MEM) {
        return -EINVAL;
    }

    rreg = kzalloc_obj!();
    if rreg.is_null() {
        return -ENOMEM;
    }

    err = request_resource(&mut iomem_resource, resource);
    if err != 0 {
        kfree(rreg as *mut core::ffi::c_void);
        return -ENOMEM;
    }

    mutex_lock(&mut regions_mtx);
    (*rreg).res = resource;
    (*rreg).ops = ops;
    list_add_tail(&mut (*rreg).list, &mut regions_list);
    mutex_unlock(&mut regions_mtx);

    0
}

#[cfg(not(CONFIG_INDIRECT_IOMEM_FALLBACK))]
unsafe fn real_ioremap(offset: phys_addr_t, size: usize) -> *mut core::ffi::c_void {
    WARN!(1, "invalid ioremap(0x%llx, 0x%zx)\n", offset as u64, size);
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_INDIRECT_IOMEM_FALLBACK))]
unsafe fn real_iounmap(addr: *mut core::ffi::c_void) {
    WARN!(1, "invalid iounmap for addr 0x%llx\n", addr as usize as u64);
}

pub unsafe fn ioremap(offset: phys_addr_t, size: usize) -> *mut core::ffi::c_void {
    let mut ret: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut found: *mut logic_iomem_region = core::ptr::null_mut();

    mutex_lock(&mut regions_mtx);
    list_for_each_entry!(rreg, &regions_list, list, logic_iomem_region) {
        if (*(*rreg).res).start > offset || (*(*rreg).res).end < offset + size - 1 {
            continue;
        }
        found = rreg;
        break;
    }

    if !found.is_null() {
        for i in 0..MAX_AREAS {
            let area = &mut mapped_areas[i];
            if !area.ops.is_null() {
                continue;
            }
            let offs = ((*(*found).ops).map)(
                offset - (*(*found).res).start,
                size,
                &mut area.ops,
                &mut area.priv_,
            );
            if offs < 0 {
                area.ops = core::ptr::null();
                break;
            }
            if WARN_ON(area.ops.is_null()) {
                area.ops = core::ptr::null();
                break;
            }
            ret = (IOREMAP_BIAS + (i << AREA_SHIFT) + offs as usize) as *mut core::ffi::c_void;
            break;
        }
    }
    mutex_unlock(&mut regions_mtx);
    if !ret.is_null() { ret } else { real_ioremap(offset, size) }
}

unsafe fn get_area(addr: *const core::ffi::c_void) -> *mut logic_iomem_area {
    let a = addr as usize;
    if WARN_ON((a & IOREMAP_MASK) != IOREMAP_BIAS) { return core::ptr::null_mut(); }
    let idx = (a & AREA_BITS) >> AREA_SHIFT;
    if !mapped_areas[idx].ops.is_null() { &mut mapped_areas[idx] } else { core::ptr::null_mut() }
}

pub unsafe fn iounmap(addr: *mut core::ffi::c_void) {
    let area = get_area(addr);
    if area.is_null() { real_iounmap(addr); return; }
    if !(*(*area).ops).unmap.is_none() { ((*(*area).ops).unmap.unwrap())((*area).priv_); }
    mutex_lock(&mut regions_mtx);
    (*area).ops = core::ptr::null();
    (*area).priv_ = core::ptr::null_mut();
    mutex_unlock(&mut regions_mtx);
}

#[cfg(not(CONFIG_INDIRECT_IOMEM_FALLBACK))]
macro_rules! make_fallback {
    ($op:ident, $ty:ty) => {
        unsafe fn $op(addr: *const core::ffi::c_void) -> $ty { WARN!(1, "Invalid read at address %llx\n", addr as usize as u64); !0 as $ty }
        unsafe fn $op##_write(val: $ty, addr: *mut core::ffi::c_void) { WARN!(1, "Invalid write at address %llx\n", addr as usize as u64); let _ = val; }
    };
}

#[cfg(not(CONFIG_INDIRECT_IOMEM_FALLBACK))]
unsafe fn real_memset_io(addr: *mut core::ffi::c_void, _value: i32, _size: usize) { WARN!(1, "Invalid memset_io at address 0x%llx\n", addr as usize as u64); }
#[cfg(not(CONFIG_INDIRECT_IOMEM_FALLBACK))]
unsafe fn real_memcpy_fromio(buffer: *mut core::ffi::c_void, addr: *const core::ffi::c_void, size: usize) { WARN!(1, "Invalid memcpy_fromio at address 0x%llx\n", addr as usize as u64); core::ptr::write_bytes(buffer, 0xff, size); }
#[cfg(not(CONFIG_INDIRECT_IOMEM_FALLBACK))]
unsafe fn real_memcpy_toio(addr: *mut core::ffi::c_void, _buffer: *const core::ffi::c_void, _size: usize) { WARN!(1, "Invalid memcpy_toio at address 0x%llx\n", addr as usize as u64); }

macro_rules! make_op {
    ($read:ident, $write:ident, $ty:ty, $bytes:expr) => {
        pub unsafe fn $read(addr: *const core::ffi::c_void) -> $ty { let area = get_area(addr); if area.is_null() { return real_$read(addr); } ((*(*area).ops).read)((*area).priv_, addr as usize & AREA_MASK, $bytes) as $ty }
        pub unsafe fn $write(val: $ty, addr: *mut core::ffi::c_void) { let area = get_area(addr); if area.is_null() { real_$write(val, addr); return; } ((*(*area).ops).write)((*area).priv_, addr as usize & AREA_MASK, $bytes, val as u64); }
    };
}

pub unsafe fn memset_io(addr: *mut core::ffi::c_void, value: i32, size: usize) {
    let area = get_area(addr); if area.is_null() { real_memset_io(addr, value, size); return; }
    let start = addr as usize & AREA_MASK;
    if let Some(set) = (*(*area).ops).set { set((*area).priv_, start, value, size); return; }
    for offs in 0..size { ((*(*area).ops).write)((*area).priv_, start + offs, 1, value as u64); }
}

pub unsafe fn memcpy_fromio(buffer: *mut core::ffi::c_void, addr: *const core::ffi::c_void, size: usize) {
    let area = get_area(addr); if area.is_null() { real_memcpy_fromio(buffer, addr, size); return; }
    let start = addr as usize & AREA_MASK;
    if let Some(copy_from) = (*(*area).ops).copy_from { copy_from((*area).priv_, buffer, start, size); return; }
    for offs in 0..size { *(buffer.add(offs) as *mut u8) = ((*(*area).ops).read)((*area).priv_, start + offs, 1) as u8; }
}

pub unsafe fn memcpy_toio(addr: *mut core::ffi::c_void, buffer: *const core::ffi::c_void, size: usize) {
    let area = get_area(addr); if area.is_null() { real_memcpy_toio(addr, buffer, size); return; }
    let start = addr as usize & AREA_MASK;
    if let Some(copy_to) = (*(*area).ops).copy_to { copy_to((*area).priv_, start, buffer, size); return; }
    for offs in 0..size { ((*(*area).ops).write)((*area).priv_, start + offs, 1, *(buffer.add(offs) as *const u8) as u64); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
