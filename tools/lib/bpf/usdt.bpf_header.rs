/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

/* Includes from the original header:
 * <linux/errno.h>
 * "bpf_helpers.h"
 * "bpf_tracing.h"
 */

/* Below types and maps are internal implementation details of libbpf's USDT
 * support and are subjects to change. Also, bpf_usdt_xxx() API helpers should
 * be considered an unstable API as well and might be adjusted based on user
 * feedback from using libbpf's USDT support in production.
 */

/* User can override BPF_USDT_MAX_SPEC_CNT to change default size of internal
 * map that keeps track of USDT argument specifications. This might be
 * necessary if there are a lot of USDT attachments.
 */
pub const BPF_USDT_MAX_SPEC_CNT: usize = 256;

/* User can override BPF_USDT_MAX_IP_CNT to change default size of internal
 * map that keeps track of IP (memory address) mapping to USDT argument
 * specification.
 * Note, if kernel supports BPF cookies, this map is not used and could be
 * resized all the way to 1 to save a bit of memory.
 */
pub const BPF_USDT_MAX_IP_CNT: usize = 4 * BPF_USDT_MAX_SPEC_CNT;

pub const ESRCH: ::core::ffi::c_int = 3;
pub const ENOENT: ::core::ffi::c_int = 2;
pub const EINVAL: ::core::ffi::c_int = 22;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum __bpf_usdt_arg_type {
    BPF_USDT_ARG_CONST = 0,
    BPF_USDT_ARG_REG = 1,
    BPF_USDT_ARG_REG_DEREF = 2,
    BPF_USDT_ARG_SIB = 3,
}

/* This struct layout is designed specifically to be backwards/forward
 * compatible between libbpf versions for ARG_CONST, ARG_REG, and
 * ARG_REG_DEREF modes. ARG_SIB requires libbpf v1.7+.
 *
 * The original C definition uses endian-dependent bitfields packed into the
 * four bytes following val_off:
 *   little endian: arg_type:8, idx_reg_off:12, scale_bitshift:4, reserved:8
 *   big endian:    reserved:8, idx_reg_off:12, scale_bitshift:4, arg_type:8
 * Rust has no C-compatible bitfield syntax, so the packed field is preserved
 * as a u32 and accessed with endian-specific helpers below.
 */
#[repr(C)]
pub struct __bpf_usdt_arg_spec {
    /* u64 scalar interpreted depending on arg_type, see below */
    pub val_off: u64,
    pub __arg_type_idx_reg_off_scale_bitshift_reserved: u32,
    /* offset of referenced register within struct pt_regs */
    pub reg_off: i16,
    /* whether arg should be interpreted as signed value */
    pub arg_signed: bool,
    /* number of bits that need to be cleared and, optionally,
     * sign-extended to cast arguments that are 1, 2, or 4 bytes
     * long into final 8-byte u64/s64 value returned to user
     */
    pub arg_bitshift: i8,
}

impl __bpf_usdt_arg_spec {
    #[inline(always)]
    pub fn arg_type(&self) -> u8 {
        if cfg!(target_endian = "little") {
            (self.__arg_type_idx_reg_off_scale_bitshift_reserved & 0xff) as u8
        } else {
            (self.__arg_type_idx_reg_off_scale_bitshift_reserved & 0xff) as u8
        }
    }

    #[inline(always)]
    pub fn idx_reg_off(&self) -> u16 {
        if cfg!(target_endian = "little") {
            ((self.__arg_type_idx_reg_off_scale_bitshift_reserved >> 8) & 0x0fff) as u16
        } else {
            ((self.__arg_type_idx_reg_off_scale_bitshift_reserved >> 12) & 0x0fff) as u16
        }
    }

    #[inline(always)]
    pub fn scale_bitshift(&self) -> u16 {
        if cfg!(target_endian = "little") {
            ((self.__arg_type_idx_reg_off_scale_bitshift_reserved >> 20) & 0x0f) as u16
        } else {
            ((self.__arg_type_idx_reg_off_scale_bitshift_reserved >> 8) & 0x0f) as u16
        }
    }
}

