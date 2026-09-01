// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */
// C dependencies: test_progs.h, network_helpers.h, map_kptr_race.skel.h

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type __u32 = u32;

const AF_INET6: c_int = 10;
const SOCK_STREAM: c_int = 1;

#[repr(C)]
struct bpf_map_info {
    id: __u32,
}

#[repr(C)]
struct bpf_test_run_opts {
    data_in: *const c_void,
    data_size_in: u32,
    repeat: u32,
    retval: u32,
}

#[repr(C)]
struct map_kptr_race {
    progs: map_kptr_race_progs,
    maps: map_kptr_race_maps,
    links: map_kptr_race_links,
    bss: *mut map_kptr_race_bss,
    rodata: *mut map_kptr_race_rodata,
}

#[repr(C)]
struct map_kptr_race_progs {
    count_ref: *mut bpf_program,
    test_htab_leak: *mut bpf_program,
    test_percpu_htab_leak: *mut bpf_program,
    map_put: *mut bpf_program,
    htab_map_free: *mut bpf_program,
    sk_map_free: *mut bpf_program,
}

#[repr(C)]
struct map_kptr_race_maps {
    race_hash_map: *mut bpf_map,
    race_percpu_hash_map: *mut bpf_map,
    race_sk_ls_map: *mut bpf_map,
}

#[repr(C)]
struct map_kptr_race_links {
    map_put: *mut bpf_link,
    htab_map_free: *mut bpf_link,
    sk_map_free: *mut bpf_link,
}

#[repr(C)]
struct map_kptr_race_bss {
    num_of_refs: c_int,
    target_map_id: c_int,
    map_freed: c_int,
    sk_ls_leak_done: c_int,
}

#[repr(C)]
struct map_kptr_race_rodata {
    nr_cpus: c_int,
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
struct bpf_link {
    _private: [u8; 0],
}

unsafe extern "C" {
    static pkt_v4: c_void;

    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, len: *mut __u32) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;

    fn map_kptr_race__open_and_load() -> *mut map_kptr_race;
    fn map_kptr_race__open() -> *mut map_kptr_race;
    fn map_kptr_race__load(skel: *mut map_kptr_race) -> c_int;
    fn map_kptr_race__attach(skel: *mut map_kptr_race) -> c_int;
    fn map_kptr_race__destroy(skel: *mut map_kptr_race);

    fn libbpf_num_possible_cpus() -> c_int;
    fn kern_sync_rcu();
    fn sched_yield() -> c_int;
    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_char,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;
    fn connect_to_fd(fd: c_int, timeout_ms: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
}

unsafe fn get_map_id(map_fd: c_int) -> c_int {
    let mut info: bpf_map_info = bpf_map_info { id: 0 };
    let mut len: __u32 = size_of::<bpf_map_info>() as __u32;

    if !ASSERT_OK(
        bpf_map_get_info_by_fd(map_fd, &mut info, &mut len),
        c"get_map_info".as_ptr(),
    ) {
        return -1;
    }
    info.id as c_int
}

unsafe fn read_refs(skel: *mut map_kptr_race) -> c_int {
    let mut opts: bpf_test_run_opts = bpf_test_run_opts {
        data_in: ptr::null(),
        data_size_in: 0,
        repeat: 0,
        retval: 0,
    };
    let ret: c_int;

    ret = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.count_ref), &mut opts);
    if !ASSERT_OK(ret, c"count_ref run".as_ptr()) {
        return -1;
    }
    if !ASSERT_OK(opts.retval as c_int, c"count_ref retval".as_ptr()) {
        return -1;
    }
    (*(*skel).bss).num_of_refs
}

