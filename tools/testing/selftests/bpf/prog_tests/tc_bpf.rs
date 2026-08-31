// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// <test_progs.h>, <linux/pkt_cls.h>, "cap_helpers.h", "test_tc_bpf.skel.h"

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const LO_IFINDEX: c_int = 1;

const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const EEXIST: c_int = 17;

const CAP_BPF: c_int = 39;
const CAP_NET_ADMIN: c_int = 12;
const CAP_SYS_ADMIN: c_int = 21;
const CAP_PERFMON: c_int = 38;

const UINT16_MAX: c_uint = 65535;

const BPF_TC_INGRESS: c_uint = 1;
const BPF_TC_EGRESS: c_uint = 2;
const BPF_TC_CUSTOM: c_uint = 4;
const BPF_TC_F_REPLACE: c_uint = 1;

const TC_H_CLSACT: c_uint = 0xfffffff1;
const TC_H_MIN_INGRESS: c_uint = 0xfff2;

type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct bpf_tc_hook {
    pub sz: usize,
    pub ifindex: c_int,
    pub attach_point: c_uint,
    pub parent: c_uint,
}

#[repr(C)]
pub struct bpf_tc_opts {
    pub sz: usize,
    pub handle: c_uint,
    pub priority: c_uint,
    pub prog_fd: c_int,
    pub prog_id: __u32,
    pub flags: c_uint,
}

#[repr(C)]
pub struct bpf_prog_info {
    pub id: __u32,
}

#[repr(C)]
pub struct test_tc_bpf {
    pub progs: test_tc_bpf_progs,
}

