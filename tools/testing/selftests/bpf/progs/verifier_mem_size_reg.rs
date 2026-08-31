// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"
// #include "bpf_kfuncs.h"

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

/*
 * The __szk size of a kfunc memory/size pair must be marked precise even when
 * the nullable buffer is passed as NULL.
 */
// SEC("?tc")
// __success __log_level(2)
// __msg("mark_precise: frame0: regs=r4 stack= before")
#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dynptr_slice_null_buf_size_precise(skb: *mut __sk_buff) -> i32 {
    let mut dptr: bpf_dynptr = core::mem::zeroed();
    let p: *mut i8;

    bpf_dynptr_from_skb(skb, 0, &mut dptr);

    p = bpf_dynptr_slice(&mut dptr, 0, core::ptr::null_mut(), 8) as *mut i8;
    if !p.is_null() {
        return *p.add(0) as i32;
    }
    0
}
