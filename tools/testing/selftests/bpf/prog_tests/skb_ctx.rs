// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, <network_helpers.h>

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

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
    pub napi_id: u32,
    pub family: u32,
    pub remote_ip4: u32,
    pub local_ip4: u32,
    pub remote_ip6: [u32; 4],
    pub local_ip6: [u32; 4],
    pub remote_port: u32,
    pub local_port: u32,
    pub data_meta: u32,
    pub flow_keys: *mut bpf_flow_keys,
    pub tstamp: u64,
    pub wire_len: u32,
    pub gso_segs: u32,
    pub sk: *mut bpf_sock,
    pub gso_size: u32,
    pub tstamp_type: u8,
    pub hwtstamp: u64,
}

#[repr(C)]
pub struct bpf_flow_keys {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_sock {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *const c_void,
    pub data_out: *mut c_void,
    pub data_size_in: u32,
    pub data_size_out: u32,
    pub ctx_in: *const c_void,
    pub ctx_out: *mut c_void,
    pub ctx_size_in: u32,
    pub ctx_size_out: u32,
    pub retval: u32,
    pub repeat: c_int,
    pub duration: u32,
    pub flags: u32,
    pub cpu: u32,
    pub batch_size: u32,
}

const BPF_PROG_TYPE_SCHED_CLS: c_int = 3;

unsafe extern "C" {
    static pkt_v4: [u8; 0];

    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        pobj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: u64, expected: u64, name: *const c_char) -> bool;
}

unsafe fn bpf_test_run_opts_default() -> bpf_test_run_opts {
    bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: ptr::addr_of!(pkt_v4) as *const c_void,
        data_out: ptr::null_mut(),
        data_size_in: size_of_val_raw(ptr::addr_of!(pkt_v4)) as u32,
        data_size_out: 0,
        ctx_in: ptr::null(),
        ctx_out: ptr::null_mut(),
        ctx_size_in: 0,
        ctx_size_out: 0,
        retval: 0,
        repeat: 0,
        duration: 0,
        flags: 0,
        cpu: 0,
        batch_size: 0,
    }
}

unsafe fn size_of_val_raw<T: ?Sized>(_: *const T) -> usize {
    size_of::<[u8; 0]>()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_skb_ctx() {
    let mut skb = __sk_buff {
        len: 0,
        pkt_type: 0,
        mark: 9,
        queue_mapping: 0,
        protocol: 0,
        vlan_present: 0,
        vlan_tci: 0,
        vlan_proto: 0,
        priority: 6,
        ingress_ifindex: 11,
        ifindex: 1,
        tc_index: 0,
        cb: [1, 2, 3, 4, 5],
        hash: 0,
        tc_classid: 0,
        data: 0,
        data_end: 0,
        napi_id: 0,
        family: 0,
        remote_ip4: 0,
        local_ip4: 0,
        remote_ip6: [0; 4],
        local_ip6: [0; 4],
        remote_port: 0,
        local_port: 0,
        data_meta: 0,
        flow_keys: ptr::null_mut(),
        tstamp: 7,
        wire_len: 100,
        gso_segs: 8,
        sk: ptr::null_mut(),
        gso_size: 10,
        tstamp_type: 0,
        hwtstamp: 11,
    };
    let mut tattr = bpf_test_run_opts_default();
    tattr.ctx_in = ptr::addr_of!(skb) as *const c_void;
    tattr.ctx_size_in = size_of::<__sk_buff>() as u32;
    tattr.ctx_out = ptr::addr_of_mut!(skb) as *mut c_void;
    tattr.ctx_size_out = size_of::<__sk_buff>() as u32;

    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut prog_fd: c_int = 0;
    let mut i: c_int;

    let mut err = bpf_prog_test_load(
        c"./test_skb_ctx.bpf.o".as_ptr(),
        BPF_PROG_TYPE_SCHED_CLS,
        ptr::addr_of_mut!(obj),
        ptr::addr_of_mut!(prog_fd),
    );
    if !ASSERT_OK(err, c"load".as_ptr()) {
        return;
    }

    /* ctx_in != NULL, ctx_size_in == 0 */

    tattr.ctx_size_in = 0;
    err = bpf_prog_test_run_opts(prog_fd, ptr::addr_of_mut!(tattr));
    ASSERT_NEQ(err, 0, c"ctx_size_in".as_ptr());
    tattr.ctx_size_in = size_of::<__sk_buff>() as u32;

    /* ctx_out != NULL, ctx_size_out == 0 */

    tattr.ctx_size_out = 0;
    err = bpf_prog_test_run_opts(prog_fd, ptr::addr_of_mut!(tattr));
    ASSERT_NEQ(err, 0, c"ctx_size_out".as_ptr());
    tattr.ctx_size_out = size_of::<__sk_buff>() as u32;

    /* non-zero [len, tc_index] fields should be rejected*/

    skb.len = 1;
    err = bpf_prog_test_run_opts(prog_fd, ptr::addr_of_mut!(tattr));
    ASSERT_NEQ(err, 0, c"len".as_ptr());
    skb.len = 0;

    skb.tc_index = 1;
    err = bpf_prog_test_run_opts(prog_fd, ptr::addr_of_mut!(tattr));
    ASSERT_NEQ(err, 0, c"tc_index".as_ptr());
    skb.tc_index = 0;

    /* non-zero [hash, sk] fields should be rejected */

    skb.hash = 1;
    err = bpf_prog_test_run_opts(prog_fd, ptr::addr_of_mut!(tattr));
    ASSERT_NEQ(err, 0, c"hash".as_ptr());
    skb.hash = 0;

    skb.sk = 1 as *mut bpf_sock;
    err = bpf_prog_test_run_opts(prog_fd, ptr::addr_of_mut!(tattr));
    ASSERT_NEQ(err, 0, c"sk".as_ptr());
    skb.sk = ptr::null_mut();

    err = bpf_prog_test_run_opts(prog_fd, ptr::addr_of_mut!(tattr));
    ASSERT_OK(err, c"test_run".as_ptr());
    ASSERT_OK(tattr.retval as c_int, c"test_run retval".as_ptr());
    ASSERT_EQ(
        tattr.ctx_size_out as u64,
        size_of::<__sk_buff>() as u64,
        c"ctx_size_out".as_ptr(),
    );

    i = 0;
    while i < 5 {
        ASSERT_EQ(skb.cb[i as usize] as u64, (i + 2) as u64, c"ctx_out_cb".as_ptr());
        i += 1;
    }
    ASSERT_EQ(skb.priority as u64, 7, c"ctx_out_priority".as_ptr());
    ASSERT_EQ(skb.ifindex as u64, 1, c"ctx_out_ifindex".as_ptr());
    ASSERT_EQ(
        skb.ingress_ifindex as u64,
        11,
        c"ctx_out_ingress_ifindex".as_ptr(),
    );
    ASSERT_EQ(skb.tstamp, 8, c"ctx_out_tstamp".as_ptr());
    ASSERT_EQ(skb.mark as u64, 10, c"ctx_out_mark".as_ptr());

    bpf_object__close(obj);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
