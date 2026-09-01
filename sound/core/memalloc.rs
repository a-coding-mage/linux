// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *                   Takashi Iwai <tiwai@suse.de>
 *
 *  Generic memory allocators
 */

use core::ffi::{c_int, c_ulong, c_void};

type size_t = usize;
type dma_addr_t = u64;
type gfp_t = c_ulong;

const GFP_KERNEL: gfp_t = 0;
const __GFP_RETRY_MAYFAIL: gfp_t = 0;
const __GFP_NOWARN: gfp_t = 0;
const __GFP_NORETRY: gfp_t = 0;
const GFP_DMA32: gfp_t = 0;
const GFP_DMA: gfp_t = 0;
const PAGE_SIZE: usize = 4096;
const PAGE_SHIFT: usize = 12;
const ENXIO: c_int = 6;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const VM_MAP: c_ulong = 0;
const PAGE_KERNEL: pgprot_t = 0;

/* Includes in the C source provide these kernel types, constants, and helpers. */
#[repr(C)]
pub struct device {
    pub coherent_dma_mask: u64,
    pub of_node: *mut c_void,
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_start: c_ulong,
    pub vm_end: c_ulong,
    pub vm_page_prot: pgprot_t,
}

#[repr(C)]
pub struct gen_pool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sg_table {
    pub sgl: *mut scatterlist,
    pub orig_nents: c_uint,
}

#[repr(C)]
pub struct sg_page_iter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sg_dma_page_iter {
    pub base: sg_page_iter,
}

type c_uint = u32;
type pgprot_t = c_ulong;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_data_direction {
    DMA_BIDIRECTIONAL = 0,
    DMA_TO_DEVICE = 1,
    DMA_FROM_DEVICE = 2,
}

use dma_data_direction::*;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_dma_sync_mode {
    SNDRV_DMA_SYNC_CPU = 0,
    SNDRV_DMA_SYNC_DEVICE = 1,
}

use snd_dma_sync_mode::*;

const SNDRV_DMA_TYPE_UNKNOWN: c_int = 0;
const SNDRV_DMA_TYPE_CONTINUOUS: c_int = 1;
const SNDRV_DMA_TYPE_VMALLOC: c_int = 2;
const SNDRV_DMA_TYPE_DEV: c_int = 3;
const SNDRV_DMA_TYPE_DEV_WC: c_int = 4;
const SNDRV_DMA_TYPE_NONCONTIG: c_int = 5;
const SNDRV_DMA_TYPE_NONCOHERENT: c_int = 6;
const SNDRV_DMA_TYPE_DEV_SG: c_int = 7;
const SNDRV_DMA_TYPE_DEV_WC_SG: c_int = 8;
const SNDRV_DMA_TYPE_DEV_IRAM: c_int = 9;

#[repr(C)]
pub struct snd_dma_device {
    pub type_: c_int,
    pub dev: *mut device,
    pub dir: dma_data_direction,
    pub need_sync: bool,
}

#[repr(C)]
pub struct snd_dma_buffer {
    pub dev: snd_dma_device,
    pub area: *mut c_void,
    pub addr: dma_addr_t,
    pub bytes: size_t,
    pub private_data: *mut c_void,
}

#[repr(C)]
struct snd_malloc_ops {
    alloc: Option<unsafe extern "C" fn(dmab: *mut snd_dma_buffer, size: size_t) -> *mut c_void>,
    free: Option<unsafe extern "C" fn(dmab: *mut snd_dma_buffer)>,
    get_addr: Option<unsafe extern "C" fn(dmab: *mut snd_dma_buffer, offset: size_t) -> dma_addr_t>,
    get_page: Option<unsafe extern "C" fn(dmab: *mut snd_dma_buffer, offset: size_t) -> *mut page>,
    get_chunk_size:
        Option<unsafe extern "C" fn(dmab: *mut snd_dma_buffer, ofs: c_uint, size: c_uint) -> c_uint>,
    mmap: Option<unsafe extern "C" fn(dmab: *mut snd_dma_buffer, area: *mut vm_area_struct) -> c_int>,
    sync: Option<unsafe extern "C" fn(dmab: *mut snd_dma_buffer, mode: snd_dma_sync_mode)>,
}

/*
 * DEFAULT_GFP:
 * GFP_KERNEL | __GFP_RETRY_MAYFAIL (don't trigger OOM-killer) |
 * __GFP_NOWARN (no stack trace print - this call is non-critical)
 */
const DEFAULT_GFP: gfp_t = GFP_KERNEL | __GFP_RETRY_MAYFAIL | __GFP_NOWARN;

