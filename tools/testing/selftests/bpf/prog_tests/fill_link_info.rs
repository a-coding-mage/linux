// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023 Yafang Shao <laoar.shao@gmail.com> */

/* Translated from C. External libbpf, kernel, skeleton, and test harness symbols
 * are intentionally referenced as dependencies supplied by the surrounding tree.
 */

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

const TP_CAT: *const c_char = b"sched\0".as_ptr() as *const c_char;
const TP_NAME: *const c_char = b"sched_switch\0".as_ptr() as *const c_char;

static mut kmulti_syms: [*const c_char; 3] = [
    b"bpf_fentry_test2\0".as_ptr() as *const c_char,
    b"bpf_fentry_test1\0".as_ptr() as *const c_char,
    b"bpf_fentry_test3\0".as_ptr() as *const c_char,
];
const KMULTI_CNT: usize = 3;
static mut kmulti_addrs: [__u64; KMULTI_CNT] = [0; KMULTI_CNT];
static mut kmulti_cookies: [__u64; 3] = [3, 1, 2];

const KPROBE_FUNC: *const c_char = b"bpf_fentry_test1\0".as_ptr() as *const c_char;
static mut kprobe_addr: __u64 = 0;

static tmulti_syms: [*const c_char; 3] = [
    b"bpf_fentry_test2\0".as_ptr() as *const c_char,
    b"bpf_fentry_test1\0".as_ptr() as *const c_char,
    b"bpf_fentry_test3\0".as_ptr() as *const c_char,
];

static mut tmulti_cookies: [__u64; 3] = [30, 10, 20];
const TRACING_MULTI_CNT: usize = 3;

#[repr(C)]
struct tmulti_target {
    name: *const c_char,
    addr: __u64,
    cookie: __u64,
    id: __u32,
}

const UPROBE_FILE: *const c_char = b"/proc/self/exe\0".as_ptr() as *const c_char;
static mut uprobe_offset: ssize_t = 0;

/* uprobe attach point */
#[inline(never)]
unsafe fn uprobe_func() {
    core::arch::asm!("", options(nostack, preserves_flags));
}

const PERF_EVENT_COOKIE: __u64 = 0xdeadbeef;

unsafe fn verify_perf_link_info(
    fd: c_int,
    type_: bpf_perf_event_type,
    addr: c_long,
    offset: ssize_t,
    entry_offset: ssize_t,
) -> c_int {
    let ref_ctr_offset: ssize_t = entry_offset; /* ref_ctr_offset for uprobes */
    let mut info: bpf_link_info = zeroed();
    let mut len: __u32 = size_of::<bpf_link_info>() as __u32;
    let mut buf: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
    let mut err: c_int;

    'again: loop {
        err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
        if !ASSERT_OK(err, b"get_link_info\0".as_ptr() as *const c_char) {
            return -1;
        }

        if !ASSERT_EQ(info.type_, BPF_LINK_TYPE_PERF_EVENT, b"link_type\0".as_ptr() as *const c_char) {
            return -1;
        }
        if !ASSERT_EQ(info.perf_event.type_, type_, b"perf_type_match\0".as_ptr() as *const c_char) {
            return -1;
        }

        match info.perf_event.type_ {
            BPF_PERF_EVENT_KPROBE | BPF_PERF_EVENT_KRETPROBE => {
                ASSERT_EQ(info.perf_event.kprobe.offset, offset, b"kprobe_offset\0".as_ptr() as *const c_char);

                /* In case kernel.kptr_restrict is not permitted or MAX_SYMS is reached */
                if addr != 0 {
                    ASSERT_EQ(
                        info.perf_event.kprobe.addr,
                        (addr + entry_offset as c_long) as __u64,
                        b"kprobe_addr\0".as_ptr() as *const c_char,
                    );
                }

                ASSERT_EQ(info.perf_event.kprobe.cookie, PERF_EVENT_COOKIE, b"kprobe_cookie\0".as_ptr() as *const c_char);
                ASSERT_EQ(
                    info.perf_event.kprobe.name_len,
                    strlen(KPROBE_FUNC) + 1,
                    b"name_len\0".as_ptr() as *const c_char,
                );
                if info.perf_event.kprobe.func_name == 0 {
                    info.perf_event.kprobe.func_name = ptr_to_u64(buf.as_mut_ptr() as *mut c_void);
                    info.perf_event.kprobe.name_len = size_of_val(&buf) as _;
                    continue 'again;
                }

                err = strncmp(
                    u64_to_ptr(info.perf_event.kprobe.func_name) as *const c_char,
                    KPROBE_FUNC,
                    strlen(KPROBE_FUNC),
                );
                ASSERT_EQ(err, 0, b"cmp_kprobe_func_name\0".as_ptr() as *const c_char);
            }
            BPF_PERF_EVENT_TRACEPOINT => {
                ASSERT_EQ(
                    info.perf_event.tracepoint.name_len,
                    strlen(TP_NAME) + 1,
                    b"name_len\0".as_ptr() as *const c_char,
                );
                if info.perf_event.tracepoint.tp_name == 0 {
                    info.perf_event.tracepoint.tp_name = ptr_to_u64(buf.as_mut_ptr() as *mut c_void);
                    info.perf_event.tracepoint.name_len = size_of_val(&buf) as _;
                    continue 'again;
                }

                ASSERT_EQ(info.perf_event.tracepoint.cookie, PERF_EVENT_COOKIE, b"tracepoint_cookie\0".as_ptr() as *const c_char);
                err = strncmp(
                    u64_to_ptr(info.perf_event.tracepoint.tp_name) as *const c_char,
                    TP_NAME,
                    strlen(TP_NAME),
                );
                ASSERT_EQ(err, 0, b"cmp_tp_name\0".as_ptr() as *const c_char);
            }
            BPF_PERF_EVENT_UPROBE | BPF_PERF_EVENT_URETPROBE => {
                ASSERT_EQ(info.perf_event.uprobe.offset, offset, b"uprobe_offset\0".as_ptr() as *const c_char);
                ASSERT_EQ(info.perf_event.uprobe.ref_ctr_offset, ref_ctr_offset, b"uprobe_ref_ctr_offset\0".as_ptr() as *const c_char);
                ASSERT_EQ(
                    info.perf_event.uprobe.name_len,
                    strlen(UPROBE_FILE) + 1,
                    b"name_len\0".as_ptr() as *const c_char,
                );
                if info.perf_event.uprobe.file_name == 0 {
                    info.perf_event.uprobe.file_name = ptr_to_u64(buf.as_mut_ptr() as *mut c_void);
                    info.perf_event.uprobe.name_len = size_of_val(&buf) as _;
                    continue 'again;
                }

                ASSERT_EQ(info.perf_event.uprobe.cookie, PERF_EVENT_COOKIE, b"uprobe_cookie\0".as_ptr() as *const c_char);
                err = strncmp(
                    u64_to_ptr(info.perf_event.uprobe.file_name) as *const c_char,
                    UPROBE_FILE,
                    strlen(UPROBE_FILE),
                );
                ASSERT_EQ(err, 0, b"cmp_file_name\0".as_ptr() as *const c_char);
            }
            BPF_PERF_EVENT_EVENT => {
                ASSERT_EQ(info.perf_event.event.type_, PERF_TYPE_SOFTWARE, b"event_type\0".as_ptr() as *const c_char);
                ASSERT_EQ(info.perf_event.event.config, PERF_COUNT_SW_PAGE_FAULTS, b"event_config\0".as_ptr() as *const c_char);
                ASSERT_EQ(info.perf_event.event.cookie, PERF_EVENT_COOKIE, b"event_cookie\0".as_ptr() as *const c_char);
            }
            _ => {
                err = -1;
            }
        }
        break;
    }
    err
}

