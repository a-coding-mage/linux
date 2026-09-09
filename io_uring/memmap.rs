// SPDX-License-Identifier: GPL-2.0
// Dependencies correspond to the Linux kernel headers and local headers used by memmap.c.

unsafe fn io_mem_alloc_compound(
    pages: *mut *mut page,
    nr_pages: c_int,
    size: size_t,
    mut gfp: gfp_t,
    user: *mut user_struct,
) -> bool {
    let order: c_int = get_order(size);
    if order > MAX_PAGE_ORDER {
        return false;
    } else if order != 0 {
        gfp |= __GFP_COMP;
    }

    /*
     * get_order() rounds a non power of two size up, so the allocation
     * can hold more pages than the region exposes. Account those too,
     * and leave the compound allocation alone if they do not fit.
     */
    let nr_compound: c_ulong = 1usize.wrapping_shl(order as u32) as c_ulong;
    let extra = nr_compound.wrapping_sub(nr_pages as c_ulong);
    if extra != 0 && !user.is_null() && __io_account_mem(user, extra) != 0 {
        return false;
    }

    let page = alloc_pages(gfp, order);
    if page.is_null() {
        if extra != 0 && !user.is_null() {
            __io_unaccount_mem(user, extra);
        }
        return false;
    }

    for i in 0..nr_pages {
        *pages.add(i as usize) = page.add(i as usize);
    }
    true
}

pub unsafe fn io_pin_pages(uaddr: c_ulong, len: c_ulong, npages: *mut c_int) -> *mut *mut page {
    let mut end: c_ulong = 0;
    if check_add_overflow(uaddr, len, &mut end) {
        return ERR_PTR(-EOVERFLOW);
    }
    if check_add_overflow(end, PAGE_SIZE - 1, &mut end) {
        return ERR_PTR(-EOVERFLOW);
    }

    let end = end >> PAGE_SHIFT;
    let start = uaddr >> PAGE_SHIFT;
    let nr_pages = end - start;
    if WARN_ON_ONCE(nr_pages == 0) {
        return ERR_PTR(-EINVAL);
    }
    if nr_pages > (INT_MAX as usize) / core::mem::size_of::<*mut page>() {
        return ERR_PTR(-EOVERFLOW);
    }

    let pages = kvmalloc_objs::<*mut page>(nr_pages, GFP_KERNEL_ACCOUNT);
    if pages.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    let ret = pin_user_pages_fast(uaddr, nr_pages, FOLL_WRITE | FOLL_LONGTERM, pages);
    if ret == nr_pages as c_int {
        *npages = nr_pages as c_int;
        return pages;
    }
    if ret >= 0 {
        if ret != 0 {
            unpin_user_pages(pages, ret);
        }
        kvfree(pages);
        return ERR_PTR(-EFAULT);
    }
    kvfree(pages);
    ERR_PTR(ret)
}

const IO_REGION_F_VMAP: c_uint = 1;
const IO_REGION_F_USER_PROVIDED: c_uint = 2;
const IO_REGION_F_SINGLE_REF: c_uint = 4;

pub unsafe fn io_free_region(user: *mut user_struct, mr: *mut io_mapped_region) {
    if !(*mr).pages.is_null() {
        let mut nr_refs = (*mr).nr_pages as c_long;
        if (*mr).flags & IO_REGION_F_SINGLE_REF != 0 { nr_refs = 1; }
        if (*mr).flags & IO_REGION_F_USER_PROVIDED != 0 {
            unpin_user_pages((*mr).pages, nr_refs as c_int);
        } else {
            release_pages((*mr).pages, nr_refs as c_int);
        }
        kvfree((*mr).pages);
    }
    if (*mr).flags & IO_REGION_F_VMAP != 0 && !(*mr).ptr.is_null() { vunmap((*mr).ptr); }
    if (*mr).nr_pages != 0 && !user.is_null() {
        let mut nr_accounted = (*mr).nr_pages as c_ulong;
        if (*mr).flags & IO_REGION_F_SINGLE_REF != 0 {
            nr_accounted = 1usize.wrapping_shl(get_order(io_region_size(mr)) as u32) as c_ulong;
        }
        __io_unaccount_mem(user, nr_accounted);
    }
    core::ptr::write_bytes(mr, 0, 1);
}

unsafe fn io_region_init_ptr(mr: *mut io_mapped_region) -> c_int {
    let mut ifd = io_imu_folio_data { nr_folios: 0 };
    if io_check_coalesce_buffer((*mr).pages, (*mr).nr_pages, &mut ifd)
        && ifd.nr_folios == 1 && !PageHighMem(*(*mr).pages) {
        (*mr).ptr = page_address(*(*mr).pages);
        return 0;
    }
    let ptr = vmap((*mr).pages, (*mr).nr_pages, VM_MAP, PAGE_KERNEL);
    if ptr.is_null() { return -ENOMEM; }
    (*mr).ptr = ptr;
    (*mr).flags |= IO_REGION_F_VMAP;
    0
}