extern "C" {
    fn WARN_ON(condition: bool) -> bool;
    fn WARN_ON_ONCE(condition: bool) -> bool;
    fn PAGE_ALIGN(size: size_t) -> size_t;
    fn get_order(size: size_t) -> c_ulong;
    fn devres_alloc(
        release: unsafe extern "C" fn(dev: *mut device, res: *mut c_void),
        size: size_t,
        gfp: gfp_t,
    ) -> *mut c_void;
    fn devres_free(res: *mut c_void);
    fn devres_add(dev: *mut device, res: *mut c_void);
    fn alloc_pages_exact(size: size_t, gfp: gfp_t) -> *mut c_void;
    fn free_pages_exact(p: *mut c_void, size: size_t);
    fn virt_to_page(p: *mut c_void) -> *mut page;
    fn page_to_phys(page: *mut page) -> dma_addr_t;
    fn remap_pfn_range(
        area: *mut vm_area_struct,
        addr: c_ulong,
        pfn: c_ulong,
        size: c_ulong,
        prot: pgprot_t,
    ) -> c_int;
    fn vmalloc(size: size_t) -> *mut c_void;
    fn vfree(p: *mut c_void);
    fn remap_vmalloc_range(area: *mut vm_area_struct, addr: *mut c_void, pgoff: c_ulong) -> c_int;
    fn vmalloc_to_page(addr: *mut c_void) -> *mut page;
    fn of_gen_pool_get(np: *mut c_void, propname: *const u8, index: c_int) -> *mut gen_pool;
    fn gen_pool_dma_alloc_align(
        pool: *mut gen_pool,
        size: size_t,
        dma: *mut dma_addr_t,
        align: size_t,
    ) -> *mut c_void;
    fn gen_pool_free(pool: *mut gen_pool, addr: c_ulong, size: size_t);
    fn pgprot_writecombine(prot: pgprot_t) -> pgprot_t;
    fn dma_alloc_coherent(
        dev: *mut device,
        size: size_t,
        dma_handle: *mut dma_addr_t,
        flag: gfp_t,
    ) -> *mut c_void;
    fn dma_free_coherent(dev: *mut device, size: size_t, cpu_addr: *mut c_void, dma_handle: dma_addr_t);
    fn dma_mmap_coherent(
        dev: *mut device,
        vma: *mut vm_area_struct,
        cpu_addr: *mut c_void,
        dma_addr: dma_addr_t,
        size: size_t,
    ) -> c_int;
    fn dma_map_single(
        dev: *mut device,
        ptr: *mut c_void,
        size: size_t,
        dir: dma_data_direction,
    ) -> dma_addr_t;
    fn dma_mapping_error(dev: *mut device, dma_addr: dma_addr_t) -> bool;
    fn dma_unmap_single(dev: *mut device, addr: dma_addr_t, size: size_t, dir: dma_data_direction);
    fn dma_alloc_wc(
        dev: *mut device,
        size: size_t,
        dma_addr: *mut dma_addr_t,
        gfp: gfp_t,
    ) -> *mut c_void;
    fn dma_free_wc(dev: *mut device, size: size_t, cpu_addr: *mut c_void, dma_addr: dma_addr_t);
    fn dma_mmap_wc(
        dev: *mut device,
        vma: *mut vm_area_struct,
        cpu_addr: *mut c_void,
        dma_addr: dma_addr_t,
        size: size_t,
    ) -> c_int;
    fn dma_alloc_noncontiguous(
        dev: *mut device,
        size: size_t,
        dir: dma_data_direction,
        gfp: gfp_t,
        attrs: c_ulong,
    ) -> *mut sg_table;
    fn dma_free_noncontiguous(
        dev: *mut device,
        size: size_t,
        sgt: *mut sg_table,
        dir: dma_data_direction,
    );
    fn dma_vmap_noncontiguous(dev: *mut device, size: size_t, sgt: *mut sg_table) -> *mut c_void;
    fn dma_vunmap_noncontiguous(dev: *mut device, vaddr: *mut c_void);
    fn dma_mmap_noncontiguous(
        dev: *mut device,
        vma: *mut vm_area_struct,
        size: size_t,
        sgt: *mut sg_table,
    ) -> c_int;
    fn dma_need_sync(dev: *mut device, dma_addr: dma_addr_t) -> bool;
    fn sg_dma_address(sg: *mut scatterlist) -> dma_addr_t;
    fn invalidate_kernel_vmap_range(vaddr: *mut c_void, size: c_int);
    fn flush_kernel_vmap_range(vaddr: *mut c_void, size: c_int);
    fn dma_sync_sgtable_for_cpu(dev: *mut device, sgt: *mut sg_table, dir: dma_data_direction);
    fn dma_sync_sgtable_for_device(dev: *mut device, sgt: *mut sg_table, dir: dma_data_direction);
    fn __sg_page_iter_start(piter: *mut sg_page_iter, sglist: *mut scatterlist, nents: c_uint, pgoffset: c_ulong);
    fn __sg_page_iter_dma_next(piter: *mut sg_dma_page_iter) -> bool;
    fn __sg_page_iter_next(piter: *mut sg_page_iter) -> bool;
    fn sg_page_iter_dma_address(piter: *mut sg_dma_page_iter) -> dma_addr_t;
    fn sg_page_iter_page(piter: *mut sg_page_iter) -> *mut page;
    fn dma_alloc_noncoherent(
        dev: *mut device,
        size: size_t,
        dma_handle: *mut dma_addr_t,
        dir: dma_data_direction,
        gfp: gfp_t,
    ) -> *mut c_void;
    fn dma_free_noncoherent(
        dev: *mut device,
        size: size_t,
        cpu_addr: *mut c_void,
        dma_handle: dma_addr_t,
        dir: dma_data_direction,
    );
    fn vma_get_page_prot(area: *mut vm_area_struct) -> pgprot_t;
    fn dma_mmap_pages(
        dev: *mut device,
        vma: *mut vm_area_struct,
        size: size_t,
        page: *mut page,
    ) -> c_int;
    fn dma_sync_single_for_cpu(dev: *mut device, addr: dma_addr_t, size: size_t, dir: dma_data_direction);
    fn dma_sync_single_for_device(dev: *mut device, addr: dma_addr_t, size: size_t, dir: dma_data_direction);
    fn kvfree(p: *mut c_void);
    fn kfree(p: *mut c_void);
    fn kzalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kvzalloc(size: size_t, flags: gfp_t) -> *mut c_void;
    fn kvcalloc(n: size_t, size: size_t, flags: gfp_t) -> *mut c_void;
    fn page_address(page: *mut page) -> *mut c_void;
    fn sg_alloc_table_from_pages(
        sgt: *mut sg_table,
        pages: *mut *mut page,
        n_pages: c_uint,
        offset: c_ulong,
        size: c_ulong,
        gfp_mask: gfp_t,
    ) -> c_int;
    fn dma_map_sgtable(
        dev: *mut device,
        sgt: *mut sg_table,
        dir: dma_data_direction,
        attrs: c_ulong,
    ) -> c_int;
    fn dma_unmap_sgtable(
        dev: *mut device,
        sgt: *mut sg_table,
        dir: dma_data_direction,
        attrs: c_ulong,
    );
    fn sg_free_table(sgt: *mut sg_table);
    fn vmap(pages: *mut *mut page, count: c_uint, flags: c_ulong, prot: pgprot_t) -> *mut c_void;
    fn vunmap(addr: *mut c_void);
    fn vm_map_pages(vma: *mut vm_area_struct, pages: *mut *mut page, num: c_uint) -> c_int;
    fn snd_dma_alloc_pages(
        type_: c_int,
        device: *mut device,
        size: size_t,
        dmab: *mut snd_dma_buffer,
    ) -> c_int;
}

#[cfg(CONFIG_X86)]
extern "C" {
    fn set_memory_wc(addr: c_ulong, numpages: c_int) -> c_int;
    fn set_memory_wb(addr: c_ulong, numpages: c_int) -> c_int;
}

fn ALIGN_DOWN(x: c_uint, a: usize) -> c_uint {
    (x as usize & !(a - 1)) as c_uint
}

fn IS_ENABLED(_config: bool) -> bool {
    _config
}

const CONFIG_ZONE_DMA32: bool = cfg!(CONFIG_ZONE_DMA32);
const CONFIG_ZONE_DMA: bool = cfg!(CONFIG_ZONE_DMA);

unsafe fn ptr_add_void(p: *mut c_void, offset: size_t) -> *mut c_void {
    (p as *mut u8).add(offset) as *mut c_void
}

