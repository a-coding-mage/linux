// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */
/* Derived from includes: <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_misc.h",
 * "bpf_experimental.h".
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type u32 = ::core::ffi::c_uint;
type u64 = ::core::ffi::c_ulonglong;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_res_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_iter_num {
    _private: [u8; ::core::mem::size_of::<::core::ffi::c_ulong>()],
}

#[repr(C)]
struct irq_ooo_refs_array__anon {
    i: ::core::ffi::c_int,
}

static mut global_flags: ::core::ffi::c_ulong = 0;

extern "C" {
    fn bpf_local_irq_save(flags: *mut ::core::ffi::c_ulong);
    fn bpf_local_irq_restore(flags: *mut ::core::ffi::c_ulong);
    fn bpf_copy_from_user_str(
        dst: *mut ::core::ffi::c_void,
        dst__sz: u32,
        unsafe_ptr__ign: *const ::core::ffi::c_void,
        flags: u64,
    ) -> ::core::ffi::c_int;
    fn bpf_copy_from_user(
        dst: *mut ::core::ffi::c_void,
        dst__sz: u32,
        unsafe_ptr__ign: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn bpf_iter_num_new(it: *mut bpf_iter_num, start: ::core::ffi::c_int, end: ::core::ffi::c_int);
    fn bpf_obj_new_irq_ooo_refs_array__anon() -> *mut irq_ooo_refs_array__anon;
    fn bpf_obj_drop(p: *mut irq_ooo_refs_array__anon);
    fn bpf_printk(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn bpf_res_spin_lock_irqsave(
        lock: *mut bpf_res_spin_lock,
        flags: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    fn bpf_res_spin_unlock_irqrestore(
        lock: *mut bpf_res_spin_lock,
        flags: *mut ::core::ffi::c_ulong,
    );
}

/* __hidden SEC(".data.A") */
static mut lockA: bpf_res_spin_lock = bpf_res_spin_lock { _private: [] };
/* __hidden SEC(".data.B") */
static mut lockB: bpf_res_spin_lock = bpf_res_spin_lock { _private: [] };

/* SEC("?tc") __failure __msg("R1 doesn't point to an irq flag on stack") */
pub unsafe extern "C" fn irq_save_bad_arg(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    bpf_local_irq_save(&mut global_flags);
    0
}

/* SEC("?tc") __failure __msg("R1 doesn't point to an irq flag on stack") */
pub unsafe extern "C" fn irq_restore_bad_arg(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    bpf_local_irq_restore(&mut global_flags);
    0
}

/* SEC("?tc") __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region") */
pub unsafe extern "C" fn irq_restore_missing_2(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags1);
    bpf_local_irq_save(&mut flags2);
    0
}

/* SEC("?tc") __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region") */
pub unsafe extern "C" fn irq_restore_missing_3(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags3: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags1);
    bpf_local_irq_save(&mut flags2);
    bpf_local_irq_save(&mut flags3);
    0
}

/* SEC("?tc") __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region") */
pub unsafe extern "C" fn irq_restore_missing_3_minus_2(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags3: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags1);
    bpf_local_irq_save(&mut flags2);
    bpf_local_irq_save(&mut flags3);
    bpf_local_irq_restore(&mut flags3);
    bpf_local_irq_restore(&mut flags2);
    0
}

unsafe fn local_irq_save(flags: *mut ::core::ffi::c_ulong) {
    bpf_local_irq_save(flags);
}

unsafe fn local_irq_restore(flags: *mut ::core::ffi::c_ulong) {
    bpf_local_irq_restore(flags);
}

/* SEC("?tc") __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region") */
pub unsafe extern "C" fn irq_restore_missing_1_subprog(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save(&mut flags);
    0
}

/* SEC("?tc") __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region") */
pub unsafe extern "C" fn irq_restore_missing_2_subprog(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save(&mut flags1);
    local_irq_save(&mut flags2);
    0
}

/* SEC("?tc") __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region") */
pub unsafe extern "C" fn irq_restore_missing_3_subprog(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags3: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save(&mut flags1);
    local_irq_save(&mut flags2);
    local_irq_save(&mut flags3);
    0
}

