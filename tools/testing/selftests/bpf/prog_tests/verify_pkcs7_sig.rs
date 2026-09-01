// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2022 Huawei Technologies Duesseldorf GmbH
 *
 * Author: Roberto Sassu <roberto.sassu@huawei.com>
 */

/* Translated from:
 * ./testing/selftests/bpf/prog_tests/verify_pkcs7_sig.c
 *
 * C includes removed. External symbols from libbpf, test_progs.h,
 * linux headers, libc, and generated BPF skeleton headers are declared below.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const MAX_DATA_SIZE: usize = 1024 * 1024;
const MAX_SIG_SIZE: usize = 1024;

const VERIFY_USE_SECONDARY_KEYRING: c_ulong = 1;
const VERIFY_USE_PLATFORM_KEYRING: c_ulong = 2;

#[allow(dead_code)]
const SHA256_DIGEST_SIZE: usize = 32;

#[repr(C)]
pub struct data {
    pub data: [u8; MAX_DATA_SIZE],
    pub data_len: u32,
    pub sig: [u8; MAX_SIG_SIZE],
    pub sig_len: u32,
}

static mut kfunc_not_supported: bool = false;

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;
type off_t = isize;
type va_list = *mut c_void;
type libbpf_print_fn_t =
    Option<unsafe extern "C" fn(libbpf_print_level, *const c_char, va_list) -> c_int>;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libbpf_print_level {
    LIBBPF_WARN = 1,
}

#[repr(C)]
pub struct stat {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_verify_pkcs7_sig_bss {
    pub monitored_pid: pid_t,
    pub user_keyring_serial: c_long,
    pub system_keyring_id: c_ulong,
}

#[repr(C)]
pub struct test_verify_pkcs7_sig {
    pub obj: *mut bpf_object,
    pub bss: *mut test_verify_pkcs7_sig_bss,
}

#[repr(C)]
pub struct test_sig_in_xattr_bss {
    pub monitored_pid: pid_t,
    pub sig_size: c_int,
    pub user_keyring_serial: c_long,
    pub digest: [u8; SHA256_DIGEST_SIZE],
}

#[repr(C)]
pub struct test_sig_in_xattr {
    pub bss: *mut test_sig_in_xattr_bss,
}

#[repr(C)]
pub struct module_signature {
    pub algo: u8,
    pub hash: u8,
    pub id_type: u8,
    pub signer_len: u8,
    pub key_id_len: u8,
    pub __pad: [u8; 3],
    pub sig_len: u32,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn vprintf(fmt: *const c_char, args: va_list) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn fork() -> pid_t;
    fn execlp(file: *const c_char, arg: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn popen(command: *const c_char, type_: *const c_char) -> *mut FILE;
    fn pclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn sleep(seconds: c_uint) -> c_uint;
    fn setxattr(
        path: *const c_char,
        name: *const c_char,
        value: *const c_void,
        size: size_t,
        flags: c_int,
    ) -> c_int;

    fn libbpf_set_print(cb: libbpf_print_fn_t) -> libbpf_print_fn_t;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_map_update_elem(fd: c_int, key: *const c_void, value: *const c_void, flags: c_ulong)
        -> c_int;

    fn test_verify_pkcs7_sig__open() -> *mut test_verify_pkcs7_sig;
    fn test_verify_pkcs7_sig__load(skel: *mut test_verify_pkcs7_sig) -> c_int;
    fn test_verify_pkcs7_sig__attach(skel: *mut test_verify_pkcs7_sig) -> c_int;
    fn test_verify_pkcs7_sig__destroy(skel: *mut test_verify_pkcs7_sig);

    fn test_sig_in_xattr__open() -> *mut test_sig_in_xattr;
    fn test_sig_in_xattr__load(skel: *mut test_sig_in_xattr) -> c_int;
    fn test_sig_in_xattr__attach(skel: *mut test_sig_in_xattr) -> c_int;
    fn test_sig_in_xattr__destroy(skel: *mut test_sig_in_xattr);

    fn test__skip();
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool;
    fn ASSERT_LT(ret: c_int, val: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(ret: c_int, val: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(ret: c_int, val: c_int, name: *const c_char) -> bool;

    static MODULE_SIGNATURE_MARKER: [c_char; 0];
}

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const E2BIG: c_int = 7;
const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 0x1;
const MAP_PRIVATE: c_int = 0x02;
const BPF_ANY: c_ulong = 0;
const KEY_SPEC_SESSION_KEYRING: c_long = -3;
const KEYCTL_SETPERM: c_int = 5;
const KEYCTL_SET_TIMEOUT: c_int = 15;
const __NR_request_key: c_long = 249;
const __NR_keyctl: c_long = 250;
const PATH_MAX: usize = 4096;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn stat_st_size(_st: *const stat) -> isize {
    /* struct stat layout is supplied by system headers in C. */
    todo!("external struct stat st_size field")
}

