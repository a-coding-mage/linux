// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel I/OAT DMA Linux driver
 * Copyright(c) 2004 - 2015 Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_void};

// Linux kernel and driver declarations are supplied by the surrounding tree.
extern "C" {
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn sscanf(buf: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn dma_has_cap(cap: c_int, mask: *const c_void) -> bool;
    fn to_ioatdma_device(dma: *mut dma_device) -> *mut ioatdma_device;
    fn to_ioat_chan(chan: *mut dma_chan) -> *mut ioatdma_chan;
    fn ioat_ring_active(chan: *mut ioatdma_chan) -> c_int;
    fn kobject_init_and_add(
        kobj: *mut kobject,
        ty: *const kobj_type,
        parent: *mut kobject,
        name: *const c_char,
        ...,
    ) -> c_int;
    fn kobject_put(kobj: *mut kobject);
    fn kobject_del(kobj: *mut kobject);
    fn set_bit(nr: c_int, addr: *mut c_void);
    fn test_bit(nr: c_int, addr: *const c_void) -> bool;
    fn to_dev(chan: *mut ioatdma_chan) -> *mut device;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct ioat_sysfs_entry {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut dma_chan, *mut c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut dma_chan, *const c_char, usize) -> isize>,
}

unsafe extern "C" fn cap_show(c: *mut dma_chan, page: *mut c_char) -> isize {
    let dma = (*c).device;

    sprintf(
        page,
        b"copy%s%s%s%s%s\0".as_ptr() as *const c_char,
        if dma_has_cap(DMA_PQ, (*dma).cap_mask) { b" pq\0".as_ptr() } else { b"\0".as_ptr() },
        if dma_has_cap(DMA_PQ_VAL, (*dma).cap_mask) { b" pq_val\0".as_ptr() } else { b"\0".as_ptr() },
        if dma_has_cap(DMA_XOR, (*dma).cap_mask) { b" xor\0".as_ptr() } else { b"\0".as_ptr() },
        if dma_has_cap(DMA_XOR_VAL, (*dma).cap_mask) { b" xor_val\0".as_ptr() } else { b"\0".as_ptr() },
        if dma_has_cap(DMA_INTERRUPT, (*dma).cap_mask) { b" intr\n\0".as_ptr() } else { b"\n\0".as_ptr() },
    )
}

pub static ioat_cap_attr: ioat_sysfs_entry = __ATTR_RO!(cap, cap_show);

unsafe extern "C" fn version_show(c: *mut dma_chan, page: *mut c_char) -> isize {
    let dma = (*c).device;
    let ioat_dma = to_ioatdma_device(dma);
    sprintf(
        page,
        b"%d.%d\n\0".as_ptr() as *const c_char,
        (*ioat_dma).version >> 4,
        (*ioat_dma).version & 0xf,
    )
}

pub static ioat_version_attr: ioat_sysfs_entry = __ATTR_RO!(version, version_show);

unsafe extern "C" fn ioat_attr_show(kobj: *mut kobject, attr: *mut attribute, page: *mut c_char) -> isize {
    let entry = container_of_const!(attr, ioat_sysfs_entry, attr);
    let ioat_chan = container_of!(kobj, ioatdma_chan, kobj);
    if (*entry).show.is_none() { return -EIO; }
    ((*entry).show.unwrap())(&mut (*ioat_chan).dma_chan, page)
}

unsafe extern "C" fn ioat_attr_store(kobj: *mut kobject, attr: *mut attribute, page: *const c_char, count: usize) -> isize {
    let entry = container_of_const!(attr, ioat_sysfs_entry, attr);
    let ioat_chan = container_of!(kobj, ioatdma_chan, kobj);
    if (*entry).store.is_none() { return -EIO; }
    ((*entry).store.unwrap())(&mut (*ioat_chan).dma_chan, page, count)
}

pub static ioat_sysfs_ops: sysfs_ops = sysfs_ops { show: Some(ioat_attr_show), store: Some(ioat_attr_store) };

pub unsafe extern "C" fn ioat_kobject_add(ioat_dma: *mut ioatdma_device, ty: *const kobj_type) {
    let dma = &mut (*ioat_dma).dma_dev;
    let mut c: *mut dma_chan;
    list_for_each_entry!(c, &dma.channels, device_node);
    {
        let ioat_chan = to_ioat_chan(c);
        let parent = &mut (*(*c).dev).device.kobj;
        let err = kobject_init_and_add(&mut (*ioat_chan).kobj, ty, parent, b"quickdata\0".as_ptr() as *const c_char);
        if err != 0 {
            dev_warn(to_dev(ioat_chan), b"sysfs init error (%d), continuing...\n\0".as_ptr() as *const c_char, err);
            kobject_put(&mut (*ioat_chan).kobj);
            set_bit(IOAT_KOBJ_INIT_FAIL, &mut (*ioat_chan).state as *mut _ as *mut c_void);
        }
    }
}

pub unsafe extern "C" fn ioat_kobject_del(ioat_dma: *mut ioatdma_device) {
    let dma = &mut (*ioat_dma).dma_dev;
    let mut c: *mut dma_chan;
    list_for_each_entry!(c, &dma.channels, device_node);
    {
        let ioat_chan = to_ioat_chan(c);
        if !test_bit(IOAT_KOBJ_INIT_FAIL, &(*ioat_chan).state as *const _ as *const c_void) {
            kobject_del(&mut (*ioat_chan).kobj);
            kobject_put(&mut (*ioat_chan).kobj);
        }
    }
}

unsafe extern "C" fn ring_size_show(c: *mut dma_chan, page: *mut c_char) -> isize {
    let ioat_chan = to_ioat_chan(c);
    sprintf(page, b"%d\n\0".as_ptr() as *const c_char, ((1 << (*ioat_chan).alloc_order) & !1))
}
pub static ring_size_attr: ioat_sysfs_entry = __ATTR_RO!(ring_size, ring_size_show);

unsafe extern "C" fn ring_active_show(c: *mut dma_chan, page: *mut c_char) -> isize {
    let ioat_chan = to_ioat_chan(c);
    sprintf(page, b"%d\n\0".as_ptr() as *const c_char, ioat_ring_active(ioat_chan))
}
pub static ring_active_attr: ioat_sysfs_entry = __ATTR_RO!(ring_active, ring_active_show);

unsafe extern "C" fn intr_coalesce_show(c: *mut dma_chan, page: *mut c_char) -> isize {
    let ioat_chan = to_ioat_chan(c);
    sprintf(page, b"%d\n\0".as_ptr() as *const c_char, (*ioat_chan).intr_coalesce)
}

unsafe extern "C" fn intr_coalesce_store(c: *mut dma_chan, page: *const c_char, count: usize) -> isize {
    let mut intr_coalesce: c_int = 0;
    let ioat_chan = to_ioat_chan(c);
    if sscanf(page, b"%du\0".as_ptr() as *const c_char, &mut intr_coalesce) != -1 {
        if intr_coalesce < 0 || intr_coalesce > IOAT_INTRDELAY_MASK { return -EINVAL; }
        (*ioat_chan).intr_coalesce = intr_coalesce;
    }
    count as isize
}

pub static intr_coalesce_attr: ioat_sysfs_entry = __ATTR_RW!(intr_coalesce, intr_coalesce_show, intr_coalesce_store);

pub static ioat_attrs: [*const attribute; 6] = [
    &ring_size_attr.attr,
    &ring_active_attr.attr,
    &ioat_cap_attr.attr,
    &ioat_version_attr.attr,
    &intr_coalesce_attr.attr,
    core::ptr::null(),
];

pub static ioat_ktype: kobj_type = kobj_type {
    sysfs_ops: &ioat_sysfs_ops,
    default_groups: ioat_groups,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