static mut SND_DMA_OPS: [*const snd_malloc_ops; 10] = [
    core::ptr::null(),
    &SND_DMA_CONTINUOUS_OPS,
    &SND_DMA_VMALLOC_OPS,
    #[cfg(CONFIG_HAS_DMA)]
    &SND_DMA_DEV_OPS,
    #[cfg(not(CONFIG_HAS_DMA))]
    core::ptr::null(),
    #[cfg(CONFIG_HAS_DMA)]
    &SND_DMA_WC_OPS,
    #[cfg(not(CONFIG_HAS_DMA))]
    core::ptr::null(),
    #[cfg(CONFIG_HAS_DMA)]
    &SND_DMA_NONCONTIG_OPS,
    #[cfg(not(CONFIG_HAS_DMA))]
    core::ptr::null(),
    #[cfg(CONFIG_HAS_DMA)]
    &SND_DMA_NONCOHERENT_OPS,
    #[cfg(not(CONFIG_HAS_DMA))]
    core::ptr::null(),
    #[cfg(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF))]
    &SND_DMA_SG_OPS,
    #[cfg(not(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF)))]
    core::ptr::null(),
    #[cfg(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF))]
    &SND_DMA_SG_OPS,
    #[cfg(not(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF)))]
    core::ptr::null(),
    #[cfg(all(CONFIG_HAS_DMA, CONFIG_GENERIC_ALLOCATOR))]
    &SND_DMA_IRAM_OPS,
    #[cfg(not(all(CONFIG_HAS_DMA, CONFIG_GENERIC_ALLOCATOR)))]
    core::ptr::null(),
];

unsafe fn snd_dma_get_ops(dmab: *mut snd_dma_buffer) -> *const snd_malloc_ops {
    if WARN_ON_ONCE(dmab.is_null()) {
        return core::ptr::null();
    }
    if WARN_ON_ONCE(
        (*dmab).dev.type_ <= SNDRV_DMA_TYPE_UNKNOWN
            || (*dmab).dev.type_ as usize >= SND_DMA_OPS.len(),
    ) {
        return core::ptr::null();
    }
    SND_DMA_OPS[(*dmab).dev.type_ as usize]
}

unsafe extern "C" fn __snd_dma_alloc_pages(dmab: *mut snd_dma_buffer, size: size_t) -> *mut c_void {
    let ops = snd_dma_get_ops(dmab);

    if WARN_ON_ONCE(ops.is_null() || (*ops).alloc.is_none()) {
        return core::ptr::null_mut();
    }
    ((*ops).alloc.unwrap())(dmab, size)
}

/**
 * snd_dma_alloc_dir_pages - allocate the buffer area according to the given
 *	type and direction
 * @type: the DMA buffer type
 * @device: the device pointer
 * @dir: DMA direction
 * @size: the buffer size to allocate
 * @dmab: buffer allocation record to store the allocated data
 *
 * Calls the memory-allocator function for the corresponding
 * buffer type.
 *
 * Return: Zero if the buffer with the given size is allocated successfully,
 * otherwise a negative value on error.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_dma_alloc_dir_pages(
    type_: c_int,
    device: *mut device,
    dir: dma_data_direction,
    mut size: size_t,
    dmab: *mut snd_dma_buffer,
) -> c_int {
    if WARN_ON(size == 0) {
        return -ENXIO;
    }
    if WARN_ON(dmab.is_null()) {
        return -ENXIO;
    }

    size = PAGE_ALIGN(size);
    (*dmab).dev.type_ = type_;
    (*dmab).dev.dev = device;
    (*dmab).dev.dir = dir;
    (*dmab).bytes = 0;
    (*dmab).addr = 0;
    (*dmab).private_data = core::ptr::null_mut();
    (*dmab).area = __snd_dma_alloc_pages(dmab, size);
    if (*dmab).area.is_null() {
        return -ENOMEM;
    }
    (*dmab).bytes = size;
    0
}

/**
 * snd_dma_alloc_pages_fallback - allocate the buffer area according to the given type with fallback
 * @type: the DMA buffer type
 * @device: the device pointer
 * @size: the buffer size to allocate
 * @dmab: buffer allocation record to store the allocated data
 *
 * Calls the memory-allocator function for the corresponding
 * buffer type.  When no space is left, this function reduces the size and
 * tries to allocate again.  The size actually allocated is stored in
 * res_size argument.
 *
 * Return: Zero if the buffer with the given size is allocated successfully,
 * otherwise a negative value on error.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_dma_alloc_pages_fallback(
    type_: c_int,
    device: *mut device,
    mut size: size_t,
    dmab: *mut snd_dma_buffer,
) -> c_int {
    let mut err: c_int;

    loop {
        err = snd_dma_alloc_pages(type_, device, size, dmab);
        if err >= 0 {
            break;
        }
        if err != -ENOMEM {
            return err;
        }
        if size <= PAGE_SIZE {
            return -ENOMEM;
        }
        size >>= 1;
        size = PAGE_SIZE << get_order(size);
    }
    if (*dmab).area.is_null() {
        return -ENOMEM;
    }
    0
}

/**
 * snd_dma_free_pages - release the allocated buffer
 * @dmab: the buffer allocation record to release
 *
 * Releases the allocated buffer via snd_dma_alloc_pages().
 */
#[no_mangle]
pub unsafe extern "C" fn snd_dma_free_pages(dmab: *mut snd_dma_buffer) {
    let ops = snd_dma_get_ops(dmab);

    if !ops.is_null() && (*ops).free.is_some() {
        ((*ops).free.unwrap())(dmab);
    }
}

/* called by devres */
unsafe extern "C" fn __snd_release_pages(_dev: *mut device, res: *mut c_void) {
    snd_dma_free_pages(res as *mut snd_dma_buffer);
}

/**
 * snd_devm_alloc_dir_pages - allocate the buffer and manage with devres
 * @dev: the device pointer
 * @type: the DMA buffer type
 * @dir: DMA direction
 * @size: the buffer size to allocate
 *
 * Allocate buffer pages depending on the given type and manage using devres.
 * The pages will be released automatically at the device removal.
 *
 * Unlike snd_dma_alloc_pages(), this function requires the real device pointer,
 * hence it can't work with SNDRV_DMA_TYPE_CONTINUOUS or
 * SNDRV_DMA_TYPE_VMALLOC type.
 *
 * Return: the snd_dma_buffer object at success, or NULL if failed
 */
