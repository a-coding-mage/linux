// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/lwt.c */

// C dependencies removed from executable Rust:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h".
// The SEC, __description, __success, __failure, __retval, __msg,
// __failure_unpriv, __naked, __imm, __imm_const, __clobber_all, and
// offsetof(...) test metadata/helper macros are preserved below as comments.

unsafe extern "C" {
    fn bpf_skb_change_head(skb: *mut __sk_buff, len: u32, flags: u64) -> i64;
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

// SEC("lwt_in")
// __description("invalid direct packet write for LWT_IN")
// __failure __msg("cannot write into packet")
// __naked
#[no_mangle]
pub unsafe extern "C" fn packet_write_for_lwt_in() {
    unsafe {
        core::arch::asm!(
            "r2 = *(u32*)(r1 + {__sk_buff_data});",
            "r3 = *(u32*)(r1 + {__sk_buff_data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r0 > r3 goto 0f;",
            "*(u8*)(r2 + 0) = r2;",
            "0:",
            "r0 = 0;",
            "exit;",
            __sk_buff_data = const 0,     // offsetof(struct __sk_buff, data)
            __sk_buff_data_end = const 0, // offsetof(struct __sk_buff, data_end)
            options(noreturn)
        );
    }
}

// SEC("lwt_out")
// __description("invalid direct packet write for LWT_OUT")
// __failure __msg("cannot write into packet")
// __naked
#[no_mangle]
pub unsafe extern "C" fn packet_write_for_lwt_out() {
    unsafe {
        core::arch::asm!(
            "r2 = *(u32*)(r1 + {__sk_buff_data});",
            "r3 = *(u32*)(r1 + {__sk_buff_data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r0 > r3 goto 0f;",
            "*(u8*)(r2 + 0) = r2;",
            "0:",
            "r0 = 0;",
            "exit;",
            __sk_buff_data = const 0,     // offsetof(struct __sk_buff, data)
            __sk_buff_data_end = const 0, // offsetof(struct __sk_buff, data_end)
            options(noreturn)
        );
    }
}

// SEC("lwt_xmit")
// __description("direct packet write for LWT_XMIT")
// __success __retval(0)
// __naked
#[no_mangle]
pub unsafe extern "C" fn packet_write_for_lwt_xmit() {
    unsafe {
        core::arch::asm!(
            "r2 = *(u32*)(r1 + {__sk_buff_data});",
            "r3 = *(u32*)(r1 + {__sk_buff_data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r0 > r3 goto 0f;",
            "*(u8*)(r2 + 0) = r2;",
            "0:",
            "r0 = 0;",
            "exit;",
            __sk_buff_data = const 0,     // offsetof(struct __sk_buff, data)
            __sk_buff_data_end = const 0, // offsetof(struct __sk_buff, data_end)
            options(noreturn)
        );
    }
}

// SEC("lwt_in")
// __description("direct packet read for LWT_IN")
// __success __retval(0)
// __naked
#[no_mangle]
pub unsafe extern "C" fn packet_read_for_lwt_in() {
    unsafe {
        core::arch::asm!(
            "r2 = *(u32*)(r1 + {__sk_buff_data});",
            "r3 = *(u32*)(r1 + {__sk_buff_data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r0 > r3 goto 0f;",
            "r0 = *(u8*)(r2 + 0);",
            "0:",
            "r0 = 0;",
            "exit;",
            __sk_buff_data = const 0,     // offsetof(struct __sk_buff, data)
            __sk_buff_data_end = const 0, // offsetof(struct __sk_buff, data_end)
            options(noreturn)
        );
    }
}

// SEC("lwt_out")
// __description("direct packet read for LWT_OUT")
// __success __retval(0)
// __naked
#[no_mangle]
pub unsafe extern "C" fn packet_read_for_lwt_out() {
    unsafe {
        core::arch::asm!(
            "r2 = *(u32*)(r1 + {__sk_buff_data});",
            "r3 = *(u32*)(r1 + {__sk_buff_data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r0 > r3 goto 0f;",
            "r0 = *(u8*)(r2 + 0);",
            "0:",
            "r0 = 0;",
            "exit;",
            __sk_buff_data = const 0,     // offsetof(struct __sk_buff, data)
            __sk_buff_data_end = const 0, // offsetof(struct __sk_buff, data_end)
            options(noreturn)
        );
    }
}

// SEC("lwt_xmit")
// __description("direct packet read for LWT_XMIT")
// __success __retval(0)
// __naked
#[no_mangle]
pub unsafe extern "C" fn packet_read_for_lwt_xmit() {
    unsafe {
        core::arch::asm!(
            "r2 = *(u32*)(r1 + {__sk_buff_data});",
            "r3 = *(u32*)(r1 + {__sk_buff_data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r0 > r3 goto 0f;",
            "r0 = *(u8*)(r2 + 0);",
            "0:",
            "r0 = 0;",
            "exit;",
            __sk_buff_data = const 0,     // offsetof(struct __sk_buff, data)
            __sk_buff_data_end = const 0, // offsetof(struct __sk_buff, data_end)
            options(noreturn)
        );
    }
}

// SEC("lwt_xmit")
// __description("overlapping checks for direct packet access")
// __success __retval(0)
// __naked
#[no_mangle]
pub unsafe extern "C" fn checks_for_direct_packet_access() {
    unsafe {
        core::arch::asm!(
            "r2 = *(u32*)(r1 + {__sk_buff_data});",
            "r3 = *(u32*)(r1 + {__sk_buff_data_end});",
            "r0 = r2;",
            "r0 += 8;",
            "if r0 > r3 goto 0f;",
            "r1 = r2;",
            "r1 += 6;",
            "if r1 > r3 goto 0f;",
            "r0 = *(u16*)(r2 + 6);",
            "0:",
            "r0 = 0;",
            "exit;",
            __sk_buff_data = const 0,     // offsetof(struct __sk_buff, data)
            __sk_buff_data_end = const 0, // offsetof(struct __sk_buff, data_end)
            options(noreturn)
        );
    }
}

// SEC("lwt_xmit")
// __description("make headroom for LWT_XMIT")
// __success __retval(0)
// __naked
#[no_mangle]
pub unsafe extern "C" fn make_headroom_for_lwt_xmit() {
    unsafe {
        core::arch::asm!(
            "r6 = r1;",
            "r2 = 34;",
            "r3 = 0;",
            "call {bpf_skb_change_head};",
            "/* split for s390 to succeed */",
            "r1 = r6;",
            "r2 = 42;",
            "r3 = 0;",
            "call {bpf_skb_change_head};",
            "r0 = 0;",
            "exit;",
            bpf_skb_change_head = sym bpf_skb_change_head,
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("invalid access of tc_classid for LWT_IN")
// __failure __msg("invalid bpf_context access")
// __failure_unpriv
// __naked
#[no_mangle]
pub unsafe extern "C" fn tc_classid_for_lwt_in() {
    unsafe {
        core::arch::asm!(
            "r0 = *(u32*)(r1 + {__sk_buff_tc_classid});",
            "exit;",
            __sk_buff_tc_classid = const 0, // offsetof(struct __sk_buff, tc_classid)
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("invalid access of tc_classid for LWT_OUT")
// __failure __msg("invalid bpf_context access")
// __failure_unpriv
// __naked
#[no_mangle]
pub unsafe extern "C" fn tc_classid_for_lwt_out() {
    unsafe {
        core::arch::asm!(
            "r0 = *(u32*)(r1 + {__sk_buff_tc_classid});",
            "exit;",
            __sk_buff_tc_classid = const 0, // offsetof(struct __sk_buff, tc_classid)
            options(noreturn)
        );
    }
}

// SEC("socket")
// __description("invalid access of tc_classid for LWT_XMIT")
// __failure __msg("invalid bpf_context access")
// __failure_unpriv
// __naked
#[no_mangle]
pub unsafe extern "C" fn tc_classid_for_lwt_xmit() {
    unsafe {
        core::arch::asm!(
            "r0 = *(u32*)(r1 + {__sk_buff_tc_classid});",
            "exit;",
            __sk_buff_tc_classid = const 0, // offsetof(struct __sk_buff, tc_classid)
            options(noreturn)
        );
    }
}

// SEC("lwt_in")
// __description("check skb->tc_classid half load not permitted for lwt prog")
// __failure __msg("invalid bpf_context access")
// __naked
#[no_mangle]
pub unsafe extern "C" fn not_permitted_for_lwt_prog() {
    unsafe {
        core::arch::asm!(
            "r0 = 0;",
            // C conditional preserved:
            // #if __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
            "r0 = *(u16*)(r1 + {__sk_buff_tc_classid});",
            // #else
            // "r0 = *(u16*)(r1 + {__imm_0});"
            // #endif
            "exit;",
            __imm_0 = const 2,              // offsetof(struct __sk_buff, tc_classid) + 2
            __sk_buff_tc_classid = const 0, // offsetof(struct __sk_buff, tc_classid)
            options(noreturn)
        );
    }
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
