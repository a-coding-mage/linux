// SPDX-License-Identifier: GPL-2.0
/*
** Tablewalk MMU emulator
**
** by Toshiyasu Morita
**
** Started 1/16/98 @ 2:22 am
*/

// Linux and architecture headers supply the types, constants, macros, and
// external symbols referenced below.

pub const CONTEXTS_NUM: usize = 8;
pub const SEGMAPS_PER_CONTEXT_NUM: usize = 2048;
pub const PAGES_PER_SEGMENT: usize = 16;
pub const PMEGS_NUM: usize = 256;
pub const PMEG_MASK: usize = 0xff;

pub static mut m68k_vmalloc_end: libc::c_ulong = 0;
pub static mut pmeg_vaddr: [libc::c_ulong; PMEGS_NUM] = [0; PMEGS_NUM];
pub static mut pmeg_alloc: [libc::c_uchar; PMEGS_NUM] = [0; PMEGS_NUM];
pub static mut pmeg_ctx: [libc::c_uchar; PMEGS_NUM] = [0; PMEGS_NUM];

static mut ctx_alloc: [*mut mm_struct; CONTEXTS_NUM] = [core::ptr::null_mut(); CONTEXTS_NUM];
static mut ctx_avail: libc::c_uchar = (CONTEXTS_NUM - 1) as libc::c_uchar;
pub static mut rom_pages: [libc::c_ulong; 256] = [0; 256];

unsafe fn print_pte(pte: pte_t) {
    let val = pte_val(pte);
    let mut flags = [0i8; 7];
    flags[0] = if val & SUN3_PAGE_VALID != 0 { b'v' as i8 } else { b'-' as i8 };
    flags[1] = if val & SUN3_PAGE_WRITEABLE != 0 { b'w' as i8 } else { b'-' as i8 };
    flags[2] = if val & SUN3_PAGE_SYSTEM != 0 { b's' as i8 } else { b'-' as i8 };
    flags[3] = if val & SUN3_PAGE_NOCACHE != 0 { b'x' as i8 } else { b'-' as i8 };
    flags[4] = if val & SUN3_PAGE_ACCESSED != 0 { b'a' as i8 } else { b'-' as i8 };
    flags[5] = if val & SUN3_PAGE_MODIFIED != 0 { b'm' as i8 } else { b'-' as i8 };
    flags[6] = 0;
    let kind = match val & SUN3_PAGE_TYPE_MASK {
        SUN3_PAGE_TYPE_MEMORY => "memory",
        SUN3_PAGE_TYPE_IO => "io",
        SUN3_PAGE_TYPE_VME16 => "vme16",
        SUN3_PAGE_TYPE_VME32 => "vme32",
        _ => "unknown?",
    };
    pr_cont!(" pte={:08x} [{:07x} {:?} {}]\n", val, (val & SUN3_PAGE_PGNUM_MASK) << PAGE_SHIFT, flags, kind);
}

pub unsafe fn print_pte_vaddr(vaddr: libc::c_ulong) {
    pr_cont!(" vaddr={:x} [{:02x}]", vaddr, sun3_get_segmap(vaddr));
    print_pte(__pte(sun3_get_pte(vaddr)));
}

pub unsafe fn mmu_emu_init(mut bootmem_end: libc::c_ulong) {
    core::ptr::write_bytes(rom_pages.as_mut_ptr(), 0, rom_pages.len());
    core::ptr::write_bytes(pmeg_vaddr.as_mut_ptr(), 0, pmeg_vaddr.len());
    core::ptr::write_bytes(pmeg_alloc.as_mut_ptr(), 0, pmeg_alloc.len());
    core::ptr::write_bytes(pmeg_ctx.as_mut_ptr(), 0, pmeg_ctx.len());

    bootmem_end = (bootmem_end + 2 * SUN3_PMEG_SIZE) & !SUN3_PMEG_MASK;
    for i in 0..(__pa(bootmem_end) / SUN3_PMEG_SIZE) { pmeg_alloc[i as usize] = 2; }
    for num in 0xf0..=0xff { pmeg_alloc[num] = 2; }
    let mut seg = bootmem_end;
    while seg < 0x0f800000 {
        let i = sun3_get_segmap(seg) as usize;
        if pmeg_alloc[i] == 0 { sun3_put_segmap(seg, SUN3_INVALID_PMEG); }
        seg += SUN3_PMEG_SIZE;
    }
    let mut m68k_end = m68k_vmalloc_end;
    seg = 0x0f800000;
    while seg < 0x10000000 {
        if sun3_get_segmap(seg) != SUN3_INVALID_PMEG {
            if m68k_end == 0 { m68k_end = seg; }
            pmeg_alloc[sun3_get_segmap(seg) as usize] = 2;
        }
        seg += 16 * PAGE_SIZE;
    }
    m68k_vmalloc_end = m68k_end;
    dvma_init();
    seg = 0;
    while seg < PAGE_OFFSET { sun3_put_segmap(seg, SUN3_INVALID_PMEG); seg += SUN3_PMEG_SIZE; }
    set_fc(3);
    seg = 0;
    while seg < 0x10000000 {
        let i = sun3_get_segmap(seg);
        for j in 1..CONTEXTS_NUM { ((*romvec).pv_setctxt)(j as _, seg as *mut _, i); }
        seg += SUN3_PMEG_SIZE;
    }
    set_fc(USER_DATA);
}

