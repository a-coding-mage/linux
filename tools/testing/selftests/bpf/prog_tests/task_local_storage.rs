// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

/* Translated from C. Original includes:
 * <unistd.h>, <sched.h>, <pthread.h>, <sys/syscall.h>, <sys/types.h>,
 * <sys/eventfd.h>, <sys/mman.h>, <test_progs.h>, <bpf/btf.h>,
 * "task_local_storage_helpers.h", "task_local_storage.skel.h",
 * "task_local_storage_exit_creds.skel.h", "task_ls_recursion.skel.h",
 * "task_storage_nodeadlock.skel.h", "uptr_test_common.h",
 * "task_ls_uptr.skel.h", "uptr_update_failure.skel.h",
 * "uptr_failure.skel.h", "uptr_map_failure.skel.h"
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::{offset_of, size_of, zeroed};
use core::ptr;

unsafe fn test_sys_enter_exit() {
    let mut skel: *mut task_local_storage;
    let pid: pid_t = sys_gettid();
    let mut err: c_int;

    skel = task_local_storage__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"skel_open_and_load".as_ptr()) {
        return;
    }

    err = task_local_storage__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        goto_out_test_sys_enter_exit(skel);
        return;
    }

    /* Set target_pid after attach so that syscalls made during
     * attach are not counted.
     */
    (*(*skel).bss).target_pid = pid;

    sys_gettid();
    sys_gettid();

    (*(*skel).bss).target_pid = 0;

    /* 2x gettid syscalls */
    ASSERT_EQ((*(*skel).bss).update_err, 0, c"update_err".as_ptr());
    ASSERT_EQ((*(*skel).bss).enter_cnt, 2, c"enter_cnt".as_ptr());
    ASSERT_EQ((*(*skel).bss).exit_cnt, 2, c"exit_cnt".as_ptr());
    ASSERT_EQ((*(*skel).bss).mismatch_cnt, 0, c"mismatch_cnt".as_ptr());

    goto_out_test_sys_enter_exit(skel);
}

unsafe fn goto_out_test_sys_enter_exit(skel: *mut task_local_storage) {
    task_local_storage__destroy(skel);
}

unsafe fn test_exit_creds() {
    let mut skel: *mut task_local_storage_exit_creds;
    let mut err: c_int;
    let mut run_count: c_int;
    let mut sync_rcu_calls: c_int = 0;
    const MAX_SYNC_RCU_CALLS: c_int = 1000;

    skel = task_local_storage_exit_creds__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"skel_open_and_load".as_ptr()) {
        return;
    }

    err = task_local_storage_exit_creds__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        goto_out_test_exit_creds(skel);
        return;
    }

    /* trigger at least one exit_creds() */
    if CHECK_FAIL(system(c"ls > /dev/null".as_ptr())) {
        goto_out_test_exit_creds(skel);
        return;
    }

    /* kern_sync_rcu is not enough on its own as the read section we want
     * to wait for may start after we enter synchronize_rcu, so our call
     * won't wait for the section to finish. Loop on the run counter
     * as well to ensure the program has run.
     */
    loop {
        kern_sync_rcu();
        run_count = ptr::read_volatile(&(*(*skel).bss).run_count);
        if !(run_count == 0 && {
            sync_rcu_calls += 1;
            sync_rcu_calls < MAX_SYNC_RCU_CALLS
        }) {
            break;
        }
    }

    ASSERT_NEQ(
        sync_rcu_calls,
        MAX_SYNC_RCU_CALLS,
        c"sync_rcu count too high".as_ptr(),
    );
    ASSERT_NEQ(run_count, 0, c"run_count".as_ptr());
    ASSERT_EQ((*(*skel).bss).valid_ptr_count, 0, c"valid_ptr_count".as_ptr());
    ASSERT_NEQ((*(*skel).bss).null_ptr_count, 0, c"null_ptr_count".as_ptr());

    goto_out_test_exit_creds(skel);
}

unsafe fn goto_out_test_exit_creds(skel: *mut task_local_storage_exit_creds) {
    task_local_storage_exit_creds__destroy(skel);
}

