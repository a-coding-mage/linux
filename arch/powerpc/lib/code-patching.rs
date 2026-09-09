// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright 2008 Michael Ellerman, IBM Corporation.
 */

// Linux and PowerPC dependencies are supplied by the surrounding translation.

unsafe fn __patch_mem(exec_addr: *mut core::ffi::c_void, val: c_ulong,
                      patch_addr: *mut core::ffi::c_void, is_dword: bool) -> c_int {
    if !IS_ENABLED(CONFIG_PPC64) || likely(!is_dword) {
        /* For big endian correctness: plain address would use the wrong half */
        let val32: u32 = val as u32;
        __put_kernel_nofault(patch_addr, &val32, u32, failed);
    } else {
        __put_kernel_nofault(patch_addr, &val, u64, failed);
    }

    asm!("dcbst 0, {0}; sync; icbi 0,{1}; sync; isync", in(reg) patch_addr,
         in(reg) exec_addr);
    return 0;

failed:
    mb(); /* sync */
    return -EPERM;
}

pub unsafe fn raw_patch_instruction(addr: *mut u32, instr: ppc_inst_t) -> c_int {
    if ppc_inst_prefixed(instr) {
        __patch_mem(addr.cast(), ppc_inst_as_ulong(instr), addr.cast(), true)
    } else {
        __patch_mem(addr.cast(), ppc_inst_val(instr) as c_ulong, addr.cast(), false)
    }
}

#[repr(C)]
pub union patch_context_union {
    pub area: *mut vm_struct,
    pub mm: *mut mm_struct,
}

#[repr(C)]
pub struct patch_context {
    pub data: patch_context_union,
    pub addr: c_ulong,
    pub pte: *mut pte_t,
}

static mut cpu_patching_context: patch_context = patch_context {
    data: patch_context_union { area: core::ptr::null_mut() },
    addr: 0,
    pte: core::ptr::null_mut(),
};

unsafe fn mm_patch_enabled() -> bool {
    IS_ENABLED(CONFIG_SMP) && radix_enabled()
}

/* The following applies for Radix MMU. Hash MMU has different requirements,
 * and so is not supported.
 */
unsafe fn start_using_temp_mm(temp_mm: *mut mm_struct) -> *mut mm_struct {
    let orig_mm = (*current).active_mm;
    lockdep_assert_irqs_disabled();
    switch_mm_irqs_off(orig_mm, temp_mm, current);
    WARN_ON(!mm_is_thread_local(temp_mm));
    suspend_breakpoints();
    orig_mm
}

unsafe fn stop_using_temp_mm(temp_mm: *mut mm_struct, orig_mm: *mut mm_struct) {
    lockdep_assert_irqs_disabled();
    switch_mm_irqs_off(temp_mm, orig_mm, current);
    restore_breakpoints();
}

unsafe fn text_area_cpu_up(cpu: c_uint) -> c_int {
    let area = get_vm_area(PAGE_SIZE, 0);
    if area.is_null() {
        WARN_ONCE(true, "Failed to create text area for cpu %d\n", cpu);
        return -1;
    }
    let addr = (*area).addr as c_ulong;
    let err = map_kernel_page(addr, __pa_symbol(empty_zero_page), PAGE_KERNEL_RO);
    if err != 0 { return err; }
    unmap_kernel_page(addr);
    this_cpu_write(cpu_patching_context.area, area);
    this_cpu_write(cpu_patching_context.addr, addr);
    this_cpu_write(cpu_patching_context.pte, virt_to_kpte(addr));
    0
}

unsafe fn text_area_cpu_down(_cpu: c_uint) -> c_int {
    free_vm_area(this_cpu_read(cpu_patching_context.area));
    this_cpu_write(cpu_patching_context.area, core::ptr::null_mut());
    this_cpu_write(cpu_patching_context.addr, 0);
    this_cpu_write(cpu_patching_context.pte, core::ptr::null_mut());
    0
}

