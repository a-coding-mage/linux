// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of arch/parisc/mm/init.c. */

extern "C" {
    static mut data_start: i32;
    fn parisc_kernel_start();
}

#[cfg(feature = "CONFIG_PGTABLE_LEVELS_3")]
static mut pmd0: [pmd_t; PTRS_PER_PMD] = [pmd_t::ZERO; PTRS_PER_PMD];
static mut swapper_pg_dir: [pgd_t; PTRS_PER_PGD] = [pgd_t::ZERO; PTRS_PER_PGD];
static mut pg0: [pte_t; PT_INITIAL * PTRS_PER_PTE] = [pte_t::ZERO; PT_INITIAL * PTRS_PER_PTE];

static mut data_resource: resource = resource { name: b"Kernel data\0".as_ptr() as *const i8, start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM };
static mut code_resource: resource = resource { name: b"Kernel code\0".as_ptr() as *const i8, start: 0, end: 0, flags: IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM };
static mut pdcdata_resource: resource = resource { name: b"PDC data (Page Zero)\0".as_ptr() as *const i8, start: 0, end: 0x9ff, flags: IORESOURCE_BUSY | IORESOURCE_MEM };
static mut sysram_resources: [resource; MAX_PHYSMEM_RANGES] = [resource::ZERO; MAX_PHYSMEM_RANGES];

static mut pmem_ranges: [physmem_range_t; MAX_PHYSMEM_RANGES] = [physmem_range_t::ZERO; MAX_PHYSMEM_RANGES];
static mut npmem_ranges: i32 = 0;

#[cfg(target_pointer_width = "64")]
const MAX_MEM: usize = 1usize << MAX_PHYSMEM_BITS;
#[cfg(not(target_pointer_width = "64"))]
const MAX_MEM: usize = 3584usize * 1024 * 1024;
static mut mem_limit: usize = MAX_MEM;

unsafe fn mem_limit_func() {
    let mut cp = boot_command_line;
    let mut end: *mut i8 = core::ptr::null_mut();
    let mut limit = MAX_MEM;
    while *cp != 0 {
        if core::slice::from_raw_parts(cp as *const u8, 4) == b"mem=" {
            cp = cp.add(4);
            limit = memparse(cp, &mut end);
            if end != cp { break; }
            cp = end;
        } else {
            while *cp != b' ' as i8 && *cp != 0 { cp = cp.add(1); }
            while *cp == b' ' as i8 { cp = cp.add(1); }
        }
    }
    if limit < mem_limit { mem_limit = limit; }
}

const MAX_GAP: usize = 0x40000000usize >> PAGE_SHIFT;