unsafe fn test_recursion() {
    let mut err: c_int;
    let mut map_fd: c_int;
    let mut prog_fd: c_int;
    let task_fd: c_int;
    let mut skel: *mut task_ls_recursion;
    let mut info: bpf_prog_info = zeroed();
    let mut info_len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let mut value: c_long = 0;

    task_fd = sys_pidfd_open(getpid(), 0);
    if !ASSERT_NEQ(task_fd, -1, c"sys_pidfd_open".as_ptr()) {
        return;
    }

    skel = task_ls_recursion__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"skel_open_and_load".as_ptr()) {
        goto_out_test_recursion(task_fd, skel);
        return;
    }

    err = task_ls_recursion__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        goto_out_test_recursion(task_fd, skel);
        return;
    }

    /* trigger sys_enter, make sure it does not cause deadlock */
    (*(*skel).bss).test_pid = getpid();
    sys_gettid();
    (*(*skel).bss).test_pid = 0;
    task_ls_recursion__detach(skel);

    /* Refer to the comment in BPF_PROG(on_update) for
     * the explanation on the value 200 and 1.
     */
    map_fd = bpf_map__fd((*skel).maps.map_a);
    err = bpf_map_lookup_elem(map_fd, &task_fd as *const _ as *const c_void, &mut value as *mut _ as *mut c_void);
    ASSERT_OK(err, c"lookup map_a".as_ptr());
    ASSERT_EQ(value, 200, c"map_a value".as_ptr());
    ASSERT_EQ((*(*skel).bss).nr_del_errs, 0, c"bpf_task_storage_delete busy".as_ptr());

    map_fd = bpf_map__fd((*skel).maps.map_b);
    err = bpf_map_lookup_elem(map_fd, &task_fd as *const _ as *const c_void, &mut value as *mut _ as *mut c_void);
    ASSERT_OK(err, c"lookup map_b".as_ptr());
    ASSERT_EQ(value, 1, c"map_b value".as_ptr());

    prog_fd = bpf_program__fd((*skel).progs.on_update);
    ptr::write_bytes(&mut info as *mut bpf_prog_info as *mut u8, 0, size_of::<bpf_prog_info>());
    err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len);
    ASSERT_OK(err, c"get prog info".as_ptr());
    ASSERT_EQ(info.recursion_misses, 2, c"on_update prog recursion".as_ptr());

    prog_fd = bpf_program__fd((*skel).progs.on_enter);
    ptr::write_bytes(&mut info as *mut bpf_prog_info as *mut u8, 0, size_of::<bpf_prog_info>());
    err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len);
    ASSERT_OK(err, c"get prog info".as_ptr());
    ASSERT_EQ(info.recursion_misses, 0, c"on_enter prog recursion".as_ptr());

    goto_out_test_recursion(task_fd, skel);
}

unsafe fn goto_out_test_recursion(task_fd: c_int, skel: *mut task_ls_recursion) {
    close(task_fd);
    task_ls_recursion__destroy(skel);
}

static mut stop: bool = false;

unsafe fn waitall(tids: *const pthread_t, nr: c_int) {
    let mut i: c_int;

    stop = true;
    i = 0;
    while i < nr {
        pthread_join(*tids.add(i as usize), ptr::null_mut());
        i += 1;
    }
}

unsafe extern "C" fn sock_create_loop(arg: *mut c_void) -> *mut c_void {
    let skel: *mut task_storage_nodeadlock = arg as *mut task_storage_nodeadlock;
    let mut fd: c_int;

    while !stop {
        fd = socket(AF_INET, SOCK_STREAM, 0);
        close(fd);
        if (*(*skel).bss).nr_get_errs != 0 || (*(*skel).bss).nr_del_errs != 0 {
            stop = true;
        }
    }

    ptr::null_mut()
}

