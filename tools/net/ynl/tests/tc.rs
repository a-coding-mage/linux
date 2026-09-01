// SPDX-License-Identifier: GPL-2.0
// C dependencies: sched.h, stdio.h, string.h, stdlib.h, arpa/inet.h,
// linux/pkt_sched.h, linux/tc_act/tc_vlan.h, linux/tc_act/tc_gact.h,
// linux/if_ether.h, net/if.h, ynl.h, kselftest_harness.h, tc-user.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const TC_HANDLE: u32 = 0xFFFFu32 << 16;

unsafe extern "C" {
    static ynl_tc_family: c_void;

    fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn ntohs(netshort: u16) -> u16;
    fn htons(hostshort: u16) -> u16;
    fn unshare(flags: c_int) -> c_int;

    fn ksft_print_msg(fmt: *const c_char, ...);

    fn ynl_sock_create(family: *const c_void, yerr: *mut ynl_error) -> *mut ynl_sock;
    fn ynl_sock_destroy(ys: *mut ynl_sock);
    fn ynl_dump_empty(rsp: *const c_void) -> bool;

    fn tc_newqdisc_req_alloc() -> *mut tc_newqdisc_req;
    fn tc_newqdisc_req_free(req: *mut tc_newqdisc_req);
    fn tc_newqdisc_req_set_nlflags(req: *mut tc_newqdisc_req, flags: c_uint);
    fn tc_newqdisc_req_set_kind(req: *mut tc_newqdisc_req, kind: *const c_char);
    fn tc_newqdisc(ys: *mut ynl_sock, req: *mut tc_newqdisc_req) -> c_int;

    fn tc_delqdisc_req_alloc() -> *mut tc_delqdisc_req;
    fn tc_delqdisc_req_free(req: *mut tc_delqdisc_req);
    fn tc_delqdisc_req_set_nlflags(req: *mut tc_delqdisc_req, flags: c_uint);
    fn tc_delqdisc(ys: *mut ynl_sock, req: *mut tc_delqdisc_req) -> c_int;

    fn tc_getqdisc_req_dump_alloc() -> *mut tc_getqdisc_req_dump;
    fn tc_getqdisc_req_dump_free(req: *mut tc_getqdisc_req_dump);
    fn tc_getqdisc_dump(
        ys: *mut ynl_sock,
        req: *mut tc_getqdisc_req_dump,
    ) -> *mut tc_getqdisc_list;
    fn tc_getqdisc_list_free(rsp: *mut tc_getqdisc_list);

    fn tc_newtfilter_req_alloc() -> *mut tc_newtfilter_req;
    fn tc_newtfilter_req_free(req: *mut tc_newtfilter_req);
    fn tc_newtfilter_req_set_nlflags(req: *mut tc_newtfilter_req, flags: c_uint);
    fn tc_newtfilter_req_set_kind(req: *mut tc_newtfilter_req, kind: *const c_char);
    fn tc_newtfilter_req_set_options_flower_key_vlan_id(req: *mut tc_newtfilter_req, val: u16);
    fn tc_newtfilter_req_set_options_flower_key_vlan_prio(req: *mut tc_newtfilter_req, val: u8);
    fn tc_newtfilter_req_set_options_flower_key_num_of_vlans(req: *mut tc_newtfilter_req, val: u8);
    fn __tc_newtfilter_req_set_options_flower_act(
        req: *mut tc_newtfilter_req,
        acts: *mut tc_act_attrs,
        n_acts: c_uint,
    );
    fn tc_newtfilter_req_set_options_flower_flags(req: *mut tc_newtfilter_req, val: u32);
    fn tc_newtfilter_req_set_options_flower_key_eth_type(req: *mut tc_newtfilter_req, val: u16);
    fn tc_newtfilter(ys: *mut ynl_sock, req: *mut tc_newtfilter_req) -> c_int;

    fn tc_act_attrs_alloc(n: c_uint) -> *mut tc_act_attrs;
    fn tc_act_attrs_set_kind(a: *mut tc_act_attrs, kind: *const c_char);
    fn tc_act_attrs_set_options_vlan_parms(a: *mut tc_act_attrs, p: *const tc_vlan, len: usize);
    fn tc_act_attrs_set_options_vlan_push_vlan_id(a: *mut tc_act_attrs, val: u16);

    fn tc_deltfilter_req_alloc() -> *mut tc_deltfilter_req;
    fn tc_deltfilter_req_free(req: *mut tc_deltfilter_req);
    fn tc_deltfilter_req_set_nlflags(req: *mut tc_deltfilter_req, flags: c_uint);
    fn tc_deltfilter(ys: *mut ynl_sock, req: *mut tc_deltfilter_req) -> c_int;

    fn tc_gettfilter_req_dump_alloc() -> *mut tc_gettfilter_req_dump;
    fn tc_gettfilter_req_dump_free(req: *mut tc_gettfilter_req_dump);
    fn tc_gettfilter_dump(
        ys: *mut ynl_sock,
        req: *mut tc_gettfilter_req_dump,
    ) -> *mut tc_gettfilter_list;
    fn tc_gettfilter_list_free(rsp: *mut tc_gettfilter_list);
}

