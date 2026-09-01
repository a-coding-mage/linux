// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/direct_packet_access.c */

// C dependencies translated as external Rust dependencies:
// linux/if_ether.h, linux/bpf.h, bpf/bpf_helpers.h, and "bpf_misc.h".

use core::arch::asm;
use core::mem::offset_of;

extern "C" {
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_skb_pull_data(skb: *mut __sk_buff, len: u32) -> i64;
}

// Provided by linux/bpf.h.
#[repr(C)]
pub struct __sk_buff {
    pub len: u32,
    pub pkt_type: u32,
    pub mark: u32,
    pub queue_mapping: u32,
    pub protocol: u32,
    pub vlan_present: u32,
    pub vlan_tci: u32,
    pub vlan_proto: u32,
    pub priority: u32,
    pub ingress_ifindex: u32,
    pub ifindex: u32,
    pub tc_index: u32,
    pub cb: [u32; 5],
    pub hash: u32,
    pub tc_classid: u32,
    pub data: u32,
    pub data_end: u32,
}

pub const ETH_HLEN: u32 = 14;
pub const TEST_DATA_LEN: u32 = 64;
pub const BPF_F_ANY_ALIGNMENT: u32 = 2;
pub const BPF_F_STRICT_ALIGNMENT: u32 = 1;
pub const BPF_F_TEST_STATE_FREQ: u32 = 8;

