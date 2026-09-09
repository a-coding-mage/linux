// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2016, Rashmica Gupta, IBM Corp.
 *
 * This traverses the kernel virtual memory and dumps the pages that are in
 * the hash pagetable, along with their flags to
 * /sys/kernel/debug/kernel_hash_pagetable.
 *
 * If radix is enabled then there is no hash page table and so no debugfs file
 * is generated.
 */

#[repr(C)]
struct pg_state {
    seq: *mut seq_file,
    marker: *const addr_marker,
    start_address: usize,
    level: u32,
    current_flags: u64,
}

#[repr(C)]
struct addr_marker {
    start_address: usize,
    name: *const c_char,
}

static mut address_markers: [addr_marker; 11] = [
    addr_marker { start_address: 0, name: c"Start of kernel VM".as_ptr() },
    addr_marker { start_address: 0, name: c"vmalloc() Area".as_ptr() },
    addr_marker { start_address: 0, name: c"vmalloc() End".as_ptr() },
    addr_marker { start_address: 0, name: c"isa I/O start".as_ptr() },
    addr_marker { start_address: 0, name: c"isa I/O end".as_ptr() },
    addr_marker { start_address: 0, name: c"phb I/O start".as_ptr() },
    addr_marker { start_address: 0, name: c"phb I/O end".as_ptr() },
    addr_marker { start_address: 0, name: c"I/O remap start".as_ptr() },
    addr_marker { start_address: 0, name: c"I/O remap end".as_ptr() },
    addr_marker { start_address: 0, name: c"vmemmap start".as_ptr() },
    addr_marker { start_address: usize::MAX, name: core::ptr::null() },
];

#[repr(C)]
struct flag_info {
    mask: u64,
    val: u64,
    set: *const c_char,
    clear: *const c_char,
    is_val: bool,
    shift: i32,
}

