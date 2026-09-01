// SPDX-License-Identifier: GPL-2.0

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, and "bpf_misc.h".
// The SEC, __naked, __imm_const, __imm_addr, __clobber_all, offsetof, and
// struct bpf_sock_ops definitions are supplied by the surrounding BPF build.

/*
 * Test the SOCK_OPS_GET_SK() and SOCK_OPS_GET_FIELD() macros in
 * sock_ops_convert_ctx_access() when dst_reg == src_reg.
 *
 * When dst_reg == src_reg, the macros borrow a temporary register to load
 * is_fullsock / is_locked_tcp_sock, because dst_reg holds the ctx pointer
 * and cannot be clobbered before ctx->sk / ctx->field is read. If
 * is_fullsock == 0 (e.g., TCP_NEW_SYN_RECV with a request_sock), the macro
 * must still zero dst_reg so the verifier's PTR_TO_SOCKET_OR_NULL /
 * SCALAR_VALUE type is correct at runtime. A missing clear leaves a stale
 * ctx pointer in dst_reg that passes NULL checks (GET_SK) or leaks a kernel
 * address as a scalar (GET_FIELD).
 *
 * When dst_reg != src_reg, dst_reg itself is used to load is_fullsock, so
 * the JEQ (dst_reg == 0) naturally leaves it zeroed on the !fullsock path.
 */

#[no_mangle]
pub static mut bug_detected: i32 = 0;
#[no_mangle]
pub static mut null_seen: i32 = 0;

#[link_section = "sockops"]
#[naked]
pub unsafe extern "C" fn sock_ops_get_sk_same_reg() {
    core::arch::asm!(
        "r7 = *(u32 *)(r1 + {is_fullsock_off});",
        "r1 = *(u64 *)(r1 + {sk_off});",
        "if r7 != 0 goto 2f;",
        "if r1 == 0 goto 1f;",
        "r1 = {bug_detected} ll;",
        "r2 = 1;",
        "*(u32 *)(r1 + 0) = r2;",
        "goto 2f;",
        "1:",
        "r1 = {null_seen} ll;",
        "r2 = 1;",
        "*(u32 *)(r1 + 0) = r2;",
        "2:",
        "r0 = 1;",
        "exit;",
        is_fullsock_off = const core::mem::offset_of!(bpf_sock_ops, is_fullsock),
        sk_off = const core::mem::offset_of!(bpf_sock_ops, sk),
        bug_detected = sym bug_detected,
        null_seen = sym null_seen,
        options(noreturn)
    );
}

/* SOCK_OPS_GET_FIELD: same-register, is_locked_tcp_sock == 0 path. */
#[no_mangle]
pub static mut field_bug_detected: i32 = 0;
#[no_mangle]
pub static mut field_null_seen: i32 = 0;

#[link_section = "sockops"]
#[naked]
pub unsafe extern "C" fn sock_ops_get_field_same_reg() {
    core::arch::asm!(
        "r7 = *(u32 *)(r1 + {is_fullsock_off});",
        "r1 = *(u32 *)(r1 + {snd_cwnd_off});",
        "if r7 != 0 goto 2f;",
        "if r1 == 0 goto 1f;",
        "r1 = {field_bug_detected} ll;",
        "r2 = 1;",
        "*(u32 *)(r1 + 0) = r2;",
        "goto 2f;",
        "1:",
        "r1 = {field_null_seen} ll;",
        "r2 = 1;",
        "*(u32 *)(r1 + 0) = r2;",
        "2:",
        "r0 = 1;",
        "exit;",
        is_fullsock_off = const core::mem::offset_of!(bpf_sock_ops, is_fullsock),
        snd_cwnd_off = const core::mem::offset_of!(bpf_sock_ops, snd_cwnd),
        field_bug_detected = sym field_bug_detected,
        field_null_seen = sym field_null_seen,
        options(noreturn)
    );
}

/* SOCK_OPS_GET_SK: different-register, is_fullsock == 0 path. */
#[no_mangle]
pub static mut diff_reg_bug_detected: i32 = 0;
#[no_mangle]
pub static mut diff_reg_null_seen: i32 = 0;

#[link_section = "sockops"]
#[naked]
pub unsafe extern "C" fn sock_ops_get_sk_diff_reg() {
    core::arch::asm!(
        "r7 = r1;",
        "r6 = *(u32 *)(r7 + {is_fullsock_off});",
        "r2 = *(u64 *)(r7 + {sk_off});",
        "if r6 != 0 goto 2f;",
        "if r2 == 0 goto 1f;",
        "r1 = {diff_reg_bug_detected} ll;",
        "r3 = 1;",
        "*(u32 *)(r1 + 0) = r3;",
        "goto 2f;",
        "1:",
        "r1 = {diff_reg_null_seen} ll;",
        "r3 = 1;",
        "*(u32 *)(r1 + 0) = r3;",
        "2:",
        "r0 = 1;",
        "exit;",
        is_fullsock_off = const core::mem::offset_of!(bpf_sock_ops, is_fullsock),
        sk_off = const core::mem::offset_of!(bpf_sock_ops, sk),
        diff_reg_bug_detected = sym diff_reg_bug_detected,
        diff_reg_null_seen = sym diff_reg_null_seen,
        options(noreturn)
    );
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