unsafe fn setup_bootmem() {
    let mut mem_max: usize;
    #[cfg(not(feature = "CONFIG_SPARSEMEM"))]
    let mut pmem_holes = [physmem_range_t::ZERO; MAX_PHYSMEM_RANGES - 1];
    #[cfg(not(feature = "CONFIG_SPARSEMEM"))]
    let mut npmem_holes: i32;
    let mut sysram_resource_count: i32;
    disable_sr_hashing();
    let mut i = 1;
    while i < npmem_ranges {
        let mut j = i;
        while j > 0 {
            if pmem_ranges[(j - 1) as usize].start_pfn < pmem_ranges[j as usize].start_pfn { break; }
            let t = pmem_ranges[(j - 1) as usize]; pmem_ranges[(j - 1) as usize] = pmem_ranges[j as usize]; pmem_ranges[j as usize] = t;
            j -= 1;
        }
        i += 1;
    }
    #[cfg(not(feature = "CONFIG_SPARSEMEM"))]
    { i = 1; while i < npmem_ranges { if pmem_ranges[i as usize].start_pfn - (pmem_ranges[(i-1) as usize].start_pfn + pmem_ranges[(i-1) as usize].pages) > MAX_GAP { npmem_ranges = i; printk(b"Large gap in memory detected (%ld pages). Consider turning on CONFIG_SPARSEMEM\0".as_ptr(), pmem_ranges[i as usize].start_pfn - (pmem_ranges[(i-1) as usize].start_pfn + pmem_ranges[(i-1) as usize].pages)); break; } i += 1; } }
    pr_info(b"Memory Ranges:\n\0".as_ptr());
    i = 0;
    while i < npmem_ranges { let r = &mut sysram_resources[i as usize]; let size = pmem_ranges[i as usize].pages << PAGE_SHIFT; let start = pmem_ranges[i as usize].start_pfn << PAGE_SHIFT; pr_info(b"%2d) Start 0x%016lx End 0x%016lx Size %6ld MB\n\0".as_ptr(), i, start, start + size - 1, size >> 20); r.name = b"System RAM\0".as_ptr() as *const i8; r.start = start; r.end = start + size - 1; r.flags = IORESOURCE_SYSTEM_RAM | IORESOURCE_BUSY; request_resource(&mut iomem_resource, r); i += 1; }
    sysram_resource_count = npmem_ranges;
    mem_limit_func(); mem_max = 0; i = 0;
    while i < npmem_ranges { let rsize = pmem_ranges[i as usize].pages << PAGE_SHIFT; if mem_max + rsize > mem_limit { printk(KERN_WARNING, b"Memory truncated to %ld MB\0".as_ptr(), mem_limit >> 20); if mem_max == mem_limit { npmem_ranges = i; } else { pmem_ranges[i as usize].pages = (mem_limit >> PAGE_SHIFT) - (mem_max >> PAGE_SHIFT); npmem_ranges = i + 1; mem_max = mem_limit; } break; } mem_max += rsize; i += 1; }
    printk(KERN_INFO, b"Total Memory: %ld MB\n\0".as_ptr(), mem_max >> 20);
    #[cfg(not(feature = "CONFIG_SPARSEMEM"))]
    { npmem_holes = 0; let mut end_pfn = pmem_ranges[0].start_pfn + pmem_ranges[0].pages; i = 1; while i < npmem_ranges { let hole_pages = pmem_ranges[i as usize].start_pfn - end_pfn; if hole_pages != 0 { pmem_holes[npmem_holes as usize].start_pfn = end_pfn; pmem_holes[npmem_holes as usize].pages = hole_pages; npmem_holes += 1; end_pfn += hole_pages; } end_pfn += pmem_ranges[i as usize].pages; i += 1; } pmem_ranges[0].pages = end_pfn - pmem_ranges[0].start_pfn; npmem_ranges = 1; }
    max_pfn = 0; i = 0; while i < npmem_ranges { let start_pfn = pmem_ranges[i as usize].start_pfn; let npages = pmem_ranges[i as usize].pages; memblock_add(start_pfn << PAGE_SHIFT, npages << PAGE_SHIFT); if start_pfn + npages > max_pfn { max_pfn = start_pfn + npages; } i += 1; }
    memblock_set_bottom_up(true); max_low_pfn = max_pfn;
    memblock_reserve(0, PAGE0.mem_free + 32768); memblock_reserve(__pa(KERNEL_BINARY_TEXT_START), _end - KERNEL_BINARY_TEXT_START);
    #[cfg(not(feature = "CONFIG_SPARSEMEM"))]
    { i = 0; while i < npmem_holes { memblock_reserve(pmem_holes[i as usize].start_pfn << PAGE_SHIFT, pmem_holes[i as usize].pages << PAGE_SHIFT); i += 1; } }
    #[cfg(feature = "CONFIG_BLK_DEV_INITRD")]
    if initrd_start != 0 { printk(KERN_INFO, b"initrd: %08lx-%08lx\n\0".as_ptr(), initrd_start, initrd_end); if __pa(initrd_start) < mem_max { let reserve = if __pa(initrd_end) > mem_max { mem_max - __pa(initrd_start) } else { initrd_end - initrd_start }; initrd_below_start_ok = 1; memblock_reserve(__pa(initrd_start), reserve); } }
    data_resource.start = virt_to_phys(&data_start as *const _); data_resource.end = virt_to_phys(_end) - 1; code_resource.start = virt_to_phys(_text); code_resource.end = virt_to_phys(&data_start as *const _) - 1;
    i = 0; while i < sysram_resource_count { request_resource(&mut sysram_resources[i as usize], &mut code_resource); request_resource(&mut sysram_resources[i as usize], &mut data_resource); i += 1; } request_resource(&mut sysram_resources[0], &mut pdcdata_resource); pdc_pdt_init(); memblock_allow_resize(); memblock_dump_all();
}