unsafe fn test_nodeadlock() {
    let mut skel: *mut task_storage_nodeadlock;
    let mut info: bpf_prog_info = zeroed();
    let mut info_len: __u32 = size_of::<bpf_prog_info>() as __u32;
    const nr_threads: c_int = 32;
    let mut tids: [pthread_t; nr_threads as usize] = zeroed();
    let mut i: c_int;
    let mut prog_fd: c_int;
    let mut err: c_int;
    let mut old: cpu_set_t = zeroed();
    let mut new: cpu_set_t = zeroed();

    /* Pin all threads to one cpu to increase the chance of preemption
     * in a sleepable bpf prog.
     */
    CPU_ZERO(&mut new);
    CPU_SET(0, &mut new);
    err = sched_getaffinity(getpid(), size_of::<cpu_set_t>(), &mut old);
    if !ASSERT_OK(err, c"getaffinity".as_ptr()) {
        return;
    }
    err = sched_setaffinity(getpid(), size_of::<cpu_set_t>(), &new);
    if !ASSERT_OK(err, c"setaffinity".as_ptr()) {
        return;
    }

    skel = task_storage_nodeadlock__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"open_and_load".as_ptr()) {
        goto_done_test_nodeadlock(skel, &old);
        return;
    }

    /* Unnecessary recursion and deadlock detection are reproducible
     * in the preemptible kernel.
     */
    if !(*(*skel).kconfig).CONFIG_PREEMPTION {
        test__skip();
        goto_done_test_nodeadlock(skel, &old);
        return;
    }

    err = task_storage_nodeadlock__attach(skel);
    ASSERT_OK(err, c"attach prog".as_ptr());

    i = 0;
    while i < nr_threads {
        err = pthread_create(
            &mut tids[i as usize],
            ptr::null(),
            Some(sock_create_loop),
            skel as *mut c_void,
        );
        if err != 0 {
            /* Only assert once here to avoid excessive
             * PASS printing during test failure.
             */
            ASSERT_OK(err, c"pthread_create".as_ptr());
            waitall(tids.as_ptr(), i);
            goto_done_test_nodeadlock(skel, &old);
            return;
        }
        i += 1;
    }

    /* With 32 threads, 1s is enough to reproduce the issue */
    sleep(1);
    waitall(tids.as_ptr(), nr_threads);

    info_len = size_of::<bpf_prog_info>() as __u32;
    prog_fd = bpf_program__fd((*skel).progs.socket_post_create);
    err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len);
    ASSERT_OK(err, c"get prog info".as_ptr());
    ASSERT_EQ(info.recursion_misses, 0, c"prog recursion".as_ptr());

    ASSERT_EQ((*(*skel).bss).nr_get_errs, 0, c"bpf_task_storage_get busy".as_ptr());
    ASSERT_EQ((*(*skel).bss).nr_del_errs, 0, c"bpf_task_storage_delete busy".as_ptr());

    goto_done_test_nodeadlock(skel, &old);
}

unsafe fn goto_done_test_nodeadlock(skel: *mut task_storage_nodeadlock, old: *const cpu_set_t) {
    task_storage_nodeadlock__destroy(skel);
    sched_setaffinity(getpid(), size_of::<cpu_set_t>(), old);
}

#[repr(align(16))]
struct AlignedUserData(user_data);

static mut udata: AlignedUserData = AlignedUserData(user_data {
    a: 1,
    b: 2,
    result: 0,
    nested_result: 0,
});

static mut udata2: AlignedUserData = AlignedUserData(user_data {
    a: 3,
    b: 4,
    result: 0,
    nested_result: 0,
});

unsafe fn check_udata2(expected: c_int) {
    udata2.0.result = 0;
    udata2.0.nested_result = 0;
    usleep(1);
    ASSERT_EQ(udata2.0.result, expected, c"udata2.result".as_ptr());
    ASSERT_EQ(udata2.0.nested_result, expected, c"udata2.nested_result".as_ptr());
}