/* SEC("?tc") __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region") */
pub unsafe extern "C" fn irq_restore_missing_3_minus_2_subprog(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags3: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save(&mut flags1);
    local_irq_save(&mut flags2);
    local_irq_save(&mut flags3);
    local_irq_restore(&mut flags3);
    local_irq_restore(&mut flags2);
    0
}

/* SEC("?tc") __success */
pub unsafe extern "C" fn irq_balance(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save(&mut flags);
    local_irq_restore(&mut flags);
    0
}

/* SEC("?tc") __success */
pub unsafe extern "C" fn irq_balance_n(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags3: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save(&mut flags1);
    local_irq_save(&mut flags2);
    local_irq_save(&mut flags3);
    local_irq_restore(&mut flags3);
    local_irq_restore(&mut flags2);
    local_irq_restore(&mut flags1);
    0
}

unsafe fn local_irq_balance() {
    let mut flags: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save(&mut flags);
    local_irq_restore(&mut flags);
}

unsafe fn local_irq_balance_n() {
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags3: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save(&mut flags1);
    local_irq_save(&mut flags2);
    local_irq_save(&mut flags3);
    local_irq_restore(&mut flags3);
    local_irq_restore(&mut flags2);
    local_irq_restore(&mut flags1);
}

/* SEC("?tc") __success */
pub unsafe extern "C" fn irq_balance_subprog(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    local_irq_balance();
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") __failure __msg("sleepable helper bpf_copy_from_user#") */
pub unsafe extern "C" fn irq_sleepable_helper(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut data: u32 = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save(&mut flags);
    bpf_copy_from_user(
        &mut data as *mut _ as *mut ::core::ffi::c_void,
        ::core::mem::size_of_val(&data) as u32,
        ::core::ptr::null(),
    );
    local_irq_restore(&mut flags);
    0
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") __failure __msg("kernel func bpf_copy_from_user_str is sleepable within IRQ-disabled region") */
pub unsafe extern "C" fn irq_sleepable_kfunc(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut data: u32 = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save(&mut flags);
    bpf_copy_from_user_str(
        &mut data as *mut _ as *mut ::core::ffi::c_void,
        ::core::mem::size_of_val(&data) as u32,
        ::core::ptr::null(),
        0,
    );
    local_irq_restore(&mut flags);
    0
}

pub unsafe extern "C" fn global_local_irq_balance() -> ::core::ffi::c_int {
    local_irq_balance_n();
    0
}

/* SEC("?tc") __success */
pub unsafe extern "C" fn irq_global_subprog(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags);
    global_local_irq_balance();
    bpf_local_irq_restore(&mut flags);
    0
}

/* SEC("?tc") __failure __msg("cannot restore irq state out of order") */
pub unsafe extern "C" fn irq_restore_ooo(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags1);
    bpf_local_irq_save(&mut flags2);
    bpf_local_irq_restore(&mut flags1);
    bpf_local_irq_restore(&mut flags2);
    0
}

/* SEC("?tc") __failure __msg("cannot restore irq state out of order") */
pub unsafe extern "C" fn irq_restore_ooo_3(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags3: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags1);
    bpf_local_irq_save(&mut flags2);
    bpf_local_irq_restore(&mut flags2);
    bpf_local_irq_save(&mut flags3);
    bpf_local_irq_restore(&mut flags1);
    bpf_local_irq_restore(&mut flags3);
    0
}

unsafe fn local_irq_save_3(
    flags1: *mut ::core::ffi::c_ulong,
    flags2: *mut ::core::ffi::c_ulong,
    flags3: *mut ::core::ffi::c_ulong,
) {
    local_irq_save(flags1);
    local_irq_save(flags2);
    local_irq_save(flags3);
}

/* SEC("?tc") __success */
pub unsafe extern "C" fn irq_restore_3_subprog(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags3: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save_3(&mut flags1, &mut flags2, &mut flags3);
    bpf_local_irq_restore(&mut flags3);
    bpf_local_irq_restore(&mut flags2);
    bpf_local_irq_restore(&mut flags1);
    0
}

/* SEC("?tc") __failure __msg("cannot restore irq state out of order") */
pub unsafe extern "C" fn irq_restore_4_subprog(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags3: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags4: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save_3(&mut flags1, &mut flags2, &mut flags3);
    bpf_local_irq_restore(&mut flags3);
    bpf_local_irq_save(&mut flags4);
    bpf_local_irq_restore(&mut flags4);
    bpf_local_irq_restore(&mut flags1);
    0
}

