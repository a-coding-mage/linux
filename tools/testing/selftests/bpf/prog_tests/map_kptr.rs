// SPDX-License-Identifier: GPL-2.0
// Translated from C source:
// #include <test_progs.h>
// #include <network_helpers.h>
// #include "map_kptr.skel.h"
// #include "map_kptr_fail.skel.h"
// #include "rcu_tasks_trace_gp.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

const EFAULT: c_int = 14;

#[repr(C)]
pub struct bpf_test_run_opts {
    pub sz: usize,
    pub data_in: *mut c_void,
    pub data_size_in: c_uint,
    pub data_out: *mut c_void,
    pub data_size_out: c_uint,
    pub ctx_in: *mut c_void,
    pub ctx_size_in: c_uint,
    pub ctx_out: *mut c_void,
    pub ctx_size_out: c_uint,
    pub retval: c_uint,
    pub repeat: c_int,
    pub duration: c_uint,
    pub flags: c_uint,
    pub cpu: c_int,
    pub batch_size: c_int,
}

impl Default for bpf_test_run_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            data_in: core::ptr::null_mut(),
            data_size_in: 0,
            data_out: core::ptr::null_mut(),
            data_size_out: 0,
            ctx_in: core::ptr::null_mut(),
            ctx_size_in: 0,
            ctx_out: core::ptr::null_mut(),
            ctx_size_out: 0,
            retval: 0,
            repeat: 0,
            duration: 0,
            flags: 0,
            cpu: 0,
            batch_size: 0,
        }
    }
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map_kptr {
    pub maps: map_kptr_maps,
    pub progs: map_kptr_progs,
    pub data: *mut map_kptr_data,
    pub bss: *mut map_kptr_bss,
}

#[repr(C)]
pub struct map_kptr_maps {
    pub array_map: *mut bpf_map,
    pub pcpu_array_map: *mut bpf_map,
    pub hash_map: *mut bpf_map,
    pub pcpu_hash_map: *mut bpf_map,
    pub hash_malloc_map: *mut bpf_map,
    pub pcpu_hash_malloc_map: *mut bpf_map,
    pub lru_hash_map: *mut bpf_map,
    pub lru_pcpu_hash_map: *mut bpf_map,
}

#[repr(C)]
pub struct map_kptr_progs {
    pub test_map_kptr_ref1: *mut bpf_program,
    pub test_map_kptr_ref2: *mut bpf_program,
    pub test_ls_map_kptr_ref1: *mut bpf_program,
    pub test_ls_map_kptr_ref2: *mut bpf_program,
    pub test_map_kptr_ref3: *mut bpf_program,
    pub test_ls_map_kptr_ref_del: *mut bpf_program,
    pub count_ref: *mut bpf_program,
    pub test_array_map_update_kptr: *mut bpf_program,
    pub test_hash_map_update_kptr: *mut bpf_program,
    pub test_hash_malloc_map_update_kptr: *mut bpf_program,
}

#[repr(C)]
pub struct map_kptr_data {
    pub ref_: c_int,
}

#[repr(C)]
pub struct map_kptr_bss {
    pub num_of_refs: c_int,
}

#[repr(C)]
pub struct rcu_tasks_trace_gp {
    pub progs: rcu_tasks_trace_gp_progs,
    pub bss: *mut rcu_tasks_trace_gp_bss,
}

#[repr(C)]
pub struct rcu_tasks_trace_gp_progs {
    pub call_rcu_tasks_trace: *mut bpf_program,
}

#[repr(C)]
pub struct rcu_tasks_trace_gp_bss {
    pub done: c_int,
}

