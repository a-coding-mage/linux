// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of testing/selftests/bpf/prog_tests/xdp_devmap_attach.c.
// C include dependencies preserved as external declarations below:
// arpa/inet.h, uapi/linux/bpf.h, linux/if_link.h, network_helpers.h,
// net/if.h, test_progs.h, and the generated libbpf skeleton headers.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, size_of_val};
use core::ptr;

const IFINDEX_LO: c_int = 1;
const TEST_NS: &[u8] = b"devmap_attach_ns\0";
const ETH_HLEN: usize = 14;
const XDP_FLAGS_SKB_MODE: c_uint = 1 << 1;
const XDP_FLAGS_DRV_MODE: c_uint = 1 << 2;
const BPF_F_TEST_XDP_LIVE_FRAMES: c_uint = 1 << 1;
const BPF_XDP_DEVMAP: bpf_attach_type = 13;

#[allow(non_camel_case_types)]
type __u32 = u32;
#[allow(non_camel_case_types)]
type bpf_attach_type = c_uint;

#[repr(C)]
struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_prog_info {
    id: __u32,
}

#[repr(C)]
union bpf_devmap_val_bpf_prog {
    fd: c_int,
    id: __u32,
}

#[repr(C)]
struct bpf_devmap_val {
    ifindex: __u32,
    bpf_prog: bpf_devmap_val_bpf_prog,
}

#[repr(C)]
struct bpf_test_run_opts {
    sz: usize,
    data_in: *mut c_void,
    data_size_in: __u32,
    flags: __u32,
    repeat: __u32,
}

#[repr(C)]
struct test_xdp_with_devmap_helpers {
    progs: test_xdp_with_devmap_helpers_progs,
    maps: test_xdp_with_devmap_helpers_maps,
}

#[repr(C)]
struct test_xdp_with_devmap_helpers_progs {
    xdp_redir_prog: *mut bpf_program,
    xdp_dummy_dm: *mut bpf_program,
    xdp_dummy_prog: *mut bpf_program,
    xdp_dummy_dm_frags: *mut bpf_program,
}

#[repr(C)]
struct test_xdp_with_devmap_helpers_maps {
    dm_ports: *mut bpf_map,
}

#[repr(C)]
struct test_xdp_devmap_helpers {
    _private: [u8; 0],
}

#[repr(C)]
struct test_xdp_devmap_tailcall {
    progs: test_xdp_devmap_tailcall_progs,
}

#[repr(C)]
struct test_xdp_devmap_tailcall_progs {
    xdp_devmap: *mut bpf_program,
    xdp_entry: *mut bpf_program,
}

#[repr(C)]
struct test_xdp_with_devmap_frags_helpers {
    progs: test_xdp_with_devmap_frags_helpers_progs,
    maps: test_xdp_with_devmap_frags_helpers_maps,
}

#[repr(C)]
struct test_xdp_with_devmap_frags_helpers_progs {
    xdp_dummy_dm_frags: *mut bpf_program,
    xdp_dummy_dm: *mut bpf_program,
}

#[repr(C)]
struct test_xdp_with_devmap_frags_helpers_maps {
    dm_ports: *mut bpf_map,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn kern_sync_rcu();
    fn if_nametoindex(ifname: *const c_char) -> c_uint;

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_program__set_expected_attach_type(prog: *mut bpf_program, attach_type: bpf_attach_type);
    fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: c_uint, opts: *const c_void) -> c_int;
    fn bpf_xdp_detach(ifindex: c_int, flags: c_uint, opts: *const c_void) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: u64) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn test_xdp_with_devmap_helpers__open_and_load() -> *mut test_xdp_with_devmap_helpers;
    fn test_xdp_with_devmap_helpers__destroy(skel: *mut test_xdp_with_devmap_helpers);
    fn test_xdp_devmap_helpers__open_and_load() -> *mut test_xdp_devmap_helpers;
    fn test_xdp_devmap_helpers__destroy(skel: *mut test_xdp_devmap_helpers);
    fn test_xdp_devmap_tailcall__open() -> *mut test_xdp_devmap_tailcall;
    fn test_xdp_devmap_tailcall__load(skel: *mut test_xdp_devmap_tailcall) -> c_int;
    fn test_xdp_devmap_tailcall__destroy(skel: *mut test_xdp_devmap_tailcall);
    fn test_xdp_with_devmap_frags_helpers__open_and_load() -> *mut test_xdp_with_devmap_frags_helpers;
    fn test_xdp_with_devmap_frags_helpers__destroy(skel: *mut test_xdp_with_devmap_frags_helpers);

    fn test__start_subtest(name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: u64, right: u64, name: *const c_char) -> bool;
    fn ASSERT_NEQ(left: u64, right: u64, name: *const c_char) -> bool;
    fn SYS(label: *const c_char, fmt: *const c_char, ...);
    fn SYS_NOFAIL(fmt: *const c_char, ...);
}

