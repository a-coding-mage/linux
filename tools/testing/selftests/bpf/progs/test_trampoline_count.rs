// SPDX-License-Identifier: GPL-2.0
// Depends on Linux BPF helper/tracing definitions provided by the build environment.

#[unsafe(no_mangle)]
#[unsafe(link_section = "fentry/bpf_testmod_trampoline_count_test")]
pub extern "C" fn fentry_test() -> i32 {
    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "fmod_ret/bpf_testmod_trampoline_count_test")]
pub extern "C" fn fmod_ret_test(ret: i32) -> i32 {
    let _ = ret;
    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "fexit/bpf_testmod_trampoline_count_test")]
pub extern "C" fn fexit_test(ret: i32) -> i32 {
    let _ = ret;
    return 0;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";
