// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

/*
 * C dependencies:
 *   #include "vmlinux.h"
 *   #include <bpf/bpf_helpers.h>
 *   #include <bpf/bpf_tracing.h>
 */

use core::ffi::c_void;
use core::ptr::{addr_of, addr_of_mut, read_volatile, write_volatile};

use crate::{__sk_buff, bpf_sockopt};

/*
 * Original C condition:
 * #if (defined(__TARGET_ARCH_arm64) || defined(__TARGET_ARCH_x86) || \
 *      (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64) ||       \
 *      defined(__TARGET_ARCH_s390) || defined(__TARGET_ARCH_loongarch)) && \
 *      __clang_major__ >= 18
 */
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "s390x",
    target_arch = "loongarch64"
))]
#[no_mangle]
pub static skip: i32 = 0;

/*
 * Fallback from the original #else path. The __clang_major__ >= 18 part is not
 * expressible as a Rust cfg from this isolated source file.
 */
#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "x86",
    all(target_arch = "riscv64", target_pointer_width = "64"),
    target_arch = "s390x",
    target_arch = "loongarch64"
)))]
#[no_mangle]
pub static skip: i32 = 1;

#[no_mangle]
pub static val1: i16 = -1;
#[no_mangle]
pub static val2: i32 = -1;
#[no_mangle]
pub static mut val3: i16 = -1;
#[no_mangle]
pub static mut val4: i32 = -1;
#[no_mangle]
pub static mut done1: i32 = 0;
#[no_mangle]
pub static mut done2: i32 = 0;
#[no_mangle]
pub static mut ret1: i32 = 0;
#[no_mangle]
pub static mut ret2: i32 = 0;

#[link_section = "?raw_tp/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn rdonly_map_prog(ctx: *const c_void) -> i32 {
    let _ = ctx;

    if read_volatile(addr_of!(done1)) != 0 {
        return 0;
    }

    write_volatile(addr_of_mut!(done1), 1);
    /* val1/val2 readonly map */
    if (read_volatile(addr_of!(val1)) as i32) == read_volatile(addr_of!(val2)) {
        write_volatile(addr_of_mut!(ret1), 1);
    }
    return 0;
}

#[link_section = "?raw_tp/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn map_val_prog(ctx: *const c_void) -> i32 {
    let _ = ctx;

    if read_volatile(addr_of!(done2)) != 0 {
        return 0;
    }

    write_volatile(addr_of_mut!(done2), 1);
    /* val1/val2 regular read/write map */
    if (read_volatile(addr_of!(val3)) as i32) == read_volatile(addr_of!(val4)) {
        write_volatile(addr_of_mut!(ret2), 1);
    }
    return 0;
}

#[repr(C)]
pub struct bpf_testmod_struct_arg_1 {
    pub a: i32,
}

#[no_mangle]
pub static mut int_member: i64 = 0;

#[link_section = "?fentry/bpf_testmod_test_arg_ptr_to_struct"]
#[no_mangle]
pub unsafe extern "C" fn test_ptr_struct_arg(p: *mut bpf_testmod_struct_arg_1) -> i32 {
    /* probed memory access */
    write_volatile(addr_of_mut!(int_member), read_volatile(addr_of!((*p).a)) as i64);
    return 0;
}

#[no_mangle]
pub static mut set_optlen: i64 = 0;
#[no_mangle]
pub static mut set_retval: i64 = 0;

#[link_section = "?cgroup/getsockopt"]
#[no_mangle]
pub unsafe extern "C" fn _getsockopt(ctx: *mut bpf_sockopt) -> i32 {
    let old_optlen: i32;
    let old_retval: i32;

    old_optlen = read_volatile(addr_of!((*ctx).optlen));
    old_retval = read_volatile(addr_of!((*ctx).retval));

    write_volatile(addr_of_mut!((*ctx).optlen), -1);
    write_volatile(addr_of_mut!((*ctx).retval), -1);

    /* sign extension for ctx member */
    write_volatile(
        addr_of_mut!(set_optlen),
        read_volatile(addr_of!((*ctx).optlen)) as i64,
    );
    write_volatile(
        addr_of_mut!(set_retval),
        read_volatile(addr_of!((*ctx).retval)) as i64,
    );

    write_volatile(addr_of_mut!((*ctx).optlen), old_optlen);
    write_volatile(addr_of_mut!((*ctx).retval), old_retval);

    return 0;
}

#[no_mangle]
pub static mut set_mark: i64 = 0;

#[link_section = "?tc"]
#[no_mangle]
pub unsafe extern "C" fn _tc(skb: *mut __sk_buff) -> i32 {
    let tmp_mark: i64;
    let old_mark: i32;

    old_mark = read_volatile(addr_of!((*skb).mark));

    write_volatile(addr_of_mut!((*skb).mark), 0xf6fe);

    /* narrowed sign extension for ctx member */
    /*
     * Original C used inline BPF assembly for __clang_major__ >= 18 to force a
     * narrow one-byte signed load:
     *
     * asm volatile ("r1 = *(s8 *)(%[ctx] + %[off_mark])\n\t"
     *               "%[tmp_mark] = r1"
     *               : [tmp_mark]"=r"(tmp_mark)
     *               : [ctx]"r"(skb),
     *                 [off_mark]"i"(offsetof(struct __sk_buff, mark)
     * #if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
     *                 + sizeof(skb->mark) - 1
     * #endif
     *                 )
     *               : "r1");
     *
     * The fallback expression is the direct Rust equivalent of:
     * tmp_mark = (char)skb->mark;
     */
    tmp_mark = read_volatile(addr_of!((*skb).mark)) as i8 as i64;
    write_volatile(addr_of_mut!(set_mark), tmp_mark);

    write_volatile(addr_of_mut!((*skb).mark), old_mark);

    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