unsafe fn test_xdp_with_devmap_helpers() {
    let mut skel: *mut test_xdp_with_devmap_helpers = ptr::null_mut();
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut val = bpf_devmap_val {
        ifindex: IFINDEX_LO as __u32,
        bpf_prog: bpf_devmap_val_bpf_prog { fd: 0 },
    };
    let mut len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let mut err: c_int;
    let dm_fd: c_int;
    let dm_fd_redir: c_int;
    let map_fd: c_int;
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut data = [0u8; ETH_HLEN];
    let mut idx: __u32 = 0;

    SYS(c"out_close".as_ptr(), c"ip netns add %s".as_ptr(), TEST_NS.as_ptr());
    nstoken = open_netns(TEST_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *const c_void, c"open_netns".as_ptr()) {
        goto_out_close_xdp_with_devmap_helpers(nstoken, skel);
        return;
    }
    SYS(c"out_close".as_ptr(), c"ip link set dev lo up".as_ptr());

    skel = test_xdp_with_devmap_helpers__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_xdp_with_devmap_helpers__open_and_load".as_ptr()) {
        goto_out_close_xdp_with_devmap_helpers(nstoken, skel);
        return;
    }

    dm_fd_redir = bpf_program__fd((*skel).progs.xdp_redir_prog);
    err = bpf_xdp_attach(IFINDEX_LO, dm_fd_redir, XDP_FLAGS_SKB_MODE, ptr::null());
    if !ASSERT_OK(err, c"Generic attach of program with 8-byte devmap".as_ptr()) {
        goto_out_close_xdp_with_devmap_helpers(nstoken, skel);
        return;
    }

    dm_fd = bpf_program__fd((*skel).progs.xdp_dummy_dm);
    map_fd = bpf_map__fd((*skel).maps.dm_ports);
    err = bpf_prog_get_info_by_fd(dm_fd, &mut info, &mut len);
    if !ASSERT_OK(err, c"bpf_prog_get_info_by_fd".as_ptr()) {
        goto_out_close_xdp_with_devmap_helpers(nstoken, skel);
        return;
    }

    val.bpf_prog.fd = dm_fd;
    err = bpf_map_update_elem(map_fd, &idx as *const _ as *const c_void, &val as *const _ as *const c_void, 0);
    ASSERT_OK(err, c"Add program to devmap entry".as_ptr());

    err = bpf_map_lookup_elem(map_fd, &idx as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
    ASSERT_OK(err, c"Read devmap entry".as_ptr());
    ASSERT_EQ(info.id as u64, val.bpf_prog.id as u64, c"Match program id to devmap entry prog_id".as_ptr());

    /* send a packet to trigger any potential bugs in there */
    let mut opts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: data.as_mut_ptr() as *mut c_void,
        data_size_in: size_of_val(&data) as __u32,
        flags: BPF_F_TEST_XDP_LIVE_FRAMES,
        repeat: 1,
    };
    err = bpf_prog_test_run_opts(dm_fd_redir, &mut opts);
    ASSERT_OK(err, c"XDP test run".as_ptr());

    /* wait for the packets to be flushed */
    kern_sync_rcu();

    err = bpf_xdp_detach(IFINDEX_LO, XDP_FLAGS_SKB_MODE, ptr::null());
    ASSERT_OK(err, c"XDP program detach".as_ptr());

    /* can not attach BPF_XDP_DEVMAP program to a device */
    err = bpf_xdp_attach(IFINDEX_LO, dm_fd, XDP_FLAGS_SKB_MODE, ptr::null());
    if !ASSERT_NEQ(err as u64, 0, c"Attach of BPF_XDP_DEVMAP program".as_ptr()) {
        bpf_xdp_detach(IFINDEX_LO, XDP_FLAGS_SKB_MODE, ptr::null());
    }

    val.ifindex = 1;
    val.bpf_prog.fd = bpf_program__fd((*skel).progs.xdp_dummy_prog);
    err = bpf_map_update_elem(map_fd, &idx as *const _ as *const c_void, &val as *const _ as *const c_void, 0);
    ASSERT_NEQ(err as u64, 0, c"Add non-BPF_XDP_DEVMAP program to devmap entry".as_ptr());

    /* Try to attach BPF_XDP program with frags to devmap when we have
     * already loaded a BPF_XDP program on the map
     */
    idx = 1;
    val.ifindex = 1;
    val.bpf_prog.fd = bpf_program__fd((*skel).progs.xdp_dummy_dm_frags);
    err = bpf_map_update_elem(map_fd, &idx as *const _ as *const c_void, &val as *const _ as *const c_void, 0);
    ASSERT_NEQ(err as u64, 0, c"Add BPF_XDP program with frags to devmap entry".as_ptr());

    goto_out_close_xdp_with_devmap_helpers(nstoken, skel);
}

