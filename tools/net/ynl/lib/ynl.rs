// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Source-level Rust translation of net/ynl/lib/ynl.c.
// C include dependencies intentionally remain external to this isolated file.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;
type ssize_t = isize;
type socklen_t = u32;

const YNL_PARSE_CB_ERROR: c_int = -1;
const YNL_PARSE_CB_STOP: c_int = 0;
const YNL_PARSE_CB_OK: c_int = 1;
const YNL_ERROR_INTERNAL: c_int = 1;
const YNL_ERROR_ATTR_INVALID: c_int = 2;
const YNL_ERROR_SUBMSG_KEY: c_int = 3;
const YNL_ERROR_INPUT_INVALID: c_int = 4;
const YNL_ERROR_INPUT_TOO_BIG: c_int = 5;
const YNL_ERROR_UNEXPECT_MSG: c_int = 6;
const YNL_ERROR_EXPECT_ACK: c_int = 7;
const YNL_ERROR_INV_RESP: c_int = 8;
const YNL_ERROR_DUMP_INTER: c_int = 9;
const YNL_ERROR_ATTR_MISSING: c_int = 10;
const YNL_ERROR_UNKNOWN_NTF: c_int = 11;

const YNL_SOCKET_BUFFER_SIZE: usize = 131072;
const YNL_MSG_OVERFLOW: __u32 = !0;

const EAGAIN: c_int = 11;
const EINVAL: c_int = 22;
const EMSGSIZE: c_int = 90;
const ENOENT: c_int = 2;
const MSG_DONTWAIT: c_int = 0x40;
const AF_NETLINK: c_int = 16;
const SOCK_RAW: c_int = 3;
const SOL_NETLINK: c_int = 270;
const NETLINK_GENERIC: c_int = 16;
const NETLINK_CAP_ACK: c_int = 10;
const NETLINK_EXT_ACK: c_int = 11;
const NETLINK_ADD_MEMBERSHIP: c_int = 1;
const NLM_F_REQUEST: __u16 = 0x01;
const NLM_F_ACK: __u16 = 0x04;
const NLM_F_DUMP: __u16 = 0x300;
const NLM_F_CAPPED: __u16 = 0x100;
const NLM_F_ACK_TLVS: __u16 = 0x200;
const NLM_F_DUMP_INTR: __u16 = 0x10;
const NLMSG_NOOP: __u16 = 1;
const NLMSG_ERROR: __u16 = 2;
const NLMSG_DONE: __u16 = 3;
const NLMSG_OVERRUN: __u16 = 4;
const NLMSG_MIN_TYPE: __u16 = 0x10;
const GENL_ID_CTRL: __u32 = 0x10;
const CTRL_CMD_GETFAMILY: __u8 = 3;
const CTRL_ATTR_FAMILY_ID: c_uint = 1;
const CTRL_ATTR_FAMILY_NAME: c_uint = 2;
const CTRL_ATTR_MCAST_GROUPS: c_uint = 7;
const CTRL_ATTR_MCAST_GRP_NAME: c_uint = 1;
const CTRL_ATTR_MCAST_GRP_ID: c_uint = 2;
const GENL_NAMSIZ: usize = 16;
const NLMSGERR_ATTR_OFFS: usize = 1;
const NLMSGERR_ATTR_MSG: usize = 2;
const NLMSGERR_ATTR_POLICY: usize = 3;
const NLMSGERR_ATTR_MISS_TYPE: usize = NLMSGERR_ATTR_POLICY + 1;
const NLMSGERR_ATTR_MISS_NEST: usize = NLMSGERR_ATTR_POLICY + 2;
const NLMSGERR_ATTR_MAX: usize = NLMSGERR_ATTR_POLICY + 2;

#[repr(C)] pub struct nlmsghdr { pub nlmsg_len: __u32, pub nlmsg_type: __u16, pub nlmsg_flags: __u16, pub nlmsg_seq: __u32, pub nlmsg_pid: __u32 }
#[repr(C)] pub struct nlmsgerr { pub error: c_int, pub msg: nlmsghdr }
#[repr(C)] pub struct nlattr { pub nla_len: __u16, pub nla_type: __u16 }
#[repr(C)] pub struct genlmsghdr { pub cmd: __u8, pub version: __u8, pub reserved: __u16 }
#[repr(C)] pub struct sockaddr { pub sa_family: __u16, pub sa_data: [c_char; 14] }
#[repr(C)] pub struct sockaddr_nl { pub nl_family: __u16, pub nl_pad: __u16, pub nl_pid: __u32, pub nl_groups: __u32 }
#[repr(C)] pub struct nla_bitfield32 { pub value: __u32, pub selector: __u32 }

