// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as external Rust dependencies:
// <sys/sysinfo.h>, <test_progs.h>, "network_helpers.h",
// "netcnt_prog.skel.h", and "netcnt_common.h".

use core::ffi::{c_char, c_int, c_ulong, c_void};

const CG_NAME: &[u8] = b"/netcnt\0";
const AF_INET6: c_int = 10;

#[repr(C)]
pub union percpu_net_cnt {
    pub packets: c_ulong,
    pub bytes: c_ulong,
}

#[repr(C)]
pub struct bpf_cgroup_storage_key {
    _private: [u8; 0],
}

#[repr(C)]
pub union net_cnt {
    pub packets: c_ulong,
    pub bytes: c_ulong,
}

#[repr(C)]
pub struct netcnt_prog {
    pub links: netcnt_prog__links,
    pub progs: netcnt_prog__progs,
    pub maps: netcnt_prog__maps,
}

#[repr(C)]
pub struct netcnt_prog__links {
    pub bpf_nextcnt: *mut bpf_link,
}

#[repr(C)]
pub struct netcnt_prog__progs {
    pub bpf_nextcnt: *mut bpf_program,
}

#[repr(C)]
pub struct netcnt_prog__maps {
    pub netcnt: *mut bpf_map,
    pub percpu_netcnt: *mut bpf_map,
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    static MAX_PERCPU_PACKETS: c_ulong;

    fn netcnt_prog__open_and_load() -> *mut netcnt_prog;
    fn netcnt_prog__destroy(skel: *mut netcnt_prog);

    fn bpf_num_possible_cpus() -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;

    fn test__join_cgroup(path: *const c_char) -> c_int;
    fn bpf_program__attach_cgroup(prog: *mut bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    fn ping_command(family: c_int) -> *const c_char;
    fn snprintf(str: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;

    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_get_next_key(fd: c_int, key: *const c_void, next_key: *mut c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(left: c_long_compat, right: c_long_compat, name: *const c_char) -> bool;
    fn ASSERT_LE(left: c_ulong, right: c_ulong, name: *const c_char) -> bool;
}

type c_long_compat = isize;

pub unsafe fn serial_test_netcnt() {
    let mut percpu_netcnt: *mut percpu_net_cnt = core::ptr::null_mut();
    let mut key: bpf_cgroup_storage_key = core::mem::zeroed();
    let map_fd: c_int;
    let percpu_map_fd: c_int;
    let skel: *mut netcnt_prog;
    let mut packets: c_ulong;
    let mut netcnt: net_cnt = core::mem::zeroed();
    let mut bytes: c_ulong;
    let mut cpu: c_int;
    let nproc: c_int;
    let mut cg_fd: c_int = -1;
    let mut cmd: [c_char; 128] = [0; 128];

    skel = netcnt_prog__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"netcnt_prog__open_and_load".as_ptr()) {
        return;
    }

    nproc = bpf_num_possible_cpus();
    percpu_netcnt = malloc(core::mem::size_of::<percpu_net_cnt>() * nproc as usize)
        as *mut percpu_net_cnt;
    if !ASSERT_OK_PTR(percpu_netcnt as *const c_void, c"malloc(percpu_netcnt)".as_ptr()) {
        goto_err(cg_fd, percpu_netcnt, skel);
        return;
    }

    cg_fd = test__join_cgroup(CG_NAME.as_ptr() as *const c_char);
    if !ASSERT_GE(cg_fd as c_long_compat, 0, c"test__join_cgroup".as_ptr()) {
        goto_err(cg_fd, percpu_netcnt, skel);
        return;
    }

    (*skel).links.bpf_nextcnt =
        bpf_program__attach_cgroup((*skel).progs.bpf_nextcnt, cg_fd);
    if !ASSERT_OK_PTR(
        (*skel).links.bpf_nextcnt as *const c_void,
        c"attach_cgroup(bpf_nextcnt)".as_ptr(),
    ) {
        goto_err(cg_fd, percpu_netcnt, skel);
        return;
    }

    snprintf(
        cmd.as_mut_ptr(),
        cmd.len(),
        c"%s ::1 -A -c 10000 -q > /dev/null".as_ptr(),
        ping_command(AF_INET6),
    );
    ASSERT_OK(system(cmd.as_ptr()), cmd.as_ptr());

    map_fd = bpf_map__fd((*skel).maps.netcnt);
    if !ASSERT_OK(
        bpf_map_get_next_key(
            map_fd,
            core::ptr::null(),
            &mut key as *mut _ as *mut c_void,
        ),
        c"bpf_map_get_next_key".as_ptr(),
    ) {
        goto_err(cg_fd, percpu_netcnt, skel);
        return;
    }

    if !ASSERT_OK(
        bpf_map_lookup_elem(
            map_fd,
            &key as *const _ as *const c_void,
            &mut netcnt as *mut _ as *mut c_void,
        ),
        c"bpf_map_lookup_elem(netcnt)".as_ptr(),
    ) {
        goto_err(cg_fd, percpu_netcnt, skel);
        return;
    }

    percpu_map_fd = bpf_map__fd((*skel).maps.percpu_netcnt);
    if !ASSERT_OK(
        bpf_map_lookup_elem(
            percpu_map_fd,
            &key as *const _ as *const c_void,
            &mut *percpu_netcnt.add(0) as *mut _ as *mut c_void,
        ),
        c"bpf_map_lookup_elem(percpu_netcnt)".as_ptr(),
    ) {
        goto_err(cg_fd, percpu_netcnt, skel);
        return;
    }

    /* Some packets can be still in per-cpu cache, but not more than
     * MAX_PERCPU_PACKETS.
     */
    packets = netcnt.packets;
    bytes = netcnt.bytes;
    cpu = 0;
    while cpu < nproc {
        ASSERT_LE(
            (*percpu_netcnt.add(cpu as usize)).packets,
            MAX_PERCPU_PACKETS,
            c"MAX_PERCPU_PACKETS".as_ptr(),
        );

        packets = packets.wrapping_add((*percpu_netcnt.add(cpu as usize)).packets);
        bytes = bytes.wrapping_add((*percpu_netcnt.add(cpu as usize)).bytes);
        cpu += 1;
    }

    /* No packets should be lost */
    ASSERT_GE(packets as c_long_compat, 10000, c"packets".as_ptr());

    /* Let's check that bytes counter matches the number of packets
     * multiplied by the size of ipv6 ICMP packet.
     */
    ASSERT_GE(
        bytes as c_long_compat,
        packets.wrapping_mul(104) as c_long_compat,
        c"bytes".as_ptr(),
    );

    goto_err(cg_fd, percpu_netcnt, skel);
}

unsafe fn goto_err(cg_fd: c_int, percpu_netcnt: *mut percpu_net_cnt, skel: *mut netcnt_prog) {
    if cg_fd != -1 {
        close(cg_fd);
    }
    free(percpu_netcnt as *mut c_void);
    netcnt_prog__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
