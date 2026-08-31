// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_char, c_int, c_void};
use core::mem::MaybeUninit;
use core::ptr;

// External dependencies corresponding to the C includes.
type size_t = usize;
type ssize_t = isize;
type __u64 = u64;

const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;
const TEST_FAIL: c_int = -1;
const TEMPL: &[u8] = b"/tmp/perf-test-XXXXXX\0";

#[repr(C)]
pub struct io {
    pub fd: c_int,
    pub buf: *mut c_void,
    pub buf_size: size_t,
    pub eof: bool,
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn mkstemp(template: *mut c_char) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn pr_debug(fmt: *const c_char, ...);
    fn zfree(ptr: *mut *mut c_void);
    fn io__init(io: *mut io, fd: c_int, buf: *mut c_void, buf_size: size_t);
    fn io__get_char(io: *mut io) -> c_int;
    fn io__get_hex(io: *mut io, hex: *mut __u64) -> c_int;
    fn io__get_dec(io: *mut io, dec: *mut __u64) -> c_int;
    fn io__getline(io: *mut io, line: *mut *mut c_char, line_len: *mut size_t) -> ssize_t;
}

macro_rules! expect_equal {
    ($ret:ident, $val:expr, $expected:expr) => {{
        let val = $val;
        let expected = $expected;
        if val != expected {
            unsafe {
                pr_debug(
                    b"%s:%d: %d != %d\n\0".as_ptr() as *const c_char,
                    concat!(file!(), "\0").as_ptr() as *const c_char,
                    line!() as c_int,
                    val as c_int,
                    expected as c_int,
                );
            }
            $ret = -1;
        }
    }};
}

macro_rules! expect_equal64 {
    ($ret:ident, $val:expr, $expected:expr) => {{
        let val = $val;
        let expected = $expected;
        if val != expected {
            unsafe {
                pr_debug(
                    b"%s:%d: %lld != %lld\n\0".as_ptr() as *const c_char,
                    concat!(file!(), "\0").as_ptr() as *const c_char,
                    line!() as c_int,
                    val as i64,
                    expected as i64,
                );
            }
            $ret = -1;
        }
    }};
}

unsafe fn make_test_file(path: *mut c_char, contents: *const c_char) -> c_int {
    let contents_len: ssize_t = strlen(contents) as ssize_t;
    let fd: c_int;

    strcpy(path, TEMPL.as_ptr() as *const c_char);
    fd = mkstemp(path);
    if fd < 0 {
        pr_debug(b"mkstemp failed\0".as_ptr() as *const c_char);
        return -1;
    }
    if write(fd, contents as *const c_void, contents_len as size_t) < contents_len {
        pr_debug(b"short write\0".as_ptr() as *const c_char);
        close(fd);
        unlink(path);
        return -1;
    }
    close(fd);
    0
}

unsafe fn setup_test(path: *mut c_char, contents: *const c_char, buf_size: size_t, io: *mut io) -> c_int {
    if make_test_file(path, contents) != 0 {
        return -1;
    }

    (*io).fd = open(path, O_RDONLY);
    if (*io).fd < 0 {
        pr_debug(b"Failed to open '%s'\n\0".as_ptr() as *const c_char, path);
        unlink(path);
        return -1;
    }
    (*io).buf = malloc(buf_size);
    if (*io).buf.is_null() {
        pr_debug(b"Failed to allocate memory\0".as_ptr() as *const c_char);
        close((*io).fd);
        unlink(path);
        return -1;
    }
    io__init(io, (*io).fd, (*io).buf, buf_size);
    0
}

unsafe fn cleanup_test(path: *mut c_char, io: *mut io) {
    zfree(ptr::addr_of_mut!((*io).buf));
    close((*io).fd);
    unlink(path);
}

unsafe fn do_test_get_char(test_string: *const c_char, buf_size: size_t) -> c_int {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut io = MaybeUninit::<io>::uninit();
    let mut ch: c_int;
    let mut ret: c_int = 0;
    let mut i: size_t;

    if setup_test(path.as_mut_ptr(), test_string, buf_size, io.as_mut_ptr()) != 0 {
        return -1;
    }
    let mut io = io.assume_init();

    i = 0;
    while i < strlen(test_string) {
        ch = io__get_char(&mut io);
        expect_equal!(ret, ch, *test_string.add(i) as c_int);
        expect_equal!(ret, io.eof, false);
        i += 1;
    }
    ch = io__get_char(&mut io);
    expect_equal!(ret, ch, -1);
    expect_equal!(ret, io.eof, true);

    cleanup_test(path.as_mut_ptr(), &mut io);
    ret
}

