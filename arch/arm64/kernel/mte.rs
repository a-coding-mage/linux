// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 ARM Ltd.
 */

// Linux and ARM headers from the C translation unit provide the external
// types, constants, macros, and functions referenced below.

static mut MTE_TCF_PREFERRED: u64 = 0;

#[cfg(feature = "CONFIG_KASAN_HW_TAGS")]
static mut MTE_ASYNC_OR_ASYMM_MODE: bool = false;

pub unsafe fn mte_sync_tags(mut pte: pte_t, nr_pages: u32) {
    let mut page = pte_page(pte);
    let folio = page_folio(page);
    let mut i: usize = 0;

    if folio_test_hugetlb(folio) {
        let nr = folio_nr_pages(folio);
        if folio_try_hugetlb_mte_tagging(folio) {
            while i < nr {
                mte_clear_page_tags(page_address(page));
                i += 1;
                page = page.add(1);
            }
            folio_set_hugetlb_mte_tagged(folio);
        }
        smp_wmb();
        return;
    }

    while i < nr_pages as usize {
        if try_page_mte_tagging(page) {
            mte_clear_page_tags(page_address(page));
            set_page_mte_tagged(page);
        }
        i += 1;
        page = page.add(1);
    }
    smp_wmb();
}

pub unsafe fn memcmp_pages(page1: *mut page, page2: *mut page) -> i32 {
    let addr1 = page_address(page1);
    let addr2 = page_address(page2);
    let ret = memcmp(addr1, addr2, PAGE_SIZE);
    if !system_supports_mte() || ret != 0 { return ret; }
    if page_mte_tagged(page1) || page_mte_tagged(page2) {
        return (addr1 != addr2) as i32;
    }
    ret
}

unsafe fn __mte_enable_kernel(mode: *const i8, tcf: usize) {
    sysreg_clear_set(sctlr_el1, SCTLR_EL1_TCF_MASK, SYS_FIELD_PREP(SCTLR_EL1, TCF, tcf));
    isb();
    pr_info_once("MTE: enabled in %s mode at EL1\n", mode);
}

#[cfg(feature = "CONFIG_KASAN_HW_TAGS")]
pub unsafe fn mte_enable_kernel_sync() {
    WARN_ONCE(system_uses_mte_async_or_asymm_mode(), "MTE async mode enabled system wide!");
    __mte_enable_kernel(b"synchronous\0".as_ptr() as *const i8, SCTLR_EL1_TCF_SYNC);
}

#[cfg(feature = "CONFIG_KASAN_HW_TAGS")]
pub unsafe fn mte_enable_kernel_async() {
    __mte_enable_kernel(b"asynchronous\0".as_ptr() as *const i8, SCTLR_EL1_TCF_ASYNC);
    if !system_uses_mte_async_or_asymm_mode() { static_branch_enable(&raw mut MTE_ASYNC_OR_ASYMM_MODE); }
}

#[cfg(feature = "CONFIG_KASAN_HW_TAGS")]
pub unsafe fn mte_enable_kernel_asymm() {
    if cpus_have_cap(ARM64_MTE_ASYMM) {
        __mte_enable_kernel(b"asymmetric\0".as_ptr() as *const i8, SCTLR_EL1_TCF_ASYMM);
        if !system_uses_mte_async_or_asymm_mode() { static_branch_enable(&raw mut MTE_ASYNC_OR_ASYMM_MODE); }
    } else { mte_enable_kernel_sync(); }
}

#[cfg(feature = "CONFIG_KASAN_HW_TAGS")]
pub unsafe fn mte_enable_kernel_store_only() -> i32 {
    if !cpus_have_cap(ARM64_MTE_STORE_ONLY) { return -EINVAL; }
    sysreg_clear_set(sctlr_el1, SCTLR_EL1_TCSO_MASK, SYS_FIELD_PREP(SCTLR_EL1, TCSO, 1));
    isb();
    pr_info_once("MTE: enabled store only mode at EL1\n");
    0
}

