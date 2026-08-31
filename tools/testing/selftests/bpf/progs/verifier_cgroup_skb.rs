// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/cgroup_skb.c */

/* Dependencies from the original C file:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 */

use core::arch::asm;

extern "C" {
    type __sk_buff;
}

macro_rules! offsetof {
    ($ty:ty, $field:tt) => {
        ::core::mem::offset_of!($ty, $field)
    };
}

macro_rules! offsetof_index {
    ($ty:ty, $field:tt, $idx:expr) => {
        (::core::mem::offset_of!($ty, $field) + $idx * ::core::mem::size_of::<u32>())
    };
}

// SEC("cgroup/skb")
// __description("direct packet read test#1 for CGROUP_SKB")
// __success __failure_unpriv
// __msg_unpriv("invalid bpf_context access off=76 size=4")
// __retval(0)
#[naked]
pub unsafe extern "C" fn test_1_for_cgroup_skb() {
    asm!(
        "r2 = *(u32*)(r1 + {__sk_buff_data})",
        "r3 = *(u32*)(r1 + {__sk_buff_data_end})",
        "r4 = *(u32*)(r1 + {__sk_buff_len})",
        "r5 = *(u32*)(r1 + {__sk_buff_pkt_type})",
        "r6 = *(u32*)(r1 + {__sk_buff_mark})",
        "*(u32*)(r1 + {__sk_buff_mark}) = r6",
        "r7 = *(u32*)(r1 + {__sk_buff_queue_mapping})",
        "r8 = *(u32*)(r1 + {__sk_buff_protocol})",
        "r9 = *(u32*)(r1 + {__sk_buff_vlan_present})",
        "r0 = r2",
        "r0 += 8",
        "if r0 > r3 goto 0f",
        "r0 = *(u8*)(r2 + 0)",
        "0:",
        "r0 = 0",
        "exit",
        __sk_buff_data = const offsetof!(__sk_buff, data),
        __sk_buff_data_end = const offsetof!(__sk_buff, data_end),
        __sk_buff_len = const offsetof!(__sk_buff, len),
        __sk_buff_mark = const offsetof!(__sk_buff, mark),
        __sk_buff_pkt_type = const offsetof!(__sk_buff, pkt_type),
        __sk_buff_protocol = const offsetof!(__sk_buff, protocol),
        __sk_buff_queue_mapping = const offsetof!(__sk_buff, queue_mapping),
        __sk_buff_vlan_present = const offsetof!(__sk_buff, vlan_present),
        options(noreturn)
    );
}

// SEC("cgroup/skb")
// __description("direct packet read test#2 for CGROUP_SKB")
// __success __success_unpriv __retval(0)
#[naked]
pub unsafe extern "C" fn test_2_for_cgroup_skb() {
    asm!(
        "r4 = *(u32*)(r1 + {__sk_buff_vlan_tci})",
        "r5 = *(u32*)(r1 + {__sk_buff_vlan_proto})",
        "r6 = *(u32*)(r1 + {__sk_buff_priority})",
        "*(u32*)(r1 + {__sk_buff_priority}) = r6",
        "r7 = *(u32*)(r1 + {__sk_buff_ingress_ifindex})",
        "r8 = *(u32*)(r1 + {__sk_buff_tc_index})",
        "r9 = *(u32*)(r1 + {__sk_buff_hash})",
        "r0 = 0",
        "exit",
        __sk_buff_hash = const offsetof!(__sk_buff, hash),
        __sk_buff_ingress_ifindex = const offsetof!(__sk_buff, ingress_ifindex),
        __sk_buff_priority = const offsetof!(__sk_buff, priority),
        __sk_buff_tc_index = const offsetof!(__sk_buff, tc_index),
        __sk_buff_vlan_proto = const offsetof!(__sk_buff, vlan_proto),
        __sk_buff_vlan_tci = const offsetof!(__sk_buff, vlan_tci),
        options(noreturn)
    );
}

