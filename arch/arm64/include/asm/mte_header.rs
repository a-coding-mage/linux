/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2020 ARM Ltd. */

/* Dependencies supplied by the surrounding kernel translation. */

#[allow(non_camel_case_types)]
pub type ulong = ::core::ffi::c_ulong;

extern "C" {
    pub fn mte_clear_page_tags(addr: *mut ::core::ffi::c_void);
    pub fn mte_copy_tags_from_user(
        to: *mut ::core::ffi::c_void,
        from: *const ::core::ffi::c_void,
        n: ulong,
    ) -> ulong;
    pub fn mte_copy_tags_to_user(
        to: *mut ::core::ffi::c_void,
        from: *mut ::core::ffi::c_void,
        n: ulong,
    ) -> ulong;
    pub fn mte_save_tags(page: *mut page) -> ::core::ffi::c_int;
    pub fn mte_save_page_tags(page_addr: *const ::core::ffi::c_void, tag_storage: *mut ::core::ffi::c_void);
    pub fn mte_restore_tags(entry: swp_entry_t, page: *mut page);
    pub fn mte_restore_page_tags(page_addr: *mut ::core::ffi::c_void, tag_storage: *const ::core::ffi::c_void);
    pub fn mte_invalidate_tags(type_: ::core::ffi::c_int, offset: pgoff_t);
    pub fn mte_invalidate_tags_area(type_: ::core::ffi::c_int);
    pub fn mte_allocate_tag_storage() -> *mut ::core::ffi::c_void;
    pub fn mte_free_tag_storage(storage: *mut ::core::ffi::c_char);
}

/* CONFIG_ARM64_MTE */
pub const PG_mte_tagged: ::core::ffi::c_ulong = PG_arch_2;
pub const PG_mte_lock: ::core::ffi::c_ulong = PG_arch_3;

#[inline]
pub unsafe fn set_page_mte_tagged(page: *mut page) {
    VM_WARN_ON_ONCE(folio_test_hugetlb(page_folio(page)));
    smp_wmb();
    set_bit(PG_mte_tagged, &mut (*page).flags.f);
}

#[inline]
pub unsafe fn page_mte_tagged(page: *mut page) -> bool {
    let ret = test_bit(PG_mte_tagged, &(*page).flags.f);
    VM_WARN_ON_ONCE(folio_test_hugetlb(page_folio(page)));
    if ret { smp_rmb(); }
    ret
}

#[inline]
pub unsafe fn try_page_mte_tagging(page: *mut page) -> bool {
    VM_WARN_ON_ONCE(folio_test_hugetlb(page_folio(page)));
    if !test_and_set_bit(PG_mte_lock, &mut (*page).flags.f) { return true; }
    smp_cond_load_acquire(&(*page).flags.f, VAL & (1UL << PG_mte_tagged));
    false
}

extern "C" {
    pub fn mte_zero_clear_page_tags(addr: *mut ::core::ffi::c_void);
    pub fn mte_sync_tags(pte: pte_t, nr_pages: ::core::ffi::c_uint);
    pub fn mte_copy_page_tags(kto: *mut ::core::ffi::c_void, kfrom: *const ::core::ffi::c_void);
    pub fn mte_thread_init_user();
    pub fn mte_thread_switch(next: *mut task_struct);
    pub fn mte_cpu_setup();
    pub fn mte_suspend_enter();
    pub fn mte_suspend_exit();
    pub fn set_mte_ctrl(task: *mut task_struct, arg: ulong) -> ::core::ffi::c_long;
    pub fn get_mte_ctrl(task: *mut task_struct) -> ::core::ffi::c_long;
    pub fn mte_ptrace_copy_tags(child: *mut task_struct, request: ::core::ffi::c_long, addr: ulong, data: ulong) -> ::core::ffi::c_int;
    pub fn mte_probe_user_range(uaddr: *const ::core::ffi::c_char, size: usize) -> usize;
}