#[cfg(feature = "CONFIG_KASAN_HW_TAGS")]
pub unsafe fn mte_check_tfsr_el1() {
    let tfsr_el1 = read_sysreg_s(SYS_TFSR_EL1);
    if unlikely(tfsr_el1 & SYS_TFSR_EL1_TF1 != 0) {
        write_sysreg_s(0, SYS_TFSR_EL1);
        kasan_report_async();
    }
}

unsafe fn mte_update_sctlr_user(task: *mut task_struct) {
    let mut sctlr = (*task).thread.sctlr_user;
    let mte_ctrl = (*task).thread.mte_ctrl;
    let pref = __this_cpu_read(MTE_TCF_PREFERRED);
    let resolved = if mte_ctrl & pref != 0 { pref } else { mte_ctrl };
    sctlr &= !(SCTLR_EL1_TCF0_MASK | SCTLR_EL1_TCSO0_MASK);
    if resolved & MTE_CTRL_TCF_ASYMM != 0 { sctlr |= SYS_FIELD_PREP_ENUM(SCTLR_EL1, TCF0, ASYMM); }
    else if resolved & MTE_CTRL_TCF_ASYNC != 0 { sctlr |= SYS_FIELD_PREP_ENUM(SCTLR_EL1, TCF0, ASYNC); }
    else if resolved & MTE_CTRL_TCF_SYNC != 0 { sctlr |= SYS_FIELD_PREP_ENUM(SCTLR_EL1, TCF0, SYNC); }
    if mte_ctrl & MTE_CTRL_STORE_ONLY != 0 { sctlr |= SYS_FIELD_PREP(SCTLR_EL1, TCSO0, 1); }
    (*task).thread.sctlr_user = sctlr;
}

unsafe fn mte_update_gcr_excl(task: *mut task_struct) {
    if kasan_hw_tags_enabled() { return; }
    write_sysreg_s((((*task).thread.mte_ctrl >> MTE_CTRL_GCR_USER_EXCL_SHIFT) & SYS_GCR_EL1_EXCL_MASK) | SYS_GCR_EL1_RRND, SYS_GCR_EL1);
}

#[cfg(feature = "CONFIG_KASAN_HW_TAGS")]
pub unsafe fn kasan_hw_tags_enable(_alt: *mut alt_instr, _origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32) {
    BUG_ON(nr_inst != 1);
    if kasan_hw_tags_enabled() { *updptr = cpu_to_le32(aarch64_insn_gen_nop()); }
}

pub unsafe fn mte_thread_init_user() {
    if !system_supports_mte() { return; }
    dsb(ish); write_sysreg_s(0, SYS_TFSRE0_EL1); clear_thread_flag(TIF_MTE_ASYNC_FAULT); set_mte_ctrl(current, 0);
}

pub unsafe fn mte_thread_switch(next: *mut task_struct) {
    if !system_supports_mte() { return; }
    mte_update_sctlr_user(next); mte_update_gcr_excl(next); mte_disable_tco_entry(next);
    if !system_uses_mte_async_or_asymm_mode() { return; }
    isb(); mte_check_tfsr_el1();
}

pub unsafe fn mte_cpu_setup() {
    BUG_ON(read_sysreg(ttbr0_el1) & TTBRx_EL1_CnP != 0); BUG_ON(read_sysreg(ttbr1_el1) & TTBRx_EL1_CnP != 0);
    sysreg_clear_set(mair_el1, MAIR_ATTRIDX(MAIR_ATTR_MASK, MT_NORMAL_TAGGED), MAIR_ATTRIDX(MAIR_ATTR_NORMAL_TAGGED, MT_NORMAL_TAGGED));
    write_sysreg_s(KERNEL_GCR_EL1, SYS_GCR_EL1);
    let mut rgsr = (read_sysreg(CNTVCT_EL0) & SYS_RGSR_EL1_SEED_MASK) << SYS_RGSR_EL1_SEED_SHIFT;
    if rgsr == 0 { rgsr = 1 << SYS_RGSR_EL1_SEED_SHIFT; }
    write_sysreg_s(rgsr, SYS_RGSR_EL1); write_sysreg_s(0, SYS_TFSR_EL1); write_sysreg_s(0, SYS_TFSRE0_EL1); local_flush_tlb_all();
}