unsafe fn goto_out_close_xdp_with_devmap_helpers(
    nstoken: *mut nstoken,
    skel: *mut test_xdp_with_devmap_helpers,
) {
    close_netns(nstoken);
    SYS_NOFAIL(c"ip netns del %s".as_ptr(), TEST_NS.as_ptr());
    test_xdp_with_devmap_helpers__destroy(skel);
}

unsafe fn test_neg_xdp_devmap_helpers() {
    let skel: *mut test_xdp_devmap_helpers;

    skel = test_xdp_devmap_helpers__open_and_load();
    if !ASSERT_EQ(
        skel as u64,
        ptr::null::<test_xdp_devmap_helpers>() as u64,
        c"Load of XDP program accessing egress ifindex without attach type".as_ptr(),
    ) {
        test_xdp_devmap_helpers__destroy(skel);
    }
}

unsafe fn test_xdp_devmap_tailcall(
    prog_dev: bpf_attach_type,
    prog_tail: bpf_attach_type,
    expect_reject: bool,
) {
    let skel: *mut test_xdp_devmap_tailcall;
    let err: c_int;

    skel = test_xdp_devmap_tailcall__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_xdp_devmap_tailcall__open".as_ptr()) {
        return;
    }

    bpf_program__set_expected_attach_type((*skel).progs.xdp_devmap, prog_dev);
    bpf_program__set_expected_attach_type((*skel).progs.xdp_entry, prog_tail);

    err = test_xdp_devmap_tailcall__load(skel);
    if expect_reject {
        ASSERT_ERR(err, c"test_xdp_devmap_tailcall__load".as_ptr());
    } else {
        ASSERT_OK(err, c"test_xdp_devmap_tailcall__load".as_ptr());
    }

    test_xdp_devmap_tailcall__destroy(skel);
}