#[no_mangle]
pub unsafe extern "C" fn snd_devm_alloc_dir_pages(
    dev: *mut device,
    type_: c_int,
    dir: dma_data_direction,
    size: size_t,
) -> *mut snd_dma_buffer {
    let dmab: *mut snd_dma_buffer;
    let err: c_int;

    if WARN_ON(type_ == SNDRV_DMA_TYPE_CONTINUOUS || type_ == SNDRV_DMA_TYPE_VMALLOC) {
        return core::ptr::null_mut();
    }

    dmab = devres_alloc(
        __snd_release_pages,
        core::mem::size_of::<snd_dma_buffer>(),
        GFP_KERNEL,
    ) as *mut snd_dma_buffer;
    if dmab.is_null() {
        return core::ptr::null_mut();
    }

    err = snd_dma_alloc_dir_pages(type_, dev, dir, size, dmab);
    if err < 0 {
        devres_free(dmab as *mut c_void);
        return core::ptr::null_mut();
    }

    devres_add(dev, dmab as *mut c_void);
    dmab
}

/**
 * snd_dma_buffer_mmap - perform mmap of the given DMA buffer
 * @dmab: buffer allocation information
 * @area: VM area information
 *
 * Return: zero if successful, or a negative error code
 */
#[no_mangle]
pub unsafe extern "C" fn snd_dma_buffer_mmap(
    dmab: *mut snd_dma_buffer,
    area: *mut vm_area_struct,
) -> c_int {
    let ops: *const snd_malloc_ops;

    if dmab.is_null() {
        return -ENOENT;
    }
    ops = snd_dma_get_ops(dmab);
    if !ops.is_null() && (*ops).mmap.is_some() {
        ((*ops).mmap.unwrap())(dmab, area)
    } else {
        -ENOENT
    }
}

#[cfg(CONFIG_HAS_DMA)]
/**
 * snd_dma_buffer_sync - sync DMA buffer between CPU and device
 * @dmab: buffer allocation information
 * @mode: sync mode
 */
#[no_mangle]
pub unsafe extern "C" fn snd_dma_buffer_sync(dmab: *mut snd_dma_buffer, mode: snd_dma_sync_mode) {
    let ops: *const snd_malloc_ops;

    if dmab.is_null() || !(*dmab).dev.need_sync {
        return;
    }
    ops = snd_dma_get_ops(dmab);
    if !ops.is_null() && (*ops).sync.is_some() {
        ((*ops).sync.unwrap())(dmab, mode);
    }
}

/**
 * snd_sgbuf_get_addr - return the physical address at the corresponding offset
 * @dmab: buffer allocation information
 * @offset: offset in the ring buffer
 *
 * Return: the physical address
 */
#[no_mangle]
pub unsafe extern "C" fn snd_sgbuf_get_addr(dmab: *mut snd_dma_buffer, offset: size_t) -> dma_addr_t {
    let ops = snd_dma_get_ops(dmab);

    if !ops.is_null() && (*ops).get_addr.is_some() {
        ((*ops).get_addr.unwrap())(dmab, offset)
    } else {
        (*dmab).addr + offset as dma_addr_t
    }
}

/**
 * snd_sgbuf_get_page - return the physical page at the corresponding offset
 * @dmab: buffer allocation information
 * @offset: offset in the ring buffer
 *
 * Return: the page pointer
 */
#[no_mangle]
pub unsafe extern "C" fn snd_sgbuf_get_page(dmab: *mut snd_dma_buffer, offset: size_t) -> *mut page {
    let ops = snd_dma_get_ops(dmab);

    if !ops.is_null() && (*ops).get_page.is_some() {
        ((*ops).get_page.unwrap())(dmab, offset)
    } else {
        virt_to_page(ptr_add_void((*dmab).area, offset))
    }
}

/**
 * snd_sgbuf_get_chunk_size - compute the max chunk size with continuous pages
 *	on sg-buffer
 * @dmab: buffer allocation information
 * @ofs: offset in the ring buffer
 * @size: the requested size
 *
 * Return: the chunk size
 */
#[no_mangle]
pub unsafe extern "C" fn snd_sgbuf_get_chunk_size(
    dmab: *mut snd_dma_buffer,
    ofs: c_uint,
    size: c_uint,
) -> c_uint {
    let ops = snd_dma_get_ops(dmab);

    if !ops.is_null() && (*ops).get_chunk_size.is_some() {
        ((*ops).get_chunk_size.unwrap())(dmab, ofs, size)
    } else {
        size
    }
}

/*
 * Continuous pages allocator
 */
unsafe extern "C" fn do_alloc_pages(
    dev: *mut device,
    size: size_t,
    addr: *mut dma_addr_t,
    wc: bool,
) -> *mut c_void {
    let mut p: *mut c_void;
    let mut gfp: gfp_t = GFP_KERNEL | __GFP_NORETRY | __GFP_NOWARN;

    loop {
        p = alloc_pages_exact(size, gfp);
        if p.is_null() {
            return core::ptr::null_mut();
        }
        *addr = page_to_phys(virt_to_page(p));
        if dev.is_null() {
            return p;
        }
        if ((*addr + size as dma_addr_t - 1) & !(*dev).coherent_dma_mask) != 0 {
            if IS_ENABLED(CONFIG_ZONE_DMA32) && (gfp & GFP_DMA32) == 0 {
                gfp |= GFP_DMA32;
                continue;
            }
            if IS_ENABLED(CONFIG_ZONE_DMA) && (gfp & GFP_DMA) == 0 {
                gfp = (gfp & !GFP_DMA32) | GFP_DMA;
                continue;
            }
        }
        break;
    }
    #[cfg(CONFIG_X86)]
    {
        if wc {
            set_memory_wc(p as c_ulong, (size >> PAGE_SHIFT) as c_int);
        }
    }
    p
}

unsafe extern "C" fn do_free_pages(p: *mut c_void, size: size_t, wc: bool) {
    #[cfg(CONFIG_X86)]
    {
        if wc {
            set_memory_wb(p as c_ulong, (size >> PAGE_SHIFT) as c_int);
        }
    }
    free_pages_exact(p, size);
}

unsafe extern "C" fn snd_dma_continuous_alloc(dmab: *mut snd_dma_buffer, size: size_t) -> *mut c_void {
    do_alloc_pages((*dmab).dev.dev, size, &mut (*dmab).addr, false)
}

unsafe extern "C" fn snd_dma_continuous_free(dmab: *mut snd_dma_buffer) {
    do_free_pages((*dmab).area, (*dmab).bytes, false);
}

unsafe extern "C" fn snd_dma_continuous_mmap(
    dmab: *mut snd_dma_buffer,
    area: *mut vm_area_struct,
) -> c_int {
    remap_pfn_range(
        area,
        (*area).vm_start,
        ((*dmab).addr >> PAGE_SHIFT) as c_ulong,
        (*area).vm_end - (*area).vm_start,
        (*area).vm_page_prot,
    )
}

static SND_DMA_CONTINUOUS_OPS: snd_malloc_ops = snd_malloc_ops {
    alloc: Some(snd_dma_continuous_alloc),
    free: Some(snd_dma_continuous_free),
    mmap: Some(snd_dma_continuous_mmap),
    get_addr: None,
    get_page: None,
    get_chunk_size: None,
    sync: None,
};