pub unsafe fn mte_suspend_enter() { if system_supports_mte() && system_uses_mte_async_or_asymm_mode() { dsb(nsh); isb(); mte_check_tfsr_el1(); } }
pub unsafe fn mte_suspend_exit() { if system_supports_mte() { mte_cpu_setup(); } }

pub unsafe fn set_mte_ctrl(task: *mut task_struct, arg: usize) -> i64 {
    let mut mte_ctrl = (!(arg & PR_MTE_TAG_MASK) >> PR_MTE_TAG_SHIFT & SYS_GCR_EL1_EXCL_MASK) << MTE_CTRL_GCR_USER_EXCL_SHIFT;
    if !system_supports_mte() { return 0; }
    if arg & PR_MTE_TCF_ASYNC != 0 { mte_ctrl |= MTE_CTRL_TCF_ASYNC; }
    if arg & PR_MTE_TCF_SYNC != 0 { mte_ctrl |= MTE_CTRL_TCF_SYNC; }
    if cpus_have_cap(ARM64_MTE_ASYMM) && arg & PR_MTE_TCF_ASYNC != 0 && arg & PR_MTE_TCF_SYNC != 0 { mte_ctrl |= MTE_CTRL_TCF_ASYMM; }
    if arg & PR_MTE_STORE_ONLY != 0 { mte_ctrl |= MTE_CTRL_STORE_ONLY; }
    (*task).thread.mte_ctrl = mte_ctrl;
    if task == current { preempt_disable(); mte_update_sctlr_user(task); mte_update_gcr_excl(task); update_sctlr_el1((*task).thread.sctlr_user); preempt_enable(); }
    0
}

pub unsafe fn get_mte_ctrl(task: *mut task_struct) -> i64 {
    let mte_ctrl = (*task).thread.mte_ctrl;
    if !system_supports_mte() { return 0; }
    let mut ret = ((!mte_ctrl >> MTE_CTRL_GCR_USER_EXCL_SHIFT) & SYS_GCR_EL1_EXCL_MASK) << PR_MTE_TAG_SHIFT;
    if mte_ctrl & MTE_CTRL_TCF_ASYNC != 0 { ret |= PR_MTE_TCF_ASYNC; }
    if mte_ctrl & MTE_CTRL_TCF_SYNC != 0 { ret |= PR_MTE_TCF_SYNC; }
    if mte_ctrl & MTE_CTRL_STORE_ONLY != 0 { ret |= PR_MTE_STORE_ONLY; }
    ret as i64
}

unsafe fn __access_remote_tags(mm: *mut mm_struct, mut addr: usize, kiov: *mut iovec, gup_flags: u32) -> i32 {
    let mut buf = (*kiov).iov_base; let mut len = (*kiov).iov_len; let mut err = 0;
    let write = gup_flags & FOLL_WRITE != 0;
    if !access_ok(buf, len) { return -EFAULT; }
    if mmap_read_lock_killable(mm) != 0 { return -EIO; }
    while len != 0 {
        let mut vma = core::ptr::null_mut(); let page = get_user_page_vma_remote(mm, addr, gup_flags, &mut vma);
        if IS_ERR(page) { err = PTR_ERR(page); break; }
        if (*vma).vm_flags & VM_MTE == 0 { err = -EOPNOTSUPP; put_page(page); break; }
        let folio = page_folio(page);
        if folio_test_hugetlb(folio) { WARN_ON_ONCE(!folio_test_hugetlb_mte_tagged(folio) && !is_huge_zero_folio(folio)); }
        else { WARN_ON_ONCE(!page_mte_tagged(page) && !is_zero_page(page)); }
        let offset = offset_in_page(addr); let tags = core::cmp::min(len, (PAGE_SIZE - offset) / MTE_GRANULE_SIZE);
        let maddr = page_address(page);
        let copied = if write { let n = mte_copy_tags_from_user(maddr.add(offset), buf, tags); set_page_dirty_lock(page); n } else { mte_copy_tags_to_user(buf, maddr.add(offset), tags) };
        put_page(page); if copied == 0 { break; }
        len -= copied; buf = buf.add(copied); addr += copied * MTE_GRANULE_SIZE;
    }
    mmap_read_unlock(mm); (*kiov).iov_len = buf.offset_from((*kiov).iov_base) as usize;
    if (*kiov).iov_len == 0 { return if err != 0 { -EIO } else { -EFAULT }; } 0
}