unsafe fn test_xdp_with_devmap_frags_helpers() {
    let skel: *mut test_xdp_with_devmap_frags_helpers;
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut val = bpf_devmap_val {
        ifindex: IFINDEX_LO as __u32,
        bpf_prog: bpf_devmap_val_bpf_prog { fd: 0 },
    };
    let mut len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let mut err: c_int;
    let dm_fd_frags: c_int;
    let map_fd: c_int;
    let mut idx: __u32 = 0;

    skel = test_xdp_with_devmap_frags_helpers__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_xdp_with_devmap_helpers__open_and_load".as_ptr()) {
        return;
    }

    dm_fd_frags = bpf_program__fd((*skel).progs.xdp_dummy_dm_frags);
    map_fd = bpf_map__fd((*skel).maps.dm_ports);
    err = bpf_prog_get_info_by_fd(dm_fd_frags, &mut info, &mut len);
    if !ASSERT_OK(err, c"bpf_prog_get_info_by_fd".as_ptr()) {
        test_xdp_with_devmap_frags_helpers__destroy(skel);
        return;
    }

    val.bpf_prog.fd = dm_fd_frags;
    err = bpf_map_update_elem(map_fd, &idx as *const _ as *const c_void, &val as *const _ as *const c_void, 0);
    ASSERT_OK(err, c"Add frags program to devmap entry".as_ptr());

    err = bpf_map_lookup_elem(map_fd, &idx as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
    ASSERT_OK(err, c"Read devmap entry".as_ptr());
    ASSERT_EQ(
        info.id as u64,
        val.bpf_prog.id as u64,
        c"Match program id to devmap entry prog_id".as_ptr(),
    );

    /* Try to attach BPF_XDP program to devmap when we have
     * already loaded a BPF_XDP program with frags on the map
     */
    idx = 1;
    val.ifindex = 1;
    val.bpf_prog.fd = bpf_program__fd((*skel).progs.xdp_dummy_dm);
    err = bpf_map_update_elem(map_fd, &idx as *const _ as *const c_void, &val as *const _ as *const c_void, 0);
    ASSERT_NEQ(err as u64, 0, c"Add BPF_XDP program to devmap entry".as_ptr());

    test_xdp_with_devmap_frags_helpers__destroy(skel);
}

