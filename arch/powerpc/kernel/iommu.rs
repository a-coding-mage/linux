// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of powerpc/kernel/iommu.c. */

// Kernel headers and build-time configuration symbols are supplied by the
// surrounding kernel translation unit.

static mut NOVMERGE: i32 = 0;

unsafe fn iommu_debugfs_add(_tbl: *mut iommu_table) {}
unsafe fn iommu_debugfs_del(_tbl: *mut iommu_table) {}

unsafe extern "C" {
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn hash_32(v: u32, bits: u32) -> u32;
    fn raw_cpu_read(v: u32) -> u32;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn iommu_area_alloc(map: *mut c_ulong, limit: c_ulong, start: c_ulong,
                        npages: c_ulong, offset: c_ulong, boundary: c_ulong,
                        align_mask: c_ulong) -> c_ulong;
    fn dma_get_seg_boundary_nr_pages(dev: *mut device, shift: c_ulong) -> c_ulong;
    fn dma_get_max_seg_size(dev: *mut device) -> c_uint;
    fn iommu_num_pages(addr: c_ulong, size: usize, page_size: c_ulong) -> c_uint;
    fn sg_virt(s: *mut scatterlist) -> *mut c_void;
    fn sg_next(s: *mut scatterlist) -> *mut scatterlist;
    fn bitmap_clear(map: *mut c_ulong, start: c_ulong, n: c_ulong);
    fn set_bit(n: c_ulong, map: *mut c_ulong);
    fn find_next_bit(map: *mut c_ulong, size: c_ulong, start: c_ulong) -> c_ulong;
    fn vzalloc_node(size: usize, nid: i32) -> *mut c_void;
    fn vfree(p: *mut c_void);
    fn kfree(p: *mut c_void);
    fn alloc_pages_node(node: i32, flags: gfp_t, order: c_uint) -> *mut page;
    fn page_address(p: *mut page) -> *mut c_void;
    fn free_pages(addr: c_ulong, order: c_uint);
    fn memset(p: *mut c_void, val: i32, n: usize);
    fn phys_to_virt(p: phys_addr_t) -> *mut c_void;
    fn get_order(size: usize) -> c_uint;
    fn get_iommu_order(size: usize, tbl: *mut iommu_table) -> c_uint;
    fn printk_ratelimit() -> bool;
    fn mb();
}

unsafe fn setup_iommu_pool_hash() -> i32 {
    let mut i: u32 = 0;
    // for_each_possible_cpu(i)
    while i < NR_CPUS {
        per_cpu_write(IOMMU_POOL_HASH, i, hash_32(i, IOMMU_POOL_HASHBITS));
        i += 1;
    }
    0
}

unsafe fn should_fail_iommu(_dev: *mut device) -> bool { false }

unsafe fn iommu_range_alloc(dev: *mut device, tbl: *mut iommu_table,
    npages: c_ulong, handle: *mut c_ulong, mask: c_ulong,
    align_order: c_uint) -> c_ulong {
    let align_mask = (1u64 << align_order) as c_ulong - 1;
    if npages == 0 { return DMA_MAPPING_ERROR; }
    if should_fail_iommu(dev) { return DMA_MAPPING_ERROR; }
    let pool_nr = raw_cpu_read(IOMMU_POOL_HASH) & ((*tbl).nr_pools - 1);
    let largealloc = npages > 15;
    let mut pool = if largealloc { &mut (*tbl).large_pool }
                   else { &mut (*tbl).pools[pool_nr as usize] };
    let mut flags = 0;
    spin_lock_irqsave(&mut pool.lock, &mut flags);
    let mut pass = 0;
    loop {
        let mut start = if pass == 0 && !handle.is_null() && *handle >= pool.start && *handle < pool.end
            { *handle } else { pool.hint };
        let mut limit = pool.end;
        if start >= limit { start = pool.start; }
        if limit + (*tbl).it_offset > mask {
            limit = mask - (*tbl).it_offset + 1;
            if (start & mask) >= limit || pass > 0 {
                spin_unlock(&mut pool.lock);
                pool = &mut (*tbl).pools[0];
                spin_lock(&mut pool.lock);
                start = pool.start;
            } else { start &= mask; }
        }
        let n = iommu_area_alloc((*tbl).it_map, limit, start, npages,
            (*tbl).it_offset, dma_get_seg_boundary_nr_pages(dev, (*tbl).it_page_shift), align_mask);
        if n != c_ulong::MAX {
            let end = n + npages;
            pool.hint = if largealloc { end } else { (end + (*tbl).it_blocksize - 1) & !((*tbl).it_blocksize - 1) };
            if !handle.is_null() { *handle = end; }
            spin_unlock_irqrestore(&mut pool.lock, flags);
            return n;
        }
        if pass == 0 { pool.hint = pool.start; pass += 1; continue; }
        if pass <= (*tbl).nr_pools {
            spin_unlock(&mut pool.lock);
            let nr = (pool_nr + pass - 1) & ((*tbl).nr_pools - 1);
            pool = &mut (*tbl).pools[nr as usize];
            spin_lock(&mut pool.lock); pool.hint = pool.start; pass += 1; continue;
        }
        if pass == (*tbl).nr_pools + 1 {
            spin_unlock(&mut pool.lock); pool = &mut (*tbl).large_pool;
            spin_lock(&mut pool.lock); pool.hint = pool.start; pass += 1; continue;
        }
        spin_unlock_irqrestore(&mut pool.lock, flags);
        return DMA_MAPPING_ERROR;
    }
}