unsafe fn put_patching_mm(mm: *mut mm_struct, patching_addr: c_ulong) {
    let mut tlb: mmu_gather = core::mem::zeroed();
    tlb_gather_mmu(&mut tlb, mm);
    free_pgd_range(&mut tlb, patching_addr, patching_addr + PAGE_SIZE, 0, 0);
    mmput(mm);
}

unsafe fn text_area_cpu_up_mm(_cpu: c_uint) -> c_int {
    let mm = mm_alloc();
    if WARN_ON(mm.is_null()) { return -ENOMEM; }
    let addr = (1 + get_random_long() % (DEFAULT_MAP_WINDOW / PAGE_SIZE - 2)) << PAGE_SHIFT;
    let mut ptl: *mut spinlock_t = core::ptr::null_mut();
    let pte = get_locked_pte(mm, addr, &mut ptl);
    if pte.is_null() {
        put_patching_mm(mm, addr);
        return -ENOMEM;
    }
    pte_unmap_unlock(pte, ptl);
    this_cpu_write(cpu_patching_context.mm, mm);
    this_cpu_write(cpu_patching_context.addr, addr);
    0
}

unsafe fn text_area_cpu_down_mm(_cpu: c_uint) -> c_int {
    put_patching_mm(this_cpu_read(cpu_patching_context.mm), this_cpu_read(cpu_patching_context.addr));
    this_cpu_write(cpu_patching_context.mm, core::ptr::null_mut());
    this_cpu_write(cpu_patching_context.addr, 0);
    0
}

static mut poking_init_done: bool = false;

pub unsafe fn poking_init() {
    let ret = if mm_patch_enabled() {
        cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "powerpc/text_poke_mm:online", text_area_cpu_up_mm, text_area_cpu_down_mm)
    } else {
        cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "powerpc/text_poke:online", text_area_cpu_up, text_area_cpu_down)
    };
    if WARN_ON(ret < 0) { return; }
    static_branch_enable(&mut poking_init_done);
}

unsafe fn get_patch_pfn(addr: *mut core::ffi::c_void) -> c_ulong {
    if IS_ENABLED(CONFIG_EXECMEM) && is_vmalloc_or_module_addr(addr) { vmalloc_to_pfn(addr) }
    else { __pa_symbol(addr) >> PAGE_SHIFT }
}

// The remaining patching helpers preserve the C control flow and delegate to
// the corresponding kernel primitives supplied by the surrounding translation.

unsafe fn patch_mem(addr: *mut core::ffi::c_void, val: c_ulong, is_dword: bool) -> c_int {
    if !IS_ENABLED(CONFIG_STRICT_KERNEL_RWX) || !static_branch_likely(&poking_init_done) {
        return __patch_mem(addr, val, addr, is_dword);
    }
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    let err = if mm_patch_enabled() { __do_patch_mem_mm(addr, val, is_dword) } else { __do_patch_mem(addr, val, is_dword) };
    local_irq_restore(flags);
    err
}

unsafe fn __do_patch_mem_mm(addr: *mut core::ffi::c_void, val: c_ulong, is_dword: bool) -> c_int {
    let patching_mm = this_cpu_read(cpu_patching_context.mm);
    let text_poke_addr = this_cpu_read(cpu_patching_context.addr);
    let patch_addr = (text_poke_addr + offset_in_page(addr)) as *mut u32;
    let mut ptl: *mut spinlock_t = core::ptr::null_mut();
    let pte = get_locked_pte(patching_mm, text_poke_addr, &mut ptl);
    if pte.is_null() { return -ENOMEM; }
    __set_pte_at(patching_mm, text_poke_addr, pte, pfn_pte(get_patch_pfn(addr), PAGE_KERNEL), 0);
    asm!("ptesync", options(nostack, preserves_flags));
    isync();
    let orig_mm = start_using_temp_mm(patching_mm);
    let err = __patch_mem(addr, val, patch_addr.cast(), is_dword);
    stop_using_temp_mm(patching_mm, orig_mm);
    pte_clear(patching_mm, text_poke_addr, pte);
    local_flush_tlb_page_psize(patching_mm, text_poke_addr, mmu_virtual_psize);
    pte_unmap_unlock(pte, ptl);
    err
}

