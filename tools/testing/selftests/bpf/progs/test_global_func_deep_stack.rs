// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2026 Meta Platforms, Inc and affiliates. */
// C source dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

/*
 * Macro tricks to tersely define for long non-recursive call chains. Add
 * computation to the functions prevent tail recursion from reducing the
 * stack size to 0.
 */

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[inline(never)]
pub unsafe extern "C" fn f0(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = (a.wrapping_add(16)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if a == 0 {
        return 0;
    }
    core::ptr::read_volatile(&b) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f1(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f0(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f2(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f1(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f3(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f2(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f4(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f3(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f5(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f4(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f6(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f5(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f7(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f6(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f8(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f7(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f9(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f8(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f10(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f9(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f11(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f10(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f12(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f11(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f13(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f12(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f14(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f13(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f15(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f14(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f16(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f15(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f17(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f16(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f18(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f17(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f19(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f18(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f20(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f19(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f21(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f20(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f22(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f21(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f23(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f22(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f24(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f23(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f25(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f24(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f26(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f25(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f27(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f26(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f28(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f27(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f29(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f28(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f30(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f29(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f31(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f30(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

#[inline(never)]
pub unsafe extern "C" fn f32(a: libc::c_ulong) -> libc::c_int {
    let b: libc::c_long = f31(a.wrapping_sub(1)) as libc::c_long;
    core::ptr::read_volatile(&b);
    if core::ptr::read_volatile(&b) == 0 {
        return 0;
    }
    (core::ptr::read_volatile(&b) + 1) as libc::c_int
}

/* Ensure that even 32 levels deep, the function verifies. */
// SEC("syscall")
// __success
#[no_mangle]
pub unsafe extern "C" fn global_func_deep_stack_success(
    skb: *mut __sk_buff,
) -> libc::c_int {
    let _ = skb;
    f31(55)
}

/*
 * Check we actually honor stack limits (33 * 16 = 528 > 512 = MAX_STACK_DEPTH).
 * The stack depth is 16 because the verifier calls round_up_stack_depth() on
 * the size.
 */
// SEC("syscall")
// __failure __msg("combined stack size of 34 calls")
#[no_mangle]
pub unsafe extern "C" fn global_func_deep_stack_fail(
    skb: *mut __sk_buff,
) -> libc::c_int {
    let _ = skb;
    f32(123)
}
