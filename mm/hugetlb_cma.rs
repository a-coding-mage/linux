// SPDX-License-Identifier: GPL-2.0-only

// Kernel dependencies supplied by the surrounding translation unit.

static mut hugetlb_cma: [*mut cma; MAX_NUMNODES] = [core::ptr::null_mut(); MAX_NUMNODES];
static mut hugetlb_cma_size_in_node: [c_ulong; MAX_NUMNODES] = [0; MAX_NUMNODES];
static mut hugetlb_cma_only: bool = false;
static mut hugetlb_cma_size: c_ulong = 0;

static mut hugetlb_cma_percent: c_uint = 0;
static mut hugetlb_cma_percent_in_node: [c_uint; MAX_NUMNODES] = [0; MAX_NUMNODES];

#[cfg(CONFIG_NUMA)]
unsafe fn memblock_node_memory_size(nid: c_int) -> phys_addr_t {
    let mut size: phys_addr_t = 0;
    // for_each_mem_region(reg)
    for reg in for_each_mem_region() {
        if (*reg).nid == nid {
            size += (*reg).size;
        }
    }
    size
}

#[cfg(not(CONFIG_NUMA))]
unsafe fn memblock_node_memory_size(_nid: c_int) -> phys_addr_t {
    memblock_phys_mem_size()
}

pub unsafe fn hugetlb_cma_free_frozen_folio(folio: *mut folio) {
    WARN_ON_ONCE(!cma_release_frozen(
        hugetlb_cma[folio_nid(folio)],
        &mut (*folio).page,
        folio_nr_pages(folio),
    ));
}

pub unsafe fn hugetlb_cma_alloc_frozen_folio(
    order: c_int,
    gfp_mask: gfp_t,
    nid: c_int,
    nodemask: *mut nodemask_t,
) -> *mut folio {
    let mut page: *mut page = core::ptr::null_mut();

    if hugetlb_cma_size == 0 {
        return core::ptr::null_mut();
    }

    if !hugetlb_cma[nid as usize].is_null() {
        page = cma_alloc_frozen_compound(hugetlb_cma[nid as usize], order);
    }

    if page.is_null() && (gfp_mask & __GFP_THISNODE) == 0 {
        for node in for_each_node_mask(*nodemask) {
            if node == nid || hugetlb_cma[node as usize].is_null() {
                continue;
            }
            page = cma_alloc_frozen_compound(hugetlb_cma[node as usize], order);
            if !page.is_null() {
                break;
            }
        }
    }

    if page.is_null() {
        return core::ptr::null_mut();
    }
    let folio = page_folio(page);
    folio_set_hugetlb_cma(folio);
    folio
}

pub unsafe fn hugetlb_cma_alloc_bootmem(h: *mut hstate, nid: c_int, node_exact: bool) -> *mut c_void {
    let mut cma = hugetlb_cma[nid as usize];
    let mut m = cma_reserve_early(cma, huge_page_size(h));
    if !m.is_null() || node_exact {
        return m;
    }
    for node in for_each_node_mask(hugetlb_bootmem_nodes) {
        cma = hugetlb_cma[node as usize];
        if cma.is_null() || node == nid {
            continue;
        }
        m = cma_reserve_early(cma, huge_page_size(h));
        if !m.is_null() {
            return m;
        }
    }
    core::ptr::null_mut()
}

unsafe fn cmdline_parse_hugetlb_cma(mut p: *mut c_char) -> c_int {
    let mut nid: c_int;
    let mut count: c_int = 0;
    let mut tmp: c_ulong;
    let mut s = p;
    while *s != 0 {
        if sscanf(s, "%lu%n", &mut tmp, &mut count) != 1 { break; }
        if *s.add(count as usize) == b':' as c_char {
            let mut next: *mut c_char = core::ptr::null_mut();
            if tmp >= MAX_NUMNODES as c_ulong { break; }
            nid = array_index_nospec(tmp, MAX_NUMNODES as c_ulong) as c_int;
            hugetlb_cma_size = 0;
            hugetlb_cma_percent = 0;
            s = s.add(count as usize + 1);
            tmp = memparse(s, &mut next);
            if *next == b'%' as c_char {
                if tmp > 100 { pr_warn!("hugetlb_cma: invalid percentage %lu for node %d\n", tmp, nid); break; }
                hugetlb_cma_percent_in_node[nid as usize] = tmp as c_uint;
                hugetlb_cma_size_in_node[nid as usize] = 0;
                s = next.add(1);
            } else {
                hugetlb_cma_size_in_node[nid as usize] = tmp;
                hugetlb_cma_percent_in_node[nid as usize] = 0;
                s = next;
            }
            if *s == b',' as c_char { s = s.add(1); } else { break; }
        } else {
            let mut next: *mut c_char = core::ptr::null_mut();
            tmp = memparse(p, &mut next);
            if *next == b'%' as c_char {
                if tmp > 100 { pr_warn!("hugetlb_cma: invalid percentage %lu\n", tmp); }
                else { hugetlb_cma_percent = tmp as c_uint; hugetlb_cma_size = 0; for n in 0..MAX_NUMNODES { hugetlb_cma_size_in_node[n] = 0; hugetlb_cma_percent_in_node[n] = 0; } }
            } else { hugetlb_cma_size = tmp; hugetlb_cma_percent = 0; for n in 0..MAX_NUMNODES { hugetlb_cma_size_in_node[n] = 0; hugetlb_cma_percent_in_node[n] = 0; } }
            break;
        }
    }
    0
}

