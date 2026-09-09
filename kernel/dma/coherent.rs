// SPDX-License-Identifier: GPL-2.0
/*
 * Coherent per-device memory handling.
 * Borrowed from i386
 */

#[repr(C)]
pub struct dma_coherent_mem {
    pub virt_base: *mut core::ffi::c_void,
    pub device_base: dma_addr_t,
    pub pfn_base: c_ulong,
    pub size: c_int,
    pub bitmap: *mut c_ulong,
    pub spinlock: spinlock_t,
    pub use_dev_dma_pfn_offset: bool,
}

#[inline]
unsafe fn dev_get_coherent_memory(dev: *mut device) -> *mut dma_coherent_mem {
    if !dev.is_null() && !(*dev).dma_mem.is_null() { (*dev).dma_mem } else { core::ptr::null_mut() }
}

#[inline]
unsafe fn dma_get_device_base(dev: *mut device, mem: *mut dma_coherent_mem) -> dma_addr_t {
    if (*mem).use_dev_dma_pfn_offset { phys_to_dma(dev, PFN_PHYS((*mem).pfn_base)) } else { (*mem).device_base }
}

unsafe fn dma_init_coherent_memory(phys_addr: phys_addr_t, device_addr: dma_addr_t, size: usize, use_dma_pfn_offset: bool) -> *mut dma_coherent_mem {
    let pages: c_int = (size >> PAGE_SHIFT) as c_int;
    let mem_base: *mut core::ffi::c_void = memremap(phys_addr, size, MEMREMAP_WC);
    if size == 0 { return ERR_PTR(-EINVAL); }
    if mem_base.is_null() { return ERR_PTR(-EINVAL); }

    let dma_mem = kzalloc_obj::<dma_coherent_mem>();
    if dma_mem.is_null() { memunmap(mem_base); pr_err!("Reserved memory: failed to init DMA memory pool at %pa, size %zu KiB\n", &phys_addr, size / SZ_1K); return ERR_PTR(-ENOMEM); }
    (*dma_mem).bitmap = bitmap_zalloc(pages as usize, GFP_KERNEL);
    if (*dma_mem).bitmap.is_null() { kfree(dma_mem); memunmap(mem_base); pr_err!("Reserved memory: failed to init DMA memory pool at %pa, size %zu KiB\n", &phys_addr, size / SZ_1K); return ERR_PTR(-ENOMEM); }

    (*dma_mem).virt_base = mem_base;
    (*dma_mem).device_base = device_addr;
    (*dma_mem).pfn_base = PFN_DOWN(phys_addr);
    (*dma_mem).size = pages;
    (*dma_mem).use_dev_dma_pfn_offset = use_dma_pfn_offset;
    spin_lock_init(&mut (*dma_mem).spinlock);
    dma_mem
}

unsafe fn _dma_release_coherent_memory(mem: *mut dma_coherent_mem) {
    if mem.is_null() { return; }
    memunmap((*mem).virt_base);
    bitmap_free((*mem).bitmap);
    kfree(mem);
}

unsafe fn dma_assign_coherent_memory(dev: *mut device, mem: *mut dma_coherent_mem) -> c_int {
    if dev.is_null() { return -ENODEV; }
    if !(*dev).dma_mem.is_null() { return -EBUSY; }
    (*dev).dma_mem = mem;
    0
}

pub unsafe fn dma_declare_coherent_memory(dev: *mut device, phys_addr: phys_addr_t, device_addr: dma_addr_t, size: usize) -> c_int {
    let mem = dma_init_coherent_memory(phys_addr, device_addr, size, false);
    if IS_ERR(mem) { return PTR_ERR(mem); }
    let ret = dma_assign_coherent_memory(dev, mem);
    if ret != 0 { _dma_release_coherent_memory(mem); }
    ret
}

pub unsafe fn dma_release_coherent_memory(dev: *mut device) {
    if !dev.is_null() { _dma_release_coherent_memory((*dev).dma_mem); (*dev).dma_mem = core::ptr::null_mut(); }
}