unsafe extern "C" {
    static mut pkt_v4: c_void;

    fn map_kptr__open_and_load() -> *mut map_kptr;
    fn map_kptr__destroy(obj: *mut map_kptr);
    fn rcu_tasks_trace_gp__open_and_load() -> *mut rcu_tasks_trace_gp;
    fn rcu_tasks_trace_gp__destroy(obj: *mut rcu_tasks_trace_gp);

    fn bpf_prog_test_run_opts(prog_fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map__update_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: usize,
        value: *const c_void,
        value_sz: usize,
        flags: u64,
    ) -> c_int;
    fn bpf_map__delete_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: usize,
        flags: u64,
    ) -> c_int;
    fn libbpf_num_possible_cpus() -> c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn sched_yield() -> c_int;
    fn kern_sync_rcu() -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(value: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(value: c_int, expected: c_int, name: *const c_char) -> bool;
    fn RUN_TESTS_map_kptr_fail();
    fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn test_map_kptr_success(test_run: bool) {
    let mut lopts = bpf_test_run_opts::default();
    let mut opts = bpf_test_run_opts {
        data_in: core::ptr::addr_of_mut!(pkt_v4),
        data_size_in: core::mem::size_of_val(&pkt_v4) as c_uint,
        repeat: 1,
        ..Default::default()
    };
    let mut key: c_int = 0;
    let mut ret: c_int;
    let cpu: c_int;
    let skel: *mut map_kptr;
    let buf: [c_char; 16] = [0; 16];
    let pbuf: *mut c_char;

    skel = map_kptr__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"map_kptr__open_and_load".as_ptr()) {
        return;
    }

    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_map_kptr_ref1),
        &mut opts,
    );
    ASSERT_OK(ret, c"test_map_kptr_ref1 refcount".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"test_map_kptr_ref1 retval".as_ptr());
    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_map_kptr_ref2),
        &mut opts,
    );
    ASSERT_OK(ret, c"test_map_kptr_ref2 refcount".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"test_map_kptr_ref2 retval".as_ptr());

    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_ls_map_kptr_ref1),
        &mut lopts,
    );
    ASSERT_OK(ret, c"test_ls_map_kptr_ref1 refcount".as_ptr());
    ASSERT_OK(lopts.retval as c_int, c"test_ls_map_kptr_ref1 retval".as_ptr());

    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_ls_map_kptr_ref2),
        &mut lopts,
    );
    ASSERT_OK(ret, c"test_ls_map_kptr_ref2 refcount".as_ptr());
    ASSERT_OK(lopts.retval as c_int, c"test_ls_map_kptr_ref2 retval".as_ptr());

    if test_run {
        map_kptr__destroy(skel);
        return;
    }

    cpu = libbpf_num_possible_cpus();
    if !ASSERT_GT(cpu, 0, c"libbpf_num_possible_cpus".as_ptr()) {
        map_kptr__destroy(skel);
        return;
    }

    pbuf = calloc(cpu as usize, core::mem::size_of_val(&buf)) as *mut c_char;
    if !ASSERT_OK_PTR(pbuf as *const c_void, c"calloc(pbuf)".as_ptr()) {
        map_kptr__destroy(skel);
        return;
    }

    ret = bpf_map__update_elem(
        (*skel).maps.array_map,
        &key as *const _ as *const c_void,
        core::mem::size_of_val(&key),
        buf.as_ptr() as *const c_void,
        core::mem::size_of_val(&buf),
        0,
    );
    ASSERT_OK(ret, c"array_map update".as_ptr());
    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_map_kptr_ref3),
        &mut opts,
    );
    ASSERT_OK(ret, c"test_map_kptr_ref3 refcount".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"test_map_kptr_ref3 retval".as_ptr());

    ret = bpf_map__update_elem(
        (*skel).maps.pcpu_array_map,
        &key as *const _ as *const c_void,
        core::mem::size_of_val(&key),
        pbuf as *const c_void,
        cpu as usize * core::mem::size_of_val(&buf),
        0,
    );
    ASSERT_OK(ret, c"pcpu_array_map update".as_ptr());
    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_map_kptr_ref3),
        &mut opts,
    );
    ASSERT_OK(ret, c"test_map_kptr_ref3 refcount".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"test_map_kptr_ref3 retval".as_ptr());

    ret = bpf_map__delete_elem(
        (*skel).maps.hash_map,
        &key as *const _ as *const c_void,
        core::mem::size_of_val(&key),
        0,
    );
    ASSERT_OK(ret, c"hash_map delete".as_ptr());
    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_map_kptr_ref3),
        &mut opts,
    );
    ASSERT_OK(ret, c"test_map_kptr_ref3 refcount".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"test_map_kptr_ref3 retval".as_ptr());

    ret = bpf_map__delete_elem(
        (*skel).maps.pcpu_hash_map,
        &key as *const _ as *const c_void,
        core::mem::size_of_val(&key),
        0,
    );
    ASSERT_OK(ret, c"pcpu_hash_map delete".as_ptr());
    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_map_kptr_ref3),
        &mut opts,
    );
    ASSERT_OK(ret, c"test_map_kptr_ref3 refcount".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"test_map_kptr_ref3 retval".as_ptr());

    ret = bpf_map__delete_elem(
        (*skel).maps.hash_malloc_map,
        &key as *const _ as *const c_void,
        core::mem::size_of_val(&key),
        0,
    );
    ASSERT_OK(ret, c"hash_malloc_map delete".as_ptr());
    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_map_kptr_ref3),
        &mut opts,
    );
    ASSERT_OK(ret, c"test_map_kptr_ref3 refcount".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"test_map_kptr_ref3 retval".as_ptr());

    ret = bpf_map__delete_elem(
        (*skel).maps.pcpu_hash_malloc_map,
        &key as *const _ as *const c_void,
        core::mem::size_of_val(&key),
        0,
    );
    ASSERT_OK(ret, c"pcpu_hash_malloc_map delete".as_ptr());
    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_map_kptr_ref3),
        &mut opts,
    );
    ASSERT_OK(ret, c"test_map_kptr_ref3 refcount".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"test_map_kptr_ref3 retval".as_ptr());

    ret = bpf_map__delete_elem(
        (*skel).maps.lru_hash_map,
        &key as *const _ as *const c_void,
        core::mem::size_of_val(&key),
        0,
    );
    ASSERT_OK(ret, c"lru_hash_map delete".as_ptr());
    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_map_kptr_ref3),
        &mut opts,
    );
    ASSERT_OK(ret, c"test_map_kptr_ref3 refcount".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"test_map_kptr_ref3 retval".as_ptr());

    ret = bpf_map__delete_elem(
        (*skel).maps.lru_pcpu_hash_map,
        &key as *const _ as *const c_void,
        core::mem::size_of_val(&key),
        0,
    );
    ASSERT_OK(ret, c"lru_pcpu_hash_map delete".as_ptr());
    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_map_kptr_ref3),
        &mut opts,
    );
    ASSERT_OK(ret, c"test_map_kptr_ref3 refcount".as_ptr());
    ASSERT_OK(opts.retval as c_int, c"test_map_kptr_ref3 retval".as_ptr());

    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*skel).progs.test_ls_map_kptr_ref_del),
        &mut lopts,
    );
    ASSERT_OK(ret, c"test_ls_map_kptr_ref_del delete".as_ptr());
    (*(*skel).data).ref_ -= 1;
    ASSERT_OK(lopts.retval as c_int, c"test_ls_map_kptr_ref_del retval".as_ptr());

    free(pbuf as *mut c_void);
    map_kptr__destroy(skel);
}