#[repr(C)] pub struct ynl_error { pub code: c_int, pub attr_offs: c_uint, pub msg: [c_char; 512] }
#[repr(C)] pub struct ynl_policy_nest { pub max_attr: c_uint, pub table: *const ynl_policy_attr }
#[repr(C)] pub struct ynl_policy_attr {
    pub name: *const c_char, pub r#type: c_int, pub len: c_uint, pub nest: *const ynl_policy_nest,
    pub is_selector: bool, pub is_submsg: bool, pub selector_type: c_uint,
}
#[repr(C)] pub struct ynl_mcast_group { pub id: __u32, pub name: [c_char; GENL_NAMSIZ] }
#[repr(C)] pub struct ynl_family {
    pub name: *const c_char, pub is_classic: bool, pub classic_id: c_int,
    pub ntf_info_size: __u32, pub ntf_info: *const ynl_ntf_info,
}
pub type ynl_parse_cb_t = unsafe extern "C" fn(*const nlmsghdr, *mut ynl_parse_arg) -> c_int;
pub type ynl_free_cb_t = unsafe extern "C" fn(*mut ynl_ntf_base_type);
#[repr(C)] pub struct ynl_ntf_info { pub cb: Option<ynl_parse_cb_t>, pub free: ynl_free_cb_t, pub alloc_sz: usize, pub policy: *const ynl_policy_nest }
#[repr(C)] pub struct ynl_parse_arg { pub ys: *mut ynl_sock, pub data: *mut c_void, pub rsp_policy: *const ynl_policy_nest }
#[repr(C)] pub struct ynl_req_state { pub yarg: ynl_parse_arg, pub cb: ynl_parse_cb_t, pub rsp_cmd: __u32 }
#[repr(C)] pub struct ynl_dump_state { pub yarg: ynl_parse_arg, pub cb: ynl_parse_cb_t, pub rsp_cmd: __u32, pub alloc_sz: usize, pub first: *mut ynl_dump_list_type, pub last: *mut ynl_dump_list_type }
#[repr(C)] pub struct ynl_ntf_base_type { pub next: *mut ynl_ntf_base_type, pub free: ynl_free_cb_t, pub family: __u32, pub cmd: __u32, pub data: [u8; 0] }
#[repr(C)] pub struct ynl_dump_list_type { pub next: *mut ynl_dump_list_type, pub data: [u8; 0] }
#[repr(C)] pub struct ynl_sock {
    pub err: ynl_error, pub nlh: *mut nlmsghdr, pub req_hdr_len: c_uint, pub req_policy: *const ynl_policy_nest,
    pub socket: c_int, pub family: *const ynl_family, pub family_id: c_int, pub portid: __u32, pub seq: __u32,
    pub n_mcast_groups: c_uint, pub mcast_groups: *mut ynl_mcast_group,
    pub ntf_first: *mut ynl_ntf_base_type, pub ntf_last_next: *mut *mut ynl_ntf_base_type,
    pub tx_buf: *mut c_void, pub rx_buf: *mut c_void, pub raw_buf: [u8; 0],
}

const YNL_PT_REJECT: c_int = 0; const YNL_PT_IGNORE: c_int = 1; const YNL_PT_U8: c_int = 2; const YNL_PT_U16: c_int = 3;
const YNL_PT_U32: c_int = 4; const YNL_PT_U64: c_int = 5; const YNL_PT_UINT: c_int = 6; const YNL_PT_FLAG: c_int = 7;
const YNL_PT_NEST: c_int = 8; const YNL_PT_BINARY: c_int = 9; const YNL_PT_NUL_STR: c_int = 10; const YNL_PT_BITFIELD32: c_int = 11;