/*
 * VMALLOC allocator
 */
unsafe extern "C" fn snd_dma_vmalloc_alloc(_dmab: *mut snd_dma_buffer, size: size_t) -> *mut c_void {
    vmalloc(size)
}

unsafe extern "C" fn snd_dma_vmalloc_free(dmab: *mut snd_dma_buffer) {
    vfree((*dmab).area);
}

unsafe extern "C" fn snd_dma_vmalloc_mmap(
    dmab: *mut snd_dma_buffer,
    area: *mut vm_area_struct,
) -> c_int {
    remap_vmalloc_range(area, (*dmab).area, 0)
}

unsafe fn get_vmalloc_page_addr(dmab: *mut snd_dma_buffer, offset: size_t) -> dma_addr_t {
    page_to_phys(vmalloc_to_page(ptr_add_void((*dmab).area, offset)))
}

unsafe extern "C" fn snd_dma_vmalloc_get_addr(
    dmab: *mut snd_dma_buffer,
    offset: size_t,
) -> dma_addr_t {
    get_vmalloc_page_addr(dmab, offset) + (offset % PAGE_SIZE) as dma_addr_t
}

unsafe extern "C" fn snd_dma_vmalloc_get_page(
    dmab: *mut snd_dma_buffer,
    offset: size_t,
) -> *mut page {
    vmalloc_to_page(ptr_add_void((*dmab).area, offset))
}

unsafe extern "C" fn snd_dma_vmalloc_get_chunk_size(
    dmab: *mut snd_dma_buffer,
    ofs: c_uint,
    size: c_uint,
) -> c_uint {
    let mut start: c_uint;
    let end: c_uint;
    let mut addr: c_ulong;

    start = ALIGN_DOWN(ofs, PAGE_SIZE);
    end = ofs + size - 1; /* the last byte address */
    /* check page continuity */
    addr = get_vmalloc_page_addr(dmab, start as size_t) as c_ulong;
    loop {
        start += PAGE_SIZE as c_uint;
        if start > end {
            break;
        }
        addr += PAGE_SIZE as c_ulong;
        if get_vmalloc_page_addr(dmab, start as size_t) as c_ulong != addr {
            return start - ofs;
        }
    }
    /* ok, all on continuous pages */
    size
}

static SND_DMA_VMALLOC_OPS: snd_malloc_ops = snd_malloc_ops {
    alloc: Some(snd_dma_vmalloc_alloc),
    free: Some(snd_dma_vmalloc_free),
    mmap: Some(snd_dma_vmalloc_mmap),
    get_addr: Some(snd_dma_vmalloc_get_addr),
    get_page: Some(snd_dma_vmalloc_get_page),
    get_chunk_size: Some(snd_dma_vmalloc_get_chunk_size),
    sync: None,
};

#[cfg(all(CONFIG_HAS_DMA, CONFIG_GENERIC_ALLOCATOR))]
/*
 * IRAM allocator
 */
unsafe extern "C" fn snd_dma_iram_alloc(dmab: *mut snd_dma_buffer, size: size_t) -> *mut c_void {
    let dev = (*dmab).dev.dev;
    let mut pool: *mut gen_pool;
    let mut p: *mut c_void;

    if !(*dev).of_node.is_null() {
        pool = of_gen_pool_get((*dev).of_node, b"iram\0".as_ptr(), 0);
        /* Assign the pool into private_data field */
        (*dmab).private_data = pool as *mut c_void;

        p = gen_pool_dma_alloc_align(pool, size, &mut (*dmab).addr, PAGE_SIZE);
        if !p.is_null() {
            return p;
        }
    }

    /*
     * Internal memory might have limited size and no enough space,
     * so if we fail to malloc, try to fetch memory traditionally.
     */
    (*dmab).dev.type_ = SNDRV_DMA_TYPE_DEV;
    __snd_dma_alloc_pages(dmab, size)
}

#[cfg(all(CONFIG_HAS_DMA, CONFIG_GENERIC_ALLOCATOR))]
unsafe extern "C" fn snd_dma_iram_free(dmab: *mut snd_dma_buffer) {
    let pool = (*dmab).private_data as *mut gen_pool;

    if !pool.is_null() && !(*dmab).area.is_null() {
        gen_pool_free(pool, (*dmab).area as c_ulong, (*dmab).bytes);
    }
}

#[cfg(all(CONFIG_HAS_DMA, CONFIG_GENERIC_ALLOCATOR))]
unsafe extern "C" fn snd_dma_iram_mmap(
    dmab: *mut snd_dma_buffer,
    area: *mut vm_area_struct,
) -> c_int {
    (*area).vm_page_prot = pgprot_writecombine((*area).vm_page_prot);
    remap_pfn_range(
        area,
        (*area).vm_start,
        ((*dmab).addr >> PAGE_SHIFT) as c_ulong,
        (*area).vm_end - (*area).vm_start,
        (*area).vm_page_prot,
    )
}

#[cfg(all(CONFIG_HAS_DMA, CONFIG_GENERIC_ALLOCATOR))]
static SND_DMA_IRAM_OPS: snd_malloc_ops = snd_malloc_ops {
    alloc: Some(snd_dma_iram_alloc),
    free: Some(snd_dma_iram_free),
    mmap: Some(snd_dma_iram_mmap),
    get_addr: None,
    get_page: None,
    get_chunk_size: None,
    sync: None,
};

#[cfg(CONFIG_HAS_DMA)]
/*
 * Coherent device pages allocator
 */
unsafe extern "C" fn snd_dma_dev_alloc(dmab: *mut snd_dma_buffer, size: size_t) -> *mut c_void {
    dma_alloc_coherent((*dmab).dev.dev, size, &mut (*dmab).addr, DEFAULT_GFP)
}

#[cfg(CONFIG_HAS_DMA)]
unsafe extern "C" fn snd_dma_dev_free(dmab: *mut snd_dma_buffer) {
    dma_free_coherent((*dmab).dev.dev, (*dmab).bytes, (*dmab).area, (*dmab).addr);
}

#[cfg(CONFIG_HAS_DMA)]
unsafe extern "C" fn snd_dma_dev_mmap(dmab: *mut snd_dma_buffer, area: *mut vm_area_struct) -> c_int {
    dma_mmap_coherent((*dmab).dev.dev, area, (*dmab).area, (*dmab).addr, (*dmab).bytes)
}

