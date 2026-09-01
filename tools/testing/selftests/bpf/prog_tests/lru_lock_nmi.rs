// SPDX-License-Identifier: GPL-2.0
/*
 * Stress every LRU lock-failure and orphan-recovery.
 * perf_event NMI BPF on every online CPU does
 * update+delete on a small LRU map; userspace threads on every CPU do
 * the same from syscall context.
 */
/* C dependencies: pthread.h, sched.h, sys/syscall.h, linux/perf_event.h,
 * test_progs.h, testing_helpers.h, lru_lock_nmi.skel.h
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

const MAP_ENTRIES: c_int = 64;
const KEY_RANGE: c_int = MAP_ENTRIES * 2;
const STRESS_NS: __u64 = 500 * 1000 * 1000u64;

type __u32 = u32;
type __u64 = u64;
type size_t = usize;
type pthread_t = c_ulong;
type cpu_set_t = c_void;

#[repr(C)]
struct hammer_arg {
    map_fd: c_int,
    cpu: c_int,
    deadline_ns: __u64,
}

#[repr(C)]
struct refill_arg {
    map_fd: c_int,
    cpu: c_int,
    per_cpu_quota: c_int,
    update_errors: c_int,
}

#[repr(C)]
struct perf_event_attr {
    size: __u32,
    type_: __u32,
    config: __u64,
    sample_freq: __u64,
    freq: __u64,
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct lru_lock_nmi_maps {
    lru_map: *mut bpf_map,
}

#[repr(C)]
struct lru_lock_nmi_progs {
    oncpu: *mut bpf_program,
}

#[repr(C)]
struct lru_lock_nmi_bss {
    hits: __u64,
}

#[repr(C)]
struct lru_lock_nmi {
    maps: lru_lock_nmi_maps,
    progs: lru_lock_nmi_progs,
    bss: *mut lru_lock_nmi_bss,
}

#[repr(C)]
enum bpf_map_type {
    BPF_MAP_TYPE_LRU_HASH,
    BPF_MAP_TYPE_LRU_PERCPU_HASH,
}

const BPF_ANY: __u64 = 0;
const BPF_F_NO_COMMON_LRU: __u32 = 1;
const PERF_TYPE_HARDWARE: __u32 = 0;
const PERF_COUNT_HW_CPU_CYCLES: __u64 = 0;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const ENOENT: c_int = 2;
const EOPNOTSUPP: c_int = 95;
const __NR_perf_event_open: c_long = 298;

extern "C" {
    static mut errno: c_int;

    fn CPU_ALLOC(count: c_int) -> *mut cpu_set_t;
    fn CPU_ALLOC_SIZE(count: c_int) -> size_t;
    fn CPU_ZERO_S(setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_SET_S(cpu: c_int, setsize: size_t, cpusetp: *mut cpu_set_t);
    fn CPU_FREE(cpusetp: *mut cpu_set_t);

    fn pthread_setaffinity_np(thread: pthread_t, cpusetsize: size_t, cpuset: *const cpu_set_t) -> c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn rand_r(seedp: *mut c_uint) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;

    fn libbpf_num_possible_cpus() -> c_int;
    fn get_time_ns() -> __u64;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: __u64) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;

    fn ASSERT_GT(a: __u64, b: __u64, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn test__skip();
    fn read_perf_max_sample_freq() -> __u64;

    fn lru_lock_nmi__open() -> *mut lru_lock_nmi;
    fn lru_lock_nmi__load(skel: *mut lru_lock_nmi) -> c_int;
    fn lru_lock_nmi__destroy(skel: *mut lru_lock_nmi);
    fn bpf_map__set_type(map: *mut bpf_map, type_: bpf_map_type) -> c_int;
    fn bpf_map__set_map_flags(map: *mut bpf_map, map_flags: __u32) -> c_int;
    fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: c_int) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_program__attach_perf_event(prog: *mut bpf_program, pfd: c_int) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
}

/*
 * Pin the calling thread to @cpu. Uses dynamically-allocated CPU sets so
 * we stay correct on hosts with @cpu >= CPU_SETSIZE (default 1024).
 */
unsafe fn pin_to_cpu(cpu: c_int) -> c_int {
    let cs: *mut cpu_set_t;
    let cs_size: size_t;
    let err: c_int;

    cs = CPU_ALLOC(cpu + 1);
    if cs.is_null() {
        return -ENOMEM;
    }
    cs_size = CPU_ALLOC_SIZE(cpu + 1);

    CPU_ZERO_S(cs_size, cs);
    CPU_SET_S(cpu, cs_size, cs);
    err = pthread_setaffinity_np(pthread_self(), cs_size, cs);
    CPU_FREE(cs);
    err
}