static mut kernel_set_to_readonly: bool = false;

unsafe fn map_pages(start_vaddr: usize, start_paddr: usize, size: usize, pgprot: pgprot_t, force: i32) {
    let mut address = start_paddr; let end_paddr = start_paddr + size; let mut vaddr = start_vaddr; let mut start_pmd = (start_vaddr >> PMD_SHIFT) & (PTRS_PER_PMD - 1); let mut start_pte = (start_vaddr >> PAGE_SHIFT) & (PTRS_PER_PTE - 1);
    let ro_start = __pa(_text); let ro_end = __pa(&data_start as *const _); let kernel_start = __pa(__init_begin); let kernel_end = __pa(_end);
    while address < end_paddr { let pgd = pgd_offset_k(vaddr); let p4d = p4d_offset(pgd, vaddr); let pud = pud_offset(p4d, vaddr); let mut pmd = pmd_offset(pud, vaddr); let mut tmp1 = start_pmd; while tmp1 < PTRS_PER_PMD { let mut pg_table; if pmd_none(*pmd) { pg_table = memblock_alloc_or_panic(PAGE_SIZE, PAGE_SIZE); pmd_populate_kernel(core::ptr::null_mut(), pmd, pg_table); } pg_table = pte_offset_kernel(pmd, vaddr); let mut tmp2 = start_pte; while tmp2 < PTRS_PER_PTE { let mut prot; let mut huge = false; if force != 0 { prot = pgprot; } else if address < kernel_start || address >= kernel_end { prot = PAGE_KERNEL; } else if !kernel_set_to_readonly { prot = PAGE_KERNEL_RWX; huge = true; } else if address >= ro_start { prot = if address < ro_end { PAGE_KERNEL_EXEC } else { PAGE_KERNEL }; huge = true; } else { prot = PAGE_KERNEL; } let mut pte = __mk_pte(address, prot); if huge { pte = pte_mkhuge(pte); } if address >= end_paddr { break; } set_pte(pg_table, pte); address += PAGE_SIZE; vaddr += PAGE_SIZE; tmp2 += 1; } start_pte = 0; if address >= end_paddr { break; } tmp1 += 1; pmd = pmd.add(1); } start_pmd = 0; }
}

pub unsafe extern "C" fn set_kernel_text_rw(enable_read_write: i32) { let start = __init_begin as usize; let end = &data_start as *const _ as usize; map_pages(start, __pa(start), end - start, PAGE_KERNEL_RWX, if enable_read_write != 0 { 1 } else { 0 }); flush_cache_all(); flush_tlb_all(); }

pub unsafe extern "C" fn free_initmem() { let init_begin = __init_begin as usize; let init_end = __init_end as usize; let kernel_end = _end as usize; map_pages(init_end, __pa(init_end), kernel_end - init_end, PAGE_KERNEL, 0); map_pages(init_begin, __pa(init_begin), init_end - init_begin, PAGE_KERNEL_RWX, 1); map_pages(init_begin, __pa(init_begin), init_end - init_begin, PAGE_KERNEL, 1); __flush_tlb_range(0, init_begin, kernel_end); flush_icache_range(init_begin, init_end); free_initmem_default(POISON_FREE_INITMEM); pdc_chassis_send_status(PDC_CHASSIS_DIRECT_BCOMPLETE); }

