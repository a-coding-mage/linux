// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of testing/selftests/bpf/prog_tests/attach_probe.c.
// C includes removed; external libbpf/test skeleton symbols are declared below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type ssize_t = isize;
type uintptr_t = usize;

const PATH_MAX: usize = 4096;
const EOPNOTSUPP: c_int = 95;
const EINVAL: c_int = 22;
const BPF_F_SLEEPABLE: c_int = 1 << 4;
const SYS_NANOSLEEP_KPROBE_NAME: *const c_char = c"hrtimer_nanosleep".as_ptr();

#[repr(C)]
#[derive(Copy, Clone)]
enum probe_attach_mode {
    PROBE_ATTACH_MODE_DEFAULT = 0,
    PROBE_ATTACH_MODE_LEGACY = 1,
    PROBE_ATTACH_MODE_PERF = 2,
    PROBE_ATTACH_MODE_LINK = 3,
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_uprobe_opts {
    attach_mode: probe_attach_mode,
    retprobe: bool,
    ref_ctr_offset: ssize_t,
    func_name: *const c_char,
}

impl Default for bpf_uprobe_opts {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
struct bpf_kprobe_opts {
    attach_mode: probe_attach_mode,
    retprobe: bool,
    offset: c_ulong,
}

impl Default for bpf_kprobe_opts {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
struct bpf_test_run_opts {
    _reserved: [u8; 0],
}

impl Default for bpf_test_run_opts {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
struct test_attach_probe_manual_progs {
    handle_kprobe: *mut bpf_program,
    handle_kretprobe: *mut bpf_program,
    handle_uprobe: *mut bpf_program,
    handle_uretprobe: *mut bpf_program,
    handle_uprobe_byname: *mut bpf_program,
}

#[repr(C)]
struct test_attach_probe_manual_links {
    handle_kprobe: *mut bpf_link,
    handle_kretprobe: *mut bpf_link,
    handle_uprobe: *mut bpf_link,
    handle_uretprobe: *mut bpf_link,
    handle_uprobe_byname: *mut bpf_link,
}

#[repr(C)]
struct test_attach_probe_manual_bss {
    kprobe_res: c_int,
    kretprobe_res: c_int,
    uprobe_res: c_int,
    uretprobe_res: c_int,
    uprobe_byname_res: c_int,
}

#[repr(C)]
struct test_attach_probe_manual {
    progs: test_attach_probe_manual_progs,
    links: test_attach_probe_manual_links,
    bss: *mut test_attach_probe_manual_bss,
}

#[repr(C)]
struct test_attach_probe_progs {
    handle_kprobe_auto: *mut bpf_program,
    handle_kretprobe_auto: *mut bpf_program,
    handle_uprobe_byname: *mut bpf_program,
    handle_uretprobe_byname: *mut bpf_program,
    handle_uprobe_byname2: *mut bpf_program,
    handle_uretprobe_byname2: *mut bpf_program,
    handle_uprobe_ref_ctr: *mut bpf_program,
    handle_uretprobe_ref_ctr: *mut bpf_program,
    handle_uprobe_byname3_sleepable: *mut bpf_program,
    handle_uprobe_byname3: *mut bpf_program,
    handle_uretprobe_byname3_sleepable: *mut bpf_program,
    handle_uretprobe_byname3: *mut bpf_program,
}

#[repr(C)]
struct test_attach_probe_links {
    handle_kprobe_auto: *mut bpf_link,
    handle_kretprobe_auto: *mut bpf_link,
    handle_uprobe_byname: *mut bpf_link,
    handle_uretprobe_byname: *mut bpf_link,
    handle_uprobe_byname2: *mut bpf_link,
    handle_uretprobe_byname2: *mut bpf_link,
    handle_uprobe_ref_ctr: *mut bpf_link,
    handle_uretprobe_ref_ctr: *mut bpf_link,
    handle_uprobe_byname3_sleepable: *mut bpf_link,
    handle_uprobe_byname3: *mut bpf_link,
    handle_uretprobe_byname3_sleepable: *mut bpf_link,
    handle_uretprobe_byname3: *mut bpf_link,
}

#[repr(C)]
struct test_attach_probe_bss {
    kprobe2_res: c_int,
    kretprobe2_res: c_int,
    uretprobe_byname_res: c_int,
    uprobe_byname2_res: c_int,
    uretprobe_byname2_res: c_int,
    user_ptr: *mut c_char,
    uprobe_byname3_sleepable_res: c_int,
    uprobe_byname3_str_sleepable_res: c_int,
    uprobe_byname3_res: c_int,
    uretprobe_byname3_sleepable_res: c_int,
    uretprobe_byname3_str_sleepable_res: c_int,
    uretprobe_byname3_res: c_int,
}

#[repr(C)]
struct test_attach_probe {
    progs: test_attach_probe_progs,
    links: test_attach_probe_links,
    bss: *mut test_attach_probe_bss,
}

#[repr(C)]
struct test_attach_kprobe_sleepable_progs {
    handle_kprobe_sleepable: *mut bpf_program,
}

#[repr(C)]
struct test_attach_kprobe_sleepable_links {
    handle_kprobe_sleepable: *mut bpf_link,
}

#[repr(C)]
struct test_attach_kprobe_sleepable {
    progs: test_attach_kprobe_sleepable_progs,
    links: test_attach_kprobe_sleepable_links,
}

#[repr(C)]
struct kprobe_write_ctx_progs {
    kprobe_write_ctx: *mut bpf_program,
    kprobe_dummy: *mut bpf_program,
    fentry: *mut bpf_program,
    freplace_kprobe: *mut bpf_program,
}

#[repr(C)]
struct kprobe_write_ctx {
    progs: kprobe_write_ctx_progs,
}

unsafe extern "C" {
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_ERR_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(a: ssize_t, b: ssize_t, name: *const c_char) -> bool;
    fn ASSERT_GT(a: ssize_t, b: ssize_t, name: *const c_char) -> bool;
    fn ASSERT_EQ(a: c_long, b: c_long, name: *const c_char) -> bool;
    fn ASSERT_NEQ(a: c_ulong, b: c_ulong, name: *const c_char) -> bool;

    fn test_attach_probe_manual__open_and_load() -> *mut test_attach_probe_manual;
    fn test_attach_probe_manual__destroy(skel: *mut test_attach_probe_manual);
    fn test_attach_probe__open() -> *mut test_attach_probe;
    fn test_attach_probe__load(skel: *mut test_attach_probe) -> c_int;
    fn test_attach_probe__destroy(skel: *mut test_attach_probe);
    fn test_attach_kprobe_sleepable__open() -> *mut test_attach_kprobe_sleepable;
    fn test_attach_kprobe_sleepable__load(skel: *mut test_attach_kprobe_sleepable) -> c_int;
    fn test_attach_kprobe_sleepable__destroy(skel: *mut test_attach_kprobe_sleepable);
    fn kprobe_write_ctx__open_and_load() -> *mut kprobe_write_ctx;
    fn kprobe_write_ctx__open() -> *mut kprobe_write_ctx;
    fn kprobe_write_ctx__load(skel: *mut kprobe_write_ctx) -> c_int;
    fn kprobe_write_ctx__destroy(skel: *mut kprobe_write_ctx);

    fn get_uprobe_offset(func: *const c_void) -> ssize_t;
    fn get_rel_offset(addr: uintptr_t) -> ssize_t;
    fn load_kallsyms() -> c_int;
    fn ksym_get_addr(name: *const c_char) -> c_ulong;
    fn bpf_program__attach_kprobe_opts(
        prog: *mut bpf_program,
        name: *const c_char,
        opts: *mut bpf_kprobe_opts,
    ) -> *mut bpf_link;
    fn bpf_program__attach_uprobe_opts(
        prog: *mut bpf_program,
        pid: c_int,
        binary_path: *const c_char,
        func_offset: ssize_t,
        opts: *mut bpf_uprobe_opts,
    ) -> *mut bpf_link;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_program__set_flags(prog: *mut bpf_program, flags: c_int) -> c_int;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_program__set_attach_target(
        prog: *mut bpf_program,
        fd: c_int,
        name: *const c_char,
    ) -> c_int;
    fn bpf_program__attach_freplace(
        prog: *mut bpf_program,
        target_fd: c_int,
        attach_func_name: *const c_char,
    ) -> *mut bpf_link;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn libbpf_get_error(ptr: *mut c_void) -> c_long;
    fn test__start_subtest(name: *const c_char) -> bool;
    fn test__skip();

    fn usleep(usec: c_int) -> c_int;
    fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: usize) -> ssize_t;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
}

/* this is how USDT semaphore is actually defined, except volatile modifier */
#[used]
#[link_section = ".probes"]
#[no_mangle]
static mut uprobe_ref_ctr: u16 = 0;

/* uprobe attach point */
#[inline(never)]
unsafe fn trigger_func() {
    core::arch::asm!("", options(nomem, nostack, preserves_flags));
}

/* attach point for byname uprobe */
#[inline(never)]
unsafe fn trigger_func2() {
    core::arch::asm!("", options(nomem, nostack, preserves_flags));
}

/* attach point for byname sleepable uprobe */
#[inline(never)]
unsafe fn trigger_func3() {
    core::arch::asm!("", options(nomem, nostack, preserves_flags));
}

/* attach point for ref_ctr */
#[inline(never)]
unsafe fn trigger_func4() {
    core::arch::asm!("", options(nomem, nostack, preserves_flags));
}

static mut test_data: [c_char; 10] = *b"test_data\0".as_ptr().cast::<[c_char; 10]>();

/* manual attach kprobe/kretprobe/uprobe/uretprobe testings */
unsafe fn test_attach_probe_manual(attach_mode: probe_attach_mode) {
    let mut uprobe_opts = bpf_uprobe_opts::default();
    let mut kprobe_opts = bpf_kprobe_opts::default();
    let kprobe_link: *mut bpf_link;
    let kretprobe_link: *mut bpf_link;
    let uprobe_link: *mut bpf_link;
    let uretprobe_link: *mut bpf_link;
    let skel: *mut test_attach_probe_manual;
    let uprobe_offset: ssize_t;

    skel = test_attach_probe_manual__open_and_load();
    if !ASSERT_OK_PTR(skel.cast(), c"skel_kprobe_manual_open_and_load".as_ptr()) {
        return;
    }

    uprobe_offset = get_uprobe_offset(trigger_func as *const c_void);
    if !ASSERT_GE(uprobe_offset, 0, c"uprobe_offset".as_ptr()) {
        goto_cleanup_manual(skel);
        return;
    }

    /* manual-attach kprobe/kretprobe */
    kprobe_opts.attach_mode = attach_mode;
    kprobe_opts.retprobe = false;
    kprobe_link = bpf_program__attach_kprobe_opts((*skel).progs.handle_kprobe, SYS_NANOSLEEP_KPROBE_NAME, &mut kprobe_opts);
    if !ASSERT_OK_PTR(kprobe_link.cast(), c"attach_kprobe".as_ptr()) {
        goto_cleanup_manual(skel);
        return;
    }
    (*skel).links.handle_kprobe = kprobe_link;

    kprobe_opts.retprobe = true;
    kretprobe_link = bpf_program__attach_kprobe_opts((*skel).progs.handle_kretprobe, SYS_NANOSLEEP_KPROBE_NAME, &mut kprobe_opts);
    if !ASSERT_OK_PTR(kretprobe_link.cast(), c"attach_kretprobe".as_ptr()) {
        goto_cleanup_manual(skel);
        return;
    }
    (*skel).links.handle_kretprobe = kretprobe_link;

    /* manual-attach uprobe/uretprobe */
    uprobe_opts.attach_mode = attach_mode;
    uprobe_opts.ref_ctr_offset = 0;
    uprobe_opts.retprobe = false;
    uprobe_link = bpf_program__attach_uprobe_opts((*skel).progs.handle_uprobe, 0, c"/proc/self/exe".as_ptr(), uprobe_offset, &mut uprobe_opts);
    if !ASSERT_OK_PTR(uprobe_link.cast(), c"attach_uprobe".as_ptr()) {
        goto_cleanup_manual(skel);
        return;
    }
    (*skel).links.handle_uprobe = uprobe_link;

    uprobe_opts.retprobe = true;
    uretprobe_link = bpf_program__attach_uprobe_opts((*skel).progs.handle_uretprobe, -1, c"/proc/self/exe".as_ptr(), uprobe_offset, &mut uprobe_opts);
    if !ASSERT_OK_PTR(uretprobe_link.cast(), c"attach_uretprobe".as_ptr()) {
        goto_cleanup_manual(skel);
        return;
    }
    (*skel).links.handle_uretprobe = uretprobe_link;

    /* attach uprobe by function name manually */
    uprobe_opts.func_name = c"trigger_func2".as_ptr();
    uprobe_opts.retprobe = false;
    uprobe_opts.ref_ctr_offset = 0;
    (*skel).links.handle_uprobe_byname = bpf_program__attach_uprobe_opts((*skel).progs.handle_uprobe_byname, 0, c"/proc/self/exe".as_ptr(), 0, &mut uprobe_opts);
    if !ASSERT_OK_PTR((*skel).links.handle_uprobe_byname.cast(), c"attach_uprobe_byname".as_ptr()) {
        goto_cleanup_manual(skel);
        return;
    }

    /* trigger & validate kprobe && kretprobe */
    usleep(1);

    /* trigger & validate uprobe & uretprobe */
    trigger_func();

    /* trigger & validate uprobe attached by name */
    trigger_func2();

    ASSERT_EQ((*(*skel).bss).kprobe_res as c_long, 1, c"check_kprobe_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).kretprobe_res as c_long, 2, c"check_kretprobe_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).uprobe_res as c_long, 3, c"check_uprobe_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).uretprobe_res as c_long, 4, c"check_uretprobe_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).uprobe_byname_res as c_long, 5, c"check_uprobe_byname_res".as_ptr());