// SEC("cgroup/skb")
// __description("direct packet read test#3 for CGROUP_SKB")
// __success __success_unpriv __retval(0)
#[naked]
pub unsafe extern "C" fn test_3_for_cgroup_skb() {
    asm!(
        "r4 = *(u32*)(r1 + {__sk_buff_cb_0})",
        "r5 = *(u32*)(r1 + {__sk_buff_cb_1})",
        "r6 = *(u32*)(r1 + {__sk_buff_cb_2})",
        "r7 = *(u32*)(r1 + {__sk_buff_cb_3})",
        "r8 = *(u32*)(r1 + {__sk_buff_cb_4})",
        "r9 = *(u32*)(r1 + {__sk_buff_napi_id})",
        "*(u32*)(r1 + {__sk_buff_cb_0}) = r4",
        "*(u32*)(r1 + {__sk_buff_cb_1}) = r5",
        "*(u32*)(r1 + {__sk_buff_cb_2}) = r6",
        "*(u32*)(r1 + {__sk_buff_cb_3}) = r7",
        "*(u32*)(r1 + {__sk_buff_cb_4}) = r8",
        "r0 = 0",
        "exit",
        __sk_buff_cb_0 = const offsetof_index!(__sk_buff, cb, 0),
        __sk_buff_cb_1 = const offsetof_index!(__sk_buff, cb, 1),
        __sk_buff_cb_2 = const offsetof_index!(__sk_buff, cb, 2),
        __sk_buff_cb_3 = const offsetof_index!(__sk_buff, cb, 3),
        __sk_buff_cb_4 = const offsetof_index!(__sk_buff, cb, 4),
        __sk_buff_napi_id = const offsetof!(__sk_buff, napi_id),
        options(noreturn)
    );
}

// SEC("cgroup/skb")
// __description("direct packet read test#4 for CGROUP_SKB")
// __success __success_unpriv __retval(0)
#[naked]
pub unsafe extern "C" fn test_4_for_cgroup_skb() {
    asm!(
        "r2 = *(u32*)(r1 + {__sk_buff_family})",
        "r3 = *(u32*)(r1 + {__sk_buff_remote_ip4})",
        "r4 = *(u32*)(r1 + {__sk_buff_local_ip4})",
        "r5 = *(u32*)(r1 + {__sk_buff_remote_ip6_0})",
        "r5 = *(u32*)(r1 + {__sk_buff_remote_ip6_1})",
        "r5 = *(u32*)(r1 + {__sk_buff_remote_ip6_2})",
        "r5 = *(u32*)(r1 + {__sk_buff_remote_ip6_3})",
        "r6 = *(u32*)(r1 + {__sk_buff_local_ip6_0})",
        "r6 = *(u32*)(r1 + {__sk_buff_local_ip6_1})",
        "r6 = *(u32*)(r1 + {__sk_buff_local_ip6_2})",
        "r6 = *(u32*)(r1 + {__sk_buff_local_ip6_3})",
        "r7 = *(u32*)(r1 + {__sk_buff_remote_port})",
        "r8 = *(u32*)(r1 + {__sk_buff_local_port})",
        "r0 = 0",
        "exit",
        __sk_buff_family = const offsetof!(__sk_buff, family),
        __sk_buff_local_ip4 = const offsetof!(__sk_buff, local_ip4),
        __sk_buff_local_ip6_0 = const offsetof_index!(__sk_buff, local_ip6, 0),
        __sk_buff_local_ip6_1 = const offsetof_index!(__sk_buff, local_ip6, 1),
        __sk_buff_local_ip6_2 = const offsetof_index!(__sk_buff, local_ip6, 2),
        __sk_buff_local_ip6_3 = const offsetof_index!(__sk_buff, local_ip6, 3),
        __sk_buff_local_port = const offsetof!(__sk_buff, local_port),
        __sk_buff_remote_ip4 = const offsetof!(__sk_buff, remote_ip4),
        __sk_buff_remote_ip6_0 = const offsetof_index!(__sk_buff, remote_ip6, 0),
        __sk_buff_remote_ip6_1 = const offsetof_index!(__sk_buff, remote_ip6, 1),
        __sk_buff_remote_ip6_2 = const offsetof_index!(__sk_buff, remote_ip6, 2),
        __sk_buff_remote_ip6_3 = const offsetof_index!(__sk_buff, remote_ip6, 3),
        __sk_buff_remote_port = const offsetof!(__sk_buff, remote_port),
        options(noreturn)
    );
}

