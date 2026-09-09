// SPDX-License-Identifier: GPL-2.0-or-later
/* PowerPC64 SLB support. */

// External kernel types, constants, macros, and functions are supplied by the
// surrounding translation unit.

static mut STRESS_SLB_ENABLED: bool = false;
static mut NO_SLB_PRELOAD: bool = false;

unsafe fn parse_stress_slb(_p: *mut core::ffi::c_char) -> i32 {
    STRESS_SLB_ENABLED = true;
    0
}

unsafe fn parse_no_slb_preload(_p: *mut core::ffi::c_char) -> i32 {
    NO_SLB_PRELOAD = true;
    0
}

unsafe fn assert_slb_presence(present: bool, mut ea: usize) {
    // CONFIG_DEBUG_VM conditional retained from the source.
    #[cfg(CONFIG_DEBUG_VM)]
    {
        let mut tmp: usize;
        WARN_ON_ONCE(mfmsr() & MSR_EE);
        if !cpu_has_feature(CPU_FTR_ARCH_206) { return; }
        ea &= !((1usize << SID_SHIFT) - 1);
        core::arch::asm!("slbfee. {0}, {1}", out(reg) tmp, in(reg) ea, options(nostack));
        WARN_ON(present == (tmp == 0));
    }
}

unsafe fn slb_shadow_update(ea: usize, ssize: i32, flags: usize, index: enum_slb_index) {
    let p = get_slb_shadow();
    WRITE_ONCE((*p).save_area[index as usize].esid, 0);
    WRITE_ONCE((*p).save_area[index as usize].vsid, cpu_to_be64(mk_vsid_data(ea, ssize, flags)));
    WRITE_ONCE((*p).save_area[index as usize].esid, cpu_to_be64(mk_esid_data(ea, ssize, index)));
}

unsafe fn slb_shadow_clear(index: enum_slb_index) {
    WRITE_ONCE((*get_slb_shadow()).save_area[index as usize].esid, cpu_to_be64(index as usize));
}

unsafe fn create_shadowed_slbe(ea: usize, ssize: i32, flags: usize, index: enum_slb_index) {
    slb_shadow_update(ea, ssize, flags, index);
    assert_slb_presence(false, ea);
    core::arch::asm!("slbmte {0}, {1}", in(reg) mk_vsid_data(ea, ssize, flags), in(reg) mk_esid_data(ea, ssize, index), options(nostack));
}

pub unsafe fn __slb_restore_bolted_realmode() {
    let p = get_slb_shadow();
    for index in 0..SLB_NUM_BOLTED {
        core::arch::asm!("slbmte {0}, {1}", in(reg) be64_to_cpu((*p).save_area[index].vsid), in(reg) be64_to_cpu((*p).save_area[index].esid), options(nostack));
    }
    assert_slb_presence(true, (*local_paca).kstack);
}

pub unsafe fn slb_restore_bolted_realmode() {
    __slb_restore_bolted_realmode();
    (*get_paca()).slb_cache_ptr = 0;
    (*get_paca()).slb_kern_bitmap = (1u32 << SLB_NUM_BOLTED) - 1;
    (*get_paca()).slb_used_bitmap = (*get_paca()).slb_kern_bitmap;
}

pub unsafe fn slb_flush_all_realmode() {
    core::arch::asm!("slbmte {0}, {0}; slbia", in(reg) 0usize, options(nostack));
}

unsafe fn __slb_flush_and_restore_bolted(preserve_kernel_lookaside: bool) {
    let p = get_slb_shadow();
    let ih: u32 = if preserve_kernel_lookaside { 1 } else { 0 };
    let ksp_esid_data = be64_to_cpu((*p).save_area[KSTACK_INDEX].esid);
    let ksp_vsid_data = be64_to_cpu((*p).save_area[KSTACK_INDEX].vsid);
    core::arch::asm!("slbia; slbmte {1}, {2}", const ih, in(reg) ksp_vsid_data, in(reg) ksp_esid_data, options(nostack));
}

pub unsafe fn slb_flush_and_restore_bolted() {
    BUILD_BUG_ON(SLB_NUM_BOLTED != 2);
    WARN_ON(!irqs_disabled());
    hard_irq_disable(); isync(); __slb_flush_and_restore_bolted(false); isync();
    assert_slb_presence(true, (*get_paca()).kstack);
    (*get_paca()).slb_cache_ptr = 0;
    (*get_paca()).slb_kern_bitmap = (1u32 << SLB_NUM_BOLTED) - 1;
    (*get_paca()).slb_used_bitmap = (*get_paca()).slb_kern_bitmap;
}

