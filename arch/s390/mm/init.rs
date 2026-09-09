// SPDX-License-Identifier: GPL-2.0
/*
 *  S390 version
 *    Copyright IBM Corp. 1999
 *    Author(s): Hartmut Penner (hp@de.ibm.com)
 *
 *  Derived from "arch/i386/mm/init.c"
 *    Copyright (C) 1995  Linus Torvalds
 */

// Linux and architecture dependencies are supplied by other translation units.

#[repr(C)]
pub struct ctlreg {
    _private: [u8; 0],
}

extern "C" {
    static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD];
    static mut invalid_pg_dir: [pgd_t; PTRS_PER_PGD];
    static mut s390_invalid_asce: ctlreg;

    static mut page_noexec_mask: c_ulong;
    static mut segment_noexec_mask: c_ulong;
    static mut region_noexec_mask: c_ulong;
    static mut empty_zero_page: c_ulong;
    static mut zero_page_mask: c_ulong;
    static mut __per_cpu_offset: [c_ulong; NR_CPUS];
}

pub unsafe fn arch_setup_zero_pages() {
    let total_pages: c_ulong = memblock_estimated_nr_free_pages();
    let mut order: c_uint;

    /* Latest machines require a mapping granularity of 512KB */
    order = 7;

    /* Limit number of empty zero pages for small memory sizes */
    while order > 2 && (total_pages >> 10) < (1usize << order) as c_ulong {
        order -= 1;
    }

    empty_zero_page = memblock_alloc_or_panic(PAGE_SIZE << order, PAGE_SIZE) as c_ulong;
    zero_page_mask = ((PAGE_SIZE << order) - 1) & PAGE_MASK;
    set_memory_ro(empty_zero_page, 1usize << order);
}

pub unsafe fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) {
    *max_zone_pfns.add(ZONE_DMA) = virt_to_pfn(MAX_DMA_ADDRESS);
    *max_zone_pfns.add(ZONE_NORMAL) = max_low_pfn;
}

/* paging_init() sets up the page tables */
pub unsafe fn paging_init() {
    vmem_map_init();
    zone_dma_limit = DMA_BIT_MASK(31);
}

pub unsafe fn mark_rodata_ro() {
    let size: c_ulong = __end_ro_after_init - __start_ro_after_init;

    if cpu_has_nx() {
        system_ctl_set_bit(0, CR0_INSTRUCTION_EXEC_PROTECTION_BIT);
    }
    __set_memory_ro(__start_ro_after_init, __end_ro_after_init);
    pr_info("Write protected read-only-after-init data: %luk\n", size >> 10);
}

pub unsafe fn set_memory_encrypted(mut vaddr: c_ulong, numpages: c_int) -> c_int {
    for _ in 0..numpages {
        uv_remove_shared(virt_to_phys(vaddr as *mut c_void));
        vaddr += PAGE_SIZE;
    }
    0
}

pub unsafe fn set_memory_decrypted(mut vaddr: c_ulong, numpages: c_int) -> c_int {
    for _ in 0..numpages {
        uv_set_shared(virt_to_phys(vaddr as *mut c_void));
        vaddr += PAGE_SIZE;
    }
    0
}

/* are we a protected virtualization guest? */
pub unsafe fn force_dma_unencrypted(_dev: *mut device) -> bool {
    is_prot_virt_guest()
}

pub unsafe fn cc_platform_has(attr: cc_attr) -> bool {
    match attr {
        cc_attr::CC_ATTR_MEM_ENCRYPT | cc_attr::CC_ATTR_GUEST_MEM_ENCRYPT => {
            is_prot_virt_guest()
        }
        _ => false,
    }
}

/* protected virtualization */
unsafe fn pv_init() {
    if !is_prot_virt_guest() {
        return;
    }

    virtio_set_mem_acc_cb(virtio_require_restricted_mem_acc);

    /* make sure bounce buffers are shared */
    swiotlb_init(true, SWIOTLB_VERBOSE | SWIOTLB_ANY);
    swiotlb_update_mem_attributes();
}

pub unsafe fn arch_mm_preinit() {
    cpumask_set_cpu(0, &mut init_mm.context.cpu_attach_mask);
    cpumask_set_cpu(0, mm_cpumask(&mut init_mm));
    pv_init();
}

pub unsafe fn memory_block_size_bytes() -> c_ulong {
    /*
     * Make sure the memory block size is always greater
     * or equal than the memory increment size.
     */
    max_t(MIN_MEMORY_BLOCK_SIZE, sclp.rzm)
}

