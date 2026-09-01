// SPDX-License-Identifier: GPL-2.0
// Dependencies from the C source:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, <stdbool.h>,
// "bpf_kfuncs.h", and "bpf_misc.h".

type __u64 = u64;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    #[link_name = "bpf_fentry_test1"]
    static bpf_fentry_test1: core::ffi::c_void;
    #[link_name = "bpf_fentry_test2"]
    static bpf_fentry_test2: core::ffi::c_void;
    #[link_name = "bpf_fentry_test3"]
    static bpf_fentry_test3: core::ffi::c_void;
    #[link_name = "bpf_fentry_test4"]
    static bpf_fentry_test4: core::ffi::c_void;
    #[link_name = "bpf_fentry_test5"]
    static bpf_fentry_test5: core::ffi::c_void;
    #[link_name = "bpf_fentry_test6"]
    static bpf_fentry_test6: core::ffi::c_void;
    #[link_name = "bpf_fentry_test7"]
    static bpf_fentry_test7: core::ffi::c_void;
    #[link_name = "bpf_fentry_test8"]
    static bpf_fentry_test8: core::ffi::c_void;

    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_get_func_ip(ctx: *mut core::ffi::c_void) -> __u64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;

#[unsafe(no_mangle)]
pub static mut kprobe_session_result: [__u64; 8] = [0; 8];

unsafe fn session_check(ctx: *mut core::ffi::c_void) -> i32 {
    let mut i: u32;
    let addr: __u64;
    let kfuncs: [*const core::ffi::c_void; 8] = [
        &bpf_fentry_test1 as *const core::ffi::c_void,
        &bpf_fentry_test2 as *const core::ffi::c_void,
        &bpf_fentry_test3 as *const core::ffi::c_void,
        &bpf_fentry_test4 as *const core::ffi::c_void,
        &bpf_fentry_test5 as *const core::ffi::c_void,
        &bpf_fentry_test6 as *const core::ffi::c_void,
        &bpf_fentry_test7 as *const core::ffi::c_void,
        &bpf_fentry_test8 as *const core::ffi::c_void,
    ];

    if (bpf_get_current_pid_tgid() >> 32) != pid as __u64 {
        return 1;
    }

    addr = bpf_get_func_ip(ctx);

    i = 0;
    while (i as usize) < kfuncs.len() {
        if kfuncs[i as usize] == addr as *mut core::ffi::c_void as *const core::ffi::c_void {
            kprobe_session_result[i as usize] = kprobe_session_result[i as usize].wrapping_add(1);
            break;
        }
        i = i.wrapping_add(1);
    }

    /*
     * Force probes for function bpf_fentry_test[5-8] not to
     * install and execute the return probe
     */
    if (addr as *const core::ffi::c_void == &bpf_fentry_test5 as *const core::ffi::c_void)
        || (addr as *const core::ffi::c_void == &bpf_fentry_test6 as *const core::ffi::c_void)
        || (addr as *const core::ffi::c_void == &bpf_fentry_test7 as *const core::ffi::c_void)
        || (addr as *const core::ffi::c_void == &bpf_fentry_test8 as *const core::ffi::c_void)
    {
        return 1;
    }

    return 0;
}

/*
 * No tests in here, just to trigger 'bpf_fentry_test*'
 * through tracing test_run
 */
#[unsafe(link_section = "fentry/bpf_modify_return_test")]
#[unsafe(no_mangle)]
pub extern "C" fn trigger() -> i32 {
    return 0;
}

#[unsafe(link_section = "kprobe.session/bpf_fentry_test*")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kprobe(ctx: *mut pt_regs) -> i32 {
    return session_check(ctx as *mut core::ffi::c_void);
}

/*
 * Exact function name (no wildcards) - exercises the fast syms[] path
 * in bpf_program__attach_kprobe_multi_opts() which bypasses kallsyms parsing.
 */
#[unsafe(link_section = "kprobe.session/bpf_fentry_test1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kprobe_syms(ctx: *mut pt_regs) -> i32 {
    return session_check(ctx as *mut core::ffi::c_void);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