unsafe fn iommu_alloc(dev: *mut device, tbl: *mut iommu_table, page: *mut c_void,
    npages: c_uint, direction: dma_data_direction, mask: c_ulong,
    align_order: c_uint, attrs: c_ulong) -> dma_addr_t {
    let mut entry = iommu_range_alloc(dev, tbl, npages as c_ulong, core::ptr::null_mut(), mask, align_order);
    if entry == DMA_MAPPING_ERROR { return DMA_MAPPING_ERROR; }
    entry += (*tbl).it_offset;
    let ret = entry << (*tbl).it_page_shift;
    let fail = ((*tbl).it_ops).set.unwrap()(tbl, entry, npages,
        page as c_ulong & iommu_page_mask(tbl), direction, attrs);
    if fail != 0 { iommu_free_inner(tbl, ret, npages); return DMA_MAPPING_ERROR; }
    if let Some(flush) = (*(*tbl).it_ops).flush { flush(tbl); }
    mb(); ret
}

unsafe fn iommu_free_check(tbl: *mut iommu_table, dma_addr: dma_addr_t, npages: c_uint) -> bool {
    let entry = dma_addr >> (*tbl).it_page_shift;
    let free_entry = entry - (*tbl).it_offset;
    if free_entry + npages as c_ulong > (*tbl).it_size || entry < (*tbl).it_offset { return false; }
    true
}

unsafe fn get_pool(tbl: *mut iommu_table, entry: c_ulong) -> *mut iommu_pool {
    if entry >= (*tbl).large_pool.start { &mut (*tbl).large_pool }
    else { &mut (*tbl).pools[(entry / (*tbl).poolsize) as usize] }
}

unsafe fn iommu_free_inner(tbl: *mut iommu_table, dma_addr: dma_addr_t, npages: c_uint) {
    let entry = dma_addr >> (*tbl).it_page_shift;
    let free_entry = entry - (*tbl).it_offset;
    if !iommu_free_check(tbl, dma_addr, npages) { return; }
    ((*tbl).it_ops).clear.unwrap()(tbl, entry, npages);
    let pool = get_pool(tbl, free_entry); let mut flags = 0;
    spin_lock_irqsave(&mut (*pool).lock, &mut flags);
    bitmap_clear((*tbl).it_map, free_entry, npages as c_ulong);
    spin_unlock_irqrestore(&mut (*pool).lock, flags);
}

unsafe fn iommu_free(tbl: *mut iommu_table, dma_addr: dma_addr_t, npages: c_uint) {
    iommu_free_inner(tbl, dma_addr, npages);
    if let Some(flush) = (*(*tbl).it_ops).flush { flush(tbl); }
}

