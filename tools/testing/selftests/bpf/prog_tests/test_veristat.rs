// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */
/* Translated from testing/selftests/bpf/prog_tests/test_veristat.c. */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type ssize_t = isize;

const F_OK: c_int = 0;
const FILTER_OBJS: &str = "veristat_foo.bpf.o veristat_bar.bpf.o";

#[repr(C)]
struct fixture {
    tmpfile: [c_char; 80],
    fd: c_int,
    output: *mut c_char,
    sz: size_t,
    veristat: [c_char; 80],
}

#[repr(C)]
struct name_filter_case {
    filters: *const c_char,
    file: *const c_char,
    prog: *const c_char,
    included: c_int,
}

unsafe extern "C" {
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn snprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: i64) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn strlen(s: *const c_char) -> size_t;
    fn syncfs(fd: c_int) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn strscpy(dst: *mut c_char, src: *const c_char) -> ssize_t;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_HAS_SUBSTR(str_: *const c_char, substr: *const c_char, name: *const c_char) -> bool;
    fn ASSERT_GE(a: ssize_t, b: ssize_t, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_FALSE(condition: bool, name: *const c_char) -> bool;
    fn ASSERT_NEQ(a: c_int, b: c_int, name: *const c_char) -> bool;
    fn PRINT_FAIL(fmt: *const c_char, ...);
}

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! __CHECK_STR {
    ($fix:expr, $str_:literal, $name:literal) => {
        if !ASSERT_HAS_SUBSTR((*$fix).output, c!($str_), c!($name)) {
            break 'out;
        }
    };
}

unsafe fn run_sys(cmd: *const c_char) -> c_int {
    system(cmd)
}

unsafe fn init_fixture() -> *mut fixture {
    let fix = malloc(size_of::<fixture>() as size_t) as *mut fixture;

    /* for no_alu32 and cpuv4 veristat is in parent folder */
    if access(c!("./veristat"), F_OK) == 0 {
        strscpy((*fix).veristat.as_mut_ptr(), c!("./veristat"));
    } else if access(c!("../veristat"), F_OK) == 0 {
        strscpy((*fix).veristat.as_mut_ptr(), c!("../veristat"));
    } else {
        PRINT_FAIL(c!("Can't find veristat binary"));
    }

    snprintf(
        (*fix).tmpfile.as_mut_ptr(),
        (*fix).tmpfile.len(),
        c!("/tmp/test_veristat.XXXXXX"),
    );
    (*fix).fd = mkstemp((*fix).tmpfile.as_mut_ptr());
    (*fix).sz = 1000000;
    (*fix).output = malloc((*fix).sz) as *mut c_char;
    fix
}

unsafe fn read_output(fix: *mut fixture) {
    let len = pread(
        (*fix).fd,
        (*fix).output as *mut c_void,
        (*fix).sz - 1,
        0,
    );

    *(*fix).output.offset(if len < 0 { 0 } else { len }) = 0;
    ASSERT_GE(len, 0, c!("pread"));
}

unsafe fn teardown_fixture(fix: *mut fixture) {
    free((*fix).output as *mut c_void);
    close((*fix).fd);
    remove((*fix).tmpfile.as_ptr());
    free(fix as *mut c_void);
}