unsafe fn kern_sync_rcu_tasks_trace(rcu: *mut rcu_tasks_trace_gp) -> c_int {
    let mut opts = bpf_test_run_opts::default();
    let ret: c_int;

    core::ptr::write_volatile(core::ptr::addr_of_mut!((*(*rcu).bss).done), 0);
    ret = bpf_prog_test_run_opts(
        bpf_program__fd((*rcu).progs.call_rcu_tasks_trace),
        &mut opts,
    );
    if !ASSERT_OK(ret, c"call_rcu_tasks_trace".as_ptr()) {
        return -EFAULT;
    }
    if !ASSERT_OK(opts.retval as c_int, c"call_rcu_tasks_trace retval".as_ptr()) {
        return -EFAULT;
    }
    while core::ptr::read_volatile(core::ptr::addr_of!((*(*rcu).bss).done)) == 0 {
        sched_yield();
    }
    0
}

unsafe fn wait_for_map_release() {
    let mut lopts = bpf_test_run_opts::default();
    let skel: *mut map_kptr;
    let mut ret: c_int;

    skel = map_kptr__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"map_kptr__open_and_load".as_ptr()) {
        return;
    }

    loop {
        ret = bpf_prog_test_run_opts(bpf_program__fd((*skel).progs.count_ref), &mut lopts);
        ASSERT_OK(ret, c"count_ref ret".as_ptr());
        ASSERT_OK(lopts.retval as c_int, c"count_ref retval".as_ptr());
        if (*(*skel).bss).num_of_refs == 2 {
            break;
        }
    }

    map_kptr__destroy(skel);
}