unsafe fn cmdline_parse_hugetlb_cma_only(p: *mut c_char) -> c_int { kstrtobool(p, &mut hugetlb_cma_only) }

pub unsafe fn arch_hugetlb_cma_order() -> c_uint { 0 }

pub unsafe fn hugetlb_cma_reserve() {
    let mut size: c_ulong;
    let mut reserved: c_ulong;
    let mut per_node: c_ulong = 0;
    let mut order: c_ulong;
    let mut gigantic_page_size: c_ulong;
    let mut node_specific_cma_alloc = false;
    let mut has_node_specific_param = false;

    for nid in 0..MAX_NUMNODES {
        if hugetlb_cma_size_in_node[nid] != 0 || hugetlb_cma_percent_in_node[nid] != 0 { has_node_specific_param = true; break; }
    }
    if has_node_specific_param {
        hugetlb_cma_size = 0;
        for nid in 0..MAX_NUMNODES {
            if hugetlb_cma_percent_in_node[nid] != 0 { hugetlb_cma_size_in_node[nid] = mul_u64_u32_div(memblock_node_memory_size(nid as c_int) as u64, hugetlb_cma_percent_in_node[nid], 100) as c_ulong; }
            hugetlb_cma_size += hugetlb_cma_size_in_node[nid];
        }
    } else if hugetlb_cma_percent != 0 { hugetlb_cma_size = mul_u64_u32_div(memblock_phys_mem_size() as u64, hugetlb_cma_percent, 100) as c_ulong; }
    if hugetlb_cma_size == 0 { return; }
    order = arch_hugetlb_cma_order() as c_ulong;
    if order == 0 { pr_warn!("hugetlb_cma: the option isn't supported by current arch\n"); return; }
    VM_WARN_ON(order <= MAX_PAGE_ORDER as c_ulong);
    gigantic_page_size = PAGE_SIZE << order;
    if hugetlb_cma_percent != 0 { hugetlb_cma_size = ALIGN_DOWN(hugetlb_cma_size, gigantic_page_size); }
    else if has_node_specific_param { hugetlb_cma_size = 0; for nid in 0..MAX_NUMNODES { if hugetlb_cma_percent_in_node[nid] != 0 { hugetlb_cma_size_in_node[nid] = ALIGN_DOWN(hugetlb_cma_size_in_node[nid], gigantic_page_size); } hugetlb_cma_size += hugetlb_cma_size_in_node[nid]; } }
    hugetlb_bootmem_set_nodes();
    for nid in 0..MAX_NUMNODES { size = hugetlb_cma_size_in_node[nid]; if size == 0 { continue; } if !node_isset(nid as c_int, hugetlb_bootmem_nodes) || !IS_ALIGNED(size, gigantic_page_size) { hugetlb_cma_size -= size; hugetlb_cma_size_in_node[nid] = 0; } else { node_specific_cma_alloc = true; } }
    if hugetlb_cma_size == 0 || !IS_ALIGNED(hugetlb_cma_size, gigantic_page_size) { if !IS_ALIGNED(hugetlb_cma_size, gigantic_page_size) { hugetlb_cma_size = 0; } return; }
    if !node_specific_cma_alloc { per_node = round_up(DIV_ROUND_UP(hugetlb_cma_size, nodes_weight(hugetlb_bootmem_nodes)), gigantic_page_size); }
    reserved = 0;
    for nid in for_each_node_mask(hugetlb_bootmem_nodes) {
        if node_specific_cma_alloc { if hugetlb_cma_size_in_node[nid as usize] == 0 { continue; } size = hugetlb_cma_size_in_node[nid as usize]; } else { size = min(per_node, hugetlb_cma_size - reserved); }
        let mut name = [0 as c_char; CMA_MAX_NAME]; snprintf(name.as_mut_ptr(), name.len(), "hugetlb%d", nid);
        let res = cma_declare_contiguous_multi(size, gigantic_page_size, HUGETLB_PAGE_ORDER, name.as_mut_ptr(), &mut hugetlb_cma[nid as usize], nid);
        if res != 0 || !cma_validate_zones(hugetlb_cma[nid as usize]) { hugetlb_cma[nid as usize] = core::ptr::null_mut(); continue; }
        reserved += size; if reserved >= hugetlb_cma_size { break; }
    }
    if reserved == 0 { hugetlb_cma_size = 0; }
}

pub unsafe fn hugetlb_cma_exclusive_alloc() -> bool { hugetlb_cma_only }
pub unsafe fn hugetlb_cma_total_size() -> c_ulong { hugetlb_cma_size }
pub unsafe fn hugetlb_cma_validate_params() { if hugetlb_cma_size == 0 { hugetlb_cma_only = false; } }
pub unsafe fn hugetlb_early_cma(h: *mut hstate) -> bool { !arch_has_huge_bootmem_alloc() && hstate_is_gigantic(h) && hugetlb_cma_only }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