static v_flag_array: [flag_info; 4] = [
    flag_info { mask: SLB_VSID_B, val: SLB_VSID_B_256M, set: c"ssize: 256M".as_ptr(), clear: c"ssize: 1T  ".as_ptr(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_V_SECONDARY, val: HPTE_V_SECONDARY, set: c"secondary".as_ptr(), clear: c"primary  ".as_ptr(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_V_VALID, val: HPTE_V_VALID, set: c"valid  ".as_ptr(), clear: c"invalid".as_ptr(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_V_BOLTED, val: HPTE_V_BOLTED, set: c"bolted".as_ptr(), clear: c"".as_ptr(), is_val: false, shift: 0 },
];

static r_flag_array: [flag_info; 12] = [
    flag_info { mask: HPTE_R_PP0 | HPTE_R_PP, val: PP_RWXX, set: c"prot:RW--".as_ptr(), clear: core::ptr::null(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_R_PP0 | HPTE_R_PP, val: PP_RWRX, set: c"prot:RWR-".as_ptr(), clear: core::ptr::null(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_R_PP0 | HPTE_R_PP, val: PP_RWRW, set: c"prot:RWRW".as_ptr(), clear: core::ptr::null(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_R_PP0 | HPTE_R_PP, val: PP_RXRX, set: c"prot:R-R-".as_ptr(), clear: core::ptr::null(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_R_PP0 | HPTE_R_PP, val: PP_RXXX, set: c"prot:R---".as_ptr(), clear: core::ptr::null(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_R_KEY_HI | HPTE_R_KEY_LO, val: HPTE_R_KEY_HI | HPTE_R_KEY_LO, set: c"key".as_ptr(), clear: c"".as_ptr(), is_val: true, shift: 0 },
    flag_info { mask: HPTE_R_R, val: HPTE_R_R, set: c"ref".as_ptr(), clear: c"   ".as_ptr(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_R_C, val: HPTE_R_C, set: c"changed".as_ptr(), clear: c"       ".as_ptr(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_R_N, val: HPTE_R_N, set: c"no execute".as_ptr(), clear: core::ptr::null(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_R_WIMG, val: HPTE_R_W, set: c"writethru".as_ptr(), clear: core::ptr::null(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_R_WIMG, val: HPTE_R_I, set: c"no cache".as_ptr(), clear: core::ptr::null(), is_val: false, shift: 0 },
    flag_info { mask: HPTE_R_WIMG, val: HPTE_R_G, set: c"guarded".as_ptr(), clear: core::ptr::null(), is_val: false, shift: 0 },
];

type c_char = i8;
type seq_file = core::ffi::c_void;

unsafe fn calculate_pagesize(st: *mut pg_state, mut ps: i32, s: *const c_char) -> i32 {
    let units = b"BKMGTPE\0";
    let mut unit = 0usize;
    while ps > 9 && units[unit + 1] != 0 { ps -= 10; unit += 1; }
    seq_printf((*st).seq, c"  %s_ps: %i%c\t".as_ptr(), s, 1i32 << ps, units[unit]);
    ps
}

unsafe fn dump_flag_info(st: *mut pg_state, mut flag: *const flag_info, pte: u64, num: u32) {
    for _ in 0..num {
        if (*flag).mask != 0 {
            if (*flag).is_val {
                let mut val = pte & (*flag).val;
                if (*flag).shift != 0 { val >>= (*flag).shift; }
                seq_printf((*st).seq, c"  %s:%llx".as_ptr(), (*flag).set, val);
            } else {
                let s = if (pte & (*flag).mask) == (*flag).val { (*flag).set } else { (*flag).clear };
                if !s.is_null() { seq_printf((*st).seq, c"  %s".as_ptr(), s); }
            }
        }
        flag = flag.add(1);
    }
}

unsafe fn dump_hpte_info(st: *mut pg_state, ea: usize, v: u64, r: u64, rpn: usize, bps: i32, aps: i32, lp: usize) {
    while ea >= (*st).marker.add(1).read().start_address {
        (*st).marker = (*st).marker.add(1);
        seq_printf((*st).seq, c"---[ %s ]---\n".as_ptr(), (*(*st).marker).name);
    }
    seq_printf((*st).seq, c"0x%lx:\t".as_ptr(), ea);
    seq_printf((*st).seq, c"AVPN:%llx\t".as_ptr(), HPTE_V_AVPN_VAL(v));
    dump_flag_info(st, v_flag_array.as_ptr(), v, v_flag_array.len() as u32);
    seq_printf((*st).seq, c"  rpn: %lx\t".as_ptr(), rpn);
    dump_flag_info(st, r_flag_array.as_ptr(), r, r_flag_array.len() as u32);
    calculate_pagesize(st, bps, c"base".as_ptr());
    let aps_index = calculate_pagesize(st, aps, c"actual".as_ptr());
    if aps_index != 2 { seq_printf((*st).seq, c"LP enc: %lx".as_ptr(), lp); }
    seq_putc((*st).seq, b'\n' as i32);
}

// The remaining functions mirror the kernel page-table walk and HPTE lookup;
// their external kernel types, constants, and helpers are supplied elsewhere.
unsafe fn native_find(ea: usize, psize: i32, primary: bool, v: *mut u64, r: *mut u64) -> i32 {
    let ssize = mmu_kernel_ssize;
    let shift = mmu_psize_defs[psize as usize].shift;
    let vsid = get_kernel_vsid(ea, ssize); let vpn = hpt_vpn(ea, vsid, ssize);
    let mut hash = hpt_hash(vpn, shift, ssize); let mut want_v = hpte_encode_avpn(vpn, psize, ssize);
    if cpu_has_feature(CPU_FTR_ARCH_300) { want_v = hpte_old_to_new_v(want_v); }
    if !primary { hash = !hash; }
    let mut hpte_group = (hash & htab_hash_mask) * HPTES_PER_GROUP;
    for _ in 0..HPTES_PER_GROUP { let hptep = htab_address.add(hpte_group); let hpte_v = be64_to_cpu((*hptep).v);
        if HPTE_V_COMPARE(hpte_v, want_v) && (hpte_v & HPTE_V_VALID) != 0 { *v = hpte_v; *r = be64_to_cpu((*hptep).r); if cpu_has_feature(CPU_FTR_ARCH_300) { *v = hpte_new_to_old_v(*v, *r); *r = hpte_new_to_old_r(*r); } return 0; }
        hpte_group += 1;
    } -1
}

unsafe fn pseries_find(ea: usize, psize: i32, primary: bool, v: *mut u64, r: *mut u64) -> i32 {
    let mut ptes: [u64; 8] = [0; 8]; let ssize = mmu_kernel_ssize; let shift = mmu_psize_defs[psize as usize].shift;
    let vsid = get_kernel_vsid(ea, ssize); let vpn = hpt_vpn(ea, vsid, ssize); let mut hash = hpt_hash(vpn, shift, ssize); let want_v = hpte_encode_avpn(vpn, psize, ssize);
    if !primary { hash = !hash; } let mut hpte_group = (hash & htab_hash_mask) * HPTES_PER_GROUP;
    for _ in (0..HPTES_PER_GROUP).step_by(4) { if plpar_pte_read_4(0, hpte_group, ptes.as_mut_ptr() as *mut core::ffi::c_void) != 0 { hpte_group += 4; continue; }
        for j in 0..4 { if HPTE_V_COMPARE(ptes[j * 2], want_v) && (ptes[j * 2] & HPTE_V_VALID) != 0 { *v = ptes[j * 2]; *r = ptes[j * 2 + 1]; return 0; } } hpte_group += 4;
    } -1
}

unsafe fn base_hpte_find(ea: usize, psize: i32, primary: bool, v: *mut u64, r: *mut u64) -> i32 { if IS_ENABLED(CONFIG_PPC_PSERIES) && firmware_has_feature(FW_FEATURE_LPAR) { pseries_find(ea, psize, primary, v, r) } else { native_find(ea, psize, primary, v, r) } }

unsafe fn decode_r(bps: i32, r: usize, rpn: *mut usize, aps: *mut i32, lp_bits: *mut usize) { let arpn = (r & HPTE_R_RPN) >> HPTE_R_RPN_SHIFT; let lp = arpn & 0xff; let entry = mmu_psize_defs[bps as usize]; let mut idx = 0; while idx < MMU_PAGE_COUNT { let penc = entry.penc[idx]; if penc != -1 && mmu_psize_defs[idx].shift != 0 { let shift = mmu_psize_defs[idx].shift - HPTE_R_RPN_SHIFT; let mask = (1usize << shift) - 1; if (lp & mask) == penc { *aps = mmu_psize_to_shift(idx as i32); *lp_bits = lp & mask; *rpn = arpn >> shift; return; } } idx += 1; } *aps = -1; }

unsafe fn hpte_find(st: *mut pg_state, ea: usize, psize: i32) -> usize {
    if ea < PAGE_OFFSET { return usize::MAX; }
    let mut v = 0u64; let mut r = 0u64; let mut slot = base_hpte_find(ea, psize, true, &mut v, &mut r);
    if slot == -1 { slot = base_hpte_find(ea, psize, false, &mut v, &mut r); } if slot == -1 { return usize::MAX; }
    let base_psize = mmu_psize_to_shift(psize); let mut actual_psize; let mut rpn = 0usize; let mut lp_bits = 0usize; if (v & HPTE_V_LARGE) == HPTE_V_LARGE { decode_r(psize, r as usize, &mut rpn, &mut actual_psize, &mut lp_bits); } else { actual_psize = 12; rpn = ((r & HPTE_R_RPN) >> HPTE_R_RPN_SHIFT) as usize; lp_bits = usize::MAX; }
    if actual_psize == -1 { return usize::MAX; } dump_hpte_info(st, ea, v, r, rpn as usize, base_psize, actual_psize, lp_bits); 0
}

unsafe fn walk_pte(st: *mut pg_state, pmd: *mut pmd_t, start: usize) {
    let mut pte = pte_offset_kernel(pmd, 0); for i in 0..PTRS_PER_PTE { let addr = start + i * PAGE_SIZE; let pteval = pte_val(*pte); let mut psize = if addr < VMALLOC_END { mmu_vmalloc_psize } else { mmu_io_psize };
        if IS_ENABLED(CONFIG_PPC_64K_PAGES) && ((pteval & H_PAGE_COMBO) == H_PAGE_COMBO || (pteval & H_PAGE_4K_PFN) == H_PAGE_4K_PFN) { psize = mmu_io_psize; }
        let status = hpte_find(st, addr, psize); if (pteval & H_PAGE_HASHPTE) != H_PAGE_HASHPTE && status != usize::MAX { seq_printf((*st).seq, c"page probably bolted before linux pagetables were set: addr:%lx, pteval:%lx\n".as_ptr(), addr, pteval); } pte = pte.add(1); }
}
unsafe fn walk_pmd(st: *mut pg_state, pud: *mut pud_t, start: usize) { let mut pmd = pmd_offset(pud, 0); for i in 0..PTRS_PER_PMD { let addr = start + i * PMD_SIZE; if !pmd_none(*pmd) { walk_pte(st, pmd, addr); } pmd = pmd.add(1); } }
unsafe fn walk_pud(st: *mut pg_state, p4d: *mut p4d_t, start: usize) { let mut pud = pud_offset(p4d, 0); for i in 0..PTRS_PER_PUD { let addr = start + i * PUD_SIZE; if !pud_none(*pud) { walk_pmd(st, pud, addr); } pud = pud.add(1); } }
unsafe fn walk_p4d(st: *mut pg_state, pgd: *mut pgd_t, start: usize) { let mut p4d = p4d_offset(pgd, 0); for i in 0..PTRS_PER_P4D { let addr = start + i * P4D_SIZE; if !p4d_none(*p4d) { walk_pud(st, p4d, addr); } p4d = p4d.add(1); } }
unsafe fn walk_pagetables(st: *mut pg_state) { let mut pgd = pgd_offset_k(0); for i in 0..PTRS_PER_PGD { let addr = KERN_VIRT_START + i * PGDIR_SIZE; if !pgd_none(*pgd) { walk_p4d(st, pgd, addr); } pgd = pgd.add(1); } }
unsafe fn walk_vmemmap(st: *mut pg_state) { if !IS_ENABLED(CONFIG_SPARSEMEM_VMEMMAP) { return; } let mut ptr = vmemmap_list; while !ptr.is_null() { hpte_find(st, (*ptr).virt_addr, mmu_vmemmap_psize); ptr = (*ptr).list; } seq_puts((*st).seq, c"---[ vmemmap end ]---\n".as_ptr()); }
unsafe fn ptdump_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 { let mut st = pg_state { seq: m, marker: address_markers.as_ptr(), start_address: PAGE_OFFSET, level: 0, current_flags: 0 }; walk_linearmapping(&mut st); walk_pagetables(&mut st); walk_vmemmap(&mut st); 0 }
unsafe fn ptdump_init() -> i32 { if !radix_enabled() { populate_markers(); debugfs_create_file(c"kernel_hash_pagetable".as_ptr(), 0o400, core::ptr::null_mut(), core::ptr::null_mut(), &ptdump_fops); } 0 }

unsafe fn walk_linearmapping(st: *mut pg_state) { let psize = 1usize << mmu_psize_defs[mmu_linear_psize as usize].shift; let mut addr = PAGE_OFFSET; while addr < PAGE_OFFSET + memblock_end_of_DRAM() { hpte_find(st, addr, mmu_linear_psize); addr += psize; } }
unsafe fn populate_markers() { address_markers[0].start_address = PAGE_OFFSET; address_markers[1].start_address = VMALLOC_START; address_markers[2].start_address = VMALLOC_END; address_markers[3].start_address = ISA_IO_BASE; address_markers[4].start_address = ISA_IO_END; address_markers[5].start_address = PHB_IO_BASE; address_markers[6].start_address = PHB_IO_END; address_markers[7].start_address = IOREMAP_BASE; address_markers[8].start_address = IOREMAP_END; address_markers[9].start_address = H_VMEMMAP_START; }

// External kernel declarations referenced above are intentionally not defined here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
