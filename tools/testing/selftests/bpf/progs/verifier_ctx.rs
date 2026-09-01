// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/ctx.c */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

type __u32 = u32;
type __u64 = u64;

static ctx_strncmp_target: [u8; 4] = *b"ctx\0";
static ctx_snprintf_fmt: [u8; 1] = *b"\0";

extern "C" {
    fn bpf_get_prandom_u32() -> __u32;
    fn bpf_strncmp(s1: *const c_void, s1_sz: __u32, s2: *const u8) -> i32;
    fn bpf_probe_read_kernel(dst: *mut c_void, size: __u32, unsafe_ptr: *const c_void) -> i32;
    fn bpf_snprintf(str_: *mut c_void, str_size: __u32, fmt: *const u8, ...) -> i32;
    fn bpf_kfunc_call_test_mem_len_pass1(mem: *mut c_void, len: __u64);
}

// SEC("tc")
// __description("context stores via BPF_ATOMIC")
// __failure __msg("BPF_ATOMIC stores into R1 ctx is not allowed")
#[no_mangle]
pub unsafe extern "C" fn context_stores_via_bpf_atomic() {
    core::arch::asm!(
        "r0 = 0",
        "lock *(u32 *)(r1 + {__sk_buff_mark}) += w0",
        "exit",
        __sk_buff_mark = const 0,
        options(noreturn)
    );
}

// SEC("tc")
// __description("arithmetic ops make PTR_TO_CTX unusable")
// __failure __msg("dereference of modified ctx ptr")
#[no_mangle]
pub unsafe extern "C" fn make_ptr_to_ctx_unusable() {
    core::arch::asm!(
        "r1 += {__imm_0}",
        "r0 = *(u32*)(r1 + {__sk_buff_mark})",
        "exit",
        __imm_0 = const 0,
        __sk_buff_mark = const 0,
        options(noreturn)
    );
}