unsafe fn __dma_alloc_from_coherent(dev: *mut device, mem: *mut dma_coherent_mem, size: isize, dma_handle: *mut dma_addr_t) -> *mut core::ffi::c_void {
    let order = get_order(size);
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*mem).spinlock, &mut flags);
    if size as usize > (((*mem).size as dma_addr_t) << PAGE_SHIFT) as usize { spin_unlock_irqrestore(&mut (*mem).spinlock, flags); return core::ptr::null_mut(); }
    let pageno = bitmap_find_free_region((*mem).bitmap, (*mem).size, order);
    if pageno < 0 { spin_unlock_irqrestore(&mut (*mem).spinlock, flags); return core::ptr::null_mut(); }
    *dma_handle = dma_get_device_base(dev, mem) + ((pageno as dma_addr_t) << PAGE_SHIFT);
    let ret = ((*mem).virt_base as *mut u8).add(((pageno as dma_addr_t) << PAGE_SHIFT) as usize) as *mut core::ffi::c_void;
    spin_unlock_irqrestore(&mut (*mem).spinlock, flags);
    memset(ret, 0, size as usize);
    ret
}

pub unsafe fn dma_alloc_from_dev_coherent(dev: *mut device, size: isize, dma_handle: *mut dma_addr_t, ret: *mut *mut core::ffi::c_void) -> c_int {
    let mem = dev_get_coherent_memory(dev);
    if mem.is_null() { return 0; }
    *ret = __dma_alloc_from_coherent(dev, mem, size, dma_handle);
    1
}

unsafe fn __dma_release_from_coherent(mem: *mut dma_coherent_mem, order: c_int, vaddr: *mut core::ffi::c_void) -> c_int {
    let base = (*mem).virt_base as usize;
    let addr = vaddr as usize;
    if !mem.is_null() && addr >= base && addr < base + (((*mem).size as dma_addr_t) << PAGE_SHIFT) as usize {
        let page = ((addr - base) >> PAGE_SHIFT) as c_int;
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut (*mem).spinlock, &mut flags);
        bitmap_release_region((*mem).bitmap, page, order);
        spin_unlock_irqrestore(&mut (*mem).spinlock, flags);
        return 1;
    }
    0
}

pub unsafe fn dma_release_from_dev_coherent(dev: *mut device, order: c_int, vaddr: *mut core::ffi::c_void) -> c_int {
    __dma_release_from_coherent(dev_get_coherent_memory(dev), order, vaddr)
}

unsafe fn __dma_mmap_from_coherent(mem: *mut dma_coherent_mem, vma: *mut vm_area_struct, vaddr: *mut core::ffi::c_void, size: usize, ret: *mut c_int) -> c_int {
    let base = (*mem).virt_base as usize;
    let addr = vaddr as usize;
    if !mem.is_null() && addr >= base && addr + size <= base + (((*mem).size as dma_addr_t) << PAGE_SHIFT) as usize {
        let pgoff_start = vma_start_pgoff(vma);
        let pgoff_end = vma_end_pgoff(vma);
        let start = ((addr - base) >> PAGE_SHIFT) as c_int;
        let user_count = vma_pages(vma);
        let count = (PAGE_ALIGN(size) >> PAGE_SHIFT) as c_int;
        *ret = -ENXIO;
        if pgoff_start < count && pgoff_end <= count {
            let pfn = (*mem).pfn_base + start as c_ulong + pgoff_start;
            *ret = remap_pfn_range(vma, (*vma).vm_start, pfn, user_count << PAGE_SHIFT, (*vma).vm_page_prot);
        }
        return 1;
    }
    0
}

pub unsafe fn dma_mmap_from_dev_coherent(dev: *mut device, vma: *mut vm_area_struct, vaddr: *mut core::ffi::c_void, size: usize, ret: *mut c_int) -> c_int {
    __dma_mmap_from_coherent(dev_get_coherent_memory(dev), vma, vaddr, size, ret)
}

// CONFIG_DMA_GLOBAL_POOL declarations and reserved-memory integration are build-time Linux configuration.
#[cfg(CONFIG_DMA_GLOBAL_POOL)]
static mut dma_coherent_default_memory: *mut dma_coherent_mem = core::ptr::null_mut();

#[cfg(CONFIG_DMA_GLOBAL_POOL)]
pub unsafe fn dma_alloc_from_global_coherent(dev: *mut device, size: isize, dma_handle: *mut dma_addr_t) -> *mut core::ffi::c_void {
    if dma_coherent_default_memory.is_null() { return core::ptr::null_mut(); }
    __dma_alloc_from_coherent(dev, dma_coherent_default_memory, size, dma_handle)
}

#[cfg(CONFIG_DMA_GLOBAL_POOL)]
pub unsafe fn dma_release_from_global_coherent(order: c_int, vaddr: *mut core::ffi::c_void) -> c_int {
    if dma_coherent_default_memory.is_null() { return 0; }
    __dma_release_from_coherent(dma_coherent_default_memory, order, vaddr)
}