pub unsafe fn slb_save_contents(mut slb_ptr: *mut slb_entry) {
    (*get_paca()).slb_save_cache_ptr = (*get_paca()).slb_cache_ptr;
    if slb_ptr.is_null() { return; }
    for i in 0..mmu_slb_size {
        let (mut e, mut v): (usize, usize);
        core::arch::asm!("slbmfee {0}, {1}", out(reg) e, in(reg) i);
        core::arch::asm!("slbmfev {0}, {1}", out(reg) v, in(reg) i);
        (*slb_ptr).esid = e; (*slb_ptr).vsid = v; slb_ptr = slb_ptr.add(1);
    }
}

pub unsafe fn slb_dump_contents(mut slb_ptr: *mut slb_entry) {
    if slb_ptr.is_null() { return; }
    pr_err!("SLB contents of cpu 0x%x\n", smp_processor_id());
    for i in 0..mmu_slb_size {
        let e = (*slb_ptr).esid; let v = (*slb_ptr).vsid; slb_ptr = slb_ptr.add(1);
        if e == 0 && v == 0 { continue; }
        pr_err!("%02d %016lx %016lx %s\n", i, e, v, if e & SLB_ESID_V != 0 { "VALID" } else { "NOT VALID" });
        if e & SLB_ESID_V == 0 { continue; }
        let llp = v & SLB_VSID_LLP;
        if v & SLB_VSID_B_1T != 0 { pr_err!("     1T ESID=%9lx VSID=%13lx LLP:%3lx\n", GET_ESID_1T(e), (v & !SLB_VSID_B) >> SLB_VSID_SHIFT_1T, llp); }
        else { pr_err!("   256M ESID=%9lx VSID=%13lx LLP:%3lx\n", GET_ESID(e), (v & !SLB_VSID_B) >> SLB_VSID_SHIFT, llp); }
    }
    if !early_cpu_has_feature(CPU_FTR_ARCH_300) {
        pr_err!("SLB RR allocator index %d\n", (*get_paca()).stab_rr);
        pr_err!("SLB cache ptr value = %d\n", (*get_paca()).slb_save_cache_ptr);
        pr_err!("Valid SLB cache entries:\n");
        let n = core::cmp::min((*get_paca()).slb_save_cache_ptr, SLB_CACHE_ENTRIES);
        for i in 0..n { pr_err!("%02d EA[0-35]=%9x\n", i, (*get_paca()).slb_cache[i]); }
        pr_err!("Rest of SLB cache entries:\n");
        for i in n..SLB_CACHE_ENTRIES { pr_err!("%02d EA[0-35]=%9x\n", i, (*get_paca()).slb_cache[i]); }
    }
}

pub unsafe fn slb_vmalloc_update() { slb_flush_and_restore_bolted(); }

unsafe fn preload_hit(ti: *mut thread_info, esid: usize) -> bool {
    for i in 0..(*ti).slb_preload_nr { let idx = ((*ti).slb_preload_tail + i) % SLB_PRELOAD_NR; if esid == (*ti).slb_preload_esid[idx] { return true; } }
    false
}

unsafe fn preload_add(ti: *mut thread_info, mut ea: usize) {
    if slb_preload_disabled() { return; }
    if mmu_has_feature(MMU_FTR_1T_SEGMENT) && ea & ESID_MASK_1T != 0 { ea &= ESID_MASK_1T; }
    let esid = ea >> SID_SHIFT;
    if preload_hit(ti, esid) { return; }
    let idx = ((*ti).slb_preload_tail + (*ti).slb_preload_nr) % SLB_PRELOAD_NR;
    (*ti).slb_preload_esid[idx] = esid;
    if (*ti).slb_preload_nr == SLB_PRELOAD_NR { (*ti).slb_preload_tail = ((*ti).slb_preload_tail + 1) % SLB_PRELOAD_NR; } else { (*ti).slb_preload_nr += 1; }
}

unsafe fn preload_age(ti: *mut thread_info) { if (*ti).slb_preload_nr != 0 { (*ti).slb_preload_nr -= 1; (*ti).slb_preload_tail = ((*ti).slb_preload_tail + 1) % SLB_PRELOAD_NR; } }

unsafe fn slb_cache_slbie_kernel(index: u32) {
    let mut slbie_data = (*get_paca()).slb_cache[index as usize]; let ksp = (*get_paca()).kstack;
    slbie_data = (slbie_data << SID_SHIFT) | 0xc000000000000000u64 as usize;
    if ksp & slb_esid_mask(mmu_kernel_ssize) == slbie_data { return; }
    slbie_data |= mmu_kernel_ssize << SLBIE_SSIZE_SHIFT;
    core::arch::asm!("slbie {0}", in(reg) slbie_data);
}