// SEC("cgroup/skb")
// __description("invalid access of tc_classid for CGROUP_SKB")
// __failure __msg("invalid bpf_context access")
// __failure_unpriv
#[naked]
pub unsafe extern "C" fn tc_classid_for_cgroup_skb() {
    asm!(
        "r0 = *(u32*)(r1 + {__sk_buff_tc_classid})",
        "r0 = 0",
        "exit",
        __sk_buff_tc_classid = const offsetof!(__sk_buff, tc_classid),
        options(noreturn)
    );
}

// SEC("cgroup/skb")
// __description("invalid access of data_meta for CGROUP_SKB")
// __failure __msg("invalid bpf_context access")
// __failure_unpriv
#[naked]
pub unsafe extern "C" fn data_meta_for_cgroup_skb() {
    asm!(
        "r0 = *(u32*)(r1 + {__sk_buff_data_meta})",
        "r0 = 0",
        "exit",
        __sk_buff_data_meta = const offsetof!(__sk_buff, data_meta),
        options(noreturn)
    );
}

// SEC("cgroup/skb")
// __description("invalid access of flow_keys for CGROUP_SKB")
// __failure __msg("invalid bpf_context access")
// __failure_unpriv
#[naked]
pub unsafe extern "C" fn flow_keys_for_cgroup_skb() {
    asm!(
        "r0 = *(u32*)(r1 + {__sk_buff_flow_keys})",
        "r0 = 0",
        "exit",
        __sk_buff_flow_keys = const offsetof!(__sk_buff, flow_keys),
        options(noreturn)
    );
}

// SEC("cgroup/skb")
// __description("invalid write access to napi_id for CGROUP_SKB")
// __failure __msg("invalid bpf_context access")
// __failure_unpriv
#[naked]
pub unsafe extern "C" fn napi_id_for_cgroup_skb() {
    asm!(
        "r9 = *(u32*)(r1 + {__sk_buff_napi_id})",
        "*(u32*)(r1 + {__sk_buff_napi_id}) = r9",
        "r0 = 0",
        "exit",
        __sk_buff_napi_id = const offsetof!(__sk_buff, napi_id),
        options(noreturn)
    );
}

// SEC("cgroup/skb")
// __description("write tstamp from CGROUP_SKB")
// __success __failure_unpriv
// __msg_unpriv("invalid bpf_context access off=152 size=8")
// __retval(0)
#[naked]
pub unsafe extern "C" fn write_tstamp_from_cgroup_skb() {
    asm!(
        "r0 = 0",
        "*(u64*)(r1 + {__sk_buff_tstamp}) = r0",
        "r0 = 0",
        "exit",
        __sk_buff_tstamp = const offsetof!(__sk_buff, tstamp),
        options(noreturn)
    );
}

// SEC("cgroup/skb")
// __description("read tstamp from CGROUP_SKB")
// __success __success_unpriv __retval(0)
#[naked]
pub unsafe extern "C" fn read_tstamp_from_cgroup_skb() {
    asm!(
        "r0 = *(u64*)(r1 + {__sk_buff_tstamp})",
        "r0 = 0",
        "exit",
        __sk_buff_tstamp = const offsetof!(__sk_buff, tstamp),
        options(noreturn)
    );
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