unsafe fn kprobe_fill_invalid_user_buffer(fd: c_int) {
    let mut info: bpf_link_info = zeroed();
    let mut len: __u32 = size_of::<bpf_link_info>() as __u32;
    let mut err: c_int;

    info.perf_event.kprobe.func_name = 0x1; /* invalid address */
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EINVAL, b"invalid_buff_and_len\0".as_ptr() as *const c_char);

    info.perf_event.kprobe.name_len = 64;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EFAULT, b"invalid_buff\0".as_ptr() as *const c_char);

    info.perf_event.kprobe.func_name = 0;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EINVAL, b"invalid_len\0".as_ptr() as *const c_char);

    ASSERT_EQ(info.perf_event.kprobe.addr, 0, b"func_addr\0".as_ptr() as *const c_char);
    ASSERT_EQ(info.perf_event.kprobe.offset, 0, b"func_offset\0".as_ptr() as *const c_char);
    ASSERT_EQ(info.perf_event.type_, 0, b"type\0".as_ptr() as *const c_char);
}

unsafe fn test_kprobe_fill_link_info(skel: *mut test_fill_link_info, type_: bpf_perf_event_type, invalid: bool) {
    let mut opts: bpf_kprobe_opts = zeroed();
    opts.attach_mode = PROBE_ATTACH_MODE_LINK;
    opts.retprobe = type_ == BPF_PERF_EVENT_KRETPROBE;
    opts.bpf_cookie = PERF_EVENT_COOKIE;
    let mut entry_offset: ssize_t = 0;
    let link: *mut bpf_link;
    let link_fd: c_int;
    let err: c_int;

    link = bpf_program__attach_kprobe_opts((*skel).progs.kprobe_run, KPROBE_FUNC, &mut opts);
    if !ASSERT_OK_PTR(link as *const c_void, b"attach_kprobe\0".as_ptr() as *const c_char) {
        return;
    }

    link_fd = bpf_link__fd(link);
    if !invalid {
        /* See also arch_adjust_kprobe_addr(). */
        if (*(*skel).kconfig).CONFIG_X86_KERNEL_IBT {
            entry_offset = 4;
        }
        if (*(*skel).kconfig).CONFIG_PPC64
            && (*(*skel).kconfig).CONFIG_KPROBES_ON_FTRACE
            && !(*(*skel).kconfig).CONFIG_PPC_FTRACE_OUT_OF_LINE
        {
            entry_offset = 4;
        }
        err = verify_perf_link_info(link_fd, type_, kprobe_addr as c_long, 0, entry_offset);
        ASSERT_OK(err, b"verify_perf_link_info\0".as_ptr() as *const c_char);
    } else {
        kprobe_fill_invalid_user_buffer(link_fd);
    }
    bpf_link__destroy(link);
}

unsafe fn test_tp_fill_link_info(skel: *mut test_fill_link_info) {
    let mut opts: bpf_tracepoint_opts = zeroed();
    opts.bpf_cookie = PERF_EVENT_COOKIE;
    let link: *mut bpf_link;
    let link_fd: c_int;
    let err: c_int;

    link = bpf_program__attach_tracepoint_opts((*skel).progs.tp_run, TP_CAT, TP_NAME, &mut opts);
    if !ASSERT_OK_PTR(link as *const c_void, b"attach_tp\0".as_ptr() as *const c_char) {
        return;
    }

    link_fd = bpf_link__fd(link);
    err = verify_perf_link_info(link_fd, BPF_PERF_EVENT_TRACEPOINT, 0, 0, 0);
    ASSERT_OK(err, b"verify_perf_link_info\0".as_ptr() as *const c_char);
    bpf_link__destroy(link);
}