unsafe fn __be32_to_cpu(v: u32) -> u32 {
    u32::from_be(v)
}

unsafe extern "C" fn libbpf_print_cb(
    level: libbpf_print_level,
    fmt: *const c_char,
    args: va_list,
) -> c_int {
    if level == libbpf_print_level::LIBBPF_WARN {
        vprintf(fmt, args);
    }

    if strcmp(
        fmt,
        c"libbpf: extern (func ksym) '%s': not found in kernel or module BTFs\n".as_ptr(),
    ) != 0
    {
        return 0;
    }

    /* C consumes the next va_arg(args, char *) here. */
    let _ = args;
    todo!("va_arg(args, char *) comparison with bpf_verify_pkcs7_signature");

    #[allow(unreachable_code)]
    {
        kfunc_not_supported = true;
        0
    }
}

unsafe fn _run_setup_process(setup_dir: *const c_char, cmd: *const c_char) -> c_int {
    let mut child_pid: c_int;
    let mut child_status: c_int = 0;

    child_pid = fork();
    if child_pid == 0 {
        execlp(
            c"./verify_sig_setup.sh".as_ptr(),
            c"./verify_sig_setup.sh".as_ptr(),
            cmd,
            setup_dir,
            ptr::null::<c_void>(),
        );
        exit(errno);
    } else if child_pid > 0 {
        waitpid(child_pid, &mut child_status, 0);
        return WEXITSTATUS(child_status);
    }

    -EINVAL
}

unsafe fn populate_data_item_str(tmp_dir: *const c_char, data_item: *mut data) -> c_int {
    let mut st: stat = core::mem::zeroed();
    let mut data_template = *b"/tmp/dataXXXXXX\0";
    let mut path = [0 as c_char; PATH_MAX];
    let mut ret: c_int;
    let mut fd: c_int;
    let mut child_status: c_int = 0;
    let child_pid: c_int;

    (*data_item).data_len = 4;
    memcpy(
        (*data_item).data.as_mut_ptr() as *mut c_void,
        c"test".as_ptr() as *const c_void,
        (*data_item).data_len as size_t,
    );

    fd = mkstemp(data_template.as_mut_ptr() as *mut c_char);
    if fd == -1 {
        return -errno;
    }

    ret = write(
        fd,
        (*data_item).data.as_ptr() as *const c_void,
        (*data_item).data_len as size_t,
    ) as c_int;

    close(fd);

    if ret != (*data_item).data_len as c_int {
        ret = -EIO;
        unlink(data_template.as_ptr() as *const c_char);
        return ret;
    }

    child_pid = fork();

    if child_pid == -1 {
        ret = -errno;
        unlink(data_template.as_ptr() as *const c_char);
        return ret;
    }

    if child_pid == 0 {
        snprintf(
            path.as_mut_ptr(),
            path.len(),
            c"%s/signing_key.pem".as_ptr(),
            tmp_dir,
        );

        return execlp(
            c"./sign-file".as_ptr(),
            c"./sign-file".as_ptr(),
            c"-d".as_ptr(),
            c"sha256".as_ptr(),
            path.as_ptr(),
            path.as_ptr(),
            data_template.as_ptr() as *const c_char,
            ptr::null::<c_void>(),
        );
    }

    waitpid(child_pid, &mut child_status, 0);

    ret = WEXITSTATUS(child_status);
    if ret != 0 {
        unlink(data_template.as_ptr() as *const c_char);
        return ret;
    }

    snprintf(
        path.as_mut_ptr(),
        path.len(),
        c"%s.p7s".as_ptr(),
        data_template.as_ptr() as *const c_char,
    );

    ret = stat(path.as_ptr(), &mut st);
    if ret == -1 {
        ret = -errno;
        unlink(data_template.as_ptr() as *const c_char);
        return ret;
    }

    if stat_st_size(&st) as usize > size_of_val(&(*data_item).sig) {
        ret = -EINVAL;
        unlink(path.as_ptr());
        unlink(data_template.as_ptr() as *const c_char);
        return ret;
    }

    (*data_item).sig_len = stat_st_size(&st) as u32;

    fd = open(path.as_ptr(), O_RDONLY);
    if fd == -1 {
        ret = -errno;
        unlink(path.as_ptr());
        unlink(data_template.as_ptr() as *const c_char);
        return ret;
    }

    ret = read(
        fd,
        (*data_item).sig.as_mut_ptr() as *mut c_void,
        (*data_item).sig_len as size_t,
    ) as c_int;

    close(fd);

    if ret != (*data_item).sig_len as c_int {
        ret = -EIO;
        unlink(path.as_ptr());
        unlink(data_template.as_ptr() as *const c_char);
        return ret;
    }

    ret = 0;
    unlink(path.as_ptr());
    unlink(data_template.as_ptr() as *const c_char);
    ret
}