unsafe fn io_region_pin_pages(mr: *mut io_mapped_region, reg: *mut io_uring_region_desc) -> c_int {
    let mut nr_pages = 0;
    let pages = io_pin_pages((*reg).user_addr, io_region_size(mr), &mut nr_pages);
    if IS_ERR(pages) { return PTR_ERR(pages); }
    if WARN_ON_ONCE(nr_pages != (*mr).nr_pages as c_int) { return -EFAULT; }
    (*mr).pages = pages;
    (*mr).flags |= IO_REGION_F_USER_PROVIDED;
    0
}

unsafe fn io_region_allocate_pages(mr: *mut io_mapped_region, reg: *mut io_uring_region_desc, mmap_offset: c_ulong, user: *mut user_struct) -> c_int {
    let gfp = GFP_KERNEL_ACCOUNT | __GFP_ZERO | __GFP_NOWARN;
    let size = io_region_size(mr);
    let pages = kvmalloc_objs::<*mut page>((*mr).nr_pages, gfp);
    if pages.is_null() { return -ENOMEM; }
    (*mr).pages = pages;
    if io_mem_alloc_compound(pages, (*mr).nr_pages as c_int, size, gfp, user) {
        (*mr).flags |= IO_REGION_F_SINGLE_REF;
    } else {
        let nr_allocated = alloc_pages_bulk_node(gfp, NUMA_NO_NODE, (*mr).nr_pages, pages);
        if nr_allocated != (*mr).nr_pages {
            if nr_allocated != 0 { release_pages(pages, nr_allocated as c_int); }
            kvfree(pages);
            (*mr).pages = core::ptr::null_mut();
            return -ENOMEM;
        }
    }
    (*reg).mmap_offset = mmap_offset;
    0
}

pub unsafe fn io_create_region(ctx: *mut io_ring_ctx, mr: *mut io_mapped_region, reg: *mut io_uring_region_desc, mmap_offset: c_ulong) -> c_int {
    if WARN_ON_ONCE(!(*mr).pages.is_null() || !(*mr).ptr.is_null() || (*mr).nr_pages != 0) { return -EFAULT; }
    if memchr_inv(&(*reg).__resv as *const _, 0, core::mem::size_of_val(&(*reg).__resv)) { return -EINVAL; }
    if (*reg).flags & !IORING_MEM_REGION_TYPE_USER != 0 { return -EINVAL; }
    if ((*reg).flags & IORING_MEM_REGION_TYPE_USER != 0) != ((*reg).user_addr != 0) { return -EFAULT; }
    if (*reg).size == 0 || (*reg).mmap_offset != 0 || (*reg).id != 0 { return -EINVAL; }
    if (*reg).size >> PAGE_SHIFT > INT_MAX as u64 { return -E2BIG; }
    if ((*reg).user_addr | (*reg).size) & !PAGE_MASK != 0 { return -EINVAL; }
    let mut end = 0;
    if check_add_overflow((*reg).user_addr, (*reg).size, &mut end) { return -EOVERFLOW; }
    let nr_pages = (*reg).size >> PAGE_SHIFT;
    if !(*ctx).user.is_null() {
        let ret = __io_account_mem((*ctx).user, nr_pages);
        if ret != 0 { return ret; }
    }
    (*mr).nr_pages = nr_pages as usize;
    let ret = if (*reg).flags & IORING_MEM_REGION_TYPE_USER != 0 { io_region_pin_pages(mr, reg) } else { io_region_allocate_pages(mr, reg, mmap_offset, (*ctx).user) };
    if ret != 0 { io_free_region((*ctx).user, mr); return ret; }
    let ret = io_region_init_ptr(mr);
    if ret != 0 { io_free_region((*ctx).user, mr); return ret; }
    0
}

unsafe fn io_mmap_get_region(ctx: *mut io_ring_ctx, pgoff: loff_t) -> *mut io_mapped_region {
    let offset = pgoff << PAGE_SHIFT;
    let id;
    match offset & IORING_OFF_MMAP_MASK {
        IORING_OFF_SQ_RING | IORING_OFF_CQ_RING => return &mut (*ctx).ring_region,
        IORING_OFF_SQES => return &mut (*ctx).sq_region,
        IORING_OFF_PBUF_RING => { id = ((offset & !IORING_OFF_MMAP_MASK) >> IORING_OFF_PBUF_SHIFT) as c_uint; return io_pbuf_get_region(ctx, id); }
        IORING_MAP_OFF_PARAM_REGION => return &mut (*ctx).param_region,
        IORING_MAP_OFF_ZCRX_REGION => { id = ((offset & !IORING_OFF_MMAP_MASK) >> IORING_OFF_ZCRX_SHIFT) as c_uint; return io_zcrx_get_region(ctx, id); }
        _ => core::ptr::null_mut(),
    }
}