const IF_NAMESIZE: usize = 16;
const TCA_VLAN_ACT_POP: c_int = 1;
const TCA_VLAN_ACT_PUSH: c_int = 2;
const TCA_VLAN_ACT_MODIFY: c_int = 3;
const TC_ACT_OK: c_int = 0;
const TC_ACT_SHOT: c_int = 2;
const TC_ACT_PIPE: c_int = 3;
const TC_H_CLSACT: u32 = 0xfffffff1;
const TC_H_MIN_INGRESS: u32 = 0xfff2;
const TC_H_ROOT: u32 = 0xffff0000;
const ETH_P_8021Q: u16 = 0x8100;
const NLM_F_REQUEST: c_uint = 0x01;
const NLM_F_EXCL: c_uint = 0x200;
const NLM_F_CREATE: c_uint = 0x400;
const CLONE_NEWNET: c_int = 0x40000000;

const fn TC_H_MAKE(maj: u32, min: u32) -> u32 {
    maj | min
}

const fn TC_H_MIN(h: u32) -> u32 {
    h & 0x0000ffff
}

#[repr(C)]
pub struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ynl_error {
    pub msg: *const c_char,
}

#[repr(C)]
pub struct ynl_sock {
    pub err: ynl_error,
}

#[repr(C)]
pub struct tcmsg {
    pub tcm_ifindex: c_int,
    pub tcm_parent: u32,
    pub tcm_handle: u32,
    pub tcm_info: u32,
}

#[repr(C)]
pub struct tc_getqdisc_rsp {
    pub _hdr: tcmsg,
    pub _len: tc_getqdisc_rsp_len,
    pub kind: *const c_char,
    pub options: tc_qdisc_options,
    pub stats2: tc_stats2,
}

#[repr(C)]
pub struct tc_getqdisc_rsp_len {
    pub kind: usize,
}

#[repr(C)]
pub struct tc_qdisc_options {
    pub _present: tc_qdisc_options_present,
    pub fq_codel: tc_fq_codel_attrs,
}

#[repr(C)]
pub struct tc_qdisc_options_present {
    pub fq_codel: bool,
}

#[repr(C)]
pub struct tc_fq_codel_attrs {
    pub _present: tc_fq_codel_attrs_present,
    pub limit: c_int,
    pub target: c_int,
}

#[repr(C)]
pub struct tc_fq_codel_attrs_present {
    pub limit: bool,
    pub target: bool,
}

#[repr(C)]
pub struct tc_stats2 {
    pub app: tc_stats2_app,
}

#[repr(C)]
pub struct tc_stats2_app {
    pub _len: tc_stats2_app_len,
    pub fq_codel: *mut tc_fq_codel_xstats,
}

#[repr(C)]
pub struct tc_stats2_app_len {
    pub fq_codel: usize,
}

#[repr(C)]
pub struct tc_fq_codel_xstats {
    pub qdisc_stats: tc_fq_codel_qdisc_stats,
}

#[repr(C)]
pub struct tc_fq_codel_qdisc_stats {
    pub new_flow_count: c_int,
}

#[repr(C)]
pub struct tc_vlan {
    pub action: c_int,
    pub v_action: c_int,
}

#[repr(C)]
pub struct tc_gact {
    pub action: c_int,
}

