// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) IBM Corporation, 2014, 2017
 * Anton Blanchard, Rashmica Gupta.
 */

// C dependencies supplied by the kernel build are intentionally not reproduced here.

#[repr(C)]
pub struct memtrace_entry {
    pub mem: *mut core::ffi::c_void,
    pub start: u64,
    pub size: u64,
    pub nid: u32,
    pub dir: *mut dentry,
    pub name: [core::ffi::c_char; 16],
}

extern "C" {
    static mut memtrace_mutex: mutex;
    static mut memtrace_size: u64;
    static mut memtrace_array: *mut memtrace_entry;
    static mut memtrace_array_nr: u32;
    static mut memtrace_debugfs_dir: *mut dentry;
}

#[allow(non_camel_case_types)]
pub enum mutex {}
#[allow(non_camel_case_types)]
pub enum dentry {}
#[allow(non_camel_case_types)]
pub enum file {}
#[allow(non_camel_case_types)]
pub enum vm_area_struct {}
#[allow(non_camel_case_types)]
pub enum page {}
#[allow(non_camel_case_types)]
pub enum mhp_params {}
#[allow(non_camel_case_types)]
pub enum file_operations {}

const FLUSH_CHUNK_SIZE: usize = 1 << 30;

unsafe fn memtrace_read(
    filp: *mut file,
    ubuf: *mut core::ffi::c_void,
    count: usize,
    ppos: *mut i64,
) -> isize {
    let ent = (*(filp as *mut file_private)).private_data as *mut memtrace_entry;
    simple_read_from_buffer(ubuf, count, ppos, (*ent).mem, (*ent).size as usize)
}

#[repr(C)]
struct file_private {
    private_data: *mut core::ffi::c_void,
}

unsafe fn memtrace_mmap(filp: *mut file, vma: *mut vm_area_struct) -> i32 {
    let ent = (*(filp as *mut file_private)).private_data as *mut memtrace_entry;
    let ent_nrpages = (*ent).size >> PAGE_SHIFT;
    let vma_nrpages = vma_pages(vma);

    if (*vma).vm_pgoff >= ent_nrpages {
        return -EINVAL;
    }
    if vma_nrpages > ent_nrpages - (*vma).vm_pgoff {
        return -EINVAL;
    }

    (*vma).vm_page_prot = pgprot_noncached((*vma).vm_page_prot);
    remap_pfn_range(
        vma,
        (*vma).vm_start,
        PHYS_PFN((*ent).start) + (*vma).vm_pgoff,
        (*vma).vm_end - (*vma).vm_start,
        (*vma).vm_page_prot,
    )
}

// The file-operation table corresponds directly to the C `memtrace_fops` initializer.
static mut memtrace_fops: file_operations = file_operations {};

unsafe fn flush_dcache_range_chunked(start: usize, stop: usize, chunk: usize) {
    let mut i = start;
    while i < stop {
        flush_dcache_range(i, core::cmp::min(stop, i.wrapping_add(chunk)));
        cond_resched();
        i = i.wrapping_add(chunk);
    }
}

unsafe fn memtrace_alloc_node(nid: u32, size: u64) -> u64 {
    let nr_pages = PHYS_PFN(size);
    let mut pfn: usize;
    let start_pfn: usize;
    let page: *mut page;

    page = alloc_contig_pages(nr_pages, GFP_KERNEL | __GFP_THISNODE | __GFP_NOWARN | __GFP_ZERO, nid, core::ptr::null_mut());
    if page.is_null() {
        return 0;
    }
    start_pfn = page_to_pfn(page);

    flush_dcache_range_chunked(
        pfn_to_kaddr(start_pfn) as usize,
        pfn_to_kaddr(start_pfn + nr_pages) as usize,
        FLUSH_CHUNK_SIZE,
    );

    pfn = start_pfn;
    while pfn < start_pfn + nr_pages {
        __SetPageOffline(pfn_to_page(pfn));
        pfn += 1;
    }

    arch_remove_linear_mapping(PFN_PHYS(start_pfn), size);
    PFN_PHYS(start_pfn)
}

unsafe fn memtrace_init_regions_runtime(size: u64) -> i32 {
    let mut nid: u32;
    let mut m: u64;

    memtrace_array = kzalloc_objs::<memtrace_entry>(num_online_nodes());
    if memtrace_array.is_null() {
        pr_err!("Failed to allocate memtrace_array\n");
        return -EINVAL;
    }

    for_each_online_node!(nid) {
        m = memtrace_alloc_node(nid, size);
        if m == 0 {
            pr_err!("Failed to allocate trace memory on node %d\n", nid);
            continue;
        }
        pr_info!("Allocated trace memory on node %d at 0x%016llx\n", nid, m);
        let ent = &mut *memtrace_array.add(memtrace_array_nr as usize);
        ent.start = m;
        ent.size = size;
        ent.nid = nid;
        memtrace_array_nr += 1;
    }
    0
}

