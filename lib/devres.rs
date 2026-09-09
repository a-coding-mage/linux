// SPDX-License-Identifier: GPL-2.0
// Translated from devres.c. Kernel dependencies are supplied externally.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
    pub end: resource_size_t,
    pub name: *const core::ffi::c_char,
    pub flags: c_ulong,
}

pub type resource_size_t = u64;
pub type c_ulong = usize;
pub type c_int = i32;
pub type iomem = core::ffi::c_void;

extern "C" {
    fn iounmap(addr: *mut iomem);
    fn devres_alloc_node(
        release: unsafe extern "C" fn(*mut device, *mut core::ffi::c_void),
        size: usize,
        gfp: c_ulong,
        node: c_int,
    ) -> *mut core::ffi::c_void;
    fn dev_to_node(dev: *mut device) -> c_int;
    fn ioremap(offset: resource_size_t, size: resource_size_t) -> *mut iomem;
    fn ioremap_uc(offset: resource_size_t, size: resource_size_t) -> *mut iomem;
    fn ioremap_wc(offset: resource_size_t, size: resource_size_t) -> *mut iomem;
    fn ioremap_np(offset: resource_size_t, size: resource_size_t) -> *mut iomem;
    fn devres_add(dev: *mut device, res: *mut core::ffi::c_void);
    fn devres_free(res: *mut core::ffi::c_void);
    fn devres_release(
        dev: *mut device,
        release: unsafe extern "C" fn(*mut device, *mut core::ffi::c_void),
        match_fn: unsafe extern "C" fn(*mut device, *mut core::ffi::c_void, *mut core::ffi::c_void) -> c_int,
        match_data: *mut core::ffi::c_void,
    ) -> c_int;
    fn dev_name(dev: *mut device) -> *const core::ffi::c_char;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const core::ffi::c_char, ...) -> c_int;
    fn devm_kasprintf(dev: *mut device, gfp: c_ulong, fmt: *const core::ffi::c_char, ...) -> *mut core::ffi::c_char;
    fn devm_kstrdup(dev: *mut device, s: *const core::ffi::c_char, gfp: c_ulong) -> *mut core::ffi::c_char;
    fn devm_request_mem_region(dev: *mut device, start: resource_size_t, size: resource_size_t, name: *const core::ffi::c_char) -> *mut resource;
    fn devm_release_mem_region(dev: *mut device, start: resource_size_t, size: resource_size_t);
    fn arch_phys_wc_del(mtrr: c_int);
    fn arch_phys_wc_add(base: c_ulong, size: c_ulong) -> c_int;
    fn arch_io_free_memtype_wc(start: resource_size_t, size: resource_size_t);
    fn arch_io_reserve_memtype_wc(start: resource_size_t, size: resource_size_t) -> c_int;
    fn ioport_unmap(addr: *mut iomem);
    fn ioport_map(port: c_ulong, nr: u32) -> *mut iomem;
    fn of_address_to_resource(node: *mut device_node, index: c_int, res: *mut resource) -> c_int;
}

const GFP_KERNEL: c_ulong = 0;
const IORESOURCE_MEM: c_ulong = 0x0000_0200;
const IORESOURCE_MEM_NONPOSTED: c_ulong = 0x0000_0800;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;

#[repr(C)]
enum devm_ioremap_type {
    DEVM_IOREMAP = 0,
    DEVM_IOREMAP_UC,
    DEVM_IOREMAP_WC,
    DEVM_IOREMAP_NP,
}

#[no_mangle]
pub unsafe extern "C" fn devm_ioremap_release(_dev: *mut device, res: *mut core::ffi::c_void) {
    iounmap(*(res as *mut *mut iomem));
}

unsafe extern "C" fn devm_ioremap_match(_dev: *mut device, res: *mut core::ffi::c_void, match_data: *mut core::ffi::c_void) -> c_int {
    (*(res as *mut *mut core::ffi::c_void) == match_data) as c_int
}

unsafe fn __devm_ioremap(dev: *mut device, offset: resource_size_t, size: resource_size_t, kind: devm_ioremap_type) -> *mut iomem {
    let ptr = devres_alloc_node(devm_ioremap_release, core::mem::size_of::<*mut iomem>(), GFP_KERNEL, dev_to_node(dev)) as *mut *mut iomem;
    if ptr.is_null() { return core::ptr::null_mut(); }
    let addr = match kind {
        devm_ioremap_type::DEVM_IOREMAP => ioremap(offset, size),
        devm_ioremap_type::DEVM_IOREMAP_UC => ioremap_uc(offset, size),
        devm_ioremap_type::DEVM_IOREMAP_WC => ioremap_wc(offset, size),
        devm_ioremap_type::DEVM_IOREMAP_NP => ioremap_np(offset, size),
    };
    if !addr.is_null() { *ptr = addr; devres_add(dev, ptr as *mut _); } else { devres_free(ptr as *mut _); }
    addr
}

#[no_mangle]
pub unsafe extern "C" fn devm_ioremap(dev: *mut device, offset: resource_size_t, size: resource_size_t) -> *mut iomem { __devm_ioremap(dev, offset, size, devm_ioremap_type::DEVM_IOREMAP) }
#[no_mangle]
pub unsafe extern "C" fn devm_ioremap_uc(dev: *mut device, offset: resource_size_t, size: resource_size_t) -> *mut iomem { __devm_ioremap(dev, offset, size, devm_ioremap_type::DEVM_IOREMAP_UC) }
#[no_mangle]
pub unsafe extern "C" fn devm_ioremap_wc(dev: *mut device, offset: resource_size_t, size: resource_size_t) -> *mut iomem { __devm_ioremap(dev, offset, size, devm_ioremap_type::DEVM_IOREMAP_WC) }