unsafe fn test_set_global_vars_succeeds() {
    let fix = init_fixture();

    'out: loop {
        let mut cmd = [0 as c_char; 4096];
        snprintf(
            cmd.as_mut_ptr(),
            cmd.len(),
            c!("%s set_global_vars.bpf.o -G \"var_s64 = 0xf000000000000001\"  -G \"var_u64 = 0xfedcba9876543210\"  -G \"var_s32 = -0x80000000\"  -G \"var_u32 = 0x76543210\"  -G \"var_s16 = -32768\"  -G \"var_u16 = 60652\"  -G \"var_s8 = -128\"  -G \"var_u8 = 255\"  -G \"var_ea = EA2\"  -G \"var_eb  =  EB2\"  -G \"var_ec=EC2\"  -G \"var_b = 1\"  -G \"struct1[2].struct2[1][2].u.var_u8[2]=170\"  -G \"union1.struct3.var_u8_l = 0xaa\"  -G \"union1.struct3.var_u8_h = 0xaa\"  -G \"arr[3]= 171\"  -G \"arr[EA2] =172\"  -G \"enum_arr[EC2]=EA3\"  -G \"three_d[31][7][EA2]=173\" -G \"struct1[2].struct2[1][2].u.mat[5][3]=174\"  -G \"struct11 [ 7 ] [ 5 ] .struct2[0][1].u.mat[3][0] = 175\"  -vl2 > %s"),
            (*fix).veristat.as_ptr(),
            (*fix).tmpfile.as_ptr(),
        );
        run_sys(cmd.as_ptr());

        read_output(fix);
        __CHECK_STR!(fix, "=0xf000000000000001 ", "var_s64 = 0xf000000000000001");
        __CHECK_STR!(fix, "=0xfedcba9876543210 ", "var_u64 = 0xfedcba9876543210");
        __CHECK_STR!(fix, "=0x80000000 ", "var_s32 = -0x80000000");
        __CHECK_STR!(fix, "=0x76543210 ", "var_u32 = 0x76543210");
        __CHECK_STR!(fix, "=0x8000 ", "var_s16 = -32768");
        __CHECK_STR!(fix, "=0xecec ", "var_u16 = 60652");
        __CHECK_STR!(fix, "=128 ", "var_s8 = -128");
        __CHECK_STR!(fix, "=255 ", "var_u8 = 255");
        __CHECK_STR!(fix, "=11 ", "var_ea = EA2");
        __CHECK_STR!(fix, "=12 ", "var_eb = EB2");
        __CHECK_STR!(fix, "=13 ", "var_ec = EC2");
        __CHECK_STR!(fix, "=1 ", "var_b = 1");
        __CHECK_STR!(fix, "=170 ", "struct1[2].struct2[1][2].u.var_u8[2]=170");
        __CHECK_STR!(fix, "=0xaaaa ", "union1.var_u16 = 0xaaaa");
        __CHECK_STR!(fix, "=171 ", "arr[3]= 171");
        __CHECK_STR!(fix, "=172 ", "arr[EA2] =172");
        __CHECK_STR!(fix, "=10 ", "enum_arr[EC2]=EA3");
        __CHECK_STR!(fix, "=173 ", "matrix[31][7][11]=173");
        __CHECK_STR!(fix, "=174 ", "struct1[2].struct2[1][2].u.mat[5][3]=174");
        __CHECK_STR!(fix, "=175 ", "struct11[7][5].struct2[0][1].u.mat[3][0]=175");
        break;
    }

    teardown_fixture(fix);
}

unsafe fn test_set_global_vars_from_file_succeeds() {
    let fix = init_fixture();
    let mut input_file = [0 as c_char; 80];
    let vars = c!("var_s16 = -32768\nvar_u16 = 60652");
    let mut fd: c_int = 0;

    'out: loop {
        snprintf(
            input_file.as_mut_ptr(),
            input_file.len(),
            c!("/tmp/veristat_input.XXXXXX"),
        );
        fd = mkstemp(input_file.as_mut_ptr());
        if !ASSERT_GE(fd as ssize_t, 0, c!("valid fd")) {
            break 'out;
        }

        write(fd, vars as *const c_void, strlen(vars));
        syncfs(fd);
        let mut cmd = [0 as c_char; 512];
        snprintf(
            cmd.as_mut_ptr(),
            cmd.len(),
            c!("%s set_global_vars.bpf.o -G \"@%s\" -vl2 > %s"),
            (*fix).veristat.as_ptr(),
            input_file.as_ptr(),
            (*fix).tmpfile.as_ptr(),
        );
        run_sys(cmd.as_ptr());
        read_output(fix);
        __CHECK_STR!(fix, "=0x8000 ", "var_s16 = -32768");
        __CHECK_STR!(fix, "=0xecec ", "var_u16 = 60652");
        break;
    }

    close(fd);
    remove(input_file.as_ptr());
    teardown_fixture(fix);
}

unsafe fn test_set_global_vars_out_of_range() {
    let fix = init_fixture();

    'out: loop {
        let mut cmd = [0 as c_char; 512];
        snprintf(
            cmd.as_mut_ptr(),
            cmd.len(),
            c!("%s set_global_vars.bpf.o -G \"var_s32 = 2147483648\" -vl2 2> %s"),
            (*fix).veristat.as_ptr(),
            (*fix).tmpfile.as_ptr(),
        );
        if run_sys(cmd.as_ptr()) == 0 {
            break 'out;
        }

        read_output(fix);
        __CHECK_STR!(fix, "is out of range [-2147483648; 2147483647]", "out of range");
        break;
    }

    teardown_fixture(fix);
}

