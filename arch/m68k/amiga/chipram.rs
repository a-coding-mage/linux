// SPDX-License-Identifier: GPL-2.0
/*
**  linux/amiga/chipram.c
**
**      Modified 03-May-94 by Geert Uytterhoeven <geert@linux-m68k.org>
**          - 64-bit aligned allocations for full AGA compatibility
**
**\tRewritten 15/9/2000 by Geert to use resource management
*/

use core::ffi::c_char;

// Types, constants, and functions below are supplied by the surrounding kernel.
#[repr(C)]
pub struct resource {
    pub name: *const c_char,
    pub start: usize,
    pub end: usize,
}

#[repr(C)]
pub struct atomic_t {
    pub counter: i32,
}

extern "C" {
    static mut iomem_resource: resource;
    fn request_resource(parent: *mut resource, new: *mut resource) -> i32;
    fn allocate_resource(
        root: *mut resource,
        new: *mut resource,
        size: usize,
        min: usize,
        max: usize,
        align: usize,
        arch_data: *mut core::ffi::c_void,
        alignf: *mut core::ffi::c_void,
    ) -> i32;
    fn lookup_resource(root: *mut resource, start: usize) -> *mut resource;
    fn release_resource(old: *mut resource) -> i32;
    fn atomic_set(v: *mut atomic_t, i: i32);
    fn atomic_sub(i: i32, v: *mut atomic_t);
    fn atomic_add(i: i32, v: *mut atomic_t);
    fn atomic_read(v: *const atomic_t) -> i32;
    fn kzalloc_resource() -> *mut resource;
    fn kfree(p: *mut core::ffi::c_void);
    fn page_align(size: usize) -> usize;
    fn ztwo_vaddr(addr: usize) -> *mut core::ffi::c_void;
    fn ztwo_paddr(ptr: *mut core::ffi::c_void) -> usize;
    fn amiga_hw_present_chip_ram() -> bool;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

#[no_mangle]
pub static mut amiga_chip_size: usize = 0;

static mut chipram_res: resource = resource {
    name: b"Chip RAM\0".as_ptr() as *const c_char,
    start: 0, // CHIP_PHYSADDR
    end: 0,
};
static mut chipavail: atomic_t = atomic_t { counter: 0 };

pub unsafe fn amiga_chip_init() {
    if !amiga_hw_present_chip_ram() {
        return;
    }

    chipram_res.end = chipram_res.start + amiga_chip_size - 1;
    request_resource(&mut iomem_resource, &mut chipram_res);

    atomic_set(&mut chipavail, amiga_chip_size as i32);
}

pub unsafe fn amiga_chip_alloc(size: usize, name: *const c_char) -> *mut core::ffi::c_void {
    let res = kzalloc_resource();
    if res.is_null() {
        return core::ptr::null_mut();
    }

    (*res).name = name;
    let p = amiga_chip_alloc_res(size, res);
    if p.is_null() {
        kfree(res as *mut core::ffi::c_void);
        return core::ptr::null_mut();
    }

    p
}

/*
 *  Warning:
 *  amiga_chip_alloc_res is meant only for drivers that need to
 *  allocate Chip RAM before kmalloc() is functional. As a consequence,
 *  those drivers must not free that Chip RAM afterwards.
 */
pub unsafe fn amiga_chip_alloc_res(
    mut size: usize,
    res: *mut resource,
) -> *mut core::ffi::c_void {
    size = page_align(size);

    pr_debug(b"amiga_chip_alloc_res: allocate %lu bytes\n\0".as_ptr() as *const c_char, size);
    let error = allocate_resource(
        &mut chipram_res,
        res,
        size,
        0,
        usize::MAX,
        4096, // PAGE_SIZE
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
    if error < 0 {
        pr_err(
            b"amiga_chip_alloc_res: allocate_resource() failed %d!\n\0".as_ptr()
                as *const c_char,
            error,
        );
        return core::ptr::null_mut();
    }

    atomic_sub(size as i32, &mut chipavail);
    pr_debug(
        b"amiga_chip_alloc_res: returning %pR\n\0".as_ptr() as *const c_char,
        res,
    );
    ztwo_vaddr((*res).start)
}

pub unsafe fn amiga_chip_free(ptr: *mut core::ffi::c_void) {
    let start = ztwo_paddr(ptr);
    let res = lookup_resource(&mut chipram_res, start);
    if res.is_null() {
        pr_err(
            b"amiga_chip_free: trying to free nonexistent region at %p\n\0".as_ptr()
                as *const c_char,
            ptr,
        );
        return;
    }

    let size = (*res).end - (*res).start + 1;
    pr_debug(
        b"amiga_chip_free: free %lu bytes at %p\n\0".as_ptr() as *const c_char,
        size,
        ptr,
    );
    atomic_add(size as i32, &mut chipavail);
    release_resource(res);
    kfree(res as *mut core::ffi::c_void);
}

pub unsafe fn amiga_chip_avail() -> usize {
    let n = atomic_read(&chipavail) as usize;

    pr_debug(b"amiga_chip_avail : %lu bytes\n\0".as_ptr() as *const c_char, n);
    n
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