unsafe fn slb_cache_slbie_user(index: u32) {
    let mut slbie_data = (*get_paca()).slb_cache[index as usize]; slbie_data <<= SID_SHIFT;
    slbie_data |= user_segment_size(slbie_data) << SLBIE_SSIZE_SHIFT; slbie_data |= SLBIE_C;
    core::arch::asm!("slbie {0}", in(reg) slbie_data);
}

pub unsafe fn switch_slb(tsk: *mut task_struct, mm: *mut mm_struct) {
    let ti = task_thread_info(tsk); hard_irq_disable(); isync();
    if stress_slb() { __slb_flush_and_restore_bolted(false); isync(); (*get_paca()).slb_cache_ptr = 0; (*get_paca()).slb_kern_bitmap = (1u32 << SLB_NUM_BOLTED) - 1; }
    else if cpu_has_feature(CPU_FTR_ARCH_300) { core::arch::asm!("slbia"); }
    else { let offset = (*get_paca()).slb_cache_ptr; if !mmu_has_feature(MMU_FTR_NO_SLBIE_B) && offset <= SLB_CACHE_ENTRIES { for i in 0..offset { slb_cache_slbie_user(i); } if !cpu_has_feature(CPU_FTR_ARCH_207S) && offset == 1 { slb_cache_slbie_user(0); } } else { __slb_flush_and_restore_bolted(true); isync(); (*get_paca()).slb_kern_bitmap = (1u32 << SLB_NUM_BOLTED) - 1; } (*get_paca()).slb_cache_ptr = 0; }
    (*get_paca()).slb_used_bitmap = (*get_paca()).slb_kern_bitmap; copy_mm_to_paca(mm);
    if slb_preload_disabled() { return; }
    (*tsk).thread.load_slb = (*tsk).thread.load_slb.wrapping_add(1);
    if (*tsk).thread.load_slb == 0 { preload_age(ti); preload_add(ti, KSTK_EIP(tsk)); }
    for i in 0..(*ti).slb_preload_nr { let idx = ((*ti).slb_preload_tail + i) % SLB_PRELOAD_NR; slb_allocate_user(mm, (*ti).slb_preload_esid[idx] << SID_SHIFT); }
    isync();
}

pub unsafe fn slb_set_size(size: u16) { mmu_slb_size = size; }

pub unsafe fn slb_initialize() {
    let linear_llp = mmu_psize_defs[mmu_linear_psize].sllp; let io_llp = mmu_psize_defs[mmu_io_psize].sllp; let vmalloc_llp = mmu_psize_defs[mmu_vmalloc_psize].sllp;
    (*get_paca()).vmalloc_sllp = SLB_VSID_KERNEL | vmalloc_llp; static mut SLB_ENCODING_INITED: i32 = 0;
    if SLB_ENCODING_INITED == 0 { SLB_ENCODING_INITED = 1; pr_devel!("SLB: linear  LLP = %04lx\n", linear_llp); pr_devel!("SLB: io      LLP = %04lx\n", io_llp); }
    (*get_paca()).stab_rr = SLB_NUM_BOLTED - 1; (*get_paca()).slb_kern_bitmap = (1u32 << SLB_NUM_BOLTED) - 1; (*get_paca()).slb_used_bitmap = (*get_paca()).slb_kern_bitmap;
    let lflags = SLB_VSID_KERNEL | linear_llp; core::arch::asm!("isync; slbmte {0}, {0}; isync; slbia; isync", in(reg) 0usize); create_shadowed_slbe(PAGE_OFFSET, mmu_kernel_ssize, lflags, LINEAR_INDEX); slb_shadow_clear(KSTACK_INDEX);
    if raw_smp_processor_id() != boot_cpuid && ((*get_paca()).kstack & slb_esid_mask(mmu_kernel_ssize)) > PAGE_OFFSET { create_shadowed_slbe((*get_paca()).kstack, mmu_kernel_ssize, lflags, KSTACK_INDEX); } core::arch::asm!("isync");
}

unsafe fn slb_cache_update(esid_data: usize) { if cpu_has_feature(CPU_FTR_ARCH_300) || stress_slb() { return; } let i = (*local_paca).slb_cache_ptr; if i < SLB_CACHE_ENTRIES { (*local_paca).slb_cache[i] = esid_data >> SID_SHIFT; (*local_paca).slb_cache_ptr += 1; } else { (*local_paca).slb_cache_ptr = SLB_CACHE_ENTRIES + 1; } }