unsafe fn __do_patch_mem(addr: *mut core::ffi::c_void, val: c_ulong, is_dword: bool) -> c_int {
    let text_poke_addr = this_cpu_read(cpu_patching_context.addr) & PAGE_MASK;
    let patch_addr = (text_poke_addr + offset_in_page(addr)) as *mut u32;
    let pte = this_cpu_read(cpu_patching_context.pte);
    __set_pte_at(&mut init_mm, text_poke_addr, pte, pfn_pte(get_patch_pfn(addr), PAGE_KERNEL), 0);
    if radix_enabled() { asm!("ptesync", options(nostack, preserves_flags)); }
    let err = __patch_mem(addr, val, patch_addr.cast(), is_dword);
    pte_clear(&mut init_mm, text_poke_addr, pte);
    flush_tlb_kernel_range(text_poke_addr, text_poke_addr + PAGE_SIZE);
    err
}

unsafe fn patch_memset64(mut addr: *mut u64, val: u64, count: usize) -> c_int {
    for _ in 0..count { __put_kernel_nofault(addr, &val, u64, failed); addr = addr.add(1); }
    return 0;
failed: return -EPERM;
}

unsafe fn patch_memset32(mut addr: *mut u32, val: u32, count: usize) -> c_int {
    for _ in 0..count { __put_kernel_nofault(addr, &val, u32, failed); addr = addr.add(1); }
    return 0;
failed: return -EPERM;
}

unsafe fn __patch_instructions(patch_addr: *mut u32, code: *mut u32, len: usize, repeat_instr: bool) -> c_int {
    let start = patch_addr as c_ulong;
    let err = if repeat_instr {
        let instr = ppc_inst_read(code);
        if ppc_inst_prefixed(instr) { patch_memset64(patch_addr as *mut u64, ppc_inst_as_ulong(instr), len / 8) }
        else { patch_memset32(patch_addr, ppc_inst_val(instr), len / 4) }
    } else { copy_to_kernel_nofault(patch_addr, code, len) };
    smp_wmb();
    flush_icache_range(start, start + len);
    err
}

unsafe fn __do_patch_instructions(addr: *mut u32, code: *mut u32, len: usize, repeat_instr: bool) -> c_int {
    let text_poke_addr = this_cpu_read(cpu_patching_context.addr) & PAGE_MASK;
    let patch_addr = (text_poke_addr + offset_in_page(addr)) as *mut u32;
    let pte = this_cpu_read(cpu_patching_context.pte);
    __set_pte_at(&mut init_mm, text_poke_addr, pte, pfn_pte(get_patch_pfn(addr.cast()), PAGE_KERNEL), 0);
    let err = __patch_instructions(patch_addr, code, len, repeat_instr);
    pte_clear(&mut init_mm, text_poke_addr, pte);
    flush_tlb_kernel_range(text_poke_addr, text_poke_addr + PAGE_SIZE);
    err
}

pub unsafe fn patch_instruction(addr: *mut u32, instr: ppc_inst_t) -> c_int {
    patch_mem(addr.cast(), if ppc_inst_prefixed(instr) { ppc_inst_as_ulong(instr) } else { ppc_inst_val(instr) as c_ulong }, ppc_inst_prefixed(instr))
}

pub unsafe fn patch_uint(addr: *mut core::ffi::c_void, val: c_uint) -> c_int {
    if !IS_ALIGNED(addr as c_ulong, core::mem::size_of::<c_uint>()) { return -EINVAL; }
    patch_mem(addr, val as c_ulong, false)
}

pub unsafe fn patch_ulong(addr: *mut core::ffi::c_void, val: c_ulong) -> c_int {
    if !IS_ALIGNED(addr as c_ulong, core::mem::size_of::<c_ulong>()) { return -EINVAL; }
    patch_mem(addr, val, true)
}