unsafe fn size_of_val<T>(val: &T) -> usize {
    core::mem::size_of_val(val)
}

unsafe fn populate_data_item_mod(data_item: *mut data) -> c_int {
    let mut mod_path = [0 as c_char; PATH_MAX];
    let mut mod_path_ptr: *mut c_char;
    let mut st: stat = core::mem::zeroed();
    let mod_: *mut c_void;
    let fp: *mut FILE;
    let mut ms: module_signature = core::mem::zeroed();
    let mut ret: c_int;
    let fd: c_int;
    let mut modlen: c_int;
    let marker_len: c_int;
    let sig_len: c_int;

    (*data_item).data_len = 0;

    if stat(c"/lib/modules".as_ptr(), &mut st) == -1 {
        return 0;
    }

    /* Requires CONFIG_TCP_CONG_BIC=m. */
    fp = popen(c"find /lib/modules/$(uname -r) -name tcp_bic.ko".as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        return 0;
    }

    mod_path_ptr = fgets(mod_path.as_mut_ptr(), mod_path.len() as c_int, fp);
    pclose(fp);

    if mod_path_ptr.is_null() {
        return 0;
    }

    mod_path_ptr = strchr(mod_path.as_ptr(), '\n' as c_int);
    if mod_path_ptr.is_null() {
        return 0;
    }

    *mod_path_ptr = '\0' as c_char;

    if stat(mod_path.as_ptr(), &mut st) == -1 {
        return 0;
    }

    modlen = stat_st_size(&st) as c_int;
    marker_len = size_of::<[c_char; 0]>() as c_int - 1;

    fd = open(mod_path.as_ptr(), O_RDONLY);
    if fd == -1 {
        return -errno;
    }

    mod_ = mmap(
        ptr::null_mut(),
        stat_st_size(&st) as size_t,
        PROT_READ,
        MAP_PRIVATE,
        fd,
        0,
    );

    close(fd);

    if mod_ == MAP_FAILED {
        return -errno;
    }

    if strncmp(
        (mod_ as *const c_char).offset((modlen - marker_len) as isize),
        MODULE_SIGNATURE_MARKER.as_ptr(),
        marker_len as size_t,
    ) != 0
    {
        ret = -EINVAL;
        munmap(mod_, stat_st_size(&st) as size_t);
        return ret;
    }

    modlen -= marker_len;

    memcpy(
        &mut ms as *mut module_signature as *mut c_void,
        (mod_ as *const u8).offset((modlen as usize - size_of::<module_signature>()) as isize)
            as *const c_void,
        size_of::<module_signature>(),
    );

    sig_len = __be32_to_cpu(ms.sig_len) as c_int;
    modlen -= sig_len + size_of::<module_signature>() as c_int;

    if modlen as usize > size_of_val(&(*data_item).data) {
        ret = -E2BIG;
        munmap(mod_, stat_st_size(&st) as size_t);
        return ret;
    }

    memcpy(
        (*data_item).data.as_mut_ptr() as *mut c_void,
        mod_ as *const c_void,
        modlen as size_t,
    );
    (*data_item).data_len = modlen as u32;

    if sig_len as usize > size_of_val(&(*data_item).sig) {
        ret = -E2BIG;
        munmap(mod_, stat_st_size(&st) as size_t);
        return ret;
    }

    memcpy(
        (*data_item).sig.as_mut_ptr() as *mut c_void,
        (mod_ as *const u8).offset(modlen as isize) as *const c_void,
        sig_len as size_t,
    );
    (*data_item).sig_len = sig_len as u32;
    ret = 0;
    munmap(mod_, stat_st_size(&st) as size_t);
    ret
}

