// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// <vmlinux.h>, <bpf/bpf_helpers.h>, and "bpf_misc.h".
// The SEC, __success, __retval, __arch_*, and __jited annotations are BPF
// selftest metadata in C; their intent is preserved here as comments.

unsafe extern "C" {
    fn bpf_get_current_task() -> *mut core::ffi::c_void;
    fn bpf_get_smp_processor_id() -> u32;
}

// SEC("fentry/bpf_fentry_test1")
// __success __retval(0)
// __arch_x86_64
// __jited("	addq	%gs:{{.*}}, %rax")
// __arch_arm64
// __jited("	mrs	x8, SP_EL0")
// __arch_riscv64
// __jited("	mv	a5, tp")
// __arch_loongarch
// __jited("	move	$a5, $tp")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn inline_bpf_get_current_task() -> i32 {
    unsafe {
        bpf_get_current_task();
    }

    0
}

// SEC("fentry/bpf_fentry_test2")
// __success __retval(0)
// __arch_loongarch
// __jited("	ld.wu	$a5, $tp, 16")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn inline_bpf_get_smp_processor_id() -> i32 {
    unsafe {
        bpf_get_smp_processor_id();
    }

    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
