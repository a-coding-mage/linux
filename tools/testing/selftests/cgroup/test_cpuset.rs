// SPDX-License-Identifier: GPL-2.0

// C dependencies: assert.h, linux/limits.h, pthread.h, sched.h, signal.h,
// sys/syscall.h, unistd.h, kselftest.h, cgroup_util.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

const TEST_UID: libc::uid_t = 65534;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;
const PATH_MAX: usize = 4096;
const WEXITED: c_int = 4;

unsafe extern "C" {
    fn pause() -> c_int;
    fn setuid(uid: libc::uid_t) -> c_int;
    fn chown(path: *const c_char, owner: libc::uid_t, group: libc::gid_t) -> c_int;
    fn kill(pid: libc::pid_t, sig: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn sched_getaffinity(pid: libc::pid_t, cpusetsize: usize, mask: *mut libc::cpu_set_t) -> c_int;

    fn pthread_create(
        thread: *mut libc::pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: libc::pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_mutex_lock(mutex: *mut libc::pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut libc::pthread_mutex_t) -> c_int;
    fn pthread_cond_wait(cond: *mut libc::pthread_cond_t, mutex: *mut libc::pthread_mutex_t) -> c_int;
    fn pthread_cond_broadcast(cond: *mut libc::pthread_cond_t) -> c_int;

    fn cg_name(root: *const c_char, name: *const c_char) -> *mut c_char;
    fn cg_create(cgroup: *const c_char) -> c_int;
    fn cg_destroy(cgroup: *const c_char) -> c_int;
    fn cg_write(cgroup: *const c_char, control: *const c_char, value: *const c_char) -> c_int;
    fn cg_read_strstr(cgroup: *const c_char, control: *const c_char, needle: *const c_char) -> c_int;
    fn cg_enter(cgroup: *const c_char, pid: libc::pid_t) -> c_int;
    fn cg_enter_current(cgroup: *const c_char) -> c_int;
    fn cg_enter_current_thread(cgroup: *const c_char) -> c_int;
    fn cg_wait_for_proc_count(cgroup: *const c_char, count: c_int) -> c_int;
    fn cg_run_nowait(
        cgroup: *const c_char,
        fn_: unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn cg_run(
        cgroup: *const c_char,
        fn_: unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn clone_reap(pid: libc::pid_t, options: c_int) -> c_int;
    fn cg_find_unified_root(root: *mut c_char, len: usize, mount: *mut c_void) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_exit_skip(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_finished();
}

unsafe extern "C" fn idle_process_fn(_cgroup: *const c_char, _arg: *mut c_void) -> c_int {
    unsafe {
        let _ = pause();
    }
    0
}

unsafe extern "C" fn do_migration_fn(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let object_pid = arg as usize as c_int;

    unsafe {
        if setuid(TEST_UID) != 0 {
            return EXIT_FAILURE;
        }

        // XXX checking /proc/$pid/cgroup would be quicker than wait
        if cg_enter(cgroup, object_pid) != 0 || cg_wait_for_proc_count(cgroup, 1) != 0 {
            return EXIT_FAILURE;
        }
    }

    EXIT_SUCCESS
}

unsafe extern "C" fn do_controller_fn(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let child = cgroup;
    let parent = arg as *const c_char;

    unsafe {
        if setuid(TEST_UID) != 0 {
            return EXIT_FAILURE;
        }

        if cg_read_strstr(child, c"cgroup.controllers".as_ptr(), c"cpuset".as_ptr()) == 0 {
            return EXIT_FAILURE;
        }

        if cg_write(parent, c"cgroup.subtree_control".as_ptr(), c"+cpuset".as_ptr()) != 0 {
            return EXIT_FAILURE;
        }

        if cg_read_strstr(child, c"cgroup.controllers".as_ptr(), c"cpuset".as_ptr()) != 0 {
            return EXIT_FAILURE;
        }

        if cg_write(parent, c"cgroup.subtree_control".as_ptr(), c"-cpuset".as_ptr()) != 0 {
            return EXIT_FAILURE;
        }

        if cg_read_strstr(child, c"cgroup.controllers".as_ptr(), c"cpuset".as_ptr()) == 0 {
            return EXIT_FAILURE;
        }
    }

    EXIT_SUCCESS
}

/*
 * Migrate a process between two sibling cgroups.
 * The success should only depend on the parent cgroup permissions and not the
 * migrated process itself (cpuset controller is in place because it uses
 * security_task_setscheduler() in cgroup v1).
 *
 * Deliberately don't set cpuset.cpus in children to avoid definining migration
 * permissions between two different cpusets.
 */
unsafe fn test_cpuset_perms_object(root: *const c_char, allow: bool) -> c_int {
    let mut parent: *mut c_char = ptr::null_mut();
    let mut child_src: *mut c_char = ptr::null_mut();
    let mut child_dst: *mut c_char = ptr::null_mut();
    let mut parent_procs: *mut c_char = ptr::null_mut();
    let mut child_src_procs: *mut c_char = ptr::null_mut();
    let mut child_dst_procs: *mut c_char = ptr::null_mut();
    let test_euid: libc::uid_t = TEST_UID;
    let mut object_pid: c_int = 0;
    let mut ret: c_int = KSFT_FAIL;

    unsafe {
        'body: loop {
            parent = cg_name(root, c"cpuset_test_0".as_ptr());
            if parent.is_null() {
                break 'body;
            }
            parent_procs = cg_name(parent, c"cgroup.procs".as_ptr());
            if parent_procs.is_null() {
                break 'body;
            }
            if cg_create(parent) != 0 {
                break 'body;
            }

            child_src = cg_name(parent, c"cpuset_test_1".as_ptr());
            if child_src.is_null() {
                break 'body;
            }
            child_src_procs = cg_name(child_src, c"cgroup.procs".as_ptr());
            if child_src_procs.is_null() {
                break 'body;
            }
            if cg_create(child_src) != 0 {
                break 'body;
            }

            child_dst = cg_name(parent, c"cpuset_test_2".as_ptr());
            if child_dst.is_null() {
                break 'body;
            }
            child_dst_procs = cg_name(child_dst, c"cgroup.procs".as_ptr());
            if child_dst_procs.is_null() {
                break 'body;
            }
            if cg_create(child_dst) != 0 {
                break 'body;
            }

            if cg_write(parent, c"cgroup.subtree_control".as_ptr(), c"+cpuset".as_ptr()) != 0 {
                break 'body;
            }

            if cg_read_strstr(child_src, c"cgroup.controllers".as_ptr(), c"cpuset".as_ptr()) != 0
                || cg_read_strstr(child_dst, c"cgroup.controllers".as_ptr(), c"cpuset".as_ptr()) != 0
            {
                break 'body;
            }

            /* Enable permissions along src->dst tree path */
            if chown(child_src_procs, test_euid, -1i32 as libc::gid_t) != 0
                || chown(child_dst_procs, test_euid, -1i32 as libc::gid_t) != 0
            {
                break 'body;
            }

            if allow && chown(parent_procs, test_euid, -1i32 as libc::gid_t) != 0 {
                break 'body;
            }

            /* Fork a privileged child as a test object */
            object_pid = cg_run_nowait(child_src, idle_process_fn, ptr::null_mut());
            if object_pid < 0 {
                break 'body;
            }

            /* Carry out migration in a child process that can drop all privileges
             * (including capabilities), the main process must remain privileged for
             * cleanup.
             * Child process's cgroup is irrelevant but we place it into child_dst
             * as hacky way to pass information about migration target to the child.
             */
            if allow
                ^ (cg_run(child_dst, do_migration_fn, object_pid as usize as *mut c_void) == EXIT_SUCCESS)
            {
                break 'body;
            }

            ret = KSFT_PASS;
            break 'body;
        }

        if object_pid > 0 {
            let _ = kill(object_pid, libc::SIGTERM);
            let _ = clone_reap(object_pid, WEXITED);
        }

        cg_destroy(child_dst);
        free(child_dst_procs as *mut c_void);
        free(child_dst as *mut c_void);

        cg_destroy(child_src);
        free(child_src_procs as *mut c_void);
        free(child_src as *mut c_void);

        cg_destroy(parent);
        free(parent_procs as *mut c_void);
        free(parent as *mut c_void);
    }

    ret
}

unsafe extern "C" fn test_cpuset_perms_object_allow(root: *const c_char) -> c_int {
    unsafe { test_cpuset_perms_object(root, true) }
}

unsafe extern "C" fn test_cpuset_perms_object_deny(root: *const c_char) -> c_int {
    unsafe { test_cpuset_perms_object(root, false) }
}

/*
 * Migrate a process between parent and child implicitely
 * Implicit migration happens when a controller is enabled/disabled.
 *
 */
unsafe extern "C" fn test_cpuset_perms_subtree(root: *const c_char) -> c_int {
    let mut parent: *mut c_char = ptr::null_mut();
    let mut child: *mut c_char = ptr::null_mut();
    let mut parent_procs: *mut c_char = ptr::null_mut();
    let mut parent_subctl: *mut c_char = ptr::null_mut();
    let mut child_procs: *mut c_char = ptr::null_mut();
    let test_euid: libc::uid_t = TEST_UID;
    let mut object_pid: c_int = 0;
    let mut ret: c_int = KSFT_FAIL;

    unsafe {
        'body: loop {
            parent = cg_name(root, c"cpuset_test_0".as_ptr());
            if parent.is_null() {
                break 'body;
            }
            parent_procs = cg_name(parent, c"cgroup.procs".as_ptr());
            if parent_procs.is_null() {
                break 'body;
            }
            parent_subctl = cg_name(parent, c"cgroup.subtree_control".as_ptr());
            if parent_subctl.is_null() {
                break 'body;
            }
            if cg_create(parent) != 0 {
                break 'body;
            }

            child = cg_name(parent, c"cpuset_test_1".as_ptr());
            if child.is_null() {
                break 'body;
            }
            child_procs = cg_name(child, c"cgroup.procs".as_ptr());
            if child_procs.is_null() {
                break 'body;
            }
            if cg_create(child) != 0 {
                break 'body;
            }

            /* Enable permissions as in a delegated subtree */
            if chown(parent_procs, test_euid, -1i32 as libc::gid_t) != 0
                || chown(parent_subctl, test_euid, -1i32 as libc::gid_t) != 0
                || chown(child_procs, test_euid, -1i32 as libc::gid_t) != 0
            {
                break 'body;
            }

            /* Put a privileged child in the subtree and modify controller state
             * from an unprivileged process, the main process remains privileged
             * for cleanup.
             * The unprivileged child runs in subtree too to avoid parent and
             * internal-node constraing violation.
             */
            object_pid = cg_run_nowait(child, idle_process_fn, ptr::null_mut());
            if object_pid < 0 {
                break 'body;
            }

            if cg_run(child, do_controller_fn, parent as *mut c_void) != EXIT_SUCCESS {
                break 'body;
            }

            ret = KSFT_PASS;
            break 'body;
        }

        if object_pid > 0 {
            let _ = kill(object_pid, libc::SIGTERM);
            let _ = clone_reap(object_pid, WEXITED);
        }

        cg_destroy(child);
        free(child_procs as *mut c_void);
        free(child as *mut c_void);

        cg_destroy(parent);
        free(parent_subctl as *mut c_void);
        free(parent_procs as *mut c_void);
        free(parent as *mut c_void);
    }

    ret
}

unsafe fn get_cpu_affinity(mask: *mut libc::cpu_set_t) -> c_int {
    unsafe {
        libc::CPU_ZERO(mask);
        sched_getaffinity(0, mem::size_of::<libc::cpu_set_t>(), mask)
    }
}

unsafe fn cpu_set_equal(dst: *mut libc::cpu_set_t, mask: c_ulong) -> c_int {
    let mut expected: libc::cpu_set_t = unsafe { mem::zeroed() };

    unsafe {
        libc::CPU_ZERO(&mut expected);
        assert!(mem::size_of_val(&mask) < libc::CPU_SETSIZE as usize);

        for cpu in 0..(mem::size_of_val(&mask) * 8) {
            if ((1 as c_ulong) << cpu) & mask != 0 {
                libc::CPU_SET(cpu, &mut expected);
            }
        }

        libc::CPU_EQUAL(&expected, dst)
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum test_phase {
    AFFINITY_SETUP,
    AFFINITY_CONTROLLER_DISABLED,
    AFFINITY_COMPLETE,
    AFFINITY_ERROR,
}

#[repr(C)]
struct thread_args {
    cgroup: *const c_char,
    affinity_before: *mut libc::cpu_set_t,
    affinity_after: *mut libc::cpu_set_t,
    affinity_before_ready: c_int,
}

static mut test_mutex: libc::pthread_mutex_t = libc::PTHREAD_MUTEX_INITIALIZER;
static mut test_cond: libc::pthread_cond_t = libc::PTHREAD_COND_INITIALIZER;
static mut test_phase: test_phase = test_phase::AFFINITY_SETUP;

unsafe extern "C" fn affinity_thread_fn(arg: *mut c_void) -> *mut c_void {
    let args = arg as *mut thread_args;

    unsafe {
        if cg_enter_current_thread((*args).cgroup) != 0 {
            return affinity_thread_fail();
        }

        if get_cpu_affinity((*args).affinity_before) != 0 {
            return affinity_thread_fail();
        }

        pthread_mutex_lock(&raw mut test_mutex);
        (*args).affinity_before_ready = 1;
        pthread_cond_broadcast(&raw mut test_cond);

        while test_phase < test_phase::AFFINITY_CONTROLLER_DISABLED {
            pthread_cond_wait(&raw mut test_cond, &raw mut test_mutex);
        }
        pthread_mutex_unlock(&raw mut test_mutex);

        if get_cpu_affinity((*args).affinity_after) != 0 {
            return affinity_thread_fail();
        }
    }

    ptr::null_mut()
}

unsafe fn affinity_thread_fail() -> *mut c_void {
    unsafe {
        pthread_mutex_lock(&raw mut test_mutex);
        test_phase = test_phase::AFFINITY_ERROR;
        pthread_cond_broadcast(&raw mut test_cond);
        pthread_mutex_unlock(&raw mut test_mutex);
    }
    ptr::null_mut()
}

/*
 * Test that disabling cpuset controller properly updates thread affinity.
 *
 * This test exposes a bug in cpuset_attach() where threads in child cgroups
 * don't get their affinity updated when the cpuset controller is disabled.
 *
 * Setup:
 * - Create parent cgroup with cpuset.cpus=0-1
 * - Create child A with cpuset.cpus=0-1
 * - Create child B with cpuset.cpus=1
 * - Place multithreaded process: group leader + thread_a in A, thread_b in B
 * - Disable cpuset controller on parent
 *
 * Expected: thread_b's affinity should expand from {1} to {0-1}
 * Buggy: thread_b's affinity remains {1}
 */
unsafe extern "C" fn test_cpuset_affinity_on_controller_disable(root: *const c_char) -> c_int {
    let mut parent: *mut c_char = ptr::null_mut();
    let mut child_a: *mut c_char = ptr::null_mut();
    let mut child_b: *mut c_char = ptr::null_mut();
    let mut thread_a: libc::pthread_t = unsafe { mem::zeroed() };
    let mut thread_b: libc::pthread_t = unsafe { mem::zeroed() };
    let mut thread_a_created: c_int = 0;
    let mut thread_b_created: c_int = 0;
    let mut affinity_a_before: libc::cpu_set_t = unsafe { mem::zeroed() };
    let mut affinity_a_after: libc::cpu_set_t = unsafe { mem::zeroed() };
    let mut affinity_b_before: libc::cpu_set_t = unsafe { mem::zeroed() };
    let mut affinity_b_after: libc::cpu_set_t = unsafe { mem::zeroed() };
    let mut ret: c_int = KSFT_FAIL;
    let mut do_cleanup_threads = false;

    unsafe {
        'body: loop {
            parent = cg_name(root, c"cpuset_affinity_test".as_ptr());
            if parent.is_null() {
                break 'body;
            }
            if cg_create(parent) != 0 {
                break 'body;
            }
            if cg_write(parent, c"cgroup.type".as_ptr(), c"threaded".as_ptr()) != 0 {
                break 'body;
            }

            child_a = cg_name(parent, c"A".as_ptr());
            if child_a.is_null() {
                break 'body;
            }
            if cg_create(child_a) != 0 {
                break 'body;
            }
            if cg_write(child_a, c"cgroup.type".as_ptr(), c"threaded".as_ptr()) != 0 {
                break 'body;
            }

            child_b = cg_name(parent, c"B".as_ptr());
            if child_b.is_null() {
                break 'body;
            }
            if cg_create(child_b) != 0 {
                break 'body;
            }
            if cg_write(child_b, c"cgroup.type".as_ptr(), c"threaded".as_ptr()) != 0 {
                break 'body;
            }

            /* Now enable cpuset controller in parent */
            if cg_write(parent, c"cgroup.subtree_control".as_ptr(), c"+cpuset".as_ptr()) != 0 {
                ret = KSFT_SKIP;
                break 'body;
            }

            /*
             * Set CPU affinity constraints
             * Skip the test if the setting of "cpuset.cpus" fails as the test
             * system may not have CPU 1.
             */
            if cg_write(parent, c"cpuset.cpus".as_ptr(), c"0-1".as_ptr()) != 0 {
                ret = KSFT_SKIP;
                break 'body;
            }
            if cg_write(child_a, c"cpuset.cpus".as_ptr(), c"0-1".as_ptr()) != 0 {
                ret = KSFT_SKIP;
                break 'body;
            }
            if cg_write(child_b, c"cpuset.cpus".as_ptr(), c"1".as_ptr()) != 0 {
                ret = KSFT_SKIP;
                break 'body;
            }

            /* Move group leader (main thread) to child A */
            if cg_enter_current(child_a) != 0 {
                break 'body;
            }

            /* Create threads - they will move themselves to their respective cgroups */
            test_phase = test_phase::AFFINITY_SETUP;

            let mut args_a = thread_args {
                cgroup: child_a,
                affinity_before: &mut affinity_a_before,
                affinity_after: &mut affinity_a_after,
                affinity_before_ready: 0,
            };
            if pthread_create(
                &mut thread_a,
                ptr::null(),
                affinity_thread_fn,
                &mut args_a as *mut thread_args as *mut c_void,
            ) != 0
            {
                break 'body;
            }
            thread_a_created = 1;

            let mut args_b = thread_args {
                cgroup: child_b,
                affinity_before: &mut affinity_b_before,
                affinity_after: &mut affinity_b_after,
                affinity_before_ready: 0,
            };
            if pthread_create(
                &mut thread_b,
                ptr::null(),
                affinity_thread_fn,
                &mut args_b as *mut thread_args as *mut c_void,
            ) != 0
            {
                do_cleanup_threads = true;
                break 'body;
            }
            thread_b_created = 1;

            pthread_mutex_lock(&raw mut test_mutex);
            while (test_phase < test_phase::AFFINITY_ERROR)
                && (args_a.affinity_before_ready + args_b.affinity_before_ready < 2)
            {
                pthread_cond_wait(&raw mut test_cond, &raw mut test_mutex);
            }

            /* If a thread failed during setup, bail out */
            if test_phase == test_phase::AFFINITY_ERROR {
                pthread_mutex_unlock(&raw mut test_mutex);
                do_cleanup_threads = true;
                break 'body;
            }
            pthread_mutex_unlock(&raw mut test_mutex);

            if cpu_set_equal(&mut affinity_a_before, 0x3) == 0 {
                ksft_print_msg(c"FAIL: thread_a initial affinity incorrect\n".as_ptr());
                do_cleanup_threads = true;
                break 'body;
            }

            if cpu_set_equal(&mut affinity_b_before, 0x2) == 0 {
                ksft_print_msg(c"FAIL: thread_b initial affinity incorrect\n".as_ptr());
                do_cleanup_threads = true;
                break 'body;
            }

            /* Disable cpuset controller - this should trigger affinity update */
            if cg_write(parent, c"cgroup.subtree_control".as_ptr(), c"-cpuset".as_ptr()) != 0 {
                do_cleanup_threads = true;
                break 'body;
            }

            /* Signal threads to save their final affinity and exit */
            pthread_mutex_lock(&raw mut test_mutex);
            test_phase = test_phase::AFFINITY_CONTROLLER_DISABLED;
            pthread_cond_broadcast(&raw mut test_cond);
            pthread_mutex_unlock(&raw mut test_mutex);

            pthread_join(thread_a, ptr::null_mut());
            pthread_join(thread_b, ptr::null_mut());

            /* Verify thread affinities AFTER disabling controller */
            if cpu_set_equal(&mut affinity_a_after, 0x3) == 0 {
                ksft_print_msg(c"FAIL: thread_a final affinity incorrect\n".as_ptr());
                break 'body;
            }

            if cpu_set_equal(&mut affinity_b_after, 0x3) == 0 {
                ksft_print_msg(c"FAIL: thread_b affinity did not expand to {0-1}\n".as_ptr());
                break 'body;
            }

            ret = KSFT_PASS;
            break 'body;
        }

        if do_cleanup_threads {
            pthread_mutex_lock(&raw mut test_mutex);
            test_phase = test_phase::AFFINITY_COMPLETE;
            pthread_cond_broadcast(&raw mut test_cond);
            pthread_mutex_unlock(&raw mut test_mutex);

            if thread_a_created != 0 {
                pthread_join(thread_a, ptr::null_mut());
            }
            if thread_b_created != 0 {
                pthread_join(thread_b, ptr::null_mut());
            }
        }

        /* Move back to root before cleanup */
        cg_enter_current(root);

        cg_destroy(child_b);
        free(child_b as *mut c_void);
        cg_destroy(child_a);
        free(child_a as *mut c_void);
        cg_destroy(parent);
        free(parent as *mut c_void);
    }

    ret
}

struct cpuset_test {
    fn_: unsafe extern "C" fn(*const c_char) -> c_int,
    name: *const c_char,
}

const TESTS: [cpuset_test; 4] = [
    cpuset_test {
        fn_: test_cpuset_perms_object_allow,
        name: c"test_cpuset_perms_object_allow".as_ptr(),
    },
    cpuset_test {
        fn_: test_cpuset_perms_object_deny,
        name: c"test_cpuset_perms_object_deny".as_ptr(),
    },
    cpuset_test {
        fn_: test_cpuset_perms_subtree,
        name: c"test_cpuset_perms_subtree".as_ptr(),
    },
    cpuset_test {
        fn_: test_cpuset_affinity_on_controller_disable,
        name: c"test_cpuset_affinity_on_controller_disable".as_ptr(),
    },
];

fn main() {
    let mut root = [0 as c_char; PATH_MAX];
    let mut i: usize;

    unsafe {
        ksft_print_header();
        ksft_set_plan(TESTS.len() as c_int);
        if cg_find_unified_root(root.as_mut_ptr(), mem::size_of_val(&root), ptr::null_mut()) != 0 {
            ksft_exit_skip(c"cgroup v2 isn't mounted\n".as_ptr());
        }

        if cg_read_strstr(
            root.as_ptr(),
            c"cgroup.subtree_control".as_ptr(),
            c"cpuset".as_ptr(),
        ) != 0
        {
            if cg_write(
                root.as_ptr(),
                c"cgroup.subtree_control".as_ptr(),
                c"+cpuset".as_ptr(),
            ) != 0
            {
                ksft_exit_skip(c"Failed to set cpuset controller\n".as_ptr());
            }
        }

        i = 0;
        while i < TESTS.len() {
            match (TESTS[i].fn_)(root.as_ptr()) {
                KSFT_PASS => {
                    ksft_test_result_pass(c"%s\n".as_ptr(), TESTS[i].name);
                }
                KSFT_SKIP => {
                    ksft_test_result_skip(c"%s\n".as_ptr(), TESTS[i].name);
                }
                _ => {
                    ksft_test_result_fail(c"%s\n".as_ptr(), TESTS[i].name);
                }
            }
            i += 1;
        }

        ksft_finished();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
