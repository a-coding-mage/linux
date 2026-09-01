// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2020 Google LLC.
 */

// C dependencies translated as external declarations:
// asm-generic/errno-base.h, sys/stat.h, test_progs.h, linux/limits.h,
// local_storage.skel.h, network_helpers.h, task_local_storage_helpers.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const TEST_STORAGE_VALUE: c_uint = 0xbeefdead;

#[repr(C)]
struct storage {
    inode: *mut c_void,
    value: c_uint,
}

#[repr(C)]
struct local_storage {
    bss: *mut local_storage_bss,
    data: *mut local_storage_data,
    maps: local_storage_maps,
}

#[repr(C)]
struct local_storage_bss {
    monitored_pid: c_int,
}

#[repr(C)]
struct local_storage_data {
    task_storage_result: c_int,
    inode_storage_result: c_int,
    sk_storage_result: c_int,
}

#[repr(C)]
struct local_storage_maps {
    task_storage_map: *mut bpf_map,
    inode_storage_map: *mut bpf_map,
    sk_storage_map: *mut bpf_map,
}

#[repr(C)]
struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    static BPF_NOEXIST: c_uint;
    static EPERM: c_int;
    static EINVAL: c_int;
    static ENOENT: c_int;
    static O_WRONLY: c_int;
    static O_RDONLY: c_int;
    static STDOUT_FILENO: c_int;
    static STDERR_FILENO: c_int;
    static AF_INET6: c_int;
    static SOCK_STREAM: c_int;

    fn fork() -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getpid() -> c_int;
    fn execlp(file: *const c_char, arg: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: c_int, wstatus: *mut c_int, options: c_int) -> c_int;
    fn WEXITSTATUS(status: c_int) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn system(command: *const c_char) -> c_int;

    fn sys_pidfd_open(pid: c_int, flags: c_uint) -> c_int;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_lookup_elem_flags(
        fd: c_int,
        key: *const c_void,
        value: *mut c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: u64,
    ) -> c_int;
    fn bpf_map_delete_elem(fd: c_int, key: *const c_void) -> c_int;

    fn local_storage__open_and_load() -> *mut local_storage;
    fn local_storage__attach(obj: *mut local_storage) -> c_int;
    fn local_storage__destroy(obj: *mut local_storage);
    fn start_server(
        family: c_int,
        type_: c_int,
        addr: *const c_void,
        port: c_int,
        timeout_ms: c_int,
    ) -> c_int;

    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
}

/* Fork and exec the provided rm binary and return the exit code of the
 * forked process and its pid.
 */
unsafe fn run_self_unlink(skel: *mut local_storage, rm_path: *const c_char) -> c_int {
    let child_pid: c_int;
    let mut child_status: c_int = 0;
    let ret: c_int;
    let null_fd: c_int;

    child_pid = fork();
    if child_pid == 0 {
        null_fd = open(c"/dev/null".as_ptr(), O_WRONLY);
        dup2(null_fd, STDOUT_FILENO);
        dup2(null_fd, STDERR_FILENO);
        close(null_fd);

        (*(*skel).bss).monitored_pid = getpid();
        /* Use the copied /usr/bin/rm to delete itself
         * /tmp/copy_of_rm /tmp/copy_of_rm.
         */
        ret = execlp(rm_path, rm_path, rm_path, ptr::null::<c_void>());
        if ret != 0 {
            exit(errno);
        }
    } else if child_pid > 0 {
        waitpid(child_pid, &mut child_status, 0);
        ASSERT_EQ(
            (*(*skel).data).task_storage_result,
            0,
            c"task_storage_result".as_ptr(),
        );
        return WEXITSTATUS(child_status);
    }

    -EINVAL
}

