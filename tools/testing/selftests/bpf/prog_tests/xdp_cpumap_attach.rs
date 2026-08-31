// SPDX-License-Identifier: GPL-2.0
// C dependencies translated from:
// <uapi/linux/bpf.h>, <linux/if_link.h>, <test_progs.h>,
// <network_helpers.h>, "test_xdp_with_cpumap_frags_helpers.skel.h",
// and "test_xdp_with_cpumap_helpers.skel.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const IFINDEX_LO: c_int = 1;
const TEST_NS: &[u8] = b"cpu_attach_ns\0";

const XDP_FLAGS_SKB_MODE: c_uint = 1 << 1;
const BPF_F_TEST_XDP_LIVE_FRAMES: c_uint = 1 << 1;
const ETH_HLEN: usize = 14;
const O_RDONLY: c_int = 0;
const EINVAL: c_int = 22;
const EBADF: c_int = 9;

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nstoken {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog_info {
    pub id: u32,
}

#[repr(C)]
pub union bpf_cpumap_val_bpf_prog {
    pub fd: c_int,
    pub id: u32,
}

#[repr(C)]
pub struct bpf_cpumap_val {
    pub qsize: u32,
    pub bpf_prog: bpf_cpumap_val_bpf_prog,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *mut c_void,
    pub data_size_in: u32,
    pub flags: u32,
    pub repeat: u32,
}

#[repr(C)]
pub struct test_xdp_with_cpumap_helpers_progs {
    pub xdp_redir_prog: *mut bpf_program,
    pub xdp_dummy_cm: *mut bpf_program,
    pub xdp_dummy_prog: *mut bpf_program,
    pub xdp_dummy_cm_frags: *mut bpf_program,
}

#[repr(C)]
pub struct test_xdp_with_cpumap_helpers_maps {
    pub cpu_map: *mut bpf_map,
}

#[repr(C)]
pub struct test_xdp_with_cpumap_helpers_bss {
    pub redirect_count: u32,
}

#[repr(C)]
pub struct test_xdp_with_cpumap_helpers {
    pub progs: test_xdp_with_cpumap_helpers_progs,
    pub maps: test_xdp_with_cpumap_helpers_maps,
    pub bss: *mut test_xdp_with_cpumap_helpers_bss,
}

#[repr(C)]
pub struct test_xdp_with_cpumap_frags_helpers_progs {
    pub xdp_dummy_cm_frags: *mut bpf_program,
    pub xdp_dummy_cm: *mut bpf_program,
}

#[repr(C)]
pub struct test_xdp_with_cpumap_frags_helpers_maps {
    pub cpu_map: *mut bpf_map,
}

#[repr(C)]
pub struct test_xdp_with_cpumap_frags_helpers {
    pub progs: test_xdp_with_cpumap_frags_helpers_progs,
    pub maps: test_xdp_with_cpumap_frags_helpers_maps,
}

unsafe extern "C" {
    fn open_netns(name: *const c_char) -> *mut nstoken;
    fn close_netns(token: *mut nstoken);
    fn kern_sync_rcu();

    fn test_xdp_with_cpumap_helpers__open_and_load() -> *mut test_xdp_with_cpumap_helpers;
    fn test_xdp_with_cpumap_helpers__destroy(skel: *mut test_xdp_with_cpumap_helpers);
    fn test_xdp_with_cpumap_frags_helpers__open_and_load(
    ) -> *mut test_xdp_with_cpumap_frags_helpers;
    fn test_xdp_with_cpumap_frags_helpers__destroy(
        skel: *mut test_xdp_with_cpumap_frags_helpers,
    );

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_xdp_attach(
        ifindex: c_int,
        prog_fd: c_int,
        flags: c_uint,
        opts: *const c_void,
    ) -> c_int;
    fn bpf_xdp_detach(ifindex: c_int, flags: c_uint, opts: *const c_void) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, len: *mut u32) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: i64, expected: i64, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: i64, expected: i64, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: i64, expected: i64, name: *const c_char) -> bool;
}

// SYS/SYS_NOFAIL are C test macros that execute formatted shell commands and
// branch to the supplied label on failure. They are kept as macro calls here.
macro_rules! SYS {
    ($label:lifetime, $fmt:expr, $arg:expr) => {
        sys!($label, $fmt, $arg)
    };
    ($label:lifetime, $fmt:expr) => {
        sys!($label, $fmt)
    };
}

macro_rules! SYS_NOFAIL {
    ($fmt:expr, $arg:expr) => {
        sys_nofail!($fmt, $arg)
    };
}