pub unsafe fn clear_context(context: libc::c_ulong) {
    if context != 0 {
        if ctx_alloc[context as usize].is_null() { panic!("clear_context: context not allocated\n"); }
        (*ctx_alloc[context as usize]).context = SUN3_INVALID_CONTEXT;
        ctx_alloc[context as usize] = core::ptr::null_mut();
        ctx_avail += 1;
    }
    let oldctx = sun3_get_context();
    sun3_put_context(context);
    for i in 0..SUN3_INVALID_PMEG {
        if pmeg_ctx[i as usize] == context as u8 && pmeg_alloc[i as usize] == 1 {
            sun3_put_segmap(pmeg_vaddr[i as usize], SUN3_INVALID_PMEG);
            pmeg_ctx[i as usize] = 0; pmeg_alloc[i as usize] = 0; pmeg_vaddr[i as usize] = 0;
        }
    }
    sun3_put_context(oldctx);
}

pub unsafe fn get_free_context(mm: *mut mm_struct) -> libc::c_ulong {
    let mut new = 1usize;
    static mut next_to_die: libc::c_uchar = 1;
    if ctx_avail == 0 {
        new = next_to_die as usize; clear_context(new as _);
        next_to_die = (next_to_die + 1) & 0x7; if next_to_die == 0 { next_to_die += 1; }
    } else {
        while new < CONTEXTS_NUM && !ctx_alloc[new].is_null() { new += 1; }
        if new == CONTEXTS_NUM { panic!("get_free_context: failed to find free context"); }
    }
    ctx_alloc[new] = mm; ctx_avail -= 1; new as _
}

pub unsafe fn mmu_emu_map_pmeg(mut context: libc::c_int, mut vaddr: libc::c_int) {
    static mut curr_pmeg: libc::c_uchar = 128;
    vaddr &= !(SUN3_PMEG_MASK as libc::c_int);
    while pmeg_alloc[curr_pmeg as usize] == 2 { curr_pmeg += 1; }
    if pmeg_alloc[curr_pmeg as usize] == 1 {
        sun3_put_context(pmeg_ctx[curr_pmeg as usize] as _);
        sun3_put_segmap(pmeg_vaddr[curr_pmeg as usize], SUN3_INVALID_PMEG);
        sun3_put_context(context as _);
    }
    if (vaddr as libc::c_ulong) >= PAGE_OFFSET {
        for i in 0..CONTEXTS_NUM { sun3_put_context(i as _); sun3_put_segmap(vaddr as _, curr_pmeg as _); }
        sun3_put_context(context as _); pmeg_alloc[curr_pmeg as usize] = 2; pmeg_ctx[curr_pmeg as usize] = 0;
    } else {
        pmeg_alloc[curr_pmeg as usize] = 1; pmeg_ctx[curr_pmeg as usize] = context as _; sun3_put_segmap(vaddr as _, curr_pmeg as _);
    }
    pmeg_vaddr[curr_pmeg as usize] = vaddr as _;
    let mut i = 0;
    while i < SUN3_PMEG_SIZE { sun3_put_pte(vaddr as libc::c_ulong + i, SUN3_PAGE_SYSTEM); i += SUN3_PTE_SIZE; }
    curr_pmeg += 1;
}

pub unsafe fn mmu_emu_handle_fault(vaddr: libc::c_ulong, read_flag: libc::c_int, kernel_fault: libc::c_int) -> libc::c_int {
    let (crp, context) = if (*current).mm.is_null() { (swapper_pg_dir, 0) } else { ((*current).mm.as_ref().unwrap().pgd, (*current).mm.as_ref().unwrap().context) };
    let crp = if kernel_fault != 0 { swapper_pg_dir } else { crp };
    let segment = (vaddr >> SUN3_PMEG_SIZE_BITS) & 0x7ff;
    let offset = (vaddr >> SUN3_PTE_SIZE_BITS) & 0xf;
    let mut pte = pgd_val(*crp.add(segment as usize)) as *mut pte_t;
    if pte.is_null() { return 0; }
    pte = __va(pte.add(offset as usize) as libc::c_ulong) as *mut pte_t;
    if pte_val(*pte) & SUN3_PAGE_VALID == 0 { return 0; }
    if sun3_get_segmap(vaddr & !SUN3_PMEG_MASK) == SUN3_INVALID_PMEG { mmu_emu_map_pmeg(context as _, vaddr as _); }
    sun3_put_pte(vaddr & PAGE_MASK, pte_val(*pte));
    if read_flag == 0 {
        if pte_val(*pte) & SUN3_PAGE_WRITEABLE != 0 { pte_val(*pte) |= SUN3_PAGE_ACCESSED | SUN3_PAGE_MODIFIED; } else { return 0; }
    } else { pte_val(*pte) |= SUN3_PAGE_ACCESSED; }
    1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