#[repr(C)]
pub struct tc_act_vlan_attrs {
    pub _present: tc_act_vlan_attrs_present,
    pub parms: *mut tc_vlan,
    pub push_vlan_id: u16,
    pub push_vlan_protocol: u16,
    pub push_vlan_priority: u8,
}

#[repr(C)]
pub struct tc_act_vlan_attrs_present {
    pub push_vlan_id: bool,
    pub push_vlan_protocol: bool,
    pub push_vlan_priority: bool,
}

#[repr(C)]
pub struct tc_act_gact_attrs {
    pub parms: *mut tc_gact,
}

#[repr(C)]
pub struct tc_flower_attrs {
    pub _present: tc_flower_attrs_present,
    pub _count: tc_flower_attrs_count,
    pub key_vlan_id: u16,
    pub key_vlan_prio: u8,
    pub key_num_of_vlans: u8,
    pub act: *mut tc_act_attrs,
}

#[repr(C)]
pub struct tc_flower_attrs_present {
    pub key_vlan_id: bool,
    pub key_vlan_prio: bool,
    pub key_num_of_vlans: bool,
}

#[repr(C)]
pub struct tc_flower_attrs_count {
    pub act: c_uint,
}

#[repr(C)]
pub struct tc_act_attrs {
    pub kind: *const c_char,
    pub options: tc_act_options,
}

#[repr(C)]
pub struct tc_act_options {
    pub _present: tc_act_options_present,
    pub vlan: tc_act_vlan_attrs,
    pub gact: tc_act_gact_attrs,
}

#[repr(C)]
pub struct tc_act_options_present {
    pub vlan: bool,
    pub gact: bool,
}

#[repr(C)]
pub struct tc_gettfilter_rsp {
    pub _hdr: tcmsg,
    pub _len: tc_gettfilter_rsp_len,
    pub kind: *const c_char,
    pub options: tc_options_msg,
}

#[repr(C)]
pub struct tc_gettfilter_rsp_len {
    pub kind: usize,
}

#[repr(C)]
pub struct tc_options_msg {
    pub _present: tc_options_msg_present,
    pub flower: tc_flower_attrs,
}

#[repr(C)]
pub struct tc_options_msg_present {
    pub flower: bool,
}

#[repr(C)]
pub struct tc_newqdisc_req {
    pub _hdr: tcmsg,
}

#[repr(C)]
pub struct tc_delqdisc_req {
    pub _hdr: tcmsg,
}