#[cfg(CONFIG_HAS_DMA)]
static SND_DMA_DEV_OPS: snd_malloc_ops = snd_malloc_ops {
    alloc: Some(snd_dma_dev_alloc),
    free: Some(snd_dma_dev_free),
    mmap: Some(snd_dma_dev_mmap),
    get_addr: None,
    get_page: None,
    get_chunk_size: None,
    sync: None,
};

#[cfg(CONFIG_HAS_DMA)]
/*
 * Write-combined pages
 */
#[cfg(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF))]
/* x86-specific allocations */
unsafe extern "C" fn snd_dma_wc_alloc(dmab: *mut snd_dma_buffer, size: size_t) -> *mut c_void {
    let p = do_alloc_pages((*dmab).dev.dev, size, &mut (*dmab).addr, true);

    if p.is_null() {
        return core::ptr::null_mut();
    }
    (*dmab).addr = dma_map_single((*dmab).dev.dev, p, size, DMA_BIDIRECTIONAL);
    if dma_mapping_error((*dmab).dev.dev, (*dmab).addr) {
        do_free_pages((*dmab).area, size, true);
        return core::ptr::null_mut();
    }
    p
}

#[cfg(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF))]
unsafe extern "C" fn snd_dma_wc_free(dmab: *mut snd_dma_buffer) {
    dma_unmap_single((*dmab).dev.dev, (*dmab).addr, (*dmab).bytes, DMA_BIDIRECTIONAL);
    do_free_pages((*dmab).area, (*dmab).bytes, true);
}

#[cfg(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF))]
unsafe extern "C" fn snd_dma_wc_mmap(dmab: *mut snd_dma_buffer, area: *mut vm_area_struct) -> c_int {
    (*area).vm_page_prot = pgprot_writecombine((*area).vm_page_prot);
    dma_mmap_coherent((*dmab).dev.dev, area, (*dmab).area, (*dmab).addr, (*dmab).bytes)
}

#[cfg(all(CONFIG_HAS_DMA, not(CONFIG_SND_DMA_SGBUF)))]
unsafe extern "C" fn snd_dma_wc_alloc(dmab: *mut snd_dma_buffer, size: size_t) -> *mut c_void {
    dma_alloc_wc((*dmab).dev.dev, size, &mut (*dmab).addr, DEFAULT_GFP)
}

#[cfg(all(CONFIG_HAS_DMA, not(CONFIG_SND_DMA_SGBUF)))]
unsafe extern "C" fn snd_dma_wc_free(dmab: *mut snd_dma_buffer) {
    dma_free_wc((*dmab).dev.dev, (*dmab).bytes, (*dmab).area, (*dmab).addr);
}

#[cfg(all(CONFIG_HAS_DMA, not(CONFIG_SND_DMA_SGBUF)))]
unsafe extern "C" fn snd_dma_wc_mmap(dmab: *mut snd_dma_buffer, area: *mut vm_area_struct) -> c_int {
    dma_mmap_wc((*dmab).dev.dev, area, (*dmab).area, (*dmab).addr, (*dmab).bytes)
}

#[cfg(CONFIG_HAS_DMA)]
static SND_DMA_WC_OPS: snd_malloc_ops = snd_malloc_ops {
    alloc: Some(snd_dma_wc_alloc),
    free: Some(snd_dma_wc_free),
    mmap: Some(snd_dma_wc_mmap),
    get_addr: None,
    get_page: None,
    get_chunk_size: None,
    sync: None,
};

#[cfg(CONFIG_HAS_DMA)]
/*
 * Non-contiguous pages allocator
 */
unsafe extern "C" fn snd_dma_noncontig_alloc(dmab: *mut snd_dma_buffer, size: size_t) -> *mut c_void {
    let sgt: *mut sg_table;
    let p: *mut c_void;

    sgt = dma_alloc_noncontiguous((*dmab).dev.dev, size, (*dmab).dev.dir, DEFAULT_GFP, 0);
    if sgt.is_null() {
        return core::ptr::null_mut();
    }

    (*dmab).dev.need_sync = dma_need_sync((*dmab).dev.dev, sg_dma_address((*sgt).sgl));
    p = dma_vmap_noncontiguous((*dmab).dev.dev, size, sgt);
    if !p.is_null() {
        (*dmab).private_data = sgt as *mut c_void;
        /* store the first page address for convenience */
        (*dmab).addr = snd_sgbuf_get_addr(dmab, 0);
    } else {
        dma_free_noncontiguous((*dmab).dev.dev, size, sgt, (*dmab).dev.dir);
    }
    p
}

#[cfg(CONFIG_HAS_DMA)]
unsafe extern "C" fn snd_dma_noncontig_free(dmab: *mut snd_dma_buffer) {
    dma_vunmap_noncontiguous((*dmab).dev.dev, (*dmab).area);
    dma_free_noncontiguous(
        (*dmab).dev.dev,
        (*dmab).bytes,
        (*dmab).private_data as *mut sg_table,
        (*dmab).dev.dir,
    );
}

#[cfg(CONFIG_HAS_DMA)]
unsafe extern "C" fn snd_dma_noncontig_mmap(dmab: *mut snd_dma_buffer, area: *mut vm_area_struct) -> c_int {
    dma_mmap_noncontiguous(
        (*dmab).dev.dev,
        area,
        (*dmab).bytes,
        (*dmab).private_data as *mut sg_table,
    )
}

#[cfg(CONFIG_HAS_DMA)]
unsafe extern "C" fn snd_dma_noncontig_sync(dmab: *mut snd_dma_buffer, mode: snd_dma_sync_mode) {
    if mode == SNDRV_DMA_SYNC_CPU {
        if (*dmab).dev.dir == DMA_TO_DEVICE {
            return;
        }
        invalidate_kernel_vmap_range((*dmab).area, (*dmab).bytes as c_int);
        dma_sync_sgtable_for_cpu((*dmab).dev.dev, (*dmab).private_data as *mut sg_table, (*dmab).dev.dir);
    } else {
        if (*dmab).dev.dir == DMA_FROM_DEVICE {
            return;
        }
        flush_kernel_vmap_range((*dmab).area, (*dmab).bytes as c_int);
        dma_sync_sgtable_for_device((*dmab).dev.dev, (*dmab).private_data as *mut sg_table, (*dmab).dev.dir);
    }
}

#[cfg(CONFIG_HAS_DMA)]
unsafe fn snd_dma_noncontig_iter_set(
    dmab: *mut snd_dma_buffer,
    piter: *mut sg_page_iter,
    offset: size_t,
) {
    let sgt = (*dmab).private_data as *mut sg_table;

    __sg_page_iter_start(piter, (*sgt).sgl, (*sgt).orig_nents, (offset >> PAGE_SHIFT) as c_ulong);
}