unsafe fn test_event_fill_link_info(skel: *mut test_fill_link_info) {
    let mut opts: bpf_perf_event_opts = zeroed();
    opts.bpf_cookie = PERF_EVENT_COOKIE;
    let link: *mut bpf_link;
    let link_fd: c_int;
    let err: c_int;
    let mut attr: perf_event_attr = zeroed();
    attr.type_ = PERF_TYPE_SOFTWARE;
    attr.config = PERF_COUNT_SW_PAGE_FAULTS;
    attr.freq = 1;
    attr.sample_freq = 1;
    attr.size = size_of::<perf_event_attr>() as _;

    let pfd = syscall(__NR_perf_event_open, &mut attr, -1, 0, -1, 0) as c_int;
    if !ASSERT_GE(pfd, 0, b"perf_event_open\0".as_ptr() as *const c_char) {
        return;
    }

    link = bpf_program__attach_perf_event_opts((*skel).progs.event_run, pfd, &mut opts);
    if !ASSERT_OK_PTR(link as *const c_void, b"attach_event\0".as_ptr() as *const c_char) {
        close(pfd);
        return;
    }

    link_fd = bpf_link__fd(link);
    err = verify_perf_link_info(link_fd, BPF_PERF_EVENT_EVENT, 0, 0, 0);
    ASSERT_OK(err, b"verify_perf_link_info\0".as_ptr() as *const c_char);
    bpf_link__destroy(link);
    close(pfd);
}

unsafe fn test_uprobe_fill_link_info(skel: *mut test_fill_link_info, type_: bpf_perf_event_type) {
    let mut opts: bpf_uprobe_opts = zeroed();
    opts.retprobe = type_ == BPF_PERF_EVENT_URETPROBE;
    opts.bpf_cookie = PERF_EVENT_COOKIE;
    let sema: [*const c_char; 1] = [b"uprobe_link_info_sema_1\0".as_ptr() as *const c_char];
    let mut ref_ctr_offset: *mut __u64 = null_mut();
    let link: *mut bpf_link;
    let link_fd: c_int;
    let mut err: c_int;

    err = elf_resolve_syms_offsets(
        b"/proc/self/exe\0".as_ptr() as *const c_char,
        1,
        sema.as_ptr(),
        &mut ref_ctr_offset as *mut *mut __u64 as *mut *mut c_ulong,
        STT_OBJECT,
    );
    if !ASSERT_OK(err, b"elf_resolve_syms_offsets_object\0".as_ptr() as *const c_char) {
        return;
    }

    opts.ref_ctr_offset = *ref_ctr_offset;
    link = bpf_program__attach_uprobe_opts(
        (*skel).progs.uprobe_run,
        0,
        UPROBE_FILE,
        uprobe_offset,
        &mut opts,
    );
    if !ASSERT_OK_PTR(link as *const c_void, b"attach_uprobe\0".as_ptr() as *const c_char) {
        free(ref_ctr_offset as *mut c_void);
        return;
    }

    link_fd = bpf_link__fd(link);
    err = verify_perf_link_info(link_fd, type_, 0, uprobe_offset, *ref_ctr_offset as ssize_t);
    ASSERT_OK(err, b"verify_perf_link_info\0".as_ptr() as *const c_char);
    bpf_link__destroy(link);
    free(ref_ctr_offset as *mut c_void);
}

unsafe fn verify_kmulti_link_info(fd: c_int, retprobe: bool, has_cookies: bool) -> c_int {
    let mut addrs: [__u64; KMULTI_CNT] = [0; KMULTI_CNT];
    let mut cookies: [__u64; KMULTI_CNT] = [0; KMULTI_CNT];
    let mut info: bpf_link_info = zeroed();
    let mut len: __u32 = size_of::<bpf_link_info>() as __u32;
    let flags: c_int;
    let mut err: c_int;

    'again: loop {
        err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
        if !ASSERT_OK(err, b"get_link_info\0".as_ptr() as *const c_char) {
            return -1;
        }

        if !ASSERT_EQ(info.type_, BPF_LINK_TYPE_KPROBE_MULTI, b"kmulti_type\0".as_ptr() as *const c_char) {
            return -1;
        }

        ASSERT_EQ(info.kprobe_multi.count, KMULTI_CNT as __u32, b"func_cnt\0".as_ptr() as *const c_char);
        flags = (info.kprobe_multi.flags & BPF_F_KPROBE_MULTI_RETURN) as c_int;
        if !retprobe {
            ASSERT_EQ(flags, 0, b"kmulti_flags\0".as_ptr() as *const c_char);
        } else {
            ASSERT_NEQ(flags, 0, b"kretmulti_flags\0".as_ptr() as *const c_char);
        }

        if info.kprobe_multi.addrs == 0 {
            info.kprobe_multi.addrs = ptr_to_u64(addrs.as_mut_ptr() as *mut c_void);
            info.kprobe_multi.cookies = ptr_to_u64(cookies.as_mut_ptr() as *mut c_void);
            continue 'again;
        }
        for i in 0..KMULTI_CNT {
            ASSERT_EQ(addrs[i], kmulti_addrs[i], b"kmulti_addrs\0".as_ptr() as *const c_char);
            ASSERT_EQ(
                cookies[i],
                if has_cookies { kmulti_cookies[i] } else { 0 },
                b"kmulti_cookies_value\0".as_ptr() as *const c_char,
            );
        }
        break;
    }
    0
}