unsafe fn test_htab_leak() {
    let mut opts: bpf_test_run_opts = bpf_test_run_opts {
        data_in: &pkt_v4 as *const c_void,
        data_size_in: size_of_val_raw(&pkt_v4 as *const c_void) as u32,
        repeat: 1,
        retval: 0,
    };
    let mut skel: *mut map_kptr_race;
    let mut watcher: *mut map_kptr_race = ptr::null_mut();
    let ret: c_int;
    let map_id: c_int;

    skel = map_kptr_race__open_and_load();
    if !ASSERT_OK_PTR(skel, c"open_and_load".as_ptr()) {
        return;
    }

    ret = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.test_htab_leak), &mut opts);
    if !ASSERT_OK(ret, c"test_htab_leak run".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }
    if !ASSERT_OK(opts.retval as c_int, c"test_htab_leak retval".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }

    map_id = get_map_id(bpf_map__fd((*skel).maps.race_hash_map));
    if !ASSERT_GE(map_id, 0, c"map_id".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }

    watcher = map_kptr_race__open_and_load();
    if !ASSERT_OK_PTR(watcher, c"watcher open_and_load".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }

    'out_watcher: {
        (*(*watcher).bss).target_map_id = map_id;
        (*watcher).links.map_put = bpf_program__attach((*watcher).progs.map_put);
        if !ASSERT_OK_PTR((*watcher).links.map_put, c"attach fentry".as_ptr()) {
            break 'out_watcher;
        }
        (*watcher).links.htab_map_free = bpf_program__attach((*watcher).progs.htab_map_free);
        if !ASSERT_OK_PTR((*watcher).links.htab_map_free, c"attach fexit".as_ptr()) {
            break 'out_watcher;
        }

        map_kptr_race__destroy(skel);
        skel = ptr::null_mut();

        kern_sync_rcu();

        while ptr::read_volatile(&(*(*watcher).bss).map_freed) == 0 {
            sched_yield();
        }

        ASSERT_EQ((*(*watcher).bss).map_freed, 1, c"map_freed".as_ptr());
        ASSERT_EQ(read_refs(watcher), 2, c"htab refcount".as_ptr());
    }

    map_kptr_race__destroy(watcher);
    map_kptr_race__destroy(skel);
}

unsafe fn test_percpu_htab_leak() {
    let mut opts: bpf_test_run_opts = bpf_test_run_opts {
        data_in: &pkt_v4 as *const c_void,
        data_size_in: size_of_val_raw(&pkt_v4 as *const c_void) as u32,
        repeat: 1,
        retval: 0,
    };
    let mut skel: *mut map_kptr_race;
    let mut watcher: *mut map_kptr_race = ptr::null_mut();
    let mut ret: c_int;
    let map_id: c_int;

    skel = map_kptr_race__open();
    if !ASSERT_OK_PTR(skel, c"open".as_ptr()) {
        return;
    }

    (*(*skel).rodata).nr_cpus = libbpf_num_possible_cpus();
    if (*(*skel).rodata).nr_cpus > 16 {
        (*(*skel).rodata).nr_cpus = 16;
    }

    ret = map_kptr_race__load(skel);
    if !ASSERT_OK(ret, c"load".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }

    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_percpu_htab_leak),
        &mut opts,
    );
    if !ASSERT_OK(ret, c"test_percpu_htab_leak run".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }
    if !ASSERT_OK(opts.retval as c_int, c"test_percpu_htab_leak retval".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }

    map_id = get_map_id(bpf_map__fd((*skel).maps.race_percpu_hash_map));
    if !ASSERT_GE(map_id, 0, c"map_id".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }

    watcher = map_kptr_race__open_and_load();
    if !ASSERT_OK_PTR(watcher, c"watcher open_and_load".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }

    'out_watcher: {
        (*(*watcher).bss).target_map_id = map_id;
        (*watcher).links.map_put = bpf_program__attach((*watcher).progs.map_put);
        if !ASSERT_OK_PTR((*watcher).links.map_put, c"attach fentry".as_ptr()) {
            break 'out_watcher;
        }
        (*watcher).links.htab_map_free = bpf_program__attach((*watcher).progs.htab_map_free);
        if !ASSERT_OK_PTR((*watcher).links.htab_map_free, c"attach fexit".as_ptr()) {
            break 'out_watcher;
        }

        map_kptr_race__destroy(skel);
        skel = ptr::null_mut();

        kern_sync_rcu();

        while ptr::read_volatile(&(*(*watcher).bss).map_freed) == 0 {
            sched_yield();
        }

        ASSERT_EQ((*(*watcher).bss).map_freed, 1, c"map_freed".as_ptr());
        ASSERT_EQ(read_refs(watcher), 2, c"percpu_htab refcount".as_ptr());
    }

    map_kptr_race__destroy(watcher);
    map_kptr_race__destroy(skel);
}

