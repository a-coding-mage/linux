// SPDX-License-Identifier: GPL-2.0
/*
 * arch/alpha/kernel/pci-sysfs.c
 *
 * Copyright (C) 2009 Ivan Kokshaysky
 *
 * Alpha PCI resource files.
 *
 * Loosely based on generic HAVE_PCI_MMAP implementation in
 * drivers/pci/pci-sysfs.c
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

unsafe fn hose_mmap_page_range(
    hose: *mut pci_controller,
    vma: *mut vm_area_struct,
    mmap_type: pci_mmap_state,
    sparse: i32,
) -> i32 {
    let base: c_ulong;
    if mmap_type == pci_mmap_state::PciMmapMem {
        base = if sparse != 0 { (*hose).sparse_mem_base } else { (*hose).dense_mem_base };
    } else {
        base = if sparse != 0 { (*hose).sparse_io_base } else { (*hose).dense_io_base };
    }
    (*vma).vm_pgoff = (*vma).vm_pgoff.wrapping_add(base >> PAGE_SHIFT);
    io_remap_pfn_range(
        vma,
        (*vma).vm_start,
        (*vma).vm_pgoff,
        (*vma).vm_end.wrapping_sub((*vma).vm_start),
        (*vma).vm_page_prot,
    )
}

unsafe fn __pci_mmap_fits(
    pdev: *mut pci_dev,
    num: i32,
    vma: *mut vm_area_struct,
    sparse: i32,
) -> i32 {
    let len = pci_resource_len(pdev, num);
    let shift: i32 = if sparse != 0 { 5 } else { 0 };
    if len == 0 { return 0; }
    let nr = vma_pages(vma);
    let start = (*vma).vm_pgoff;
    let size = ((len.wrapping_sub(1)) >> (PAGE_SHIFT - shift)) + 1;
    if start < size && size.wrapping_sub(start) >= nr { 1 } else { 0 }
}

unsafe fn pci_mmap_resource(
    kobj: *mut kobject,
    attr: *const bin_attribute,
    vma: *mut vm_area_struct,
    sparse: i32,
) -> i32 {
    let pdev = to_pci_dev(kobj_to_dev(kobj));
    let barno = (*attr).private as c_ulong as i32;
    let mut mmap_type: pci_mmap_state;
    let mut bar: pci_bus_region = core::mem::zeroed();
    let ret = security_locked_down(LOCKDOWN_PCI_ACCESS);
    if ret != 0 { return ret; }
    if pci_resource_is_mem(pdev, barno) != 0 && iomem_is_exclusive(pci_resource_start(pdev, barno)) != 0 {
        return -EINVAL;
    }
    if __pci_mmap_fits(pdev, barno, vma, sparse) == 0 { return -EINVAL; }
    pcibios_resource_to_bus((*pdev).bus, &mut bar, pci_resource_n(pdev, barno));
    (*vma).vm_pgoff = (*vma).vm_pgoff.wrapping_add(bar.start >> (PAGE_SHIFT - if sparse != 0 { 5 } else { 0 }));
    mmap_type = if pci_resource_is_mem(pdev, barno) != 0 { pci_mmap_state::PciMmapMem } else { pci_mmap_state::PciMmapIo };
    hose_mmap_page_range((*pdev).sysdata, vma, mmap_type, sparse)
}

unsafe fn pci_mmap_resource_sparse(_filp: *mut file, kobj: *mut kobject, attr: *const bin_attribute, vma: *mut vm_area_struct) -> i32 {
    pci_mmap_resource(kobj, attr, vma, 1)
}

unsafe fn pci_mmap_resource_dense(_filp: *mut file, kobj: *mut kobject, attr: *const bin_attribute, vma: *mut vm_area_struct) -> i32 {
    pci_mmap_resource(kobj, attr, vma, 0)
}

unsafe fn sparse_mem_mmap_fits(pdev: *mut pci_dev, num: i32) -> i32 {
    let mut bar: pci_bus_region = core::mem::zeroed();
    let hose = (*pdev).sysdata;
    pcibios_resource_to_bus((*pdev).bus, &mut bar, pci_resource_n(pdev, num));
    let dense_offset = ((*hose).dense_mem_base.wrapping_sub((*hose).sparse_mem_base)) as c_long;
    let sparse_size: c_ulong = if dense_offset >= 0x400000000 { 0x20000000 } else { 0x8000000 };
    if bar.end < sparse_size { 1 } else { 0 }
}

/* Legacy I/O bus mapping stuff. */
unsafe fn __legacy_mmap_fits(vma: *mut vm_area_struct, res_size: c_ulong) -> i32 {
    let nr = vma_pages(vma);
    let start = (*vma).vm_pgoff;
    let size = ((res_size - 1) >> PAGE_SHIFT) + 1;
    if start < size && size - start >= nr { 1 } else { 0 }
}