/* SEC("?tc") __failure __msg("cannot restore irq state out of order") */
pub unsafe extern "C" fn irq_restore_ooo_3_subprog(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags3: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    local_irq_save_3(&mut flags1, &mut flags2, &mut flags3);
    bpf_local_irq_restore(&mut flags3);
    bpf_local_irq_restore(&mut flags2);
    bpf_local_irq_save(&mut flags3);
    bpf_local_irq_restore(&mut flags1);
    0
}

/* SEC("?tc") __failure __msg("expected an initialized") */
pub unsafe extern "C" fn irq_restore_invalid(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags: ::core::ffi::c_ulong = 0xfaceb00c;

    bpf_local_irq_save(&mut flags1);
    bpf_local_irq_restore(&mut flags);
    0
}

/* SEC("?tc") __failure __msg("expected uninitialized") */
pub unsafe extern "C" fn irq_save_invalid(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags1);
    bpf_local_irq_save(&mut flags1);
    0
}

/* SEC("?tc") __failure __msg("expected an initialized") */
pub unsafe extern "C" fn irq_restore_iter(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut it: bpf_iter_num = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_iter_num_new(&mut it, 0, 42);
    bpf_local_irq_restore(&mut it as *mut _ as *mut ::core::ffi::c_ulong);
    0
}

/* SEC("?tc") __failure __msg("Unreleased reference id=1") */
pub unsafe extern "C" fn irq_save_iter(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut it: bpf_iter_num = ::core::mem::MaybeUninit::uninit().assume_init();

    /* Ensure same sized slot has st->ref_obj_id set, so we reject based on
     * slot_type != STACK_IRQ_FLAG...
     */
    const _: [(); ::core::mem::size_of::<bpf_iter_num>()] =
        [(); ::core::mem::size_of::<::core::ffi::c_ulong>()];

    bpf_iter_num_new(&mut it, 0, 42);
    bpf_local_irq_save(&mut it as *mut _ as *mut ::core::ffi::c_ulong);
    bpf_local_irq_restore(&mut it as *mut _ as *mut ::core::ffi::c_ulong);
    0
}

/* SEC("?tc") __failure __msg("expected an initialized") */
pub unsafe extern "C" fn irq_flag_overwrite(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags);
    flags = 0xdeadbeef;
    bpf_local_irq_restore(&mut flags);
    0
}

/* SEC("?tc") __failure __msg("expected an initialized") */
pub unsafe extern "C" fn irq_flag_overwrite_partial(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags);
    *((&mut flags as *mut _ as *mut ::core::ffi::c_char).add(1)) = 0xff_u8 as ::core::ffi::c_char;
    bpf_local_irq_restore(&mut flags);
    0
}

/* SEC("?tc") __failure __msg("cannot restore irq state out of order") */
pub unsafe extern "C" fn irq_ooo_refs_array(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags: [::core::ffi::c_ulong; 4] = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut p: *mut irq_ooo_refs_array__anon;

    /* refs=1 */
    bpf_local_irq_save(&mut flags[0]);

    /* refs=1,2 */
    p = bpf_obj_new_irq_ooo_refs_array__anon();
    if p.is_null() {
        bpf_local_irq_restore(&mut flags[0]);
        return 0;
    }

    /* refs=1,2,3 */
    bpf_local_irq_save(&mut flags[1]);

    /* refs=1,2,3,4 */
    bpf_local_irq_save(&mut flags[2]);

    /* Now when we remove ref=2, the verifier must not break the ordering in
     * the refs array between 1,3,4. With an older implementation, the
     * verifier would swap the last element with the removed element, but to
     * maintain the stack property we need to use memmove.
     */
    bpf_obj_drop(p);

    /* Save and restore to reset active_irq_id to 3, as the ordering is now
     * refs=1,4,3. When restoring the linear scan will find prev_id in order
     * as 3 instead of 4.
     */
    bpf_local_irq_save(&mut flags[3]);
    bpf_local_irq_restore(&mut flags[3]);

    /* With the incorrect implementation, we can release flags[1], flags[2],
     * and flags[0], i.e. in the wrong order.
     */
    bpf_local_irq_restore(&mut flags[1]);
    bpf_local_irq_restore(&mut flags[2]);
    bpf_local_irq_restore(&mut flags[0]);
    0
}

