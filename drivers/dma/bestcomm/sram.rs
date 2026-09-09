// SPDX-License-Identifier: GPL-2.0-only
/*
 * Simple memory allocator for on-board SRAM
 *
 * Maintainer : Sylvain Munaut <tnt@246tNt.com>
 *
 * Copyright (C) 2005 Sylvain Munaut <tnt@246tNt.com>
 */

use core::ffi::c_void;

/* Types and functions supplied by the surrounding kernel/dependency headers. */
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct resource {
    pub start: usize,
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
pub type phys_addr_t = usize;
pub type u32 = core::ffi::c_uint;

#[repr(C)]
pub struct bcom_sram {
    pub base_phys: phys_addr_t,
    pub size: usize,
    pub base_virt: *mut u8,
    pub rh: *mut c_void,
    pub lock: spinlock_t,
}

extern "C" {
    fn kmalloc_obj() -> *mut bcom_sram;
    fn kfree(ptr: *mut bcom_sram);
    fn of_address_to_resource(node: *mut device_node, index: i32, res: *mut resource) -> i32;
    fn request_mem_region(start: usize, size: usize, owner: *const i8) -> *mut c_void;
    fn release_mem_region(start: usize, size: usize);
    fn ioremap(start: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn rh_create(alignment: usize) -> *mut c_void;
    fn rh_destroy(rh: *mut c_void);
    fn rh_attach_region(rh: *mut c_void, start: usize, size: usize);
    fn rh_alloc_align(rh: *mut c_void, size: i32, align: i32, arg: *mut c_void) -> usize;
    fn rh_free(rh: *mut c_void, offset: usize);
    fn of_translate_address(node: *mut device_node, address: *const u32) -> phys_addr_t;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn printk(fmt: *const i8, ...);
}

/* Struct keeping our 'state' */
#[no_mangle]
pub static mut bcom_sram: *mut bcom_sram = core::ptr::null_mut();

/* DO NOT USE in interrupts, if needed in irq handler, we should use the
   _irqsave version of the spin_locks */
#[no_mangle]
pub unsafe extern "C" fn bcom_sram_init(sram_node: *mut device_node, owner: *mut i8) -> i32 {
    let mut rv: i32;
    let mut regaddr_p: *const u32;
    let mut res = core::mem::MaybeUninit::<resource>::uninit();
    let mut psize: u32;

    if !bcom_sram.is_null() {
        printk(b"%s: bcom_sram_init: Already initialized !\n\0".as_ptr() as *const i8, owner);
        return -16; // -EBUSY
    }

    bcom_sram = kmalloc_obj();
    if bcom_sram.is_null() {
        printk(b"%s: bcom_sram_init: Couldn't allocate internal state !\n\0".as_ptr() as *const i8, owner);
        return -12; // -ENOMEM
    }

    rv = of_address_to_resource(sram_node, 0, res.as_mut_ptr());
    if rv != 0 {
        printk(b"%s: bcom_sram_init: Invalid device node !\n\0".as_ptr() as *const i8, owner);
        goto_error_free(rv);
    }
    let res = res.assume_init();
    (*bcom_sram).base_phys = res.start;
    (*bcom_sram).size = resource_size(&res);

    if request_mem_region(res.start, resource_size(&res), owner).is_null() {
        printk(b"%s: bcom_sram_init: Couldn't request region !\n\0".as_ptr() as *const i8, owner);
        rv = -16; // -EBUSY
        goto_error_free(rv);
    }

    /* sram is not really __iomem */
    (*bcom_sram).base_virt = ioremap(res.start, resource_size(&res)) as *mut u8;
    if (*bcom_sram).base_virt.is_null() {
        printk(b"%s: bcom_sram_init: Map error SRAM zone 0x%08lx (0x%0x)!\n\0".as_ptr() as *const i8,
            owner, (*bcom_sram).base_phys, (*bcom_sram).size);
        rv = -12; // -ENOMEM
        release_mem_region(res.start, resource_size(&res));
        goto_error_free(rv);
    }

    (*bcom_sram).rh = rh_create(4);
    regaddr_p = core::ptr::null();
    psize = 0;
    if regaddr_p.is_null() || psize == 0 {
        rh_attach_region((*bcom_sram).rh, 0, (*bcom_sram).size);
    } else {
        while (psize as usize >= 2 * core::mem::size_of::<u32>()) {
            let zbase = of_translate_address(sram_node, regaddr_p);
            rh_attach_region((*bcom_sram).rh, zbase - (*bcom_sram).base_phys, *regaddr_p.add(1) as usize);
            regaddr_p = regaddr_p.add(2);
            psize -= 2 * core::mem::size_of::<u32>() as u32;
        }
    }
    spin_lock_init(&mut (*bcom_sram).lock);
    return 0;

    unsafe fn goto_error_free(rv: i32) -> ! {
        bcom_sram = core::ptr::null_mut();
        panic!("error path: {}", rv)
    }
}

#[inline]
unsafe fn resource_size(res: &resource) -> usize { res.start }

#[no_mangle]
pub unsafe extern "C" fn bcom_sram_cleanup() {
    if !bcom_sram.is_null() {
        rh_destroy((*bcom_sram).rh);
        iounmap((*bcom_sram).base_virt as *mut c_void);
        release_mem_region((*bcom_sram).base_phys, (*bcom_sram).size);
        kfree(bcom_sram);
        bcom_sram = core::ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn bcom_sram_alloc(size: i32, align: i32, phys: *mut phys_addr_t) -> *mut c_void {
    let offset;
    spin_lock(&mut (*bcom_sram).lock);
    offset = rh_alloc_align((*bcom_sram).rh, size, align, core::ptr::null_mut());
    spin_unlock(&mut (*bcom_sram).lock);
    if offset >= usize::MAX - 4095 { return core::ptr::null_mut(); }
    *phys = (*bcom_sram).base_phys + offset;
    (*bcom_sram).base_virt.add(offset) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn bcom_sram_free(ptr: *mut c_void) {
    if ptr.is_null() { return; }
    let offset = (ptr as *mut u8).offset_from((*bcom_sram).base_virt) as usize;
    spin_lock(&mut (*bcom_sram).lock);
    rh_free((*bcom_sram).rh, offset);
    spin_unlock(&mut (*bcom_sram).lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