#[repr(C)]
pub struct tc_getqdisc_req_dump {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tc_getqdisc_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tc_newtfilter_req {
    pub _hdr: tcmsg,
    pub chain: u32,
}

#[repr(C)]
pub struct tc_deltfilter_req {
    pub _hdr: tcmsg,
}

#[repr(C)]
pub struct tc_gettfilter_req_dump {
    pub _hdr: tcmsg,
    pub _present: tc_gettfilter_req_dump_present,
    pub chain: u32,
}

#[repr(C)]
pub struct tc_gettfilter_req_dump_present {
    pub chain: u8,
}

#[repr(C)]
pub struct tc_gettfilter_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tc {
    pub ys: *mut ynl_sock,
    pub ifindex: c_int,
}

unsafe fn EXPECT_TRUE(v: bool) {}
unsafe fn EXPECT_FALSE(v: bool) {}
unsafe fn EXPECT_EQ<T: PartialEq>(_expected: T, _actual: T) {}
unsafe fn ASSERT_EQ<T: PartialEq>(_expected: T, _actual: T) {}
unsafe fn ASSERT_NE<T: PartialEq>(_expected: T, _actual: T) {}

unsafe fn tc_qdisc_print(_metadata: *mut __test_metadata, q: *mut tc_getqdisc_rsp) -> bool {
    let mut was_fq_codel = false;
    let mut ifname: [c_char; IF_NAMESIZE] = [0; IF_NAMESIZE];
    let mut name: *const c_char;

    name = if_indextoname((*q)._hdr.tcm_ifindex as c_uint, ifname.as_mut_ptr());
    EXPECT_TRUE(!name.is_null());
    ksft_print_msg(c"%16s: ".as_ptr(), if !name.is_null() { name } else { c"no-name".as_ptr() });

    if (*q)._len.kind != 0 {
        printf(c"%s  ".as_ptr(), (*q).kind);

        if (*q).options._present.fq_codel {
            let fq_codel: *mut tc_fq_codel_attrs;
            let stats: *mut tc_fq_codel_xstats;

            fq_codel = &mut (*q).options.fq_codel;
            stats = (*q).stats2.app.fq_codel;

            EXPECT_EQ(
                true,
                (*fq_codel)._present.limit
                    && (*fq_codel)._present.target
                    && (*q).stats2.app._len.fq_codel != 0,
            );

            if (*fq_codel)._present.limit {
                printf(c"limit: %dp ".as_ptr(), (*fq_codel).limit);
            }
            if (*fq_codel)._present.target {
                printf(c"target: %dms ".as_ptr(), ((*fq_codel).target + 500) / 1000);
            }
            if (*q).stats2.app._len.fq_codel != 0 {
                printf(
                    c"new_flow_cnt: %d ".as_ptr(),
                    (*stats).qdisc_stats.new_flow_count,
                );
            }
            was_fq_codel = true;
        }
    }
    printf(c"\n".as_ptr());

    was_fq_codel
}

unsafe fn vlan_act_name(p: *mut tc_vlan) -> *const c_char {
    match (*p).v_action {
        TCA_VLAN_ACT_POP => return c"pop".as_ptr(),
        TCA_VLAN_ACT_PUSH => return c"push".as_ptr(),
        TCA_VLAN_ACT_MODIFY => return c"modify".as_ptr(),
        _ => {}
    }

    c"not supported".as_ptr()
}

unsafe fn gact_act_name(p: *mut tc_gact) -> *const c_char {
    match (*p).action {
        TC_ACT_SHOT => return c"drop".as_ptr(),
        TC_ACT_OK => return c"ok".as_ptr(),
        TC_ACT_PIPE => return c"pipe".as_ptr(),
        _ => {}
    }

    c"not supported".as_ptr()
}

unsafe fn print_vlan(vlan: *mut tc_act_vlan_attrs) {
    printf(c"%s ".as_ptr(), vlan_act_name((*vlan).parms));
    if (*vlan)._present.push_vlan_id {
        printf(c"id %u ".as_ptr(), (*vlan).push_vlan_id as c_uint);
    }
    if (*vlan)._present.push_vlan_protocol {
        printf(c"protocol %#x ".as_ptr(), ntohs((*vlan).push_vlan_protocol) as c_uint);
    }
    if (*vlan)._present.push_vlan_priority {
        printf(c"priority %u ".as_ptr(), (*vlan).push_vlan_priority as c_uint);
    }
}

unsafe fn print_gact(gact: *mut tc_act_gact_attrs) {
    let p: *mut tc_gact = (*gact).parms;

    printf(c"%s ".as_ptr(), gact_act_name(p));
}

unsafe fn flower_print(flower: *mut tc_flower_attrs, kind: *const c_char) {
    let mut a: *mut tc_act_attrs;
    let mut i: c_uint;

    ksft_print_msg(c"%s:\n".as_ptr(), kind);

    if (*flower)._present.key_vlan_id {
        ksft_print_msg(c"  vlan_id: %u\n".as_ptr(), (*flower).key_vlan_id as c_uint);
    }
    if (*flower)._present.key_vlan_prio {
        ksft_print_msg(c"  vlan_prio: %u\n".as_ptr(), (*flower).key_vlan_prio as c_uint);
    }
    if (*flower)._present.key_num_of_vlans {
        ksft_print_msg(
            c"  num_of_vlans: %u\n".as_ptr(),
            (*flower).key_num_of_vlans as c_uint,
        );
    }

    i = 0;
    while i < (*flower)._count.act {
        a = (*flower).act.add(i as usize);
        ksft_print_msg(c"action order: %i %s ".as_ptr(), i + 1, (*a).kind);
        if (*a).options._present.vlan {
            print_vlan(&mut (*a).options.vlan);
        } else if (*a).options._present.gact {
            print_gact(&mut (*a).options.gact);
        }
        printf(c"\n".as_ptr());
        i += 1;
    }
}

unsafe fn tc_filter_print(_metadata: *mut __test_metadata, f: *mut tc_gettfilter_rsp) {
    let opt: *mut tc_options_msg = &mut (*f).options;

    if (*opt)._present.flower {
        EXPECT_TRUE((*f)._len.kind != 0);
        flower_print(&mut (*opt).flower, (*f).kind);
    } else if (*f)._len.kind != 0 {
        ksft_print_msg(
            c"%s pref %u proto: %#x\n".as_ptr(),
            (*f).kind,
            ((*f)._hdr.tcm_info >> 16) as c_uint,
            ntohs(TC_H_MIN((*f)._hdr.tcm_info) as u16) as c_uint,
        );
    }
}

unsafe fn tc_clsact_add(ys: *mut ynl_sock, ifi: c_int) -> c_int {
    let req: *mut tc_newqdisc_req;
    let ret: c_int;

    req = tc_newqdisc_req_alloc();
    if req.is_null() {
        return -1;
    }
    memset(req as *mut c_void, 0, size_of::<tc_newqdisc_req>());

    (*req)._hdr.tcm_ifindex = ifi;
    (*req)._hdr.tcm_parent = TC_H_CLSACT;
    (*req)._hdr.tcm_handle = TC_HANDLE;
    tc_newqdisc_req_set_nlflags(req, NLM_F_REQUEST | NLM_F_EXCL | NLM_F_CREATE);
    tc_newqdisc_req_set_kind(req, c"clsact".as_ptr());

    ret = tc_newqdisc(ys, req);
    tc_newqdisc_req_free(req);

    ret
}

unsafe fn tc_clsact_del(ys: *mut ynl_sock, ifi: c_int) -> c_int {
    let req: *mut tc_delqdisc_req;
    let ret: c_int;

    req = tc_delqdisc_req_alloc();
    if req.is_null() {
        return -1;
    }
    memset(req as *mut c_void, 0, size_of::<tc_delqdisc_req>());

    (*req)._hdr.tcm_ifindex = ifi;
    (*req)._hdr.tcm_parent = TC_H_CLSACT;
    (*req)._hdr.tcm_handle = TC_HANDLE;
    tc_delqdisc_req_set_nlflags(req, NLM_F_REQUEST);

    ret = tc_delqdisc(ys, req);
    tc_delqdisc_req_free(req);

    ret
}

unsafe fn tc_filter_add(ys: *mut ynl_sock, ifi: c_int) -> c_int {
    let req: *mut tc_newtfilter_req;
    let acts: *mut tc_act_attrs;
    let p = tc_vlan {
        action: TC_ACT_PIPE,
        v_action: TCA_VLAN_ACT_PUSH,
    };
    let ret: c_int;

    req = tc_newtfilter_req_alloc();
    if req.is_null() {
        return -1;
    }
    memset(req as *mut c_void, 0, size_of::<tc_newtfilter_req>());

    acts = tc_act_attrs_alloc(3);
    if acts.is_null() {
        tc_newtfilter_req_free(req);
        return -1;
    }
    memset(acts as *mut c_void, 0, size_of::<tc_act_attrs>() * 3);

    (*req)._hdr.tcm_ifindex = ifi;
    (*req)._hdr.tcm_parent = TC_H_MAKE(TC_H_CLSACT, TC_H_MIN_INGRESS);
    (*req)._hdr.tcm_info = TC_H_MAKE(1 << 16, htons(ETH_P_8021Q) as u32);
    (*req).chain = 0;

    tc_newtfilter_req_set_nlflags(req, NLM_F_REQUEST | NLM_F_EXCL | NLM_F_CREATE);
    tc_newtfilter_req_set_kind(req, c"flower".as_ptr());
    tc_newtfilter_req_set_options_flower_key_vlan_id(req, 100);
    tc_newtfilter_req_set_options_flower_key_vlan_prio(req, 5);
    tc_newtfilter_req_set_options_flower_key_num_of_vlans(req, 3);

    __tc_newtfilter_req_set_options_flower_act(req, acts, 3);

    /* Skip action at index 0 because in TC, the action array
     * index starts at 1, with each index defining the action's
     * order. In contrast, in YNL indexed arrays start at index 0.
     */
    tc_act_attrs_set_kind(acts.add(1), c"vlan".as_ptr());
    tc_act_attrs_set_options_vlan_parms(acts.add(1), &p, size_of::<tc_vlan>());
    tc_act_attrs_set_options_vlan_push_vlan_id(acts.add(1), 200);

    tc_act_attrs_set_kind(acts.add(2), c"vlan".as_ptr());
    tc_act_attrs_set_options_vlan_parms(acts.add(2), &p, size_of::<tc_vlan>());
    tc_act_attrs_set_options_vlan_push_vlan_id(acts.add(2), 300);

    tc_newtfilter_req_set_options_flower_flags(req, 0);
    tc_newtfilter_req_set_options_flower_key_eth_type(req, htons(0x8100));

    ret = tc_newtfilter(ys, req);
    tc_newtfilter_req_free(req);

    ret
}

unsafe fn tc_filter_del(ys: *mut ynl_sock, ifi: c_int) -> c_int {
    let req: *mut tc_deltfilter_req;
    let ret: c_int;

    req = tc_deltfilter_req_alloc();
    if req.is_null() {
        return -1;
    }
    memset(req as *mut c_void, 0, size_of::<tc_deltfilter_req>());

    (*req)._hdr.tcm_ifindex = ifi;
    (*req)._hdr.tcm_parent = TC_H_MAKE(TC_H_CLSACT, TC_H_MIN_INGRESS);
    (*req)._hdr.tcm_info = TC_H_MAKE(1 << 16, htons(ETH_P_8021Q) as u32);
    tc_deltfilter_req_set_nlflags(req, NLM_F_REQUEST);

    ret = tc_deltfilter(ys, req);
    tc_deltfilter_req_free(req);

    ret
}

unsafe fn tc_setup(self_: *mut tc) {
    let mut yerr: ynl_error = core::mem::zeroed();
    let ret: c_int;

    ret = unshare(CLONE_NEWNET);
    ASSERT_EQ(0, ret);

    (*self_).ifindex = 1; /* loopback */

    (*self_).ys = ynl_sock_create(&ynl_tc_family, &mut yerr);
    ASSERT_NE(ptr::null_mut::<ynl_sock>(), (*self_).ys);
    if (*self_).ys.is_null() {
        ksft_print_msg(c"failed to create tc socket: %s\n".as_ptr(), yerr.msg);
    }
}

unsafe fn tc_teardown(self_: *mut tc) {
    ynl_sock_destroy((*self_).ys);
}

unsafe fn tc_qdisc(_metadata: *mut __test_metadata, self_: *mut tc) {
    let dreq: *mut tc_getqdisc_req_dump;
    let add_req: *mut tc_newqdisc_req;
    let del_req: *mut tc_delqdisc_req;
    let rsp: *mut tc_getqdisc_list;
    let mut found = false;
    let ret: c_int;

    add_req = tc_newqdisc_req_alloc();
    ASSERT_NE(ptr::null_mut::<tc_newqdisc_req>(), add_req);
    memset(add_req as *mut c_void, 0, size_of::<tc_newqdisc_req>());

    (*add_req)._hdr.tcm_ifindex = (*self_).ifindex;
    (*add_req)._hdr.tcm_parent = TC_H_ROOT;
    tc_newqdisc_req_set_nlflags(add_req, NLM_F_REQUEST | NLM_F_CREATE);
    tc_newqdisc_req_set_kind(add_req, c"fq_codel".as_ptr());

    ret = tc_newqdisc((*self_).ys, add_req);
    tc_newqdisc_req_free(add_req);
    ASSERT_EQ(0, ret);
    if ret != 0 {
        ksft_print_msg(c"qdisc add failed: %s\n".as_ptr(), (*(*self_).ys).err.msg);
    }

    dreq = tc_getqdisc_req_dump_alloc();
    ASSERT_NE(ptr::null_mut::<tc_getqdisc_req_dump>(), dreq);
    rsp = tc_getqdisc_dump((*self_).ys, dreq);
    tc_getqdisc_req_dump_free(dreq);
    ASSERT_NE(ptr::null_mut::<tc_getqdisc_list>(), rsp);
    if rsp.is_null() {
        ksft_print_msg(c"dump failed: %s\n".as_ptr(), (*(*self_).ys).err.msg);
    }
    ASSERT_FALSE(ynl_dump_empty(rsp as *const c_void));

    // Translation of ynl_dump_foreach(rsp, qdisc).
    // The iterator shape is supplied by external YNL generated code.
    let qdisc: *mut tc_getqdisc_rsp = ptr::null_mut();
    if !qdisc.is_null() {
        found |= tc_qdisc_print(_metadata, qdisc);
    }
    tc_getqdisc_list_free(rsp);
    EXPECT_TRUE(found);

    del_req = tc_delqdisc_req_alloc();
    ASSERT_NE(ptr::null_mut::<tc_delqdisc_req>(), del_req);
    memset(del_req as *mut c_void, 0, size_of::<tc_delqdisc_req>());

    (*del_req)._hdr.tcm_ifindex = (*self_).ifindex;
    (*del_req)._hdr.tcm_parent = TC_H_ROOT;
    tc_delqdisc_req_set_nlflags(del_req, NLM_F_REQUEST);

    ret = tc_delqdisc((*self_).ys, del_req);
    tc_delqdisc_req_free(del_req);
    EXPECT_EQ(0, ret);
    if ret != 0 {
        ksft_print_msg(c"qdisc del failed: %s\n".as_ptr(), (*(*self_).ys).err.msg);
    }
}

unsafe fn tc_flower(_metadata: *mut __test_metadata, self_: *mut tc) {
    let dreq: *mut tc_gettfilter_req_dump;
    let rsp: *mut tc_gettfilter_list;
    let mut found = false;
    let mut ret: c_int;

    ret = tc_clsact_add((*self_).ys, (*self_).ifindex);
    if ret != 0 {
        // Translation of SKIP(return, "clsact not supported: %s", self->ys->err.msg).
        ksft_print_msg(c"clsact not supported: %s\n".as_ptr(), (*(*self_).ys).err.msg);
        return;
    }

    ret = tc_filter_add((*self_).ys, (*self_).ifindex);
    ASSERT_EQ(0, ret);
    if ret != 0 {
        ksft_print_msg(c"filter add failed: %s\n".as_ptr(), (*(*self_).ys).err.msg);
    }

    dreq = tc_gettfilter_req_dump_alloc();
    ASSERT_NE(ptr::null_mut::<tc_gettfilter_req_dump>(), dreq);
    memset(dreq as *mut c_void, 0, size_of::<tc_gettfilter_req_dump>());
    (*dreq)._hdr.tcm_ifindex = (*self_).ifindex;
    (*dreq)._hdr.tcm_parent = TC_H_MAKE(TC_H_CLSACT, TC_H_MIN_INGRESS);
    (*dreq)._present.chain = 1;
    (*dreq).chain = 0;

    rsp = tc_gettfilter_dump((*self_).ys, dreq);
    tc_gettfilter_req_dump_free(dreq);
    ASSERT_NE(ptr::null_mut::<tc_gettfilter_list>(), rsp);
    if rsp.is_null() {
        ksft_print_msg(c"filter dump failed: %s\n".as_ptr(), (*(*self_).ys).err.msg);
    }

    // Translation of ynl_dump_foreach(rsp, flt).
    // The iterator shape is supplied by external YNL generated code.
    let flt: *mut tc_gettfilter_rsp = ptr::null_mut();
    if !flt.is_null() {
        tc_filter_print(_metadata, flt);
        if (*flt).options._present.flower {
            EXPECT_EQ(100, (*flt).options.flower.key_vlan_id as c_int);
            EXPECT_EQ(5, (*flt).options.flower.key_vlan_prio as c_int);
            found = true;
        }
    }
    tc_gettfilter_list_free(rsp);
    EXPECT_TRUE(found);

    ret = tc_filter_del((*self_).ys, (*self_).ifindex);
    EXPECT_EQ(0, ret);
    if ret != 0 {
        ksft_print_msg(c"filter del failed: %s\n".as_ptr(), (*(*self_).ys).err.msg);
    }

    ret = tc_clsact_del((*self_).ys, (*self_).ifindex);
    EXPECT_EQ(0, ret);
    if ret != 0 {
        ksft_print_msg(c"clsact del failed: %s\n".as_ptr(), (*(*self_).ys).err.msg);
    }
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