unsafe extern "C" fn hammer_thread(p: *mut c_void) -> *mut c_void {
    let a: *mut hammer_arg = p as *mut hammer_arg;
    let nr_possible_cpus: c_int = libbpf_num_possible_cpus();
    let mut val: Vec<__u64> = vec![0; nr_possible_cpus as usize];
    let mut seed: c_uint;
    let mut key: __u32;

    memset(
        val.as_mut_ptr() as *mut c_void,
        0,
        val.len() * mem::size_of::<__u64>(),
    );
    pin_to_cpu((*a).cpu);

    seed = (*a).cpu as c_uint ^ pthread_self() as usize as c_uint;

    while get_time_ns() < (*a).deadline_ns {
        let do_update: bool = (rand_r(&mut seed) & 1) != 0;

        key = (rand_r(&mut seed) % KEY_RANGE) as __u32;
        if do_update {
            bpf_map_update_elem((*a).map_fd, &key as *const _ as *const c_void, val.as_ptr() as *const c_void, BPF_ANY);
        } else {
            bpf_map_delete_elem((*a).map_fd, &key as *const _ as *const c_void);
        }
    }
    ptr::null_mut()
}

unsafe extern "C" fn refill_thread(p: *mut c_void) -> *mut c_void {
    let a: *mut refill_arg = p as *mut refill_arg;
    let nr_possible_cpus: c_int = libbpf_num_possible_cpus();
    let mut val: Vec<__u64> = vec![0; nr_possible_cpus as usize];
    let mut start: __u32;
    let end: __u32;
    let mut key: __u32;

    memset(
        val.as_mut_ptr() as *mut c_void,
        0,
        val.len() * mem::size_of::<__u64>(),
    );
    pin_to_cpu((*a).cpu);

    start = (*a).cpu as __u32 * (*a).per_cpu_quota as __u32;
    end = start + (*a).per_cpu_quota as __u32;
    key = start;
    while key < end {
        if bpf_map_update_elem((*a).map_fd, &key as *const _ as *const c_void, val.as_ptr() as *const c_void, BPF_ANY) != 0 {
            (*a).update_errors += 1;
        }
        key += 1;
    }
    ptr::null_mut()
}

/*
 * Drain the map, then refill it with each CPU inserting only its own
 * quota of keys.
 * After refill, lookup every key we inserted - a stranded node on any
 * CPU's pool would have forced eviction.
 */
unsafe fn drain_then_verify_capacity(map_fd: c_int, nr_cpus: c_int) -> c_int {
    let per_cpu_quota: c_int = MAP_ENTRIES / nr_cpus;
    let total: c_int = per_cpu_quota * nr_cpus;
    let nr_possible_cpus: c_int = libbpf_num_possible_cpus();
    let mut threads: Vec<pthread_t> = vec![0; nr_cpus as usize];
    let mut args: Vec<refill_arg> = (0..nr_cpus)
        .map(|_| refill_arg {
            map_fd: 0,
            cpu: 0,
            per_cpu_quota: 0,
            update_errors: 0,
        })
        .collect();
    let mut val: Vec<__u64> = vec![0; nr_possible_cpus as usize];
    let mut i: c_int;
    let mut hits: c_int = 0;
    let mut nthreads: c_int = 0;
    let mut key: __u32;

    memset(
        val.as_mut_ptr() as *mut c_void,
        0,
        val.len() * mem::size_of::<__u64>(),
    );

    key = 0;
    while key < KEY_RANGE as __u32 {
        bpf_map_delete_elem(map_fd, &key as *const _ as *const c_void);
        key += 1;
    }

    i = 0;
    while i < nr_cpus {
        args[i as usize] = refill_arg {
            map_fd,
            cpu: i,
            per_cpu_quota,
            update_errors: 0,
        };
        if pthread_create(
            &mut threads[nthreads as usize],
            ptr::null(),
            refill_thread,
            &mut args[i as usize] as *mut _ as *mut c_void,
        ) == 0
        {
            nthreads += 1;
        }
        i += 1;
    }
    i = 0;
    while i < nthreads {
        pthread_join(threads[i as usize], ptr::null_mut());
        i += 1;
    }

    i = 0;
    while i < nr_cpus {
        if args[i as usize].update_errors != 0 {
            return -ENOMEM;
        }
        i += 1;
    }

    key = 0;
    while key < total as __u32 {
        if bpf_map_lookup_elem(map_fd, &key as *const _ as *const c_void, val.as_mut_ptr() as *mut c_void) == 0 {
            hits += 1;
        }
        key += 1;
    }

    if hits == total {
        0
    } else {
        -EIO
    }
}

