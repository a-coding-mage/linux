// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * This file contains the routines for handling the MMU on those
 * PowerPC implementations where the MMU substantially follows the
 * architecture specification. This includes the 6xx, 7xx, 7xxx,
 * and 8260 implementations but excludes the 8xx and 4xx.
 */

// Kernel and architecture declarations supplied by the surrounding build.

pub static mut early_hash: [u8; SZ_256K as usize] = [0; SZ_256K as usize];
static mut Hash: *mut hash_pte = early_hash.as_mut_ptr() as *mut hash_pte;
static mut Hash_size: c_ulong = 0;
static mut Hash_mask: c_ulong = 0;
static mut hash_mb: c_uint = 0;
static mut hash_mb2: c_uint = 0;
pub static mut _SDR1: c_ulong = 0;

pub static mut BATS: [[ppc_bat; 2]; 8] = [[ppc_bat { batu: 0, batl: 0 }; 2]; 8];

#[repr(C)]
struct batrange {
    start: c_ulong,
    limit: c_ulong,
    phys: phys_addr_t,
}
static mut bat_addrs: [batrange; 8] = [batrange { start: 0, limit: 0, phys: 0 }; 8];

#[cfg(CONFIG_SMP)]
pub static mut mmu_hash_lock: c_ulong = 0;

pub unsafe fn v_block_mapped(va: c_ulong) -> phys_addr_t {
    for b in 0..bat_addrs.len() {
        if va >= bat_addrs[b].start && va < bat_addrs[b].limit {
            return bat_addrs[b].phys + (va - bat_addrs[b].start);
        }
    }
    0
}

pub unsafe fn p_block_mapped(pa: phys_addr_t) -> c_ulong {
    for b in 0..bat_addrs.len() {
        if pa >= bat_addrs[b].phys
            && pa < (bat_addrs[b].limit - bat_addrs[b].start) + bat_addrs[b].phys
        {
            return bat_addrs[b].start + (pa - bat_addrs[b].phys);
        }
    }
    0
}

pub unsafe fn find_free_bat() -> c_int {
    let n = if mmu_has_feature(MMU_FTR_USE_HIGH_BATS) { 8 } else { 4 };
    for b in 0..n {
        let bat = BATS[b].as_ptr();
        if ((*bat.add(1)).batu & 3) == 0 {
            return b as c_int;
        }
    }
    -1
}

pub unsafe fn bat_block_size(base: c_ulong, top: c_ulong) -> c_uint {
    let max_size: c_uint = SZ_256M;
    let base_shift = (ffs(base) - 1) & 31;
    let block_shift = (fls(top - base) - 1) & 31;
    core::cmp::min(max_size, core::cmp::min(1u32 << base_shift, 1u32 << block_shift))
}

unsafe fn setibat(index: c_int, virt: c_ulong, phys: phys_addr_t, size: c_uint, prot: pgprot_t) {
    let bl = (size >> 17) - 1;
    let bat = BATS[index as usize].as_mut_ptr();
    let mut flags = pgprot_val(prot);
    if !cpu_has_feature(CPU_FTR_NEED_COHERENT) { flags &= !_PAGE_COHERENT; }
    let wimgxpp = (flags & _PAGE_COHERENT) | (if _PAGE_EXEC != 0 { BPP_RX } else { BPP_XX });
    (*bat).batu = virt | ((bl as c_ulong) << 2) | 2;
    (*bat).batl = BAT_PHYS_ADDR(phys) | wimgxpp;
    if !is_kernel_addr(virt) { (*bat).batu |= 1; }
}

unsafe fn clearibat(index: c_int) {
    let bat = BATS[index as usize].as_mut_ptr();
    (*bat).batu = 0;
    (*bat).batl = 0;
}

unsafe fn __mmu_mapin_ram(mut base: c_ulong, top: c_ulong) -> c_ulong {
    loop {
        let idx = find_free_bat();
        if idx == -1 || base == top { break; }
        let size = bat_block_size(base, top);
        if size < 128 << 10 { break; }
        setbat(idx, PAGE_OFFSET + base, base, size, PAGE_KERNEL_X);
        base += size as c_ulong;
    }
    base
}