unsafe fn has_sparse(hose: *mut pci_controller, mmap_type: pci_mmap_state) -> i32 {
    let base = if mmap_type == pci_mmap_state::PciMmapMem { (*hose).sparse_mem_base } else { (*hose).sparse_io_base };
    if base != 0 { 1 } else { 0 }
}

pub unsafe fn pci_mmap_legacy_page_range(bus: *mut pci_bus, vma: *mut vm_area_struct, mmap_type: pci_mmap_state) -> i32 {
    let hose = (*bus).sysdata;
    let sparse = has_sparse(hose, mmap_type);
    let mut res_size = if mmap_type == pci_mmap_state::PciMmapMem { PCI_LEGACY_MEM_SIZE } else { PCI_LEGACY_IO_SIZE };
    if sparse != 0 { res_size <<= 5; }
    if __legacy_mmap_fits(vma, res_size) == 0 { return -EINVAL; }
    hose_mmap_page_range(hose, vma, mmap_type, sparse)
}

pub unsafe fn pci_legacy_has_sparse(bus: *mut pci_bus, ty: pci_mmap_state) -> bool {
    has_sparse((*bus).sysdata, ty) != 0
}

/* Legacy I/O bus read/write functions */
pub unsafe fn pci_legacy_read(bus: *mut pci_bus, mut port: loff_t, val: *mut u32, size: usize) -> i32 {
    let hose = (*bus).sysdata;
    port += (*hose).io_space.start as loff_t;
    match size {
        1 => { *(val as *mut u8) = inb(port); 1 }
        2 => { if port & 1 != 0 { return -EINVAL; } *(val as *mut u16) = inw(port); 2 }
        4 => { if port & 3 != 0 { return -EINVAL; } *val = inl(port); 4 }
        _ => -EINVAL,
    }
}

pub unsafe fn pci_legacy_write(bus: *mut pci_bus, mut port: loff_t, val: u32, size: usize) -> i32 {
    let hose = (*bus).sysdata;
    port += (*hose).io_space.start as loff_t;
    match size {
        1 => { outb(val, port); 1 }
        2 => { if port & 1 != 0 { return -EINVAL; } outw(val, port); 2 }
        4 => { if port & 3 != 0 { return -EINVAL; } outl(val, port); 4 }
        _ => -EINVAL,
    }
}

// The C macro-generated bin_attribute declarations are represented by the
// corresponding external kernel objects; their definitions are supplied by
// the surrounding PCI sysfs support.
extern "C" {
    static pci_dev_resource0_attr: bin_attribute;
    static pci_dev_resource1_attr: bin_attribute;
    static pci_dev_resource2_attr: bin_attribute;
    static pci_dev_resource3_attr: bin_attribute;
    static pci_dev_resource4_attr: bin_attribute;
    static pci_dev_resource5_attr: bin_attribute;
    static pci_dev_resource0_sparse_attr: bin_attribute;
    static pci_dev_resource1_sparse_attr: bin_attribute;
    static pci_dev_resource2_sparse_attr: bin_attribute;
    static pci_dev_resource3_sparse_attr: bin_attribute;
    static pci_dev_resource4_sparse_attr: bin_attribute;
    static pci_dev_resource5_sparse_attr: bin_attribute;
    static pci_dev_resource0_dense_attr: bin_attribute;
    static pci_dev_resource1_dense_attr: bin_attribute;
    static pci_dev_resource2_dense_attr: bin_attribute;
    static pci_dev_resource3_dense_attr: bin_attribute;
    static pci_dev_resource4_dense_attr: bin_attribute;
    static pci_dev_resource5_dense_attr: bin_attribute;
}

unsafe fn pci_bar_mmap_type(pdev: *mut pci_dev, bar: i32) -> pci_mmap_state {
    if pci_resource_is_mem(pdev, bar) != 0 { pci_mmap_state::PciMmapMem } else { pci_mmap_state::PciMmapIo }
}

unsafe fn __pci_resource_attr_is_visible(kobj: *mut kobject, a: *const bin_attribute, bar: i32) -> umode_t {
    let pdev = to_pci_dev(kobj_to_dev(kobj));
    if pci_resource_len(pdev, bar) == 0 { 0 } else { (*a).attr.mode }
}

