// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
// <bpf/bpf_core_read.h>, and "bpf_misc.h".

pub type __u64 = u64;

unsafe extern "C" {
    fn bpf_jiffies64() -> __u64;
    fn bpf_get_current_pid_tgid() -> __u64;
}

#[unsafe(no_mangle)]
pub static mut in_user: __u64 = 0;
#[unsafe(no_mangle)]
pub static mut ret_user: __u64 = 0;

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;

/*
 * Skip all the tests if compiler doesn't support indirect jumps.
 *
 * If tests are skipped, then all functions below are compiled as
 * dummy, such that the skeleton looks the same, and the userspace
 * program can avoid any checks rather than if data->skip is set.
 */
// In C this is conditional on __BPF_FEATURE_GOTOX:
//   with __BPF_FEATURE_GOTOX: __u64 skip SEC(".data") = 0;
//   otherwise:               __u64 skip = 1;
#[unsafe(no_mangle)]
#[unsafe(link_section = ".data")]
pub static mut skip: __u64 = 0;

#[repr(C)]
pub struct simple_ctx {
    pub x: __u64,
}

#[unsafe(no_mangle)]
pub static mut some_var: __u64 = 0;

/*
 * This function adds code which will be replaced by a different
 * number of instructions by the verifier. This adds additional
 * stress on testing the insn_array maps corresponding to indirect jumps.
 */
#[inline(always)]
unsafe fn adjust_insns(x: __u64) {
    unsafe {
        some_var ^= x.wrapping_add(bpf_jiffies64());
    }
}

// SEC("syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn one_switch(ctx: *mut simple_ctx) -> i32 {
    unsafe {
        match (*ctx).x {
            0 => {
                adjust_insns((*ctx).x.wrapping_add(1));
                ret_user = 2;
            }
            1 => {
                adjust_insns((*ctx).x.wrapping_add(7));
                ret_user = 3;
            }
            2 => {
                adjust_insns((*ctx).x.wrapping_add(9));
                ret_user = 4;
            }
            3 => {
                adjust_insns((*ctx).x.wrapping_add(11));
                ret_user = 5;
            }
            4 => {
                adjust_insns((*ctx).x.wrapping_add(17));
                ret_user = 7;
            }
            _ => {
                adjust_insns((*ctx).x.wrapping_add(177));
                ret_user = 19;
            }
        }

        0
    }
}

// SEC("syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn one_switch_non_zero_sec_off(ctx: *mut simple_ctx) -> i32 {
    unsafe {
        match (*ctx).x {
            0 => {
                adjust_insns((*ctx).x.wrapping_add(1));
                ret_user = 2;
            }
            1 => {
                adjust_insns((*ctx).x.wrapping_add(7));
                ret_user = 3;
            }
            2 => {
                adjust_insns((*ctx).x.wrapping_add(9));
                ret_user = 4;
            }
            3 => {
                adjust_insns((*ctx).x.wrapping_add(11));
                ret_user = 5;
            }
            4 => {
                adjust_insns((*ctx).x.wrapping_add(17));
                ret_user = 7;
            }
            _ => {
                adjust_insns((*ctx).x.wrapping_add(177));
                ret_user = 19;
            }
        }

        0
    }
}

#[repr(C)]
pub struct pt_regs {
    _unused: [u8; 0],
}

// SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn simple_test_other_sec(_ctx: *mut pt_regs) -> i32 {
    unsafe {
        let x: __u64 = in_user;

        if bpf_get_current_pid_tgid() >> 32 != pid as __u64 {
            return 0;
        }

        match x {
            0 => {
                adjust_insns(x.wrapping_add(1));
                ret_user = 2;
            }
            1 => {
                adjust_insns(x.wrapping_add(7));
                ret_user = 3;
            }
            2 => {
                adjust_insns(x.wrapping_add(9));
                ret_user = 4;
            }
            3 => {
                adjust_insns(x.wrapping_add(11));
                ret_user = 5;
            }
            4 => {
                adjust_insns(x.wrapping_add(17));
                ret_user = 7;
            }
            _ => {
                adjust_insns(x.wrapping_add(177));
                ret_user = 19;
            }
        }

        0
    }
}