unsafe fn test_verify_pkcs7_sig_from_map() {
    let mut old_print_cb: libbpf_print_fn_t;
    let mut tmp_dir_template = *b"/tmp/verify_sigXXXXXX\0";
    let tmp_dir: *mut c_char;
    let mut skel: *mut test_verify_pkcs7_sig = ptr::null_mut();
    let mut map: *mut bpf_map;
    let mut data: data = core::mem::zeroed();
    let mut ret: c_int;
    let mut zero: c_int = 0;

    /* Trigger creation of session keyring. */
    syscall(
        __NR_request_key,
        c"keyring".as_ptr(),
        c"_uid.0".as_ptr(),
        ptr::null::<c_void>(),
        KEY_SPEC_SESSION_KEYRING,
    );

    tmp_dir = mkdtemp(tmp_dir_template.as_mut_ptr() as *mut c_char);
    if !ASSERT_OK_PTR(tmp_dir, c"mkdtemp".as_ptr()) {
        return;
    }

    ret = _run_setup_process(tmp_dir, c"setup".as_ptr());
    if !ASSERT_OK(ret, c"_run_setup_process".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        return;
    }

    skel = test_verify_pkcs7_sig__open();
    if !ASSERT_OK_PTR(skel, c"test_verify_pkcs7_sig__open".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        return;
    }

    old_print_cb = libbpf_set_print(Some(libbpf_print_cb));
    ret = test_verify_pkcs7_sig__load(skel);
    libbpf_set_print(old_print_cb);

    if ret < 0 && kfunc_not_supported {
        printf_skip_kfunc();
        test__skip();
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        if skel.is_null() {
            return;
        }
        (*(*skel).bss).monitored_pid = 0;
        test_verify_pkcs7_sig__destroy(skel);
        return;
    }

    if !ASSERT_OK(ret, c"test_verify_pkcs7_sig__load".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        if skel.is_null() {
            return;
        }
        (*(*skel).bss).monitored_pid = 0;
        test_verify_pkcs7_sig__destroy(skel);
        return;
    }

    ret = test_verify_pkcs7_sig__attach(skel);
    if !ASSERT_OK(ret, c"test_verify_pkcs7_sig__attach".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        if !skel.is_null() {
            (*(*skel).bss).monitored_pid = 0;
            test_verify_pkcs7_sig__destroy(skel);
        }
        return;
    }

    map = bpf_object__find_map_by_name((*skel).obj, c"data_input".as_ptr());
    if !ASSERT_OK_PTR(map, c"data_input not found".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        if !skel.is_null() {
            (*(*skel).bss).monitored_pid = 0;
            test_verify_pkcs7_sig__destroy(skel);
        }
        return;
    }

    (*(*skel).bss).monitored_pid = getpid();

    /* Test without data and signature. */
    (*(*skel).bss).user_keyring_serial = KEY_SPEC_SESSION_KEYRING;

    ret = bpf_map_update_elem(
        bpf_map__fd(map),
        &mut zero as *mut c_int as *const c_void,
        &mut data as *mut data as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_LT(ret, 0, c"bpf_map_update_elem data_input".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_verify_pkcs7_sig__destroy(skel);
        return;
    }

    /* Test successful signature verification with session keyring. */
    ret = populate_data_item_str(tmp_dir, &mut data);
    if !ASSERT_OK(ret, c"populate_data_item_str".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_verify_pkcs7_sig__destroy(skel);
        return;
    }

    ret = bpf_map_update_elem(
        bpf_map__fd(map),
        &mut zero as *mut c_int as *const c_void,
        &mut data as *mut data as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_OK(ret, c"bpf_map_update_elem data_input".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_verify_pkcs7_sig__destroy(skel);
        return;
    }

    /* Test successful signature verification with testing keyring. */
    (*(*skel).bss).user_keyring_serial = syscall(
        __NR_request_key,
        c"keyring".as_ptr(),
        c"ebpf_testing_keyring".as_ptr(),
        ptr::null::<c_void>(),
        KEY_SPEC_SESSION_KEYRING,
    );

    ret = bpf_map_update_elem(
        bpf_map__fd(map),
        &mut zero as *mut c_int as *const c_void,
        &mut data as *mut data as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_OK(ret, c"bpf_map_update_elem data_input".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_verify_pkcs7_sig__destroy(skel);
        return;
    }

    /*
     * Ensure key_task_permission() is called and rejects the keyring
     * (no Search permission).
     */
    syscall(
        __NR_keyctl,
        KEYCTL_SETPERM,
        (*(*skel).bss).user_keyring_serial,
        0x37373737,
    );

    ret = bpf_map_update_elem(
        bpf_map__fd(map),
        &mut zero as *mut c_int as *const c_void,
        &mut data as *mut data as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_LT(ret, 0, c"bpf_map_update_elem data_input".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_verify_pkcs7_sig__destroy(skel);
        return;
    }

    syscall(
        __NR_keyctl,
        KEYCTL_SETPERM,
        (*(*skel).bss).user_keyring_serial,
        0x3f3f3f3f,
    );

    /*
     * Ensure key_validate() is called and rejects the keyring (key expired)
     */
    syscall(
        __NR_keyctl,
        KEYCTL_SET_TIMEOUT,
        (*(*skel).bss).user_keyring_serial,
        1,
    );
    sleep(1);

    ret = bpf_map_update_elem(
        bpf_map__fd(map),
        &mut zero as *mut c_int as *const c_void,
        &mut data as *mut data as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_LT(ret, 0, c"bpf_map_update_elem data_input".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_verify_pkcs7_sig__destroy(skel);
        return;
    }

    (*(*skel).bss).user_keyring_serial = KEY_SPEC_SESSION_KEYRING;

    /* Test with corrupted data (signature verification should fail). */
    data.data[0] = b'a';
    ret = bpf_map_update_elem(
        bpf_map__fd(map),
        &mut zero as *mut c_int as *const c_void,
        &mut data as *mut data as *const c_void,
        BPF_ANY,
    );
    if !ASSERT_LT(ret, 0, c"bpf_map_update_elem data_input".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_verify_pkcs7_sig__destroy(skel);
        return;
    }

    ret = populate_data_item_mod(&mut data);
    if !ASSERT_OK(ret, c"populate_data_item_mod".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_verify_pkcs7_sig__destroy(skel);
        return;
    }

    /* Test signature verification with system keyrings. */
    if data.data_len != 0 {
        (*(*skel).bss).user_keyring_serial = 0;
        (*(*skel).bss).system_keyring_id = 0;

        ret = bpf_map_update_elem(
            bpf_map__fd(map),
            &mut zero as *mut c_int as *const c_void,
            &mut data as *mut data as *const c_void,
            BPF_ANY,
        );
        if !ASSERT_OK(ret, c"bpf_map_update_elem data_input".as_ptr()) {
            _run_setup_process(tmp_dir, c"cleanup".as_ptr());
            (*(*skel).bss).monitored_pid = 0;
            test_verify_pkcs7_sig__destroy(skel);
            return;
        }

        (*(*skel).bss).system_keyring_id = VERIFY_USE_SECONDARY_KEYRING;

        ret = bpf_map_update_elem(
            bpf_map__fd(map),
            &mut zero as *mut c_int as *const c_void,
            &mut data as *mut data as *const c_void,
            BPF_ANY,
        );
        if !ASSERT_OK(ret, c"bpf_map_update_elem data_input".as_ptr()) {
            _run_setup_process(tmp_dir, c"cleanup".as_ptr());
            (*(*skel).bss).monitored_pid = 0;
            test_verify_pkcs7_sig__destroy(skel);
            return;
        }

        (*(*skel).bss).system_keyring_id = VERIFY_USE_PLATFORM_KEYRING;

        ret = bpf_map_update_elem(
            bpf_map__fd(map),
            &mut zero as *mut c_int as *const c_void,
            &mut data as *mut data as *const c_void,
            BPF_ANY,
        );
        ASSERT_LT(ret, 0, c"bpf_map_update_elem data_input".as_ptr());
    }

    _run_setup_process(tmp_dir, c"cleanup".as_ptr());

    if skel.is_null() {
        return;
    }

    (*(*skel).bss).monitored_pid = 0;
    test_verify_pkcs7_sig__destroy(skel);
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn getpid() -> pid_t;
}

unsafe fn printf_skip_kfunc() {
    printf(
        c"%s:SKIP:bpf_verify_pkcs7_signature() kfunc not supported\n".as_ptr(),
        c"test_verify_pkcs7_sig_from_map".as_ptr(),
    );
}

unsafe fn get_signature_size(sig_path: *const c_char) -> c_int {
    let mut st: stat = core::mem::zeroed();

    if stat(sig_path, &mut st) == -1 {
        return -1;
    }

    stat_st_size(&st) as c_int
}

unsafe fn add_signature_to_xattr(data_path: *const c_char, sig_path: *const c_char) -> c_int {
    let mut sig = [0 as c_char; MAX_SIG_SIZE];
    let fd: c_int;
    let size: c_int;
    let ret: c_int;

    if !sig_path.is_null() {
        fd = open(sig_path, O_RDONLY);
        if fd < 0 {
            return -1;
        }

        size = read(fd, sig.as_mut_ptr() as *mut c_void, MAX_SIG_SIZE) as c_int;
        close(fd);
        if size <= 0 {
            return -1;
        }
    } else {
        /* no sig_path, just write 32 bytes of zeros */
        size = 32;
    }
    ret = setxattr(
        data_path,
        c"user.sig".as_ptr(),
        sig.as_ptr() as *const c_void,
        size as size_t,
        0,
    );
    if !ASSERT_OK(ret, c"setxattr".as_ptr()) {
        return -1;
    }

    0
}

unsafe fn test_open_file(
    skel: *mut test_sig_in_xattr,
    data_path: *mut c_char,
    pid: pid_t,
    should_success: bool,
    name: *mut c_char,
) -> c_int {
    let ret: c_int;

    (*(*skel).bss).monitored_pid = pid;
    ret = open(data_path, O_RDONLY);
    close(ret);
    (*(*skel).bss).monitored_pid = 0;

    if should_success {
        if !ASSERT_GE(ret, 0, name) {
            return -1;
        }
    } else if !ASSERT_LT(ret, 0, name) {
        return -1;
    }
    0
}

unsafe fn test_pkcs7_sig_fsverity() {
    let mut data_path = [0 as c_char; PATH_MAX];
    let mut sig_path = [0 as c_char; PATH_MAX];
    let mut tmp_dir_template = *b"/tmp/verify_sigXXXXXX\0";
    let tmp_dir: *mut c_char;
    let mut skel: *mut test_sig_in_xattr = ptr::null_mut();
    let pid: pid_t;
    let mut ret: c_int;

    tmp_dir = mkdtemp(tmp_dir_template.as_mut_ptr() as *mut c_char);
    if !ASSERT_OK_PTR(tmp_dir, c"mkdtemp".as_ptr()) {
        return;
    }

    snprintf(
        data_path.as_mut_ptr(),
        PATH_MAX,
        c"%s/data-file".as_ptr(),
        tmp_dir,
    );
    snprintf(
        sig_path.as_mut_ptr(),
        PATH_MAX,
        c"%s/sig-file".as_ptr(),
        tmp_dir,
    );

    ret = _run_setup_process(tmp_dir, c"setup".as_ptr());
    if !ASSERT_OK(ret, c"_run_setup_process".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        return;
    }

    ret = _run_setup_process(tmp_dir, c"fsverity-create-sign".as_ptr());

    if ret != 0 {
        printf(
            c"%s: SKIP: fsverity [sign|enable] doesn't work.\nTo run this test, try enable CONFIG_FS_VERITY and enable FSVerity for the filesystem.\n".as_ptr(),
            c"test_pkcs7_sig_fsverity".as_ptr(),
        );
        test__skip();
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        return;
    }

    skel = test_sig_in_xattr__open();
    if !ASSERT_OK_PTR(skel, c"test_sig_in_xattr__open".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        return;
    }
    ret = get_signature_size(sig_path.as_ptr());
    if !ASSERT_GT(ret, 0, c"get_signature_size".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        if !skel.is_null() {
            (*(*skel).bss).monitored_pid = 0;
            test_sig_in_xattr__destroy(skel);
        }
        return;
    }
    (*(*skel).bss).sig_size = ret;
    (*(*skel).bss).user_keyring_serial = syscall(
        __NR_request_key,
        c"keyring".as_ptr(),
        c"ebpf_testing_keyring".as_ptr(),
        ptr::null::<c_void>(),
        KEY_SPEC_SESSION_KEYRING,
    );
    memcpy(
        (*(*skel).bss).digest.as_mut_ptr() as *mut c_void,
        c"FSVerity".as_ptr() as *const c_void,
        8,
    );

    ret = test_sig_in_xattr__load(skel);
    if !ASSERT_OK(ret, c"test_sig_in_xattr__load".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_sig_in_xattr__destroy(skel);
        return;
    }

    ret = test_sig_in_xattr__attach(skel);
    if !ASSERT_OK(ret, c"test_sig_in_xattr__attach".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_sig_in_xattr__destroy(skel);
        return;
    }

    pid = getpid();

    /* Case 1: fsverity is not enabled, open should succeed */
    if test_open_file(
        skel,
        data_path.as_mut_ptr(),
        pid,
        true,
        c"open_1".as_ptr() as *mut c_char,
    ) != 0
    {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_sig_in_xattr__destroy(skel);
        return;
    }

    /* Case 2: fsverity is enabled, xattr is missing, open should
     * fail
     */
    ret = _run_setup_process(tmp_dir, c"fsverity-enable".as_ptr());
    if !ASSERT_OK(ret, c"fsverity-enable".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_sig_in_xattr__destroy(skel);
        return;
    }
    if test_open_file(
        skel,
        data_path.as_mut_ptr(),
        pid,
        false,
        c"open_2".as_ptr() as *mut c_char,
    ) != 0
    {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_sig_in_xattr__destroy(skel);
        return;
    }

    /* Case 3: fsverity is enabled, xattr has valid signature, open
     * should succeed
     */
    ret = add_signature_to_xattr(data_path.as_ptr(), sig_path.as_ptr());
    if !ASSERT_OK(ret, c"add_signature_to_xattr_1".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_sig_in_xattr__destroy(skel);
        return;
    }

    if test_open_file(
        skel,
        data_path.as_mut_ptr(),
        pid,
        true,
        c"open_3".as_ptr() as *mut c_char,
    ) != 0
    {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_sig_in_xattr__destroy(skel);
        return;
    }

    /* Case 4: fsverity is enabled, xattr has invalid signature, open
     * should fail
     */
    ret = add_signature_to_xattr(data_path.as_ptr(), ptr::null());
    if !ASSERT_OK(ret, c"add_signature_to_xattr_2".as_ptr()) {
        _run_setup_process(tmp_dir, c"cleanup".as_ptr());
        (*(*skel).bss).monitored_pid = 0;
        test_sig_in_xattr__destroy(skel);
        return;
    }
    test_open_file(
        skel,
        data_path.as_mut_ptr(),
        pid,
        false,
        c"open_4".as_ptr() as *mut c_char,
    );

    _run_setup_process(tmp_dir, c"cleanup".as_ptr());
    if skel.is_null() {
        return;
    }

    (*(*skel).bss).monitored_pid = 0;
    test_sig_in_xattr__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_verify_pkcs7_sig() {
    if test__start_subtest(c"pkcs7_sig_from_map".as_ptr()) {
        test_verify_pkcs7_sig_from_map();
    }
    if test__start_subtest(c"pkcs7_sig_fsverity".as_ptr()) {
        test_pkcs7_sig_fsverity();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