unsafe fn pci_dev_resource_is_visible(kobj: *mut kobject, a: *const bin_attribute, bar: i32) -> umode_t {
    let pdev = to_pci_dev(kobj_to_dev(kobj));
    let hose = (*pdev).sysdata;
    if has_sparse(hose, pci_bar_mmap_type(pdev, bar)) != 0 { 0 } else { __pci_resource_attr_is_visible(kobj, a, bar) }
}

unsafe fn pci_dev_resource_sparse_is_visible(kobj: *mut kobject, a: *const bin_attribute, bar: i32) -> umode_t {
    let pdev = to_pci_dev(kobj_to_dev(kobj));
    let hose = (*pdev).sysdata;
    let ty = pci_bar_mmap_type(pdev, bar);
    if has_sparse(hose, ty) == 0 { return 0; }
    if ty == pci_mmap_state::PciMmapMem && sparse_mem_mmap_fits(pdev, bar) == 0 { return 0; }
    __pci_resource_attr_is_visible(kobj, a, bar)
}

unsafe fn pci_dev_resource_dense_is_visible(kobj: *mut kobject, a: *const bin_attribute, bar: i32) -> umode_t {
    let pdev = to_pci_dev(kobj_to_dev(kobj));
    let hose = (*pdev).sysdata;
    let ty = pci_bar_mmap_type(pdev, bar);
    if has_sparse(hose, ty) == 0 { return 0; }
    if ty == pci_mmap_state::PciMmapMem && sparse_mem_mmap_fits(pdev, bar) == 0 {
        return __pci_resource_attr_is_visible(kobj, a, bar);
    }
    let dense_base = if ty == pci_mmap_state::PciMmapMem { (*hose).dense_mem_base } else { (*hose).dense_io_base };
    if dense_base == 0 { return 0; }
    __pci_resource_attr_is_visible(kobj, a, bar)
}

unsafe fn __pci_dev_resource_bin_size(kobj: *mut kobject, bar: i32, sparse: bool) -> usize {
    let pdev = to_pci_dev(kobj_to_dev(kobj));
    let size = pci_resource_len(pdev, bar) as usize;
    if sparse { size << 5 } else { size }
}

unsafe fn pci_dev_resource_bin_size(kobj: *mut kobject, _a: *const bin_attribute, bar: i32) -> usize {
    __pci_dev_resource_bin_size(kobj, bar, false)
}
unsafe fn pci_dev_resource_sparse_bin_size(kobj: *mut kobject, _a: *const bin_attribute, bar: i32) -> usize {
    __pci_dev_resource_bin_size(kobj, bar, true)
}

// Attribute arrays correspond to the six C macro expansions for each mapping mode.
#[no_mangle]
pub static pci_dev_resource_attrs: [*const bin_attribute; 7] = unsafe {
    [&pci_dev_resource0_attr, &pci_dev_resource1_attr, &pci_dev_resource2_attr,
     &pci_dev_resource3_attr, &pci_dev_resource4_attr, &pci_dev_resource5_attr, core::ptr::null()]
};
#[no_mangle]
pub static pci_dev_resource_sparse_attrs: [*const bin_attribute; 7] = unsafe {
    [&pci_dev_resource0_sparse_attr, &pci_dev_resource1_sparse_attr, &pci_dev_resource2_sparse_attr,
     &pci_dev_resource3_sparse_attr, &pci_dev_resource4_sparse_attr, &pci_dev_resource5_sparse_attr, core::ptr::null()]
};
#[no_mangle]
pub static pci_dev_resource_dense_attrs: [*const bin_attribute; 7] = unsafe {
    [&pci_dev_resource0_dense_attr, &pci_dev_resource1_dense_attr, &pci_dev_resource2_dense_attr,
     &pci_dev_resource3_dense_attr, &pci_dev_resource4_dense_attr, &pci_dev_resource5_dense_attr, core::ptr::null()]
};

#[no_mangle]
pub static pci_dev_resource_attr_group: attribute_group = attribute_group {
    bin_attrs: pci_dev_resource_attrs.as_ptr(), is_bin_visible: Some(pci_dev_resource_is_visible), bin_size: Some(pci_dev_resource_bin_size),
};
#[no_mangle]
pub static pci_dev_resource_sparse_attr_group: attribute_group = attribute_group {
    bin_attrs: pci_dev_resource_sparse_attrs.as_ptr(), is_bin_visible: Some(pci_dev_resource_sparse_is_visible), bin_size: Some(pci_dev_resource_sparse_bin_size),
};
#[no_mangle]
pub static pci_dev_resource_dense_attr_group: attribute_group = attribute_group {
    bin_attrs: pci_dev_resource_dense_attrs.as_ptr(), is_bin_visible: Some(pci_dev_resource_dense_is_visible), bin_size: Some(pci_dev_resource_bin_size),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
