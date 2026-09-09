/*
 * Functions for ST-RAM allocations
 *
 * Copyright 1994-97 Roman Hodek <Roman.Hodek@informatik.uni-erlangen.de>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Linux and Atari declarations supplied by the surrounding kernel sources.

/*
 * The ST-RAM allocator allocates memory from a pool of reserved ST-RAM of
 * configurable size, set aside on ST-RAM init.
 * As long as this pool is not exhausted, allocation of real ST-RAM can be
 * guaranteed.
 */

/* set if kernel is in ST-RAM */
static mut kernel_in_stram: i32 = 0;

static mut stram_pool: resource = resource {
    name: b"ST-RAM Pool\0".as_ptr() as *const i8,
    ..unsafe { core::mem::zeroed() }
};

static mut pool_size: usize = 1024 * 1024;
static mut stram_virt_offset: usize = 0;

unsafe extern "C" {
    static mut m68k_memory: [m68k_mem_info; 1];
    static mut m68k_num_memory: i32;
    static mut iomem_resource: resource;

    fn memparse(arg: *mut i8, retptr: *mut *mut i8) -> usize;
    fn request_resource(parent: *mut resource, new: *mut resource) -> i32;
    fn memblock_alloc_low(size: usize, align: usize) -> usize;
    fn ioremap(offset: usize, size: usize) -> *mut core::ffi::c_void;
    fn resource_size(res: *const resource) -> usize;
    fn allocate_resource(parent: *mut resource, res: *mut resource, size: usize,
                         min: usize, max: u32, align: usize,
                         goal: *mut core::ffi::c_void,
                         realloc: *mut core::ffi::c_void) -> i32;
    fn lookup_resource(parent: *mut resource, start: usize) -> *mut resource;
    fn release_resource(res: *mut resource) -> i32;
    fn kzalloc_resource() -> *mut resource;
    fn kfree(p: *mut resource);
    fn panic(fmt: *const i8, ...);
    fn MACH_IS_ATARI() -> bool;
}

#[repr(C)]
struct m68k_mem_info {
    addr: usize,
}

#[repr(C)]
struct resource {
    start: usize,
    end: usize,
    name: *const i8,
    ..
}

// Build-time kernel macros/attributes are represented by their source intent.
unsafe fn atari_stram_setup(arg: *mut i8) -> i32 {
    if !MACH_IS_ATARI() {
        return 0;
    }
    pool_size = memparse(arg, core::ptr::null_mut());
    0
}

pub unsafe fn atari_stram_init() {
    let mut i: i32;

    kernel_in_stram = if m68k_memory[0].addr == 0 { 1 } else { 0 };

    i = 0;
    while i < m68k_num_memory {
        if m68k_memory[i as usize].addr == 0 {
            return;
        }
        i += 1;
    }

    panic(b"atari_stram_init: no ST-RAM found!\0".as_ptr() as *const i8);
}

pub unsafe fn atari_stram_reserve_pages(_start_mem: *mut core::ffi::c_void) {
    if kernel_in_stram != 0 {
        stram_pool.start = memblock_alloc_low(pool_size, PAGE_SIZE);
        if stram_pool.start == 0 {
            panic(b"%s: Failed to allocate %lu bytes align=%lx\n\0".as_ptr() as *const i8);
        }
        stram_pool.end = stram_pool.start + pool_size - 1;
        request_resource(&mut iomem_resource, &mut stram_pool);
        stram_virt_offset = 0;
    }
}

unsafe fn atari_stram_map_pages() -> i32 {
    if kernel_in_stram == 0 {
        stram_pool.start = PAGE_SIZE;
        stram_pool.end = stram_pool.start + pool_size - 1;
        request_resource(&mut iomem_resource, &mut stram_pool);
        stram_virt_offset = ioremap(stram_pool.start, resource_size(&stram_pool)) as usize
            - stram_pool.start;
    }
    0
}

pub unsafe fn atari_stram_to_virt(phys: usize) -> *mut core::ffi::c_void {
    (phys + stram_virt_offset) as *mut core::ffi::c_void
}

pub unsafe fn atari_stram_to_phys(virt: *mut core::ffi::c_void) -> usize {
    virt as usize - stram_virt_offset
}

pub unsafe fn atari_stram_alloc(size: usize, owner: *const i8) -> *mut core::ffi::c_void {
    let mut size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    let res = kzalloc_resource();
    if res.is_null() {
        return core::ptr::null_mut();
    }
    (*res).name = owner;
    let error = allocate_resource(&mut stram_pool, res, size, 0, u32::MAX,
                                  PAGE_SIZE, core::ptr::null_mut(), core::ptr::null_mut());
    if error < 0 {
        kfree(res);
        return core::ptr::null_mut();
    }
    atari_stram_to_virt((*res).start)
}

pub unsafe fn atari_stram_free(addr: *mut core::ffi::c_void) {
    let start = atari_stram_to_phys(addr);
    let res = lookup_resource(&mut stram_pool, start);
    if res.is_null() {
        return;
    }
    let _size = resource_size(res);
    release_resource(res);
    kfree(res);
}

// PAGE_SIZE is supplied by asm/page.h; early_param, arch_initcall, and
// EXPORT_SYMBOL are kernel registration/export annotations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