pub unsafe fn mmu_mapin_ram(mut base: c_ulong, mut top: c_ulong) -> c_ulong {
    let border = __srwx_boundary as c_ulong - PAGE_OFFSET;
    let size = roundup_pow_of_two(_einittext as c_ulong - PAGE_OFFSET);
    setibat(0, PAGE_OFFSET, 0, size, PAGE_KERNEL_X);
    if debug_pagealloc_enabled_or_kfence() {
        pr_debug_once!("Read-Write memory mapped without BATs\n");
        if base >= border { return base; }
        if top >= border { top = border; }
    }
    if !strict_kernel_rwx_enabled() || base >= border || top <= border {
        return __mmu_mapin_ram(base, top);
    }
    let done = __mmu_mapin_ram(base, border);
    if done != border { return done; }
    __mmu_mapin_ram(border, top)
}

unsafe fn is_module_segment(addr: c_ulong) -> bool {
    if !IS_ENABLED(CONFIG_EXECMEM) || addr < ALIGN_DOWN(MODULES_VADDR, SZ_256M) { return false; }
    if addr > ALIGN(MODULES_END, SZ_256M) - 1 { return false; }
    true
}

pub unsafe fn mmu_mark_initmem_nx() -> c_int {
    let nb = if mmu_has_feature(MMU_FTR_USE_HIGH_BATS) { 8 } else { 4 };
    let mut i = 0;
    let mut base = _stext as c_ulong - PAGE_OFFSET;
    let top = ALIGN(_etext as c_ulong - PAGE_OFFSET, SZ_128K);
    let border = __init_begin as c_ulong - PAGE_OFFSET;
    while i < nb - 1 && base < top {
        let size = bat_block_size(base, top);
        setibat(i, PAGE_OFFSET + base, base, size, PAGE_KERNEL_X);
        i += 1; base += size as c_ulong;
    }
    if base < top {
        let mut size = bat_block_size(base, top);
        if top - base > size as c_ulong {
            size <<= 1;
            if strict_kernel_rwx_enabled() && base + size as c_ulong > border { pr_warn!("Some RW data is getting mapped X. Adjust CONFIG_DATA_SHIFT to avoid that.\n"); }
        }
        setibat(i, PAGE_OFFSET + base, base, size, PAGE_KERNEL_X); base += size as c_ulong; i += 1;
    }
    while i < nb { clearibat(i); i += 1; }
    update_bats();
    for j in (ALIGN(TASK_SIZE, SZ_256M) >> 28)..16 {
        if is_module_segment(j << 28) { continue; }
        mtsr(mfsr(j << 28) | 0x10000000, j << 28);
    }
    0
}

pub unsafe fn mmu_mark_rodata_ro() -> c_int {
    let nb = if mmu_has_feature(MMU_FTR_USE_HIGH_BATS) { 8 } else { 4 };
    for i in 0..nb {
        let bat = BATS[i].as_mut_ptr();
        if bat_addrs[i].start < __end_rodata as c_ulong { (*bat.add(1)).batl = ((*bat.add(1)).batl & !BPP_RW) | BPP_RX; }
    }
    update_bats(); 0
}

pub unsafe fn setbat(index_in: c_int, virt: c_ulong, phys: phys_addr_t, size: c_uint, prot: pgprot_t) {
    let mut index = index_in;
    let mut flags = pgprot_val(prot);
    if index == -1 { index = find_free_bat(); }
    if index == -1 { pr_err!("{}: no BAT available for mapping 0x{:llx}\n", "setbat", phys as u64); return; }
    let bat = BATS[index as usize].as_mut_ptr();
    if (flags & _PAGE_NO_CACHE) != 0 || !cpu_has_feature(CPU_FTR_NEED_COHERENT) { flags &= !_PAGE_COHERENT; }
    let bl = (size >> 17) - 1;
    let mut wimgxpp = flags & (_PAGE_WRITETHRU | _PAGE_NO_CACHE | _PAGE_COHERENT | _PAGE_GUARDED);
    wimgxpp |= if (flags & _PAGE_WRITE) != 0 { BPP_RW } else { BPP_RX };
    (*bat.add(1)).batu = virt | ((bl as c_ulong) << 2) | 2;
    (*bat.add(1)).batl = BAT_PHYS_ADDR(phys) | wimgxpp;
    if !is_kernel_addr(virt) { (*bat.add(1)).batu |= 1; }
    if (flags & _PAGE_GUARDED) != 0 { flags &= !_PAGE_EXEC; }
    bat_addrs[index as usize] = batrange { start: virt, limit: virt + (((bl + 1) as c_ulong) << 17) - 1, phys };
}