#[cfg(feature = "CONFIG_STRICT_KERNEL_RWX")]
pub unsafe extern "C" fn mark_rodata_ro() { let start = __start_rodata as usize; let end = __end_rodata as usize; pr_info(b"Write protecting the kernel read-only data: %luk\n\0".as_ptr(), (end - start) >> 10); kernel_set_to_readonly = true; map_pages(start, __pa(start), end - start, PAGE_KERNEL, 0); flush_cache_all(); flush_tlb_all(); }

const VM_MAP_OFFSET: usize = 32 * 1024;
unsafe fn set_map_offset(x: usize) -> usize { (x + VM_MAP_OFFSET) & !(VM_MAP_OFFSET - 1) }
pub static mut parisc_vmalloc_start: *mut core::ffi::c_void = core::ptr::null_mut();

pub unsafe extern "C" fn mem_init() { parisc_vmalloc_start = set_map_offset(KERNEL_MAP_START) as *mut _; }

unsafe fn pagetable_init() { let mut range = 0; while range < npmem_ranges { let start = pmem_ranges[range as usize].start_pfn << PAGE_SHIFT; let size = pmem_ranges[range as usize].pages << PAGE_SHIFT; map_pages(__va(start) as usize, start, size, PAGE_KERNEL, 0); range += 1; } }
unsafe fn gateway_init() { let addr = LINUX_GATEWAY_ADDR & PAGE_MASK; map_pages(addr, __pa(&linux_gateway_page), PAGE_SIZE, PAGE_GATEWAY, 1); }
unsafe fn fixmap_init() { let mut addr = FIXMAP_START; let end = FIXMAP_START + FIXMAP_SIZE; let pgd = pgd_offset_k(addr); let p4d = p4d_offset(pgd, addr); let pud = pud_offset(p4d, addr); let mut pmd = pmd_offset(pud, addr); while addr < end { let pte = memblock_alloc_or_panic(PAGE_SIZE, PAGE_SIZE); pmd_populate_kernel(&mut init_mm, pmd, pte); addr += PAGE_SIZE; pmd = pmd.add(1); } }
pub unsafe extern "C" fn arch_zone_limits_init(max_zone_pfns: *mut usize) { *max_zone_pfns.add(ZONE_NORMAL) = PFN_DOWN(memblock_end_of_DRAM()); }
pub unsafe extern "C" fn paging_init() { setup_bootmem(); pagetable_init(); gateway_init(); fixmap_init(); flush_cache_all_local(); flush_tlb_all_local(core::ptr::null_mut()); }

unsafe fn alloc_btlb(mut start: usize, end: usize, slot: *mut i32, entry_info: usize) { let slot_max = btlb_info.fixed_range_info.num_comb; let mut min_num_pages = btlb_info.min_size; if min_num_pages < 4 { min_num_pages = 4; } let mut size = HUGEPAGE_SIZE; while start < end && *slot < slot_max && size >= PAGE_SIZE { if start & (2 * size - 1) == 0 && end - start >= 2 * size { size <<= 1; continue; } if start & (size - 1) != 0 { size >>= 1; continue; } if end - start >= size { if (size >> PAGE_SHIFT) >= min_num_pages { pdc_btlb_insert(start >> PAGE_SHIFT, __pa(start) >> PAGE_SHIFT, size >> PAGE_SHIFT, entry_info, *slot); } *slot += 1; start += size; continue; } size /= 2; } }
pub unsafe extern "C" fn btlb_init_per_cpu() { if cfg!(feature = "CONFIG_PA20") { return; } if pdc_btlb_info(&mut btlb_info) < 0 { core::ptr::write_bytes(&mut btlb_info, 0, 1); } let s = dereference_function_descriptor(&_stext) as usize; let e = dereference_function_descriptor(&_etext) as usize; let t = dereference_function_descriptor(&_sdata) as usize; BUG_ON(t != e); let mut slot = 0; alloc_btlb(s, e, &mut slot, 0x13800000); let t = dereference_function_descriptor(&_edata) as usize; let e = dereference_function_descriptor(&__bss_start) as usize; BUG_ON(t != e); let s = dereference_function_descriptor(&_sdata) as usize; let e = dereference_function_descriptor(&__bss_stop) as usize; alloc_btlb(s, e, &mut slot, 0x11800000); }

