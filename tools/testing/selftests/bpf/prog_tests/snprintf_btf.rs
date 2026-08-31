// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, <linux/btf.h>, "netif_receive_skb.skel.h"

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct netif_receive_skb {
    pub bss: *mut netif_receive_skb__bss,
}

#[repr(C)]
pub struct netif_receive_skb__bss {
    pub skip: c_int,
    pub ret: c_int,
    pub ran_subtests: c_int,
    pub num_subtests: c_int,
}

unsafe extern "C" {
    fn netif_receive_skb__open() -> *mut netif_receive_skb;
    fn netif_receive_skb__load(skel: *mut netif_receive_skb) -> c_int;
    fn netif_receive_skb__attach(skel: *mut netif_receive_skb) -> c_int;
    fn netif_receive_skb__destroy(skel: *mut netif_receive_skb);

    fn system(command: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;

    fn test__skip();

    fn CHECK(condition: bool, name: *const c_char, format: *const c_char, ...) -> bool;
    fn ASSERT_GT(left: c_int, right: c_int, name: *const c_char) -> bool;
}

/* Demonstrate that bpf_snprintf_btf succeeds and that various data types
 * are formatted correctly.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_snprintf_btf() {
    let mut skel: *mut netif_receive_skb;
    let bss: *mut netif_receive_skb__bss;
    let mut err: c_int;
    let _duration: c_int = 0;

    skel = unsafe { netif_receive_skb__open() };
    if unsafe {
        CHECK(
            skel.is_null(),
            c"skel_open".as_ptr(),
            c"failed to open skeleton\n".as_ptr(),
        )
    } {
        return;
    }

    err = unsafe { netif_receive_skb__load(skel) };
    if unsafe {
        CHECK(
            err != 0,
            c"skel_load".as_ptr(),
            c"failed to load skeleton: %d\n".as_ptr(),
            err,
        )
    } {
        unsafe { netif_receive_skb__destroy(skel) };
        return;
    }

    bss = unsafe { (*skel).bss };

    err = unsafe { netif_receive_skb__attach(skel) };
    if unsafe {
        CHECK(
            err != 0,
            c"skel_attach".as_ptr(),
            c"skeleton attach failed: %d\n".as_ptr(),
            err,
        )
    } {
        unsafe { netif_receive_skb__destroy(skel) };
        return;
    }

    /* generate receive event */
    err = unsafe { system(c"ping -c 1 127.0.0.1 > /dev/null".as_ptr()) };
    if unsafe {
        CHECK(
            err != 0,
            c"system".as_ptr(),
            c"ping failed: %d\n".as_ptr(),
            err,
        )
    } {
        unsafe { netif_receive_skb__destroy(skel) };
        return;
    }

    if unsafe { (*bss).skip != 0 } {
        unsafe {
            printf(
                c"%s:SKIP:no __builtin_btf_type_id\n".as_ptr(),
                c"serial_test_snprintf_btf".as_ptr(),
            );
            test__skip();
            netif_receive_skb__destroy(skel);
        }
        return;
    }

    /*
     * Make sure netif_receive_skb program was triggered
     * and it set expected return values from bpf_trace_printk()s
     * and all tests ran.
     */
    if !unsafe { ASSERT_GT((*bss).ret, 0, c"bpf_snprintf_ret".as_ptr()) } {
        unsafe { netif_receive_skb__destroy(skel) };
        return;
    }

    if unsafe {
        CHECK(
            (*bss).ran_subtests == 0,
            c"check if subtests ran".as_ptr(),
            c"no subtests ran, did BPF program run?".as_ptr(),
        )
    } {
        unsafe { netif_receive_skb__destroy(skel) };
        return;
    }

    if unsafe {
        CHECK(
            (*bss).num_subtests != (*bss).ran_subtests,
            c"check all subtests ran".as_ptr(),
            c"only ran %d of %d tests\n".as_ptr(),
            (*bss).num_subtests,
            (*bss).ran_subtests,
        )
    } {
        unsafe { netif_receive_skb__destroy(skel) };
        return;
    }

    unsafe { netif_receive_skb__destroy(skel) };
}