unsafe fn check_syscall_operations(map_fd: c_int, obj_fd: c_int) -> bool {
    let mut val = storage {
        inode: ptr::null_mut(),
        value: TEST_STORAGE_VALUE,
    };
    let mut lookup_val = storage {
        inode: ptr::null_mut(),
        value: 0,
    };
    let mut err: c_int;

    /* Looking up an existing element should fail initially */
    err = bpf_map_lookup_elem_flags(
        map_fd,
        &obj_fd as *const c_int as *const c_void,
        &mut lookup_val as *mut storage as *mut c_void,
        0,
    );
    if !ASSERT_EQ(err, -ENOENT, c"bpf_map_lookup_elem".as_ptr()) {
        return false;
    }

    /* Create a new element */
    err = bpf_map_update_elem(
        map_fd,
        &obj_fd as *const c_int as *const c_void,
        &mut val as *mut storage as *const c_void,
        BPF_NOEXIST as u64,
    );
    if !ASSERT_OK(err, c"bpf_map_update_elem".as_ptr()) {
        return false;
    }

    /* Lookup the newly created element */
    err = bpf_map_lookup_elem_flags(
        map_fd,
        &obj_fd as *const c_int as *const c_void,
        &mut lookup_val as *mut storage as *mut c_void,
        0,
    );
    if !ASSERT_OK(err, c"bpf_map_lookup_elem".as_ptr()) {
        return false;
    }

    /* Check the value of the newly created element */
    if !ASSERT_EQ(
        lookup_val.value as c_int,
        val.value as c_int,
        c"bpf_map_lookup_elem".as_ptr(),
    ) {
        return false;
    }

    err = bpf_map_delete_elem(map_fd, &obj_fd as *const c_int as *const c_void);
    if !ASSERT_OK(err, c"bpf_map_delete_elem()".as_ptr()) {
        return false;
    }

    /* The lookup should fail, now that the element has been deleted */
    err = bpf_map_lookup_elem_flags(
        map_fd,
        &obj_fd as *const c_int as *const c_void,
        &mut lookup_val as *mut storage as *mut c_void,
        0,
    );
    if !ASSERT_EQ(err, -ENOENT, c"bpf_map_lookup_elem".as_ptr()) {
        return false;
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn test_test_local_storage() {
    let mut tmp_dir_path = *b"/tmp/local_storageXXXXXX\0";
    let mut err: c_int;
    let mut serv_sk: c_int = -1;
    let mut task_fd: c_int = -1;
    let mut rm_fd: c_int = -1;
    let mut skel: *mut local_storage = ptr::null_mut();
    let mut tmp_exec_path = [0 as c_char; 64];
    let mut cmd = [0 as c_char; 256];

    skel = local_storage__open_and_load();
    if !ASSERT_OK_PTR(skel as *mut c_void, c"skel_load".as_ptr()) {
        goto_close_prog(serv_sk, rm_fd, task_fd, skel);
        return;
    }

    err = local_storage__attach(skel);
    if !ASSERT_OK(err, c"attach".as_ptr()) {
        goto_close_prog(serv_sk, rm_fd, task_fd, skel);
        return;
    }

    task_fd = sys_pidfd_open(getpid(), 0);
    if !ASSERT_GE(task_fd, 0, c"pidfd_open".as_ptr()) {
        goto_close_prog(serv_sk, rm_fd, task_fd, skel);
        return;
    }

    if !check_syscall_operations(bpf_map__fd((*skel).maps.task_storage_map), task_fd) {
        goto_close_prog(serv_sk, rm_fd, task_fd, skel);
        return;
    }

    if !ASSERT_OK_PTR(
        mkdtemp(tmp_dir_path.as_mut_ptr() as *mut c_char) as *mut c_void,
        c"mkdtemp".as_ptr(),
    ) {
        goto_close_prog(serv_sk, rm_fd, task_fd, skel);
        return;
    }

    snprintf(
        tmp_exec_path.as_mut_ptr(),
        tmp_exec_path.len(),
        c"%s/copy_of_rm".as_ptr(),
        tmp_dir_path.as_ptr(),
    );
    snprintf(
        cmd.as_mut_ptr(),
        cmd.len(),
        c"cp /bin/rm %s".as_ptr(),
        tmp_exec_path.as_ptr(),
    );
    if !ASSERT_OK(system(cmd.as_ptr()), c"system(cp)".as_ptr()) {
        goto_close_prog_rmdir(cmd.as_mut_ptr(), cmd.len(), tmp_dir_path.as_ptr(), serv_sk, rm_fd, task_fd, skel);
        return;
    }

    rm_fd = open(tmp_exec_path.as_ptr(), O_RDONLY);
    if !ASSERT_GE(rm_fd, 0, c"open(tmp_exec_path)".as_ptr()) {
        goto_close_prog_rmdir(cmd.as_mut_ptr(), cmd.len(), tmp_dir_path.as_ptr(), serv_sk, rm_fd, task_fd, skel);
        return;
    }

    if !check_syscall_operations(bpf_map__fd((*skel).maps.inode_storage_map), rm_fd) {
        goto_close_prog_rmdir(cmd.as_mut_ptr(), cmd.len(), tmp_dir_path.as_ptr(), serv_sk, rm_fd, task_fd, skel);
        return;
    }

    /* Sets skel->bss->monitored_pid to the pid of the forked child
     * forks a child process that executes tmp_exec_path and tries to
     * unlink its executable. This operation should be denied by the loaded
     * LSM program.
     */
    err = run_self_unlink(skel, tmp_exec_path.as_ptr());
    if !ASSERT_EQ(err, EPERM, c"run_self_unlink".as_ptr()) {
        goto_close_prog_rmdir(cmd.as_mut_ptr(), cmd.len(), tmp_dir_path.as_ptr(), serv_sk, rm_fd, task_fd, skel);
        return;
    }

    /* Set the process being monitored to be the current process */
    (*(*skel).bss).monitored_pid = getpid();

    /* Move copy_of_rm to a new location so that it triggers the
     * inode_rename LSM hook with a new_dentry that has a NULL inode ptr.
     */
    snprintf(
        cmd.as_mut_ptr(),
        cmd.len(),
        c"mv %s/copy_of_rm %s/check_null_ptr".as_ptr(),
        tmp_dir_path.as_ptr(),
        tmp_dir_path.as_ptr(),
    );
    if !ASSERT_OK(system(cmd.as_ptr()), c"system(mv)".as_ptr()) {
        goto_close_prog_rmdir(cmd.as_mut_ptr(), cmd.len(), tmp_dir_path.as_ptr(), serv_sk, rm_fd, task_fd, skel);
        return;
    }

    ASSERT_EQ(
        (*(*skel).data).inode_storage_result,
        0,
        c"inode_storage_result".as_ptr(),
    );

    serv_sk = start_server(AF_INET6, SOCK_STREAM, ptr::null(), 0, 0);
    if !ASSERT_GE(serv_sk, 0, c"start_server".as_ptr()) {
        goto_close_prog_rmdir(cmd.as_mut_ptr(), cmd.len(), tmp_dir_path.as_ptr(), serv_sk, rm_fd, task_fd, skel);
        return;
    }

    ASSERT_EQ(
        (*(*skel).data).sk_storage_result,
        0,
        c"sk_storage_result".as_ptr(),
    );

    if !check_syscall_operations(bpf_map__fd((*skel).maps.sk_storage_map), serv_sk) {
        goto_close_prog_rmdir(cmd.as_mut_ptr(), cmd.len(), tmp_dir_path.as_ptr(), serv_sk, rm_fd, task_fd, skel);
        return;
    }

    goto_close_prog_rmdir(cmd.as_mut_ptr(), cmd.len(), tmp_dir_path.as_ptr(), serv_sk, rm_fd, task_fd, skel);
}

unsafe fn goto_close_prog_rmdir(
    cmd: *mut c_char,
    cmd_len: usize,
    tmp_dir_path: *const u8,
    serv_sk: c_int,
    rm_fd: c_int,
    task_fd: c_int,
    skel: *mut local_storage,
) {
    snprintf(
        cmd,
        cmd_len,
        c"rm -rf %s".as_ptr(),
        tmp_dir_path as *const c_char,
    );
    system(cmd);
    goto_close_prog(serv_sk, rm_fd, task_fd, skel);
}

unsafe fn goto_close_prog(
    serv_sk: c_int,
    rm_fd: c_int,
    task_fd: c_int,
    skel: *mut local_storage,
) {
    close(serv_sk);
    close(rm_fd);
    close(task_fd);
    local_storage__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