unsafe fn test_xdp_with_cpumap_helpers() {
    let mut skel: *mut test_xdp_with_cpumap_helpers = ptr::null_mut();
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut len: u32 = size_of::<bpf_prog_info>() as u32;
    let mut val = bpf_cpumap_val {
        qsize: 192,
        bpf_prog: bpf_cpumap_val_bpf_prog { fd: 0 },
    };
    let mut err: c_int;
    let prog_fd: c_int;
    let prog_redir_fd: c_int;
    let map_fd: c_int;
    let bad_fd: c_int;
    let mut nstoken: *mut nstoken = ptr::null_mut();
    let mut idx: u32 = 0;

    'out_close: {
        SYS!('out_close, c"ip netns add %s".as_ptr(), TEST_NS.as_ptr() as *const c_char);
        nstoken = open_netns(TEST_NS.as_ptr() as *const c_char);
        if !ASSERT_OK_PTR(nstoken as *const c_void, c"open_netns".as_ptr()) {
            break 'out_close;
        }
        SYS!('out_close, c"ip link set dev lo up".as_ptr());

        skel = test_xdp_with_cpumap_helpers__open_and_load();
        if !ASSERT_OK_PTR(
            skel as *const c_void,
            c"test_xdp_with_cpumap_helpers__open_and_load".as_ptr(),
        ) {
            return;
        }

        prog_redir_fd = bpf_program__fd((*skel).progs.xdp_redir_prog);
        err = bpf_xdp_attach(IFINDEX_LO, prog_redir_fd, XDP_FLAGS_SKB_MODE, ptr::null());
        if !ASSERT_OK(err, c"Generic attach of program with 8-byte CPUMAP".as_ptr()) {
            break 'out_close;
        }

        prog_fd = bpf_program__fd((*skel).progs.xdp_dummy_cm);
        map_fd = bpf_map__fd((*skel).maps.cpu_map);
        err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut len);
        if !ASSERT_OK(err, c"bpf_prog_get_info_by_fd".as_ptr()) {
            break 'out_close;
        }

        val.bpf_prog.fd = prog_fd;
        err = bpf_map_update_elem(
            map_fd,
            &idx as *const _ as *const c_void,
            &val as *const _ as *const c_void,
            0,
        );
        ASSERT_OK(err, c"Add program to cpumap entry".as_ptr());

        err = bpf_map_lookup_elem(
            map_fd,
            &idx as *const _ as *const c_void,
            &mut val as *mut _ as *mut c_void,
        );
        ASSERT_OK(err, c"Read cpumap entry".as_ptr());
        ASSERT_EQ(
            info.id as i64,
            val.bpf_prog.id as i64,
            c"Match program id to cpumap entry prog_id".as_ptr(),
        );

        /* send a packet to trigger any potential bugs in there */
        let mut data = [0u8; ETH_HLEN];
        let mut opts = bpf_test_run_opts {
            data_in: data.as_mut_ptr() as *mut c_void,
            data_size_in: data.len() as u32,
            flags: BPF_F_TEST_XDP_LIVE_FRAMES,
            repeat: 1,
        };
        err = bpf_prog_test_run_opts(prog_redir_fd, &mut opts);
        ASSERT_OK(err, c"XDP test run".as_ptr());

        /* wait for the packets to be flushed, then check that redirect has been
         * performed
         */
        kern_sync_rcu();
        ASSERT_NEQ(
            (*(*skel).bss).redirect_count as i64,
            0,
            c"redirected packets".as_ptr(),
        );

        err = bpf_xdp_detach(IFINDEX_LO, XDP_FLAGS_SKB_MODE, ptr::null());
        ASSERT_OK(err, c"XDP program detach".as_ptr());

        /* can not attach BPF_XDP_CPUMAP program to a device */
        err = bpf_xdp_attach(IFINDEX_LO, prog_fd, XDP_FLAGS_SKB_MODE, ptr::null());
        if !ASSERT_NEQ(err as i64, 0, c"Attach of BPF_XDP_CPUMAP program".as_ptr()) {
            bpf_xdp_detach(IFINDEX_LO, XDP_FLAGS_SKB_MODE, ptr::null());
        }

        val.qsize = 192;
        val.bpf_prog.fd = bpf_program__fd((*skel).progs.xdp_dummy_prog);
        err = bpf_map_update_elem(
            map_fd,
            &idx as *const _ as *const c_void,
            &val as *const _ as *const c_void,
            0,
        );
        ASSERT_EQ(
            err as i64,
            -EINVAL as i64,
            c"Add non-BPF_XDP_CPUMAP program to cpumap entry".as_ptr(),
        );

        /* Try to attach non-BPF file descriptor */
        bad_fd = open(c"/dev/null".as_ptr(), O_RDONLY);
        ASSERT_GE(bad_fd as i64, 0, c"Open /dev/null for non-BPF fd".as_ptr());

        val.bpf_prog.fd = bad_fd;
        err = bpf_map_update_elem(
            map_fd,
            &idx as *const _ as *const c_void,
            &val as *const _ as *const c_void,
            0,
        );
        ASSERT_EQ(
            err as i64,
            -EINVAL as i64,
            c"Add non-BPF fd to cpumap entry".as_ptr(),
        );

        /* Try to attach nonexistent file descriptor */
        err = close(bad_fd);
        ASSERT_EQ(err as i64, 0, c"Close non-BPF fd for nonexistent fd".as_ptr());

        err = bpf_map_update_elem(
            map_fd,
            &idx as *const _ as *const c_void,
            &val as *const _ as *const c_void,
            0,
        );
        ASSERT_EQ(
            err as i64,
            -EBADF as i64,
            c"Add nonexistent fd to cpumap entry".as_ptr(),
        );

        /* Try to attach BPF_XDP program with frags to cpumap when we have
         * already loaded a BPF_XDP program on the map
         */
        idx = 1;
        val.qsize = 192;
        val.bpf_prog.fd = bpf_program__fd((*skel).progs.xdp_dummy_cm_frags);
        err = bpf_map_update_elem(
            map_fd,
            &idx as *const _ as *const c_void,
            &val as *const _ as *const c_void,
            0,
        );
        ASSERT_NEQ(
            err as i64,
            0,
            c"Add BPF_XDP program with frags to cpumap entry".as_ptr(),
        );
    }

    close_netns(nstoken);
    SYS_NOFAIL!(c"ip netns del %s".as_ptr(), TEST_NS.as_ptr() as *const c_char);
    test_xdp_with_cpumap_helpers__destroy(skel);
}