pub unsafe extern "C" fn global_subprog(mut i: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if i != 0 {
        bpf_printk(c"%p".as_ptr(), &mut i as *mut _);
    }
    i
}

pub unsafe extern "C" fn global_sleepable_helper_subprog(mut i: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if i != 0 {
        bpf_copy_from_user(
            &mut i as *mut _ as *mut ::core::ffi::c_void,
            ::core::mem::size_of_val(&i) as u32,
            ::core::ptr::null(),
        );
    }
    i
}

pub unsafe extern "C" fn global_sleepable_kfunc_subprog(mut i: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if i != 0 {
        bpf_copy_from_user_str(
            &mut i as *mut _ as *mut ::core::ffi::c_void,
            ::core::mem::size_of_val(&i) as u32,
            ::core::ptr::null(),
            0,
        );
    }
    global_subprog(i);
    i
}

pub unsafe extern "C" fn global_subprog_calling_sleepable_global(i: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if i == 0 {
        global_sleepable_kfunc_subprog(i);
    }
    i
}

/* SEC("?syscall") __success */
pub unsafe extern "C" fn irq_non_sleepable_global_subprog(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags);
    global_subprog(0);
    bpf_local_irq_restore(&mut flags);
    0
}

/* SEC("?syscall") __failure __msg("sleepable global function") */
pub unsafe extern "C" fn irq_sleepable_helper_global_subprog(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags);
    global_sleepable_helper_subprog(0);
    bpf_local_irq_restore(&mut flags);
    0
}

/* SEC("?syscall") __failure __msg("sleepable global function") */
pub unsafe extern "C" fn irq_sleepable_global_subprog_indirect(ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags);
    global_subprog_calling_sleepable_global(0);
    bpf_local_irq_restore(&mut flags);
    0
}

/* SEC("?tc") __failure __msg("cannot restore irq state out of order") */
pub unsafe extern "C" fn irq_ooo_lock_cond_inv(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    if bpf_res_spin_lock_irqsave(&mut lockA, &mut flags1) != 0 {
        return 0;
    }
    if bpf_res_spin_lock_irqsave(&mut lockB, &mut flags2) != 0 {
        bpf_res_spin_unlock_irqrestore(&mut lockA, &mut flags1);
        return 0;
    }

    bpf_res_spin_unlock_irqrestore(&mut lockB, &mut flags1);
    bpf_res_spin_unlock_irqrestore(&mut lockA, &mut flags2);
    0
}

/* SEC("?tc") __failure __msg("function calls are not allowed") */
pub unsafe extern "C" fn irq_wrong_kfunc_class_1(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    if bpf_res_spin_lock_irqsave(&mut lockA, &mut flags1) != 0 {
        return 0;
    }
    /* For now, bpf_local_irq_restore is not allowed in critical section,
     * but this test ensures error will be caught with kfunc_class when it's
     * opened up. Tested by temporarily permitting this kfunc in critical
     * section.
     */
    bpf_local_irq_restore(&mut flags1);
    bpf_res_spin_unlock_irqrestore(&mut lockA, &mut flags1);
    0
}

/* SEC("?tc") __failure __msg("function calls are not allowed") */
pub unsafe extern "C" fn irq_wrong_kfunc_class_2(ctx: *mut __sk_buff) -> ::core::ffi::c_int {
    let _ = ctx;
    let mut flags1: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();
    let mut flags2: ::core::ffi::c_ulong = ::core::mem::MaybeUninit::uninit().assume_init();

    bpf_local_irq_save(&mut flags1);
    if bpf_res_spin_lock_irqsave(&mut lockA, &mut flags2) != 0 {
        return 0;
    }
    bpf_local_irq_restore(&mut flags2);
    bpf_res_spin_unlock_irqrestore(&mut lockA, &mut flags1);
    0
}

/* SEC("license") */
#[no_mangle]
pub static _license: [::core::ffi::c_char; 4] = [b'G' as ::core::ffi::c_char, b'P' as ::core::ffi::c_char, b'L' as ::core::ffi::c_char, 0];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