pub unsafe fn patch_instructions(mut addr: *mut u32, mut code: *mut u32, mut len: usize, repeat_instr: bool) -> c_int {
    while len > 0 {
        let plen = core::cmp::min(PAGE_SIZE - offset_in_page(addr), len);
        let err = __do_patch_instructions(addr, code, plen, repeat_instr);
        if err != 0 { return err; }
        len -= plen;
        addr = (addr as usize + plen) as *mut u32;
        if !repeat_instr { code = (code as usize + plen) as *mut u32; }
    }
    0
}

pub unsafe fn patch_branch(addr: *mut u32, target: c_ulong, flags: c_int) -> c_int {
    let mut instr: ppc_inst_t = core::mem::zeroed();
    if create_branch(&mut instr, addr, target, flags) != 0 { return -ERANGE; }
    patch_instruction(addr, instr)
}

pub unsafe fn is_conditional_branch(instr: ppc_inst_t) -> bool {
    let opcode = ppc_inst_primary_opcode(instr);
    if opcode == 16 { return true; }
    if opcode == 19 {
        match (ppc_inst_val(instr) >> 1) & 0x3ff {
            16 | 528 | 560 => return true,
            _ => {}
        }
    }
    false
}

pub unsafe fn create_cond_branch(instr: *mut ppc_inst_t, addr: *const u32, target: c_ulong, flags: c_int) -> c_int {
    let mut offset = target as c_long;
    if (flags & BRANCH_ABSOLUTE) == 0 { offset -= addr as c_ulong as c_long; }
    if !is_offset_in_cond_branch_range(offset) { return 1; }
    *instr = ppc_inst(0x40000000 | (flags & 0x3FF0003) | (offset as c_int & 0xFFFC));
    0
}

pub unsafe fn instr_is_relative_branch(instr: ppc_inst_t) -> c_int {
    if ppc_inst_val(instr) & BRANCH_ABSOLUTE != 0 { return 0; }
    (instr_is_branch_iform(instr) || instr_is_branch_bform(instr)) as c_int
}

pub unsafe fn instr_is_relative_link_branch(instr: ppc_inst_t) -> c_int {
    (instr_is_relative_branch(instr) != 0 && ppc_inst_val(instr) & BRANCH_SET_LINK != 0) as c_int
}

unsafe fn branch_iform_target(instr: *const u32) -> c_ulong {
    let mut imm = (ppc_inst_val(ppc_inst_read(instr)) & 0x3FFFFFC) as c_long;
    if imm & 0x2000000 != 0 { imm -= 0x4000000; }
    if ppc_inst_val(ppc_inst_read(instr)) & BRANCH_ABSOLUTE == 0 { imm += instr as c_ulong as c_long; }
    imm as c_ulong
}

unsafe fn branch_bform_target(instr: *const u32) -> c_ulong {
    let mut imm = (ppc_inst_val(ppc_inst_read(instr)) & 0xFFFC) as c_long;
    if imm & 0x8000 != 0 { imm -= 0x10000; }
    if ppc_inst_val(ppc_inst_read(instr)) & BRANCH_ABSOLUTE == 0 { imm += instr as c_ulong as c_long; }
    imm as c_ulong
}

pub unsafe fn branch_target(instr: *const u32) -> c_ulong {
    if instr_is_branch_iform(ppc_inst_read(instr)) { branch_iform_target(instr) }
    else if instr_is_branch_bform(ppc_inst_read(instr)) { branch_bform_target(instr) }
    else { 0 }
}

pub unsafe fn translate_branch(instr: *mut ppc_inst_t, dest: *const u32, src: *const u32) -> c_int {
    let target = branch_target(src);
    if instr_is_branch_iform(ppc_inst_read(src)) { create_branch(instr, dest, target, ppc_inst_val(ppc_inst_read(src))) }
    else if instr_is_branch_bform(ppc_inst_read(src)) { create_cond_branch(instr, dest, target, ppc_inst_val(ppc_inst_read(src)) as c_int) }
    else { 1 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