unsafe fn access_remote_tags(tsk: *mut task_struct, addr: usize, kiov: *mut iovec, flags: u32) -> i32 {
    let mm = get_task_mm(tsk); if mm.is_null() { return -EPERM; }
    if !ptracer_access_allowed(tsk) { mmput(mm); return -EPERM; }
    let ret = __access_remote_tags(mm, addr, kiov, flags); mmput(mm); ret
}

pub unsafe fn mte_ptrace_copy_tags(child: *mut task_struct, request: i64, mut addr: usize, data: usize) -> i32 {
    if !system_supports_mte() { return -EIO; }
    let uiov = data as *mut iovec; let mut kiov = core::mem::zeroed::<iovec>(); let mut flags = FOLL_FORCE;
    if get_user(&mut kiov.iov_base, &mut (*uiov).iov_base) != 0 || get_user(&mut kiov.iov_len, &mut (*uiov).iov_len) != 0 { return -EFAULT; }
    if request == PTRACE_POKEMTETAGS { flags |= FOLL_WRITE; }
    addr &= MTE_GRANULE_MASK; let mut ret = access_remote_tags(child, addr, &mut kiov, flags);
    if ret == 0 { ret = put_user(kiov.iov_len, &mut (*uiov).iov_len); } ret
}

unsafe fn mte_tcf_preferred_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    match per_cpu(MTE_TCF_PREFERRED, (*dev).id) { MTE_CTRL_TCF_ASYNC => sysfs_emit(buf, "async\n"), MTE_CTRL_TCF_SYNC => sysfs_emit(buf, "sync\n"), MTE_CTRL_TCF_ASYMM => sysfs_emit(buf, "asymm\n"), _ => sysfs_emit(buf, "???\n") }
}

unsafe fn mte_tcf_preferred_store(dev: *mut device, _attr: *mut device_attribute, buf: *const i8, count: usize) -> isize {
    let tcf = if sysfs_streq(buf, "async") { MTE_CTRL_TCF_ASYNC } else if sysfs_streq(buf, "sync") { MTE_CTRL_TCF_SYNC } else if cpus_have_cap(ARM64_MTE_ASYMM) && sysfs_streq(buf, "asymm") { MTE_CTRL_TCF_ASYMM } else { return -EINVAL as isize };
    device_lock(dev); per_cpu(MTE_TCF_PREFERRED, (*dev).id) = tcf; device_unlock(dev); count as isize
}

unsafe fn register_mte_tcf_preferred_sysctl() -> i32 {
    if !system_supports_mte() { return 0; }
    for_each_possible_cpu(|cpu| { per_cpu(MTE_TCF_PREFERRED, cpu) = MTE_CTRL_TCF_ASYNC; device_create_file(get_cpu_device(cpu), &dev_attr_mte_tcf_preferred); }); 0
}

pub unsafe fn mte_probe_user_range(mut uaddr: *const i8, size: usize) -> usize {
    let end = uaddr.add(size); let mut val: i8 = 0;
    __raw_get_user(&mut val, uaddr, efault);
    uaddr = PTR_ALIGN(uaddr, MTE_GRANULE_SIZE);
    while uaddr < end { __raw_get_user(&mut val, uaddr, efault); uaddr = uaddr.add(MTE_GRANULE_SIZE); }
    let _ = val; return 0;
efault: end.offset_from(uaddr) as usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
