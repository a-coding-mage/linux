// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Google LLC. */

// C includes translated as dependency intent:
// <linux/bpf.h>
// <bpf/bpf_helpers.h>
// "../../../include/linux/filter.h"
// "bpf_misc.h"

// Original C conditional: #ifdef CAN_USE_LOAD_ACQ_STORE_REL
#[cfg(CAN_USE_LOAD_ACQ_STORE_REL)]
mod can_use_load_acq_store_rel {
    use core::arch::asm;

    // The BPF verifier test metadata macros (__description, __success,
    // __failure, __retval, __msg, __flag, __naked, etc.) are supplied by the
    // BPF selftest harness in C. They are preserved below as comments on the
    // translated Rust items.

    #[link_section = "socket"]
    // __description("store-release, 8-bit")
    // __success __success_unpriv __retval(0)
    #[no_mangle]
    pub unsafe extern "C" fn store_release_8() {
        unsafe {
            asm!(
                "r0 = 0;",
                "w1 = 0x12;",
                ".8byte {store_release_insn}", // store_release((u8 *)(r10 - 1), w1);
                "w2 = *(u8 *)(r10 - 1);",
                "if r2 == r1 goto 1f;",
                "r0 = 1;",
                "1:",
                "exit;",
                store_release_insn = const BPF_ATOMIC_OP(BPF_B, BPF_STORE_REL, BPF_REG_10, BPF_REG_1, -1),
                options(noreturn)
            );
        }
    }

    #[link_section = "socket"]
    // __description("store-release, 16-bit")
    // __success __success_unpriv __retval(0)
    #[no_mangle]
    pub unsafe extern "C" fn store_release_16() {
        unsafe {
            asm!(
                "r0 = 0;",
                "w1 = 0x1234;",
                ".8byte {store_release_insn}", // store_release((u16 *)(r10 - 2), w1);
                "w2 = *(u16 *)(r10 - 2);",
                "if r2 == r1 goto 1f;",
                "r0 = 1;",
                "1:",
                "exit;",
                store_release_insn = const BPF_ATOMIC_OP(BPF_H, BPF_STORE_REL, BPF_REG_10, BPF_REG_1, -2),
                options(noreturn)
            );
        }
    }

    #[link_section = "socket"]
    // __description("store-release, 32-bit")
    // __success __success_unpriv __retval(0)
    #[no_mangle]
    pub unsafe extern "C" fn store_release_32() {
        unsafe {
            asm!(
                "r0 = 0;",
                "w1 = 0x12345678;",
                ".8byte {store_release_insn}", // store_release((u32 *)(r10 - 4), w1);
                "w2 = *(u32 *)(r10 - 4);",
                "if r2 == r1 goto 1f;",
                "r0 = 1;",
                "1:",
                "exit;",
                store_release_insn = const BPF_ATOMIC_OP(BPF_W, BPF_STORE_REL, BPF_REG_10, BPF_REG_1, -4),
                options(noreturn)
            );
        }
    }