unsafe fn test_get_char() -> c_int {
    let mut i: c_int;
    let mut ret: c_int = 0;
    let mut j: size_t;
    static TEST_STRINGS: [&[u8]; 3] = [
        b"12345678abcdef90\0",
        b"a\nb\nc\nd\n\0",
        b"\x07\x08\t\x0b\x0c\r\0",
    ];

    i = 0;
    while i <= 10 {
        j = 0;
        while j < TEST_STRINGS.len() {
            if do_test_get_char(TEST_STRINGS[j].as_ptr() as *const c_char, (1usize) << i) != 0 {
                ret = -1;
            }
            j += 1;
        }
        i += 1;
    }
    ret
}

unsafe fn do_test_get_hex(
    test_string: *const c_char,
    val1: __u64,
    ch1: c_int,
    val2: __u64,
    ch2: c_int,
    val3: __u64,
    ch3: c_int,
    end_eof: bool,
) -> c_int {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut io = MaybeUninit::<io>::uninit();
    let mut ch: c_int;
    let mut ret: c_int = 0;
    let mut hex: __u64 = 0;

    if setup_test(path.as_mut_ptr(), test_string, 4, io.as_mut_ptr()) != 0 {
        return -1;
    }
    let mut io = io.assume_init();

    ch = io__get_hex(&mut io, &mut hex);
    expect_equal64!(ret, hex, val1);
    expect_equal!(ret, ch, ch1);

    ch = io__get_hex(&mut io, &mut hex);
    expect_equal64!(ret, hex, val2);
    expect_equal!(ret, ch, ch2);

    ch = io__get_hex(&mut io, &mut hex);
    expect_equal64!(ret, hex, val3);
    expect_equal!(ret, ch, ch3);

    expect_equal!(ret, io.eof, end_eof);

    cleanup_test(path.as_mut_ptr(), &mut io);
    ret
}

unsafe fn test_get_hex() -> c_int {
    let mut ret: c_int = 0;

    if do_test_get_hex(b"12345678abcdef90\0".as_ptr() as *const c_char, 0x12345678abcdef90, -1, 0, -1, 0, -1, true) != 0 {
        ret = -1;
    }
    if do_test_get_hex(b"1\n2\n3\n\0".as_ptr() as *const c_char, 1, '\n' as c_int, 2, '\n' as c_int, 3, '\n' as c_int, false) != 0 {
        ret = -1;
    }
    if do_test_get_hex(b"12345678ABCDEF90;a;b\0".as_ptr() as *const c_char, 0x12345678abcdef90, ';' as c_int, 0xa, ';' as c_int, 0xb, -1, true) != 0 {
        ret = -1;
    }
    if do_test_get_hex(b"0x1x2x\0".as_ptr() as *const c_char, 0, 'x' as c_int, 1, 'x' as c_int, 2, 'x' as c_int, false) != 0 {
        ret = -1;
    }
    if do_test_get_hex(b"x1x\0".as_ptr() as *const c_char, 0, -2, 1, 'x' as c_int, 0, -1, true) != 0 {
        ret = -1;
    }
    if do_test_get_hex(b"10000000000000000000000000000abcdefgh99i\0".as_ptr() as *const c_char, 0xabcdef, 'g' as c_int, 0, -2, 0x99, 'i' as c_int, false) != 0 {
        ret = -1;
    }

    ret
}

unsafe fn do_test_get_dec(
    test_string: *const c_char,
    val1: __u64,
    ch1: c_int,
    val2: __u64,
    ch2: c_int,
    val3: __u64,
    ch3: c_int,
    end_eof: bool,
) -> c_int {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut io = MaybeUninit::<io>::uninit();
    let mut ch: c_int;
    let mut ret: c_int = 0;
    let mut dec: __u64 = 0;

    if setup_test(path.as_mut_ptr(), test_string, 4, io.as_mut_ptr()) != 0 {
        return -1;
    }
    let mut io = io.assume_init();

    ch = io__get_dec(&mut io, &mut dec);
    expect_equal64!(ret, dec, val1);
    expect_equal!(ret, ch, ch1);

    ch = io__get_dec(&mut io, &mut dec);
    expect_equal64!(ret, dec, val2);
    expect_equal!(ret, ch, ch2);

    ch = io__get_dec(&mut io, &mut dec);
    expect_equal64!(ret, dec, val3);
    expect_equal!(ret, ch, ch3);

    expect_equal!(ret, io.eof, end_eof);

    cleanup_test(path.as_mut_ptr(), &mut io);
    ret
}

