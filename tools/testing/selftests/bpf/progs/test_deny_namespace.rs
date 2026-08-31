// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
// <errno.h>, <linux/capability.h>.

const EPERM: i32 = 1;
const CAP_SYS_ADMIN: u64 = 21;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_cap_t {
    pub val: u64,
}

#[repr(C)]
pub struct cred {
    pub cap_effective: kernel_cap_t,
}
// C attribute preserved intent: __attribute__((preserve_access_index)).

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// C section: SEC("lsm.s/userns_create")
// C declaration used BPF_PROG(test_userns_create, const struct cred *cred, int ret).
#[no_mangle]
#[link_section = "lsm.s/userns_create"]
pub unsafe extern "C" fn test_userns_create(cred: *const cred, mut ret: i32) -> i32 {
    let caps: kernel_cap_t = (*cred).cap_effective;
    let cap_mask: u64 = 1u64 << CAP_SYS_ADMIN;

    if ret != 0 {
        return 0;
    }

    ret = -EPERM;
    if (caps.val & cap_mask) != 0 {
        return 0;
    }

    -EPERM
}