pub unsafe fn ppc_iommu_map_sg(dev: *mut device, tbl: *mut iommu_table,
    sglist: *mut scatterlist, nelems: i32, mask: c_ulong,
    direction: dma_data_direction, attrs: c_ulong) -> i32 {
    if nelems == 0 || tbl.is_null() { return -EINVAL; }
    let mut s = sglist; let mut outs = sglist; let mut segstart = sglist;
    let mut outcount = 1; let mut handle = 0; let mut dma_next = 0;
    (*outs).dma_length = 0;
    let max_seg_size = dma_get_max_seg_size(dev);
    for i in 0..nelems {
        let slen = (*s).length;
        if slen != 0 {
            let vaddr = sg_virt(s) as c_ulong;
            let npages = iommu_num_pages(vaddr, slen, iommu_page_size(tbl));
            let align = if (*tbl).it_page_shift < PAGE_SHIFT && slen >= PAGE_SIZE && vaddr & !PAGE_MASK == 0 { PAGE_SHIFT - (*tbl).it_page_shift } else { 0 };
            let mut entry = iommu_range_alloc(dev, tbl, npages as c_ulong, &mut handle, mask >> (*tbl).it_page_shift, align);
            if entry == DMA_MAPPING_ERROR { return -EIO; }
            entry += (*tbl).it_offset;
            let dma_addr = (entry << (*tbl).it_page_shift) | (vaddr & !iommu_page_mask(tbl));
            let fail = ((*tbl).it_ops).set.unwrap()(tbl, entry, npages, vaddr & iommu_page_mask(tbl), direction, attrs);
            if fail != 0 { return -EIO; }
            if segstart != s && (NOVMERGE != 0 || dma_addr != dma_next || (*outs).dma_length + slen > max_seg_size as usize) {
                segstart = s; outcount += 1; outs = sg_next(outs);
            } else if segstart != s { (*outs).dma_length += slen; }
            if segstart == s { (*outs).dma_address = dma_addr; (*outs).dma_length = slen; }
            dma_next = dma_addr + slen; let _ = i;
        }
        s = sg_next(s);
    }
    if let Some(flush) = (*(*tbl).it_ops).flush { flush(tbl); }
    mb(); outcount
}

pub unsafe fn ppc_iommu_unmap_sg(tbl: *mut iommu_table, sglist: *mut scatterlist,
    mut nelems: i32, _direction: dma_data_direction, _attrs: c_ulong) {
    if tbl.is_null() { return; }
    let mut sg = sglist;
    while nelems > 0 && (*sg).dma_length != 0 {
        let n = iommu_num_pages((*sg).dma_address, (*sg).dma_length, iommu_page_size(tbl));
        iommu_free_inner(tbl, (*sg).dma_address, n); sg = sg_next(sg); nelems -= 1;
    }
    if let Some(flush) = (*(*tbl).it_ops).flush { flush(tbl); }
}

pub unsafe fn iommu_table_reserve_pages(tbl: *mut iommu_table, mut start: c_ulong, mut end: c_ulong) {
    if (*tbl).it_offset == 0 { set_bit(0, (*tbl).it_map); }
    if start < (*tbl).it_offset { start = (*tbl).it_offset; }
    if end > (*tbl).it_offset + (*tbl).it_size { end = (*tbl).it_offset + (*tbl).it_size; }
    if start >= end { (*tbl).it_reserved_start = (*tbl).it_offset; (*tbl).it_reserved_end = (*tbl).it_offset; return; }
    (*tbl).it_reserved_start = start; (*tbl).it_reserved_end = end;
    let mut i = start; while i < end { set_bit(i - (*tbl).it_offset, (*tbl).it_map); i += 1; }
}

pub unsafe fn iommu_init_table(tbl: *mut iommu_table, nid: i32, start: c_ulong, end: c_ulong) -> *mut iommu_table {
    let sz = bits_to_longs((*tbl).it_size) * core::mem::size_of::<c_ulong>();
    (*tbl).it_map = vzalloc_node(sz, nid) as *mut c_ulong; if (*tbl).it_map.is_null() { return core::ptr::null_mut(); }
    iommu_table_reserve_pages(tbl, start, end);
    (*tbl).nr_pools = if ((*tbl).it_size << (*tbl).it_page_shift) >= 1024 * 1024 * 1024 { IOMMU_NR_POOLS } else { 1 };
    (*tbl).poolsize = ((*tbl).it_size * 3 / 4) / (*tbl).nr_pools;
    let mut i = 0; while i < (*tbl).nr_pools { let p = &mut (*tbl).pools[i as usize]; p.start = (*tbl).poolsize * i; p.hint = p.start; p.end = p.start + (*tbl).poolsize; i += 1; }
    let p = &mut (*tbl).large_pool; p.start = (*tbl).poolsize * i; p.hint = p.start; p.end = (*tbl).it_size;
    iommu_debugfs_add(tbl); tbl
}

