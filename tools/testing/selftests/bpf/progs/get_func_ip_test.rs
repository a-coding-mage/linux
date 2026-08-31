// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependency intent:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub type __u64 = u64;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_func_ip(ctx: *mut ::core::ffi::c_void) -> __u64;

    // __ksym
    fn bpf_fentry_test1(a: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn bpf_modify_return_test(
        a: ::core::ffi::c_int,
        b: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    // __ksym
    static bpf_fentry_test2: ::core::ffi::c_void;
    static bpf_fentry_test3: ::core::ffi::c_void;
    static bpf_fentry_test4: ::core::ffi::c_void;

    // __kconfig __weak
    static CONFIG_X86_KERNEL_IBT: bool;
}

// SEC("license")
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* This function is here to have CONFIG_X86_KERNEL_IBT
 * used and added to object BTF.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unused() -> ::core::ffi::c_int {
    if unsafe { CONFIG_X86_KERNEL_IBT } {
        0
    } else {
        1
    }
}

#[unsafe(no_mangle)]
pub static mut test1_result: __u64 = 0;

// SEC("fentry/bpf_fentry_test1")
// int BPF_PROG(test1, int a)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test1(ctx: *mut ::core::ffi::c_void, a: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let addr: __u64 = unsafe { bpf_get_func_ip(ctx) };

    unsafe {
        test1_result = ((addr as *const ::core::ffi::c_void) == (bpf_fentry_test1 as *const ::core::ffi::c_void)) as __u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test2_result: __u64 = 0;

// SEC("fexit/bpf_fentry_test2")
// int BPF_PROG(test2, int a)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test2(ctx: *mut ::core::ffi::c_void, a: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let addr: __u64 = unsafe { bpf_get_func_ip(ctx) };

    unsafe {
        test2_result = ((addr as *const ::core::ffi::c_void) == (&bpf_fentry_test2 as *const ::core::ffi::c_void)) as __u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test3_result: __u64 = 0;

// SEC("kprobe/bpf_fentry_test3")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test3(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    let addr: __u64 = unsafe { bpf_get_func_ip(ctx as *mut ::core::ffi::c_void) };

    unsafe {
        test3_result = ((addr as *const ::core::ffi::c_void) == (&bpf_fentry_test3 as *const ::core::ffi::c_void)) as __u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test4_result: __u64 = 0;

// SEC("kretprobe/bpf_fentry_test4")
// int BPF_KRETPROBE(test4)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test4(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    let addr: __u64 = unsafe { bpf_get_func_ip(ctx as *mut ::core::ffi::c_void) };

    unsafe {
        test4_result = ((addr as *const ::core::ffi::c_void) == (&bpf_fentry_test4 as *const ::core::ffi::c_void)) as __u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test5_result: __u64 = 0;

// SEC("fmod_ret/bpf_modify_return_test")
// int BPF_PROG(test5, int a, int *b, int ret)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test5(
    ctx: *mut ::core::ffi::c_void,
    a: ::core::ffi::c_int,
    b: *mut ::core::ffi::c_int,
    ret: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let addr: __u64 = unsafe { bpf_get_func_ip(ctx) };

    unsafe {
        test5_result = ((addr as *const ::core::ffi::c_void) == (bpf_modify_return_test as *const ::core::ffi::c_void)) as __u64;
    }
    ret
}

#[unsafe(no_mangle)]
pub static mut test6_result: __u64 = 0;

// SEC("?kprobe")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test6(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    let addr: __u64 = unsafe { bpf_get_func_ip(ctx as *mut ::core::ffi::c_void) };

    unsafe {
        test6_result = ((addr as *const ::core::ffi::c_void) == ::core::ptr::null()) as __u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut uprobe_trigger: ::core::ffi::c_ulong = 0;

#[unsafe(no_mangle)]
pub static mut test7_result: __u64 = 0;

// SEC("uprobe//proc/self/exe:uprobe_trigger")
// int BPF_UPROBE(test7)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test7(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    let addr: __u64 = unsafe { bpf_get_func_ip(ctx as *mut ::core::ffi::c_void) };

    unsafe {
        test7_result = ((addr as *const ::core::ffi::c_void)
            == (uprobe_trigger as *const ::core::ffi::c_void)) as __u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test8_result: __u64 = 0;

// SEC("uretprobe//proc/self/exe:uprobe_trigger")
// int BPF_URETPROBE(test8, int ret)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test8(ctx: *mut pt_regs, ret: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let addr: __u64 = unsafe { bpf_get_func_ip(ctx as *mut ::core::ffi::c_void) };

    unsafe {
        test8_result = ((addr as *const ::core::ffi::c_void)
            == (uprobe_trigger as *const ::core::ffi::c_void)) as __u64;
    }
    0
}