#[unsafe(no_mangle)]
#[link_section = "tc"]
// __description("pkt_end - pkt_start is allowed")
// __success __retval(TEST_DATA_LEN)
pub unsafe extern "C" fn end_pkt_start_is_allowed() {
    unsafe {
        asm!(
            r#"
            r0 = *(u32*)(r1 + {__sk_buff_data_end});
            r2 = *(u32*)(r1 + {__sk_buff_data});
            r0 -= r2;
            exit;
            "#,
            __sk_buff_data = const offset_of!(__sk_buff, data),
            __sk_buff_data_end = const offset_of!(__sk_buff, data_end),
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[link_section = "tc"]
// __description("direct packet access: test1")
// __success __retval(0)
pub unsafe extern "C" fn direct_packet_access_test1() {
    unsafe {
        asm!(
            r#"
            r2 = *(u32*)(r1 + {__sk_buff_data});
            r3 = *(u32*)(r1 + {__sk_buff_data_end});
            r0 = r2;
            r0 += 8;
            if r0 > r3 goto 0f;
            r0 = *(u8*)(r2 + 0);
        0:
            r0 = 0;
            exit;
            "#,
            __sk_buff_data = const offset_of!(__sk_buff, data),
            __sk_buff_data_end = const offset_of!(__sk_buff, data_end),
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[link_section = "tc"]
// __description("direct packet access: test2")
// __success __retval(0)
pub unsafe extern "C" fn direct_packet_access_test2() {
    unsafe {
        asm!(
            r#"
            r0 = 1;
            r4 = *(u32*)(r1 + {__sk_buff_data_end});
            r3 = *(u32*)(r1 + {__sk_buff_data});
            r5 = r3;
            r5 += 14;
            if r5 > r4 goto 0f;
            r0 = *(u8*)(r3 + 7);
            r4 = *(u8*)(r3 + 12);
            r4 *= 14;
            r3 = *(u32*)(r1 + {__sk_buff_data});
            r3 += r4;
            r2 = *(u32*)(r1 + {__sk_buff_len});
            r2 <<= 49;
            r2 >>= 49;
            r3 += r2;
            r2 = r3;
            r2 += 8;
            r1 = *(u32*)(r1 + {__sk_buff_data_end});
            if r2 > r1 goto 1f;
            r1 = *(u8*)(r3 + 4);
        1:
            r0 = 0;
        0:
            exit;
            "#,
            __sk_buff_data = const offset_of!(__sk_buff, data),
            __sk_buff_data_end = const offset_of!(__sk_buff, data_end),
            __sk_buff_len = const offset_of!(__sk_buff, len),
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[link_section = "socket"]
// __description("direct packet access: test3")
// __failure __msg("invalid bpf_context access off=76")
// __failure_unpriv
pub unsafe extern "C" fn direct_packet_access_test3() {
    unsafe {
        asm!(
            r#"
            r2 = *(u32*)(r1 + {__sk_buff_data});
            r0 = 0;
            exit;
            "#,
            __sk_buff_data = const offset_of!(__sk_buff, data),
            options(noreturn)
        );
    }
}

macro_rules! bpf_asm_test {
    ($(#[$attr:meta])* $name:ident, $section:literal, $asm_body:literal, $($operand:tt)*) => {
        $(#[$attr])*
        #[unsafe(no_mangle)]
        #[link_section = $section]
        pub unsafe extern "C" fn $name() {
            unsafe {
                asm!($asm_body, $($operand)* options(noreturn));
            }
        }
    };
}

bpf_asm_test!(
    direct_packet_access_test4_write,
    "tc",
    r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r0 > r3 goto 0f;
    *(u8*)(r2 + 0) = r2;
0:  r0 = 0;
    exit;
    "#,
    __sk_buff_data = const offset_of!(__sk_buff, data),
    __sk_buff_data_end = const offset_of!(__sk_buff, data_end),
);

bpf_asm_test!(pkt_end_reg_good_access, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r3 >= r0 goto 0f;
    r0 = 1;
    exit;
0:  r0 = *(u8*)(r2 + 0);
    r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(pkt_end_reg_bad_access, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r3 >= r0 goto 0f;
    r0 = *(u8*)(r2 + 0);
    r0 = 1;
    exit;
0:  r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(pkt_end_reg_both_accesses, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r3 >= r0 goto 0f;
    r0 = *(u8*)(r2 + 0);
    r0 = 1;
    exit;
0:  r0 = *(u8*)(r2 + 0);
    r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(test8_double_test_variant_1, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r3 >= r0 goto 0f;
    if r0 > r3 goto 1f;
    r0 = *(u8*)(r2 + 0);
1:  r0 = 1;
    exit;
0:  r0 = *(u8*)(r2 + 0);
    r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(test9_double_test_variant_2, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r3 >= r0 goto 0f;
    r0 = 1;
    exit;
0:  if r0 > r3 goto 1f;
    r0 = *(u8*)(r2 + 0);
1:  r0 = *(u8*)(r2 + 0);
    r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(packet_access_test10_write_invalid, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r0 > r3 goto 0f;
    r0 = 0;
    exit;
0:  *(u8*)(r2 + 0) = r2;
    r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(access_test11_shift_good_access, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 22;
    if r0 > r3 goto 0f;
    r3 = 144;
    r5 = r3;
    r5 += 23;
    r5 >>= 3;
    r6 = r2;
    r6 += r5;
    r0 = 1;
    exit;
0:  r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(access_test12_and_good_access, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 22;
    if r0 > r3 goto 0f;
    r3 = 144;
    r5 = r3;
    r5 += 23;
    r5 &= 15;
    r6 = r2;
    r6 += r5;
    r0 = 1;
    exit;
0:  r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(access_test13_branches_good_access, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 22;
    if r0 > r3 goto 0f;
    r3 = *(u32*)(r1 + {__sk_buff_mark});
    r4 = 1;
    if r3 > r4 goto 1f;
    r3 = 14;
    goto 2f;
1:  r3 = 24;
2:  r5 = r3;
    r5 += 23;
    r5 &= 15;
    r6 = r2;
    r6 += r5;
    r0 = 1;
    exit;
0:  r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end), __sk_buff_mark = const offset_of!(__sk_buff, mark),);

bpf_asm_test!(_0_const_imm_good_access, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 22;
    if r0 > r3 goto 0f;
    r5 = 12;
    r5 >>= 4;
    r6 = r2;
    r6 += r5;
    r0 = *(u8*)(r6 + 0);
    r0 = 1;
    exit;
0:  r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(access_test15_spill_with_xadd, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r0 > r3 goto 0f;
    r5 = 4096;
    r4 = r10;
    r4 += -8;
    *(u64*)(r4 + 0) = r2;
    lock *(u64 *)(r4 + 0) += r5;
    r2 = *(u64*)(r4 + 0);
    *(u32*)(r2 + 0) = r5;
    r0 = 0;
0:  exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(test16_arith_on_data_end, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    r3 += 16;
    if r0 > r3 goto 0f;
    *(u8*)(r2 + 0) = r2;
0:  r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(packet_access_test17_pruning_alignment, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r7 = *(u32*)(r1 + {__sk_buff_mark});
    r0 = r2;
    r0 += 14;
    if r7 > 1 goto 0f;
2:  if r0 > r3 goto 1f;
    *(u32*)(r0 - 4) = r0;
1:  r0 = 0;
    exit;
0:  r0 += 1;
    goto 2b;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end), __sk_buff_mark = const offset_of!(__sk_buff, mark),);

bpf_asm_test!(test18_imm_pkt_ptr_1, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = 8;
    r0 += r2;
    if r0 > r3 goto 0f;
    *(u8*)(r2 + 0) = r2;
0:  r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(test19_imm_pkt_ptr_2, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r0 > r3 goto 0f;
    r4 = 4;
    r4 += r2;
    *(u8*)(r4 + 0) = r4;
0:  r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(test20_x_pkt_ptr_1, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = 0xffffffff;
    *(u64*)(r10 - 8) = r0;
    r0 = *(u64*)(r10 - 8);
    r0 &= 0x7fff;
    r4 = r0;
    r4 += r2;
    r5 = r4;
    r4 += {__imm_0};
    if r4 > r3 goto 0f;
    *(u64*)(r5 + 0) = r4;
0:  r0 = 0;
    exit;
    "#, __imm_0 = const 0x7fff - 1, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(test21_x_pkt_ptr_2, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r0 > r3 goto 0f;
    r4 = 0xffffffff;
    *(u64*)(r10 - 8) = r4;
    r4 = *(u64*)(r10 - 8);
    r4 &= 0x7fff;
    r4 += r2;
    r5 = r4;
    r4 += {__imm_0};
    if r4 > r3 goto 0f;
    *(u64*)(r5 + 0) = r4;
0:  r0 = 0;
    exit;
    "#, __imm_0 = const 0x7fff - 1, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(test22_x_pkt_ptr_3, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    *(u64*)(r10 - 8) = r2;
    *(u64*)(r10 - 16) = r3;
    r3 = *(u64*)(r10 - 16);
    if r0 > r3 goto 0f;
    r2 = *(u64*)(r10 - 8);
    r4 = 0xffffffff;
    lock *(u64 *)(r10 - 8) += r4;
    r4 = *(u64*)(r10 - 8);
    r4 >>= 49;
    r4 += r2;
    r0 = r4;
    r0 += 2;
    if r0 > r3 goto 0f;
    r2 = 1;
    *(u16*)(r4 + 0) = r2;
0:  r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(test23_x_pkt_ptr_4, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = *(u32*)(r1 + {__sk_buff_mark});
    *(u64*)(r10 - 8) = r0;
    r0 = *(u64*)(r10 - 8);
    r0 &= 0xffff;
    r4 = r0;
    r0 = 31;
    r0 += r4;
    r0 += r2;
    r5 = r0;
    r0 += {__imm_0};
    if r0 > r3 goto 0f;
    *(u64*)(r5 + 0) = r0;
0:  r0 = 0;
    exit;
    "#, __imm_0 = const 0xffff - 1, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end), __sk_buff_mark = const offset_of!(__sk_buff, mark),);

bpf_asm_test!(test24_x_pkt_ptr_5, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = 0xffffffff;
    *(u64*)(r10 - 8) = r0;
    r0 = *(u64*)(r10 - 8);
    r0 &= 0xff;
    r4 = r0;
    r0 = 64;
    r0 += r4;
    r0 += r2;
    r5 = r0;
    r0 += {__imm_0};
    if r0 > r3 goto 0f;
    *(u64*)(r5 + 0) = r0;
0:  r0 = 0;
    exit;
    "#, __imm_0 = const 0x7fff - 1, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(test25_marking_on_good_access, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r0 < r3 goto 0f;
1:  r0 = 0;
    exit;
0:  r0 = *(u8*)(r2 + 0);
    goto 1b;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(test26_marking_on_bad_access, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r0 < r3 goto 0f;
    r0 = *(u8*)(r2 + 0);
1:  r0 = 0;
    exit;
0:  goto 1b;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(test27_marking_on_good_access, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r3 <= r0 goto 0f;
    r0 = *(u8*)(r2 + 0);
0:  r0 = 1;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(test28_marking_on_bad_access, "tc", r#"
    r2 = *(u32*)(r1 + {__sk_buff_data});
    r3 = *(u32*)(r1 + {__sk_buff_data_end});
    r0 = r2;
    r0 += 8;
    if r3 <= r0 goto 0f;
1:  r0 = 1;
    exit;
0:  r0 = *(u8*)(r2 + 0);
    goto 1b;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(reg_pkt_end_in_subprog, "tc", r#"
    r6 = *(u32*)(r1 + {__sk_buff_data});
    r2 = *(u32*)(r1 + {__sk_buff_data_end});
    r3 = r6;
    r3 += 8;
    call reg_pkt_end_in_subprog__1;
    if r0 == 0 goto 0f;
    r0 = *(u8*)(r6 + 0);
0:  r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

#[unsafe(no_mangle)]
#[inline(never)]
unsafe extern "C" fn reg_pkt_end_in_subprog__1() {
    unsafe {
        asm!(
            r#"
            r0 = 0;
            if r3 > r2 goto 0f;
            r0 = 1;
        0:  exit;
            "#,
            options(noreturn)
        );
    }
}

bpf_asm_test!(id_in_regsafe_bad_access, "tc", r#"
    /* r9 = ctx */
    r9 = r1;
    /* r7 = ktime_get_ns() */
    call {bpf_ktime_get_ns};
    r7 = r0;
    /* r6 = ktime_get_ns() */
    call {bpf_ktime_get_ns};
    r6 = r0;
    /* r2 = ctx->data
     * r3 = ctx->data
     * r4 = ctx->data_end
     */
    r2 = *(u32*)(r9 + {__sk_buff_data});
    r3 = *(u32*)(r9 + {__sk_buff_data});
    r4 = *(u32*)(r9 + {__sk_buff_data_end});
    /* if r6 > 100 goto exit
     * if r7 > 100 goto exit
     */
    if r6 > 100 goto 0f;
    if r7 > 100 goto 0f;
    /* r2 += r6              ; this forces assignment of ID to r2
     * r2 += 1               ; get some fixed off for r2
     * r3 += r7              ; this forces assignment of ID to r3
     * r3 += 1               ; get some fixed off for r3
     */
    r2 += r6;
    r2 += 1;
    r3 += r7;
    r3 += 1;
    /* if r6 > r7 goto +1    ; no new information about the state is derived from
     *                       ; this check, thus produced verifier states differ
     *                       ; only in 'insn_idx'
     * r2 = r3               ; optionally share ID between r2 and r3
     */
    if r6 != r7 goto 1f;
    r2 = r3;
1:  /* if r3 > ctx->data_end goto exit */
    if r3 > r4 goto 0f;
    /* r5 = *(u8 *) (r2 - 1) ; access packet memory using r2,
     *                       ; this is not always safe
     */
    r5 = *(u8*)(r2 - 1);
0:  /* exit(0) */
    r0 = 0;
    exit;
    "#, bpf_ktime_get_ns = sym bpf_ktime_get_ns, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

macro_rules! access_test_non_linear {
    ($name:ident, $section:literal, $desc:literal, $retval:expr, $linear_sz:expr, $off:expr) => {
        #[unsafe(no_mangle)]
        #[link_section = $section]
        pub unsafe extern "C" fn $name() {
            unsafe {
                asm!(
                    r#"
                    r2 = *(u32*)(r1 + {skb_data});
                    r3 = *(u32*)(r1 + {skb_data_end});
                    r0 = r2;
                    r0 += {offset};
                    if r0 > r3 goto 0f;
                    r0 = *(u8*)(r0 - 1);
                    r0 = 0;
                    exit;
                0:  r0 = 1;
                    exit;
                    "#,
                    skb_data = const offset_of!(__sk_buff, data),
                    skb_data_end = const offset_of!(__sk_buff, data_end),
                    offset = const $off,
                    options(noreturn)
                );
            }
        }
    };
}

access_test_non_linear!(access_non_linear_test31, "tc", "too short eth", 1, ETH_HLEN, 22);
access_test_non_linear!(access_non_linear_test32, "tc", "too short 1", 1, 1, 22);
access_test_non_linear!(access_non_linear_test33, "tc", "long enough", 0, 22, 22);
access_test_non_linear!(access_non_linear_test34, "cgroup_skb/ingress", "too short eth", 1, ETH_HLEN, 8);
access_test_non_linear!(access_non_linear_test35, "cgroup_skb/ingress", "too short 1", 1, 1, 8);
access_test_non_linear!(access_non_linear_test36, "cgroup_skb/ingress", "long enough", 0, 22, 8);

bpf_asm_test!(access_non_linear_linearized, "tc", r#"
    r6 = r1;
    r2 = 22;
    call {bpf_skb_pull_data};
    r2 = *(u32*)(r6 + {skb_data});
    r3 = *(u32*)(r6 + {skb_data_end});
    r0 = r2;
    r0 += 22;
    if r0 > r3 goto 0f;
    r0 = *(u8*)(r0 - 1);
    exit;
0:  r0 = 1;
    exit;
    "#, bpf_skb_pull_data = sym bpf_skb_pull_data, skb_data = const offset_of!(__sk_buff, data), skb_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(pkt_range_clear_after_sub, "tc", r#"
    r9 = *(u32*)(r1 + {__sk_buff_data});
    r8 = *(u32*)(r1 + {__sk_buff_data_end});
    r9 += 256;
    if r9 >= r8 goto 0f;
    r0 = 0;
    exit;
0:  /* r9 has AT_PKT_END (pkt + 256 >= pkt_end) */
    r9 -= 256;
    /*
     * AT_PKT_END must not survive the arithmetic.
     * is_pkt_ptr_branch_taken must validate both
     * branches when visiting the next condition.
     */
    if r9 < r8 goto 1f;
    r0 = 0;
    exit;
1:  r0 = *(u8*)(r9 + 0);
    r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

bpf_asm_test!(pkt_range_clear_after_add, "tc", r#"
    r9 = *(u32*)(r1 + {__sk_buff_data});
    r8 = *(u32*)(r1 + {__sk_buff_data_end});
    r9 += 256;
    if r9 >= r8 goto 0f;
    r0 = 0;
    exit;
0:  /* r9 has AT_PKT_END (pkt + 256 >= pkt_end) */
    r9 += -256;
    /*
     * Same as sub, but goes through BPF_ADD path.
     * AT_PKT_END must not survive the arithmetic.
     */
    if r9 < r8 goto 1f;
    r0 = 0;
    exit;
1:  r0 = *(u8*)(r9 + 0);
    r0 = 0;
    exit;
    "#, __sk_buff_data = const offset_of!(__sk_buff, data), __sk_buff_data_end = const offset_of!(__sk_buff, data_end),);

#[unsafe(no_mangle)]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