unsafe fn verify_kmulti_invalid_user_buffer(fd: c_int) {
    let mut addrs: [__u64; KMULTI_CNT] = [0; KMULTI_CNT];
    let mut cookies: [__u64; KMULTI_CNT] = [0; KMULTI_CNT];
    let mut info: bpf_link_info = zeroed();
    let mut len: __u32 = size_of::<bpf_link_info>() as __u32;
    let mut err: c_int;
    let mut i: usize;

    info.kprobe_multi.count = KMULTI_CNT as __u32;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EINVAL, b"no_addr\0".as_ptr() as *const c_char);

    info.kprobe_multi.addrs = ptr_to_u64(addrs.as_mut_ptr() as *mut c_void);
    info.kprobe_multi.count = 0;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EINVAL, b"no_cnt\0".as_ptr() as *const c_char);

    for item in addrs.iter_mut() {
        *item = 0;
    }
    info.kprobe_multi.count = (KMULTI_CNT - 1) as __u32;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -ENOSPC, b"smaller_cnt\0".as_ptr() as *const c_char);
    i = 0;
    while i < KMULTI_CNT - 1 {
        ASSERT_EQ(addrs[i], kmulti_addrs[i], b"kmulti_addrs\0".as_ptr() as *const c_char);
        i += 1;
    }
    ASSERT_EQ(addrs[i], 0, b"kmulti_addrs\0".as_ptr() as *const c_char);

    for item in addrs.iter_mut() {
        *item = 0;
    }
    info.kprobe_multi.count = (KMULTI_CNT + 1) as __u32;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, 0, b"bigger_cnt\0".as_ptr() as *const c_char);
    for i in 0..KMULTI_CNT {
        ASSERT_EQ(addrs[i], kmulti_addrs[i], b"kmulti_addrs\0".as_ptr() as *const c_char);
    }

    info.kprobe_multi.count = KMULTI_CNT as __u32;
    info.kprobe_multi.addrs = 0x1; /* invalid addr */
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EFAULT, b"invalid_buff_addrs\0".as_ptr() as *const c_char);

    info.kprobe_multi.count = KMULTI_CNT as __u32;
    info.kprobe_multi.addrs = ptr_to_u64(addrs.as_mut_ptr() as *mut c_void);
    info.kprobe_multi.cookies = 0x1; /* invalid addr */
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EFAULT, b"invalid_buff_cookies\0".as_ptr() as *const c_char);

    /* cookies && !count */
    info.kprobe_multi.count = 0;
    info.kprobe_multi.addrs = ptr_to_u64(null_mut());
    info.kprobe_multi.cookies = ptr_to_u64(cookies.as_mut_ptr() as *mut c_void);
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EINVAL, b"invalid_cookies_count\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn symbols_cmp_r(a: *const c_void, b: *const c_void) -> c_int {
    let str_a = a as *const *const c_char;
    let str_b = b as *const *const c_char;

    strcmp(*str_a, *str_b)
}

unsafe fn test_kprobe_multi_fill_link_info(skel: *mut test_fill_link_info, retprobe: bool, cookies: bool, invalid: bool) {
    let mut opts: bpf_kprobe_multi_opts = zeroed();
    let link: *mut bpf_link;
    let link_fd: c_int;
    let err: c_int;

    opts.syms = kmulti_syms.as_ptr();
    opts.cookies = if cookies { kmulti_cookies.as_mut_ptr() } else { null_mut() };
    opts.cnt = KMULTI_CNT as _;
    opts.retprobe = retprobe;
    link = bpf_program__attach_kprobe_multi_opts((*skel).progs.kmulti_run, null(), &mut opts);
    if !ASSERT_OK_PTR(link as *const c_void, b"attach_kprobe_multi\0".as_ptr() as *const c_char) {
        return;
    }

    link_fd = bpf_link__fd(link);
    if !invalid {
        err = verify_kmulti_link_info(link_fd, retprobe, cookies);
        ASSERT_OK(err, b"verify_kmulti_link_info\0".as_ptr() as *const c_char);
    } else {
        verify_kmulti_invalid_user_buffer(link_fd);
    }
    bpf_link__destroy(link);
}

unsafe extern "C" fn tmulti_target_cmp(a: *const c_void, b: *const c_void) -> c_int {
    let ta = a as *const tmulti_target;
    let tb = b as *const tmulti_target;

    (((*ta).id > (*tb).id) as c_int) - (((*ta).id < (*tb).id) as c_int)
}

unsafe fn setup_tmulti_targets(
    prog: *const bpf_program,
    targets: *mut tmulti_target,
    btf_obj_id: *mut __u32,
) -> c_int {
    let mut prog_info: bpf_prog_info = zeroed();
    let mut len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let btf: *mut btf;
    let mut err: c_int;
    let mut id: __s32;

    btf = btf__load_vmlinux_btf();
    if !ASSERT_OK_PTR(btf as *const c_void, b"btf__load_vmlinux_btf\0".as_ptr() as *const c_char) {
        return -1;
    }

    for i in 0..TRACING_MULTI_CNT {
        id = btf__find_by_name_kind(btf, tmulti_syms[i], BTF_KIND_FUNC);
        if !ASSERT_GT(id, 0, b"btf__find_by_name_kind\0".as_ptr() as *const c_char) {
            btf__free(btf);
            return -1;
        }

        (*targets.add(i)).name = tmulti_syms[i];
        (*targets.add(i)).addr = ksym_get_addr(tmulti_syms[i]);
        (*targets.add(i)).cookie = tmulti_cookies[i];
        (*targets.add(i)).id = id as __u32;
    }

    err = bpf_prog_get_info_by_fd(bpf_program__fd(prog), &mut prog_info, &mut len);
    if !ASSERT_OK(err, b"bpf_prog_get_info_by_fd\0".as_ptr() as *const c_char) {
        btf__free(btf);
        return -1;
    }
    if !ASSERT_GT(prog_info.attach_btf_obj_id, 0, b"attach_btf_obj_id\0".as_ptr() as *const c_char) {
        btf__free(btf);
        return -1;
    }
    *btf_obj_id = prog_info.attach_btf_obj_id;

    /*
     * The kernel tracing multi attach sorts ids. We sort as well,
     * so we can easily compare ids and cookies later.
     */
    qsort(
        targets as *mut c_void,
        TRACING_MULTI_CNT,
        size_of::<tmulti_target>(),
        Some(tmulti_target_cmp),
    );
    btf__free(btf);
    0
}