unsafe fn pcpu_cpu_distance(_from: c_uint, _to: c_uint) -> c_int { LOCAL_DISTANCE }

unsafe fn pcpu_cpu_to_node(_cpu: c_int) -> c_int { 0 }

pub unsafe fn setup_per_cpu_areas() {
    let delta: c_ulong;
    let mut cpu: c_uint;
    let rc: c_int;

    /*
     * Always reserve area for module percpu variables.  That's
     * what the legacy allocator did.
     */
    rc = pcpu_embed_first_chunk(
        PERCPU_MODULE_RESERVE,
        PERCPU_DYNAMIC_RESERVE,
        PAGE_SIZE,
        Some(pcpu_cpu_distance),
        Some(pcpu_cpu_to_node),
    );
    if rc < 0 {
        panic!("Failed to initialize percpu areas.");
    }

    delta = pcpu_base_addr as c_ulong - __per_cpu_start as c_ulong;
    for_each_possible_cpu!(cpu) {
        __per_cpu_offset[cpu as usize] = delta + pcpu_unit_offsets[cpu as usize];
    }
}

#[cfg(feature = "CONFIG_MEMORY_HOTPLUG")]
mod memory_hotplug {
    #[cfg(feature = "CONFIG_CMA")]
    #[repr(C)]
    struct s390_cma_mem_data { start: c_ulong, end: c_ulong }

    #[cfg(feature = "CONFIG_CMA")]
    unsafe fn s390_cma_check_range(cma: *mut cma, data: *mut c_void) -> c_int {
        let mem_data = data as *mut s390_cma_mem_data;
        if cma_intersects(cma, (*mem_data).start, (*mem_data).end) { return -EBUSY; }
        0
    }

    #[cfg(feature = "CONFIG_CMA")]
    unsafe fn s390_cma_mem_notifier(_nb: *mut notifier_block, action: c_ulong, data: *mut c_void) -> c_int {
        let arg = data as *mut memory_notify;
        let mut mem_data = s390_cma_mem_data { start: (*arg).start_pfn << PAGE_SHIFT, end: 0 };
        mem_data.end = mem_data.start + ((*arg).nr_pages << PAGE_SHIFT);
        let mut rc = 0;
        if action == MEM_GOING_OFFLINE { rc = cma_for_each_area(s390_cma_check_range, &mut mem_data as *mut _ as *mut c_void); }
        notifier_from_errno(rc)
    }

    #[cfg(feature = "CONFIG_CMA")]
    static mut s390_cma_mem_nb: notifier_block = notifier_block { notifier_call: Some(s390_cma_mem_notifier) };

    #[cfg(feature = "CONFIG_CMA")]
    unsafe fn s390_cma_mem_init() -> c_int { register_memory_notifier(&mut s390_cma_mem_nb) }

    pub unsafe fn arch_add_memory(nid: c_int, start: u64, size: u64, params: *mut mhp_params) -> c_int {
        let start_pfn = PFN_DOWN(start);
        let size_pages = PFN_DOWN(size);
        if WARN_ON_ONCE(pgprot_val((*params).pgprot) != pgprot_val(PAGE_KERNEL)) { return -EINVAL; }
        VM_BUG_ON!(mhp_range_allowed(start, size, true));
        let mut rc = vmem_add_mapping(start, size);
        if rc != 0 { return rc; }
        rc = __add_pages(nid, start_pfn, size_pages, params);
        if rc != 0 { vmem_remove_mapping(start, size); }
        rc
    }

    pub unsafe fn arch_remove_memory(start: u64, size: u64, altmap: *mut vmem_altmap, pgmap: *mut dev_pagemap) {
        __remove_pages(start >> PAGE_SHIFT, size >> PAGE_SHIFT, altmap, pgmap);
        vmem_remove_mapping(start, size);
    }
}

#[cfg(feature = "CONFIG_EXECMEM")]
static mut execmem_info: execmem_info = execmem_info { ranges: unsafe { core::mem::zeroed() } };

#[cfg(feature = "CONFIG_EXECMEM")]
pub unsafe fn execmem_arch_setup() -> *mut execmem_info {
    let mut module_load_offset: c_ulong = 0;
    if kaslr_enabled() { module_load_offset = get_random_u32_inclusive(1, 1024) as c_ulong * PAGE_SIZE; }
    let start = MODULES_VADDR + module_load_offset;
    execmem_info = execmem_info { ranges: [execmem_range { flags: EXECMEM_KASAN_SHADOW, start, end: MODULES_END, pgprot: PAGE_KERNEL, alignment: MODULE_ALIGN }] };
    &mut execmem_info
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