unsafe fn alloc_slb_index(kernel: bool) -> enum_slb_index {
    let index;
    if (*local_paca).slb_used_bitmap != U32_MAX { index = ffz((*local_paca).slb_used_bitmap); (*local_paca).slb_used_bitmap |= 1u32 << index; if kernel { (*local_paca).slb_kern_bitmap |= 1u32 << index; } }
    else { index = (*local_paca).stab_rr; if index < mmu_slb_size - 1 { (*local_paca).stab_rr += 1; } else { (*local_paca).stab_rr = SLB_NUM_BOLTED; } if index < 32 { if kernel { (*local_paca).slb_kern_bitmap |= 1u32 << index; } else { (*local_paca).slb_kern_bitmap &= !(1u32 << index); } } }
    BUG_ON(index < SLB_NUM_BOLTED); index
}

unsafe fn slb_insert_entry(ea: usize, context: usize, flags: usize, ssize: i32, kernel: bool) -> i64 {
    let vsid = get_vsid(context, ea, ssize); if vsid == 0 { return -EFAULT; } barrier(); let index = alloc_slb_index(kernel); let vsid_data = __mk_vsid_data(vsid, ssize, flags); let esid_data = mk_esid_data(ea, ssize, index); assert_slb_presence(false, ea);
    if stress_slb() { let mut i = (*local_paca).slb_cache_ptr; BUILD_BUG_ON(SLB_CACHE_ENTRIES < 3); if !kernel || i == 3 { for j in 0..i { slb_cache_slbie_kernel(j); } i = 0; } if kernel { (*local_paca).slb_cache[i] = esid_data >> SID_SHIFT; i += 1; } (*local_paca).slb_cache_ptr = i; }
    core::arch::asm!("slbmte {0}, {1}", in(reg) vsid_data, in(reg) esid_data); barrier(); if !kernel { slb_cache_update(esid_data); } 0
}

unsafe fn slb_allocate_kernel(ea: usize, id: usize) -> i64 {
    let flags; if id == LINEAR_MAP_REGION_ID { if ea & EA_MASK > 1usize << H_MAX_PHYSMEM_BITS { return -EFAULT; } flags = SLB_VSID_KERNEL | mmu_psize_defs[mmu_linear_psize].sllp; } else if id == VMEMMAP_REGION_ID { if ea >= H_VMEMMAP_END { return -EFAULT; } flags = SLB_VSID_KERNEL | mmu_psize_defs[mmu_vmemmap_psize].sllp; } else if id == VMALLOC_REGION_ID { if ea >= H_VMALLOC_END { return -EFAULT; } flags = (*local_paca).vmalloc_sllp; } else if id == IO_REGION_ID { if ea >= H_KERN_IO_END { return -EFAULT; } flags = SLB_VSID_KERNEL | mmu_psize_defs[mmu_io_psize].sllp; } else { return -EFAULT; }
    let ssize = if mmu_has_feature(MMU_FTR_1T_SEGMENT) { MMU_SEGSIZE_1T } else { MMU_SEGSIZE_256M }; slb_insert_entry(ea, get_kernel_context(ea), flags, ssize, true)
}

unsafe fn slb_allocate_user(mm: *mut mm_struct, ea: usize) -> i64 { if ea >= mm_ctx_slb_addr_limit(&(*mm).context) { return -EFAULT; } let context = get_user_context(&(*mm).context, ea); if context == 0 { return -EFAULT; } if unlikely(ea >= H_PGTABLE_RANGE) { WARN_ON(1); return -EFAULT; } let ssize = user_segment_size(ea); let bpsize = get_slice_psize(mm, ea); slb_insert_entry(ea, context, SLB_VSID_USER | mmu_psize_defs[bpsize].sllp, ssize, false) }

pub unsafe fn do_slb_fault(regs: *mut pt_regs) -> i64 { let ea = (*regs).dar; let id = get_region_id(ea); VM_WARN_ON(mfmsr() & MSR_EE); if regs_is_unrecoverable(regs) { return -EINVAL; } if id >= LINEAR_MAP_REGION_ID { #[cfg(CONFIG_DEBUG_VM)] { BUG_ON((*local_paca).in_kernel_slb_handler); (*local_paca).in_kernel_slb_handler = 1; } let err = slb_allocate_kernel(ea, id); #[cfg(CONFIG_DEBUG_VM)] { (*local_paca).in_kernel_slb_handler = 0; } err } else { let mm = (*current).mm; if mm.is_null() { return -EFAULT; } let err = slb_allocate_user(mm, ea); if err == 0 { preload_add(current_thread_info(), ea); } err } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