/* !CONFIG_ARM64_MTE: declarations below are no-op compatibility helpers. */
#[inline] pub unsafe fn set_page_mte_tagged_disabled(_page: *mut page) {}
#[inline] pub unsafe fn page_mte_tagged_disabled(_page: *mut page) -> bool { false }
#[inline] pub unsafe fn try_page_mte_tagging_disabled(_page: *mut page) -> bool { false }
#[inline] pub unsafe fn mte_zero_clear_page_tags_disabled(_addr: *mut ::core::ffi::c_void) {}
#[inline] pub unsafe fn mte_sync_tags_disabled(_pte: pte_t, _nr_pages: ::core::ffi::c_uint) {}
#[inline] pub unsafe fn mte_copy_page_tags_disabled(_kto: *mut ::core::ffi::c_void, _kfrom: *const ::core::ffi::c_void) {}
#[inline] pub unsafe fn mte_thread_init_user_disabled() {}
#[inline] pub unsafe fn mte_thread_switch_disabled(_next: *mut task_struct) {}
#[inline] pub unsafe fn mte_suspend_enter_disabled() {}
#[inline] pub unsafe fn mte_suspend_exit_disabled() {}
#[inline] pub unsafe fn set_mte_ctrl_disabled(_task: *mut task_struct, _arg: ulong) -> ::core::ffi::c_long { 0 }
#[inline] pub unsafe fn get_mte_ctrl_disabled(_task: *mut task_struct) -> ::core::ffi::c_long { 0 }
#[inline] pub unsafe fn mte_ptrace_copy_tags_disabled(_child: *mut task_struct, _request: ::core::ffi::c_long, _addr: ulong, _data: ulong) -> ::core::ffi::c_int { -EIO }

/* CONFIG_HUGETLB_PAGE && CONFIG_ARM64_MTE */
#[inline]
pub unsafe fn folio_set_hugetlb_mte_tagged(folio: *mut folio) {
    VM_WARN_ON_ONCE(!folio_test_hugetlb(folio));
    smp_wmb();
    set_bit(PG_mte_tagged, &mut (*folio).flags.f);
}

#[inline]
pub unsafe fn folio_test_hugetlb_mte_tagged(folio: *mut folio) -> bool {
    let ret = test_bit(PG_mte_tagged, &(*folio).flags.f);
    VM_WARN_ON_ONCE(!folio_test_hugetlb(folio));
    if ret { smp_rmb(); }
    ret
}

#[inline]
pub unsafe fn folio_try_hugetlb_mte_tagging(folio: *mut folio) -> bool {
    VM_WARN_ON_ONCE(!folio_test_hugetlb(folio));
    if !test_and_set_bit(PG_mte_lock, &mut (*folio).flags.f) { return true; }
    smp_cond_load_acquire(&(*folio).flags.f, VAL & (1UL << PG_mte_tagged));
    false
}

#[inline]
pub unsafe fn mte_disable_tco_entry(task: *mut task_struct) {
    if !system_supports_mte() { return; }
    if kasan_hw_tags_enabled() || ((*task).thread.sctlr_user & (1UL << SCTLR_EL1_TCF0_SHIFT)) != 0 {
        asm!(SET_PSTATE_TCO(0));
    }
}

/* CONFIG_KASAN_HW_TAGS */
extern "C" { pub fn mte_check_tfsr_el1(); }

#[inline]
pub unsafe fn mte_check_tfsr_entry() {
    if !kasan_hw_tags_enabled() || !system_uses_mte_async_or_asymm_mode() { return; }
    mte_check_tfsr_el1();
}

#[inline]
pub unsafe fn mte_check_tfsr_exit() {
    if !kasan_hw_tags_enabled() || !system_uses_mte_async_or_asymm_mode() { return; }
    dsb(nsh);
    isb();
    mte_check_tfsr_el1();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
