// SPDX-License-Identifier: GPL-2.0
// Translated from C source. Original includes:
// #include <test_progs.h>
// #include <bpf/btf.h>

use core::ffi::{c_char, c_int, c_uint, c_void};

const TRACEFS: &[u8] = b"/sys/kernel/tracing\0";
const DEBUGFS_TRACING: &[u8] = b"/sys/kernel/debug/tracing\0";
const EVENT_SUBPATH: &[u8] = b"events/bpf_testmod/bpf_testmod_test_read/btf_ids\0";

const F_OK: c_int = 0;
const O_RDONLY: c_int = 0;
const EIO: c_int = 5;
const EINVAL: c_int = 22;

#[repr(C)]
struct btf_ids_info {
    obj_id: u32,
    raw_id: u32,
    tp_id: u32,
}

#[repr(C)]
struct btf {
    _private: [u8; 0],
}

#[repr(C)]
struct btf_type {
    name_off: u32,
}

#[repr(C)]
struct btf_param {
    name_off: u32,
}

#[repr(C)]
struct btf_member {
    name_off: u32,
}

#[repr(C)]
struct env {
    has_testmod: bool,
}

unsafe extern "C" {
    static env: env;

    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn test__skip();
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_TRUE(actual: bool, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;

    fn btf__name_by_offset(btf: *mut btf, offset: u32) -> *const c_char;
    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf__load_from_kernel_by_id_split(id: u32, base_btf: *mut btf) -> *mut btf;
    fn btf__free(btf: *mut btf);
    fn btf__type_by_id(btf: *mut btf, type_id: u32) -> *const btf_type;
    fn btf_is_func_proto(t: *const btf_type) -> bool;
    fn btf_vlen(t: *const btf_type) -> c_uint;
    fn btf_params(t: *const btf_type) -> *const btf_param;
    fn btf_is_struct(t: *const btf_type) -> bool;
    fn btf_members(t: *const btf_type) -> *const btf_member;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn btf_ids_path(buf: *mut c_char, sz: usize) -> *const c_char {
    if access(b"/sys/kernel/tracing/trace\0".as_ptr() as *const c_char, F_OK) == 0 {
        snprintf(
            buf,
            sz,
            b"%s/%s\0".as_ptr() as *const c_char,
            TRACEFS.as_ptr() as *const c_char,
            EVENT_SUBPATH.as_ptr() as *const c_char,
        );
    } else {
        snprintf(
            buf,
            sz,
            b"%s/%s\0".as_ptr() as *const c_char,
            DEBUGFS_TRACING.as_ptr() as *const c_char,
            EVENT_SUBPATH.as_ptr() as *const c_char,
        );
    }
    buf
}

unsafe fn read_btf_ids(info: *mut btf_ids_info) -> c_int {
    let mut path = [0 as c_char; 256];
    let mut buf = [0 as c_char; 256];
    let fd: c_int;
    let n: isize;

    fd = open(btf_ids_path(path.as_mut_ptr(), core::mem::size_of_val(&path)), O_RDONLY);
    if fd < 0 {
        return -errno();
    }

    n = read(
        fd,
        buf.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&buf) - 1,
    );
    close(fd);
    if n <= 0 {
        return -EIO;
    }
    buf[n as usize] = b'\0' as c_char;

    if sscanf(
        buf.as_ptr(),
        b"btf_obj_id: %u\nraw_btf_id: %u\ntp_btf_id: %u\n\0".as_ptr() as *const c_char,
        &mut (*info).obj_id as *mut u32,
        &mut (*info).raw_id as *mut u32,
        &mut (*info).tp_id as *mut u32,
    ) != 3
    {
        return -EINVAL;
    }
    0
}

unsafe fn param_name(btf: *mut btf, p: *const btf_param) -> *const c_char {
    btf__name_by_offset(btf, (*p).name_off)
}

unsafe fn member_name(btf: *mut btf, m: *const btf_member) -> *const c_char {
    btf__name_by_offset(btf, (*m).name_off)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_tp_btf_ids() {
    let mut proto_t: *const btf_type;
    let mut rec_t: *const btf_type;
    let mut params: *const btf_param;
    let mut members: *const btf_member;
    let mut info = btf_ids_info {
        obj_id: 0,
        raw_id: 0,
        tp_id: 0,
    };
    let vmlinux_btf: *mut btf;
    let btf: *mut btf;
    let name: *const c_char;
    let err: c_int;

    if !env.has_testmod {
        test__skip();
        return;
    }

    err = read_btf_ids(&mut info);
    if !ASSERT_OK(err, b"read btf_ids\0".as_ptr() as *const c_char) {
        return;
    }

    ASSERT_GT(info.obj_id, 0, b"obj_id non-zero\0".as_ptr() as *const c_char);
    ASSERT_GT(info.raw_id, 0, b"raw_id non-zero\0".as_ptr() as *const c_char);
    ASSERT_GT(info.tp_id, 0, b"tp_id non-zero\0".as_ptr() as *const c_char);

    vmlinux_btf = btf__load_vmlinux_btf();
    if !ASSERT_OK_PTR(vmlinux_btf as *const c_void, b"load vmlinux BTF\0".as_ptr() as *const c_char) {
        return;
    }

    /* Module BTF is split BTF; load with vmlinux as base. */
    btf = btf__load_from_kernel_by_id_split(info.obj_id, vmlinux_btf);
    if !ASSERT_OK_PTR(btf as *const c_void, b"load module BTF\0".as_ptr() as *const c_char) {
        btf__free(vmlinux_btf);
        return;
    }

    /*
     * raw_btf_id should be the FUNC_PROTO of __bpf_trace_<call>:
     *   void *__data, struct task_struct *task,
     *   struct bpf_testmod_test_read_ctx *ctx
     */
    proto_t = btf__type_by_id(btf, info.raw_id);
    if !ASSERT_OK_PTR(proto_t as *const c_void, b"raw type_by_id\0".as_ptr() as *const c_char) {
        goto_out(btf, vmlinux_btf);
        return;
    }
    if !ASSERT_TRUE(btf_is_func_proto(proto_t), b"raw is FUNC_PROTO\0".as_ptr() as *const c_char) {
        goto_out(btf, vmlinux_btf);
        return;
    }
    if !ASSERT_EQ(btf_vlen(proto_t), 3, b"func_proto arg count\0".as_ptr() as *const c_char) {
        goto_out(btf, vmlinux_btf);
        return;
    }

    params = btf_params(proto_t);
    ASSERT_STREQ(param_name(btf, params.add(0)), b"__data\0".as_ptr() as *const c_char, b"arg0 name\0".as_ptr() as *const c_char);
    ASSERT_STREQ(param_name(btf, params.add(1)), b"task\0".as_ptr() as *const c_char, b"arg1 name\0".as_ptr() as *const c_char);
    ASSERT_STREQ(param_name(btf, params.add(2)), b"ctx\0".as_ptr() as *const c_char, b"arg2 name\0".as_ptr() as *const c_char);

    /*
     * tp_btf_id should be STRUCT trace_event_raw_<call> with the
     * fields declared by TP_STRUCT__entry plus the common header.
     */
    rec_t = btf__type_by_id(btf, info.tp_id);
    if !ASSERT_OK_PTR(rec_t as *const c_void, b"tp type_by_id\0".as_ptr() as *const c_char) {
        goto_out(btf, vmlinux_btf);
        return;
    }
    if !ASSERT_TRUE(btf_is_struct(rec_t), b"tp is STRUCT\0".as_ptr() as *const c_char) {
        goto_out(btf, vmlinux_btf);
        return;
    }
    name = btf__name_by_offset(btf, (*rec_t).name_off);
    ASSERT_STREQ(
        name,
        b"trace_event_raw_bpf_testmod_test_read\0".as_ptr() as *const c_char,
        b"tp struct name\0".as_ptr() as *const c_char,
    );
    if !ASSERT_GE(btf_vlen(rec_t), 5, b"tp struct field count\0".as_ptr() as *const c_char) {
        goto_out(btf, vmlinux_btf);
        return;
    }

    members = btf_members(rec_t);
    ASSERT_STREQ(member_name(btf, members.add(0)), b"ent\0".as_ptr() as *const c_char, b"field0 name\0".as_ptr() as *const c_char);
    ASSERT_STREQ(member_name(btf, members.add(1)), b"pid\0".as_ptr() as *const c_char, b"field1 name\0".as_ptr() as *const c_char);
    ASSERT_STREQ(member_name(btf, members.add(2)), b"comm\0".as_ptr() as *const c_char, b"field2 name\0".as_ptr() as *const c_char);
    ASSERT_STREQ(member_name(btf, members.add(3)), b"off\0".as_ptr() as *const c_char, b"field3 name\0".as_ptr() as *const c_char);
    ASSERT_STREQ(member_name(btf, members.add(4)), b"len\0".as_ptr() as *const c_char, b"field4 name\0".as_ptr() as *const c_char);

    goto_out(btf, vmlinux_btf);
}

unsafe fn goto_out(btf: *mut btf, vmlinux_btf: *mut btf) {
    btf__free(btf);
    btf__free(vmlinux_btf);
}
