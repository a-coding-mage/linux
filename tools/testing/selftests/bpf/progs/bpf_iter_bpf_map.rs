// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// C source included <vmlinux.h> and <bpf/bpf_helpers.h>; their Rust
// equivalents are expected to provide the BPF types, SEC helper, and
// BPF_SEQ_PRINTF macro used below.

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "iter/bpf_map"]
pub unsafe extern "C" fn dump_bpf_map(ctx: *mut bpf_iter__bpf_map) -> ::core::ffi::c_int {
    let seq: *mut seq_file = (*(*ctx).meta).seq;
    let seq_num: __u64 = (*(*ctx).meta).seq_num;
    let map: *mut bpf_map = (*ctx).map;

    if map == 0 as *mut bpf_map {
        BPF_SEQ_PRINTF!(seq, "      %%%%%% END %%%%%%\n");
        return 0;
    }

    if seq_num == 0 {
        BPF_SEQ_PRINTF!(seq, "      id   refcnt  usercnt  locked_vm\n");
    }

    BPF_SEQ_PRINTF!(
        seq,
        "%8u %8ld %8ld %10lu\n",
        (*map).id,
        (*map).refcnt.counter,
        (*map).usercnt.counter,
        0u64
    );
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