unsafe fn hash_preload(mm: *mut mm_struct, ea: c_ulong) {
    if !mmu_has_feature(MMU_FTR_HPTE_TABLE) { return; }
    let pmd = pmd_off(mm, ea);
    if !pmd_none(*pmd) { add_hash_page((*mm).context.id, ea, pmd_val(*pmd)); }
}

pub unsafe fn __update_mmu_cache(vma: *mut vm_area_struct, address: c_ulong, ptep: *mut pte_t) {
    if !pte_young(*ptep) || address >= TASK_SIZE || (*current).thread.regs.is_null() { return; }
    if TRAP((*current).thread.regs) != 0x300 && TRAP((*current).thread.regs) != 0x400 { return; }
    hash_preload((*vma).vm_mm, address);
}

pub unsafe fn MMU_init_hw() {
    if !mmu_has_feature(MMU_FTR_HPTE_TABLE) { return; }
    if ppc_md.progress.is_some() { ppc_md.progress.unwrap()("hash:enter".as_ptr() as *const c_char, 0x105); }
    let mut n_hpteg = total_memory / (PAGE_SIZE * 8);
    if n_hpteg < 1024 { n_hpteg = 1024; }
    let mut lg_n_hpteg = __ilog2(n_hpteg);
    if n_hpteg & (n_hpteg - 1) != 0 { lg_n_hpteg += 1; n_hpteg = 1 << lg_n_hpteg; }
    Hash_size = n_hpteg << 6;
    if ppc_md.progress.is_some() { ppc_md.progress.unwrap()("hash:find piece".as_ptr() as *const c_char, 0x322); }
    Hash = memblock_alloc_or_panic(Hash_size, Hash_size);
    _SDR1 = __pa(Hash) | ((n_hpteg - 1) >> 10);
    pr_info!("Total memory = {}MB; using {}kB for hash table\n", total_memory >> 20, Hash_size >> 10);
    Hash_mask = n_hpteg - 1;
    hash_mb2 = 32 - 6 - lg_n_hpteg; hash_mb = hash_mb2;
    if lg_n_hpteg > 16 { hash_mb2 = 16 - 6; }
}

pub unsafe fn MMU_init_hw_patch() {
    if !mmu_has_feature(MMU_FTR_HPTE_TABLE) { return; }
    let hmask = Hash_mask >> (16 - 6);
    let hash = Hash as c_ulong - PAGE_OFFSET;
    if ppc_md.progress.is_some() { ppc_md.progress.unwrap()("hash:patch".as_ptr() as *const c_char, 0x345); ppc_md.progress.unwrap()("hash:done".as_ptr() as *const c_char, 0x205); }
    modify_instruction_site(&patch__hash_page_A0, 0xffff, hash >> 16); modify_instruction_site(&patch__hash_page_A1, 0x7c0, hash_mb << 6); modify_instruction_site(&patch__hash_page_A2, 0x7c0, hash_mb2 << 6); modify_instruction_site(&patch__hash_page_B, 0xffff, hmask); modify_instruction_site(&patch__hash_page_C, 0xffff, hmask);
    modify_instruction_site(&patch__flush_hash_A0, 0xffff, hash >> 16); modify_instruction_site(&patch__flush_hash_A1, 0x7c0, hash_mb << 6); modify_instruction_site(&patch__flush_hash_A2, 0x7c0, hash_mb2 << 6); modify_instruction_site(&patch__flush_hash_B, 0xffff, hmask);
}

pub unsafe fn setup_initial_memory_limit(first_memblock_base: phys_addr_t, first_memblock_size: phys_addr_t) {
    BUG_ON(first_memblock_base != 0);
    memblock_set_current_limit(core::cmp::min(first_memblock_size, SZ_256M as phys_addr_t));
}

pub unsafe fn print_system_hash_info() { pr_info!("Hash_size         = 0x{:lx}\n", Hash_size); if Hash_mask != 0 { pr_info!("Hash_mask         = 0x{:lx}\n", Hash_mask); } }

pub unsafe fn early_init_mmu() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