macro_rules! naked_bpf_asm_fn {
    ($(#[$meta:meta])* $name:ident, $asm_text:literal) => {
        $(#[$meta])*
        #[no_mangle]
        pub unsafe extern "C" fn $name() {
            core::arch::asm!($asm_text, options(noreturn));
        }
    };
}

// SEC("tc")
// __description("pass unmodified ctx pointer to helper")
// __success __retval(0)
naked_bpf_asm_fn!(unmodified_ctx_pointer_to_helper, "r2 = 0; call bpf_csum_update; r0 = 0; exit");

// SEC("tc")
// __description("pass modified ctx pointer to helper, 1")
// __failure __msg("negative offset ctx ptr R1 off=-612 disallowed")
naked_bpf_asm_fn!(ctx_pointer_to_helper_1, "r1 += -612; r2 = 0; call bpf_csum_update; r0 = 0; exit");

// SEC("socket")
// __description("pass modified ctx pointer to helper, 2")
// __failure __msg("negative offset ctx ptr R1 off=-612 disallowed")
naked_bpf_asm_fn!(ctx_pointer_to_helper_2, "r1 += -612; call bpf_get_socket_cookie; r0 = 0; exit");

// SEC("tc")
// __description("pass modified ctx pointer to helper, 3")
// __failure __msg("variable ctx access var_off=(0x0; 0x4)")
naked_bpf_asm_fn!(ctx_pointer_to_helper_3, "r3 = *(u32*)(r1 + 0); r3 &= 4; r1 += r3; r2 = 0; call bpf_csum_update; r0 = 0; exit");

// SEC("cgroup/sendmsg6")
// __description("pass ctx or null check, 1: ctx")
// __success
naked_bpf_asm_fn!(or_null_check_1_ctx, "call bpf_get_netns_cookie; r0 = 0; exit");

// SEC("cgroup/sendmsg6")
// __description("pass ctx or null check, 2: null")
// __success
naked_bpf_asm_fn!(or_null_check_2_null, "r1 = 0; call bpf_get_netns_cookie; r0 = 0; exit");

// SEC("cgroup/sendmsg6")
// __description("pass ctx or null check, 3: 1")
// __failure __msg("R1 type=scalar expected=ctx")
naked_bpf_asm_fn!(or_null_check_3_1, "r1 = 1; call bpf_get_netns_cookie; r0 = 0; exit");

// SEC("cgroup/sendmsg6")
// __description("pass ctx or null check, 4: ctx - const")
// __failure __msg("negative offset ctx ptr R1 off=-612 disallowed")
naked_bpf_asm_fn!(null_check_4_ctx_const, "r1 += -612; call bpf_get_netns_cookie; r0 = 0; exit");

// SEC("cgroup/connect4")
// __description("pass ctx or null check, 5: null (connect)")
// __success
naked_bpf_asm_fn!(null_check_5_null_connect, "r1 = 0; call bpf_get_netns_cookie; r0 = 0; exit");

// SEC("cgroup/post_bind4")
// __description("pass ctx or null check, 6: null (bind)")
// __success
naked_bpf_asm_fn!(null_check_6_null_bind, "r1 = 0; call bpf_get_netns_cookie; r0 = 0; exit");

// SEC("cgroup/post_bind4")
// __description("pass ctx or null check, 7: ctx (bind)")
// __success
naked_bpf_asm_fn!(null_check_7_ctx_bind, "call bpf_get_socket_cookie; r0 = 0; exit");

// SEC("cgroup/post_bind4")
// __description("pass ctx or null check, 8: null (bind)")
// __failure __msg("R1 type=scalar expected=ctx")
naked_bpf_asm_fn!(null_check_8_null_bind, "r1 = 0; call bpf_get_socket_cookie; r0 = 0; exit");

macro_rules! narrow_load {
    ($sec:literal, $ctx:ident, $field:ident, $name:ident) => {
        // SEC($sec)
        // __description(concat!("narrow load on field ", stringify!($field), " of ", stringify!($ctx)))
        // __failure __msg("invalid bpf_context access")
        #[no_mangle]
        pub unsafe extern "C" fn $name() {
            core::arch::asm!("r1 = *(u32 *)(r1 + off); r0 = 0; exit;", options(noreturn));
        }
    };
}

narrow_load!("cgroup/getsockopt", bpf_sockopt, sk, invalid_narrow_loadbpf_sockoptsk);
narrow_load!("cgroup/getsockopt", bpf_sockopt, optval, invalid_narrow_loadbpf_sockoptoptval);
narrow_load!("cgroup/getsockopt", bpf_sockopt, optval_end, invalid_narrow_loadbpf_sockoptoptval_end);
narrow_load!("tc", __sk_buff, sk, invalid_narrow_load__sk_buffsk);
narrow_load!("cgroup/bind4", bpf_sock_addr, sk, invalid_narrow_loadbpf_sock_addrsk);
narrow_load!("sockops", bpf_sock_ops, sk, invalid_narrow_loadbpf_sock_opssk);
narrow_load!("sockops", bpf_sock_ops, skb_data, invalid_narrow_loadbpf_sock_opsskb_data);
narrow_load!("sockops", bpf_sock_ops, skb_data_end, invalid_narrow_loadbpf_sock_opsskb_data_end);
narrow_load!("sockops", bpf_sock_ops, skb_hwtstamp, invalid_narrow_loadbpf_sock_opsskb_hwtstamp);

macro_rules! unaligned_access {
    ($sec:literal, $ctx:ident, $field:ident, $name:ident) => {
        // SEC($sec)
        // __description(concat!("unaligned access on field ", stringify!($field), " of ", stringify!($ctx)))
        // __failure __msg("invalid bpf_context access")
        #[no_mangle]
        pub unsafe extern "C" fn $name() {
            core::arch::asm!("r1 = *(u64 *)(r1 + off); r0 = 0; exit;", options(noreturn));
        }
    };
}

unaligned_access!("flow_dissector", __sk_buff, data, unaligned_ctx_access___sk_buffdata);
unaligned_access!("netfilter", bpf_nf_ctx, skb, unaligned_ctx_access_bpf_nf_ctxskb);

macro_rules! padding_access {
    ($sec:literal, $ctx:ident, $prev_field:tt, $sz:literal, $name:ident) => {
        // SEC($sec)
        // __description(concat!("access on ", stringify!($ctx), " padding after ", stringify!($prev_field)))
        #[no_mangle]
        pub unsafe extern "C" fn $name() {
            core::arch::asm!("r1 = *(u64 *)(r1 + off); r0 = 0; exit;", options(noreturn));
        }
    };
}

// __failure __msg("invalid bpf_context access")
padding_access!("cgroup/bind4", bpf_sock_addr, msg_src_ip6[3], 4, padding_ctx_access_bpf_sock_addr);
// __success
padding_access!("sk_lookup", bpf_sk_lookup, remote_port, 2, padding_ctx_access_bpf_sk_lookup);
// __failure __msg("invalid bpf_context access")
padding_access!("tc", __sk_buff, tstamp_type, 2, padding_ctx_access___sk_buff);
// __failure __msg("invalid bpf_context access")
padding_access!("cgroup/post_bind4", bpf_sock, dst_port, 2, padding_ctx_access_bpf_sock);
// __failure __msg("invalid bpf_context access")
padding_access!("sk_reuseport", sk_reuseport_md, hash, 4, padding_ctx_access_sk_reuseport_md);

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_fixed_off_write(ctx: *mut c_void) -> i32 {
    let p = ctx as *mut u8;
    *(p as *mut __u32) = 0;
    *(p.add(4) as *mut __u32) = 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_fixed_off_read(ctx: *mut c_void) -> i32 {
    let p = ctx as *mut u8;
    let val: __u32 = core::ptr::read_volatile(p.add(4) as *const __u32);
    let _ = val;
    0
}

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_unaligned_fixed_off_read(ctx: *mut c_void) -> i32 {
    let p = ctx as *mut u8;
    let val: __u32 = core::ptr::read_volatile(p.add(2) as *const __u32);
    let _ = val;
    0
}

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_unaligned_fixed_off_write(ctx: *mut c_void) -> i32 {
    let p = ctx as *mut u8;
    *(p.add(2) as *mut __u32) = 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_var_off_read(ctx: *mut c_void) -> i32 {
    let mut off: __u64 = bpf_get_prandom_u32() as __u64;
    let mut p = ctx as *mut u8;
    off &= 0xfc;
    p = p.add(off as usize);
    let val: __u32 = core::ptr::read_volatile(p as *const __u32);
    let _ = val;
    0
}

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_var_off_write(ctx: *mut c_void) -> i32 {
    let mut off: __u64 = bpf_get_prandom_u32() as __u64;
    let mut p = ctx as *mut u8;
    off &= 0xfc;
    p = p.add(off as usize);
    *(p as *mut __u32) = 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_unaligned_var_off_read(ctx: *mut c_void) -> i32 {
    let mut off: __u64 = bpf_get_prandom_u32() as __u64;
    let mut p = ctx as *mut u8;
    off &= 0xfc;
    off += 2;
    p = p.add(off as usize);
    let val: __u32 = core::ptr::read_volatile(p as *const __u32);
    let _ = val;
    0
}

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_unaligned_var_off_write(ctx: *mut c_void) -> i32 {
    let mut off: __u64 = bpf_get_prandom_u32() as __u64;
    let mut p = ctx as *mut u8;
    off &= 0xfc;
    off += 2;
    p = p.add(off as usize);
    *(p as *mut __u32) = 0;
    0
}

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_u16_max_fixed_off(ctx: *mut c_void) -> i32 {
    let mut p = ctx as *mut u8;
    p = p.add(65535);
    let val: __u32 = core::ptr::read_volatile(p as *const __u32);
    let _ = val;
    0
}

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_u16_max_var_off(ctx: *mut c_void) -> i32 {
    let mut off: __u64 = bpf_get_prandom_u32() as __u64;
    let mut p = ctx as *mut u8;
    off &= 0xffff;
    off += 1;
    p = p.add(off as usize);
    let val: __u32 = core::ptr::read_volatile(p as *const __u32);
    let _ = val;
    0
}

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_neg_var_off(ctx: *mut c_void) -> i32 {
    let mut off: __u64 = bpf_get_prandom_u32() as __u64;
    let mut p = ctx as *mut u8;
    off &= 4;
    p = p.sub(off as usize);
    *(p as *const __u32) as i32
}

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_unbounded_var_off(ctx: *mut c_void) -> i32 {
    let mut off: __u64 = bpf_get_prandom_u32() as __u32 as __u64;
    let mut p = ctx as *mut u8;
    off <<= 2;
    p = p.add(off as usize);
    *(p as *const __u32) as i32
}

macro_rules! syscall_helper_read {
    ($name:ident, $adj:block) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(ctx: *mut c_void) -> i32 {
            let mut off: __u64 = 0;
            let mut p = ctx as *mut u8;
            $adj
            bpf_strncmp(p as *const c_void, 4, ctx_strncmp_target.as_ptr())
        }
    };
}

macro_rules! syscall_helper_write {
    ($name:ident, $adj:block) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(ctx: *mut c_void) -> i32 {
            let mut off: __u64 = 0;
            let mut p = ctx as *mut u8;
            $adj
            bpf_probe_read_kernel(p as *mut c_void, 4, core::ptr::null())
        }
    };
}

syscall_helper_read!(syscall_ctx_helper_fixed_off_read, { p = p.add(4); });
syscall_helper_write!(syscall_ctx_helper_fixed_off_write, { p = p.add(4); });
syscall_helper_read!(syscall_ctx_helper_unaligned_fixed_off_read, { p = p.add(2); });
syscall_helper_write!(syscall_ctx_helper_unaligned_fixed_off_write, { p = p.add(2); });
syscall_helper_read!(syscall_ctx_helper_var_off_read, { off = bpf_get_prandom_u32() as __u64; off &= 0xfc; p = p.add(off as usize); });
syscall_helper_write!(syscall_ctx_helper_var_off_write, { off = bpf_get_prandom_u32() as __u64; off &= 0xfc; p = p.add(off as usize); });
syscall_helper_read!(syscall_ctx_helper_unaligned_var_off_read, { off = bpf_get_prandom_u32() as __u64; off &= 0xfc; off += 2; p = p.add(off as usize); });
syscall_helper_write!(syscall_ctx_helper_unaligned_var_off_write, { off = bpf_get_prandom_u32() as __u64; off &= 0xfc; off += 2; p = p.add(off as usize); });
syscall_helper_read!(syscall_ctx_helper_u16_max_fixed_off_read, { p = p.add(65535); });
syscall_helper_write!(syscall_ctx_helper_u16_max_fixed_off_write, { p = p.add(65535); });
syscall_helper_read!(syscall_ctx_helper_u16_max_var_off_read, { off = bpf_get_prandom_u32() as __u64; off &= 0xffff; off += 1; p = p.add(off as usize); });
syscall_helper_write!(syscall_ctx_helper_u16_max_var_off_write, { off = bpf_get_prandom_u32() as __u64; off &= 0xffff; off += 1; p = p.add(off as usize); });

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_zero_sized_read(ctx: *mut c_void) -> i32 {
    bpf_snprintf(core::ptr::null_mut(), 0, ctx_snprintf_fmt.as_ptr(), ctx, 0)
}

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_zero_sized_write(ctx: *mut c_void) -> i32 {
    bpf_probe_read_kernel(ctx, 0, core::ptr::null())
}

macro_rules! syscall_kfunc {
    ($name:ident, $adj:block) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(ctx: *mut c_void) -> i32 {
            let mut off: __u64 = 0;
            let mut p = ctx as *mut u8;
            $adj
            bpf_kfunc_call_test_mem_len_pass1(p as *mut c_void, 4);
            0
        }
    };
}

syscall_kfunc!(syscall_ctx_kfunc_fixed_off, { p = p.add(4); });
syscall_kfunc!(syscall_ctx_kfunc_var_off, { off = bpf_get_prandom_u32() as __u64; off &= 0xfc; p = p.add(off as usize); });
syscall_kfunc!(syscall_ctx_kfunc_unaligned_fixed_off, { p = p.add(2); });
syscall_kfunc!(syscall_ctx_kfunc_unaligned_var_off, { off = bpf_get_prandom_u32() as __u64; off &= 0xfc; off += 2; p = p.add(off as usize); });
syscall_kfunc!(syscall_ctx_kfunc_u16_max_fixed_off, { p = p.add(65535); });
syscall_kfunc!(syscall_ctx_kfunc_u16_max_var_off, { off = bpf_get_prandom_u32() as __u64; off &= 0xffff; off += 1; p = p.add(off as usize); });

#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_kfunc_zero_sized(ctx: *mut c_void) -> i32 {
    bpf_kfunc_call_test_mem_len_pass1(ctx, 0);
    0
}

/*
 * For non-syscall program types without convert_ctx_access, direct ctx
 * dereference is still allowed after adding a fixed offset, while variable
 * and negative direct accesses reject.
 *
 * Passing ctx as a helper or kfunc memory argument is only permitted for
 * syscall programs, so the helper and kfunc cases below validate rejection
 * for non-syscall ctx pointers at fixed, variable, and zero-sized accesses.
 */
macro_rules! no_rewrite_ctx_access {
    ($type_name:literal, $name:ident, $off:expr, $load_t:ty,
     $fixed:ident, $var:ident, $neg:ident, $helper_read_fixed:ident,
     $helper_write_fixed:ident, $helper_read_var:ident, $helper_write_var:ident,
     $helper_read_zero:ident, $helper_write_zero:ident, $kfunc_fixed:ident,
     $kfunc_var:ident, $kfunc_zero:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $fixed(ctx: *mut c_void) -> i32 {
            let p = ctx as *mut u8;
            let val: $load_t = core::ptr::read_volatile(p.add($off as usize) as *const $load_t);
            let _ = val;
            0
        }

        #[no_mangle]
        pub unsafe extern "C" fn $var(ctx: *mut c_void) -> i32 {
            let mut off_var: __u64 = bpf_get_prandom_u32() as __u64;
            let mut p = ctx as *mut u8;
            off_var &= 4;
            p = p.add(off_var as usize);
            *(p as *const $load_t) as i32
        }

        #[no_mangle]
        pub unsafe extern "C" fn $neg(ctx: *mut c_void) -> i32 {
            let mut p = ctx as *mut u8;
            p = p.sub(612);
            *(p as *const $load_t) as i32
        }

        #[no_mangle]
        pub unsafe extern "C" fn $helper_read_fixed(ctx: *mut c_void) -> i32 {
            let mut p = ctx as *mut u8;
            p = p.add($off as usize);
            bpf_strncmp(p as *const c_void, 4, ctx_strncmp_target.as_ptr())
        }

        #[no_mangle]
        pub unsafe extern "C" fn $helper_write_fixed(ctx: *mut c_void) -> i32 {
            let mut p = ctx as *mut u8;
            p = p.add($off as usize);
            bpf_probe_read_kernel(p as *mut c_void, 4, core::ptr::null())
        }

        #[no_mangle]
        pub unsafe extern "C" fn $helper_read_var(ctx: *mut c_void) -> i32 {
            let mut off_var: __u64 = bpf_get_prandom_u32() as __u64;
            let mut p = ctx as *mut u8;
            off_var &= 4;
            p = p.add(off_var as usize);
            bpf_strncmp(p as *const c_void, 4, ctx_strncmp_target.as_ptr())
        }

        #[no_mangle]
        pub unsafe extern "C" fn $helper_write_var(ctx: *mut c_void) -> i32 {
            let mut off_var: __u64 = bpf_get_prandom_u32() as __u64;
            let mut p = ctx as *mut u8;
            off_var &= 4;
            p = p.add(off_var as usize);
            bpf_probe_read_kernel(p as *mut c_void, 4, core::ptr::null())
        }

        #[no_mangle]
        pub unsafe extern "C" fn $helper_read_zero(ctx: *mut c_void) -> i32 {
            bpf_snprintf(core::ptr::null_mut(), 0, ctx_snprintf_fmt.as_ptr(), ctx, 0)
        }

        #[no_mangle]
        pub unsafe extern "C" fn $helper_write_zero(ctx: *mut c_void) -> i32 {
            bpf_probe_read_kernel(ctx, 0, core::ptr::null())
        }

        #[no_mangle]
        pub unsafe extern "C" fn $kfunc_fixed(ctx: *mut c_void) -> i32 {
            let mut p = ctx as *mut u8;
            p = p.add($off as usize);
            bpf_kfunc_call_test_mem_len_pass1(p as *mut c_void, 4);
            0
        }

        #[no_mangle]
        pub unsafe extern "C" fn $kfunc_var(ctx: *mut c_void) -> i32 {
            let mut off_var: __u64 = bpf_get_prandom_u32() as __u64;
            let mut p = ctx as *mut u8;
            off_var &= 4;
            p = p.add(off_var as usize);
            bpf_kfunc_call_test_mem_len_pass1(p as *mut c_void, 4);
            0
        }

        #[no_mangle]
        pub unsafe extern "C" fn $kfunc_zero(ctx: *mut c_void) -> i32 {
            bpf_kfunc_call_test_mem_len_pass1(ctx, 0);
            0
        }
    };
}

no_rewrite_ctx_access!("kprobe", kprobe, 8, u64, no_rewrite_kprobe_fixed, no_rewrite_kprobe_var, no_rewrite_kprobe_neg, no_rewrite_kprobe_helper_read_fixed, no_rewrite_kprobe_helper_write_fixed, no_rewrite_kprobe_helper_read_var, no_rewrite_kprobe_helper_write_var, no_rewrite_kprobe_helper_read_zero, no_rewrite_kprobe_helper_write_zero, no_rewrite_kprobe_kfunc_fixed, no_rewrite_kprobe_kfunc_var, no_rewrite_kprobe_kfunc_zero);
no_rewrite_ctx_access!("tracepoint", tp, 8, u64, no_rewrite_tp_fixed, no_rewrite_tp_var, no_rewrite_tp_neg, no_rewrite_tp_helper_read_fixed, no_rewrite_tp_helper_write_fixed, no_rewrite_tp_helper_read_var, no_rewrite_tp_helper_write_var, no_rewrite_tp_helper_read_zero, no_rewrite_tp_helper_write_zero, no_rewrite_tp_kfunc_fixed, no_rewrite_tp_kfunc_var, no_rewrite_tp_kfunc_zero);
no_rewrite_ctx_access!("raw_tp", raw_tp, 8, u64, no_rewrite_raw_tp_fixed, no_rewrite_raw_tp_var, no_rewrite_raw_tp_neg, no_rewrite_raw_tp_helper_read_fixed, no_rewrite_raw_tp_helper_write_fixed, no_rewrite_raw_tp_helper_read_var, no_rewrite_raw_tp_helper_write_var, no_rewrite_raw_tp_helper_read_zero, no_rewrite_raw_tp_helper_write_zero, no_rewrite_raw_tp_kfunc_fixed, no_rewrite_raw_tp_kfunc_var, no_rewrite_raw_tp_kfunc_zero);
no_rewrite_ctx_access!("raw_tracepoint.w", raw_tp_w, 8, u64, no_rewrite_raw_tp_w_fixed, no_rewrite_raw_tp_w_var, no_rewrite_raw_tp_w_neg, no_rewrite_raw_tp_w_helper_read_fixed, no_rewrite_raw_tp_w_helper_write_fixed, no_rewrite_raw_tp_w_helper_read_var, no_rewrite_raw_tp_w_helper_write_var, no_rewrite_raw_tp_w_helper_read_zero, no_rewrite_raw_tp_w_helper_write_zero, no_rewrite_raw_tp_w_kfunc_fixed, no_rewrite_raw_tp_w_kfunc_var, no_rewrite_raw_tp_w_kfunc_zero);
no_rewrite_ctx_access!("fentry/bpf_modify_return_test", fentry, 8, u64, no_rewrite_fentry_fixed, no_rewrite_fentry_var, no_rewrite_fentry_neg, no_rewrite_fentry_helper_read_fixed, no_rewrite_fentry_helper_write_fixed, no_rewrite_fentry_helper_read_var, no_rewrite_fentry_helper_write_var, no_rewrite_fentry_helper_read_zero, no_rewrite_fentry_helper_write_zero, no_rewrite_fentry_kfunc_fixed, no_rewrite_fentry_kfunc_var, no_rewrite_fentry_kfunc_zero);
no_rewrite_ctx_access!("cgroup/dev", cgroup_dev, 4, u32, no_rewrite_cgroup_dev_fixed, no_rewrite_cgroup_dev_var, no_rewrite_cgroup_dev_neg, no_rewrite_cgroup_dev_helper_read_fixed, no_rewrite_cgroup_dev_helper_write_fixed, no_rewrite_cgroup_dev_helper_read_var, no_rewrite_cgroup_dev_helper_write_var, no_rewrite_cgroup_dev_helper_read_zero, no_rewrite_cgroup_dev_helper_write_zero, no_rewrite_cgroup_dev_kfunc_fixed, no_rewrite_cgroup_dev_kfunc_var, no_rewrite_cgroup_dev_kfunc_zero);
// The original offset is offsetof(struct bpf_nf_ctx, skb), supplied by external BPF headers.
no_rewrite_ctx_access!("netfilter", netfilter, 0, u64, no_rewrite_netfilter_fixed, no_rewrite_netfilter_var, no_rewrite_netfilter_neg, no_rewrite_netfilter_helper_read_fixed, no_rewrite_netfilter_helper_write_fixed, no_rewrite_netfilter_helper_read_var, no_rewrite_netfilter_helper_write_var, no_rewrite_netfilter_helper_read_zero, no_rewrite_netfilter_helper_write_zero, no_rewrite_netfilter_kfunc_fixed, no_rewrite_netfilter_kfunc_var, no_rewrite_netfilter_kfunc_zero);

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