unsafe fn memtrace_init_debugfs() -> i32 {
    let mut ret = 0;
    for i in 0..memtrace_array_nr {
        let ent = &mut *memtrace_array.add(i as usize);
        ent.mem = ioremap(ent.start, ent.size);
        if ent.mem.is_null() {
            pr_err!("Failed to map trace memory at 0x%llx\n", ent.start);
            ret = -1;
            continue;
        }
        snprintf!(ent.name.as_mut_ptr(), 16, "%08x", ent.nid);
        ent.dir = debugfs_create_dir(ent.name.as_ptr(), memtrace_debugfs_dir);
        debugfs_create_file_unsafe!("trace", 0o600, ent.dir, ent, &memtrace_fops);
        debugfs_create_x64!("start", 0o400, ent.dir, &mut ent.start);
        debugfs_create_x64!("size", 0o400, ent.dir, &mut ent.size);
    }
    ret
}

unsafe fn memtrace_free(nid: i32, start: u64, size: u64) -> i32 {
    let mut params = mhp_params {};
    let nr_pages = PHYS_PFN(size);
    let start_pfn = PHYS_PFN(start);
    let mut pfn;
    params.pgprot = PAGE_KERNEL;
    let ret = arch_create_linear_mapping(nid, start, size, &mut params);
    if ret != 0 { return ret; }
    pfn = start_pfn;
    while pfn < start_pfn + nr_pages {
        __ClearPageOffline(pfn_to_page(pfn));
        pfn += 1;
    }
    free_contig_range(start_pfn, nr_pages);
    0
}

unsafe fn memtrace_free_regions() -> i32 {
    let mut ret = 0;
    let mut i = memtrace_array_nr as i32 - 1;
    while i >= 0 {
        let ent = &mut *memtrace_array.add(i as usize);
        if ent.nid == NUMA_NO_NODE { i -= 1; continue; }
        if !ent.mem.is_null() { iounmap(ent.mem); ent.mem = core::ptr::null_mut(); }
        if memtrace_free(ent.nid as i32, ent.start, ent.size) != 0 {
            pr_err!("Failed to free trace memory on node %d\n", ent.nid);
            ret += 1;
            i -= 1;
            continue;
        }
        debugfs_remove_recursive(ent.dir);
        pr_info!("Freed trace memory back on node %d\n", ent.nid);
        ent.size = NUMA_NO_NODE as u64;
        ent.start = NUMA_NO_NODE as u64;
        ent.nid = NUMA_NO_NODE;
        i -= 1;
    }
    if ret != 0 { return ret; }
    kfree(memtrace_array as *mut core::ffi::c_void);
    memtrace_array = core::ptr::null_mut();
    memtrace_size = 0;
    memtrace_array_nr = 0;
    0
}

unsafe fn memtrace_enable_set(_data: *mut core::ffi::c_void, val: u64) -> i32 {
    let mut rc = -EAGAIN;
    let bytes = memory_block_size_bytes();
    if val & (bytes - 1) != 0 {
        pr_err!("Value must be aligned with 0x%llx\n", bytes);
        return -EINVAL;
    }
    mutex_lock(&mut memtrace_mutex);
    if memtrace_size != 0 && memtrace_free_regions() != 0 { mutex_unlock(&mut memtrace_mutex); return rc; }
    if val == 0 { mutex_unlock(&mut memtrace_mutex); return 0; }
    if memtrace_init_regions_runtime(val) != 0 { mutex_unlock(&mut memtrace_mutex); return rc; }
    if memtrace_init_debugfs() != 0 { mutex_unlock(&mut memtrace_mutex); return rc; }
    memtrace_size = val;
    rc = 0;
    mutex_unlock(&mut memtrace_mutex);
    rc
}

unsafe fn memtrace_enable_get(_data: *mut core::ffi::c_void, val: *mut u64) -> i32 {
    *val = memtrace_size;
    0
}

static mut memtrace_init_fops: file_operations = file_operations {};

unsafe fn memtrace_init() -> i32 {
    memtrace_debugfs_dir = debugfs_create_dir(c"memtrace".as_ptr(), arch_debugfs_dir);
    debugfs_create_file!("enable", 0o600, memtrace_debugfs_dir, core::ptr::null_mut(), &memtrace_init_fops);
    0
}

// machine_device_initcall(powernv, memtrace_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