#[cfg(CONFIG_HAS_DMA)]
unsafe extern "C" fn snd_dma_noncontig_get_addr(dmab: *mut snd_dma_buffer, offset: size_t) -> dma_addr_t {
    let mut iter: sg_dma_page_iter = core::mem::zeroed();

    snd_dma_noncontig_iter_set(dmab, &mut iter.base, offset);
    __sg_page_iter_dma_next(&mut iter);
    sg_page_iter_dma_address(&mut iter) + (offset % PAGE_SIZE) as dma_addr_t
}

#[cfg(CONFIG_HAS_DMA)]
unsafe extern "C" fn snd_dma_noncontig_get_page(dmab: *mut snd_dma_buffer, offset: size_t) -> *mut page {
    let mut iter: sg_page_iter = core::mem::zeroed();

    snd_dma_noncontig_iter_set(dmab, &mut iter, offset);
    __sg_page_iter_next(&mut iter);
    sg_page_iter_page(&mut iter)
}

#[cfg(CONFIG_HAS_DMA)]
unsafe extern "C" fn snd_dma_noncontig_get_chunk_size(
    dmab: *mut snd_dma_buffer,
    ofs: c_uint,
    size: c_uint,
) -> c_uint {
    let mut iter: sg_dma_page_iter = core::mem::zeroed();
    let mut start: c_uint;
    let end: c_uint;
    let mut addr: c_ulong;

    start = ALIGN_DOWN(ofs, PAGE_SIZE);
    end = ofs + size - 1; /* the last byte address */
    snd_dma_noncontig_iter_set(dmab, &mut iter.base, start as size_t);
    if !__sg_page_iter_dma_next(&mut iter) {
        return 0;
    }
    /* check page continuity */
    addr = sg_page_iter_dma_address(&mut iter) as c_ulong;
    loop {
        start += PAGE_SIZE as c_uint;
        if start > end {
            break;
        }
        addr += PAGE_SIZE as c_ulong;
        if !__sg_page_iter_dma_next(&mut iter) || sg_page_iter_dma_address(&mut iter) as c_ulong != addr {
            return start - ofs;
        }
    }
    /* ok, all on continuous pages */
    size
}

#[cfg(CONFIG_HAS_DMA)]
static SND_DMA_NONCONTIG_OPS: snd_malloc_ops = snd_malloc_ops {
    alloc: Some(snd_dma_noncontig_alloc),
    free: Some(snd_dma_noncontig_free),
    mmap: Some(snd_dma_noncontig_mmap),
    sync: Some(snd_dma_noncontig_sync),
    get_addr: Some(snd_dma_noncontig_get_addr),
    get_page: Some(snd_dma_noncontig_get_page),
    get_chunk_size: Some(snd_dma_noncontig_get_chunk_size),
};

#[cfg(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF))]
/* Fallback SG-buffer allocations for x86 */
#[repr(C)]
struct snd_dma_sg_fallback {
    sgt: sg_table, /* used by get_addr - must be the first item */
    count: size_t,
    pages: *mut *mut page,
    npages: *mut c_uint,
}

#[cfg(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF))]
unsafe fn __snd_dma_sg_fallback_free(dmab: *mut snd_dma_buffer, sgbuf: *mut snd_dma_sg_fallback) {
    let wc = (*dmab).dev.type_ == SNDRV_DMA_TYPE_DEV_WC_SG;
    let mut i: size_t;
    let mut size: size_t;

    if !(*sgbuf).pages.is_null() && !(*sgbuf).npages.is_null() {
        i = 0;
        while i < (*sgbuf).count {
            size = *(*sgbuf).npages.add(i) as size_t;
            if size == 0 {
                break;
            }
            do_free_pages(page_address(*(*sgbuf).pages.add(i)), size << PAGE_SHIFT, wc);
            i += size;
        }
    }
    kvfree((*sgbuf).pages as *mut c_void);
    kvfree((*sgbuf).npages as *mut c_void);
    kfree(sgbuf as *mut c_void);
}

#[cfg(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF))]
/* fallback manual S/G buffer allocations */
unsafe extern "C" fn snd_dma_sg_fallback_alloc(dmab: *mut snd_dma_buffer, mut size: size_t) -> *mut c_void {
    let wc = (*dmab).dev.type_ == SNDRV_DMA_TYPE_DEV_WC_SG;
    let sgbuf: *mut snd_dma_sg_fallback;
    let mut pagep: *mut *mut page;
    let mut curp: *mut page;
    let mut chunk: size_t;
    let mut addr: dma_addr_t = 0;
    let mut idx: c_uint;
    let mut npages: c_uint;
    let mut p: *mut c_void;

    sgbuf = kzalloc(core::mem::size_of::<snd_dma_sg_fallback>(), GFP_KERNEL) as *mut snd_dma_sg_fallback;
    if sgbuf.is_null() {
        return core::ptr::null_mut();
    }
    size = PAGE_ALIGN(size);
    (*sgbuf).count = size >> PAGE_SHIFT;
    (*sgbuf).pages =
        kvzalloc(core::mem::size_of::<*mut page>() * (*sgbuf).count, GFP_KERNEL) as *mut *mut page;
    (*sgbuf).npages = kvcalloc((*sgbuf).count, core::mem::size_of::<c_uint>(), GFP_KERNEL) as *mut c_uint;
    if (*sgbuf).pages.is_null() || (*sgbuf).npages.is_null() {
        __snd_dma_sg_fallback_free(dmab, sgbuf);
        return core::ptr::null_mut();
    }

    pagep = (*sgbuf).pages;
    chunk = size;
    idx = 0;
    while size > 0 {
        chunk = if size < chunk { size } else { chunk };
        p = do_alloc_pages((*dmab).dev.dev, chunk, &mut addr, wc);
        if p.is_null() {
            if chunk <= PAGE_SIZE {
                __snd_dma_sg_fallback_free(dmab, sgbuf);
                return core::ptr::null_mut();
            }
            chunk >>= 1;
            chunk = PAGE_SIZE << get_order(chunk);
            continue;
        }

        size -= chunk;
        /* fill pages */
        npages = (chunk >> PAGE_SHIFT) as c_uint;
        *(*sgbuf).npages.add(idx as usize) = npages;
        idx += npages;
        curp = virt_to_page(p);
        while npages != 0 {
            *pagep = curp;
            pagep = pagep.add(1);
            curp = curp.add(1);
            npages -= 1;
        }
    }

    if sg_alloc_table_from_pages(
        &mut (*sgbuf).sgt,
        (*sgbuf).pages,
        (*sgbuf).count as c_uint,
        0,
        ((*sgbuf).count << PAGE_SHIFT) as c_ulong,
        GFP_KERNEL,
    ) != 0
    {
        __snd_dma_sg_fallback_free(dmab, sgbuf);
        return core::ptr::null_mut();
    }

    if dma_map_sgtable((*dmab).dev.dev, &mut (*sgbuf).sgt, DMA_BIDIRECTIONAL, 0) != 0 {
        sg_free_table(&mut (*sgbuf).sgt);
        __snd_dma_sg_fallback_free(dmab, sgbuf);
        return core::ptr::null_mut();
    }

    p = vmap((*sgbuf).pages, (*sgbuf).count as c_uint, VM_MAP, PAGE_KERNEL);
    if p.is_null() {
        dma_unmap_sgtable((*dmab).dev.dev, &mut (*sgbuf).sgt, DMA_BIDIRECTIONAL, 0);
        sg_free_table(&mut (*sgbuf).sgt);
        __snd_dma_sg_fallback_free(dmab, sgbuf);
        return core::ptr::null_mut();
    }

    (*dmab).private_data = sgbuf as *mut c_void;
    /* store the first page address for convenience */
    (*dmab).addr = snd_sgbuf_get_addr(dmab, 0);
    p
}