unsafe fn verify_tracing_multi_link_info(
    fd: c_int,
    prog: *const bpf_program,
    targets: *const tmulti_target,
    btf_obj_id: __u32,
    has_cookies: bool,
) -> c_int {
    let attach_type: bpf_attach_type = bpf_program__expected_attach_type(prog);
    let mut addrs: [__u64; TRACING_MULTI_CNT] = [0; TRACING_MULTI_CNT];
    let mut cookies: [__u64; TRACING_MULTI_CNT] = [0; TRACING_MULTI_CNT];
    let mut ids: [__u32; TRACING_MULTI_CNT] = [0; TRACING_MULTI_CNT];
    let mut info: bpf_link_info = zeroed();
    let mut len: __u32 = size_of::<bpf_link_info>() as __u32;
    let mut err: c_int;

    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    if !ASSERT_OK(err, b"bpf_link_get_info_by_fd\0".as_ptr() as *const c_char) {
        return -1;
    }

    if !ASSERT_EQ(info.type_, BPF_LINK_TYPE_TRACING_MULTI, b"info.type\0".as_ptr() as *const c_char) {
        return -1;
    }

    ASSERT_EQ(info.tracing_multi.attach_type, attach_type, b"info.tracing_multi.attach_type\0".as_ptr() as *const c_char);
    ASSERT_EQ(info.tracing_multi.count, TRACING_MULTI_CNT as __u32, b"info.tracing_multi.count\0".as_ptr() as *const c_char);

    info.tracing_multi.ids = ptr_to_u64(ids.as_mut_ptr() as *mut c_void);
    info.tracing_multi.addrs = ptr_to_u64(addrs.as_mut_ptr() as *mut c_void);
    info.tracing_multi.cookies = if has_cookies {
        ptr_to_u64(cookies.as_mut_ptr() as *mut c_void)
    } else {
        0
    };
    info.tracing_multi.count = TRACING_MULTI_CNT as __u32;

    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    if !ASSERT_OK(err, b"bpf_link_get_info_by_fd\0".as_ptr() as *const c_char) {
        return -1;
    }

    if !ASSERT_EQ(info.type_, BPF_LINK_TYPE_TRACING_MULTI, b"info.type\0".as_ptr() as *const c_char) {
        return -1;
    }

    ASSERT_EQ(info.tracing_multi.attach_type, attach_type, b"info.tracing_multi.attach_type\0".as_ptr() as *const c_char);
    ASSERT_EQ(info.tracing_multi.count, TRACING_MULTI_CNT as __u32, b"info.tracing_multi.count\0".as_ptr() as *const c_char);
    ASSERT_EQ(info.tracing_multi.btf_obj_id, btf_obj_id, b"tracing_multi.btf_obj_id\0".as_ptr() as *const c_char);

    for i in 0..TRACING_MULTI_CNT {
        ASSERT_EQ(ids[i], (*targets.add(i)).id, b"tracing_multi.ids\0".as_ptr() as *const c_char);
        ASSERT_EQ(
            cookies[i],
            if has_cookies { (*targets.add(i)).cookie } else { 0 },
            b"tracing_multi.cookies\0".as_ptr() as *const c_char,
        );

        if (*targets.add(i)).addr != 0 {
            let ksym: *mut ksym;

            if !ASSERT_NEQ(addrs[i], 0, b"tracing_multi.addrs\0".as_ptr() as *const c_char) {
                return -1;
            }
            ksym = ksym_search(addrs[i]);
            if !ASSERT_OK_PTR(ksym as *const c_void, b"ksym_search\0".as_ptr() as *const c_char) {
                return -1;
            }
            ASSERT_STREQ((*ksym).name, (*targets.add(i)).name, b"tracing_multi.addr_name\0".as_ptr() as *const c_char);
        } else {
            ASSERT_EQ(addrs[i], 0, b"tracing_multi.addrs\0".as_ptr() as *const c_char);
        }
    }

    0
}

unsafe fn verify_tracing_multi_invalid_user_buffer(fd: c_int, targets: *const tmulti_target) {
    let mut ids: [__u32; TRACING_MULTI_CNT] = [0; TRACING_MULTI_CNT];
    let mut info: bpf_link_info = zeroed();
    let mut len: __u32 = size_of::<bpf_link_info>() as __u32;
    let mut err: c_int;
    let mut i: usize;

    /* Wrong info setup (ids != NULL and cnt == 0) -> EINVAL */
    info.tracing_multi.ids = ptr_to_u64(ids.as_mut_ptr() as *mut c_void);
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EINVAL, b"tracing_multi.invalid_count\0".as_ptr() as *const c_char);

    /* Smaller than actual count provided -> ENOSPC */
    ids = [0; TRACING_MULTI_CNT];
    info = zeroed();
    info.tracing_multi.ids = ptr_to_u64(ids.as_mut_ptr() as *mut c_void);
    info.tracing_multi.count = (TRACING_MULTI_CNT - 1) as __u32;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -ENOSPC, b"tracing_multi.small_count\0".as_ptr() as *const c_char);
    i = 0;
    while i < TRACING_MULTI_CNT - 1 {
        ASSERT_EQ(ids[i], (*targets.add(i)).id, b"tracing_multi.partial_ids\0".as_ptr() as *const c_char);
        i += 1;
    }
    /* check that the last entry is not populated */
    ASSERT_EQ(ids[i], 0, b"tracing_multi.partial_ids\0".as_ptr() as *const c_char);

    /* Bigger than actual count provided -> OK */
    ids = [0; TRACING_MULTI_CNT];
    info = zeroed();
    info.tracing_multi.ids = ptr_to_u64(ids.as_mut_ptr() as *mut c_void);
    info.tracing_multi.count = (TRACING_MULTI_CNT + 1) as __u32;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_OK(err, b"tracing_multi.big_count\0".as_ptr() as *const c_char);
    for i in 0..TRACING_MULTI_CNT {
        ASSERT_EQ(ids[i], (*targets.add(i)).id, b"tracing_multi.ids\0".as_ptr() as *const c_char);
    }

    /* Invalid ids pointer -> EFAULT */
    info = zeroed();
    info.tracing_multi.ids = 0x1;
    info.tracing_multi.count = TRACING_MULTI_CNT as __u32;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EFAULT, b"tracing_multi.bad_btf_ids\0".as_ptr() as *const c_char);

    /* Invalid cookies pointer -> EFAULT */
    info = zeroed();
    info.tracing_multi.cookies = 0x1;
    info.tracing_multi.count = TRACING_MULTI_CNT as __u32;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EFAULT, b"tracing_multi.bad_cookies\0".as_ptr() as *const c_char);

    /* Invalid addrs pointer -> EFAULT */
    info = zeroed();
    info.tracing_multi.addrs = 0x1;
    info.tracing_multi.count = TRACING_MULTI_CNT as __u32;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EFAULT, b"tracing_multi.bad_addrs\0".as_ptr() as *const c_char);
}