unsafe fn test_unsupported_ptr_array_type() {
    let fix = init_fixture();

    'out: loop {
        let mut cmd = [0 as c_char; 512];
        snprintf(
            cmd.as_mut_ptr(),
            cmd.len(),
            c!("%s set_global_vars.bpf.o -G \"ptr_arr[0] = 0\" -vl2 2> %s"),
            (*fix).veristat.as_ptr(),
            (*fix).tmpfile.as_ptr(),
        );
        if run_sys(cmd.as_ptr()) == 0 {
            break 'out;
        }

        read_output(fix);
        __CHECK_STR!(fix, "Can't set ptr_arr[0]. Only ints and enums are supported", "ptr_arr");
        break;
    }

    teardown_fixture(fix);
}

unsafe fn test_array_out_of_bounds() {
    let fix = init_fixture();

    'out: loop {
        let mut cmd = [0 as c_char; 512];
        snprintf(
            cmd.as_mut_ptr(),
            cmd.len(),
            c!("%s set_global_vars.bpf.o -G \"arr[99] = 0\" -vl2 2> %s"),
            (*fix).veristat.as_ptr(),
            (*fix).tmpfile.as_ptr(),
        );
        if run_sys(cmd.as_ptr()) == 0 {
            break 'out;
        }

        read_output(fix);
        __CHECK_STR!(fix, "Array index 99 is out of bounds", "arr[99]");
        break;
    }

    teardown_fixture(fix);
}

unsafe fn test_array_index_not_found() {
    let fix = init_fixture();

    'out: loop {
        let mut cmd = [0 as c_char; 512];
        snprintf(
            cmd.as_mut_ptr(),
            cmd.len(),
            c!("%s set_global_vars.bpf.o -G \"arr[EG2] = 0\" -vl2 2> %s"),
            (*fix).veristat.as_ptr(),
            (*fix).tmpfile.as_ptr(),
        );
        if run_sys(cmd.as_ptr()) == 0 {
            break 'out;
        }

        read_output(fix);
        __CHECK_STR!(fix, "Can't resolve enum value EG2", "arr[EG2]");
        break;
    }

    teardown_fixture(fix);
}

unsafe fn test_array_index_for_non_array() {
    let fix = init_fixture();

    'out: loop {
        let mut cmd = [0 as c_char; 512];
        snprintf(
            cmd.as_mut_ptr(),
            cmd.len(),
            c!("%s set_global_vars.bpf.o -G \"var_b[0] = 1\" -vl2 2> %s"),
            (*fix).veristat.as_ptr(),
            (*fix).tmpfile.as_ptr(),
        );
        if run_sys(cmd.as_ptr()) == 0 {
            break 'out;
        }

        pread((*fix).fd, (*fix).output as *mut c_void, (*fix).sz, 0);
        __CHECK_STR!(fix, "Array index is not expected for var_b", "var_b[0] = 1");

        snprintf(
            cmd.as_mut_ptr(),
            cmd.len(),
            c!("%s set_global_vars.bpf.o -G \"union1.struct3[0].var_u8_l=1\" -vl2 2> %s"),
            (*fix).veristat.as_ptr(),
            (*fix).tmpfile.as_ptr(),
        );
        if run_sys(cmd.as_ptr()) == 0 {
            break 'out;
        }

        pread((*fix).fd, (*fix).output as *mut c_void, (*fix).sz, 0);
        __CHECK_STR!(
            fix,
            "Array index is not expected for struct3",
            "union1.struct3[0].var_u8_l=1"
        );
        break;
    }

    teardown_fixture(fix);
}

