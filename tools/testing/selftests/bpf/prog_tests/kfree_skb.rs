// SPDX-License-Identifier: GPL-2.0
// C source dependencies: test_progs.h, network_helpers.h, kfree_skb.skel.h

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;

const ETH_P_IPV6: c_int = 0x86DD;
const BPF_PROG_TYPE_SCHED_CLS: c_int = 3;

#[repr(C)]
struct meta {
    ifindex: c_int,
    cb32_0: __u32,
    cb8_0: __u8,
}

#[repr(C)]
union cb_union {
    cb32: [__u32; 5],
    cb8: [__u8; 20],
}

static mut cb: cb_union = cb_union {
    cb32: [0x81828384, 0, 0, 0, 0],
};

#[repr(C)]
struct ethhdr {
    h_dest: [__u8; 6],
    h_source: [__u8; 6],
    h_proto: __u16,
}

#[repr(C)]
struct ipv6hdr {
    _private: [__u8; 6],
    nexthdr: __u8,
    _private2: [__u8; 33],
}

#[repr(C)]
struct tcphdr {
    _private: [__u8; 12],
    doff: __u8,
}

#[repr(C)]
struct ipv6_packet {
    eth: ethhdr,
    iph: ipv6hdr,
    tcp: tcphdr,
}

#[repr(C)]
struct __sk_buff {
    cb: [__u32; 5],
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct perf_buffer {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct kfree_skb_progs {
    trace_kfree_skb: *mut bpf_program,
    fentry_eth_type_trans: *mut bpf_program,
    fexit_eth_type_trans: *mut bpf_program,
}

#[repr(C)]
struct kfree_skb_links {
    trace_kfree_skb: *mut bpf_link,
    fentry_eth_type_trans: *mut bpf_link,
    fexit_eth_type_trans: *mut bpf_link,
}

#[repr(C)]
struct kfree_skb_maps {
    perf_buf_map: *mut bpf_map,
    bss: *mut bpf_map,
}

#[repr(C)]
struct kfree_skb {
    progs: kfree_skb_progs,
    links: kfree_skb_links,
    maps: kfree_skb_maps,
}

#[repr(C)]
struct bpf_test_run_opts {
    data_in: *const c_void,
    data_size_in: __u32,
    ctx_in: *mut c_void,
    ctx_size_in: __u32,
    retval: __u32,
}

unsafe extern "C" {
    static pkt_v6: ipv6_packet;
    static mut errno: c_int;

    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn CHECK_FAIL(condition: bool);
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_TRUE(condition: bool, name: *const c_char) -> bool;

    fn htons(hostshort: __u16) -> __u16;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn bpf_prog_test_load(
        file: *const c_char,
        prog_type: c_int,
        obj: *mut *mut bpf_object,
        prog_fd: *mut c_int,
    ) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__attach_raw_tracepoint(
        prog: *mut bpf_program,
        name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_program__attach_trace(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);

    fn perf_buffer__new(
        map_fd: c_int,
        page_cnt: usize,
        sample_cb: Option<unsafe extern "C" fn(*mut c_void, c_int, *mut c_void, __u32)>,
        lost_cb: Option<unsafe extern "C" fn(*mut c_void, c_int, __u64)>,
        ctx: *mut c_void,
        opts: *const c_void,
    ) -> *mut perf_buffer;
    fn perf_buffer__poll(pb: *mut perf_buffer, timeout_ms: c_int) -> c_int;
    fn perf_buffer__free(pb: *mut perf_buffer);

    fn kfree_skb__open_and_load() -> *mut kfree_skb;
    fn kfree_skb__destroy(skel: *mut kfree_skb);
}

type __u64 = u64;

unsafe extern "C" fn on_sample(ctx: *mut c_void, _cpu: c_int, data: *mut c_void, size: __u32) {
    let meta = data as *mut meta;
    let pkt_v6 = (data as *mut u8).add(size_of::<meta>()) as *mut ipv6_packet;
    let _duration: c_int = 0;

    if CHECK(
        size != (72 + size_of::<meta>()) as __u32,
        c"check_size".as_ptr(),
        c"size %u != %zu\n".as_ptr(),
        size,
        72 + size_of::<meta>(),
    ) {
        return;
    }
    if CHECK(
        (*meta).ifindex != 1,
        c"check_meta_ifindex".as_ptr(),
        c"meta->ifindex = %d\n".as_ptr(),
        (*meta).ifindex,
    ) {
        /* spurious kfree_skb not on loopback device */
        return;
    }
    if CHECK(
        (*meta).cb8_0 != cb.cb8[0],
        c"check_cb8_0".as_ptr(),
        c"cb8_0 %x != %x\n".as_ptr(),
        (*meta).cb8_0 as c_int,
        cb.cb8[0] as c_int,
    ) {
        return;
    }
    if CHECK(
        (*meta).cb32_0 != cb.cb32[0],
        c"check_cb32_0".as_ptr(),
        c"cb32_0 %x != %x\n".as_ptr(),
        (*meta).cb32_0,
        cb.cb32[0],
    ) {
        return;
    }
    if CHECK(
        (*pkt_v6).eth.h_proto != htons(ETH_P_IPV6 as __u16),
        c"check_eth".as_ptr(),
        c"h_proto %x\n".as_ptr(),
        (*pkt_v6).eth.h_proto as c_int,
    ) {
        return;
    }
    if CHECK(
        (*pkt_v6).iph.nexthdr != 6,
        c"check_ip".as_ptr(),
        c"iph.nexthdr %x\n".as_ptr(),
        (*pkt_v6).iph.nexthdr as c_int,
    ) {
        return;
    }
    if CHECK(
        (*pkt_v6).tcp.doff != 5,
        c"check_tcp".as_ptr(),
        c"tcp.doff %x\n".as_ptr(),
        (*pkt_v6).tcp.doff as c_int,
    ) {
        return;
    }

    *(ctx as *mut bool) = true;
}

/* TODO: fix kernel panic caused by this test in parallel mode */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_kfree_skb() {
    let mut skb: __sk_buff = zeroed();
    /*
     * LIBBPF_OPTS(bpf_test_run_opts, topts,
     *     .data_in = &pkt_v6,
     *     .data_size_in = sizeof(pkt_v6),
     *     .ctx_in = &skb,
     *     .ctx_size_in = sizeof(skb),
     * );
     */
    let mut topts = bpf_test_run_opts {
        data_in: (&pkt_v6 as *const ipv6_packet).cast(),
        data_size_in: size_of::<ipv6_packet>() as __u32,
        ctx_in: (&mut skb as *mut __sk_buff).cast(),
        ctx_size_in: size_of::<__sk_buff>() as __u32,
        retval: 0,
    };
    let mut skel: *mut kfree_skb = ptr::null_mut();
    let mut link: *mut bpf_link;
    let mut obj: *mut bpf_object = ptr::null_mut();
    let mut pb: *mut perf_buffer = ptr::null_mut();
    let mut err: c_int;
    let mut prog_fd: c_int = 0;
    let mut passed: bool = false;
    let _duration: __u32 = 0;
    let zero: c_int = 0;
    let mut test_ok: [bool; 2] = [false; 2];

    err = bpf_prog_test_load(
        c"./test_pkt_access.bpf.o".as_ptr(),
        BPF_PROG_TYPE_SCHED_CLS,
        &mut obj,
        &mut prog_fd,
    );
    if CHECK(
        err != 0,
        c"prog_load sched cls".as_ptr(),
        c"err %d errno %d\n".as_ptr(),
        err,
        errno,
    ) {
        return;
    }

    skel = kfree_skb__open_and_load();
    if !ASSERT_OK_PTR(skel.cast(), c"kfree_skb_skel".as_ptr()) {
        goto_close_prog(skel, obj, pb);
        return;
    }

    link = bpf_program__attach_raw_tracepoint((*skel).progs.trace_kfree_skb, ptr::null());
    if !ASSERT_OK_PTR(link.cast(), c"attach_raw_tp".as_ptr()) {
        goto_close_prog(skel, obj, pb);
        return;
    }
    (*skel).links.trace_kfree_skb = link;

    link = bpf_program__attach_trace((*skel).progs.fentry_eth_type_trans);
    if !ASSERT_OK_PTR(link.cast(), c"attach fentry".as_ptr()) {
        goto_close_prog(skel, obj, pb);
        return;
    }
    (*skel).links.fentry_eth_type_trans = link;

    link = bpf_program__attach_trace((*skel).progs.fexit_eth_type_trans);
    if !ASSERT_OK_PTR(link.cast(), c"attach fexit".as_ptr()) {
        goto_close_prog(skel, obj, pb);
        return;
    }
    (*skel).links.fexit_eth_type_trans = link;

    /* set up perf buffer */
    pb = perf_buffer__new(
        bpf_map__fd((*skel).maps.perf_buf_map),
        1,
        Some(on_sample),
        None,
        (&mut passed as *mut bool).cast(),
        ptr::null(),
    );
    if !ASSERT_OK_PTR(pb.cast(), c"perf_buf__new".as_ptr()) {
        goto_close_prog(skel, obj, pb);
        return;
    }

    memcpy(
        skb.cb.as_mut_ptr().cast(),
        (&cb as *const cb_union).cast(),
        size_of::<cb_union>(),
    );
    err = bpf_prog_test_run_opts(prog_fd, &mut topts);
    ASSERT_OK(err, c"ipv6 test_run".as_ptr());
    ASSERT_OK(topts.retval as c_int, c"ipv6 test_run retval".as_ptr());

    /* read perf buffer */
    err = perf_buffer__poll(pb, 100);
    if CHECK(
        err < 0,
        c"perf_buffer__poll".as_ptr(),
        c"err %d\n".as_ptr(),
        err,
    ) {
        goto_close_prog(skel, obj, pb);
        return;
    }

    /*
     * make sure kfree_skb program was triggered
     * and it sent expected skb into ring buffer
     */
    ASSERT_TRUE(passed, c"passed".as_ptr());

    err = bpf_map_lookup_elem(
        bpf_map__fd((*skel).maps.bss),
        (&zero as *const c_int).cast(),
        test_ok.as_mut_ptr().cast(),
    );
    if CHECK(
        err != 0,
        c"get_result".as_ptr(),
        c"failed to get output data: %d\n".as_ptr(),
        err,
    ) {
        goto_close_prog(skel, obj, pb);
        return;
    }

    CHECK_FAIL(!test_ok[0] || !test_ok[1]);

    goto_close_prog(skel, obj, pb);
}

unsafe fn goto_close_prog(skel: *mut kfree_skb, obj: *mut bpf_object, pb: *mut perf_buffer) {
    perf_buffer__free(pb);
    bpf_object__close(obj);
    kfree_skb__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