unsafe fn test_tracing_multi_fill_link_info(skel: *mut test_fill_link_info, has_cookies: bool, invalid: bool) {
    let mut opts: bpf_tracing_multi_opts = zeroed();
    let mut targets: [tmulti_target; TRACING_MULTI_CNT] = zeroed();
    let mut ids: [__u32; TRACING_MULTI_CNT] = [0; TRACING_MULTI_CNT];
    let mut btf_obj_id: __u32 = 0;
    let mut cookies: [__u64; TRACING_MULTI_CNT] = [0; TRACING_MULTI_CNT];
    let link: *mut bpf_link;
    let link_fd: c_int;
    let err: c_int;

    /* Original C condition:
     * #ifndef __x86_64__
     *     test__skip();
     *     return;
     * #endif
     */
    #[cfg(not(target_arch = "x86_64"))]
    {
        test__skip();
        return;
    }

    if setup_tmulti_targets((*skel).progs.tmulti_run, targets.as_mut_ptr(), &mut btf_obj_id) != 0 {
        return;
    }

    for i in 0..TRACING_MULTI_CNT {
        ids[i] = targets[i].id;
        cookies[i] = targets[i].cookie;
    }

    opts.ids = ids.as_mut_ptr();
    opts.cnt = TRACING_MULTI_CNT as _;
    if has_cookies {
        opts.cookies = cookies.as_mut_ptr();
    }

    link = bpf_program__attach_tracing_multi((*skel).progs.tmulti_run, null(), &mut opts);
    if !ASSERT_OK_PTR(link as *const c_void, b"bpf_program__attach_tracing_multi\0".as_ptr() as *const c_char) {
        return;
    }

    link_fd = bpf_link__fd(link);
    if invalid {
        verify_tracing_multi_invalid_user_buffer(link_fd, targets.as_ptr());
    } else {
        err = verify_tracing_multi_link_info(
            link_fd,
            (*skel).progs.tmulti_run,
            targets.as_ptr(),
            btf_obj_id,
            has_cookies,
        );
        ASSERT_OK(err, b"verify_tracing_multi_link_info\0".as_ptr() as *const c_char);
    }

    bpf_link__destroy(link);
}