unsafe fn io_region_validate_mmap(ctx: *mut io_ring_ctx, mr: *mut io_mapped_region) -> *mut c_void {
    lockdep_assert_held(&mut (*ctx).mmap_lock);
    if !io_region_is_set(mr) || (*mr).flags & IO_REGION_F_USER_PROVIDED != 0 { return ERR_PTR(-EINVAL); }
    io_region_get_ptr(mr)
}

unsafe fn io_uring_validate_mmap_request(file: *mut file, pgoff: loff_t) -> *mut c_void {
    let ctx = (*file).private_data as *mut io_ring_ctx;
    let region = io_mmap_get_region(ctx, pgoff);
    if region.is_null() { return ERR_PTR(-EINVAL); }
    io_region_validate_mmap(ctx, region)
}

#[cfg(CONFIG_MMU)]
unsafe fn io_region_mmap(_ctx: *mut io_ring_ctx, mr: *mut io_mapped_region, vma: *mut vm_area_struct, max_pages: c_uint) -> c_int {
    let mut nr_pages = core::cmp::min((*mr).nr_pages, max_pages as usize);
    vm_flags_set(vma, VM_DONTEXPAND);
    vm_insert_pages(vma, (*vma).vm_start, (*mr).pages, &mut nr_pages)
}

#[cfg(CONFIG_MMU)]
pub unsafe fn io_uring_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    let ctx = (*file).private_data as *mut io_ring_ctx;
    let sz = (*vma).vm_end - (*vma).vm_start;
    let offset = (*vma).vm_pgoff << PAGE_SHIFT;
    let mut page_limit = UINT_MAX;
    let _guard = mutex_guard(&mut (*ctx).mmap_lock);
    let ptr = io_uring_validate_mmap_request(file, (*vma).vm_pgoff);
    if IS_ERR(ptr) { return PTR_ERR(ptr); }
    if offset & IORING_OFF_MMAP_MASK == IORING_OFF_SQ_RING || offset & IORING_OFF_MMAP_MASK == IORING_OFF_CQ_RING { page_limit = (sz + PAGE_SIZE - 1) >> PAGE_SHIFT; }
    io_region_mmap(ctx, io_mmap_get_region(ctx, (*vma).vm_pgoff), vma, page_limit)
}

#[cfg(CONFIG_MMU)]
pub unsafe fn io_uring_get_unmapped_area(filp: *mut file, addr: c_ulong, len: c_ulong, pgoff: c_ulong, mut flags: c_ulong) -> c_ulong {
    let ctx = (*filp).private_data as *mut io_ring_ctx;
    if addr != 0 { return -EINVAL as c_ulong; }
    let _guard = mutex_guard(&mut (*ctx).mmap_lock);
    let ptr = io_uring_validate_mmap_request(filp, pgoff as loff_t);
    if IS_ERR(ptr) { return PTR_ERR(ptr) as c_ulong; }
    filp = core::ptr::null_mut(); flags |= MAP_SHARED; let mut pgoff = 0;
    #[cfg(SHM_COLOUR)] { addr = ptr as c_ulong; pgoff = addr >> PAGE_SHIFT; }
    #[cfg(not(SHM_COLOUR))] { addr = 0; }
    mm_get_unmapped_area(filp, addr, len, pgoff, flags)
}

#[cfg(not(CONFIG_MMU))]
unsafe fn io_uring_nommu_vm_close(vma: *mut vm_area_struct) {
    let mut index = (*vma).vm_start;
    while index < (*vma).vm_end { put_page(virt_to_page(index as *mut c_void)); index += PAGE_SIZE; }
}

#[cfg(not(CONFIG_MMU))]
pub unsafe fn io_uring_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    let ctx = (*file).private_data as *mut io_ring_ctx;
    let _guard = mutex_guard(&mut (*ctx).mmap_lock);
    if !is_nommu_shared_mapping((*vma).vm_flags) { return -EINVAL; }
    let region = io_mmap_get_region(ctx, (*vma).vm_pgoff);
    if region.is_null() || !io_region_is_set(region) || (*vma).vm_end - (*vma).vm_start != ((*region).nr_pages << PAGE_SHIFT) { return -EINVAL; }
    for i in 0..(*region).nr_pages { get_page(*(*region).pages.add(i)); }
    (*vma).vm_ops = &io_uring_nommu_vm_ops;
    0
}

#[cfg(not(CONFIG_MMU))]
pub unsafe fn io_uring_nommu_mmap_capabilities(_file: *mut file) -> c_uint { NOMMU_MAP_DIRECT | NOMMU_MAP_READ | NOMMU_MAP_WRITE }

#[cfg(not(CONFIG_MMU))]
pub unsafe fn io_uring_get_unmapped_area(file: *mut file, _addr: c_ulong, _len: c_ulong, pgoff: c_ulong, _flags: c_ulong) -> c_ulong {
    let ctx = (*file).private_data as *mut io_ring_ctx;
    let _guard = mutex_guard(&mut (*ctx).mmap_lock);
    let ptr = io_uring_validate_mmap_request(file, pgoff as loff_t);
    if IS_ERR(ptr) { PTR_ERR(ptr) as c_ulong } else { ptr as c_ulong }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