#[cfg(feature = "CONFIG_PA20")] const NR_SPACE_IDS: usize = 262144;
#[cfg(not(feature = "CONFIG_PA20"))] const NR_SPACE_IDS: usize = 32768;
const RECYCLE_THRESHOLD: usize = NR_SPACE_IDS / 2;
const SID_ARRAY_SIZE: usize = NR_SPACE_IDS / (8 * core::mem::size_of::<usize>());
static mut space_id: [usize; SID_ARRAY_SIZE] = { let mut a = [0; SID_ARRAY_SIZE]; a[0] = 1; a };
static mut dirty_space_id: [usize; SID_ARRAY_SIZE] = [0; SID_ARRAY_SIZE];
static mut space_id_index: usize = 0; static mut free_space_ids: usize = NR_SPACE_IDS - 1; static mut dirty_space_ids: usize = 0;
static mut sid_lock: spinlock_t = spinlock_t::ZERO;

pub unsafe extern "C" fn alloc_sid() -> usize { spin_lock(&mut sid_lock); if free_space_ids == 0 { if dirty_space_ids != 0 { spin_unlock(&mut sid_lock); flush_tlb_all(); spin_lock(&mut sid_lock); } BUG_ON(free_space_ids == 0); } free_space_ids -= 1; let index = find_next_zero_bit(space_id.as_ptr(), NR_SPACE_IDS, space_id_index); space_id[index / (8 * core::mem::size_of::<usize>())] |= 1usize << (index % (8 * core::mem::size_of::<usize>())); space_id_index = index; spin_unlock(&mut sid_lock); index << SPACEID_SHIFT }
pub unsafe extern "C" fn free_sid(spaceid: usize) { let index = spaceid >> SPACEID_SHIFT; let word = index / (8 * core::mem::size_of::<usize>()); let mask = 1usize << (index % (8 * core::mem::size_of::<usize>())); spin_lock(&mut sid_lock); BUG_ON(dirty_space_id[word] & mask != 0); dirty_space_id[word] |= mask; dirty_space_ids += 1; spin_unlock(&mut sid_lock); }

unsafe fn recycle_sids() { if dirty_space_ids != 0 { for i in 0..SID_ARRAY_SIZE { space_id[i] ^= dirty_space_id[i]; dirty_space_id[i] = 0; } free_space_ids += dirty_space_ids; dirty_space_ids = 0; space_id_index = 0; } }
pub unsafe extern "C" fn flush_tlb_all() { spin_lock(&mut sid_lock); __inc_irq_stat(irq_tlb_count); flush_tlb_all_local(core::ptr::null_mut()); recycle_sids(); spin_unlock(&mut sid_lock); }

static protection_map: [pgprot_t; 16] = [PAGE_NONE, PAGE_READONLY, PAGE_NONE, PAGE_READONLY, PAGE_EXECREAD, PAGE_EXECREAD, PAGE_EXECREAD, PAGE_EXECREAD, PAGE_NONE, PAGE_READONLY, PAGE_WRITEONLY, PAGE_SHARED, PAGE_EXECREAD, PAGE_EXECREAD, PAGE_RWX, PAGE_RWX];

#[cfg(feature = "CONFIG_EXECMEM")]
static mut execmem_info: execmem_info_t = execmem_info_t::ZERO;
#[cfg(feature = "CONFIG_EXECMEM")]
pub unsafe extern "C" fn execmem_arch_setup() -> *mut execmem_info_t { execmem_info.ranges[EXECMEM_DEFAULT] = execmem_range { start: VMALLOC_START, end: VMALLOC_END, pgprot: PAGE_KERNEL_RWX, alignment: 1 }; &mut execmem_info }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