unsafe fn test_uptr_basic() {
    let mut map_fd: c_int;
    let parent_task_fd: c_int;
    let ev_fd: c_int;
    let mut value: value_type = zeroed();
    let mut skel: *mut task_ls_uptr;
    let child_pid: pid_t;
    let my_tid: pid_t;
    let mut ev_dummy_data: __u64 = 1;
    let mut err: c_int;

    my_tid = sys_gettid();
    parent_task_fd = sys_pidfd_open(my_tid, 0);
    if !ASSERT_OK_FD(parent_task_fd, c"parent_task_fd".as_ptr()) {
        return;
    }

    ev_fd = eventfd(0, 0);
    if !ASSERT_OK_FD(ev_fd, c"ev_fd".as_ptr()) {
        close(parent_task_fd);
        return;
    }

    skel = task_ls_uptr__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"skel_open_and_load".as_ptr()) {
        goto_out_test_uptr_basic(skel, ev_fd, parent_task_fd);
        return;
    }

    map_fd = bpf_map__fd((*skel).maps.datamap);
    value.udata = &mut udata.0;
    value.nested.udata = &mut udata.0;
    err = bpf_map_update_elem(
        map_fd,
        &parent_task_fd as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        BPF_NOEXIST,
    );
    if !ASSERT_OK(err, c"update_elem(udata)".as_ptr()) {
        goto_out_test_uptr_basic(skel, ev_fd, parent_task_fd);
        return;
    }

    err = task_ls_uptr__attach(skel);
    if !ASSERT_OK(err, c"skel_attach".as_ptr()) {
        goto_out_test_uptr_basic(skel, ev_fd, parent_task_fd);
        return;
    }

    child_pid = fork();
    if !ASSERT_NEQ(child_pid, -1, c"fork".as_ptr()) {
        goto_out_test_uptr_basic(skel, ev_fd, parent_task_fd);
        return;
    }

    /* Call syscall in the child process, but access the map value of
     * the parent process in the BPF program to check if the user kptr
     * is translated/mapped correctly.
     */
    if child_pid == 0 {
        /* child */

        /* Overwrite the user_data in the child process to check if
         * the BPF program accesses the user_data of the parent.
         */
        udata.0.a = 0;
        udata.0.b = 0;

        /* Wait for the parent to set child_pid */
        read(
            ev_fd,
            &mut ev_dummy_data as *mut _ as *mut c_void,
            size_of::<__u64>(),
        );
        exit(0);
    }

    (*(*skel).bss).parent_pid = my_tid;
    (*(*skel).bss).target_pid = child_pid;

    write(
        ev_fd,
        &ev_dummy_data as *const _ as *const c_void,
        size_of::<__u64>(),
    );

    err = waitpid(child_pid, ptr::null_mut(), 0);
    ASSERT_EQ(err, child_pid, c"waitpid".as_ptr());
    ASSERT_EQ(udata.0.result, MAGIC_VALUE + udata.0.a + udata.0.b, c"udata.result".as_ptr());
    ASSERT_EQ(
        udata.0.nested_result,
        MAGIC_VALUE + udata.0.a + udata.0.b,
        c"udata.nested_result".as_ptr(),
    );

    (*(*skel).bss).target_pid = my_tid;

    /* update_elem: uptr changes from udata1 to udata2 */
    value.udata = &mut udata2.0;
    value.nested.udata = &mut udata2.0;
    err = bpf_map_update_elem(
        map_fd,
        &parent_task_fd as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        BPF_EXIST,
    );
    if !ASSERT_OK(err, c"update_elem(udata2)".as_ptr()) {
        goto_out_test_uptr_basic(skel, ev_fd, parent_task_fd);
        return;
    }
    check_udata2(MAGIC_VALUE + udata2.0.a + udata2.0.b);

    /* update_elem: uptr changes from udata2 uptr to NULL */
    ptr::write_bytes(&mut value as *mut value_type as *mut u8, 0, size_of::<value_type>());
    err = bpf_map_update_elem(
        map_fd,
        &parent_task_fd as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        BPF_EXIST,
    );
    if !ASSERT_OK(err, c"update_elem(udata2)".as_ptr()) {
        goto_out_test_uptr_basic(skel, ev_fd, parent_task_fd);
        return;
    }
    check_udata2(0);

    /* update_elem: uptr changes from NULL to udata2 */
    value.udata = &mut udata2.0;
    value.nested.udata = &mut udata2.0;
    err = bpf_map_update_elem(
        map_fd,
        &parent_task_fd as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        BPF_EXIST,
    );
    if !ASSERT_OK(err, c"update_elem(udata2)".as_ptr()) {
        goto_out_test_uptr_basic(skel, ev_fd, parent_task_fd);
        return;
    }
    check_udata2(MAGIC_VALUE + udata2.0.a + udata2.0.b);

    /* Check if user programs can access the value of user kptrs
     * through bpf_map_lookup_elem(). Make sure the kernel value is not
     * leaked.
     */
    err = bpf_map_lookup_elem(
        map_fd,
        &parent_task_fd as *const _ as *const c_void,
        &mut value as *mut _ as *mut c_void,
    );
    if !ASSERT_OK(err, c"bpf_map_lookup_elem".as_ptr()) {
        goto_out_test_uptr_basic(skel, ev_fd, parent_task_fd);
        return;
    }
    ASSERT_EQ(value.udata, ptr::null_mut(), c"value.udata".as_ptr());
    ASSERT_EQ(value.nested.udata, ptr::null_mut(), c"value.nested.udata".as_ptr());

    /* delete_elem */
    err = bpf_map_delete_elem(map_fd, &parent_task_fd as *const _ as *const c_void);
    ASSERT_OK(err, c"delete_elem(udata2)".as_ptr());
    check_udata2(0);

    /* update_elem: add uptr back to test map_free */
    value.udata = &mut udata2.0;
    value.nested.udata = &mut udata2.0;
    err = bpf_map_update_elem(
        map_fd,
        &parent_task_fd as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        BPF_NOEXIST,
    );
    ASSERT_OK(err, c"update_elem(udata2)".as_ptr());

    goto_out_test_uptr_basic(skel, ev_fd, parent_task_fd);
}