unsafe extern "C" {
    static mut errno: c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn random() -> isize;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn setsockopt(fd: c_int, level: c_int, optname: c_int, optval: *const c_void, optlen: socklen_t) -> c_int;
    fn bind(fd: c_int, addr: *const sockaddr, len: socklen_t) -> c_int;
    fn getsockname(fd: c_int, addr: *mut sockaddr, len: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn send(fd: c_int, buf: *const c_void, len: usize, flags: c_int) -> ssize_t;
    fn recv(fd: c_int, buf: *mut c_void, len: usize, flags: c_int) -> ssize_t;
    fn ynl_attr_type(attr: *const nlattr) -> c_uint;
    fn ynl_attr_data(attr: *const nlattr) -> *mut c_void;
    fn ynl_attr_data_end(attr: *const nlattr) -> *mut c_void;
    fn ynl_attr_data_len(attr: *const nlattr) -> c_uint;
    fn ynl_attr_get_str(attr: *const nlattr) -> *const c_char;
    fn ynl_attr_get_u16(attr: *const nlattr) -> __u16;
    fn ynl_attr_get_u32(attr: *const nlattr) -> __u32;
    fn ynl_attr_put_str(nlh: *mut nlmsghdr, typ: c_uint, str: *const c_char);
    fn ynl_nlmsg_put_header(buf: *mut c_void) -> *mut nlmsghdr;
    fn ynl_nlmsg_put_extra_header(nlh: *mut nlmsghdr, len: usize) -> *mut c_void;
    fn ynl_nlmsg_data(nlh: *const nlmsghdr) -> *mut c_void;
    fn ynl_nlmsg_data_offset(nlh: *const nlmsghdr, off: c_uint) -> *mut c_void;
    fn ynl_nlmsg_data_len(nlh: *const nlmsghdr) -> c_uint;
    fn ynl_nlmsg_end_addr(nlh: *const nlmsghdr) -> *mut c_void;
}

unsafe fn cfmt(dst: *mut c_char, sz: usize, s: &'static [u8]) -> c_int { snprintf(dst, sz, s.as_ptr() as *const c_char) }
unsafe fn yerr_msg(ys: *mut ynl_sock, s: &'static [u8]) { if !ys.is_null() { cfmt((*ys).err.msg.as_mut_ptr(), (*ys).err.msg.len() - 1, s); (*ys).err.msg[(*ys).err.msg.len()-1] = 0; } }
unsafe fn yerr_code(ys: *mut ynl_sock, code: c_int) { if !ys.is_null() { (*ys).err.code = code; } }
unsafe fn yerr_set(ys: *mut ynl_sock, code: c_int, s: &'static [u8]) { yerr_msg(ys, s); yerr_code(ys, code); }
unsafe fn nlmsg_data(nlh: *const nlmsghdr) -> *mut c_void { (nlh as *mut u8).add(size_of::<nlmsghdr>()) as *mut c_void }
unsafe fn nlmsg_ok(nlh: *const nlmsghdr, rem: ssize_t) -> bool { rem >= size_of::<nlmsghdr>() as isize && (*nlh).nlmsg_len as isize >= size_of::<nlmsghdr>() as isize && (*nlh).nlmsg_len as isize <= rem }
fn nlmsg_align(len: usize) -> usize { (len + 3) & !3 }

unsafe fn ynl_err_walk_is_sel(policy: *const ynl_policy_nest, attr: *const nlattr) -> bool {
    let typ = ynl_attr_type(attr);
    !policy.is_null() && typ <= (*policy).max_attr && (*(*policy).table.add(typ as usize)).is_selector
}

unsafe fn ynl_err_walk_sel_policy(policy_attr: *const ynl_policy_attr, selector: *const nlattr) -> *const ynl_policy_nest {
    let policy = (*policy_attr).nest;
    if !(*policy_attr).is_submsg { return policy; }
    let sel = ynl_attr_get_str(selector);
    let mut i = 0;
    while i <= (*policy).max_attr {
        let p = (*policy).table.add(i as usize);
        if strcmp(sel, (*p).name) == 0 { return (*p).nest; }
        i += 1;
    }
    ptr::null()
}

unsafe fn ynl_err_walk_report_one(policy: *const ynl_policy_nest, selector: *const nlattr, typ: c_uint, mut strp: *mut c_char, str_sz: c_int, n: *mut c_int) -> c_int {
    if policy.is_null() { if *n < str_sz { *n += cfmt(strp, str_sz as usize, b"!policy\0"); } return 1; }
    if typ > (*policy).max_attr { if *n < str_sz { *n += cfmt(strp, str_sz as usize, b"!oob\0"); } return 1; }
    let pa = (*policy).table.add(typ as usize);
    if (*pa).name.is_null() { if *n < str_sz { *n += cfmt(strp, str_sz as usize, b"!name\0"); } return 1; }
    if *n < str_sz {
        let sz = snprintf(strp, (str_sz - *n) as usize, b".%s\0".as_ptr() as *const c_char, (*pa).name);
        *n += sz; strp = strp.add(sz as usize);
    }
    if (*pa).is_submsg {
        if selector.is_null() { if *n < str_sz { *n += cfmt(strp, str_sz as usize, b"(!selector)\0"); } return 1; }
        if ynl_attr_type(selector) != (*pa).selector_type { if *n < str_sz { *n += cfmt(strp, str_sz as usize, b"(!=selector)\0"); } return 1; }
        if *n < str_sz { *n += snprintf(strp, (str_sz - *n) as usize, b"(%s)\0".as_ptr() as *const c_char, ynl_attr_get_str(selector)); }
    }
    0
}

unsafe fn ynl_err_walk(ys: *mut ynl_sock, mut start: *mut c_void, end: *mut c_void, mut off: c_uint, policy: *const ynl_policy_nest, strp: *mut c_char, str_sz: c_int, nest_pol: *mut *const ynl_policy_nest) -> c_int {
    let mut selector: *const nlattr = ptr::null();
    let mut found = false;
    let mut n = 0;
    if policy.is_null() { if n < str_sz { n += cfmt(strp, str_sz as usize, b"!policy\0"); } return n; }
    let data_len = (end as usize).wrapping_sub(start as usize) as c_uint;
    let mut attr = start as *const nlattr;
    let stop = end as usize;
    while (attr as usize) < stop && (attr as usize) + size_of::<nlattr>() <= stop {
        let astart_off = (attr as usize).wrapping_sub(start as usize) as c_uint;
        let aend_off = (ynl_attr_data_end(attr) as usize).wrapping_sub(start as usize) as c_uint;
        if ynl_err_walk_is_sel(policy, attr) { selector = attr; }
        if aend_off > off { found = true; break; }
        attr = (attr as *const u8).add(nlmsg_align((*attr).nla_len as usize)) as *const nlattr;
    }
    if !found { return 0; }
    off = off.wrapping_sub((attr as usize).wrapping_sub(start as usize) as c_uint);
    let typ = ynl_attr_type(attr);
    if ynl_err_walk_report_one(policy, selector, typ, strp, str_sz, &mut n) != 0 { return n; }
    let next_pol = ynl_err_walk_sel_policy((*policy).table.add(typ as usize), selector);
    if next_pol.is_null() { return n; }
    if off == 0 { if !nest_pol.is_null() { *nest_pol = next_pol; } return n; }
    off = off.wrapping_sub(size_of::<nlattr>() as c_uint);
    start = ynl_attr_data(attr);
    let end2 = (start as *mut u8).add(ynl_attr_data_len(attr) as usize) as *mut c_void;
    n + ynl_err_walk(ys, start, end2, off, next_pol, strp.add(n as usize), str_sz - n, nest_pol)
}

unsafe fn ynl_ext_ack_check(ys: *mut ynl_sock, nlh: *const nlmsghdr, hlen: c_uint) -> c_int {
    let mut tb: [*const nlattr; NLMSGERR_ATTR_MAX + 1] = [ptr::null(); NLMSGERR_ATTR_MAX + 1];
    let mut miss_attr = [0 as c_char; 512];
    let mut bad_attr = [0 as c_char; 512];
    let mut strp: *const c_char = ptr::null();
    if ((*nlh).nlmsg_flags & NLM_F_ACK_TLVS) == 0 {
        snprintf((*ys).err.msg.as_mut_ptr(), (*ys).err.msg.len() - 1, b"%s\0".as_ptr() as *const c_char, strerror((*ys).err.code));
        return YNL_PARSE_CB_OK;
    }
    let mut attr = ynl_nlmsg_data_offset(nlh, hlen) as *const nlattr;
    let end = ynl_nlmsg_end_addr(nlh) as usize;
    while (attr as usize) < end && (attr as usize) + size_of::<nlattr>() <= end {
        let len = ynl_attr_data_len(attr);
        let typ = ynl_attr_type(attr) as usize;
        if typ <= NLMSGERR_ATTR_MAX {
            tb[typ] = attr;
            match typ {
                NLMSGERR_ATTR_OFFS | NLMSGERR_ATTR_MISS_TYPE | NLMSGERR_ATTR_MISS_NEST => if len as usize != size_of::<__u32>() { return YNL_PARSE_CB_ERROR; },
                NLMSGERR_ATTR_MSG => { strp = ynl_attr_get_str(attr); if *(strp.add(len as usize - 1)) != 0 { return YNL_PARSE_CB_ERROR; } },
                _ => {}
            }
        }
        attr = (attr as *const u8).add(nlmsg_align((*attr).nla_len as usize)) as *const nlattr;
    }
    bad_attr[0] = 0; miss_attr[0] = 0;
    if !tb[NLMSGERR_ATTR_OFFS].is_null() {
        (*ys).err.attr_offs = ynl_attr_get_u32(tb[NLMSGERR_ATTR_OFFS]);
        let mut n = snprintf(bad_attr.as_mut_ptr(), bad_attr.len(), b"%sbad attribute: \0".as_ptr() as *const c_char, if !strp.is_null() { b" (\0".as_ptr() } else { b"\0".as_ptr() } as *const c_char) as c_uint;
        let start = ynl_nlmsg_data_offset((*ys).nlh, (*ys).req_hdr_len);
        let endp = ynl_nlmsg_end_addr((*ys).nlh);
        let off = (*ys).err.attr_offs.wrapping_sub(size_of::<nlmsghdr>() as c_uint).wrapping_sub((*ys).req_hdr_len);
        n += ynl_err_walk(ys, start, endp, off, (*ys).req_policy, bad_attr.as_mut_ptr().add(n as usize), (bad_attr.len() - n as usize) as c_int, ptr::null_mut()) as c_uint;
        if n as usize >= bad_attr.len() { n = (bad_attr.len() - 1) as c_uint; } bad_attr[n as usize] = 0;
    }
    if !tb[NLMSGERR_ATTR_MISS_TYPE].is_null() {
        let mut nest_pol = (*ys).req_policy;
        let typ = ynl_attr_get_u32(tb[NLMSGERR_ATTR_MISS_TYPE]);
        let mut n = snprintf(miss_attr.as_mut_ptr(), miss_attr.len(), b"%smissing attribute: \0".as_ptr() as *const c_char, if bad_attr[0] != 0 { b", \0".as_ptr() } else if !strp.is_null() { b" (\0".as_ptr() } else { b"\0".as_ptr() } as *const c_char) as c_uint;
        let start = ynl_nlmsg_data_offset((*ys).nlh, (*ys).req_hdr_len);
        let endp = ynl_nlmsg_end_addr((*ys).nlh);
        if !tb[NLMSGERR_ATTR_MISS_NEST].is_null() {
            let off = ynl_attr_get_u32(tb[NLMSGERR_ATTR_MISS_NEST]).wrapping_sub(size_of::<nlmsghdr>() as c_uint).wrapping_sub((*ys).req_hdr_len);
            n += ynl_err_walk(ys, start, endp, off, (*ys).req_policy, miss_attr.as_mut_ptr().add(n as usize), (miss_attr.len() - n as usize) as c_int, &mut nest_pol) as c_uint;
        }
        let mut n2 = 0;
        ynl_err_walk_report_one(nest_pol, ptr::null(), typ, miss_attr.as_mut_ptr().add(n as usize), (miss_attr.len() - n as usize) as c_int, &mut n2);
        n += n2 as c_uint;
        if n as usize >= miss_attr.len() { n = (miss_attr.len() - 1) as c_uint; } miss_attr[n as usize] = 0;
    }
    if !strp.is_null() {
        snprintf((*ys).err.msg.as_mut_ptr(), (*ys).err.msg.len() - 1, b"Kernel %s: '%s'%s%s%s\0".as_ptr() as *const c_char,
            if (*ys).err.code != 0 { b"error\0".as_ptr() } else { b"warning\0".as_ptr() } as *const c_char,
            strp, bad_attr.as_ptr(), miss_attr.as_ptr(), if bad_attr[0] != 0 || miss_attr[0] != 0 { b")\0".as_ptr() } else { b"\0".as_ptr() } as *const c_char);
    } else if bad_attr[0] != 0 || miss_attr[0] != 0 {
        snprintf((*ys).err.msg.as_mut_ptr(), (*ys).err.msg.len() - 1, b"Kernel %s: %s%s\0".as_ptr() as *const c_char,
            if (*ys).err.code != 0 { b"error\0".as_ptr() } else { b"warning\0".as_ptr() } as *const c_char, bad_attr.as_ptr(), miss_attr.as_ptr());
    } else {
        snprintf((*ys).err.msg.as_mut_ptr(), (*ys).err.msg.len() - 1, b"%s\0".as_ptr() as *const c_char, strerror((*ys).err.code));
    }
    YNL_PARSE_CB_OK
}

unsafe extern "C" fn ynl_cb_error(nlh: *const nlmsghdr, yarg: *mut ynl_parse_arg) -> c_int {
    let err = ynl_nlmsg_data(nlh) as *const nlmsgerr;
    let code = if (*err).error >= 0 { (*err).error } else { -(*err).error };
    (*(*yarg).ys).err.code = code; errno = code;
    let mut hlen = size_of::<nlmsgerr>() as c_uint;
    if ((*nlh).nlmsg_flags & NLM_F_CAPPED) == 0 { hlen += ynl_nlmsg_data_len(&(*err).msg); }
    ynl_ext_ack_check((*yarg).ys, nlh, hlen);
    if code != 0 { YNL_PARSE_CB_ERROR } else { YNL_PARSE_CB_STOP }
}

unsafe extern "C" fn ynl_cb_done(nlh: *const nlmsghdr, yarg: *mut ynl_parse_arg) -> c_int {
    let err = *(nlmsg_data(nlh) as *const c_int);
    if err < 0 { (*(*yarg).ys).err.code = -err; errno = -err; ynl_ext_ack_check((*yarg).ys, nlh, size_of::<c_int>() as c_uint); return YNL_PARSE_CB_ERROR; }
    YNL_PARSE_CB_STOP
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __ynl_attr_validate(yarg: *mut ynl_parse_arg, attr: *const nlattr, typ: c_uint) -> c_int {
    let data = ynl_attr_data(attr) as *mut u8;
    let len = ynl_attr_data_len(attr) as usize;
    if typ > (*(*yarg).rsp_policy).max_attr { yerr_set((*yarg).ys, YNL_ERROR_INTERNAL, b"Internal error, validating unknown attribute\0"); return -1; }
    let policy = (*(*yarg).rsp_policy).table.add(typ as usize);
    match (*policy).r#type {
        YNL_PT_REJECT => { yerr_set((*yarg).ys, YNL_ERROR_ATTR_INVALID, b"Rejected attribute (%s)\0"); return -1; }
        YNL_PT_IGNORE => {}
        YNL_PT_U8 if len == size_of::<__u8>() => {}
        YNL_PT_U16 if len == size_of::<__u16>() => {}
        YNL_PT_U32 if len == size_of::<__u32>() => {}
        YNL_PT_U64 if len == size_of::<__u64>() => {}
        YNL_PT_UINT if len == size_of::<__u32>() || len == size_of::<__u64>() => {}
        YNL_PT_FLAG => {}
        YNL_PT_NEST if len == 0 || len >= size_of::<nlattr>() => {}
        YNL_PT_BINARY if (*policy).len == 0 || len == (*policy).len as usize => {}
        YNL_PT_NUL_STR if len != 0 && ((*policy).len == 0 || len <= (*policy).len as usize) && *data.add(len - 1) == 0 => {}
        YNL_PT_BITFIELD32 if len == size_of::<nla_bitfield32>() => {}
        YNL_PT_U8 | YNL_PT_U16 | YNL_PT_U32 | YNL_PT_U64 | YNL_PT_UINT | YNL_PT_NEST | YNL_PT_BINARY | YNL_PT_NUL_STR | YNL_PT_BITFIELD32 => { yerr_set((*yarg).ys, YNL_ERROR_ATTR_INVALID, b"Invalid attribute (%s)\0"); return -1; }
        _ => { yerr_set((*yarg).ys, YNL_ERROR_ATTR_INVALID, b"Invalid attribute (unknown %s)\0"); return -1; }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ynl_submsg_failed(yarg: *mut ynl_parse_arg, field_name: *const c_char, sel_name: *const c_char) -> c_int {
    snprintf((*(*yarg).ys).err.msg.as_mut_ptr(), (*(*yarg).ys).err.msg.len() - 1, b"Parsing error: Sub-message key not set (msg %s, key %s)\0".as_ptr() as *const c_char, field_name, sel_name);
    (*(*yarg).ys).err.code = YNL_ERROR_SUBMSG_KEY;
    YNL_PARSE_CB_ERROR
}

unsafe fn ynl_err_reset(ys: *mut ynl_sock) { (*ys).err.code = 0; (*ys).err.attr_offs = 0; (*ys).err.msg[0] = 0; }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ynl_msg_start(ys: *mut ynl_sock, id: __u32, flags: __u16) -> *mut nlmsghdr {
    ynl_err_reset(ys);
    let nlh = ynl_nlmsg_put_header((*ys).tx_buf); (*ys).nlh = nlh;
    (*nlh).nlmsg_type = id as __u16; (*nlh).nlmsg_flags = flags; (*nlh).nlmsg_seq = (*ys).seq.wrapping_add(1); (*ys).seq = (*nlh).nlmsg_seq;
    (*nlh).nlmsg_pid = YNL_SOCKET_BUFFER_SIZE as __u32;
    nlh
}

unsafe fn ynl_msg_end(ys: *mut ynl_sock, nlh: *mut nlmsghdr) -> c_int {
    if (*nlh).nlmsg_pid == 0 { yerr_set(ys, YNL_ERROR_INPUT_INVALID, b"Unknown input buffer length\0"); return -EINVAL; }
    if (*nlh).nlmsg_pid == YNL_MSG_OVERFLOW { yerr_set(ys, YNL_ERROR_INPUT_TOO_BIG, b"Constructed message longer than internal buffer\0"); return -EMSGSIZE; }
    (*nlh).nlmsg_pid = 0; 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ynl_gemsg_start(ys: *mut ynl_sock, id: __u32, flags: __u16, cmd: __u8, version: __u8) -> *mut nlmsghdr {
    let mut gehdr: genlmsghdr = zeroed();
    let nlh = ynl_msg_start(ys, id, flags);
    gehdr.cmd = cmd; gehdr.version = version;
    let data = ynl_nlmsg_put_extra_header(nlh, size_of::<genlmsghdr>());
    memcpy(data, &gehdr as *const _ as *const c_void, size_of::<genlmsghdr>());
    nlh
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn ynl_msg_start_req(ys: *mut ynl_sock, id: __u32, flags: __u16) -> *mut nlmsghdr { ynl_msg_start(ys, id, NLM_F_REQUEST | NLM_F_ACK | flags) }
#[unsafe(no_mangle)] pub unsafe extern "C" fn ynl_msg_start_dump(ys: *mut ynl_sock, id: __u32) -> *mut nlmsghdr { ynl_msg_start(ys, id, NLM_F_REQUEST | NLM_F_ACK | NLM_F_DUMP) }
#[unsafe(no_mangle)] pub unsafe extern "C" fn ynl_gemsg_start_req(ys: *mut ynl_sock, id: __u32, cmd: __u8, version: __u8) -> *mut nlmsghdr { ynl_gemsg_start(ys, id, NLM_F_REQUEST | NLM_F_ACK, cmd, version) }
#[unsafe(no_mangle)] pub unsafe extern "C" fn ynl_gemsg_start_dump(ys: *mut ynl_sock, id: __u32, cmd: __u8, version: __u8) -> *mut nlmsghdr { ynl_gemsg_start(ys, id, NLM_F_REQUEST | NLM_F_ACK | NLM_F_DUMP, cmd, version) }

unsafe extern "C" fn ynl_cb_null(_nlh: *const nlmsghdr, yarg: *mut ynl_parse_arg) -> c_int {
    yerr_set((*yarg).ys, YNL_ERROR_UNEXPECT_MSG, b"Received a message when none were expected\0"); YNL_PARSE_CB_ERROR
}

unsafe fn __ynl_sock_read_msgs(yarg: *mut ynl_parse_arg, cb: ynl_parse_cb_t, flags: c_int) -> c_int {
    let ys = (*yarg).ys;
    let len = recv((*ys).socket, (*ys).rx_buf, YNL_SOCKET_BUFFER_SIZE, flags);
    if len < 0 { if (flags & MSG_DONTWAIT) != 0 && errno == EAGAIN { return YNL_PARSE_CB_STOP; } return len as c_int; }
    let mut ret = YNL_PARSE_CB_STOP;
    let mut rem = len;
    while rem > 0 {
        let nlh = ((*ys).rx_buf as *const u8).add((len - rem) as usize) as *const nlmsghdr;
        if !nlmsg_ok(nlh, rem) { yerr_set((*yarg).ys, YNL_ERROR_INV_RESP, b"Invalid message or trailing data in the response.\0"); return YNL_PARSE_CB_ERROR; }
        if ((*nlh).nlmsg_flags & NLM_F_DUMP_INTR) != 0 { yerr_set((*yarg).ys, YNL_ERROR_DUMP_INTER, b"Dump interrupted / inconsistent, please retry.\0"); return YNL_PARSE_CB_ERROR; }
        match (*nlh).nlmsg_type {
            0 => { yerr_set((*yarg).ys, YNL_ERROR_INV_RESP, b"Invalid message type in the response.\0"); return YNL_PARSE_CB_ERROR; }
            NLMSG_NOOP => ret = YNL_PARSE_CB_OK,
            t if t >= NLMSG_OVERRUN && t < NLMSG_MIN_TYPE => ret = YNL_PARSE_CB_OK,
            NLMSG_ERROR => ret = ynl_cb_error(nlh, yarg),
            NLMSG_DONE => ret = ynl_cb_done(nlh, yarg),
            _ => ret = cb(nlh, yarg),
        }
        rem -= nlmsg_align((*nlh).nlmsg_len as usize) as isize;
    }
    ret
}

unsafe fn ynl_sock_read_msgs(yarg: *mut ynl_parse_arg, cb: ynl_parse_cb_t) -> c_int { __ynl_sock_read_msgs(yarg, cb, 0) }
unsafe fn ynl_recv_ack(ys: *mut ynl_sock, ret: c_int) -> c_int {
    let mut yarg = ynl_parse_arg { ys, data: ptr::null_mut(), rsp_policy: ptr::null() };
    if ret == 0 { yerr_set(ys, YNL_ERROR_EXPECT_ACK, b"Expecting an ACK but nothing received\0"); return -1; }
    ynl_sock_read_msgs(&mut yarg, ynl_cb_null)
}

unsafe fn ynl_get_family_info_mcast(ys: *mut ynl_sock, mcasts: *const nlattr) -> c_int {
    let mut attr = ynl_attr_data(mcasts) as *const nlattr;
    let end = ynl_attr_data_end(mcasts) as usize;
    while (attr as usize) < end { (*ys).n_mcast_groups += 1; attr = (attr as *const u8).add(nlmsg_align((*attr).nla_len as usize)) as *const nlattr; }
    if (*ys).n_mcast_groups == 0 { return 0; }
    (*ys).mcast_groups = calloc((*ys).n_mcast_groups as usize, size_of::<ynl_mcast_group>()) as *mut ynl_mcast_group;
    if (*ys).mcast_groups.is_null() { return YNL_PARSE_CB_ERROR; }
    let mut i = 0usize;
    let mut entry = ynl_attr_data(mcasts) as *const nlattr;
    while (entry as usize) < end {
        let mut a = ynl_attr_data(entry) as *const nlattr;
        let eend = ynl_attr_data_end(entry) as usize;
        while (a as usize) < eend {
            if ynl_attr_type(a) == CTRL_ATTR_MCAST_GRP_ID { (*(*ys).mcast_groups.add(i)).id = ynl_attr_get_u32(a); }
            if ynl_attr_type(a) == CTRL_ATTR_MCAST_GRP_NAME {
                strncpy((*(*ys).mcast_groups.add(i)).name.as_mut_ptr(), ynl_attr_get_str(a), GENL_NAMSIZ - 1);
                (*(*ys).mcast_groups.add(i)).name[GENL_NAMSIZ - 1] = 0;
            }
            a = (a as *const u8).add(nlmsg_align((*a).nla_len as usize)) as *const nlattr;
        }
        i += 1; entry = (entry as *const u8).add(nlmsg_align((*entry).nla_len as usize)) as *const nlattr;
    }
    0
}

unsafe extern "C" fn ynl_get_family_info_cb(nlh: *const nlmsghdr, yarg: *mut ynl_parse_arg) -> c_int {
    let ys = (*yarg).ys; let mut found_id = true;
    let mut attr = ynl_nlmsg_data_offset(nlh, size_of::<genlmsghdr>() as c_uint) as *const nlattr;
    let end = ynl_nlmsg_end_addr(nlh) as usize;
    while (attr as usize) < end {
        if ynl_attr_type(attr) == CTRL_ATTR_MCAST_GROUPS { if ynl_get_family_info_mcast(ys, attr) != 0 { return YNL_PARSE_CB_ERROR; } }
        if ynl_attr_type(attr) == CTRL_ATTR_FAMILY_ID {
            if ynl_attr_data_len(attr) as usize != size_of::<__u16>() { yerr_set(ys, YNL_ERROR_ATTR_INVALID, b"Invalid family ID\0"); return YNL_PARSE_CB_ERROR; }
            (*ys).family_id = ynl_attr_get_u16(attr) as c_int; found_id = true;
        }
        attr = (attr as *const u8).add(nlmsg_align((*attr).nla_len as usize)) as *const nlattr;
    }
    if !found_id { yerr_set(ys, YNL_ERROR_ATTR_MISSING, b"Family ID missing\0"); return YNL_PARSE_CB_ERROR; }
    YNL_PARSE_CB_OK
}

unsafe fn ynl_sock_read_family(ys: *mut ynl_sock, family_name: *const c_char) -> c_int {
    let mut yarg = ynl_parse_arg { ys, data: ptr::null_mut(), rsp_policy: ptr::null() };
    let nlh = ynl_gemsg_start_req(ys, GENL_ID_CTRL, CTRL_CMD_GETFAMILY, 1);
    ynl_attr_put_str(nlh, CTRL_ATTR_FAMILY_NAME, family_name);
    let mut err = ynl_msg_end(ys, nlh); if err < 0 { return err; }
    err = send((*ys).socket, nlh as *const c_void, (*nlh).nlmsg_len as usize, 0) as c_int;
    if err < 0 { yerr_code(ys, errno); yerr_msg(ys, b"failed to request socket family info\0"); return err; }
    err = ynl_sock_read_msgs(&mut yarg, ynl_get_family_info_cb);
    if err < 0 { free((*ys).mcast_groups as *mut c_void); yerr_code(ys, errno); yerr_msg(ys, b"failed to receive the socket family info - no such family?\0"); return err; }
    err = ynl_recv_ack(ys, err);
    if err < 0 { free((*ys).mcast_groups as *mut c_void); return err; }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ynl_sock_create(yf: *const ynl_family, yse: *mut ynl_error) -> *mut ynl_sock {
    let ys = malloc(size_of::<ynl_sock>() + 2 * YNL_SOCKET_BUFFER_SIZE) as *mut ynl_sock;
    if ys.is_null() { return ptr::null_mut(); }
    memset(ys as *mut c_void, 0, size_of::<ynl_sock>());
    (*ys).family = yf; (*ys).tx_buf = (*ys).raw_buf.as_mut_ptr() as *mut c_void; (*ys).rx_buf = (*ys).raw_buf.as_mut_ptr().add(YNL_SOCKET_BUFFER_SIZE) as *mut c_void; (*ys).ntf_last_next = &mut (*ys).ntf_first;
    let sock_type = if (*yf).is_classic { (*yf).classic_id } else { NETLINK_GENERIC };
    (*ys).socket = socket(AF_NETLINK, SOCK_RAW, sock_type);
    if (*ys).socket < 0 { if !yse.is_null() { (*yse).code = errno; } free(ys as *mut c_void); return ptr::null_mut(); }
    let one: c_int = 1;
    if setsockopt((*ys).socket, SOL_NETLINK, NETLINK_CAP_ACK, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t) != 0 { close((*ys).socket); free(ys as *mut c_void); return ptr::null_mut(); }
    if setsockopt((*ys).socket, SOL_NETLINK, NETLINK_EXT_ACK, &one as *const _ as *const c_void, size_of::<c_int>() as socklen_t) != 0 { close((*ys).socket); free(ys as *mut c_void); return ptr::null_mut(); }
    let mut addr: sockaddr_nl = zeroed(); addr.nl_family = AF_NETLINK as __u16;
    if bind((*ys).socket, &addr as *const _ as *const sockaddr, size_of::<sockaddr_nl>() as socklen_t) < 0 { close((*ys).socket); free(ys as *mut c_void); return ptr::null_mut(); }
    let mut addrlen = size_of::<sockaddr_nl>() as socklen_t; memset(&mut addr as *mut _ as *mut c_void, 0, size_of::<sockaddr_nl>());
    if getsockname((*ys).socket, &mut addr as *mut _ as *mut sockaddr, &mut addrlen) < 0 { close((*ys).socket); free(ys as *mut c_void); return ptr::null_mut(); }
    (*ys).portid = addr.nl_pid; (*ys).seq = random() as __u32;
    if (*yf).is_classic { (*ys).family_id = (*yf).classic_id; }
    else if ynl_sock_read_family(ys, (*yf).name) != 0 { if !yse.is_null() { memcpy(yse as *mut c_void, &(*ys).err as *const _ as *const c_void, size_of::<ynl_error>()); } close((*ys).socket); free(ys as *mut c_void); return ptr::null_mut(); }
    ys
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ynl_sock_destroy(ys: *mut ynl_sock) {
    close((*ys).socket);
    loop { let ntf = ynl_ntf_dequeue(ys); if ntf.is_null() { break; } ynl_ntf_free(ntf); }
    free((*ys).mcast_groups as *mut c_void); free(ys as *mut c_void);
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn ynl_ntf_free(ntf: *mut ynl_ntf_base_type) { ((*ntf).free)(ntf); }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ynl_subscribe(ys: *mut ynl_sock, grp_name: *const c_char) -> c_int {
    let mut i = 0;
    while i < (*ys).n_mcast_groups {
        if strcmp((*(*ys).mcast_groups.add(i as usize)).name.as_ptr(), grp_name) == 0 { break; }
        i += 1;
    }
    if i == (*ys).n_mcast_groups { snprintf((*ys).err.msg.as_mut_ptr(), (*ys).err.msg.len() - 1, b"Multicast group '%s' not found\0".as_ptr() as *const c_char, grp_name); (*ys).err.code = ENOENT; return -1; }
    let err = setsockopt((*ys).socket, SOL_NETLINK, NETLINK_ADD_MEMBERSHIP, &(*(*ys).mcast_groups.add(i as usize)).id as *const _ as *const c_void, size_of::<__u32>() as socklen_t);
    if err < 0 { yerr_code(ys, errno); yerr_msg(ys, b"Subscribing to multicast group failed\0"); return -1; }
    0
}

#[unsafe(no_mangle)] pub unsafe extern "C" fn ynl_socket_get_fd(ys: *mut ynl_sock) -> c_int { (*ys).socket }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ynl_ntf_dequeue(ys: *mut ynl_sock) -> *mut ynl_ntf_base_type {
    if (*ys).ntf_first.is_null() { return ptr::null_mut(); }
    let ntf = (*ys).ntf_first; (*ys).ntf_first = (*ntf).next;
    if (*ys).ntf_last_next == &mut (*ntf).next { (*ys).ntf_last_next = &mut (*ys).ntf_first; }
    ntf
}

unsafe fn ynl_ntf_parse(ys: *mut ynl_sock, nlh: *const nlmsghdr) -> c_int {
    let mut yarg = ynl_parse_arg { ys, data: ptr::null_mut(), rsp_policy: ptr::null() };
    let cmd: __u32 = if (*(*ys).family).is_classic { (*nlh).nlmsg_type as __u32 } else { (*(ynl_nlmsg_data(nlh) as *mut genlmsghdr)).cmd as __u32 };
    if cmd >= (*(*ys).family).ntf_info_size { return YNL_PARSE_CB_ERROR; }
    let info = (*(*ys).family).ntf_info.add(cmd as usize);
    if (*info).cb.is_none() { return YNL_PARSE_CB_ERROR; }
    let rsp = calloc(1, (*info).alloc_sz) as *mut ynl_ntf_base_type;
    if rsp.is_null() { return YNL_PARSE_CB_ERROR; }
    (*rsp).free = (*info).free; yarg.data = (*rsp).data.as_mut_ptr() as *mut c_void; yarg.rsp_policy = (*info).policy;
    let ret = ((*info).cb.unwrap())(nlh, &mut yarg);
    if ret <= YNL_PARSE_CB_STOP { ((*info).free)(rsp); return YNL_PARSE_CB_ERROR; }
    (*rsp).family = (*nlh).nlmsg_type as __u32; (*rsp).cmd = cmd;
    *(*ys).ntf_last_next = rsp; (*ys).ntf_last_next = &mut (*rsp).next;
    YNL_PARSE_CB_OK
}

unsafe extern "C" fn ynl_ntf_trampoline(nlh: *const nlmsghdr, yarg: *mut ynl_parse_arg) -> c_int { ynl_ntf_parse((*yarg).ys, nlh) }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ynl_ntf_check(ys: *mut ynl_sock) -> c_int {
    let mut yarg = ynl_parse_arg { ys, data: ptr::null_mut(), rsp_policy: ptr::null() };
    loop { let err = __ynl_sock_read_msgs(&mut yarg, ynl_ntf_trampoline, MSG_DONTWAIT); if err < 0 { return err; } if err <= 0 { break; } }
    0
}

#[unsafe(no_mangle)]
pub static mut YNL_LIST_END: *mut ynl_dump_list_type = 0xb4d123usize as *mut ynl_dump_list_type;

#[unsafe(no_mangle)] pub unsafe extern "C" fn ynl_error_unknown_notification(ys: *mut ynl_sock, cmd: __u8) { snprintf((*ys).err.msg.as_mut_ptr(), (*ys).err.msg.len() - 1, b"Unknown notification message type '%d'\0".as_ptr() as *const c_char, cmd as c_int); (*ys).err.code = YNL_ERROR_UNKNOWN_NTF; }
#[unsafe(no_mangle)] pub unsafe extern "C" fn ynl_error_parse(yarg: *mut ynl_parse_arg, msg: *const c_char) -> c_int { snprintf((*(*yarg).ys).err.msg.as_mut_ptr(), (*(*yarg).ys).err.msg.len() - 1, b"Error parsing response: %s\0".as_ptr() as *const c_char, msg); (*(*yarg).ys).err.code = YNL_ERROR_INV_RESP; YNL_PARSE_CB_ERROR }

unsafe fn ynl_check_alien(ys: *mut ynl_sock, nlh: *const nlmsghdr, rsp_cmd: __u32) -> c_int {
    if (*(*ys).family).is_classic {
        if (*nlh).nlmsg_type as __u32 != rsp_cmd { return ynl_ntf_parse(ys, nlh); }
    } else {
        if ynl_nlmsg_data_len(nlh) as usize  < size_of::<genlmsghdr>() { yerr_set(ys, YNL_ERROR_INV_RESP, b"Kernel responded with truncated message\0"); return -1; }
        let gehdr = ynl_nlmsg_data(nlh) as *mut genlmsghdr;
        if (*gehdr).cmd as __u32 != rsp_cmd { return ynl_ntf_parse(ys, nlh); }
    }
    0
}

unsafe extern "C" fn ynl_req_trampoline(nlh: *const nlmsghdr, yarg: *mut ynl_parse_arg) -> c_int {
    let yrs = yarg as *mut ynl_req_state;
    let ret = ynl_check_alien((*yrs).yarg.ys, nlh, (*yrs).rsp_cmd);
    if ret != 0 { return if ret < 0 { YNL_PARSE_CB_ERROR } else { YNL_PARSE_CB_OK }; }
    ((*yrs).cb)(nlh, &mut (*yrs).yarg)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ynl_exec(ys: *mut ynl_sock, req_nlh: *mut nlmsghdr, yrs: *mut ynl_req_state) -> c_int {
    let mut err = ynl_msg_end(ys, req_nlh); if err < 0 { return err; }
    err = send((*ys).socket, req_nlh as *const c_void, (*req_nlh).nlmsg_len as usize, 0) as c_int; if err < 0 { return err; }
    loop { err = ynl_sock_read_msgs(&mut (*yrs).yarg, ynl_req_trampoline); if err <= 0 { break; } }
    err
}

unsafe extern "C" fn ynl_dump_trampoline(nlh: *const nlmsghdr, data: *mut ynl_parse_arg) -> c_int {
    let ds = data as *mut ynl_dump_state;
    let ret = ynl_check_alien((*ds).yarg.ys, nlh, (*ds).rsp_cmd);
    if ret != 0 { return if ret < 0 { YNL_PARSE_CB_ERROR } else { YNL_PARSE_CB_OK }; }
    let obj = calloc(1, (*ds).alloc_sz) as *mut ynl_dump_list_type;
    if obj.is_null() { return YNL_PARSE_CB_ERROR; }
    if (*ds).first.is_null() { (*ds).first = obj; }
    if !(*ds).last.is_null() { (*(*ds).last).next = obj; }
    (*ds).last = obj;
    let mut yarg = ynl_parse_arg { ys: (*ds).yarg.ys, data: (*obj).data.as_mut_ptr() as *mut c_void, rsp_policy: (*ds).yarg.rsp_policy };
    ((*ds).cb)(nlh, &mut yarg)
}

unsafe fn ynl_dump_end(ds: *mut ynl_dump_state) -> *mut c_void {
    if (*ds).first.is_null() { return YNL_LIST_END as *mut c_void; }
    (*(*ds).last).next = YNL_LIST_END;
    (*ds).first as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ynl_exec_dump(ys: *mut ynl_sock, req_nlh: *mut nlmsghdr, yds: *mut ynl_dump_state) -> c_int {
    let mut err = ynl_msg_end(ys, req_nlh); if err < 0 { return err; }
    err = send((*ys).socket, req_nlh as *const c_void, (*req_nlh).nlmsg_len as usize, 0) as c_int; if err < 0 { return err; }
    loop { err = ynl_sock_read_msgs(&mut (*yds).yarg, ynl_dump_trampoline); if err < 0 { (*yds).first = ynl_dump_end(yds) as *mut ynl_dump_list_type; return -1; } if err <= 0 { break; } }
    (*yds).first = ynl_dump_end(yds) as *mut ynl_dump_list_type;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
