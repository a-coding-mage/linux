// SPDX-License-Identifier: GPL-2.0-only
// Rust owner of the unchanged adjacent init.c.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]
include!("init_support.rs");
include!("init_percpu.rs");
include!("init_cpu_ids.rs");

const fn cache_to_pte_defaults() -> [u16; _PAGE_CACHE_MODE_NUM as usize] {
    let mut table = [0; _PAGE_CACHE_MODE_NUM as usize];
    table[_PAGE_CACHE_MODE_WC as usize] = _PAGE_PCD as u16;
    table[_PAGE_CACHE_MODE_UC_MINUS as usize] = _PAGE_PCD as u16;
    table[_PAGE_CACHE_MODE_UC as usize] = (_PAGE_PWT | _PAGE_PCD) as u16;
    table[_PAGE_CACHE_MODE_WT as usize] = _PAGE_PCD as u16;
    table[_PAGE_CACHE_MODE_WP as usize] = _PAGE_PCD as u16;
    table
}
static mut __cachemode2pte_tbl: [u16; _PAGE_CACHE_MODE_NUM as usize] = cache_to_pte_defaults();
static mut __pte2cachemode_tbl: [u8; 8] = [
    _PAGE_CACHE_MODE_WB as u8,
    _PAGE_CACHE_MODE_UC_MINUS as u8,
    _PAGE_CACHE_MODE_UC_MINUS as u8,
    _PAGE_CACHE_MODE_UC as u8,
    _PAGE_CACHE_MODE_WB as u8,
    _PAGE_CACHE_MODE_UC_MINUS as u8,
    _PAGE_CACHE_MODE_UC_MINUS as u8,
    _PAGE_CACHE_MODE_UC as u8,
];
#[inline(always)]
fn pte2cm_idx(value: c_ulong) -> usize {
    (((value >> (b::RUST_MM_PAGE_BIT_PAT - 2)) & 4)
        | ((value >> (b::RUST_MM_PAGE_BIT_PCD - 1)) & 2)
        | ((value >> b::RUST_MM_PAGE_BIT_PWT) & 1)) as usize
}
#[inline(always)]
fn cm_idx2pte(value: c_uint) -> c_ulong {
    (((value & 4) << (b::RUST_MM_PAGE_BIT_PAT - 2))
        | ((value & 2) << (b::RUST_MM_PAGE_BIT_PCD - 1))
        | ((value & 1) << b::RUST_MM_PAGE_BIT_PWT)) as c_ulong
}
#[no_mangle]
pub unsafe extern "C" fn cachemode2protval(pcm: page_cache_mode) -> c_ulong {
    if pcm == 0 {
        0
    } else {
        *addr_of!(__cachemode2pte_tbl)
            .cast::<u16>()
            .add(pcm as usize) as c_ulong
    }
}
#[no_mangle]
pub unsafe extern "C" fn x86_has_pat_wp() -> bool {
    let prot = *addr_of!(__cachemode2pte_tbl)
        .cast::<u16>()
        .add(_PAGE_CACHE_MODE_WP as usize);
    *addr_of!(__pte2cachemode_tbl)
        .cast::<u8>()
        .add(pte2cm_idx(prot as c_ulong))
        == _PAGE_CACHE_MODE_WP as u8
}
#[no_mangle]
pub unsafe extern "C" fn pgprot2cachemode(pgprot: pgprot_t) -> page_cache_mode {
    let masked = b::rust_mm_pgprot_val(pgprot) & _PAGE_CACHE_MASK;
    if masked == 0 {
        0
    } else {
        *addr_of!(__pte2cachemode_tbl)
            .cast::<u8>()
            .add(pte2cm_idx(masked)) as page_cache_mode
    }
}
#[link_section = ".init.data"]
static mut pgt_buf_start: c_ulong = 0;
#[link_section = ".init.data"]
static mut pgt_buf_end: c_ulong = 0;
#[link_section = ".init.data"]
static mut pgt_buf_top: c_ulong = 0;
static mut min_pfn_mapped: c_ulong = 0;
#[link_section = ".init.data"]
static mut can_use_brk_pgt: bool = true;
#[no_mangle]
pub static mut after_bootmem: c_int = 0;
#[no_mangle]
pub static mut direct_gbpages: c_int = if cfg!(CONFIG_X86_DIRECT_GBPAGES) {
    1
} else {
    0
};
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_mm_parse_gbpages_on(_: *mut c_char) -> c_int {
    direct_gbpages = 1;
    0
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_mm_parse_gbpages_off(_: *mut c_char) -> c_int {
    direct_gbpages = 0;
    0
}

#[no_mangle]
#[link_section = ".ref.text"]
pub unsafe extern "C" fn alloc_low_pages(num: c_uint) -> *mut c_void {
    if after_bootmem != 0 {
        let order = b::rust_mm_get_order((num as c_ulong).wrapping_shl(PAGE_SHIFT));
        return b::rust_mm_alloc_low_after_bootmem(order) as *mut c_void;
    }
    let pfn;
    if pgt_buf_end.wrapping_add(num as c_ulong) > pgt_buf_top || !can_use_brk_pgt {
        let mut ret = 0;
        if min_pfn_mapped < max_pfn_mapped {
            ret = memblock_phys_alloc_range(
                PAGE_SIZE.wrapping_mul(num as c_ulong) as phys_addr_t,
                PAGE_SIZE as phys_addr_t,
                min_pfn_mapped.wrapping_shl(PAGE_SHIFT) as phys_addr_t,
                max_pfn_mapped.wrapping_shl(PAGE_SHIFT) as phys_addr_t,
            ) as c_ulong;
        }
        if ret == 0 && can_use_brk_pgt {
            ret = b::rust_mm___pa(extend_brk(
                PAGE_SIZE.wrapping_mul(num as c_ulong),
                PAGE_SIZE,
            ));
        }
        if ret == 0 {
            panic(
                c"alloc_low_pages: can not alloc memory"
                    .as_ptr()
                    .cast::<c_char>(),
            );
        }
        pfn = ret >> PAGE_SHIFT;
    } else {
        pfn = pgt_buf_end;
        pgt_buf_end = pgt_buf_end.wrapping_add(num as c_ulong);
    }
    for i in 0..num {
        b::rust_mm_clear_page(b::rust_mm___va(
            pfn.wrapping_add(i as c_ulong).wrapping_shl(PAGE_SHIFT),
        ));
    }
    b::rust_mm___va(pfn.wrapping_shl(PAGE_SHIFT))
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn early_alloc_pgt_buf() {
    let tables = b::RUST_MM_INIT_PGT_BUF_SIZE as c_ulong;
    let base = b::rust_mm___pa(extend_brk(tables as usize, PAGE_SIZE as usize));
    pgt_buf_start = base >> PAGE_SHIFT;
    pgt_buf_end = pgt_buf_start;
    pgt_buf_top = pgt_buf_start.wrapping_add(tables >> PAGE_SHIFT);
}

#[derive(Clone, Copy)]
#[repr(C)]
struct map_range {
    start: c_ulong,
    end: c_ulong,
    page_size_mask: c_uint,
}
static mut page_size_mask: c_int = 0;
#[inline]
unsafe fn cr4_set_bits_and_update_boot(mask: c_ulong) {
    mmu_cr4_features |= mask;
    if !trampoline_cr4_features.is_null() {
        *trampoline_cr4_features = mmu_cr4_features as u32;
    }
    b::rust_mm_cr4_set_bits(mask);
}
#[cold]
#[link_section = ".init.text"]
unsafe fn probe_page_size_mask() {
    if b::rust_mm_boot_has_pse() && !b::rust_mm_debug_pagealloc_enabled() {
        page_size_mask |= 1 << PG_LEVEL_2M;
    } else {
        direct_gbpages = 0;
    }
    if b::rust_mm_boot_has_pse() {
        cr4_set_bits_and_update_boot(b::RUST_MM_X86_CR4_PSE);
    }
    __supported_pte_mask &= !(_PAGE_GLOBAL as pteval_t);
    if b::rust_mm_boot_has_pge() {
        cr4_set_bits_and_update_boot(b::RUST_MM_X86_CR4_PGE);
        __supported_pte_mask |= _PAGE_GLOBAL as pteval_t;
    }
    __default_kernel_pte_mask = __supported_pte_mask;
    if b::rust_mm_feature_pti() {
        __default_kernel_pte_mask &= !(_PAGE_GLOBAL as pteval_t);
    }
    if direct_gbpages != 0 && b::rust_mm_boot_has_gbpages() {
        b::rust_mm_log_gbpages();
        page_size_mask |= 1 << PG_LEVEL_1G;
    } else {
        direct_gbpages = 0;
    }
}
unsafe fn setup_pcid() {
    #[cfg(CONFIG_X86_64)]
    {
        if !b::rust_mm_boot_has_pcid() {
            return;
        }
        let matched = x86_match_cpu(addr_of!(invlpg_miss_ids).cast());
        if !matched.is_null()
            && (boot_cpu_data.microcode as kernel_ulong_t) < (*matched).driver_data
        {
            b::rust_mm_log_disable_pcid();
            setup_clear_cpu_cap(b::RUST_MM_X86_FEATURE_PCID);
            return;
        }
        if b::rust_mm_boot_has_pge() {
            b::rust_mm_cr4_set_bits(b::RUST_MM_X86_CR4_PCIDE);
        } else {
            setup_clear_cpu_cap(b::RUST_MM_X86_FEATURE_PCID);
        }
    }
}
const NR_RANGE_MR: usize = if cfg!(CONFIG_X86_32) { 3 } else { 5 };
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn save_mr(
    mr: *mut map_range,
    mut nr: usize,
    start: c_ulong,
    end: c_ulong,
    mask: c_ulong,
) -> usize {
    if start < end {
        if nr >= NR_RANGE_MR {
            panic(
                c"run out of range for init_memory_mapping\n"
                    .as_ptr()
                    .cast::<c_char>(),
            );
        }
        (*mr.add(nr)).start = start.wrapping_shl(PAGE_SHIFT);
        (*mr.add(nr)).end = end.wrapping_shl(PAGE_SHIFT);
        (*mr.add(nr)).page_size_mask = mask as c_uint;
        nr += 1;
    }
    nr
}
#[link_section = ".ref.text"]
unsafe fn adjust_range_page_size_mask(mr: *mut map_range, nr: usize) {
    for i in 0..nr {
        let r = mr.add(i);
        if page_size_mask & (1 << PG_LEVEL_2M) != 0 && (*r).page_size_mask & (1 << PG_LEVEL_2M) == 0
        {
            let start = round_down((*r).start, PMD_SIZE);
            let end = round_up((*r).end, PMD_SIZE);
            #[cfg(CONFIG_X86_32)]
            if (end >> PAGE_SHIFT) > max_low_pfn {
                continue;
            }
            if memblock_is_region_memory(
                start as phys_addr_t,
                end.wrapping_sub(start) as phys_addr_t,
            ) {
                (*r).page_size_mask |= 1 << PG_LEVEL_2M;
            }
        }
        if page_size_mask & (1 << PG_LEVEL_1G) != 0 && (*r).page_size_mask & (1 << PG_LEVEL_1G) == 0
        {
            let start = round_down((*r).start, PUD_SIZE);
            let end = round_up((*r).end, PUD_SIZE);
            if memblock_is_region_memory(
                start as phys_addr_t,
                end.wrapping_sub(start) as phys_addr_t,
            ) {
                (*r).page_size_mask |= 1 << PG_LEVEL_1G;
            }
        }
    }
}
unsafe fn page_size_string(r: *const map_range) -> *const c_char {
    if (*r).page_size_mask & (1 << PG_LEVEL_1G) != 0 {
        return c"1G".as_ptr().cast::<c_char>();
    }
    if cfg!(all(CONFIG_X86_32, not(CONFIG_X86_PAE)))
        && (*r).page_size_mask & (1 << PG_LEVEL_2M) != 0
    {
        return c"4M".as_ptr().cast::<c_char>();
    }
    if (*r).page_size_mask & (1 << PG_LEVEL_2M) != 0 {
        return c"2M".as_ptr().cast::<c_char>();
    }
    c"4k".as_ptr().cast::<c_char>()
}
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), cold)]
#[cfg_attr(not(CONFIG_MEMORY_HOTPLUG), link_section = ".init.text")]
unsafe fn split_mem_range(
    mr: *mut map_range,
    mut nr: usize,
    start: c_ulong,
    end: c_ulong,
) -> usize {
    let limit = end >> PAGE_SHIFT;
    let mut pfn = start >> PAGE_SHIFT;
    let mut end_pfn = if cfg!(CONFIG_X86_32) && pfn == 0 {
        PMD_SIZE >> PAGE_SHIFT
    } else {
        round_up(pfn, PMD_SIZE >> PAGE_SHIFT)
    };
    end_pfn = min(end_pfn, limit);
    if pfn < end_pfn {
        nr = save_mr(mr, nr, pfn, end_pfn, 0);
        pfn = end_pfn;
    }
    let mut start_pfn = round_up(pfn, PMD_SIZE >> PAGE_SHIFT);
    end_pfn = if cfg!(CONFIG_X86_32) {
        round_down(limit, PMD_SIZE >> PAGE_SHIFT)
    } else {
        min(
            round_up(pfn, PUD_SIZE >> PAGE_SHIFT),
            round_down(limit, PMD_SIZE >> PAGE_SHIFT),
        )
    };
    if start_pfn < end_pfn {
        nr = save_mr(
            mr,
            nr,
            start_pfn,
            end_pfn,
            page_size_mask as c_ulong & (1 << PG_LEVEL_2M),
        );
        pfn = end_pfn;
    }
    #[cfg(CONFIG_X86_64)]
    {
        start_pfn = round_up(pfn, PUD_SIZE >> PAGE_SHIFT);
        end_pfn = round_down(limit, PUD_SIZE >> PAGE_SHIFT);
        if start_pfn < end_pfn {
            nr = save_mr(
                mr,
                nr,
                start_pfn,
                end_pfn,
                page_size_mask as c_ulong & ((1 << PG_LEVEL_2M) | (1 << PG_LEVEL_1G)),
            );
            pfn = end_pfn;
        }
        start_pfn = round_up(pfn, PMD_SIZE >> PAGE_SHIFT);
        end_pfn = round_down(limit, PMD_SIZE >> PAGE_SHIFT);
        if start_pfn < end_pfn {
            nr = save_mr(
                mr,
                nr,
                start_pfn,
                end_pfn,
                page_size_mask as c_ulong & (1 << PG_LEVEL_2M),
            );
            pfn = end_pfn;
        }
    }
    nr = save_mr(mr, nr, pfn, limit, 0);
    if after_bootmem == 0 {
        adjust_range_page_size_mask(mr, nr);
    }
    // memmove semantics: the ranges overlap. Revisit this index after a merge.
    let mut i = 0;
    while nr > 1 && i < nr - 1 {
        if (*mr.add(i)).end != (*mr.add(i + 1)).start
            || (*mr.add(i)).page_size_mask != (*mr.add(i + 1)).page_size_mask
        {
            i += 1;
            continue;
        }
        let old_start = (*mr.add(i)).start;
        core::ptr::copy(mr.add(i + 1), mr.add(i), nr - 1 - i);
        (*mr.add(i)).start = old_start;
        nr -= 1;
    }
    for i in 0..nr {
        b::rust_mm_debug_range(
            (*mr.add(i)).start,
            (*mr.add(i)).end.wrapping_sub(1),
            page_size_string(mr.add(i)),
        );
    }
    nr
}
#[no_mangle]
pub static mut pfn_mapped: [range; b::RUST_MM_E820_MAX_ENTRIES as usize] =
    unsafe { MaybeUninit::zeroed().assume_init() };