/* Original SEC(name) macro: __attribute__((section(name), used)) */
#[used]
#[link_section = ".probes"]
static mut uprobe_link_info_sema_1: i16 = 0;
#[used]
#[link_section = ".probes"]
static mut uprobe_link_info_sema_2: i16 = 0;
#[used]
#[link_section = ".probes"]
static mut uprobe_link_info_sema_3: i16 = 0;

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn uprobe_link_info_func_1() {
    core::arch::asm!("", options(nostack, preserves_flags));
    uprobe_link_info_sema_1 = uprobe_link_info_sema_1.wrapping_add(1);
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn uprobe_link_info_func_2() {
    core::arch::asm!("", options(nostack, preserves_flags));
    uprobe_link_info_sema_2 = uprobe_link_info_sema_2.wrapping_add(1);
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn uprobe_link_info_func_3() {
    core::arch::asm!("", options(nostack, preserves_flags));
    uprobe_link_info_sema_3 = uprobe_link_info_sema_3.wrapping_add(1);
}

unsafe fn verify_umulti_link_info(
    fd: c_int,
    retprobe: bool,
    offsets: *mut __u64,
    cookies: *mut __u64,
    ref_ctr_offsets: *mut __u64,
) -> c_int {
    let mut path: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
    let mut path_buf: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
    let mut info: bpf_link_info = zeroed();
    let mut len: __u32 = size_of::<bpf_link_info>() as __u32;
    let mut ref_ctr_offsets_buf: [__u64; 3] = [0; 3];
    let mut offsets_buf: [__u64; 3] = [0; 3];
    let mut cookies_buf: [__u64; 3] = [0; 3];
    let mut err: c_int;
    let mut count: __u32 = 0;

    err = readlink(
        b"/proc/self/exe\0".as_ptr() as *const c_char,
        path.as_mut_ptr(),
        size_of_val(&path),
    ) as c_int;
    if !ASSERT_NEQ(err, -1, b"readlink\0".as_ptr() as *const c_char) {
        return -1;
    }

    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    if !ASSERT_OK(err, b"bpf_link_get_info_by_fd\0".as_ptr() as *const c_char) {
        return -1;
    }

    ASSERT_EQ(info.uprobe_multi.count, 3, b"info.uprobe_multi.count\0".as_ptr() as *const c_char);
    ASSERT_EQ(info.uprobe_multi.path_size, strlen(path.as_ptr()) + 1, b"info.uprobe_multi.path_size\0".as_ptr() as *const c_char);

    for bit in 0..8 {
        info = zeroed();
        info.uprobe_multi.path = ptr_to_u64(path_buf.as_mut_ptr() as *mut c_void);
        info.uprobe_multi.path_size = size_of_val(&path_buf) as _;
        info.uprobe_multi.count = count;

        if bit & 0x1 != 0 {
            info.uprobe_multi.offsets = ptr_to_u64(offsets_buf.as_mut_ptr() as *mut c_void);
        }
        if bit & 0x2 != 0 {
            info.uprobe_multi.cookies = ptr_to_u64(cookies_buf.as_mut_ptr() as *mut c_void);
        }
        if bit & 0x4 != 0 {
            info.uprobe_multi.ref_ctr_offsets = ptr_to_u64(ref_ctr_offsets_buf.as_mut_ptr() as *mut c_void);
        }

        err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
        if !ASSERT_OK(err, b"bpf_link_get_info_by_fd\0".as_ptr() as *const c_char) {
            return -1;
        }

        if !ASSERT_EQ(info.type_, BPF_LINK_TYPE_UPROBE_MULTI, b"info.type\0".as_ptr() as *const c_char) {
            return -1;
        }

        ASSERT_EQ(info.uprobe_multi.pid, getpid(), b"info.uprobe_multi.pid\0".as_ptr() as *const c_char);
        ASSERT_EQ(info.uprobe_multi.count, 3, b"info.uprobe_multi.count\0".as_ptr() as *const c_char);
        ASSERT_EQ(
            info.uprobe_multi.flags & BPF_F_UPROBE_MULTI_RETURN,
            retprobe as _,
            b"info.uprobe_multi.flags.retprobe\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(info.uprobe_multi.path_size, strlen(path.as_ptr()) + 1, b"info.uprobe_multi.path_size\0".as_ptr() as *const c_char);
        ASSERT_STREQ(path_buf.as_ptr(), path.as_ptr(), b"info.uprobe_multi.path\0".as_ptr() as *const c_char);

        for i in 0..info.uprobe_multi.count as usize {
            if info.uprobe_multi.offsets != 0 {
                ASSERT_EQ(offsets_buf[i], *offsets.add(i), b"info.uprobe_multi.offsets\0".as_ptr() as *const c_char);
            }
            if info.uprobe_multi.cookies != 0 {
                ASSERT_EQ(cookies_buf[i], *cookies.add(i), b"info.uprobe_multi.cookies\0".as_ptr() as *const c_char);
            }
            if info.uprobe_multi.ref_ctr_offsets != 0 {
                ASSERT_EQ(
                    ref_ctr_offsets_buf[i],
                    *ref_ctr_offsets.add(i),
                    b"info.uprobe_multi.ref_ctr_offsets\0".as_ptr() as *const c_char,
                );
            }
        }
        count = if count != 0 { count } else { info.uprobe_multi.count };
    }

    0
}

unsafe fn verify_umulti_invalid_user_buffer(fd: c_int) {
    let mut info: bpf_link_info = zeroed();
    let mut len: __u32 = size_of::<bpf_link_info>() as __u32;
    let mut buf: [__u64; 3] = [0; 3];
    let mut err: c_int;

    /* upath_size defined, not path */
    info.uprobe_multi.path_size = 3;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EINVAL, b"failed_upath_size\0".as_ptr() as *const c_char);

    /* path defined, but small */
    info = zeroed();
    info.uprobe_multi.path = ptr_to_u64(buf.as_mut_ptr() as *mut c_void);
    info.uprobe_multi.path_size = 3;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_LT(err, 0, b"failed_upath_small\0".as_ptr() as *const c_char);

    /* path has wrong pointer */
    info = zeroed();
    info.uprobe_multi.path_size = PATH_MAX as _;
    info.uprobe_multi.path = 123;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EFAULT, b"failed_bad_path_ptr\0".as_ptr() as *const c_char);

    /* count zero, with offsets */
    info = zeroed();
    info.uprobe_multi.offsets = ptr_to_u64(buf.as_mut_ptr() as *mut c_void);
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EINVAL, b"failed_count\0".as_ptr() as *const c_char);

    /* offsets not big enough */
    info = zeroed();
    info.uprobe_multi.offsets = ptr_to_u64(buf.as_mut_ptr() as *mut c_void);
    info.uprobe_multi.count = 2;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -ENOSPC, b"failed_small_count\0".as_ptr() as *const c_char);

    /* offsets has wrong pointer */
    info = zeroed();
    info.uprobe_multi.offsets = 123;
    info.uprobe_multi.count = 3;
    err = bpf_link_get_info_by_fd(fd, &mut info, &mut len);
    ASSERT_EQ(err, -EFAULT, b"failed_wrong_offsets\0".as_ptr() as *const c_char);
}