unsafe fn test_no_array_index_for_array() {
    let fix = init_fixture();

    'out: loop {
        let mut cmd = [0 as c_char; 512];
        snprintf(
            cmd.as_mut_ptr(),
            cmd.len(),
            c!("%s set_global_vars.bpf.o -G \"arr = 1\" -vl2 2> %s"),
            (*fix).veristat.as_ptr(),
            (*fix).tmpfile.as_ptr(),
        );
        if run_sys(cmd.as_ptr()) == 0 {
            break 'out;
        }

        pread((*fix).fd, (*fix).output as *mut c_void, (*fix).sz, 0);
        __CHECK_STR!(fix, "Can't set arr. Only ints and enums are supported", "arr = 1");

        snprintf(
            cmd.as_mut_ptr(),
            cmd.len(),
            c!("%s set_global_vars.bpf.o -G \"struct1[0].struct2.u.var_u8[2]=1\" -vl2 2> %s"),
            (*fix).veristat.as_ptr(),
            (*fix).tmpfile.as_ptr(),
        );
        if run_sys(cmd.as_ptr()) == 0 {
            break 'out;
        }

        pread((*fix).fd, (*fix).output as *mut c_void, (*fix).sz, 0);
        __CHECK_STR!(
            fix,
            "Can't resolve field u for non-composite type",
            "struct1[0].struct2.u.var_u8[2]=1"
        );
        break;
    }

    teardown_fixture(fix);
}

/*
 * Name filter tests below run veristat on veristat_foo.bpf.o and
 * veristat_bar.bpf.o, both defining programs 'foo', 'bar' and 'buz'.
 * Every entry describes a single (filters, file, prog) combination and
 * tells whether that program is expected in the veristat output:
 * 'true' if it is, 'false' if it is not and -1 if veristat is expected
 * to reject the filter.
 */
static NAME_FILTER_CASES: [name_filter_case; 33] = [
    /* no filters, every program is processed */
    name_filter_case { filters: c!(""), file: c!("foo"), prog: c!("foo"), included: true as c_int },
    name_filter_case { filters: c!(""), file: c!("foo"), prog: c!("bar"), included: true as c_int },
    name_filter_case { filters: c!(""), file: c!("foo"), prog: c!("buz"), included: true as c_int },
    name_filter_case { filters: c!(""), file: c!("bar"), prog: c!("foo"), included: true as c_int },
    name_filter_case { filters: c!(""), file: c!("bar"), prog: c!("bar"), included: true as c_int },
    name_filter_case { filters: c!(""), file: c!("bar"), prog: c!("buz"), included: true as c_int },
    /* deny filters */
    name_filter_case { filters: c!("-f '!*foo*'"), file: c!("foo"), prog: c!("bar"), included: false as c_int },
    name_filter_case { filters: c!("-f '!*foo*'"), file: c!("bar"), prog: c!("foo"), included: false as c_int },
    name_filter_case { filters: c!("-f '!*foo*'"), file: c!("bar"), prog: c!("bar"), included: true as c_int },
    name_filter_case { filters: c!("-f '!*foo*/bar'"), file: c!("foo"), prog: c!("bar"), included: false as c_int },
    name_filter_case { filters: c!("-f '!*foo*/bar'"), file: c!("foo"), prog: c!("buz"), included: true as c_int },
    name_filter_case { filters: c!("-f '!*foo*/bar'"), file: c!("bar"), prog: c!("bar"), included: true as c_int },
    name_filter_case { filters: c!("-f '!*foo*/'"), file: c!("foo"), prog: c!("bar"), included: false as c_int },
    name_filter_case { filters: c!("-f '!*foo*/'"), file: c!("bar"), prog: c!("bar"), included: true as c_int },
    name_filter_case { filters: c!("-f '!/bar'"), file: c!("foo"), prog: c!("bar"), included: false as c_int },
    name_filter_case { filters: c!("-f '!/bar'"), file: c!("foo"), prog: c!("foo"), included: true as c_int },
    name_filter_case { filters: c!("-f '!/'"), file: c!("foo"), prog: c!("bar"), included: -1 },
    name_filter_case { filters: c!("-f '!'"), file: c!("foo"), prog: c!("bar"), included: -1 },
    /* allow filters */
    name_filter_case { filters: c!("-f '*foo*'"), file: c!("foo"), prog: c!("bar"), included: true as c_int },
    name_filter_case { filters: c!("-f '*foo*'"), file: c!("bar"), prog: c!("foo"), included: true as c_int },
    name_filter_case { filters: c!("-f '*foo*'"), file: c!("bar"), prog: c!("bar"), included: false as c_int },
    name_filter_case { filters: c!("-f '*foo*/bar'"), file: c!("foo"), prog: c!("bar"), included: true as c_int },
    name_filter_case { filters: c!("-f '*foo*/bar'"), file: c!("foo"), prog: c!("buz"), included: false as c_int },
    name_filter_case { filters: c!("-f '*foo*/bar'"), file: c!("bar"), prog: c!("bar"), included: false as c_int },
    name_filter_case { filters: c!("-f '*foo*/'"), file: c!("foo"), prog: c!("bar"), included: true as c_int },
    name_filter_case { filters: c!("-f '*foo*/'"), file: c!("bar"), prog: c!("bar"), included: false as c_int },
    name_filter_case { filters: c!("-f '/bar'"), file: c!("foo"), prog: c!("bar"), included: true as c_int },
    name_filter_case { filters: c!("-f '/bar'"), file: c!("foo"), prog: c!("foo"), included: false as c_int },
    name_filter_case { filters: c!("-f '/'"), file: c!("foo"), prog: c!("bar"), included: -1 },
    name_filter_case { filters: c!("-f ''"), file: c!("foo"), prog: c!("bar"), included: -1 },
    /* allow and deny filters combined */
    name_filter_case { filters: c!("-f '*foo*/' -f '!/bar'"), file: c!("foo"), prog: c!("foo"), included: true as c_int },
    name_filter_case { filters: c!("-f '*foo*/' -f '!/bar'"), file: c!("foo"), prog: c!("bar"), included: false as c_int },
    name_filter_case { filters: c!("-f '*foo*/' -f '!/bar'"), file: c!("bar"), prog: c!("foo"), included: false as c_int },
];

