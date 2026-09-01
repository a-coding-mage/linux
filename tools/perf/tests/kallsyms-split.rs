// SPDX-License-Identifier: GPL-2.0
//
// Dependencies from the original C includes:
// linux/compiler.h, fcntl.h, signal.h, unistd.h, sys/stat.h,
// util/dso.h, util/map.h, util/symbol.h, util/debug.h, util/machine.h,
// tests.h

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

const O_RDWR: c_int = 0o2;
const O_CREAT: c_int = 0o100;

const SIGINT: c_int = 2;
const SIGPIPE: c_int = 13;
const SIGSEGV: c_int = 11;
const SIGTERM: c_int = 15;

const HOST_KERNEL_ID: c_int = 0;
const TEST_FAIL: c_int = -1;
const TEST_OK: c_int = 0;
const TEST_SKIP: c_int = 2;

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol_conf_t {
    pub ignore_vmlinux: bool,
    pub ignore_vmlinux_buildid: bool,
    pub allow_aliases: bool,
}

#[repr(C)]
struct proc_file {
    name: *const c_char,
    contents: *const c_char,
    len: c_long,
}

unsafe impl Sync for proc_file {}

unsafe extern "C" {
    fn mkdtemp(template: *mut c_char) -> *mut c_char;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, mode: c_uint) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: c_long) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
    fn scnprintf(buf: *mut c_char, size: c_long, fmt: *const c_char, ...) -> c_int;

    fn pr_debug(fmt: *const c_char, ...);

    fn machine__init(machine: *mut machine, root_dir: *const c_char, kernel_id: c_int) -> c_int;
    fn machine__create_kernel_maps(machine: *mut machine) -> c_int;
    fn machine__kernel_map(machine: *mut machine) -> *mut map;
    fn machine__kernel_maps(machine: *mut machine) -> *mut maps;
    fn machine__find_kernel_symbol_by_name(
        machine: *mut machine,
        name: *const c_char,
        mapp: *mut *mut map,
    ) -> *mut c_void;
    fn machine__exit(machine: *mut machine);

    fn map__load(map: *mut map) -> c_int;
    fn map__put(map: *mut map);
    fn maps__nr_maps(maps: *mut maps) -> c_uint;

    static mut symbol_conf: symbol_conf_t;
}

/*
 * This test is to check whether a bad symbol in a module won't split kallsyms maps.
 * The main_symbol[1-3] should belong to the main [kernel.kallsyms] map even if the
 * bad_symbol from the module is found in the middle.
 */
static mut root_template: [u8; 22] = *b"/tmp/perf-test.XXXXXX\0";
static mut root_dir: *mut c_char = ptr::null_mut();

static proc_version: &[u8] = b"Linux version X.Y.Z (just for perf test)\n\0";
static proc_modules: &[u8] = b"module 4096 1 - Live 0xffffffffcd000000\n\0";
static proc_kallsyms: &[u8] =
    b"ffffffffab200000 T _stext\n\
      ffffffffab200010 T good_symbol\n\
      ffffffffab200020 t bad_symbol\n\
      ffffffffab200030 t main_symbol1\n\
      ffffffffab200040 t main_symbol2\n\
      ffffffffab200050 t main_symbol3\n\
      ffffffffab200060 T _etext\n\
      ffffffffcd000000 T start_module\t[module]\n\
      ffffffffab200020 u bad_symbol\t[module]\n\
      ffffffffcd000040 T end_module\t[module]\n\0";

static proc_files: [proc_file; 3] = [
    proc_file {
        name: b"version\0".as_ptr() as *const c_char,
        contents: proc_version.as_ptr() as *const c_char,
        len: (proc_version.len() - 1) as c_long,
    },
    proc_file {
        name: b"modules\0".as_ptr() as *const c_char,
        contents: proc_modules.as_ptr() as *const c_char,
        len: (proc_modules.len() - 1) as c_long,
    },
    proc_file {
        name: b"kallsyms\0".as_ptr() as *const c_char,
        contents: proc_kallsyms.as_ptr() as *const c_char,
        len: (proc_kallsyms.len() - 1) as c_long,
    },
];

extern "C" fn remove_proc_dir(_sig: c_int) {
    let mut buf = [0 as c_char; 128];

    unsafe {
        if root_dir.is_null() {
            return;
        }

        for i in 0..proc_files.len() {
            scnprintf(
                buf.as_mut_ptr(),
                buf.len() as c_long,
                b"%s/proc/%s\0".as_ptr() as *const c_char,
                root_dir,
                proc_files[i].name,
            );
            remove(buf.as_ptr());
        }

        scnprintf(
            buf.as_mut_ptr(),
            buf.len() as c_long,
            b"%s/proc\0".as_ptr() as *const c_char,
            root_dir,
        );
        rmdir(buf.as_ptr());

        rmdir(root_dir);
        root_dir = ptr::null_mut();
    }
}