unsafe fn run_variant(type_: bpf_map_type, map_flags: __u32, name: *const c_char) {
    let mut attr: perf_event_attr = perf_event_attr {
        size: mem::size_of::<perf_event_attr>() as __u32,
        type_: PERF_TYPE_HARDWARE,
        config: PERF_COUNT_HW_CPU_CYCLES,
        sample_freq: 0,
        freq: 1,
    };
    let mut nr_cpus: c_int;
    let max_cpus: c_int = 64;
    let mut links: Vec<*mut bpf_link> = vec![ptr::null_mut(); max_cpus as usize];
    let mut threads: Vec<pthread_t> = vec![0; max_cpus as usize];
    let mut args: Vec<hammer_arg> = (0..max_cpus)
        .map(|_| hammer_arg {
            map_fd: 0,
            cpu: 0,
            deadline_ns: 0,
        })
        .collect();
    let mut skel: *mut lru_lock_nmi = ptr::null_mut();
    let map_fd: c_int;
    let mut i: c_int;
    let mut err: c_int;
    let mut nr_threads: c_int = 0;
    let mut pmu_fd: c_int = -1;
    let deadline: __u64;

    nr_cpus = libbpf_num_possible_cpus();
    if !ASSERT_GT(nr_cpus as __u64, 0, c"num_cpus".as_ptr()) {
        return;
    }

    if nr_cpus > max_cpus {
        nr_cpus = max_cpus;
    }

    if !test__start_subtest(name) {
        return;
    }

    memset(
        links.as_mut_ptr() as *mut c_void,
        0,
        links.len() * mem::size_of::<*mut bpf_link>(),
    );
    skel = lru_lock_nmi__open();
    if !ASSERT_OK_PTR(skel as *const c_void, c"skel_open".as_ptr()) {
        goto_cleanup(nr_cpus, &mut links, skel);
        return;
    }

    err = bpf_map__set_type((*skel).maps.lru_map, type_);
    if !ASSERT_OK(err, c"set_type".as_ptr()) {
        goto_cleanup(nr_cpus, &mut links, skel);
        return;
    }
    err = bpf_map__set_map_flags((*skel).maps.lru_map, map_flags);
    if !ASSERT_OK(err, c"set_flags".as_ptr()) {
        goto_cleanup(nr_cpus, &mut links, skel);
        return;
    }
    err = bpf_map__set_max_entries((*skel).maps.lru_map, MAP_ENTRIES);
    if !ASSERT_OK(err, c"set_max_entries".as_ptr()) {
        goto_cleanup(nr_cpus, &mut links, skel);
        return;
    }

    err = lru_lock_nmi__load(skel);
    if !ASSERT_OK(err, c"skel_load".as_ptr()) {
        goto_cleanup(nr_cpus, &mut links, skel);
        return;
    }

    (*(*skel).bss).hits = 0;
    map_fd = bpf_map__fd((*skel).maps.lru_map);
    attr.sample_freq = read_perf_max_sample_freq();

    i = 0;
    while i < nr_cpus {
        pmu_fd = syscall(__NR_perf_event_open, &mut attr as *mut _, -1, i, -1, 0) as c_int;
        if pmu_fd < 0 {
            if i == 0 && (errno == ENOENT || errno == EOPNOTSUPP) {
                test__skip();
                goto_cleanup(nr_cpus, &mut links, skel);
                return;
            }
            i += 1;
            continue;
        }
        /* libbpf takes ownership of pfd on success */
        links[i as usize] = bpf_program__attach_perf_event((*skel).progs.oncpu, pmu_fd);
        if links[i as usize].is_null() {
            close(pmu_fd);
        }
        i += 1;
    }

    deadline = get_time_ns() + STRESS_NS;
    i = 0;
    while i < nr_cpus {
        args[i as usize].map_fd = map_fd;
        args[i as usize].cpu = i;
        args[i as usize].deadline_ns = deadline;
        if pthread_create(
            &mut threads[nr_threads as usize],
            ptr::null(),
            hammer_thread,
            &mut args[i as usize] as *mut _ as *mut c_void,
        ) == 0
        {
            nr_threads += 1;
        }
        i += 1;
    }
    i = 0;
    while i < nr_threads {
        pthread_join(threads[i as usize], ptr::null_mut());
        i += 1;
    }

    i = 0;
    while i < nr_cpus {
        if !links[i as usize].is_null() {
            bpf_link__destroy(links[i as usize]);
            links[i as usize] = ptr::null_mut();
        }
        i += 1;
    }

    ASSERT_GT((*(*skel).bss).hits, 0, c"nmi_bpf_ran".as_ptr());
    ASSERT_OK(drain_then_verify_capacity(map_fd, nr_cpus), c"drain_then_verify_capacity".as_ptr());

    goto_cleanup(nr_cpus, &mut links, skel);
}

unsafe fn goto_cleanup(nr_cpus: c_int, links: &mut Vec<*mut bpf_link>, skel: *mut lru_lock_nmi) {
    let mut i: c_int = 0;

    while i < nr_cpus {
        if !links[i as usize].is_null() {
            bpf_link__destroy(links[i as usize]);
        }
        i += 1;
    }
    lru_lock_nmi__destroy(skel);
}

pub unsafe fn serial_test_lru_lock_nmi() {
    run_variant(bpf_map_type::BPF_MAP_TYPE_LRU_HASH, 0, c"common_lru".as_ptr());
    run_variant(
        bpf_map_type::BPF_MAP_TYPE_LRU_HASH,
        BPF_F_NO_COMMON_LRU,
        c"no_common_lru".as_ptr(),
    );
    run_variant(
        bpf_map_type::BPF_MAP_TYPE_LRU_PERCPU_HASH,
        0,
        c"percpu_lru".as_ptr(),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