unsafe fn test_name_filters() {
    let fix = init_fixture();
    let mut cmd = [0 as c_char; 512];
    let mut row = [0 as c_char; 64];
    let mut name = [0 as c_char; 128];
    let mut err: c_int;

    for i in 0..NAME_FILTER_CASES.len() {
        let t = &NAME_FILTER_CASES[i];
        /* stderr is merged with stdout in order to catch error messages */
        snprintf(
            cmd.as_mut_ptr(),
            cmd.len(),
            c!("%s veristat_foo.bpf.o veristat_bar.bpf.o -q -o csv -e file,prog %s > %s 2>&1"),
            (*fix).veristat.as_ptr(),
            t.filters,
            (*fix).tmpfile.as_ptr(),
        );
        err = system(cmd.as_ptr());
        read_output(fix);

        snprintf(row.as_mut_ptr(), row.len(), c!("veristat_%s.bpf.o,%s"), t.file, t.prog);
        snprintf(name.as_mut_ptr(), name.len(), c!("veristat %s: %s"), t.filters, row.as_ptr());
        match t.included {
            x if x == true as c_int => {
                ASSERT_OK(err, name.as_ptr());
                ASSERT_HAS_SUBSTR((*fix).output, row.as_ptr(), name.as_ptr());
            }
            x if x == false as c_int => {
                ASSERT_OK(err, name.as_ptr());
                ASSERT_FALSE(!strstr((*fix).output, row.as_ptr()).is_null(), name.as_ptr());
            }
            -1 => {
                ASSERT_NEQ(err, 0, name.as_ptr());
                ASSERT_HAS_SUBSTR((*fix).output, c!("Invalid filter"), name.as_ptr());
            }
            _ => {}
        }
    }

    teardown_fixture(fix);
}

#[no_mangle]
pub unsafe extern "C" fn test_veristat() {
    if test__start_subtest(c!("set_global_vars_succeeds")) {
        test_set_global_vars_succeeds();
    }

    if test__start_subtest(c!("set_global_vars_out_of_range")) {
        test_set_global_vars_out_of_range();
    }

    if test__start_subtest(c!("set_global_vars_from_file_succeeds")) {
        test_set_global_vars_from_file_succeeds();
    }

    if test__start_subtest(c!("test_unsupported_ptr_array_type")) {
        test_unsupported_ptr_array_type();
    }

    if test__start_subtest(c!("test_array_out_of_bounds")) {
        test_array_out_of_bounds();
    }

    if test__start_subtest(c!("test_array_index_not_found")) {
        test_array_index_not_found();
    }

    if test__start_subtest(c!("test_array_index_for_non_array")) {
        test_array_index_for_non_array();
    }

    if test__start_subtest(c!("test_no_array_index_for_array")) {
        test_no_array_index_for_array();
    }

    if test__start_subtest(c!("name_filters")) {
        test_name_filters();
    }
}