unsafe fn test_xdp_with_devmap_helpers_veth() {
    let mut skel: *mut test_xdp_with_devmap_helpers = ptr::null_mut();
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut val: bpf_devmap_val = core::mem::zeroed();
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let mut err: c_int;
    let mut dm_fd: c_int;
    let dm_fd_redir: c_int;
    let map_fd: c_int;
    let ifindex_dst: c_int;
    let mut data = [0u8; ETH_HLEN];
    let idx: __u32 = 0;

    SYS(c"out_close".as_ptr(), c"ip netns add %s".as_ptr(), TEST_NS.as_ptr());
    nstoken = open_netns(TEST_NS.as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(nstoken as *const c_void, c"open_netns".as_ptr()) {
        goto_out_close_xdp_with_devmap_helpers_veth(nstoken, skel);
        return;
    }

    SYS(c"out_close".as_ptr(), c"ip link add veth_src type veth peer name veth_dst".as_ptr());
    SYS(c"out_close".as_ptr(), c"ip link set dev veth_src up".as_ptr());
    SYS(c"out_close".as_ptr(), c"ip link set dev veth_dst up".as_ptr());

    val.ifindex = if_nametoindex(c"veth_src".as_ptr()) as __u32;
    ifindex_dst = if_nametoindex(c"veth_dst".as_ptr()) as c_int;
    if !ASSERT_NEQ(val.ifindex as u64, 0, c"val.ifindex".as_ptr())
        || !ASSERT_NEQ(ifindex_dst as u64, 0, c"ifindex_dst".as_ptr())
    {
        goto_out_close_xdp_with_devmap_helpers_veth(nstoken, skel);
        return;
    }

    skel = test_xdp_with_devmap_helpers__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"test_xdp_with_devmap_helpers__open_and_load".as_ptr()) {
        goto_out_close_xdp_with_devmap_helpers_veth(nstoken, skel);
        return;
    }

    dm_fd_redir = bpf_program__fd((*skel).progs.xdp_redir_prog);
    err = bpf_xdp_attach(val.ifindex as c_int, dm_fd_redir, XDP_FLAGS_DRV_MODE, ptr::null());
    if !ASSERT_OK(err, c"Attach of program with 8-byte devmap".as_ptr()) {
        goto_out_close_xdp_with_devmap_helpers_veth(nstoken, skel);
        return;
    }

    dm_fd = bpf_program__fd((*skel).progs.xdp_dummy_dm);
    map_fd = bpf_map__fd((*skel).maps.dm_ports);
    err = bpf_prog_get_info_by_fd(dm_fd, &mut info, &mut len);
    if !ASSERT_OK(err, c"bpf_prog_get_info_by_fd".as_ptr()) {
        goto_out_close_xdp_with_devmap_helpers_veth(nstoken, skel);
        return;
    }

    val.bpf_prog.fd = dm_fd;
    err = bpf_map_update_elem(map_fd, &idx as *const _ as *const c_void, &val as *const _ as *const c_void, 0);
    ASSERT_OK(err, c"Add program to devmap entry".as_ptr());

    err = bpf_map_lookup_elem(map_fd, &idx as *const _ as *const c_void, &mut val as *mut _ as *mut c_void);
    ASSERT_OK(err, c"Read devmap entry".as_ptr());
    ASSERT_EQ(info.id as u64, val.bpf_prog.id as u64, c"Match program id to devmap entry prog_id".as_ptr());

    /* attach dummy to other side to enable reception */
    dm_fd = bpf_program__fd((*skel).progs.xdp_dummy_prog);
    err = bpf_xdp_attach(ifindex_dst, dm_fd, XDP_FLAGS_DRV_MODE, ptr::null());
    if !ASSERT_OK(err, c"Attach of dummy XDP".as_ptr()) {
        goto_out_close_xdp_with_devmap_helpers_veth(nstoken, skel);
        return;
    }

    /* send a packet to trigger any potential bugs in there */
    let mut opts = bpf_test_run_opts {
        sz: size_of::<bpf_test_run_opts>(),
        data_in: data.as_mut_ptr() as *mut c_void,
        data_size_in: size_of_val(&data) as __u32,
        flags: BPF_F_TEST_XDP_LIVE_FRAMES,
        repeat: 1,
    };
    err = bpf_prog_test_run_opts(dm_fd_redir, &mut opts);
    ASSERT_OK(err, c"XDP test run".as_ptr());

    /* wait for the packets to be flushed */
    kern_sync_rcu();

    err = bpf_xdp_detach(val.ifindex as c_int, XDP_FLAGS_DRV_MODE, ptr::null());
    ASSERT_OK(err, c"XDP program detach".as_ptr());

    err = bpf_xdp_detach(ifindex_dst, XDP_FLAGS_DRV_MODE, ptr::null());
    ASSERT_OK(err, c"XDP program detach".as_ptr());

    goto_out_close_xdp_with_devmap_helpers_veth(nstoken, skel);
}

unsafe fn goto_out_close_xdp_with_devmap_helpers_veth(
    nstoken: *mut nstoken,
    skel: *mut test_xdp_with_devmap_helpers,
) {
    close_netns(nstoken);
    SYS_NOFAIL(c"ip netns del %s".as_ptr(), TEST_NS.as_ptr());
    test_xdp_with_devmap_helpers__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_xdp_devmap_attach() {
    if test__start_subtest(c"DEVMAP with programs in entries".as_ptr()) {
        test_xdp_with_devmap_helpers();
    }

    if test__start_subtest(c"DEVMAP with frags programs in entries".as_ptr()) {
        test_xdp_with_devmap_frags_helpers();
    }

    if test__start_subtest(c"Verifier check of DEVMAP programs".as_ptr()) {
        test_neg_xdp_devmap_helpers();
        test_xdp_devmap_tailcall(BPF_XDP_DEVMAP, BPF_XDP_DEVMAP, false);
        test_xdp_devmap_tailcall(0, 0, true);
        test_xdp_devmap_tailcall(BPF_XDP_DEVMAP, 0, true);
        test_xdp_devmap_tailcall(0, BPF_XDP_DEVMAP, true);
    }

    if test__start_subtest(c"DEVMAP with programs in entries on veth".as_ptr()) {
        test_xdp_with_devmap_helpers_veth();
    }
}