pub unsafe fn iommu_table_in_use(tbl: *mut iommu_table) -> bool {
    let start = if (*tbl).it_offset == 0 { 1 } else { 0 };
    if (*tbl).it_reserved_start == 0 && (*tbl).it_reserved_end == 0 { return find_next_bit((*tbl).it_map, (*tbl).it_size, start) != (*tbl).it_size; }
    let end = (*tbl).it_reserved_start - (*tbl).it_offset;
    if find_next_bit((*tbl).it_map, end, start) != end { return true; }
    find_next_bit((*tbl).it_map, (*tbl).it_size, (*tbl).it_reserved_end - (*tbl).it_offset) != (*tbl).it_size
}

pub unsafe fn iommu_map_phys(dev: *mut device, tbl: *mut iommu_table, phys: phys_addr_t, size: usize, mask: c_ulong, dir: dma_data_direction, attrs: c_ulong) -> dma_addr_t {
    if tbl.is_null() { return DMA_MAPPING_ERROR; }
    let vaddr = phys_to_virt(phys); let uaddr = vaddr as c_ulong;
    let n = iommu_num_pages(uaddr, size, iommu_page_size(tbl));
    let align = if (*tbl).it_page_shift < PAGE_SHIFT && size >= PAGE_SIZE && uaddr & !PAGE_MASK == 0 { PAGE_SHIFT - (*tbl).it_page_shift } else { 0 };
    let mut dma = iommu_alloc(dev, tbl, vaddr, n, dir, mask >> (*tbl).it_page_shift, align, attrs);
    if dma != DMA_MAPPING_ERROR { dma |= uaddr & !iommu_page_mask(tbl); } dma
}

pub unsafe fn iommu_unmap_phys(tbl: *mut iommu_table, dma: dma_addr_t, size: usize, _dir: dma_data_direction, _attrs: c_ulong) {
    if !tbl.is_null() { iommu_free(tbl, dma, iommu_num_pages(dma, size, iommu_page_size(tbl))); }
}

pub fn iommu_direction_to_tce_perm(dir: dma_data_direction) -> c_ulong {
    match dir { DMA_BIDIRECTIONAL => TCE_PCI_READ | TCE_PCI_WRITE, DMA_FROM_DEVICE => TCE_PCI_WRITE, DMA_TO_DEVICE => TCE_PCI_READ, _ => 0 }
}

pub fn iommu_tce_direction(tce: c_ulong) -> dma_data_direction {
    if tce & TCE_PCI_READ != 0 && tce & TCE_PCI_WRITE != 0 { DMA_BIDIRECTIONAL }
    else if tce & TCE_PCI_READ != 0 { DMA_TO_DEVICE } else if tce & TCE_PCI_WRITE != 0 { DMA_FROM_DEVICE } else { DMA_NONE }
}

pub unsafe fn iommu_flush_tce(tbl: *mut iommu_table) { if let Some(f) = (*(*tbl).it_ops).flush { f(tbl); } mb(); }

pub fn iommu_tce_check_ioba(page_shift: c_ulong, offset: c_ulong, size: c_ulong, mut ioba: c_ulong, _npages: c_ulong) -> i32 {
    let mask = (1u64 << page_shift) as c_ulong - 1; if ioba & mask != 0 { return -EINVAL; } ioba >>= page_shift;
    if ioba < offset || ioba + 1 > offset + size { return -EINVAL; } 0
}
pub fn iommu_tce_check_gpa(page_shift: c_ulong, gpa: c_ulong) -> i32 { if gpa & ((1u64 << page_shift) as c_ulong - 1) != 0 { -EINVAL } else { 0 } }

pub unsafe fn iommu_alloc_coherent(dev: *mut device, tbl: *mut iommu_table,
    mut size: usize, dma_handle: *mut dma_addr_t, mask: c_ulong,
    flag: gfp_t, node: i32) -> *mut c_void {
    if tbl.is_null() { return core::ptr::null_mut(); }
    size = page_align(size); let order = get_order(size);
    if order >= IOMAP_MAX_ORDER { return core::ptr::null_mut(); }
    let page = alloc_pages_node(node, flag, order); if page.is_null() { return core::ptr::null_mut(); }
    let ret = page_address(page); memset(ret, 0, size);
    let nio_pages = iommu_page_align(size, tbl) >> (*tbl).it_page_shift;
    let mapping = iommu_alloc(dev, tbl, ret, nio_pages as c_uint, DMA_BIDIRECTIONAL,
        mask >> (*tbl).it_page_shift, get_iommu_order(size, tbl), 0);
    if mapping == DMA_MAPPING_ERROR { free_pages(ret as c_ulong, order); return core::ptr::null_mut(); }
    *dma_handle = mapping | (ret as u64 & ((1u64 << (*tbl).it_page_shift) - 1)); ret
}