#[no_mangle]
pub unsafe extern "C" fn devm_iounmap(dev: *mut device, addr: *mut iomem) {
    let _ = devres_release(dev, devm_ioremap_release, devm_ioremap_match, addr);
}

unsafe fn __devm_ioremap_resource(dev: *mut device, res: *const resource, kind: devm_ioremap_type) -> *mut iomem {
    if res.is_null() || ((*res).flags & IORESOURCE_MEM) == 0 { return (-EINVAL as isize) as *mut iomem; }
    let size = (*res).end.wrapping_sub((*res).start).wrapping_add(1);
    let selected = if matches!(kind, devm_ioremap_type::DEVM_IOREMAP) && ((*res).flags & IORESOURCE_MEM_NONPOSTED) != 0 { devm_ioremap_type::DEVM_IOREMAP_NP } else { kind };
    let addr = __devm_ioremap(dev, (*res).start, size, selected);
    if addr.is_null() { devm_release_mem_region(dev, (*res).start, size); return (-ENOMEM as isize) as *mut iomem; }
    addr
}

#[no_mangle]
pub unsafe extern "C" fn devm_ioremap_resource(dev: *mut device, res: *const resource) -> *mut iomem { __devm_ioremap_resource(dev, res, devm_ioremap_type::DEVM_IOREMAP) }
#[no_mangle]
pub unsafe extern "C" fn devm_ioremap_resource_wc(dev: *mut device, res: *const resource) -> *mut iomem { __devm_ioremap_resource(dev, res, devm_ioremap_type::DEVM_IOREMAP_WC) }

#[no_mangle]
pub unsafe extern "C" fn devm_of_iomap(dev: *mut device, node: *mut device_node, index: c_int, size: *mut resource_size_t) -> *mut iomem {
    let mut res = core::mem::MaybeUninit::<resource>::uninit();
    if of_address_to_resource(node, index, res.as_mut_ptr()) != 0 { return (-EINVAL as isize) as *mut iomem; }
    let res = res.assume_init();
    if !size.is_null() { *size = res.end.wrapping_sub(res.start).wrapping_add(1); }
    devm_ioremap_resource(dev, &res)
}

unsafe extern "C" fn devm_ioport_map_release(_dev: *mut device, res: *mut core::ffi::c_void) { ioport_unmap(*(res as *mut *mut iomem)); }
unsafe extern "C" fn devm_ioport_map_match(_dev: *mut device, res: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> c_int { (*(res as *mut *mut core::ffi::c_void) == data) as c_int }

#[no_mangle]
pub unsafe extern "C" fn devm_ioport_map(dev: *mut device, port: c_ulong, nr: u32) -> *mut iomem {
    let ptr = devres_alloc_node(devm_ioport_map_release, core::mem::size_of::<*mut iomem>(), GFP_KERNEL, dev_to_node(dev)) as *mut *mut iomem;
    if ptr.is_null() { return core::ptr::null_mut(); }
    let addr = ioport_map(port, nr);
    if !addr.is_null() { *ptr = addr; devres_add(dev, ptr as *mut _); } else { devres_free(ptr as *mut _); }
    addr
}

#[no_mangle]
pub unsafe extern "C" fn devm_ioport_unmap(dev: *mut device, addr: *mut iomem) { let _ = devres_release(dev, devm_ioport_map_release, devm_ioport_map_match, addr); }

#[repr(C)]
pub struct arch_io_reserve_memtype_wc_devres { pub start: resource_size_t, pub size: resource_size_t }

unsafe extern "C" fn devm_arch_io_free_memtype_wc_release(_dev: *mut device, res: *mut core::ffi::c_void) {
    let this = &*(res as *const arch_io_reserve_memtype_wc_devres);
    arch_io_free_memtype_wc(this.start, this.size);
}

#[no_mangle]
pub unsafe extern "C" fn devm_arch_phys_wc_add(dev: *mut device, base: c_ulong, size: c_ulong) -> c_int {
    let mtrr = devres_alloc_node(devm_arch_phys_ac_add_release, core::mem::size_of::<c_int>(), GFP_KERNEL, dev_to_node(dev)) as *mut c_int;
    if mtrr.is_null() { return -ENOMEM; }
    let ret = arch_phys_wc_add(base, size);
    if ret < 0 { devres_free(mtrr as *mut _); return ret; }
    *mtrr = ret; devres_add(dev, mtrr as *mut _); ret
}

unsafe extern "C" fn devm_arch_phys_ac_add_release(_dev: *mut device, res: *mut core::ffi::c_void) { arch_phys_wc_del(*(res as *mut c_int)); }

#[no_mangle]
pub unsafe extern "C" fn devm_arch_io_reserve_memtype_wc(dev: *mut device, start: resource_size_t, size: resource_size_t) -> c_int {
    let dr = devres_alloc_node(devm_arch_io_free_memtype_wc_release, core::mem::size_of::<arch_io_reserve_memtype_wc_devres>(), GFP_KERNEL, dev_to_node(dev)) as *mut arch_io_reserve_memtype_wc_devres;
    if dr.is_null() { return -ENOMEM; }
    let ret = arch_io_reserve_memtype_wc(start, size);
    if ret < 0 { devres_free(dr as *mut _); return ret; }
    (*dr).start = start; (*dr).size = size; devres_add(dev, dr as *mut _); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