/* should match USDT_MAX_ARG_CNT in usdt.c exactly */
pub const BPF_USDT_MAX_ARG_CNT: usize = 12;

#[repr(C)]
pub struct __bpf_usdt_spec {
    pub args: [__bpf_usdt_arg_spec; BPF_USDT_MAX_ARG_CNT],
    pub usdt_cookie: u64,
    pub arg_cnt: i16,
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def {
    _private: [u8; 0],
}

unsafe extern "C" {
    /* Original BPF map declarations:
     * struct {
     *     __uint(type, BPF_MAP_TYPE_ARRAY);
     *     __uint(max_entries, BPF_USDT_MAX_SPEC_CNT);
     *     __type(key, int);
     *     __type(value, struct __bpf_usdt_spec);
     * } __bpf_usdt_specs SEC(".maps") __weak;
     *
     * struct {
     *     __uint(type, BPF_MAP_TYPE_HASH);
     *     __uint(max_entries, BPF_USDT_MAX_IP_CNT);
     *     __type(key, long);
     *     __type(value, __u32);
     * } __bpf_usdt_ip_to_spec_id SEC(".maps") __weak;
     */
    pub static mut __bpf_usdt_specs: bpf_map_def;
    pub static mut __bpf_usdt_ip_to_spec_id: bpf_map_def;

    pub static LINUX_HAS_BPF_COOKIE: bool;

    pub fn PT_REGS_IP(ctx: *mut pt_regs) -> isize;
    pub fn bpf_map_lookup_elem(
        map: *mut bpf_map_def,
        key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    pub fn bpf_get_attach_cookie(ctx: *mut pt_regs) -> i64;
    pub fn bpf_probe_read_kernel(
        dst: *mut ::core::ffi::c_void,
        size: usize,
        unsafe_ptr: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    pub fn bpf_probe_read_user(
        dst: *mut ::core::ffi::c_void,
        size: usize,
        unsafe_ptr: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}

#[inline(always)]
pub unsafe fn barrier_var<T>(_var: T) {}

#[inline(always)]
pub unsafe fn __bpf_usdt_spec_id(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    if !LINUX_HAS_BPF_COOKIE {
        let ip: isize = PT_REGS_IP(ctx);
        let spec_id_ptr: *mut ::core::ffi::c_int = bpf_map_lookup_elem(
            &raw mut __bpf_usdt_ip_to_spec_id,
            &ip as *const _ as *const ::core::ffi::c_void,
        ) as *mut ::core::ffi::c_int;

        return if !spec_id_ptr.is_null() {
            *spec_id_ptr
        } else {
            -ESRCH
        };
    }

    bpf_get_attach_cookie(ctx) as ::core::ffi::c_int
}

/* Return number of USDT arguments defined for currently traced USDT. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_usdt_arg_cnt(ctx: *mut pt_regs) -> ::core::ffi::c_int {
    let spec: *mut __bpf_usdt_spec;
    let spec_id: ::core::ffi::c_int;

    spec_id = __bpf_usdt_spec_id(ctx);
    if spec_id < 0 {
        return -ESRCH;
    }

    spec = bpf_map_lookup_elem(
        &raw mut __bpf_usdt_specs,
        &spec_id as *const _ as *const ::core::ffi::c_void,
    ) as *mut __bpf_usdt_spec;
    if spec.is_null() {
        return -ESRCH;
    }

    (*spec).arg_cnt as ::core::ffi::c_int
}

/* Returns the size in bytes of the #*arg_num* (zero-indexed) USDT argument.
 * Returns negative error if argument is not found or arg_num is invalid.
 */
#[inline(always)]
pub unsafe fn bpf_usdt_arg_size(ctx: *mut pt_regs, mut arg_num: u64) -> ::core::ffi::c_int {
    let arg_spec: *mut __bpf_usdt_arg_spec;
    let spec: *mut __bpf_usdt_spec;
    let spec_id: ::core::ffi::c_int;

    spec_id = __bpf_usdt_spec_id(ctx);
    if spec_id < 0 {
        return -ESRCH;
    }

    spec = bpf_map_lookup_elem(
        &raw mut __bpf_usdt_specs,
        &spec_id as *const _ as *const ::core::ffi::c_void,
    ) as *mut __bpf_usdt_spec;
    if spec.is_null() {
        return -ESRCH;
    }

    if arg_num >= BPF_USDT_MAX_ARG_CNT as u64 {
        return -ENOENT;
    }
    barrier_var(arg_num);
    if arg_num >= (*spec).arg_cnt as u64 {
        return -ENOENT;
    }

    arg_spec = (*spec).args.as_mut_ptr().add(arg_num as usize);

    /* arg_spec->arg_bitshift = 64 - arg_sz * 8
     * so: arg_sz = (64 - arg_spec->arg_bitshift) / 8
     */
    ((64u32).wrapping_sub((*arg_spec).arg_bitshift as u32) / 8) as ::core::ffi::c_int
}

/* Fetch USDT argument #*arg_num* (zero-indexed) and put its value into *res.
 * Returns 0 on success; negative error, otherwise.
 * On error *res is guaranteed to be set to zero.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_usdt_arg(
    ctx: *mut pt_regs,
    mut arg_num: u64,
    res: *mut ::core::ffi::c_long,
) -> ::core::ffi::c_int {
    let spec: *mut __bpf_usdt_spec;
    let arg_spec: *mut __bpf_usdt_arg_spec;
    let mut val: ::core::ffi::c_ulong;
    let mut idx: ::core::ffi::c_ulong = 0;
    let mut err: ::core::ffi::c_int;
    let spec_id: ::core::ffi::c_int;

    *res = 0;

    spec_id = __bpf_usdt_spec_id(ctx);
    if spec_id < 0 {
        return -ESRCH;
    }

    spec = bpf_map_lookup_elem(
        &raw mut __bpf_usdt_specs,
        &spec_id as *const _ as *const ::core::ffi::c_void,
    ) as *mut __bpf_usdt_spec;
    if spec.is_null() {
        return -ESRCH;
    }

    if arg_num >= BPF_USDT_MAX_ARG_CNT as u64 {
        return -ENOENT;
    }
    barrier_var(arg_num);
    if arg_num >= (*spec).arg_cnt as u64 {
        return -ENOENT;
    }

    arg_spec = (*spec).args.as_mut_ptr().add(arg_num as usize);
    match (*arg_spec).arg_type() as u32 {
        x if x == __bpf_usdt_arg_type::BPF_USDT_ARG_CONST as u32 => {
            /* Arg is just a constant ("-4@$-9" in USDT arg spec).
             * value is recorded in arg_spec->val_off directly.
             */
            val = (*arg_spec).val_off as ::core::ffi::c_ulong;
        }
        x if x == __bpf_usdt_arg_type::BPF_USDT_ARG_REG as u32 => {
            /* Arg is in a register (e.g, "8@%rax" in USDT arg spec),
             * so we read the contents of that register directly from
             * struct pt_regs. To keep things simple user-space parts
             * record offsetof(struct pt_regs, <regname>) in arg_spec->reg_off.
             */
            err = bpf_probe_read_kernel(
                &mut val as *mut _ as *mut ::core::ffi::c_void,
                ::core::mem::size_of_val(&val),
                (ctx as *mut u8).offset((*arg_spec).reg_off as isize)
                    as *const ::core::ffi::c_void,
            );
            if err != 0 {
                return err;
            }
        }
        x if x == __bpf_usdt_arg_type::BPF_USDT_ARG_REG_DEREF as u32 => {
            /* Arg is in memory addressed by register, plus some offset
             * (e.g., "-4@-1204(%rbp)" in USDT arg spec). Register is
             * identified like with BPF_USDT_ARG_REG case, and the offset
             * is in arg_spec->val_off. We first fetch register contents
             * from pt_regs, then do another user-space probe read to
             * fetch argument value itself.
             */
            err = bpf_probe_read_kernel(
                &mut val as *mut _ as *mut ::core::ffi::c_void,
                ::core::mem::size_of_val(&val),
                (ctx as *mut u8).offset((*arg_spec).reg_off as isize)
                    as *const ::core::ffi::c_void,
            );
            if err != 0 {
                return err;
            }
            err = bpf_probe_read_user(
                &mut val as *mut _ as *mut ::core::ffi::c_void,
                ::core::mem::size_of_val(&val),
                (val as *mut u8).add((*arg_spec).val_off as usize)
                    as *const ::core::ffi::c_void,
            );
            if err != 0 {
                return err;
            }
            if cfg!(target_endian = "big") {
                val >>= (*arg_spec).arg_bitshift as u32;
            }
        }
        x if x == __bpf_usdt_arg_type::BPF_USDT_ARG_SIB as u32 => {
            /* Arg is in memory addressed by SIB (Scale-Index-Base) mode
             * (e.g., "-1@-96(%rbp,%rax,8)" in USDT arg spec). We first
             * fetch the base register contents and the index register
             * contents from pt_regs. Then we calculate the final address
             * as base + (index * scale) + offset, and do a user-space
             * probe read to fetch the argument value.
             */
            err = bpf_probe_read_kernel(
                &mut val as *mut _ as *mut ::core::ffi::c_void,
                ::core::mem::size_of_val(&val),
                (ctx as *mut u8).offset((*arg_spec).reg_off as isize)
                    as *const ::core::ffi::c_void,
            );
            if err != 0 {
                return err;
            }
            err = bpf_probe_read_kernel(
                &mut idx as *mut _ as *mut ::core::ffi::c_void,
                ::core::mem::size_of_val(&idx),
                (ctx as *mut u8).add((*arg_spec).idx_reg_off() as usize)
                    as *const ::core::ffi::c_void,
            );
            if err != 0 {
                return err;
            }
            err = bpf_probe_read_user(
                &mut val as *mut _ as *mut ::core::ffi::c_void,
                ::core::mem::size_of_val(&val),
                (val
                    .wrapping_add(idx << ((*arg_spec).scale_bitshift() as u32))
                    .wrapping_add((*arg_spec).val_off as ::core::ffi::c_ulong))
                    as *const ::core::ffi::c_void,
            );
            if err != 0 {
                return err;
            }
            if cfg!(target_endian = "big") {
                val >>= (*arg_spec).arg_bitshift as u32;
            }
        }
        _ => {
            return -EINVAL;
        }
    }

    /* cast arg from 1, 2, or 4 bytes to final 8 byte size clearing
     * necessary upper arg_bitshift bits, with sign extension if argument
     * is signed
     */
    val <<= (*arg_spec).arg_bitshift as u32;
    if (*arg_spec).arg_signed {
        val = (((val as ::core::ffi::c_long) >> ((*arg_spec).arg_bitshift as u32))
            as ::core::ffi::c_ulong);
    } else {
        val >>= (*arg_spec).arg_bitshift as u32;
    }
    *res = val as ::core::ffi::c_long;
    0
}

/* Retrieve user-specified cookie value provided during attach as
 * bpf_usdt_opts.usdt_cookie. This serves the same purpose as BPF cookie
 * returned by bpf_get_attach_cookie(). Libbpf's support for USDT is itself
 * utilizing BPF cookies internally, so user can't use BPF cookie directly
 * for USDT programs and has to use bpf_usdt_cookie() API instead.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bpf_usdt_cookie(ctx: *mut pt_regs) -> ::core::ffi::c_long {
    let spec: *mut __bpf_usdt_spec;
    let spec_id: ::core::ffi::c_int;

    spec_id = __bpf_usdt_spec_id(ctx);
    if spec_id < 0 {
        return 0;
    }

    spec = bpf_map_lookup_elem(
        &raw mut __bpf_usdt_specs,
        &spec_id as *const _ as *const ::core::ffi::c_void,
    ) as *mut __bpf_usdt_spec;
    if spec.is_null() {
        return 0;
    }

    (*spec).usdt_cookie as ::core::ffi::c_long
}

/* we rely on ___bpf_apply() and ___bpf_narg() macros already defined in bpf_tracing.h */
#[macro_export]
macro_rules! ___bpf_usdt_args0 {
    () => {
        ctx
    };
}

#[macro_export]
macro_rules! ___bpf_usdt_arg {
    ($n:expr) => {{
        let mut _x: ::core::ffi::c_long = 0;
        bpf_usdt_arg(ctx, $n, &mut _x);
        _x
    }};
}

#[macro_export]
macro_rules! ___bpf_usdt_args1 {
    ($x:tt) => {
        ctx, ___bpf_usdt_arg!(0)
    };
}

#[macro_export]
macro_rules! ___bpf_usdt_args2 {
    ($x:tt, $($args:tt)*) => {
        ___bpf_usdt_args1!($($args)*), ___bpf_usdt_arg!(1)
    };
}

#[macro_export]
macro_rules! ___bpf_usdt_args3 {
    ($x:tt, $($args:tt)*) => {
        ___bpf_usdt_args2!($($args)*), ___bpf_usdt_arg!(2)
    };
}

#[macro_export]
macro_rules! ___bpf_usdt_args4 {
    ($x:tt, $($args:tt)*) => {
        ___bpf_usdt_args3!($($args)*), ___bpf_usdt_arg!(3)
    };
}

#[macro_export]
macro_rules! ___bpf_usdt_args5 {
    ($x:tt, $($args:tt)*) => {
        ___bpf_usdt_args4!($($args)*), ___bpf_usdt_arg!(4)
    };
}

#[macro_export]
macro_rules! ___bpf_usdt_args6 {
    ($x:tt, $($args:tt)*) => {
        ___bpf_usdt_args5!($($args)*), ___bpf_usdt_arg!(5)
    };
}

#[macro_export]
macro_rules! ___bpf_usdt_args7 {
    ($x:tt, $($args:tt)*) => {
        ___bpf_usdt_args6!($($args)*), ___bpf_usdt_arg!(6)
    };
}

#[macro_export]
macro_rules! ___bpf_usdt_args8 {
    ($x:tt, $($args:tt)*) => {
        ___bpf_usdt_args7!($($args)*), ___bpf_usdt_arg!(7)
    };
}

#[macro_export]
macro_rules! ___bpf_usdt_args9 {
    ($x:tt, $($args:tt)*) => {
        ___bpf_usdt_args8!($($args)*), ___bpf_usdt_arg!(8)
    };
}

#[macro_export]
macro_rules! ___bpf_usdt_args10 {
    ($x:tt, $($args:tt)*) => {
        ___bpf_usdt_args9!($($args)*), ___bpf_usdt_arg!(9)
    };
}

#[macro_export]
macro_rules! ___bpf_usdt_args11 {
    ($x:tt, $($args:tt)*) => {
        ___bpf_usdt_args10!($($args)*), ___bpf_usdt_arg!(10)
    };
}

#[macro_export]
macro_rules! ___bpf_usdt_args12 {
    ($x:tt, $($args:tt)*) => {
        ___bpf_usdt_args11!($($args)*), ___bpf_usdt_arg!(11)
    };
}

/* ___bpf_usdt_args(args...) expands through ___bpf_apply(___bpf_usdt_args,
 * ___bpf_narg(args))(args) in the original C header.
 */

/* BPF_USDT serves the same purpose for USDT handlers as BPF_PROG for
 * tp_btf/fentry/fexit BPF programs and BPF_KPROBE for kprobes.
 * Original struct pt_regs * context is preserved as 'ctx' argument.
 *
 * The original macro generates a public wrapper named `name` and an inline
 * implementation named `____name`, forwarding ctx and fetched USDT arguments.
 * Rust cannot concatenate identifiers in macro_rules without external support,
 * so this file preserves the macro intent rather than inventing a dependency.
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