pub unsafe fn iommu_free_coherent(tbl: *mut iommu_table, mut size: usize,
    vaddr: *mut c_void, dma_handle: dma_addr_t) {
    if !tbl.is_null() { size = page_align(size); let n = iommu_page_align(size, tbl) >> (*tbl).it_page_shift; iommu_free(tbl, dma_handle, n as c_uint); free_pages(vaddr as c_ulong, get_order(size)); }
}

pub unsafe fn iommu_table_clear(tbl: *mut iommu_table) {
    ((*tbl).it_ops).clear.unwrap()(tbl, (*tbl).it_offset, (*tbl).it_size as c_uint);
}

pub unsafe fn iommu_tce_table_get(tbl: *mut iommu_table) -> *mut iommu_table {
    if kref_get_unless_zero(&mut (*tbl).it_kref) { tbl } else { core::ptr::null_mut() }
}
pub unsafe fn iommu_tce_table_put(tbl: *mut iommu_table) -> i32 {
    if tbl.is_null() { return 0; }
    kref_put(&mut (*tbl).it_kref, iommu_table_free)
}
unsafe fn iommu_table_free(kref: *mut kref) {
    let tbl = container_of!(kref, iommu_table, it_kref);
    if let Some(f) = (*(*tbl).it_ops).free { f(tbl); }
    if (*tbl).it_map.is_null() { kfree(tbl as *mut c_void); return; }
    iommu_debugfs_del(tbl); if iommu_table_in_use(tbl) { /* pr_warn */ }
    vfree((*tbl).it_map as *mut c_void); kfree(tbl as *mut c_void);
}

pub unsafe fn iommu_tce_xchg_no_kill(mm: *mut mm_struct, tbl: *mut iommu_table,
    entry: c_ulong, hpa: *mut c_ulong, direction: *mut dma_data_direction) -> c_long {
    let ret = ((*tbl).it_ops).xchg_no_kill.unwrap()(tbl, entry, hpa, direction);
    if ret == 0 && (*direction == DMA_FROM_DEVICE || *direction == DMA_BIDIRECTIONAL) { let _ = (mm, hpa); }
    ret
}
pub unsafe fn iommu_tce_kill(tbl: *mut iommu_table, entry: c_ulong, pages: c_ulong) {
    if let Some(f) = (*(*tbl).it_ops).tce_kill { f(tbl, entry, pages); }
}

pub unsafe fn iommu_add_device(table_group: *mut iommu_table_group, dev: *mut device) -> i32 {
    if !device_is_registered(dev) { return -ENOENT; }
    if device_iommu_mapped(dev) { return -EBUSY; }
    let _ = table_group; iommu_probe_device(dev)
}

pub unsafe fn dev_has_iommu_table(dev: *mut device, data: *mut c_void) -> i32 {
    if dev.is_null() { return 0; }
    if device_iommu_mapped(dev) { *(data as *mut *mut pci_dev) = to_pci_dev(dev); return 1; } 0
}

// SPAPR TCE API and the platform IOMMU registration hooks retain their
// kernel ABI; their implementations are supplied by the surrounding API
// translation and are declared here as external entry points.
pub unsafe fn iommu_register_group(table_group: *mut iommu_table_group,
    pci_domain_number: i32, pe_num: c_ulong) {
    let grp = iommu_group_alloc();
    if is_err(grp) { return; }
    (*table_group).group = grp;
    iommu_group_set_iommudata(grp, table_group as *mut c_void, group_release);
    let name = kasprintf(GFP_KERNEL, pci_domain_number, pe_num);
    if !name.is_null() { iommu_group_set_name(grp, name); kfree(name as *mut c_void); }
}
unsafe fn group_release(data: *mut c_void) { (*(data as *mut iommu_table_group)).group = core::ptr::null_mut(); }

pub unsafe fn ppc_iommu_register_device(phb: *mut pci_controller) {
    iommu_device_sysfs_add(&mut (*phb).iommu, (*phb).parent, core::ptr::null(), (*phb).global_number);
    iommu_device_register(&mut (*phb).iommu, core::ptr::null(), (*phb).parent);
}
pub unsafe fn ppc_iommu_unregister_device(phb: *mut pci_controller) {
    iommu_device_unregister(&mut (*phb).iommu); iommu_device_sysfs_remove(&mut (*phb).iommu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