    #[link_section = "socket"]
    // __description("store-release, 64-bit")
    // __success __success_unpriv __retval(0)
    #[no_mangle]
    pub unsafe extern "C" fn store_release_64() {
        unsafe {
            asm!(
                "r0 = 0;",
                "r1 = 0x1234567890abcdef ll;",
                ".8byte {store_release_insn}", // store_release((u64 *)(r10 - 8), r1);
                "r2 = *(u64 *)(r10 - 8);",
                "if r2 == r1 goto 1f;",
                "r0 = 1;",
                "1:",
                "exit;",
                store_release_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_STORE_REL, BPF_REG_10, BPF_REG_1, -8),
                options(noreturn)
            );
        }
    }

    #[link_section = "socket"]
    // __description("store-release with uninitialized src_reg")
    // __failure __failure_unpriv __msg("R2 !read_ok")
    #[no_mangle]
    pub unsafe extern "C" fn store_release_with_uninitialized_src_reg() {
        unsafe {
            asm!(
                ".8byte {store_release_insn}", // store_release((u64 *)(r10 - 8), r2);
                "exit;",
                store_release_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_STORE_REL, BPF_REG_10, BPF_REG_2, -8),
                options(noreturn)
            );
        }
    }

    #[link_section = "socket"]
    // __description("store-release with uninitialized dst_reg")
    // __failure __failure_unpriv __msg("R2 !read_ok")
    #[no_mangle]
    pub unsafe extern "C" fn store_release_with_uninitialized_dst_reg() {
        unsafe {
            asm!(
                "r1 = 0;",
                ".8byte {store_release_insn}", // store_release((u64 *)(r2 - 8), r1);
                "exit;",
                store_release_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_STORE_REL, BPF_REG_2, BPF_REG_1, -8),
                options(noreturn)
            );
        }
    }

    #[link_section = "socket"]
    // __description("store-release with non-pointer dst_reg")
    // __failure __failure_unpriv __msg("R1 invalid mem access 'scalar'")
    #[no_mangle]
    pub unsafe extern "C" fn store_release_with_non_pointer_dst_reg() {
        unsafe {
            asm!(
                "r1 = 0;",
                ".8byte {store_release_insn}", // store_release((u64 *)(r1 + 0), r1);
                "exit;",
                store_release_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_STORE_REL, BPF_REG_1, BPF_REG_1, 0),
                options(noreturn)
            );
        }
    }

    #[link_section = "socket"]
    // __description("misaligned store-release")
    // __failure __failure_unpriv __msg("misaligned stack access off")
    // __flag(BPF_F_ANY_ALIGNMENT)
    #[no_mangle]
    pub unsafe extern "C" fn store_release_misaligned() {
        unsafe {
            asm!(
                "w0 = 0;",
                ".8byte {store_release_insn}", // store_release((u32 *)(r10 - 5), w0);
                "exit;",
                store_release_insn = const BPF_ATOMIC_OP(BPF_W, BPF_STORE_REL, BPF_REG_10, BPF_REG_0, -5),
                options(noreturn)
            );
        }
    }

    #[link_section = "socket"]
    // __description("store-release to ctx pointer")
    // __failure __failure_unpriv __msg("BPF_ATOMIC stores into R1 ctx is not allowed")
    #[no_mangle]
    pub unsafe extern "C" fn store_release_to_ctx_pointer() {
        unsafe {
            asm!(
                "w0 = 0;",
                // store_release((u8 *)(r1 + offsetof(struct __sk_buff, cb[0])), w0);
                ".8byte {store_release_insn}",
                "exit;",
                store_release_insn = const BPF_ATOMIC_OP(BPF_B, BPF_STORE_REL, BPF_REG_1, BPF_REG_0, offset_of!(__sk_buff, cb[0])),
                options(noreturn)
            );
        }
    }

    #[link_section = "xdp"]
    // __description("store-release to pkt pointer")
    // __failure __msg("BPF_ATOMIC stores into R2 pkt is not allowed")
    #[no_mangle]
    pub unsafe extern "C" fn store_release_to_pkt_pointer() {
        unsafe {
            asm!(
                "w0 = 0;",
                "r2 = *(u32 *)(r1 + {xdp_md_data});",
                "r3 = *(u32 *)(r1 + {xdp_md_data_end});",
                "r1 = r2;",
                "r1 += 8;",
                "if r1 >= r3 goto l0_{id};",
                ".8byte {store_release_insn}", // store_release((u8 *)(r2 + 0), w0);
                "l0_{id}:  r0 = 0;",
                "exit;",
                xdp_md_data = const offset_of!(xdp_md, data),
                xdp_md_data_end = const offset_of!(xdp_md, data_end),
                store_release_insn = const BPF_ATOMIC_OP(BPF_B, BPF_STORE_REL, BPF_REG_2, BPF_REG_0, 0),
                id = const 0,
                options(noreturn)
            );
        }
    }

    #[link_section = "flow_dissector"]
    // __description("store-release to flow_keys pointer")
    // __failure __msg("BPF_ATOMIC stores into R2 flow_keys is not allowed")
    #[no_mangle]
    pub unsafe extern "C" fn store_release_to_flow_keys_pointer() {
        unsafe {
            asm!(
                "w0 = 0;",
                "r2 = *(u64 *)(r1 + {__sk_buff_flow_keys});",
                ".8byte {store_release_insn}", // store_release((u8 *)(r2 + 0), w0);
                "exit;",
                __sk_buff_flow_keys = const offset_of!(__sk_buff, flow_keys),
                store_release_insn = const BPF_ATOMIC_OP(BPF_B, BPF_STORE_REL, BPF_REG_2, BPF_REG_0, 0),
                options(noreturn)
            );
        }
    }

    #[link_section = "sk_reuseport"]
    // __description("store-release to sock pointer")
    // __failure __msg("R2 cannot write into sock")
    #[no_mangle]
    pub unsafe extern "C" fn store_release_to_sock_pointer() {
        unsafe {
            asm!(
                "w0 = 0;",
                "r2 = *(u64 *)(r1 + {sk_reuseport_md_sk});",
                ".8byte {store_release_insn}", // store_release((u8 *)(r2 + 0), w0);
                "exit;",
                sk_reuseport_md_sk = const offset_of!(sk_reuseport_md, sk),
                store_release_insn = const BPF_ATOMIC_OP(BPF_B, BPF_STORE_REL, BPF_REG_2, BPF_REG_0, 0),
                options(noreturn)
            );
        }
    }

    #[link_section = "socket"]
    // __description("store-release, leak pointer to stack")
    // __success __success_unpriv __retval(0)
    #[no_mangle]
    pub unsafe extern "C" fn store_release_leak_pointer_to_stack() {
        unsafe {
            asm!(
                ".8byte {store_release_insn}", // store_release((u64 *)(r10 - 8), r1);
                "r0 = 0;",
                "exit;",
                store_release_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_STORE_REL, BPF_REG_10, BPF_REG_1, -8),
                options(noreturn)
            );
        }
    }

    #[repr(C)]
    pub struct MapHash8b {
        // __uint(type, BPF_MAP_TYPE_HASH);
        pub type_: u32,
        // __uint(max_entries, 1);
        pub max_entries: u32,
        // __type(key, long long);
        pub key: *mut i64,
        // __type(value, long long);
        pub value: *mut i64,
    }

    #[link_section = ".maps"]
    #[no_mangle]
    pub static mut map_hash_8b: MapHash8b = MapHash8b {
        type_: BPF_MAP_TYPE_HASH,
        max_entries: 1,
        key: core::ptr::null_mut(),
        value: core::ptr::null_mut(),
    };

    unsafe extern "C" {
        fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    }

    #[link_section = "socket"]
    // __description("store-release, leak pointer to map")
    // __success __retval(0)
    // __failure_unpriv __msg_unpriv("R6 leaks addr into map")
    #[no_mangle]
    pub unsafe extern "C" fn store_release_leak_pointer_to_map() {
        unsafe {
            asm!(
                "r6 = r1;",
                "r1 = {map_hash_8b} ll;",
                "r2 = 0;",
                "*(u64 *)(r10 - 8) = r2;",
                "r2 = r10;",
                "r2 += -8;",
                "call {bpf_map_lookup_elem};",
                "if r0 == 0 goto l0_{id};",
                ".8byte {store_release_insn}", // store_release((u64 *)(r0 + 0), r6);
                "l0_{id}:",
                "r0 = 0;",
                "exit;",
                map_hash_8b = sym map_hash_8b,
                bpf_map_lookup_elem = sym bpf_map_lookup_elem,
                store_release_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_STORE_REL, BPF_REG_0, BPF_REG_6, 0),
                id = const 0,
                options(noreturn)
            );
        }
    }

    #[link_section = "socket"]
    // __description("store-release with invalid register R15")
    // __failure __failure_unpriv __msg("R15 is invalid")
    #[no_mangle]
    pub unsafe extern "C" fn store_release_with_invalid_reg() {
        unsafe {
            asm!(
                ".8byte {store_release_insn}", // store_release((u64 *)(r15 + 0), r1);
                "exit;",
                store_release_insn = const BPF_ATOMIC_OP(BPF_DW, BPF_STORE_REL, 15 /* invalid reg */, BPF_REG_1, 0),
                options(noreturn)
            );
        }
    }
}

// Original C conditional: #else /* CAN_USE_LOAD_ACQ_STORE_REL */
#[cfg(not(CAN_USE_LOAD_ACQ_STORE_REL))]
#[link_section = "socket"]
// __description("Clang version < 18, ENABLE_ATOMICS_TESTS not defined, and/or JIT doesn't support store-release, use a dummy test")
// __success
#[no_mangle]
pub extern "C" fn dummy_test() -> i32 {
    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