#[repr(C)]
pub struct test_tc_bpf_progs {
    pub cls: *mut bpf_program,
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *__u32) -> c_int;
    fn bpf_tc_attach(hook: *const bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_tc_query(hook: *const bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_tc_detach(hook: *const bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> c_int;
    fn test_tc_bpf__open_and_load() -> *mut test_tc_bpf;
    fn test_tc_bpf__destroy(skel: *mut test_tc_bpf);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn cap_enable_effective(caps: __u64, old_caps: *__u64) -> c_int;
    fn cap_disable_effective(caps: __u64, old_caps: *__u64) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;
}

unsafe fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool {
    unsafe extern "C" {
        fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    }
    unsafe { ASSERT_OK(ret, name) }
}

unsafe fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool {
    unsafe extern "C" {
        fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    }
    unsafe { ASSERT_EQ(actual, expected, name) }
}

unsafe fn ASSERT_EQ_U32(actual: __u32, expected: __u32, name: *const c_char) -> bool {
    unsafe extern "C" {
        fn ASSERT_EQ(actual: __u32, expected: __u32, name: *const c_char) -> bool;
    }
    unsafe { ASSERT_EQ(actual, expected, name) }
}

unsafe fn ASSERT_OK_PTR<T>(ptr: *mut T, name: *const c_char) -> bool {
    unsafe extern "C" {
        fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    }
    unsafe { ASSERT_OK_PTR(ptr as *mut c_void, name) }
}

const fn TC_H_MAKE(maj: c_ulong, min: c_uint) -> c_uint {
    (maj as c_uint) | min
}

const fn bpf_tc_opts_default() -> bpf_tc_opts {
    bpf_tc_opts {
        sz: core::mem::size_of::<bpf_tc_opts>(),
        handle: 0,
        priority: 0,
        prog_fd: 0,
        prog_id: 0,
        flags: 0,
    }
}

const fn bpf_tc_hook_default() -> bpf_tc_hook {
    bpf_tc_hook {
        sz: core::mem::size_of::<bpf_tc_hook>(),
        ifindex: 0,
        attach_point: 0,
        parent: 0,
    }
}

macro_rules! test_declare_opts {
    ($fd:expr) => {
        let mut opts_h = bpf_tc_opts {
            handle: 1,
            ..bpf_tc_opts_default()
        };
        let mut opts_p = bpf_tc_opts {
            priority: 1,
            ..bpf_tc_opts_default()
        };
        let mut opts_f = bpf_tc_opts {
            prog_fd: $fd,
            ..bpf_tc_opts_default()
        };
        let mut opts_hp = bpf_tc_opts {
            handle: 1,
            priority: 1,
            ..bpf_tc_opts_default()
        };
        let mut opts_hf = bpf_tc_opts {
            handle: 1,
            prog_fd: $fd,
            ..bpf_tc_opts_default()
        };
        let mut opts_pf = bpf_tc_opts {
            priority: 1,
            prog_fd: $fd,
            ..bpf_tc_opts_default()
        };
        let mut opts_hpf = bpf_tc_opts {
            handle: 1,
            priority: 1,
            prog_fd: $fd,
            ..bpf_tc_opts_default()
        };
        let mut opts_hpi = bpf_tc_opts {
            handle: 1,
            priority: 1,
            prog_id: 42,
            ..bpf_tc_opts_default()
        };
        let mut opts_hpr = bpf_tc_opts {
            handle: 1,
            priority: 1,
            flags: BPF_TC_F_REPLACE,
            ..bpf_tc_opts_default()
        };
        let mut opts_hpfi = bpf_tc_opts {
            handle: 1,
            priority: 1,
            prog_fd: $fd,
            prog_id: 42,
            ..bpf_tc_opts_default()
        };
        let mut opts_prio_max = bpf_tc_opts {
            handle: 1,
            priority: UINT16_MAX + 1,
            ..bpf_tc_opts_default()
        };
    };
}

unsafe fn test_tc_bpf_basic(hook: *const bpf_tc_hook, fd: c_int) -> c_int {
    let mut opts = bpf_tc_opts {
        handle: 1,
        priority: 1,
        prog_fd: fd,
        ..bpf_tc_opts_default()
    };
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut info_len: __u32 = core::mem::size_of::<bpf_prog_info>() as __u32;
    let mut ret: c_int;

    ret = bpf_prog_get_info_by_fd(fd, &mut info, &mut info_len);
    if !ASSERT_OK(ret, c"bpf_prog_get_info_by_fd".as_ptr()) {
        return ret;
    }

    ret = bpf_tc_attach(hook, &mut opts);
    if !ASSERT_OK(ret, c"bpf_tc_attach".as_ptr()) {
        return ret;
    }

    'body: {
        if !ASSERT_EQ(opts.handle as c_int, 1, c"handle set".as_ptr())
            || !ASSERT_EQ(opts.priority as c_int, 1, c"priority set".as_ptr())
            || !ASSERT_EQ_U32(opts.prog_id, info.id, c"prog_id set".as_ptr())
        {
            break 'body;
        }

        opts.prog_id = 0;
        opts.flags = BPF_TC_F_REPLACE;
        ret = bpf_tc_attach(hook, &mut opts);
        if !ASSERT_OK(ret, c"bpf_tc_attach replace mode".as_ptr()) {
            break 'body;
        }

        opts.prog_id = 0;
        opts.prog_fd = 0;
        opts.flags = 0;
        ret = bpf_tc_query(hook, &mut opts);
        if !ASSERT_OK(ret, c"bpf_tc_query".as_ptr()) {
            break 'body;
        }

        if !ASSERT_EQ(opts.handle as c_int, 1, c"handle set".as_ptr())
            || !ASSERT_EQ(opts.priority as c_int, 1, c"priority set".as_ptr())
            || !ASSERT_EQ_U32(opts.prog_id, info.id, c"prog_id set".as_ptr())
        {
            break 'body;
        }
    }

    opts.prog_id = 0;
    opts.prog_fd = 0;
    opts.flags = 0;
    ret = bpf_tc_detach(hook, &mut opts);
    ASSERT_OK(ret, c"bpf_tc_detach".as_ptr());
    ret
}

unsafe fn test_tc_bpf_api(hook: *mut bpf_tc_hook, fd: c_int) -> c_int {
    let mut attach_opts = bpf_tc_opts {
        handle: 1,
        priority: 1,
        prog_fd: fd,
        ..bpf_tc_opts_default()
    };
    let mut inv_hook = bpf_tc_hook {
        attach_point: BPF_TC_INGRESS,
        ..bpf_tc_hook_default()
    };
    let mut opts = bpf_tc_opts {
        handle: 1,
        priority: 1,
        ..bpf_tc_opts_default()
    };
    let mut ret: c_int;

    ret = bpf_tc_hook_create(core::ptr::null_mut());
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_hook_create invalid hook = NULL".as_ptr()) {
        return -EINVAL;
    }

    /* hook ifindex = 0 */
    ret = bpf_tc_hook_create(&mut inv_hook);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_hook_create invalid hook ifindex == 0".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_hook_destroy(&mut inv_hook);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_hook_destroy invalid hook ifindex == 0".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_attach(&inv_hook, &mut attach_opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_attach invalid hook ifindex == 0".as_ptr()) {
        return -EINVAL;
    }
    attach_opts.prog_id = 0;

    ret = bpf_tc_detach(&inv_hook, &mut opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid hook ifindex == 0".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_query(&inv_hook, &mut opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid hook ifindex == 0".as_ptr()) {
        return -EINVAL;
    }

    /* hook ifindex < 0 */
    inv_hook.ifindex = -1;

    ret = bpf_tc_hook_create(&mut inv_hook);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_hook_create invalid hook ifindex < 0".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_hook_destroy(&mut inv_hook);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_hook_destroy invalid hook ifindex < 0".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_attach(&inv_hook, &mut attach_opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_attach invalid hook ifindex < 0".as_ptr()) {
        return -EINVAL;
    }
    attach_opts.prog_id = 0;

    ret = bpf_tc_detach(&inv_hook, &mut opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid hook ifindex < 0".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_query(&inv_hook, &mut opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid hook ifindex < 0".as_ptr()) {
        return -EINVAL;
    }

    inv_hook.ifindex = LO_IFINDEX;

    /* hook.attach_point invalid */
    inv_hook.attach_point = 0xabcd;
    ret = bpf_tc_hook_create(&mut inv_hook);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_hook_create invalid hook.attach_point".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_hook_destroy(&mut inv_hook);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_hook_destroy invalid hook.attach_point".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_attach(&inv_hook, &mut attach_opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_attach invalid hook.attach_point".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_detach(&inv_hook, &mut opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid hook.attach_point".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_query(&inv_hook, &mut opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid hook.attach_point".as_ptr()) {
        return -EINVAL;
    }

    inv_hook.attach_point = BPF_TC_INGRESS;

    /* hook.attach_point valid, but parent invalid */
    inv_hook.parent = TC_H_MAKE(1_u64.wrapping_shl(16) as c_ulong, 10);
    ret = bpf_tc_hook_create(&mut inv_hook);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_hook_create invalid hook parent".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_hook_destroy(&mut inv_hook);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_hook_destroy invalid hook parent".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_attach(&inv_hook, &mut attach_opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_attach invalid hook parent".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_detach(&inv_hook, &mut opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid hook parent".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_query(&inv_hook, &mut opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid hook parent".as_ptr()) {
        return -EINVAL;
    }

    inv_hook.attach_point = BPF_TC_CUSTOM;
    inv_hook.parent = 0;
    /* These return EOPNOTSUPP instead of EINVAL as parent is checked after
     * attach_point of the hook.
     */
    ret = bpf_tc_hook_create(&mut inv_hook);
    if !ASSERT_EQ(ret, -EOPNOTSUPP, c"bpf_tc_hook_create invalid hook parent".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_hook_destroy(&mut inv_hook);
    if !ASSERT_EQ(ret, -EOPNOTSUPP, c"bpf_tc_hook_destroy invalid hook parent".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_attach(&inv_hook, &mut attach_opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_attach invalid hook parent".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_detach(&inv_hook, &mut opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid hook parent".as_ptr()) {
        return -EINVAL;
    }

    ret = bpf_tc_query(&inv_hook, &mut opts);
    if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid hook parent".as_ptr()) {
        return -EINVAL;
    }

    inv_hook.attach_point = BPF_TC_INGRESS;

    /* detach */
    {
        test_declare_opts!(fd);

        ret = bpf_tc_detach(core::ptr::null(), &mut opts_hp);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid hook = NULL".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_detach(hook, core::ptr::null_mut());
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid opts = NULL".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_detach(hook, &mut opts_hpr);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid flags set".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_detach(hook, &mut opts_hpf);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid prog_fd set".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_detach(hook, &mut opts_hpi);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid prog_id set".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_detach(hook, &mut opts_p);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid handle unset".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_detach(hook, &mut opts_h);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid priority unset".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_detach(hook, &mut opts_prio_max);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_detach invalid priority > UINT16_MAX".as_ptr()) {
            return -EINVAL;
        }
    }

    /* query */
    {
        test_declare_opts!(fd);

        ret = bpf_tc_query(core::ptr::null(), &mut opts);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid hook = NULL".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_query(hook, core::ptr::null_mut());
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid opts = NULL".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_query(hook, &mut opts_hpr);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid flags set".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_query(hook, &mut opts_hpf);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid prog_fd set".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_query(hook, &mut opts_hpi);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid prog_id set".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_query(hook, &mut opts_p);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid handle unset".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_query(hook, &mut opts_h);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid priority unset".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_query(hook, &mut opts_prio_max);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query invalid priority > UINT16_MAX".as_ptr()) {
            return -EINVAL;
        }

        /* when chain is not present, kernel returns -EINVAL */
        ret = bpf_tc_query(hook, &mut opts_hp);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_query valid handle, priority set".as_ptr()) {
            return -EINVAL;
        }
    }

    /* attach */
    {
        test_declare_opts!(fd);

        ret = bpf_tc_attach(core::ptr::null(), &mut opts_hp);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_attach invalid hook = NULL".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_attach(hook, core::ptr::null_mut());
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_attach invalid opts = NULL".as_ptr()) {
            return -EINVAL;
        }

        opts_hp.flags = 42;
        ret = bpf_tc_attach(hook, &mut opts_hp);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_attach invalid flags".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_attach(hook, core::ptr::null_mut());
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_attach invalid prog_fd unset".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_attach(hook, &mut opts_hpi);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_attach invalid prog_id set".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_attach(hook, &mut opts_pf);
        if !ASSERT_OK(ret, c"bpf_tc_attach valid handle unset".as_ptr()) {
            return -EINVAL;
        }
        opts_pf.prog_id = 0;
        opts_pf.prog_fd = 0;
        ASSERT_OK(bpf_tc_detach(hook, &mut opts_pf), c"bpf_tc_detach".as_ptr());

        ret = bpf_tc_attach(hook, &mut opts_hf);
        if !ASSERT_OK(ret, c"bpf_tc_attach valid priority unset".as_ptr()) {
            return -EINVAL;
        }
        opts_hf.prog_id = 0;
        opts_hf.prog_fd = 0;
        ASSERT_OK(bpf_tc_detach(hook, &mut opts_hf), c"bpf_tc_detach".as_ptr());

        ret = bpf_tc_attach(hook, &mut opts_prio_max);
        if !ASSERT_EQ(ret, -EINVAL, c"bpf_tc_attach invalid priority > UINT16_MAX".as_ptr()) {
            return -EINVAL;
        }

        ret = bpf_tc_attach(hook, &mut opts_f);
        if !ASSERT_OK(ret, c"bpf_tc_attach valid both handle and priority unset".as_ptr()) {
            return -EINVAL;
        }
        opts_f.prog_id = 0;
        opts_f.prog_fd = 0;
        ASSERT_OK(bpf_tc_detach(hook, &mut opts_f), c"bpf_tc_detach".as_ptr());
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tc_bpf_root() {
    let mut hook = bpf_tc_hook {
        ifindex: LO_IFINDEX,
        attach_point: BPF_TC_INGRESS,
        ..bpf_tc_hook_default()
    };
    let mut skel: *mut test_tc_bpf = core::ptr::null_mut();
    let mut hook_created: bool = false;
    let cls_fd: c_int;
    let mut ret: c_int;

    skel = test_tc_bpf__open_and_load();
    if !ASSERT_OK_PTR(skel, c"test_tc_bpf__open_and_load".as_ptr()) {
        return;
    }

    cls_fd = bpf_program__fd((*skel).progs.cls);

    ret = bpf_tc_hook_create(&mut hook);
    if ret == 0 {
        hook_created = true;
    }

    'body: {
        ret = if ret == -EEXIST { 0 } else { ret };
        if !ASSERT_OK(ret, c"bpf_tc_hook_create(BPF_TC_INGRESS)".as_ptr()) {
            break 'body;
        }

        hook.attach_point = BPF_TC_CUSTOM;
        hook.parent = TC_H_MAKE(TC_H_CLSACT as c_ulong, TC_H_MIN_INGRESS);
        ret = bpf_tc_hook_create(&mut hook);
        if !ASSERT_EQ(ret, -EOPNOTSUPP, c"bpf_tc_hook_create invalid hook.attach_point".as_ptr()) {
            break 'body;
        }

        ret = test_tc_bpf_basic(&hook, cls_fd);
        if !ASSERT_OK(ret, c"test_tc_internal ingress".as_ptr()) {
            break 'body;
        }

        ret = bpf_tc_hook_destroy(&mut hook);
        if !ASSERT_EQ(ret, -EOPNOTSUPP, c"bpf_tc_hook_destroy invalid hook.attach_point".as_ptr()) {
            break 'body;
        }

        hook.attach_point = BPF_TC_INGRESS;
        hook.parent = 0;
        bpf_tc_hook_destroy(&mut hook);

        ret = test_tc_bpf_basic(&hook, cls_fd);
        if !ASSERT_OK(ret, c"test_tc_internal ingress".as_ptr()) {
            break 'body;
        }

        bpf_tc_hook_destroy(&mut hook);

        hook.attach_point = BPF_TC_EGRESS;
        ret = test_tc_bpf_basic(&hook, cls_fd);
        if !ASSERT_OK(ret, c"test_tc_internal egress".as_ptr()) {
            break 'body;
        }

        bpf_tc_hook_destroy(&mut hook);

        ret = test_tc_bpf_api(&mut hook, cls_fd);
        if !ASSERT_OK(ret, c"test_tc_bpf_api".as_ptr()) {
            break 'body;
        }

        bpf_tc_hook_destroy(&mut hook);
    }

    if hook_created {
        hook.attach_point = BPF_TC_INGRESS | BPF_TC_EGRESS;
        bpf_tc_hook_destroy(&mut hook);
    }
    test_tc_bpf__destroy(skel);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tc_bpf_non_root() {
    let mut skel: *mut test_tc_bpf = core::ptr::null_mut();
    let mut caps: __u64 = 0;
    let mut ret: c_int;

    /* In case CAP_BPF and CAP_PERFMON is not set */
    ret = cap_enable_effective(
        (1_u64 << CAP_BPF) | (1_u64 << CAP_NET_ADMIN),
        &mut caps,
    );
    if !ASSERT_OK(ret, c"set_cap_bpf_cap_net_admin".as_ptr()) {
        return;
    }
    'restore_cap: {
        ret = cap_disable_effective(
            (1_u64 << CAP_SYS_ADMIN) | (1_u64 << CAP_PERFMON),
            core::ptr::null_mut(),
        );
        if !ASSERT_OK(ret, c"disable_cap_sys_admin".as_ptr()) {
            break 'restore_cap;
        }

        skel = test_tc_bpf__open_and_load();
        if !ASSERT_OK_PTR(skel, c"test_tc_bpf__open_and_load".as_ptr()) {
            break 'restore_cap;
        }

        test_tc_bpf__destroy(skel);
    }

    if caps != 0 {
        cap_enable_effective(caps, core::ptr::null_mut());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_tc_bpf() {
    if test__start_subtest(c"tc_bpf_root".as_ptr()) {
        tc_bpf_root();
    }
    if test__start_subtest(c"tc_bpf_non_root".as_ptr()) {
        tc_bpf_non_root();
    }
}