#[no_mangle]
pub static mut nr_pfn_mapped: c_int = 0;
unsafe fn add_pfn_range_mapped(start: c_ulong, end: c_ulong) {
    let ranges = addr_of_mut!(pfn_mapped).cast();
    nr_pfn_mapped = add_range_with_merge(
        ranges,
        b::RUST_MM_E820_MAX_ENTRIES as c_int,
        nr_pfn_mapped,
        start as u64,
        end as u64,
    );
    nr_pfn_mapped = clean_sort_range(ranges, b::RUST_MM_E820_MAX_ENTRIES as c_int);
    max_pfn_mapped = max(max_pfn_mapped, end);
    let low_limit = (1 as c_ulong) << (32 - PAGE_SHIFT);
    if start < low_limit {
        max_low_pfn_mapped = max(max_low_pfn_mapped, min(end, low_limit));
    }
}
#[no_mangle]
pub unsafe extern "C" fn pfn_range_is_mapped(start: c_ulong, end: c_ulong) -> bool {
    for i in 0..nr_pfn_mapped as usize {
        let r = addr_of!(pfn_mapped).cast::<range>().add(i);
        if start as u64 >= (*r).start && end as u64 <= (*r).end {
            return true;
        }
    }
    false
}
#[no_mangle]
#[link_section = ".ref.text"]
pub unsafe extern "C" fn init_memory_mapping(
    start: c_ulong,
    end: c_ulong,
    prot: pgprot_t,
) -> c_ulong {
    b::rust_mm_debug_init_mapping(start, end.wrapping_sub(1));
    let mut mr = [map_range {
        start: 0,
        end: 0,
        page_size_mask: 0,
    }; NR_RANGE_MR];
    let nr = split_mem_range(mr.as_mut_ptr(), 0, start, end);
    let mut ret = 0;
    for r in &mr[..nr] {
        ret = kernel_physical_mapping_init(r.start, r.end, r.page_size_mask as c_ulong, prot);
    }
    add_pfn_range_mapped(start >> PAGE_SHIFT, ret >> PAGE_SHIFT);
    ret >> PAGE_SHIFT
}
#[cold]
#[link_section = ".init.text"]
unsafe fn init_range_memory_mapping(r_start: c_ulong, r_end: c_ulong) -> c_ulong {
    let mut i: c_int = -1;
    let (mut start_pfn, mut end_pfn): (c_ulong, c_ulong) = (0, 0);
    let mut mapped_ram_size: c_ulong = 0;
    loop {
        __next_mem_pfn_range(
            &mut i,
            b::RUST_MM_MAX_NUMNODES as c_int,
            &mut start_pfn,
            &mut end_pfn,
            null_mut(),
        );
        if i < 0 {
            break;
        }
        let start = max(
            r_start as u64,
            min(r_end as u64, (start_pfn as u64).wrapping_shl(PAGE_SHIFT)),
        );
        let end = max(
            r_start as u64,
            min(r_end as u64, (end_pfn as u64).wrapping_shl(PAGE_SHIFT)),
        );
        if start >= end {
            continue;
        }
        can_use_brk_pgt = max(start, (pgt_buf_end as u64).wrapping_shl(PAGE_SHIFT))
            >= min(end, (pgt_buf_top as u64).wrapping_shl(PAGE_SHIFT));
        init_memory_mapping(start as c_ulong, end as c_ulong, b::rust_mm_page_kernel());
        mapped_ram_size = mapped_ram_size.wrapping_add(end.wrapping_sub(start) as c_ulong);
        can_use_brk_pgt = true;
    }
    mapped_ram_size
}
#[cold]
#[link_section = ".init.text"]
fn get_new_step_size(step: c_ulong) -> c_ulong {
    step.wrapping_shl(PMD_SHIFT - PAGE_SHIFT - 1)
}
#[cold]
#[link_section = ".init.text"]
unsafe fn memory_map_top_down(map_start: c_ulong, map_end: c_ulong) {
    let addr = memblock_phys_alloc_range(
        PMD_SIZE as phys_addr_t,
        PMD_SIZE as phys_addr_t,
        map_start as phys_addr_t,
        map_end as phys_addr_t,
    ) as c_ulong;
    let real_end;
    if addr == 0 {
        b::rust_mm_log_release_failed();
        real_end = max(map_start, round_down(map_end, PMD_SIZE));
    } else {
        memblock_phys_free(addr as phys_addr_t, PMD_SIZE as phys_addr_t);
        real_end = addr.wrapping_add(PMD_SIZE);
    }
    let mut step_size = PMD_SIZE;
    max_pfn_mapped = 0;
    min_pfn_mapped = real_end >> PAGE_SHIFT;
    let mut last_start = real_end;
    let mut mapped_ram_size: c_ulong = 0;
    while last_start > map_start {
        let start = if last_start > step_size {
            max(round_down(last_start.wrapping_sub(1), step_size), map_start)
        } else {
            map_start
        };
        mapped_ram_size =
            mapped_ram_size.wrapping_add(init_range_memory_mapping(start, last_start));
        last_start = start;
        min_pfn_mapped = last_start >> PAGE_SHIFT;
        if mapped_ram_size >= step_size {
            step_size = get_new_step_size(step_size);
        }
    }
    if real_end < map_end {
        init_range_memory_mapping(real_end, map_end);
    }
}
#[cold]
#[link_section = ".init.text"]
unsafe fn memory_map_bottom_up(map_start: c_ulong, map_end: c_ulong) {
    let mut start = map_start;
    let mut step_size = PMD_SIZE;
    let mut mapped_ram_size: c_ulong = 0;
    min_pfn_mapped = start >> PAGE_SHIFT;
    while start < map_end {
        let next = if step_size != 0 && map_end.wrapping_sub(start) > step_size {
            min(round_up(start.wrapping_add(1), step_size), map_end)
        } else {
            map_end
        };
        mapped_ram_size = mapped_ram_size.wrapping_add(init_range_memory_mapping(start, next));
        start = next;
        if mapped_ram_size >= step_size {
            step_size = get_new_step_size(step_size);
        }
    }
}
#[cold]
#[link_section = ".init.text"]
unsafe fn init_trampoline() {
    #[cfg(CONFIG_X86_64)]
    {
        if !b::rust_mm_kaslr_memory_enabled() {
            trampoline_pgd_entry = *addr_of!(init_top_pgt)
                .cast::<pgd_t>()
                .add(pgd_index(b::rust_mm_page_offset_base()));
        } else {
            b::rust_mm_init_trampoline_kaslr();
        }
    }
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn init_mem_mapping() {
    b::rust_mm_pti_check_boottime_disable();
    probe_page_size_mask();
    setup_pcid();
    let end = if cfg!(CONFIG_X86_64) {
        max_pfn.wrapping_shl(PAGE_SHIFT)
    } else {
        max_low_pfn.wrapping_shl(PAGE_SHIFT)
    };
    init_memory_mapping(0, b::RUST_MM_ISA_END_ADDRESS, b::rust_mm_page_kernel());
    init_trampoline();
    if b::rust_mm_memblock_bottom_up() {
        let kernel_end = b::rust_mm_pa_symbol(addr_of!(_end).cast());
        memory_map_bottom_up(kernel_end, end);
        memory_map_bottom_up(b::RUST_MM_ISA_END_ADDRESS, kernel_end);
    } else {
        memory_map_top_down(b::RUST_MM_ISA_END_ADDRESS, end);
    }
    #[cfg(CONFIG_X86_64)]
    if max_pfn > max_low_pfn {
        max_low_pfn = max_pfn;
    }
    #[cfg(CONFIG_X86_32)]
    early_ioremap_page_table_range_init();
    b::rust_mm_load_swapper_cr3();
    b::rust_mm___flush_tlb_all();
    ((*addr_of!(x86_init.hyper.init_mem_mapping)).unwrap())();
    b::rust_mm_early_memtest(0, max_pfn_mapped.wrapping_shl(PAGE_SHIFT) as phys_addr_t);
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn poking_init() {
    let mut ptl: *mut spinlock_t = null_mut();
    text_poke_mm = mm_alloc();
    bug_on(text_poke_mm.is_null());
    b::rust_mm_paravirt_enter_mmap(text_poke_mm);
    b::rust_mm_set_notrack_mm(text_poke_mm);
    text_poke_mm_addr = b::rust_mm_task_unmapped_base();
    #[cfg(CONFIG_RANDOMIZE_BASE)]
    {
        text_poke_mm_addr = text_poke_mm_addr.wrapping_add(
            (kaslr_get_random_long(c"Poking".as_ptr().cast::<c_char>()) & PAGE_MASK)
                % b::rust_mm_task_size()
                    .wrapping_sub(b::rust_mm_task_unmapped_base())
                    .wrapping_sub(3 * PAGE_SIZE),
        );
    }
    if text_poke_mm_addr.wrapping_add(PAGE_SIZE) & !PMD_MASK == 0 {
        text_poke_mm_addr = text_poke_mm_addr.wrapping_add(PAGE_SIZE);
    }
    let ptep = b::rust_mm_get_locked_pte(text_poke_mm, text_poke_mm_addr, &mut ptl);
    bug_on(ptep.is_null());
    b::rust_mm_pte_unmap_unlock(ptep, ptl);
}
#[no_mangle]
pub unsafe extern "C" fn devmem_is_allowed(pagenr: c_ulong) -> c_int {
    if region_intersects(
        (pagenr as resource_size_t).wrapping_shl(PAGE_SHIFT),
        PAGE_SIZE as usize,
        b::RUST_MM_IORESOURCE_SYSTEM_RAM,
        IORES_DESC_NONE as c_ulong,
    ) != REGION_DISJOINT as c_int
    {
        return if pagenr < 256 { 2 } else { 0 };
    }
    if iomem_is_exclusive(pagenr.wrapping_shl(PAGE_SHIFT) as u64) {
        return if pagenr < 256 { 1 } else { 0 };
    }
    1
}
#[no_mangle]
pub unsafe extern "C" fn free_init_pages(
    what: *const c_char,
    mut begin: c_ulong,
    mut end: c_ulong,
) {
    let begin_aligned = round_up(begin, PAGE_SIZE);
    let end_aligned = end & PAGE_MASK;
    if b::rust_mm_warn_free_init_alignment(begin_aligned != begin || end_aligned != end) {
        begin = begin_aligned;
        end = end_aligned;
    }
    if begin >= end {
        return;
    }
    if b::rust_mm_debug_pagealloc_enabled() {
        b::rust_mm_log_unmapping_init(begin, end.wrapping_sub(1));
        b::rust_mm_kmemleak_free_part(begin as *const c_void, end.wrapping_sub(begin) as usize);
        set_memory_np(begin, (end.wrapping_sub(begin) >> PAGE_SHIFT) as c_int);
    } else {
        set_memory_nx(begin, (end.wrapping_sub(begin) >> PAGE_SHIFT) as c_int);
        set_memory_rw(begin, (end.wrapping_sub(begin) >> PAGE_SHIFT) as c_int);
        free_reserved_area(
            begin as *mut c_void,
            end as *mut c_void,
            b::RUST_MM_POISON_FREE_INITMEM as c_int,
            what,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn free_kernel_image_pages(
    what: *const c_char,
    begin: *mut c_void,
    end: *mut c_void,
) {
    let b = begin as c_ulong;
    let e = end as c_ulong;
    let len_pages = e.wrapping_sub(b) >> PAGE_SHIFT;
    free_init_pages(what, b, e);
    #[cfg(CONFIG_X86_64)]
    if b::rust_mm_feature_pti() {
        set_memory_np_noalias(b, len_pages as c_int);
    }
}
#[no_mangle]
#[link_section = ".ref.text"]
pub unsafe extern "C" fn free_initmem() {
    e820__reallocate_tables();
    b::rust_mm_mem_encrypt_free_decrypted_mem();
    free_kernel_image_pages(
        c"unused kernel image (initmem)".as_ptr().cast::<c_char>(),
        addr_of_mut!(__init_begin).cast(),
        addr_of_mut!(__init_end).cast(),
    );
}
#[cfg(CONFIG_BLK_DEV_INITRD)]
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn free_initrd_mem(start: c_ulong, end: c_ulong) {
    free_init_pages(
        c"initrd".as_ptr().cast::<c_char>(),
        start,
        round_up(end, PAGE_SIZE),
    );
}
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn arch_zone_limits_init(max_zone_pfns: *mut c_ulong) {
    #[cfg(CONFIG_ZONE_DMA)]
    {
        *max_zone_pfns.add(ZONE_DMA as usize) = min(b::RUST_MM_MAX_DMA_PFN, max_low_pfn);
    }
    #[cfg(CONFIG_ZONE_DMA32)]
    {
        *max_zone_pfns.add(ZONE_DMA32 as usize) = min(b::RUST_MM_MAX_DMA32_PFN, max_low_pfn);
    }
    *max_zone_pfns.add(ZONE_NORMAL as usize) = max_low_pfn;
    #[cfg(CONFIG_HIGHMEM)]
    {
        *max_zone_pfns.add(ZONE_HIGHMEM as usize) = max_pfn;
    }
}
#[no_mangle]
pub unsafe extern "C" fn update_cache_mode_entry(entry: c_uint, cache: page_cache_mode) {
    bug_on(entry == 0 && cache != _PAGE_CACHE_MODE_WB);
    *addr_of_mut!(__cachemode2pte_tbl)
        .cast::<u16>()
        .add(cache as usize) = cm_idx2pte(entry) as u16;
    *addr_of_mut!(__pte2cachemode_tbl)
        .cast::<u8>()
        .add(entry as usize) = cache as u8;
}
#[cfg(CONFIG_SWAP)]
#[no_mangle]
pub unsafe extern "C" fn arch_max_swapfile_size() -> c_ulong {
    let mut pages = generic_max_swapfile_size();
    if b::rust_mm_boot_bug_l1tf() && l1tf_mitigation != L1TF_MITIGATION_OFF {
        let mut limit = b::rust_mm_l1tf_pfn_limit() as u64;
        // PAE and x86-64 have >2 levels; native header carries the exact shift.
        limit = limit.wrapping_shl(b::RUST_MM_SWAP_LIMIT_SHIFT);
        pages = min(limit, pages as u64) as c_ulong;
    }
    pages
}
#[cfg(CONFIG_EXECMEM)]
#[link_section = ".data..ro_after_init"]
static mut execmem_info: b::execmem_info = unsafe { MaybeUninit::zeroed().assume_init() };
#[cfg(all(CONFIG_EXECMEM, CONFIG_ARCH_HAS_EXECMEM_ROX))]
#[no_mangle]
pub unsafe extern "C" fn execmem_fill_trapping_insns(ptr: *mut c_void, size: usize) {
    core::ptr::write_bytes(ptr.cast::<u8>(), b::RUST_MM_INT3_INSN_OPCODE as u8, size);
}
#[cfg(CONFIG_EXECMEM)]
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn execmem_arch_setup() -> *mut b::execmem_info {
    let offset = if b::rust_mm_kaslr_enabled() {
        (b::rust_mm_get_random_u32_inclusive(1, 1024) as c_ulong).wrapping_mul(PAGE_SIZE)
    } else {
        0
    };
    let start = b::rust_mm_modules_vaddr().wrapping_add(offset);
    let (prot, flags) = if cfg!(CONFIG_ARCH_HAS_EXECMEM_ROX) && b::rust_mm_feature_pse() {
        (
            b::rust_mm_page_kernel_rox(),
            EXECMEM_KASAN_SHADOW | EXECMEM_ROX_CACHE,
        )
    } else {
        (b::rust_mm_page_kernel(), EXECMEM_KASAN_SHADOW)
    };
    // Compound assignment in C zeros every unspecified range/member.
    core::ptr::write_bytes(addr_of_mut!(execmem_info), 0, 1);
    let ranges = addr_of_mut!(execmem_info.ranges).cast::<execmem_range>();
    for idx in [
        EXECMEM_MODULE_TEXT,
        EXECMEM_KPROBES,
        EXECMEM_FTRACE,
        EXECMEM_BPF,
        EXECMEM_MODULE_DATA,
    ] {
        let r = ranges.add(idx as usize);
        (*r).flags = if idx == EXECMEM_MODULE_DATA {
            EXECMEM_KASAN_SHADOW
        } else {
            flags
        };
        (*r).start = start;
        (*r).end = b::rust_mm_modules_end();
        (*r).pgprot = if idx == EXECMEM_KPROBES {
            b::rust_mm_page_kernel_rox()
        } else if idx == EXECMEM_MODULE_DATA {
            b::rust_mm_page_kernel()
        } else {
            prot
        };
        (*r).alignment = b::RUST_MM_MODULE_ALIGN as c_uint;
    }
    addr_of_mut!(execmem_info)
}