unsafe fn create_proc_dir() -> c_int {
    let mut buf = [0 as c_char; 128];

    root_dir = mkdtemp(root_template.as_mut_ptr() as *mut c_char);
    if root_dir.is_null() {
        return -1;
    }

    scnprintf(
        buf.as_mut_ptr(),
        buf.len() as c_long,
        b"%s/proc\0".as_ptr() as *const c_char,
        root_dir,
    );
    if mkdir(buf.as_ptr(), 0o700) < 0 {
        remove_proc_dir(0);
        return -1;
    }

    for i in 0..proc_files.len() {
        let fd: c_int;
        let len: c_long;

        scnprintf(
            buf.as_mut_ptr(),
            buf.len() as c_long,
            b"%s/proc/%s\0".as_ptr() as *const c_char,
            root_dir,
            proc_files[i].name,
        );
        fd = open(buf.as_ptr(), O_RDWR | O_CREAT, 0o600);
        if fd < 0 {
            remove_proc_dir(0);
            return -1;
        }

        len = write(
            fd,
            proc_files[i].contents as *const c_void,
            proc_files[i].len,
        );
        close(fd);
        if len != proc_files[i].len {
            remove_proc_dir(0);
            return -1;
        }
    }
    0
}

fn RC_CHK_EQUAL(a: *mut map, b: *mut map) -> bool {
    a == b
}

unsafe extern "C" fn test__kallsyms_split(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    /* struct machine m = { 0 }; */
    let mut m: machine = core::mem::zeroed();
    let mut map: *mut map = ptr::null_mut();
    let mut ret: c_int = TEST_FAIL;

    pr_debug(b"try to create fake root directory\n\0".as_ptr() as *const c_char);
    if create_proc_dir() < 0 {
        pr_debug(b"SKIP: cannot create a fake root directory\n\0".as_ptr() as *const c_char);
        return TEST_SKIP;
    }

    signal(SIGINT, remove_proc_dir);
    signal(SIGPIPE, remove_proc_dir);
    signal(SIGSEGV, remove_proc_dir);
    signal(SIGTERM, remove_proc_dir);

    pr_debug(b"create kernel maps from the fake root directory\n\0".as_ptr() as *const c_char);
    if machine__init(&mut m, root_dir, HOST_KERNEL_ID) != 0 {
        pr_debug(b"FAIL: failed to init machine\n\0".as_ptr() as *const c_char);
        goto_out(&mut map, &mut m, ret);
        return ret;
    }
    if machine__create_kernel_maps(&mut m) < 0 {
        pr_debug(b"FAIL: failed to create kernel maps\n\0".as_ptr() as *const c_char);
        goto_out(&mut map, &mut m, ret);
        return ret;
    }

    /* force to use /proc/kallsyms */
    symbol_conf.ignore_vmlinux = true;
    symbol_conf.ignore_vmlinux_buildid = true;
    symbol_conf.allow_aliases = true;

    if map__load(machine__kernel_map(&mut m)) < 0 {
        pr_debug(b"FAIL: failed to load kallsyms\n\0".as_ptr() as *const c_char);
        goto_out(&mut map, &mut m, ret);
        return ret;
    }

    pr_debug(b"kernel map loaded - check symbol and map\n\0".as_ptr() as *const c_char);
    if maps__nr_maps(machine__kernel_maps(&mut m)) != 2 {
        pr_debug(
            b"FAIL: it should have the kernel and a module, but has %u maps\n\0".as_ptr()
                as *const c_char,
            maps__nr_maps(machine__kernel_maps(&mut m)),
        );
        goto_out(&mut map, &mut m, ret);
        return ret;
    }

    if machine__find_kernel_symbol_by_name(
        &mut m,
        b"main_symbol3\0".as_ptr() as *const c_char,
        &mut map,
    )
    .is_null()
    {
        pr_debug(b"FAIL: failed to find a symbol\n\0".as_ptr() as *const c_char);
        goto_out(&mut map, &mut m, ret);
        return ret;
    }

    if !RC_CHK_EQUAL(map, machine__kernel_map(&mut m)) {
        pr_debug(b"FAIL: the symbol is not in the kernel map\n\0".as_ptr() as *const c_char);
        goto_out(&mut map, &mut m, ret);
        return ret;
    }
    ret = TEST_OK;

    goto_out(&mut map, &mut m, ret);
    ret
}

unsafe fn goto_out(map: *mut *mut map, m: *mut machine, _ret: c_int) {
    map__put(*map);
    remove_proc_dir(0);
    machine__exit(m);
}

/* DEFINE_SUITE("split kallsyms", kallsyms_split); */
#[used]
static kallsyms_split: unsafe extern "C" fn(*mut test_suite, c_int) -> c_int =
    test__kallsyms_split;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