unsafe fn goto_out_test_uptr_basic(skel: *mut task_ls_uptr, ev_fd: c_int, parent_task_fd: c_int) {
    task_ls_uptr__destroy(skel);
    close(ev_fd);
    close(parent_task_fd);
}

unsafe fn test_uptr_across_pages() {
    let page_size: c_int = getpagesize();
    let mut value: value_type = zeroed();
    let mut skel: *mut task_ls_uptr;
    let mut err: c_int;
    let task_fd: c_int;
    let mut map_fd: c_int;
    let mem: *mut c_void;

    task_fd = sys_pidfd_open(getpid(), 0);
    if !ASSERT_OK_FD(task_fd, c"task_fd".as_ptr()) {
        return;
    }

    mem = mmap(
        ptr::null_mut(),
        (page_size * 2) as usize,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    );
    if !ASSERT_OK_PTR(mem, c"mmap(page_size * 2)".as_ptr()) {
        close(task_fd);
        return;
    }

    skel = task_ls_uptr__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"skel_open_and_load".as_ptr()) {
        goto_out_test_uptr_across_pages(skel, task_fd, mem, page_size);
        return;
    }

    map_fd = bpf_map__fd((*skel).maps.datamap);
    value.udata = (mem as *mut u8)
        .add(page_size as usize)
        .sub(offset_of!(user_data, b)) as *mut user_data;
    err = bpf_map_update_elem(
        map_fd,
        &task_fd as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        0,
    );
    if !ASSERT_ERR(err, c"update_elem(udata)".as_ptr()) {
        goto_out_test_uptr_across_pages(skel, task_fd, mem, page_size);
        return;
    }
    ASSERT_EQ(*__errno_location(), EOPNOTSUPP, c"errno".as_ptr());

    value.udata = (mem as *mut u8)
        .add(page_size as usize)
        .sub(size_of::<user_data>()) as *mut user_data;
    err = bpf_map_update_elem(
        map_fd,
        &task_fd as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        0,
    );
    ASSERT_OK(err, c"update_elem(udata)".as_ptr());

    goto_out_test_uptr_across_pages(skel, task_fd, mem, page_size);
}

unsafe fn goto_out_test_uptr_across_pages(
    skel: *mut task_ls_uptr,
    task_fd: c_int,
    mem: *mut c_void,
    page_size: c_int,
) {
    task_ls_uptr__destroy(skel);
    close(task_fd);
    munmap(mem, (page_size * 2) as usize);
}

unsafe fn test_uptr_update_failure() {
    let mut value: value_lock_type = zeroed();
    let mut skel: *mut uptr_update_failure;
    let mut err: c_int;
    let task_fd: c_int;
    let mut map_fd: c_int;

    task_fd = sys_pidfd_open(getpid(), 0);
    if !ASSERT_OK_FD(task_fd, c"task_fd".as_ptr()) {
        return;
    }

    skel = uptr_update_failure__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"skel_open_and_load".as_ptr()) {
        goto_out_test_uptr_update_failure(skel, task_fd);
        return;
    }

    map_fd = bpf_map__fd((*skel).maps.datamap);

    value.udata = &mut udata.0;
    err = bpf_map_update_elem(
        map_fd,
        &task_fd as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        BPF_F_LOCK,
    );
    if !ASSERT_ERR(err, c"update_elem(udata, BPF_F_LOCK)".as_ptr()) {
        goto_out_test_uptr_update_failure(skel, task_fd);
        return;
    }
    ASSERT_EQ(*__errno_location(), EOPNOTSUPP, c"errno".as_ptr());

    err = bpf_map_update_elem(
        map_fd,
        &task_fd as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        BPF_EXIST,
    );
    if !ASSERT_ERR(err, c"update_elem(udata, BPF_EXIST)".as_ptr()) {
        goto_out_test_uptr_update_failure(skel, task_fd);
        return;
    }
    ASSERT_EQ(*__errno_location(), ENOENT, c"errno".as_ptr());

    err = bpf_map_update_elem(
        map_fd,
        &task_fd as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        BPF_NOEXIST,
    );
    if !ASSERT_OK(err, c"update_elem(udata, BPF_NOEXIST)".as_ptr()) {
        goto_out_test_uptr_update_failure(skel, task_fd);
        return;
    }

    value.udata = &mut udata2.0;
    err = bpf_map_update_elem(
        map_fd,
        &task_fd as *const _ as *const c_void,
        &value as *const _ as *const c_void,
        BPF_NOEXIST,
    );
    if !ASSERT_ERR(err, c"update_elem(udata2, BPF_NOEXIST)".as_ptr()) {
        goto_out_test_uptr_update_failure(skel, task_fd);
        return;
    }
    ASSERT_EQ(*__errno_location(), EEXIST, c"errno".as_ptr());

    goto_out_test_uptr_update_failure(skel, task_fd);
}