unsafe fn test_uprobe_multi_fill_link_info(skel: *mut test_fill_link_info, retprobe: bool, invalid: bool) {
    let mut opts: bpf_uprobe_multi_opts = zeroed();
    opts.retprobe = retprobe;
    let syms: [*const c_char; 3] = [
        b"uprobe_link_info_func_1\0".as_ptr() as *const c_char,
        b"uprobe_link_info_func_2\0".as_ptr() as *const c_char,
        b"uprobe_link_info_func_3\0".as_ptr() as *const c_char,
    ];
    let mut cookies: [__u64; 3] = [0xdead, 0xbeef, 0xcafe];
    let sema: [*const c_char; 3] = [
        b"uprobe_link_info_sema_1\0".as_ptr() as *const c_char,
        b"uprobe_link_info_sema_2\0".as_ptr() as *const c_char,
        b"uprobe_link_info_sema_3\0".as_ptr() as *const c_char,
    ];
    let mut offsets: *mut __u64 = null_mut();
    let mut ref_ctr_offsets: *mut __u64 = null_mut();
    let link: *mut bpf_link;
    let link_fd: c_int;
    let mut err: c_int;

    err = elf_resolve_syms_offsets(
        b"/proc/self/exe\0".as_ptr() as *const c_char,
        3,
        sema.as_ptr(),
        &mut ref_ctr_offsets as *mut *mut __u64 as *mut *mut c_ulong,
        STT_OBJECT,
    );
    if !ASSERT_OK(err, b"elf_resolve_syms_offsets_object\0".as_ptr() as *const c_char) {
        return;
    }

    err = elf_resolve_syms_offsets(
        b"/proc/self/exe\0".as_ptr() as *const c_char,
        3,
        syms.as_ptr(),
        &mut offsets as *mut *mut __u64 as *mut *mut c_ulong,
        STT_FUNC,
    );
    if !ASSERT_OK(err, b"elf_resolve_syms_offsets_func\0".as_ptr() as *const c_char) {
        free(ref_ctr_offsets as *mut c_void);
        return;
    }

    opts.syms = syms.as_ptr();
    opts.cookies = cookies.as_mut_ptr();
    opts.ref_ctr_offsets = ref_ctr_offsets as *mut c_ulong;
    opts.cnt = syms.len() as _;

    link = bpf_program__attach_uprobe_multi(
        (*skel).progs.umulti_run,
        0,
        b"/proc/self/exe\0".as_ptr() as *const c_char,
        null(),
        &mut opts,
    );
    if !ASSERT_OK_PTR(link as *const c_void, b"bpf_program__attach_uprobe_multi\0".as_ptr() as *const c_char) {
        free(ref_ctr_offsets as *mut c_void);
        free(offsets as *mut c_void);
        return;
    }

    link_fd = bpf_link__fd(link);
    if invalid {
        verify_umulti_invalid_user_buffer(link_fd);
    } else {
        verify_umulti_link_info(link_fd, retprobe, offsets, cookies.as_mut_ptr(), ref_ctr_offsets);
    }

    bpf_link__destroy(link);
    free(ref_ctr_offsets as *mut c_void);
    free(offsets as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn test_fill_link_info() {
    let skel: *mut test_fill_link_info;

    skel = test_fill_link_info__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr() as *const c_char) {
        return;
    }

    /* load kallsyms to compare the addr */
    if !ASSERT_OK(load_kallsyms(), b"load_kallsyms\0".as_ptr() as *const c_char) {
        test_fill_link_info__destroy(skel);
        return;
    }

    kprobe_addr = ksym_get_addr(KPROBE_FUNC);
    if test__start_subtest(b"kprobe_link_info\0".as_ptr() as *const c_char) {
        test_kprobe_fill_link_info(skel, BPF_PERF_EVENT_KPROBE, false);
    }
    if test__start_subtest(b"kretprobe_link_info\0".as_ptr() as *const c_char) {
        test_kprobe_fill_link_info(skel, BPF_PERF_EVENT_KRETPROBE, false);
    }
    if test__start_subtest(b"kprobe_invalid_ubuff\0".as_ptr() as *const c_char) {
        test_kprobe_fill_link_info(skel, BPF_PERF_EVENT_KPROBE, true);
    }
    if test__start_subtest(b"tracepoint_link_info\0".as_ptr() as *const c_char) {
        test_tp_fill_link_info(skel);
    }
    if test__start_subtest(b"event_link_info\0".as_ptr() as *const c_char) {
        test_event_fill_link_info(skel);
    }

    uprobe_offset = get_uprobe_offset(uprobe_func as *const c_void);
    if test__start_subtest(b"uprobe_link_info\0".as_ptr() as *const c_char) {
        test_uprobe_fill_link_info(skel, BPF_PERF_EVENT_UPROBE);
    }
    if test__start_subtest(b"uretprobe_link_info\0".as_ptr() as *const c_char) {
        test_uprobe_fill_link_info(skel, BPF_PERF_EVENT_URETPROBE);
    }

    qsort(
        kmulti_syms.as_mut_ptr() as *mut c_void,
        KMULTI_CNT,
        size_of::<*const c_char>(),
        Some(symbols_cmp_r),
    );
    for i in 0..KMULTI_CNT {
        kmulti_addrs[i] = ksym_get_addr(kmulti_syms[i]);
    }
    if test__start_subtest(b"kprobe_multi_link_info\0".as_ptr() as *const c_char) {
        test_kprobe_multi_fill_link_info(skel, false, false, false);
        test_kprobe_multi_fill_link_info(skel, false, true, false);
    }
    if test__start_subtest(b"kretprobe_multi_link_info\0".as_ptr() as *const c_char) {
        test_kprobe_multi_fill_link_info(skel, true, false, false);
        test_kprobe_multi_fill_link_info(skel, true, true, false);
    }
    if test__start_subtest(b"kprobe_multi_invalid_ubuff\0".as_ptr() as *const c_char) {
        test_kprobe_multi_fill_link_info(skel, true, true, true);
    }

    if test__start_subtest(b"tracing_multi_link_info\0".as_ptr() as *const c_char) {
        test_tracing_multi_fill_link_info(skel, false, false);
        test_tracing_multi_fill_link_info(skel, true, false);
    }
    if test__start_subtest(b"tracing_multi_invalid_ubuff\0".as_ptr() as *const c_char) {
        test_tracing_multi_fill_link_info(skel, true, true);
    }

    if test__start_subtest(b"uprobe_multi_link_info\0".as_ptr() as *const c_char) {
        test_uprobe_multi_fill_link_info(skel, false, false);
    }
    if test__start_subtest(b"uretprobe_multi_link_info\0".as_ptr() as *const c_char) {
        test_uprobe_multi_fill_link_info(skel, true, false);
    }
    if test__start_subtest(b"uprobe_multi_invalid\0".as_ptr() as *const c_char) {
        test_uprobe_multi_fill_link_info(skel, false, true);
    }

    test_fill_link_info__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