// SEC("syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn two_switches(ctx: *mut simple_ctx) -> i32 {
    unsafe {
        match (*ctx).x {
            0 => {
                adjust_insns((*ctx).x.wrapping_add(1));
                ret_user = 2;
            }
            1 => {
                adjust_insns((*ctx).x.wrapping_add(7));
                ret_user = 3;
            }
            2 => {
                adjust_insns((*ctx).x.wrapping_add(9));
                ret_user = 4;
            }
            3 => {
                adjust_insns((*ctx).x.wrapping_add(11));
                ret_user = 5;
            }
            4 => {
                adjust_insns((*ctx).x.wrapping_add(17));
                ret_user = 7;
            }
            _ => {
                adjust_insns((*ctx).x.wrapping_add(177));
                ret_user = 19;
            }
        }

        match (*ctx).x.wrapping_add((ret_user != 0) as __u64) {
            1 => {
                adjust_insns((*ctx).x.wrapping_add(7));
                ret_user = 103;
            }
            2 => {
                adjust_insns((*ctx).x.wrapping_add(9));
                ret_user = 104;
            }
            3 => {
                adjust_insns((*ctx).x.wrapping_add(11));
                ret_user = 107;
            }
            4 => {
                adjust_insns((*ctx).x.wrapping_add(11));
                ret_user = 205;
            }
            5 => {
                adjust_insns((*ctx).x.wrapping_add(11));
                ret_user = 115;
            }
            _ => {
                adjust_insns((*ctx).x.wrapping_add(177));
                ret_user = 1019;
            }
        }

        0
    }
}

// SEC("syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn big_jump_table(ctx: *mut simple_ctx) -> i32 {
    unsafe {
        // C uses a 256-entry computed-goto table with default_label at all
        // entries except 0, 11, 27, and 31.
        match (*ctx).x & 0xff {
            0 => {
                adjust_insns((*ctx).x.wrapping_add(1));
                ret_user = 2;
                0
            }
            11 => {
                adjust_insns((*ctx).x.wrapping_add(7));
                ret_user = 3;
                0
            }
            27 => {
                adjust_insns((*ctx).x.wrapping_add(9));
                ret_user = 4;
                0
            }
            31 => {
                adjust_insns((*ctx).x.wrapping_add(11));
                ret_user = 5;
                0
            }
            _ => {
                adjust_insns((*ctx).x.wrapping_add(177));
                ret_user = 19;
                0
            }
        }
    }
}

// SEC("syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn one_jump_two_maps(ctx: *mut simple_ctx) -> i32 {
    unsafe {
        let a: u32 = ((*ctx).x % 2) as u32;
        let b: u32 = (((*ctx).x / 2) % 2) as u32;
        let mut ret: i32 = 0;

        if !(a < 2 && b < 2) {
            return 19;
        }

        // Computed goto into fallthrough labels l1..l4.
        let start = if (*ctx).x % 2 != 0 {
            if a == 0 { 1 } else { 2 }
        } else if b == 0 {
            3
        } else {
            4
        };

        if start <= 1 {
            core::ptr::write_volatile(&mut ret, core::ptr::read_volatile(&ret).wrapping_add(1));
        }
        if start <= 2 {
            core::ptr::write_volatile(&mut ret, core::ptr::read_volatile(&ret).wrapping_add(3));
        }
        if start <= 3 {
            core::ptr::write_volatile(&mut ret, core::ptr::read_volatile(&ret).wrapping_add(5));
        }
        if start <= 4 {
            core::ptr::write_volatile(&mut ret, core::ptr::read_volatile(&ret).wrapping_add(7));
        }

        ret_user = core::ptr::read_volatile(&ret) as __u64;
        core::ptr::read_volatile(&ret)
    }
}

// SEC("syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn one_map_two_jumps(ctx: *mut simple_ctx) -> i32 {
    unsafe {
        let a: u32 = (((*ctx).x >> 2) & 1) as u32;
        let b: u32 = (((*ctx).x >> 3) & 1) as u32;
        let mut ret: i32 = 0;

        // Computed goto into fallthrough labels l1..l3.
        let mut start = 1u32;
        if (*ctx).x % 2 != 0 {
            start = a + 1;
        } else if (*ctx).x % 3 != 0 {
            start = a + b + 1;
        }

        if start <= 1 {
            core::ptr::write_volatile(&mut ret, core::ptr::read_volatile(&ret).wrapping_add(3));
        }
        if start <= 2 {
            core::ptr::write_volatile(&mut ret, core::ptr::read_volatile(&ret).wrapping_add(5));
        }
        if start <= 3 {
            core::ptr::write_volatile(&mut ret, core::ptr::read_volatile(&ret).wrapping_add(7));
        }

        ret_user = core::ptr::read_volatile(&ret) as __u64;
        core::ptr::read_volatile(&ret)
    }
}

/* Just to introduce some non-zero offsets in .text */
#[inline(never)]
unsafe fn f0(ctx: *mut simple_ctx) -> i32 {
    if !ctx.is_null() {
        1
    } else {
        13
    }
}