unsafe fn goto_out_test_uptr_update_failure(skel: *mut uptr_update_failure, task_fd: c_int) {
    uptr_update_failure__destroy(skel);
    close(task_fd);
}

unsafe fn test_uptr_map_failure(map_name: *const c_char, expected_errno: c_int) {
    let mut create_attr: bpf_map_create_opts = zeroed();
    let mut skel: *mut uptr_map_failure;
    let mut map: *mut bpf_map;
    let mut btf: *mut btf;
    let map_fd: c_int;
    let mut err: c_int;

    skel = uptr_map_failure__open();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"uptr_map_failure__open".as_ptr()) {
        return;
    }

    map = bpf_object__find_map_by_name((*skel).obj, map_name);
    btf = bpf_object__btf((*skel).obj);
    err = btf__load_into_kernel(btf);
    if !ASSERT_OK(err, c"btf__load_into_kernel".as_ptr()) {
        goto_done_test_uptr_map_failure(skel);
        return;
    }

    create_attr.map_flags = bpf_map__map_flags(map);
    create_attr.btf_fd = btf__fd(btf);
    create_attr.btf_key_type_id = bpf_map__btf_key_type_id(map);
    create_attr.btf_value_type_id = bpf_map__btf_value_type_id(map);
    map_fd = bpf_map_create(
        bpf_map__type(map),
        map_name,
        bpf_map__key_size(map),
        bpf_map__value_size(map),
        0,
        &mut create_attr,
    );
    if ASSERT_ERR_FD(map_fd, c"map_create".as_ptr()) {
        ASSERT_EQ(*__errno_location(), expected_errno, c"errno".as_ptr());
    } else {
        close(map_fd);
    }

    goto_done_test_uptr_map_failure(skel);
}

unsafe fn goto_done_test_uptr_map_failure(skel: *mut uptr_map_failure) {
    uptr_map_failure__destroy(skel);
}

pub unsafe fn test_task_local_storage() {
    if test__start_subtest(c"sys_enter_exit".as_ptr()) {
        test_sys_enter_exit();
    }
    if test__start_subtest(c"exit_creds".as_ptr()) {
        test_exit_creds();
    }
    if test__start_subtest(c"recursion".as_ptr()) {
        test_recursion();
    }
    if test__start_subtest(c"nodeadlock".as_ptr()) {
        test_nodeadlock();
    }
    if test__start_subtest(c"uptr_basic".as_ptr()) {
        test_uptr_basic();
    }
    if test__start_subtest(c"uptr_across_pages".as_ptr()) {
        test_uptr_across_pages();
    }
    if test__start_subtest(c"uptr_update_failure".as_ptr()) {
        test_uptr_update_failure();
    }
    if test__start_subtest(c"uptr_map_failure_e2big".as_ptr()) {
        if getpagesize() == PAGE_SIZE {
            test_uptr_map_failure(c"large_uptr_map".as_ptr(), E2BIG);
        } else {
            test__skip();
        }
    }
    if test__start_subtest(c"uptr_map_failure_size0".as_ptr()) {
        test_uptr_map_failure(c"empty_uptr_map".as_ptr(), EINVAL);
    }
    if test__start_subtest(c"uptr_map_failure_kstruct".as_ptr()) {
        test_uptr_map_failure(c"kstruct_uptr_map".as_ptr(), EINVAL);
    }
    RUN_TESTS(c"uptr_failure".as_ptr());
}