#[cfg(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF))]
unsafe extern "C" fn snd_dma_sg_fallback_free(dmab: *mut snd_dma_buffer) {
    let sgbuf = (*dmab).private_data as *mut snd_dma_sg_fallback;

    vunmap((*dmab).area);
    dma_unmap_sgtable((*dmab).dev.dev, &mut (*sgbuf).sgt, DMA_BIDIRECTIONAL, 0);
    sg_free_table(&mut (*sgbuf).sgt);
    __snd_dma_sg_fallback_free(dmab, (*dmab).private_data as *mut snd_dma_sg_fallback);
}

#[cfg(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF))]
unsafe extern "C" fn snd_dma_sg_fallback_mmap(dmab: *mut snd_dma_buffer, area: *mut vm_area_struct) -> c_int {
    let sgbuf = (*dmab).private_data as *mut snd_dma_sg_fallback;

    if (*dmab).dev.type_ == SNDRV_DMA_TYPE_DEV_WC_SG {
        (*area).vm_page_prot = pgprot_writecombine((*area).vm_page_prot);
    }
    vm_map_pages(area, (*sgbuf).pages, (*sgbuf).count as c_uint)
}

#[cfg(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF))]
unsafe extern "C" fn snd_dma_sg_alloc(dmab: *mut snd_dma_buffer, size: size_t) -> *mut c_void {
    let type_ = (*dmab).dev.type_;
    let p: *mut c_void;

    /* try the standard DMA API allocation at first */
    if type_ == SNDRV_DMA_TYPE_DEV_WC_SG {
        (*dmab).dev.type_ = SNDRV_DMA_TYPE_DEV_WC;
    } else {
        (*dmab).dev.type_ = SNDRV_DMA_TYPE_DEV;
    }
    p = __snd_dma_alloc_pages(dmab, size);
    if !p.is_null() {
        return p;
    }

    (*dmab).dev.type_ = type_; /* restore the type */
    snd_dma_sg_fallback_alloc(dmab, size)
}

#[cfg(all(CONFIG_HAS_DMA, CONFIG_SND_DMA_SGBUF))]
static SND_DMA_SG_OPS: snd_malloc_ops = snd_malloc_ops {
    alloc: Some(snd_dma_sg_alloc),
    free: Some(snd_dma_sg_fallback_free),
    mmap: Some(snd_dma_sg_fallback_mmap),
    /* reuse noncontig helper */
    get_addr: Some(snd_dma_noncontig_get_addr),
    /* reuse vmalloc helpers */
    get_page: Some(snd_dma_vmalloc_get_page),
    get_chunk_size: Some(snd_dma_vmalloc_get_chunk_size),
    sync: None,
};

#[cfg(CONFIG_HAS_DMA)]
/*
 * Non-coherent pages allocator
 */
unsafe extern "C" fn snd_dma_noncoherent_alloc(dmab: *mut snd_dma_buffer, size: size_t) -> *mut c_void {
    let p: *mut c_void;

    p = dma_alloc_noncoherent((*dmab).dev.dev, size, &mut (*dmab).addr, (*dmab).dev.dir, DEFAULT_GFP);
    if !p.is_null() {
        (*dmab).dev.need_sync = dma_need_sync((*dmab).dev.dev, (*dmab).addr);
    }
    p
}

#[cfg(CONFIG_HAS_DMA)]
unsafe extern "C" fn snd_dma_noncoherent_free(dmab: *mut snd_dma_buffer) {
    dma_free_noncoherent(
        (*dmab).dev.dev,
        (*dmab).bytes,
        (*dmab).area,
        (*dmab).addr,
        (*dmab).dev.dir,
    );
}

#[cfg(CONFIG_HAS_DMA)]
unsafe extern "C" fn snd_dma_noncoherent_mmap(dmab: *mut snd_dma_buffer, area: *mut vm_area_struct) -> c_int {
    (*area).vm_page_prot = vma_get_page_prot(area);
    dma_mmap_pages(
        (*dmab).dev.dev,
        area,
        (*area).vm_end - (*area).vm_start,
        virt_to_page((*dmab).area),
    )
}

#[cfg(CONFIG_HAS_DMA)]
unsafe extern "C" fn snd_dma_noncoherent_sync(dmab: *mut snd_dma_buffer, mode: snd_dma_sync_mode) {
    if mode == SNDRV_DMA_SYNC_CPU {
        if (*dmab).dev.dir != DMA_TO_DEVICE {
            dma_sync_single_for_cpu((*dmab).dev.dev, (*dmab).addr, (*dmab).bytes, (*dmab).dev.dir);
        }
    } else if (*dmab).dev.dir != DMA_FROM_DEVICE {
        dma_sync_single_for_device((*dmab).dev.dev, (*dmab).addr, (*dmab).bytes, (*dmab).dev.dir);
    }
}

#[cfg(CONFIG_HAS_DMA)]
static SND_DMA_NONCOHERENT_OPS: snd_malloc_ops = snd_malloc_ops {
    alloc: Some(snd_dma_noncoherent_alloc),
    free: Some(snd_dma_noncoherent_free),
    mmap: Some(snd_dma_noncoherent_mmap),
    sync: Some(snd_dma_noncoherent_sync),
    get_addr: None,
    get_page: None,
    get_chunk_size: None,
};

/*
 * Entry points
 *
 * The C source uses a designated-initializer table:
 * [SNDRV_DMA_TYPE_*] = &snd_dma_*_ops, with CONFIG_HAS_DMA,
 * CONFIG_SND_DMA_SGBUF, and CONFIG_GENERIC_ALLOCATOR gated entries.
 * The equivalent Rust table is SND_DMA_OPS above.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
