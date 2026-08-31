// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2018 Facebook
//
// Translated from bpf/bpftool/netlink_dumper.c. C include dependencies are
// expected to provide the netlink/libbpf/kernel constants, types, globals, and
// output helpers referenced below.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct nlattr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ifinfomsg {
    pub ifi_family: u8,
    pub __ifi_pad: u8,
    pub ifi_type: u16,
    pub ifi_index: c_int,
    pub ifi_flags: c_uint,
    pub ifi_change: c_uint,
}

#[repr(C)]
pub struct tcmsg {
    pub tcm_family: u8,
    pub tcm__pad1: u8,
    pub tcm__pad2: u16,
    pub tcm_ifindex: c_int,
    pub tcm_handle: u32,
    pub tcm_parent: u32,
    pub tcm_info: u32,
}

extern "C" {
    static json_output: bool;
    static mut json_wtr: *mut c_void;

    fn libbpf_nla_parse_nested(
        tb: *mut *mut nlattr,
        maxtype: c_int,
        nla: *mut nlattr,
        policy: *const c_void,
    ) -> c_int;
    fn libbpf_nla_getattr_u8(attr: *const nlattr) -> u8;
    fn libbpf_nla_getattr_u32(attr: *const nlattr) -> u32;
    fn libbpf_nla_getattr_str(attr: *const nlattr) -> *const c_char;
    fn libbpf_nla_data(attr: *const nlattr) -> *mut c_void;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn jsonw_name(wtr: *mut c_void, name: *const c_char);
    fn jsonw_start_array(wtr: *mut c_void);
    fn jsonw_end_array(wtr: *mut c_void);
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

// External C macros from main.h/netlink_dumper.h are preserved as Rust macro
// calls and are expected to be supplied by the surrounding translation unit.

unsafe fn xdp_dump_prog_id(
    tb: *mut *mut nlattr,
    attr: c_int,
    mode: *const c_char,
    new_json_object: bool,
) {
    if (*tb.add(attr as usize)).is_null() {
        return;
    }

    if new_json_object {
        NET_START_OBJECT!();
    }
    NET_DUMP_STR!(c_str!("mode"), c_str!(" %s"), mode);
    NET_DUMP_UINT!(
        c_str!("id"),
        c_str!(" id %u"),
        libbpf_nla_getattr_u32(*tb.add(attr as usize))
    );
    if new_json_object {
        NET_END_OBJECT!();
    }
}

unsafe fn do_xdp_dump_one(attr: *mut nlattr, ifindex: c_uint, name: *const c_char) -> c_int {
    let mut tb: [*mut nlattr; IFLA_XDP_MAX as usize + 1] =
        [ptr::null_mut(); IFLA_XDP_MAX as usize + 1];
    let mode: u8;

    if libbpf_nla_parse_nested(
        tb.as_mut_ptr(),
        IFLA_XDP_MAX as c_int,
        attr,
        ptr::null(),
    ) < 0
    {
        return -1;
    }

    if tb[IFLA_XDP_ATTACHED as usize].is_null() {
        return 0;
    }

    mode = libbpf_nla_getattr_u8(tb[IFLA_XDP_ATTACHED as usize]);
    if mode as c_int == XDP_ATTACHED_NONE {
        return 0;
    }

    NET_START_OBJECT!();
    if !name.is_null() {
        NET_DUMP_STR!(c_str!("devname"), c_str!("%s"), name);
    }
    NET_DUMP_UINT!(c_str!("ifindex"), c_str!("(%u)"), ifindex);

    if mode as c_int == XDP_ATTACHED_MULTI {
        if json_output {
            jsonw_name(json_wtr, c_str!("multi_attachments"));
            jsonw_start_array(json_wtr);
        }
        xdp_dump_prog_id(
            tb.as_mut_ptr(),
            IFLA_XDP_SKB_PROG_ID as c_int,
            c_str!("generic"),
            true,
        );
        xdp_dump_prog_id(
            tb.as_mut_ptr(),
            IFLA_XDP_DRV_PROG_ID as c_int,
            c_str!("driver"),
            true,
        );
        xdp_dump_prog_id(
            tb.as_mut_ptr(),
            IFLA_XDP_HW_PROG_ID as c_int,
            c_str!("offload"),
            true,
        );
        if json_output {
            jsonw_end_array(json_wtr);
        }
    } else if mode as c_int == XDP_ATTACHED_DRV {
        xdp_dump_prog_id(
            tb.as_mut_ptr(),
            IFLA_XDP_PROG_ID as c_int,
            c_str!("driver"),
            false,
        );
    } else if mode as c_int == XDP_ATTACHED_SKB {
        xdp_dump_prog_id(
            tb.as_mut_ptr(),
            IFLA_XDP_PROG_ID as c_int,
            c_str!("generic"),
            false,
        );
    } else if mode as c_int == XDP_ATTACHED_HW {
        xdp_dump_prog_id(
            tb.as_mut_ptr(),
            IFLA_XDP_PROG_ID as c_int,
            c_str!("offload"),
            false,
        );
    }

    NET_END_OBJECT_FINAL!();
    0
}

#[no_mangle]
pub unsafe extern "C" fn do_xdp_dump(ifinfo: *mut ifinfomsg, tb: *mut *mut nlattr) -> c_int {
    if (*tb.add(IFLA_XDP as usize)).is_null() {
        return 0;
    }

    do_xdp_dump_one(
        *tb.add(IFLA_XDP as usize),
        (*ifinfo).ifi_index as c_uint,
        libbpf_nla_getattr_str(*tb.add(IFLA_IFNAME as usize)),
    )
}

unsafe fn do_bpf_dump_one_act(attr: *mut nlattr) -> c_int {
    let mut tb: [*mut nlattr; TCA_ACT_BPF_MAX as usize + 1] =
        [ptr::null_mut(); TCA_ACT_BPF_MAX as usize + 1];

    if libbpf_nla_parse_nested(
        tb.as_mut_ptr(),
        TCA_ACT_BPF_MAX as c_int,
        attr,
        ptr::null(),
    ) < 0
    {
        return -LIBBPF_ERRNO__NLPARSE;
    }

    if tb[TCA_ACT_BPF_PARMS as usize].is_null() {
        return -LIBBPF_ERRNO__NLPARSE;
    }

    NET_START_OBJECT_NESTED2!();
    if !tb[TCA_ACT_BPF_NAME as usize].is_null() {
        NET_DUMP_STR!(
            c_str!("name"),
            c_str!("%s"),
            libbpf_nla_getattr_str(tb[TCA_ACT_BPF_NAME as usize])
        );
    }
    if !tb[TCA_ACT_BPF_ID as usize].is_null() {
        NET_DUMP_UINT!(
            c_str!("id"),
            c_str!(" id %u"),
            libbpf_nla_getattr_u32(tb[TCA_ACT_BPF_ID as usize])
        );
    }
    NET_END_OBJECT_NESTED!();
    0
}

unsafe fn do_dump_one_act(attr: *mut nlattr) -> c_int {
    let mut tb: [*mut nlattr; TCA_ACT_MAX as usize + 1] =
        [ptr::null_mut(); TCA_ACT_MAX as usize + 1];

    if attr.is_null() {
        return 0;
    }

    if libbpf_nla_parse_nested(tb.as_mut_ptr(), TCA_ACT_MAX as c_int, attr, ptr::null()) < 0 {
        return -LIBBPF_ERRNO__NLPARSE;
    }

    if !tb[TCA_ACT_KIND as usize].is_null()
        && strcmp(libbpf_nla_data(tb[TCA_ACT_KIND as usize]) as *const c_char, c_str!("bpf")) == 0
    {
        return do_bpf_dump_one_act(tb[TCA_ACT_OPTIONS as usize]);
    }

    0
}

unsafe fn do_bpf_act_dump(attr: *mut nlattr) -> c_int {
    let mut tb: [*mut nlattr; TCA_ACT_MAX_PRIO as usize + 1] =
        [ptr::null_mut(); TCA_ACT_MAX_PRIO as usize + 1];
    let mut act: c_int;
    let mut ret: c_int;

    if libbpf_nla_parse_nested(
        tb.as_mut_ptr(),
        TCA_ACT_MAX_PRIO as c_int,
        attr,
        ptr::null(),
    ) < 0
    {
        return -LIBBPF_ERRNO__NLPARSE;
    }

    NET_START_ARRAY!(c_str!("act"), c_str!(" %s ["));
    act = 0;
    loop {
        if act > TCA_ACT_MAX_PRIO as c_int {
            ret = 0;
            break;
        }
        ret = do_dump_one_act(tb[act as usize]);
        if ret != 0 {
            break;
        }
        act += 1;
    }
    NET_END_ARRAY!(c_str!("] "));

    ret
}

unsafe fn do_bpf_filter_dump(attr: *mut nlattr) -> c_int {
    let mut tb: [*mut nlattr; TCA_BPF_MAX as usize + 1] =
        [ptr::null_mut(); TCA_BPF_MAX as usize + 1];
    let ret: c_int;

    if libbpf_nla_parse_nested(tb.as_mut_ptr(), TCA_BPF_MAX as c_int, attr, ptr::null()) < 0 {
        return -LIBBPF_ERRNO__NLPARSE;
    }

    if !tb[TCA_BPF_NAME as usize].is_null() {
        NET_DUMP_STR!(
            c_str!("name"),
            c_str!(" %s"),
            libbpf_nla_getattr_str(tb[TCA_BPF_NAME as usize])
        );
    }
    if !tb[TCA_BPF_ID as usize].is_null() {
        NET_DUMP_UINT!(
            c_str!("id"),
            c_str!(" id %u"),
            libbpf_nla_getattr_u32(tb[TCA_BPF_ID as usize])
        );
    }
    if !tb[TCA_BPF_ACT as usize].is_null() {
        ret = do_bpf_act_dump(tb[TCA_BPF_ACT as usize]);
        if ret != 0 {
            return ret;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn do_filter_dump(
    _info: *mut tcmsg,
    tb: *mut *mut nlattr,
    kind: *const c_char,
    devname: *const c_char,
    ifindex: c_int,
) -> c_int {
    let mut ret: c_int = 0;

    if !(*tb.add(TCA_OPTIONS as usize)).is_null()
        && strcmp(libbpf_nla_data(*tb.add(TCA_KIND as usize)) as *const c_char, c_str!("bpf")) == 0
    {
        NET_START_OBJECT!();
        if *devname != b'\0' as c_char {
            NET_DUMP_STR!(c_str!("devname"), c_str!("%s"), devname);
        }
        NET_DUMP_UINT!(c_str!("ifindex"), c_str!("(%u)"), ifindex as c_uint);
        NET_DUMP_STR!(c_str!("kind"), c_str!(" %s"), kind);
        ret = do_bpf_filter_dump(*tb.add(TCA_OPTIONS as usize));
        NET_END_OBJECT_FINAL!();
    }

    ret
}