// SEC("syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn f1(ctx: *mut simple_ctx) -> i32 {
    unsafe {
        ret_user = 0;
        f0(ctx)
    }
}

#[inline(never)]
unsafe fn __static_global(x: __u64) -> i32 {
    unsafe {
        match x {
            0 => {
                adjust_insns(x.wrapping_add(1));
                ret_user = 2;
            }
            1 => {
                adjust_insns(x.wrapping_add(7));
                ret_user = 3;
            }
            2 => {
                adjust_insns(x.wrapping_add(9));
                ret_user = 4;
            }
            3 => {
                adjust_insns(x.wrapping_add(11));
                ret_user = 5;
            }
            4 => {
                adjust_insns(x.wrapping_add(17));
                ret_user = 7;
            }
            _ => {
                adjust_insns(x.wrapping_add(177));
                ret_user = 19;
            }
        }

        0
    }
}

// SEC("syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn use_static_global1(ctx: *mut simple_ctx) -> i32 {
    unsafe {
        ret_user = 0;
        __static_global((*ctx).x)
    }
}

// SEC("syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn use_static_global2(ctx: *mut simple_ctx) -> i32 {
    unsafe {
        ret_user = 0;
        adjust_insns((*ctx).x.wrapping_add(1));
        __static_global((*ctx).x)
    }
}

// SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn use_static_global_other_sec(_ctx: *mut core::ffi::c_void) -> i32 {
    unsafe {
        if bpf_get_current_pid_tgid() >> 32 != pid as __u64 {
            return 0;
        }

        __static_global(in_user)
    }
}

#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __nonstatic_global(x: __u64) -> i32 {
    unsafe {
        match x {
            0 => {
                adjust_insns(x.wrapping_add(1));
                ret_user = 2;
            }
            1 => {
                adjust_insns(x.wrapping_add(7));
                ret_user = 3;
            }
            2 => {
                adjust_insns(x.wrapping_add(9));
                ret_user = 4;
            }
            3 => {
                adjust_insns(x.wrapping_add(11));
                ret_user = 5;
            }
            4 => {
                adjust_insns(x.wrapping_add(17));
                ret_user = 7;
            }
            _ => {
                adjust_insns(x.wrapping_add(177));
                ret_user = 19;
            }
        }

        0
    }
}

// SEC("syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn use_nonstatic_global1(ctx: *mut simple_ctx) -> i32 {
    unsafe {
        ret_user = 0;
        __nonstatic_global((*ctx).x)
    }
}

// SEC("syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn use_nonstatic_global2(ctx: *mut simple_ctx) -> i32 {
    unsafe {
        ret_user = 0;
        adjust_insns((*ctx).x.wrapping_add(1));
        __nonstatic_global((*ctx).x)
    }
}

// SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn use_nonstatic_global_other_sec(_ctx: *mut core::ffi::c_void) -> i32 {
    unsafe {
        if bpf_get_current_pid_tgid() >> 32 != pid as __u64 {
            return 0;
        }

        __nonstatic_global(in_user)
    }
}

// SEC("syscall")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn load_with_nonzero_offset(ctx: *mut simple_ctx) -> i32 {
    unsafe {
        /*
         * This makes LLVM to generate a load from the jj map with an offset:
         *      r1 = 0x0 ll
         *      r1 = *(u64 *)(r1 + 0x10)
         *      gotox r1
         */
        if (*ctx).x == 2 {
            // C jumps through jj[ctx->x], which is l3 for x == 2.
            ret_user = 5;
            return 5;
        }

        ret_user = 1;
        1

        // l1:
        // never reached, but leave it here to outsmart LLVM
        // ret_user = 0; return 0;
        // l2:
        // never reached, but leave it here to outsmart LLVM
        // ret_user = 3; return 3;
    }
}

// Else branch for missing __BPF_FEATURE_GOTOX in C:
// SKIP_TEST(one_switch);
// SKIP_TEST(one_switch_non_zero_sec_off);
// SKIP_TEST(simple_test_other_sec);
// SKIP_TEST(two_switches);
// SKIP_TEST(big_jump_table);
// SKIP_TEST(one_jump_two_maps);
// SKIP_TEST(one_map_two_jumps);
// SKIP_TEST(use_static_global1);
// SKIP_TEST(use_static_global2);
// SKIP_TEST(use_static_global_other_sec);
// SKIP_TEST(use_nonstatic_global1);
// SKIP_TEST(use_nonstatic_global2);
// SKIP_TEST(use_nonstatic_global_other_sec);
// SKIP_TEST(load_with_nonzero_offset);

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