unsafe fn test_get_dec() -> c_int {
    let mut ret: c_int = 0;

    if do_test_get_dec(b"12345678abcdef90\0".as_ptr() as *const c_char, 12345678, 'a' as c_int, 0, -2, 0, -2, false) != 0 {
        ret = -1;
    }
    if do_test_get_dec(b"1\n2\n3\n\0".as_ptr() as *const c_char, 1, '\n' as c_int, 2, '\n' as c_int, 3, '\n' as c_int, false) != 0 {
        ret = -1;
    }
    if do_test_get_dec(b"12345678;1;2\0".as_ptr() as *const c_char, 12345678, ';' as c_int, 1, ';' as c_int, 2, -1, true) != 0 {
        ret = -1;
    }
    if do_test_get_dec(b"0x1x2x\0".as_ptr() as *const c_char, 0, 'x' as c_int, 1, 'x' as c_int, 2, 'x' as c_int, false) != 0 {
        ret = -1;
    }
    if do_test_get_dec(b"x1x\0".as_ptr() as *const c_char, 0, -2, 1, 'x' as c_int, 0, -1, true) != 0 {
        ret = -1;
    }
    if do_test_get_dec(b"10000000000000000000000000000000000000000000000000000000000123456789ab99c\0".as_ptr() as *const c_char, 123456789, 'a' as c_int, 0, -2, 99, 'c' as c_int, false) != 0 {
        ret = -1;
    }

    ret
}

unsafe fn test_get_line() -> c_int {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut io = MaybeUninit::<io>::uninit();
    let mut test_string: [c_char; 1024] = [0; 1024];
    let mut line: *mut c_char = ptr::null_mut();
    let mut i: size_t;
    let mut line_len: size_t = 0;
    let buf_size: size_t = 128;
    let mut ret: c_int = 0;

    i = 0;
    while i < 512 {
        test_string[i] = 'a' as c_char;
        i += 1;
    }
    test_string[512] = '\n' as c_char;
    i = 513;
    while i < 1023 {
        test_string[i] = 'b' as c_char;
        i += 1;
    }
    test_string[1023] = '\0' as c_char;

    if setup_test(path.as_mut_ptr(), test_string.as_ptr(), buf_size, io.as_mut_ptr()) != 0 {
        return -1;
    }
    let mut io = io.assume_init();

    expect_equal!(ret, io__getline(&mut io, &mut line, &mut line_len) as c_int, 513);
    expect_equal!(ret, strlen(line) as c_int, 513);
    i = 0;
    while i < 512 {
        expect_equal!(ret, *line.add(i) as c_int, 'a' as c_int);
        i += 1;
    }
    expect_equal!(ret, *line.add(512) as c_int, '\n' as c_int);
    expect_equal!(ret, io__getline(&mut io, &mut line, &mut line_len) as c_int, 510);
    i = 0;
    while i < 510 {
        expect_equal!(ret, *line.add(i) as c_int, 'b' as c_int);
        i += 1;
    }

    free(line as *mut c_void);
    cleanup_test(path.as_mut_ptr(), &mut io);
    ret
}

unsafe fn test__api_io(_test: *mut test_suite, _subtest: c_int) -> c_int {
    let mut ret: c_int = 0;

    if test_get_char() != 0 {
        ret = TEST_FAIL;
    }
    if test_get_hex() != 0 {
        ret = TEST_FAIL;
    }
    if test_get_dec() != 0 {
        ret = TEST_FAIL;
    }
    if test_get_line() != 0 {
        ret = TEST_FAIL;
    }
    ret
}

// C macro preserved in intent: DEFINE_SUITE("Test api io", api_io);
#[unsafe(no_mangle)]
pub static mut api_io: test_suite = test_suite { _private: [] };