    goto_cleanup_manual(skel);
}

unsafe fn goto_cleanup_manual(skel: *mut test_attach_probe_manual) {
    test_attach_probe_manual__destroy(skel);
}

/* manual attach address-based kprobe/kretprobe testings */
unsafe fn test_attach_kprobe_by_addr(attach_mode: probe_attach_mode) {
    let mut kprobe_opts = bpf_kprobe_opts::default();
    let skel: *mut test_attach_probe_manual;
    let func_addr: c_ulong;

    if !ASSERT_OK(load_kallsyms(), c"load_kallsyms".as_ptr()) {
        return;
    }

    func_addr = ksym_get_addr(SYS_NANOSLEEP_KPROBE_NAME);
    if !ASSERT_NEQ(func_addr, 0, c"func_addr".as_ptr()) {
        return;
    }

    skel = test_attach_probe_manual__open_and_load();
    if !ASSERT_OK_PTR(skel.cast(), c"skel_kprobe_manual_open_and_load".as_ptr()) {
        return;
    }

    kprobe_opts.attach_mode = attach_mode;
    kprobe_opts.retprobe = false;
    kprobe_opts.offset = func_addr;
    (*skel).links.handle_kprobe = bpf_program__attach_kprobe_opts((*skel).progs.handle_kprobe, core::ptr::null(), &mut kprobe_opts);
    if !ASSERT_OK_PTR((*skel).links.handle_kprobe.cast(), c"attach_kprobe_by_addr".as_ptr()) {
        test_attach_probe_manual__destroy(skel);
        return;
    }

    kprobe_opts.retprobe = true;
    (*skel).links.handle_kretprobe = bpf_program__attach_kprobe_opts((*skel).progs.handle_kretprobe, core::ptr::null(), &mut kprobe_opts);
    if !ASSERT_OK_PTR((*skel).links.handle_kretprobe.cast(), c"attach_kretprobe_by_addr".as_ptr()) {
        test_attach_probe_manual__destroy(skel);
        return;
    }

    /* trigger & validate kprobe && kretprobe */
    usleep(1);

    ASSERT_EQ((*(*skel).bss).kprobe_res as c_long, 1, c"check_kprobe_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).kretprobe_res as c_long, 2, c"check_kretprobe_res".as_ptr());

    test_attach_probe_manual__destroy(skel);
}

/* reject legacy address-based kprobe attach */
unsafe fn test_attach_kprobe_legacy_by_addr_reject() {
    let mut kprobe_opts = bpf_kprobe_opts::default();
    let skel: *mut test_attach_probe_manual;
    let func_addr: c_ulong;

    if !ASSERT_OK(load_kallsyms(), c"load_kallsyms".as_ptr()) {
        return;
    }

    func_addr = ksym_get_addr(SYS_NANOSLEEP_KPROBE_NAME);
    if !ASSERT_NEQ(func_addr, 0, c"func_addr".as_ptr()) {
        return;
    }

    skel = test_attach_probe_manual__open_and_load();
    if !ASSERT_OK_PTR(skel.cast(), c"skel_kprobe_manual_open_and_load".as_ptr()) {
        return;
    }

    kprobe_opts.attach_mode = probe_attach_mode::PROBE_ATTACH_MODE_LEGACY;
    kprobe_opts.offset = func_addr;
    (*skel).links.handle_kprobe = bpf_program__attach_kprobe_opts((*skel).progs.handle_kprobe, core::ptr::null(), &mut kprobe_opts);
    ASSERT_ERR_PTR((*skel).links.handle_kprobe.cast(), c"attach_kprobe_legacy_by_addr".as_ptr());
    ASSERT_EQ(libbpf_get_error((*skel).links.handle_kprobe.cast()), -(EOPNOTSUPP as c_long), c"attach_kprobe_legacy_by_addr_err".as_ptr());

    test_attach_probe_manual__destroy(skel);
}

/*
 * bpf_fentry_shadow_test exists in both vmlinux (net/bpf/test_run.c) and
 * bpf_testmod (bpf_testmod.c). When bpf_testmod is loaded the symbol is
 * duplicated. Test that kprobe attachment handles this correctly:
 * - Unqualified name ("bpf_fentry_shadow_test") attaches to vmlinux.
 * - MOD:SYM name ("bpf_testmod:bpf_fentry_shadow_test") attaches to module.
 *
 * Note: bpf_fentry_shadow_test is not invoked via test_run, so we only
 * verify that attach and detach succeed without triggering the probe.
 */
unsafe fn test_attach_probe_dup_sym(attach_mode: probe_attach_mode) {
    let mut kprobe_opts = bpf_kprobe_opts::default();
    let kprobe_link: *mut bpf_link;
    let kretprobe_link: *mut bpf_link;
    let skel: *mut test_attach_probe_manual;

    skel = test_attach_probe_manual__open_and_load();
    if !ASSERT_OK_PTR(skel.cast(), c"skel_dup_sym_open_and_load".as_ptr()) {
        return;
    }

    kprobe_opts.attach_mode = attach_mode;

    /* Unqualified: should attach to vmlinux symbol */
    kprobe_opts.retprobe = false;
    kprobe_link = bpf_program__attach_kprobe_opts((*skel).progs.handle_kprobe, c"bpf_fentry_shadow_test".as_ptr(), &mut kprobe_opts);
    if !ASSERT_OK_PTR(kprobe_link.cast(), c"attach_kprobe_vmlinux".as_ptr()) {
        test_attach_probe_manual__destroy(skel);
        return;
    }
    bpf_link__destroy(kprobe_link);

    kprobe_opts.retprobe = true;
    kretprobe_link = bpf_program__attach_kprobe_opts((*skel).progs.handle_kretprobe, c"bpf_fentry_shadow_test".as_ptr(), &mut kprobe_opts);
    if !ASSERT_OK_PTR(kretprobe_link.cast(), c"attach_kretprobe_vmlinux".as_ptr()) {
        test_attach_probe_manual__destroy(skel);
        return;
    }
    bpf_link__destroy(kretprobe_link);

    /* MOD:SYM qualified: should attach to module symbol */
    kprobe_opts.retprobe = false;
    kprobe_link = bpf_program__attach_kprobe_opts((*skel).progs.handle_kprobe, c"bpf_testmod:bpf_fentry_shadow_test".as_ptr(), &mut kprobe_opts);
    if !ASSERT_OK_PTR(kprobe_link.cast(), c"attach_kprobe_module".as_ptr()) {
        test_attach_probe_manual__destroy(skel);
        return;
    }
    bpf_link__destroy(kprobe_link);

    kprobe_opts.retprobe = true;
    kretprobe_link = bpf_program__attach_kprobe_opts((*skel).progs.handle_kretprobe, c"bpf_testmod:bpf_fentry_shadow_test".as_ptr(), &mut kprobe_opts);
    if !ASSERT_OK_PTR(kretprobe_link.cast(), c"attach_kretprobe_module".as_ptr()) {
        test_attach_probe_manual__destroy(skel);
        return;
    }
    bpf_link__destroy(kretprobe_link);

    test_attach_probe_manual__destroy(skel);
}

/* attach uprobe/uretprobe long event name testings */
unsafe fn test_attach_uprobe_long_event_name() {
    let mut uprobe_opts = bpf_uprobe_opts::default();
    let uprobe_link: *mut bpf_link;
    let uretprobe_link: *mut bpf_link;
    let skel: *mut test_attach_probe_manual;
    let uprobe_offset: ssize_t;
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];

    skel = test_attach_probe_manual__open_and_load();
    if !ASSERT_OK_PTR(skel.cast(), c"skel_kprobe_manual_open_and_load".as_ptr()) {
        return;
    }

    uprobe_offset = get_uprobe_offset(trigger_func as *const c_void);
    if !ASSERT_GE(uprobe_offset, 0, c"uprobe_offset".as_ptr()) {
        test_attach_probe_manual__destroy(skel);
        return;
    }

    if !ASSERT_GT(readlink(c"/proc/self/exe".as_ptr(), path.as_mut_ptr(), PATH_MAX - 1), 0, c"readlink".as_ptr()) {
        test_attach_probe_manual__destroy(skel);
        return;
    }

    /* manual-attach uprobe/uretprobe */
    uprobe_opts.attach_mode = probe_attach_mode::PROBE_ATTACH_MODE_LEGACY;
    uprobe_opts.ref_ctr_offset = 0;
    uprobe_opts.retprobe = false;
    uprobe_link = bpf_program__attach_uprobe_opts((*skel).progs.handle_uprobe, 0, path.as_ptr(), uprobe_offset, &mut uprobe_opts);
    if !ASSERT_OK_PTR(uprobe_link.cast(), c"attach_uprobe_long_event_name".as_ptr()) {
        test_attach_probe_manual__destroy(skel);
        return;
    }
    (*skel).links.handle_uprobe = uprobe_link;

    uprobe_opts.retprobe = true;
    uretprobe_link = bpf_program__attach_uprobe_opts((*skel).progs.handle_uretprobe, -1, path.as_ptr(), uprobe_offset, &mut uprobe_opts);
    if !ASSERT_OK_PTR(uretprobe_link.cast(), c"attach_uretprobe_long_event_name".as_ptr()) {
        test_attach_probe_manual__destroy(skel);
        return;
    }
    (*skel).links.handle_uretprobe = uretprobe_link;

    test_attach_probe_manual__destroy(skel);
}

/* attach kprobe/kretprobe long event name testings */
unsafe fn test_attach_kprobe_long_event_name() {
    let mut kprobe_opts = bpf_kprobe_opts::default();
    let kprobe_link: *mut bpf_link;
    let kretprobe_link: *mut bpf_link;
    let skel: *mut test_attach_probe_manual;

    skel = test_attach_probe_manual__open_and_load();
    if !ASSERT_OK_PTR(skel.cast(), c"skel_kprobe_manual_open_and_load".as_ptr()) {
        return;
    }

    /* manual-attach kprobe/kretprobe */
    kprobe_opts.attach_mode = probe_attach_mode::PROBE_ATTACH_MODE_LEGACY;
    kprobe_opts.retprobe = false;
    kprobe_link = bpf_program__attach_kprobe_opts((*skel).progs.handle_kprobe, c"bpf_testmod_looooooooooooooooooooooooooooooong_name".as_ptr(), &mut kprobe_opts);
    if !ASSERT_OK_PTR(kprobe_link.cast(), c"attach_kprobe_long_event_name".as_ptr()) {
        test_attach_probe_manual__destroy(skel);
        return;
    }
    (*skel).links.handle_kprobe = kprobe_link;

    kprobe_opts.retprobe = true;
    kretprobe_link = bpf_program__attach_kprobe_opts((*skel).progs.handle_kretprobe, c"bpf_testmod_looooooooooooooooooooooooooooooong_name".as_ptr(), &mut kprobe_opts);
    if !ASSERT_OK_PTR(kretprobe_link.cast(), c"attach_kretprobe_long_event_name".as_ptr()) {
        test_attach_probe_manual__destroy(skel);
        return;
    }
    (*skel).links.handle_kretprobe = kretprobe_link;

    test_attach_probe_manual__destroy(skel);
}

/* Original C uses #ifdef __x86_64__; Rust cfg preserves that build-time intent. */
#[cfg(target_arch = "x86_64")]
unsafe fn test_attach_kprobe_write_ctx() {
    let mut skel: *mut kprobe_write_ctx = core::ptr::null_mut();
    let mut link: *mut bpf_link = core::ptr::null_mut();

    skel = kprobe_write_ctx__open_and_load();
    if !ASSERT_OK_PTR(skel.cast(), c"kprobe_write_ctx__open_and_load".as_ptr()) {
        return;
    }

    link = bpf_program__attach_kprobe_opts((*skel).progs.kprobe_write_ctx, c"bpf_fentry_test1".as_ptr(), core::ptr::null_mut());
    if !ASSERT_ERR_PTR(link.cast(), c"bpf_program__attach_kprobe_opts".as_ptr()) {
        bpf_link__destroy(link);
    }

    kprobe_write_ctx__destroy(skel);
}

#[cfg(target_arch = "x86_64")]
unsafe fn test_freplace_kprobe_write_ctx() {
    let prog_kprobe: *mut bpf_program;
    let prog_ext: *mut bpf_program;
    let prog_fentry: *mut bpf_program;
    let skel_kprobe: *mut kprobe_write_ctx;
    let mut skel_ext: *mut kprobe_write_ctx = core::ptr::null_mut();
    let mut link_kprobe: *mut bpf_link = core::ptr::null_mut();
    let mut link_ext: *mut bpf_link = core::ptr::null_mut();
    let mut err: c_int;
    let mut prog_fd: c_int;
    let mut kprobe_opts = bpf_kprobe_opts::default();
    let mut topts = bpf_test_run_opts::default();

    skel_kprobe = kprobe_write_ctx__open();
    if !ASSERT_OK_PTR(skel_kprobe.cast(), c"kprobe_write_ctx__open kprobe".as_ptr()) {
        return;
    }

    prog_kprobe = (*skel_kprobe).progs.kprobe_dummy;
    bpf_program__set_autoload(prog_kprobe, true);

    prog_fentry = (*skel_kprobe).progs.fentry;
    bpf_program__set_autoload(prog_fentry, true);

    err = kprobe_write_ctx__load(skel_kprobe);
    if !ASSERT_OK(err, c"kprobe_write_ctx__load kprobe".as_ptr()) {
        kprobe_write_ctx__destroy(skel_ext);
        kprobe_write_ctx__destroy(skel_kprobe);
        return;
    }

    skel_ext = kprobe_write_ctx__open();
    if !ASSERT_OK_PTR(skel_ext.cast(), c"kprobe_write_ctx__open ext".as_ptr()) {
        kprobe_write_ctx__destroy(skel_ext);
        kprobe_write_ctx__destroy(skel_kprobe);
        return;
    }

    prog_ext = (*skel_ext).progs.freplace_kprobe;
    bpf_program__set_autoload(prog_ext, true);

    prog_fd = bpf_program__fd((*skel_kprobe).progs.kprobe_write_ctx);
    bpf_program__set_attach_target(prog_ext, prog_fd, c"kprobe_write_ctx".as_ptr());

    err = kprobe_write_ctx__load(skel_ext);
    if !ASSERT_OK(err, c"kprobe_write_ctx__load ext".as_ptr()) {
        bpf_link__destroy(link_ext);
        bpf_link__destroy(link_kprobe);
        kprobe_write_ctx__destroy(skel_ext);
        kprobe_write_ctx__destroy(skel_kprobe);
        return;
    }

    prog_fd = bpf_program__fd(prog_kprobe);
    link_ext = bpf_program__attach_freplace(prog_ext, prog_fd, c"kprobe_dummy".as_ptr());
    ASSERT_ERR_PTR(link_ext.cast(), c"bpf_program__attach_freplace link".as_ptr());
    ASSERT_EQ(libbpf_get_error(link_ext.cast()), -(EINVAL as c_long), c"bpf_program__attach_freplace error".as_ptr());

    link_kprobe = bpf_program__attach_kprobe_opts(prog_kprobe, c"bpf_fentry_test1".as_ptr(), &mut kprobe_opts);
    if !ASSERT_OK_PTR(link_kprobe.cast(), c"bpf_program__attach_kprobe_opts".as_ptr()) {
        bpf_link__destroy(link_ext);
        bpf_link__destroy(link_kprobe);
        kprobe_write_ctx__destroy(skel_ext);
        kprobe_write_ctx__destroy(skel_kprobe);
        return;
    }

    err = bpf_prog_test_run_opts(bpf_program__fd(prog_fentry), &mut topts);
    ASSERT_OK(err, c"bpf_prog_test_run_opts".as_ptr());

    bpf_link__destroy(link_ext);
    bpf_link__destroy(link_kprobe);
    kprobe_write_ctx__destroy(skel_ext);
    kprobe_write_ctx__destroy(skel_kprobe);
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn test_attach_kprobe_write_ctx() {
    test__skip();
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn test_freplace_kprobe_write_ctx() {
    test__skip();
}

unsafe fn test_attach_probe_auto(skel: *mut test_attach_probe) {
    let uprobe_err_link: *mut bpf_link;

    /* auto-attachable kprobe and kretprobe */
    (*skel).links.handle_kprobe_auto = bpf_program__attach((*skel).progs.handle_kprobe_auto);
    ASSERT_OK_PTR((*skel).links.handle_kprobe_auto.cast(), c"attach_kprobe_auto".as_ptr());

    (*skel).links.handle_kretprobe_auto = bpf_program__attach((*skel).progs.handle_kretprobe_auto);
    ASSERT_OK_PTR((*skel).links.handle_kretprobe_auto.cast(), c"attach_kretprobe_auto".as_ptr());

    /* verify auto-attach fails for old-style uprobe definition */
    uprobe_err_link = bpf_program__attach((*skel).progs.handle_uprobe_byname);
    if !ASSERT_EQ(libbpf_get_error(uprobe_err_link.cast()), -(EOPNOTSUPP as c_long), c"auto-attach should fail for old-style name".as_ptr()) {
        return;
    }

    /* verify auto-attach works */
    (*skel).links.handle_uretprobe_byname = bpf_program__attach((*skel).progs.handle_uretprobe_byname);
    if !ASSERT_OK_PTR((*skel).links.handle_uretprobe_byname.cast(), c"attach_uretprobe_byname".as_ptr()) {
        return;
    }

    /* trigger & validate kprobe && kretprobe */
    usleep(1);

    /* trigger & validate uprobe attached by name */
    trigger_func2();

    ASSERT_EQ((*(*skel).bss).kprobe2_res as c_long, 11, c"check_kprobe_auto_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).kretprobe2_res as c_long, 22, c"check_kretprobe_auto_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).uretprobe_byname_res as c_long, 6, c"check_uretprobe_byname_res".as_ptr());
}

unsafe fn test_uprobe_lib(skel: *mut test_attach_probe) {
    let mut uprobe_opts = bpf_uprobe_opts::default();
    let devnull: *mut FILE;

    /* test attach by name for a library function, using the library
     * as the binary argument. libc.so.6 will be resolved via dlopen()/dlinfo().
     */
    uprobe_opts.func_name = c"fopen".as_ptr();
    uprobe_opts.retprobe = false;
    (*skel).links.handle_uprobe_byname2 = bpf_program__attach_uprobe_opts((*skel).progs.handle_uprobe_byname2, 0, c"libc.so.6".as_ptr(), 0, &mut uprobe_opts);
    if !ASSERT_OK_PTR((*skel).links.handle_uprobe_byname2.cast(), c"attach_uprobe_byname2".as_ptr()) {
        return;
    }

    uprobe_opts.func_name = c"fclose".as_ptr();
    uprobe_opts.retprobe = true;
    (*skel).links.handle_uretprobe_byname2 = bpf_program__attach_uprobe_opts((*skel).progs.handle_uretprobe_byname2, -1, c"libc.so.6".as_ptr(), 0, &mut uprobe_opts);
    if !ASSERT_OK_PTR((*skel).links.handle_uretprobe_byname2.cast(), c"attach_uretprobe_byname2".as_ptr()) {
        return;
    }

    /* trigger & validate shared library u[ret]probes attached by name */
    devnull = fopen(c"/dev/null".as_ptr(), c"r".as_ptr());
    fclose(devnull);

    ASSERT_EQ((*(*skel).bss).uprobe_byname2_res as c_long, 7, c"check_uprobe_byname2_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).uretprobe_byname2_res as c_long, 8, c"check_uretprobe_byname2_res".as_ptr());
}

unsafe fn test_uprobe_ref_ctr(skel: *mut test_attach_probe) {
    let mut uprobe_opts = bpf_uprobe_opts::default();
    let uprobe_link: *mut bpf_link;
    let uretprobe_link: *mut bpf_link;
    let uprobe_offset: ssize_t;
    let ref_ctr_offset: ssize_t;

    uprobe_offset = get_uprobe_offset(trigger_func4 as *const c_void);
    if !ASSERT_GE(uprobe_offset, 0, c"uprobe_offset_ref_ctr".as_ptr()) {
        return;
    }

    ref_ctr_offset = get_rel_offset(core::ptr::addr_of!(uprobe_ref_ctr) as uintptr_t);
    if !ASSERT_GE(ref_ctr_offset, 0, c"ref_ctr_offset".as_ptr()) {
        return;
    }

    ASSERT_EQ(core::ptr::read_volatile(core::ptr::addr_of!(uprobe_ref_ctr)) as c_long, 0, c"uprobe_ref_ctr_before".as_ptr());

    uprobe_opts.retprobe = false;
    uprobe_opts.ref_ctr_offset = ref_ctr_offset;
    uprobe_link = bpf_program__attach_uprobe_opts((*skel).progs.handle_uprobe_ref_ctr, 0, c"/proc/self/exe".as_ptr(), uprobe_offset, &mut uprobe_opts);
    if !ASSERT_OK_PTR(uprobe_link.cast(), c"attach_uprobe_ref_ctr".as_ptr()) {
        return;
    }
    (*skel).links.handle_uprobe_ref_ctr = uprobe_link;

    ASSERT_GT(core::ptr::read_volatile(core::ptr::addr_of!(uprobe_ref_ctr)) as ssize_t, 0, c"uprobe_ref_ctr_after".as_ptr());

    /* if uprobe uses ref_ctr, uretprobe has to use ref_ctr as well */
    uprobe_opts.retprobe = true;
    uprobe_opts.ref_ctr_offset = ref_ctr_offset;
    uretprobe_link = bpf_program__attach_uprobe_opts((*skel).progs.handle_uretprobe_ref_ctr, -1, c"/proc/self/exe".as_ptr(), uprobe_offset, &mut uprobe_opts);
    if !ASSERT_OK_PTR(uretprobe_link.cast(), c"attach_uretprobe_ref_ctr".as_ptr()) {
        return;
    }
    (*skel).links.handle_uretprobe_ref_ctr = uretprobe_link;
}

unsafe fn test_kprobe_sleepable() {
    let skel: *mut test_attach_kprobe_sleepable;

    skel = test_attach_kprobe_sleepable__open();
    if !ASSERT_OK_PTR(skel.cast(), c"skel_kprobe_sleepable_open".as_ptr()) {
        return;
    }

    /* sleepable kprobe test case needs flags set before loading */
    if !ASSERT_OK(
        bpf_program__set_flags((*skel).progs.handle_kprobe_sleepable, BPF_F_SLEEPABLE),
        c"kprobe_sleepable_flags".as_ptr(),
    ) {
        test_attach_kprobe_sleepable__destroy(skel);
        return;
    }

    if !ASSERT_OK(
        test_attach_kprobe_sleepable__load(skel),
        c"skel_kprobe_sleepable_load".as_ptr(),
    ) {
        test_attach_kprobe_sleepable__destroy(skel);
        return;
    }

    /* sleepable kprobes should not attach successfully */
    (*skel).links.handle_kprobe_sleepable = bpf_program__attach((*skel).progs.handle_kprobe_sleepable);
    ASSERT_ERR_PTR((*skel).links.handle_kprobe_sleepable.cast(), c"attach_kprobe_sleepable".as_ptr());

    test_attach_kprobe_sleepable__destroy(skel);
}

unsafe fn test_uprobe_sleepable(skel: *mut test_attach_probe) {
    /* test sleepable uprobe and uretprobe variants */
    (*skel).links.handle_uprobe_byname3_sleepable = bpf_program__attach((*skel).progs.handle_uprobe_byname3_sleepable);
    if !ASSERT_OK_PTR((*skel).links.handle_uprobe_byname3_sleepable.cast(), c"attach_uprobe_byname3_sleepable".as_ptr()) {
        return;
    }

    (*skel).links.handle_uprobe_byname3 = bpf_program__attach((*skel).progs.handle_uprobe_byname3);
    if !ASSERT_OK_PTR((*skel).links.handle_uprobe_byname3.cast(), c"attach_uprobe_byname3".as_ptr()) {
        return;
    }

    (*skel).links.handle_uretprobe_byname3_sleepable = bpf_program__attach((*skel).progs.handle_uretprobe_byname3_sleepable);
    if !ASSERT_OK_PTR((*skel).links.handle_uretprobe_byname3_sleepable.cast(), c"attach_uretprobe_byname3_sleepable".as_ptr()) {
        return;
    }

    (*skel).links.handle_uretprobe_byname3 = bpf_program__attach((*skel).progs.handle_uretprobe_byname3);
    if !ASSERT_OK_PTR((*skel).links.handle_uretprobe_byname3.cast(), c"attach_uretprobe_byname3".as_ptr()) {
        return;
    }

    (*(*skel).bss).user_ptr = core::ptr::addr_of_mut!(test_data).cast::<c_char>();

    /* trigger & validate sleepable uprobe attached by name */
    trigger_func3();

    ASSERT_EQ((*(*skel).bss).uprobe_byname3_sleepable_res as c_long, 9, c"check_uprobe_byname3_sleepable_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).uprobe_byname3_str_sleepable_res as c_long, 10, c"check_uprobe_byname3_str_sleepable_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).uprobe_byname3_res as c_long, 11, c"check_uprobe_byname3_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).uretprobe_byname3_sleepable_res as c_long, 12, c"check_uretprobe_byname3_sleepable_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).uretprobe_byname3_str_sleepable_res as c_long, 13, c"check_uretprobe_byname3_str_sleepable_res".as_ptr());
    ASSERT_EQ((*(*skel).bss).uretprobe_byname3_res as c_long, 14, c"check_uretprobe_byname3_res".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn test_attach_probe() {
    let skel: *mut test_attach_probe;

    skel = test_attach_probe__open();
    if !ASSERT_OK_PTR(skel.cast(), c"skel_open".as_ptr()) {
        return;
    }

    if !ASSERT_OK(test_attach_probe__load(skel), c"skel_load".as_ptr()) {
        test_attach_probe__destroy(skel);
        return;
    }
    if !ASSERT_OK_PTR((*skel).bss.cast(), c"check_bss".as_ptr()) {
        test_attach_probe__destroy(skel);
        return;
    }

    if test__start_subtest(c"manual-default".as_ptr()) {
        test_attach_probe_manual(probe_attach_mode::PROBE_ATTACH_MODE_DEFAULT);
    }
    if test__start_subtest(c"manual-legacy".as_ptr()) {
        test_attach_probe_manual(probe_attach_mode::PROBE_ATTACH_MODE_LEGACY);
    }
    if test__start_subtest(c"manual-perf".as_ptr()) {
        test_attach_probe_manual(probe_attach_mode::PROBE_ATTACH_MODE_PERF);
    }
    if test__start_subtest(c"manual-link".as_ptr()) {
        test_attach_probe_manual(probe_attach_mode::PROBE_ATTACH_MODE_LINK);
    }
    if test__start_subtest(c"kprobe-perf-by-addr".as_ptr()) {
        test_attach_kprobe_by_addr(probe_attach_mode::PROBE_ATTACH_MODE_PERF);
    }
    if test__start_subtest(c"kprobe-link-by-addr".as_ptr()) {
        test_attach_kprobe_by_addr(probe_attach_mode::PROBE_ATTACH_MODE_LINK);
    }
    if test__start_subtest(c"kprobe-legacy-by-addr-reject".as_ptr()) {
        test_attach_kprobe_legacy_by_addr_reject();
    }

    if test__start_subtest(c"dup-sym-default".as_ptr()) {
        test_attach_probe_dup_sym(probe_attach_mode::PROBE_ATTACH_MODE_DEFAULT);
    }
    if test__start_subtest(c"dup-sym-legacy".as_ptr()) {
        test_attach_probe_dup_sym(probe_attach_mode::PROBE_ATTACH_MODE_LEGACY);
    }
    if test__start_subtest(c"dup-sym-perf".as_ptr()) {
        test_attach_probe_dup_sym(probe_attach_mode::PROBE_ATTACH_MODE_PERF);
    }
    if test__start_subtest(c"dup-sym-link".as_ptr()) {
        test_attach_probe_dup_sym(probe_attach_mode::PROBE_ATTACH_MODE_LINK);
    }

    if test__start_subtest(c"auto".as_ptr()) {
        test_attach_probe_auto(skel);
    }
    if test__start_subtest(c"kprobe-sleepable".as_ptr()) {
        test_kprobe_sleepable();
    }
    if test__start_subtest(c"uprobe-lib".as_ptr()) {
        test_uprobe_lib(skel);
    }
    if test__start_subtest(c"uprobe-sleepable".as_ptr()) {
        test_uprobe_sleepable(skel);
    }
    if test__start_subtest(c"uprobe-ref_ctr".as_ptr()) {
        test_uprobe_ref_ctr(skel);
    }

    if test__start_subtest(c"uprobe-long_name".as_ptr()) {
        test_attach_uprobe_long_event_name();
    }
    if test__start_subtest(c"kprobe-long_name".as_ptr()) {
        test_attach_kprobe_long_event_name();
    }
    if test__start_subtest(c"kprobe-write-ctx".as_ptr()) {
        test_attach_kprobe_write_ctx();
    }
    if test__start_subtest(c"freplace-kprobe-write-ctx".as_ptr()) {
        test_freplace_kprobe_write_ctx();
    }

    test_attach_probe__destroy(skel);
    ASSERT_EQ(core::ptr::read_volatile(core::ptr::addr_of!(uprobe_ref_ctr)) as c_long, 0, c"uprobe_ref_ctr_cleanup".as_ptr());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
