// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2018 Facebook */

/*
 * Translated from C. The original included:
 * <linux/in.h>, <linux/ip.h>, <linux/ipv6.h>, <linux/tcp.h>,
 * <linux/udp.h>, <linux/bpf.h>, <linux/types.h>, <linux/if_ether.h>,
 * <bpf/bpf_endian.h>, <bpf/bpf_helpers.h>, and
 * "test_select_reuseport_common.h".
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;

const BPF_MAP_TYPE_ARRAY_OF_MAPS: __u32 = 12;
const BPF_MAP_TYPE_ARRAY: __u32 = 2;
const BPF_HDR_START_NET: __u32 = 1;
const BPF_ANY: __u64 = 0;
const ETH_P_IP: __u16 = 0x0800;
const IPPROTO_TCP: __u8 = 6;
const IPPROTO_UDP: __u8 = 17;
const SK_DROP: i32 = 0;
const SK_PASS: i32 = 1;

type __u64 = u64;

#[repr(C)]
pub struct sk_reuseport_md {
    pub data: *mut core::ffi::c_void,
    pub data_end: *mut core::ffi::c_void,
    pub len: __u32,
    pub eth_protocol: __u32,
    pub ip_protocol: __u32,
    pub bind_inany: __u32,
    pub hash: __u32,
}

#[repr(C)]
pub struct iphdr {
    pub ihl_version: __u8,
    pub tos: __u8,
    pub tot_len: __u16,
    pub id: __u16,
    pub frag_off: __u16,
    pub ttl: __u8,
    pub protocol: __u8,
    pub check: __u16,
    pub saddr: __u32,
    pub daddr: __u32,
}

#[repr(C)]
pub struct ipv6hdr {
    pub priority_version: __u8,
    pub flow_lbl: [__u8; 3],
    pub payload_len: __u16,
    pub nexthdr: __u8,
    pub hop_limit: __u8,
    pub saddr: [__u8; 16],
    pub daddr: [__u8; 16],
}

#[repr(C)]
pub struct tcphdr {
    pub source: __u16,
    pub dest: __u16,
    pub seq: __u32,
    pub ack_seq: __u32,
    pub doff_res_flags: __u16,
    pub window: __u16,
    pub check: __u16,
    pub urg_ptr: __u16,
}

impl tcphdr {
    unsafe fn doff(&self) -> __u16 {
        (self.doff_res_flags >> 12) & 0x0f
    }

    unsafe fn fin(&self) -> bool {
        (self.doff_res_flags & 0x0001) != 0
    }
}

#[repr(C)]
pub struct udphdr {
    pub source: __u16,
    pub dest: __u16,
    pub len: __u16,
    pub check: __u16,
}

#[repr(C)]
pub struct cmd {
    pub reuseport_index: __u32,
    pub pass_on_failure: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_check {
    pub len: __u32,
    pub eth_protocol: __u32,
    pub ip_protocol: __u32,
    pub skb_addrs: [__u8; 32],
    pub skb_ports: [__u16; 2],
    pub hash: __u32,
    pub bind_inany: __u32,
}

impl Default for data_check {
    fn default() -> Self {
        Self {
            len: 0,
            eth_protocol: 0,
            ip_protocol: 0,
            skb_addrs: [0; 32],
            skb_ports: [0; 2],
            hash: 0,
            bind_inany: 0,
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum result {
    DROP_ERR_SKB_DATA = 0,
    DROP_ERR_SK_SELECT_REUSEPORT = 1,
    DROP_ERR_INNER_MAP = 2,
    DROP_MISC = 3,
    PASS = 4,
    PASS_ERR_SK_SELECT_REUSEPORT = 5,
}

const NR_RESULTS: __u32 = 6;

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static outer_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static result_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: NR_RESULTS,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static tmp_index_ovr_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<i32>() as __u32,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static linum_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static data_check_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<data_check>() as __u32,
};

unsafe extern "C" {
    fn bpf_htons(x: __u16) -> __u16;
    fn bpf_skb_load_bytes_relative(
        skb: *mut sk_reuseport_md,
        offset: __u32,
        to: *mut core::ffi::c_void,
        len: __u32,
        start_header: __u32,
    ) -> i64;
    fn bpf_skb_load_bytes(
        skb: *mut sk_reuseport_md,
        offset: __u32,
        to: *mut core::ffi::c_void,
        len: __u32,
    ) -> i64;
    fn bpf_map_lookup_elem(
        map: *const core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *const core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
    fn bpf_sk_select_reuseport(
        reuse: *mut sk_reuseport_md,
        map: *mut core::ffi::c_void,
        key: *mut core::ffi::c_void,
        flags: __u64,
    ) -> i64;
}

macro_rules! GOTO_DONE {
    ($result_var:ident, $linum_var:ident, $label_result:expr, $line:expr) => {{
        $result_var = $label_result;
        $linum_var = $line;
        break;
    }};
}

#[unsafe(link_section = "sk_reuseport")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _select_by_skb_data(reuse_md: *mut sk_reuseport_md) -> i32 {
    let mut linum: __u32 = 0;
    let mut index: __u32 = 0;
    let flags: __u64 = 0;
    let index_zero: __u32 = 0;
    let mut result_cnt: *mut __u32;
    let mut data_check: data_check = data_check::default();
    let mut cmd: *mut cmd;
    let mut cmd_copy: cmd = core::mem::zeroed();
    let data: *mut core::ffi::c_void;
    let data_end: *mut core::ffi::c_void;
    let mut reuseport_array: *mut core::ffi::c_void;
    let mut result: result = result::DROP_MISC;
    let mut index_ovr: *mut i32;
    let err: i64;

    loop {
        data = (*reuse_md).data;
        data_end = (*reuse_md).data_end;
        data_check.len = (*reuse_md).len;
        data_check.eth_protocol = (*reuse_md).eth_protocol;
        data_check.ip_protocol = (*reuse_md).ip_protocol;
        data_check.hash = (*reuse_md).hash;
        data_check.bind_inany = (*reuse_md).bind_inany;
        if data_check.eth_protocol == bpf_htons(ETH_P_IP) as __u32 {
            if bpf_skb_load_bytes_relative(
                reuse_md,
                core::mem::offset_of!(iphdr, saddr) as __u32,
                data_check.skb_addrs.as_mut_ptr() as *mut core::ffi::c_void,
                8,
                BPF_HDR_START_NET,
            ) != 0
            {
                GOTO_DONE!(result, linum, result::DROP_MISC, line!());
            }
        } else if bpf_skb_load_bytes_relative(
            reuse_md,
            core::mem::offset_of!(ipv6hdr, saddr) as __u32,
            data_check.skb_addrs.as_mut_ptr() as *mut core::ffi::c_void,
            32,
            BPF_HDR_START_NET,
        ) != 0
        {
            GOTO_DONE!(result, linum, result::DROP_MISC, line!());
        }

        /*
         * The ip_protocol could be a compile time decision
         * if the bpf_prog.o is dedicated to either TCP or
         * UDP.
         *
         * Otherwise, reuse_md->ip_protocol or
         * the protocol field in the iphdr can be used.
         */
        if data_check.ip_protocol == IPPROTO_TCP as __u32 {
            let th: *mut tcphdr = data as *mut tcphdr;

            if th.add(1) as *mut core::ffi::c_void > data_end {
                GOTO_DONE!(result, linum, result::DROP_MISC, line!());
            }

            data_check.skb_ports[0] = (*th).source;
            data_check.skb_ports[1] = (*th).dest;

            if (*th).fin() {
                /*
                 * The connection is being torn down at the end of a
                 * test. It can't contain a cmd, so return early.
                 */
                return SK_PASS;
            }

            if (((*th).doff() << 2) as usize + core::mem::size_of::<cmd>()) as __u32
                > data_check.len
            {
                GOTO_DONE!(result, linum, result::DROP_ERR_SKB_DATA, line!());
            }
            if bpf_skb_load_bytes(
                reuse_md,
                ((*th).doff() << 2) as __u32,
                &mut cmd_copy as *mut cmd as *mut core::ffi::c_void,
                core::mem::size_of_val(&cmd_copy) as __u32,
            ) != 0
            {
                GOTO_DONE!(result, linum, result::DROP_MISC, line!());
            }
            cmd = &mut cmd_copy;
        } else if data_check.ip_protocol == IPPROTO_UDP as __u32 {
            let uh: *mut udphdr = data as *mut udphdr;

            if uh.add(1) as *mut core::ffi::c_void > data_end {
                GOTO_DONE!(result, linum, result::DROP_MISC, line!());
            }

            data_check.skb_ports[0] = (*uh).source;
            data_check.skb_ports[1] = (*uh).dest;

            if (core::mem::size_of::<udphdr>() + core::mem::size_of::<cmd>()) as __u32
                > data_check.len
            {
                GOTO_DONE!(result, linum, result::DROP_ERR_SKB_DATA, line!());
            }
            if (data as *mut u8).add(core::mem::size_of::<udphdr>() + core::mem::size_of::<cmd>())
                as *mut core::ffi::c_void
                > data_end
            {
                if bpf_skb_load_bytes(
                    reuse_md,
                    core::mem::size_of::<udphdr>() as __u32,
                    &mut cmd_copy as *mut cmd as *mut core::ffi::c_void,
                    core::mem::size_of_val(&cmd_copy) as __u32,
                ) != 0
                {
                    GOTO_DONE!(result, linum, result::DROP_MISC, line!());
                }
                cmd = &mut cmd_copy;
            } else {
                cmd = (data as *mut u8).add(core::mem::size_of::<udphdr>()) as *mut cmd;
            }
        } else {
            GOTO_DONE!(result, linum, result::DROP_MISC, line!());
        }

        reuseport_array = bpf_map_lookup_elem(
            &outer_map as *const bpf_map_def as *const core::ffi::c_void,
            &index_zero as *const __u32 as *const core::ffi::c_void,
        );
        if reuseport_array.is_null() {
            GOTO_DONE!(result, linum, result::DROP_ERR_INNER_MAP, line!());
        }

        index = (*cmd).reuseport_index;
        index_ovr = bpf_map_lookup_elem(
            &tmp_index_ovr_map as *const bpf_map_def as *const core::ffi::c_void,
            &index_zero as *const __u32 as *const core::ffi::c_void,
        ) as *mut i32;
        if index_ovr.is_null() {
            GOTO_DONE!(result, linum, result::DROP_MISC, line!());
        }

        if *index_ovr != -1 {
            index = *index_ovr as __u32;
            *index_ovr = -1;
        }
        err = bpf_sk_select_reuseport(
            reuse_md,
            reuseport_array,
            &mut index as *mut __u32 as *mut core::ffi::c_void,
            flags,
        );
        if err == 0 {
            GOTO_DONE!(result, linum, result::PASS, line!());
        }

        if (*cmd).pass_on_failure != 0 {
            GOTO_DONE!(
                result,
                linum,
                result::PASS_ERR_SK_SELECT_REUSEPORT,
                line!()
            );
        } else {
            GOTO_DONE!(
                result,
                linum,
                result::DROP_ERR_SK_SELECT_REUSEPORT,
                line!()
            );
        }
    }

    result_cnt = bpf_map_lookup_elem(
        &result_map as *const bpf_map_def as *const core::ffi::c_void,
        &result as *const result as *const core::ffi::c_void,
    ) as *mut __u32;
    if result_cnt.is_null() {
        return SK_DROP;
    }

    bpf_map_update_elem(
        &linum_map as *const bpf_map_def as *const core::ffi::c_void,
        &index_zero as *const __u32 as *const core::ffi::c_void,
        &linum as *const __u32 as *const core::ffi::c_void,
        BPF_ANY,
    );
    bpf_map_update_elem(
        &data_check_map as *const bpf_map_def as *const core::ffi::c_void,
        &index_zero as *const __u32 as *const core::ffi::c_void,
        &data_check as *const data_check as *const core::ffi::c_void,
        BPF_ANY,
    );

    *result_cnt = (*result_cnt).wrapping_add(1);
    if (result as __u32) < result::PASS as __u32 {
        SK_DROP
    } else {
        SK_PASS
    }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";