unsafe fn test_sk_ls_leak() {
    let mut skel: *mut map_kptr_race;
    let mut watcher: *mut map_kptr_race = ptr::null_mut();
    let mut listen_fd: c_int = -1;
    let mut client_fd: c_int = -1;
    let map_id: c_int;

    skel = map_kptr_race__open_and_load();
    if !ASSERT_OK_PTR(skel, c"open_and_load".as_ptr()) {
        return;
    }

    if !ASSERT_OK(map_kptr_race__attach(skel), c"attach".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }

    listen_fd = start_server(AF_INET6, SOCK_STREAM, c"::1".as_ptr(), 0, 0);
    if !ASSERT_GE(listen_fd, 0, c"start_server".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }

    client_fd = connect_to_fd(listen_fd, 0);
    if !ASSERT_GE(client_fd, 0, c"connect_to_fd".as_ptr()) {
        close(listen_fd);
        map_kptr_race__destroy(skel);
        return;
    }

    if !ASSERT_EQ((*(*skel).bss).sk_ls_leak_done, 1, c"sk_ls_leak_done".as_ptr()) {
        close(client_fd);
        close(listen_fd);
        map_kptr_race__destroy(skel);
        return;
    }

    close(client_fd);
    client_fd = -1;
    close(listen_fd);
    listen_fd = -1;

    map_id = get_map_id(bpf_map__fd((*skel).maps.race_sk_ls_map));
    if !ASSERT_GE(map_id, 0, c"map_id".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }

    watcher = map_kptr_race__open_and_load();
    if !ASSERT_OK_PTR(watcher, c"watcher open_and_load".as_ptr()) {
        map_kptr_race__destroy(skel);
        return;
    }

    'out_watcher: {
        (*(*watcher).bss).target_map_id = map_id;
        (*watcher).links.map_put = bpf_program__attach((*watcher).progs.map_put);
        if !ASSERT_OK_PTR((*watcher).links.map_put, c"attach fentry".as_ptr()) {
            break 'out_watcher;
        }
        (*watcher).links.sk_map_free = bpf_program__attach((*watcher).progs.sk_map_free);
        if !ASSERT_OK_PTR((*watcher).links.sk_map_free, c"attach fexit".as_ptr()) {
            break 'out_watcher;
        }

        map_kptr_race__destroy(skel);
        skel = ptr::null_mut();

        kern_sync_rcu();

        while ptr::read_volatile(&(*(*watcher).bss).map_freed) == 0 {
            sched_yield();
        }

        ASSERT_EQ((*(*watcher).bss).map_freed, 1, c"map_freed".as_ptr());
        ASSERT_EQ(read_refs(watcher), 2, c"sk_ls refcount".as_ptr());
    }

    map_kptr_race__destroy(watcher);
    if client_fd >= 0 {
        close(client_fd);
    }
    if listen_fd >= 0 {
        close(listen_fd);
    }
    map_kptr_race__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_map_kptr_race() {
    if test__start_subtest(c"htab_leak".as_ptr()) {
        test_htab_leak();
    }
    if test__start_subtest(c"percpu_htab_leak".as_ptr()) {
        test_percpu_htab_leak();
    }
    if test__start_subtest(c"sk_ls_leak".as_ptr()) {
        test_sk_ls_leak();
    }
}

unsafe fn size_of_val_raw<T: ?Sized>(_val: *const T) -> usize {
    // File-local stand-in for sizeof(pkt_v4), whose complete type is provided by C headers.
    size_of::<*const c_void>()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