#[cfg(CONFIG_DMA_GLOBAL_POOL)]
pub unsafe fn dma_mmap_from_global_coherent(vma: *mut vm_area_struct, vaddr: *mut core::ffi::c_void, size: usize, ret: *mut c_int) -> c_int {
    if dma_coherent_default_memory.is_null() { return 0; }
    __dma_mmap_from_coherent(dma_coherent_default_memory, vma, vaddr, size, ret)
}

#[cfg(CONFIG_DMA_GLOBAL_POOL)]
pub unsafe fn dma_init_global_coherent(phys_addr: phys_addr_t, size: usize) -> c_int {
    let mem = dma_init_coherent_memory(phys_addr, phys_addr, size, true);
    if IS_ERR(mem) { return PTR_ERR(mem); }
    dma_coherent_default_memory = mem;
    pr_info!("DMA: default coherent area is set\n");
    0
}

// Support for reserved memory regions defined in device tree.
#[cfg(CONFIG_OF_RESERVED_MEM)]
#[cfg(CONFIG_DMA_GLOBAL_POOL)]
static mut dma_reserved_default_memory_base: phys_addr_t = 0;
#[cfg(CONFIG_OF_RESERVED_MEM)]
#[cfg(CONFIG_DMA_GLOBAL_POOL)]
static mut dma_reserved_default_memory_size: phys_addr_t = 0;

#[cfg(CONFIG_OF_RESERVED_MEM)]
unsafe fn rmem_dma_device_init(rmem: *mut reserved_mem, dev: *mut device) -> c_int {
    let mut mem = (*rmem).priv_data as *mut dma_coherent_mem;
    if mem.is_null() {
        mem = dma_init_coherent_memory((*rmem).base, (*rmem).base, (*rmem).size, true);
        if IS_ERR(mem) { return PTR_ERR(mem); }
        (*rmem).priv_data = mem as *mut core::ffi::c_void;
    }
    if (*mem).device_base + (*rmem).size - 1 > min_not_zero((*dev).coherent_dma_mask, (*dev).bus_dma_limit) {
        dev_warn!(dev, "reserved memory is beyond device's set DMA address range\n");
    }
    dma_assign_coherent_memory(dev, mem);
    0
}

#[cfg(CONFIG_OF_RESERVED_MEM)]
unsafe fn rmem_dma_device_release(rmem: *mut reserved_mem, dev: *mut device) {
    if !dev.is_null() { (*dev).dma_mem = core::ptr::null_mut(); }
}

#[cfg(CONFIG_OF_RESERVED_MEM)]
unsafe fn rmem_dma_setup(node: c_ulong, rmem: *mut reserved_mem) -> c_int {
    if !of_get_flat_dt_prop(node, c"reusable".as_ptr(), core::ptr::null_mut()).is_null() { return -ENODEV; }
    #[cfg(CONFIG_ARM)]
    if of_get_flat_dt_prop(node, c"no-map".as_ptr(), core::ptr::null_mut()).is_null() {
        pr_err!("Reserved memory: regions without no-map are not yet supported\n");
        return -EINVAL;
    }
    #[cfg(CONFIG_DMA_GLOBAL_POOL)]
    if !of_get_flat_dt_prop(node, c"linux,dma-default".as_ptr(), core::ptr::null_mut()).is_null() {
        WARN!(dma_reserved_default_memory_size, "Reserved memory: region for default DMA coherent area is redefined\n");
        dma_reserved_default_memory_base = (*rmem).base;
        dma_reserved_default_memory_size = (*rmem).size;
    }
    pr_info!("Reserved memory: created DMA memory pool at %pa, size %llu KiB\n", &(*rmem).base, (*rmem).size / SZ_1K);
    0
}

#[cfg(CONFIG_OF_RESERVED_MEM)]
#[cfg(CONFIG_DMA_GLOBAL_POOL)]
unsafe fn dma_init_reserved_memory() -> c_int {
    if dma_reserved_default_memory_size == 0 { return -ENOMEM; }
    dma_init_global_coherent(dma_reserved_default_memory_base, dma_reserved_default_memory_size as usize)
}

#[cfg(CONFIG_OF_RESERVED_MEM)]
// Equivalent of RESERVEDMEM_OF_DECLARE(dma, "shared-dma-pool", &rmem_dma_ops).
extern "C" {
    static rmem_dma_ops: reserved_mem_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