#[repr(C)]
enum map_update_kptr_case {
    MAP_UPDATE_KPTR_ARRAY,
    MAP_UPDATE_KPTR_HASH,
    MAP_UPDATE_KPTR_HASH_MALLOC,
}

unsafe fn map_update_kptr_prog(
    skel: *mut map_kptr,
    test: map_update_kptr_case,
) -> *mut bpf_program {
    match test {
        map_update_kptr_case::MAP_UPDATE_KPTR_ARRAY => (*skel).progs.test_array_map_update_kptr,
        map_update_kptr_case::MAP_UPDATE_KPTR_HASH => (*skel).progs.test_hash_map_update_kptr,
        map_update_kptr_case::MAP_UPDATE_KPTR_HASH_MALLOC => {
            (*skel).progs.test_hash_malloc_map_update_kptr
        }
    }
}

unsafe fn test_map_update_kptr(test: map_update_kptr_case) {
    let mut opts = bpf_test_run_opts::default();
    let skel: *mut map_kptr;
    let prog: *mut bpf_program;
    let ret: c_int;

    skel = map_kptr__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, c"map_kptr__open_and_load".as_ptr()) {
        return;
    }

    prog = map_update_kptr_prog(skel, test);
    if !ASSERT_OK_PTR(prog as *const c_void, c"map_update_kptr_prog".as_ptr()) {
        map_kptr__destroy(skel);
        wait_for_map_release();
        return;
    }

    ret = bpf_prog_test_run_opts(bpf_program__fd(prog), &mut opts);
    if !ASSERT_OK(ret, c"map_update_kptr".as_ptr()) {
        map_kptr__destroy(skel);
        wait_for_map_release();
        return;
    }
    if !ASSERT_OK(opts.retval as c_int, c"map_update_kptr retval".as_ptr()) {
        map_kptr__destroy(skel);
        wait_for_map_release();
        return;
    }

    ASSERT_EQ((*(*skel).bss).num_of_refs, 3, c"refs_after_update".as_ptr());

    map_kptr__destroy(skel);
    wait_for_map_release();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn serial_test_map_kptr() {
    let skel: *mut rcu_tasks_trace_gp;

    RUN_TESTS_map_kptr_fail();

    if test__start_subtest(c"update_array_map_kptr".as_ptr()) {
        test_map_update_kptr(map_update_kptr_case::MAP_UPDATE_KPTR_ARRAY);
    }
    if test__start_subtest(c"update_hash_map_kptr".as_ptr()) {
        test_map_update_kptr(map_update_kptr_case::MAP_UPDATE_KPTR_HASH);
    }
    if test__start_subtest(c"update_hash_malloc_map_kptr".as_ptr()) {
        test_map_update_kptr(map_update_kptr_case::MAP_UPDATE_KPTR_HASH_MALLOC);
    }

    skel = rcu_tasks_trace_gp__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        c"rcu_tasks_trace_gp__open_and_load".as_ptr(),
    ) {
        return;
    }

    if test__start_subtest(c"success-map".as_ptr()) {
        test_map_kptr_success(true);

        ASSERT_OK(kern_sync_rcu_tasks_trace(skel), c"sync rcu_tasks_trace".as_ptr());
        ASSERT_OK(kern_sync_rcu(), c"sync rcu".as_ptr());
        wait_for_map_release();

        /* Observe refcount dropping to 1 on bpf_map_free_deferred */
        test_map_kptr_success(false);

        ASSERT_OK(kern_sync_rcu_tasks_trace(skel), c"sync rcu_tasks_trace".as_ptr());
        ASSERT_OK(kern_sync_rcu(), c"sync rcu".as_ptr());
        wait_for_map_release();

        /* Observe refcount dropping to 1 on map release. */
        test_map_kptr_success(true);
    }

    rcu_tasks_trace_gp__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