unsafe fn test_xdp_with_cpumap_frags_helpers() {
    let skel: *mut test_xdp_with_cpumap_frags_helpers;
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut len: u32 = size_of::<bpf_prog_info>() as u32;
    let mut val = bpf_cpumap_val {
        qsize: 192,
        bpf_prog: bpf_cpumap_val_bpf_prog { fd: 0 },
    };
    let mut err: c_int;
    let frags_prog_fd: c_int;
    let map_fd: c_int;
    let mut idx: u32 = 0;

    'out_close: {
        skel = test_xdp_with_cpumap_frags_helpers__open_and_load();
        if !ASSERT_OK_PTR(
            skel as *const c_void,
            c"test_xdp_with_cpumap_helpers__open_and_load".as_ptr(),
        ) {
            return;
        }

        frags_prog_fd = bpf_program__fd((*skel).progs.xdp_dummy_cm_frags);
        map_fd = bpf_map__fd((*skel).maps.cpu_map);
        err = bpf_prog_get_info_by_fd(frags_prog_fd, &mut info, &mut len);
        if !ASSERT_OK(err, c"bpf_prog_get_info_by_fd".as_ptr()) {
            break 'out_close;
        }

        val.bpf_prog.fd = frags_prog_fd;
        err = bpf_map_update_elem(
            map_fd,
            &idx as *const _ as *const c_void,
            &val as *const _ as *const c_void,
            0,
        );
        ASSERT_OK(err, c"Add program to cpumap entry".as_ptr());

        err = bpf_map_lookup_elem(
            map_fd,
            &idx as *const _ as *const c_void,
            &mut val as *mut _ as *mut c_void,
        );
        ASSERT_OK(err, c"Read cpumap entry".as_ptr());
        ASSERT_EQ(
            info.id as i64,
            val.bpf_prog.id as i64,
            c"Match program id to cpumap entry prog_id".as_ptr(),
        );

        /* Try to attach BPF_XDP program to cpumap when we have
         * already loaded a BPF_XDP program with frags on the map
         */
        idx = 1;
        val.qsize = 192;
        val.bpf_prog.fd = bpf_program__fd((*skel).progs.xdp_dummy_cm);
        err = bpf_map_update_elem(
            map_fd,
            &idx as *const _ as *const c_void,
            &val as *const _ as *const c_void,
            0,
        );
        ASSERT_NEQ(err as i64, 0, c"Add BPF_XDP program to cpumap entry".as_ptr());
    }

    test_xdp_with_cpumap_frags_helpers__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_xdp_cpumap_attach() {
    if test__start_subtest(c"CPUMAP with programs in entries".as_ptr()) {
        test_xdp_with_cpumap_helpers();
    }

    if test__start_subtest(c"CPUMAP with frags programs in entries".as_ptr()) {
        test_xdp_with_cpumap_frags_helpers();
    }
}
