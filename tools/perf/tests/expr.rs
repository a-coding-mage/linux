// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/tests/expr.c. C include dependencies:
// "util/cputopo.h", "util/debug.h", "util/expr.h", "util/hashmap.h",
// "util/header.h", "util/smt.h", "tests.h", <perf/cpumap.h>, <math.h>,
// <stdlib.h>, <string.h>, <string2.h>, <linux/zalloc.h>.

use core::ffi::{c_char, c_double, c_int, c_void};

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct expr_id_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct expr_scanner_ctx {
    pub runtime: c_int,
}

#[repr(C)]
pub struct expr_parse_ctx {
    pub ids: *mut hashmap,
    pub sctx: expr_scanner_ctx,
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

const FP_ZERO: c_int = 2;

unsafe extern "C" {
    fn ids__new() -> *mut hashmap;
    fn ids__union(ids1: *mut hashmap, ids2: *mut hashmap) -> *mut hashmap;
    fn ids__insert(ids: *mut hashmap, id: *mut c_char) -> c_int;
    fn ids__free(ids: *mut hashmap);

    fn hashmap__size(map: *const hashmap) -> usize;
    fn hashmap__find(map: *const hashmap, key: *const c_char, value: *mut *mut expr_id_data) -> bool;

    fn expr__ctx_new() -> *mut expr_parse_ctx;
    fn expr__ctx_free(ctx: *mut expr_parse_ctx);
    fn expr__ctx_clear(ctx: *mut expr_parse_ctx);
    fn expr__add_id_val(ctx: *mut expr_parse_ctx, id: *mut c_char, val: c_double) -> c_int;
    fn expr__parse(val: *mut c_double, ctx: *mut expr_parse_ctx, expr: *const c_char) -> c_int;
    fn expr__find_ids(expr: *const c_char, one: *const c_char, ctx: *mut expr_parse_ctx) -> c_int;

    fn get_cpuid_allow_env_override(cpu: perf_cpu) -> *mut c_char;
    fn smt_on() -> bool;
    fn core_wide(system_wide: bool, user_requested_cpus: bool) -> bool;
    fn strreplace_chars(needle: c_char, haystack: *const c_char, replace: *const c_char) -> *mut c_char;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;

    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn fpclassify(x: c_double) -> c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! TEST_ASSERT_VAL {
    ($msg:literal, $cond:expr) => {
        if !$cond {
            return -1;
        }
    };
}

macro_rules! TEST_ASSERT_EQUAL {
    ($msg:literal, $left:expr, $right:expr) => {
        if $left != $right {
            return -1;
        }
    };
}

unsafe fn test_ids_union() -> c_int {
    let mut ids1: *mut hashmap;
    let mut ids2: *mut hashmap;

    /* Empty union. */
    ids1 = ids__new();
    TEST_ASSERT_VAL!("ids__new", !ids1.is_null());
    ids2 = ids__new();
    TEST_ASSERT_VAL!("ids__new", !ids2.is_null());

    ids1 = ids__union(ids1, ids2);
    TEST_ASSERT_EQUAL!("union", hashmap__size(ids1) as c_int, 0);

    /* Union {foo, bar} against {}. */
    ids2 = ids__new();
    TEST_ASSERT_VAL!("ids__new", !ids2.is_null());

    TEST_ASSERT_EQUAL!("ids__insert", ids__insert(ids1, strdup(cstr!("foo"))), 0);
    TEST_ASSERT_EQUAL!("ids__insert", ids__insert(ids1, strdup(cstr!("bar"))), 0);

    ids1 = ids__union(ids1, ids2);
    TEST_ASSERT_EQUAL!("union", hashmap__size(ids1) as c_int, 2);

    /* Union {foo, bar} against {foo}. */
    ids2 = ids__new();
    TEST_ASSERT_VAL!("ids__new", !ids2.is_null());
    TEST_ASSERT_EQUAL!("ids__insert", ids__insert(ids2, strdup(cstr!("foo"))), 0);

    ids1 = ids__union(ids1, ids2);
    TEST_ASSERT_EQUAL!("union", hashmap__size(ids1) as c_int, 2);

    /* Union {foo, bar} against {bar,baz}. */
    ids2 = ids__new();
    TEST_ASSERT_VAL!("ids__new", !ids2.is_null());
    TEST_ASSERT_EQUAL!("ids__insert", ids__insert(ids2, strdup(cstr!("bar"))), 0);
    TEST_ASSERT_EQUAL!("ids__insert", ids__insert(ids2, strdup(cstr!("baz"))), 0);

    ids1 = ids__union(ids1, ids2);
    TEST_ASSERT_EQUAL!("union", hashmap__size(ids1) as c_int, 3);

    ids__free(ids1);

    0
}

unsafe fn test(ctx: *mut expr_parse_ctx, e: *const c_char, val2: c_double) -> c_int {
    let mut val: c_double = 0.0;

    if expr__parse(&mut val, ctx, e) != 0 {
        TEST_ASSERT_VAL!("parse test failed", false);
    }
    TEST_ASSERT_VAL!("unexpected value", val == val2);
    0
}

unsafe fn test__expr(_t: *mut test_suite, _subtest: c_int) -> c_int {
    let mut val_ptr: *mut expr_id_data = core::ptr::null_mut();
    let mut p: *const c_char;
    let mut val: c_double = 0.0;
    let mut num_cpus_online: c_double = 0.0;
    let mut num_cpus: c_double = 0.0;
    let mut num_cores: c_double = 0.0;
    let mut num_dies: c_double = 0.0;
    let mut num_packages: c_double = 0.0;
    let mut ret: c_int;
    let ctx: *mut expr_parse_ctx;
    let mut strcmp_cpuid_buf = [0 as c_char; 256];
    let cpu = perf_cpu { cpu: -1 };
    let cpuid = get_cpuid_allow_env_override(cpu);
    let mut escaped_cpuid1: *mut c_char;
    let mut escaped_cpuid2: *mut c_char;

    TEST_ASSERT_VAL!("get_cpuid", !cpuid.is_null());

    TEST_ASSERT_EQUAL!("ids_union", test_ids_union(), 0);

    ctx = expr__ctx_new();
    TEST_ASSERT_VAL!("expr__ctx_new", !ctx.is_null());
    expr__add_id_val(ctx, strdup(cstr!("FOO")), 1.0);
    expr__add_id_val(ctx, strdup(cstr!("BAR")), 2.0);

    ret = test(ctx, cstr!("1+1"), 2.0);
    ret |= test(ctx, cstr!("FOO+BAR"), 3.0);
    ret |= test(ctx, cstr!("(BAR/2)%2"), 1.0);
    ret |= test(ctx, cstr!("1 - -4"), 5.0);
    ret |= test(ctx, cstr!("(FOO-1)*2 + (BAR/2)%2 - -4"), 5.0);
    ret |= test(ctx, cstr!("1-1 | 1"), 1.0);
    ret |= test(ctx, cstr!("1-1 & 1"), 0.0);
    ret |= test(ctx, cstr!("min(1,2) + 1"), 2.0);
    ret |= test(ctx, cstr!("max(1,2) + 1"), 3.0);
    ret |= test(ctx, cstr!("1+1 if 3*4 else 0"), 2.0);
    ret |= test(ctx, cstr!("100 if 1 else 200 if 1 else 300"), 100.0);
    ret |= test(ctx, cstr!("100 if 0 else 200 if 1 else 300"), 200.0);
    ret |= test(ctx, cstr!("100 if 1 else 200 if 0 else 300"), 100.0);
    ret |= test(ctx, cstr!("100 if 0 else 200 if 0 else 300"), 300.0);
    ret |= test(ctx, cstr!("1.1 + 2.1"), 3.2);
    ret |= test(ctx, cstr!(".1 + 2."), 2.1);
    ret |= test(ctx, cstr!("d_ratio(1, 2)"), 0.5);
    ret |= test(ctx, cstr!("d_ratio(2.5, 0)"), 0.0);
    ret |= test(ctx, cstr!("1.1 < 2.2"), 1.0);
    ret |= test(ctx, cstr!("2.2 > 1.1"), 1.0);
    ret |= test(ctx, cstr!("1.1 < 1.1"), 0.0);
    ret |= test(ctx, cstr!("2.2 > 2.2"), 0.0);
    ret |= test(ctx, cstr!("2.2 < 1.1"), 0.0);
    ret |= test(ctx, cstr!("1.1 > 2.2"), 0.0);
    ret |= test(ctx, cstr!("1.1e10 < 1.1e100"), 1.0);
    ret |= test(ctx, cstr!("1.1e2 > 1.1e-2"), 1.0);

    if ret != 0 {
        expr__ctx_free(ctx);
        return ret;
    }

    p = cstr!("FOO/0");
    ret = expr__parse(&mut val, ctx, p);
    TEST_ASSERT_VAL!("division by zero", ret == 0);
    TEST_ASSERT_VAL!("division by zero", val.is_nan());

    p = cstr!("BAR/");
    ret = expr__parse(&mut val, ctx, p);
    TEST_ASSERT_VAL!("missing operand", ret == -1);

    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!(
        "find ids",
        expr__find_ids(cstr!("FOO + BAR + BAZ + BOZO"), cstr!("FOO"), ctx) == 0
    );
    TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 3);
    TEST_ASSERT_VAL!("find ids", hashmap__find((*ctx).ids, cstr!("BAR"), &mut val_ptr));
    TEST_ASSERT_VAL!("find ids", hashmap__find((*ctx).ids, cstr!("BAZ"), &mut val_ptr));
    TEST_ASSERT_VAL!("find ids", hashmap__find((*ctx).ids, cstr!("BOZO"), &mut val_ptr));

    expr__ctx_clear(ctx);
    (*ctx).sctx.runtime = 3;
    TEST_ASSERT_VAL!(
        "find ids",
        expr__find_ids(cstr!("EVENT1\\,param\\=?@ + EVENT2\\,param\\=?@"), core::ptr::null(), ctx) == 0
    );
    TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 2);
    TEST_ASSERT_VAL!("find ids", hashmap__find((*ctx).ids, cstr!("EVENT1,param=3@"), &mut val_ptr));
    TEST_ASSERT_VAL!("find ids", hashmap__find((*ctx).ids, cstr!("EVENT2,param=3@"), &mut val_ptr));

    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!(
        "find ids",
        expr__find_ids(cstr!("dash\\-event1 - dash\\-event2"), core::ptr::null(), ctx) == 0
    );
    TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 2);
    TEST_ASSERT_VAL!("find ids", hashmap__find((*ctx).ids, cstr!("dash-event1"), &mut val_ptr));
    TEST_ASSERT_VAL!("find ids", hashmap__find((*ctx).ids, cstr!("dash-event2"), &mut val_ptr));

    /* Only EVENT1 or EVENT2 need be measured depending on the value of smt_on. */
    {
        let smton = smt_on();
        let corewide = core_wide(false, false);

        expr__ctx_clear(ctx);
        TEST_ASSERT_VAL!(
            "find ids",
            expr__find_ids(cstr!("EVENT1 if #smt_on else EVENT2"), core::ptr::null(), ctx) == 0
        );
        TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 1);
        TEST_ASSERT_VAL!(
            "find ids",
            hashmap__find((*ctx).ids, if smton { cstr!("EVENT1") } else { cstr!("EVENT2") }, &mut val_ptr)
        );

        expr__ctx_clear(ctx);
        TEST_ASSERT_VAL!(
            "find ids",
            expr__find_ids(cstr!("EVENT1 if #core_wide else EVENT2"), core::ptr::null(), ctx) == 0
        );
        TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 1);
        TEST_ASSERT_VAL!(
            "find ids",
            hashmap__find((*ctx).ids, if corewide { cstr!("EVENT1") } else { cstr!("EVENT2") }, &mut val_ptr)
        );
    }
    /* The expression is a constant 1.0 without needing to evaluate EVENT1. */
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!(
        "find ids",
        expr__find_ids(cstr!("1.0 if EVENT1 > 100.0 else 1.0"), core::ptr::null(), ctx) == 0
    );
    TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 0);

    /* The expression is a constant 0.0 without needing to evaluate EVENT1. */
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!("find ids", expr__find_ids(cstr!("0 & EVENT1 > 0"), core::ptr::null(), ctx) == 0);
    TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 0);
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!("find ids", expr__find_ids(cstr!("EVENT1 > 0 & 0"), core::ptr::null(), ctx) == 0);
    TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 0);
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!("find ids", expr__find_ids(cstr!("1 & EVENT1 > 0"), core::ptr::null(), ctx) == 0);
    TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 1);
    TEST_ASSERT_VAL!("find ids", hashmap__find((*ctx).ids, cstr!("EVENT1"), &mut val_ptr));
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!("find ids", expr__find_ids(cstr!("EVENT1 > 0 & 1"), core::ptr::null(), ctx) == 0);
    TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 1);
    TEST_ASSERT_VAL!("find ids", hashmap__find((*ctx).ids, cstr!("EVENT1"), &mut val_ptr));

    /* The expression is a constant 1.0 without needing to evaluate EVENT1. */
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!("find ids", expr__find_ids(cstr!("1 | EVENT1 > 0"), core::ptr::null(), ctx) == 0);
    TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 0);
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!("find ids", expr__find_ids(cstr!("EVENT1 > 0 | 1"), core::ptr::null(), ctx) == 0);
    TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 0);
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!("find ids", expr__find_ids(cstr!("0 | EVENT1 > 0"), core::ptr::null(), ctx) == 0);
    TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 1);
    TEST_ASSERT_VAL!("find ids", hashmap__find((*ctx).ids, cstr!("EVENT1"), &mut val_ptr));
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!("find ids", expr__find_ids(cstr!("EVENT1 > 0 | 0"), core::ptr::null(), ctx) == 0);
    TEST_ASSERT_VAL!("find ids", hashmap__size((*ctx).ids) == 1);
    TEST_ASSERT_VAL!("find ids", hashmap__find((*ctx).ids, cstr!("EVENT1"), &mut val_ptr));

    /* Test toplogy constants appear well ordered. */
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!(
        "#num_cpus_online",
        expr__parse(&mut num_cpus_online, ctx, cstr!("#num_cpus_online")) == 0
    );
    TEST_ASSERT_VAL!("#num_cpus", expr__parse(&mut num_cpus, ctx, cstr!("#num_cpus")) == 0);
    TEST_ASSERT_VAL!("#num_cpus >= #num_cpus_online", num_cpus >= num_cpus_online);
    TEST_ASSERT_VAL!("#num_cores", expr__parse(&mut num_cores, ctx, cstr!("#num_cores")) == 0);
    TEST_ASSERT_VAL!("#num_cpus >= #num_cores", num_cpus >= num_cores);
    TEST_ASSERT_VAL!("#num_dies", expr__parse(&mut num_dies, ctx, cstr!("#num_dies")) == 0);
    TEST_ASSERT_VAL!("#num_cores >= #num_dies", num_cores >= num_dies);
    TEST_ASSERT_VAL!("#num_packages", expr__parse(&mut num_packages, ctx, cstr!("#num_packages")) == 0);

    if num_dies != 0.0 {
        // Some platforms do not have CPU die support, for example s390
        TEST_ASSERT_VAL!("#num_dies >= #num_packages", num_dies >= num_packages);
    }

    if expr__parse(&mut val, ctx, cstr!("#system_tsc_freq")) == 0 {
        let is_intel = !strstr(cpuid, cstr!("Intel")).is_null();

        if is_intel {
            TEST_ASSERT_VAL!("#system_tsc_freq > 0", val > 0.0);
        } else {
            TEST_ASSERT_VAL!("#system_tsc_freq == 0", fpclassify(val) == FP_ZERO);
        }
    } else {
        // C condition preserved: #if defined(__i386__) || defined(__x86_64__)
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            TEST_ASSERT_VAL!("#system_tsc_freq unsupported", false);
        }
    }
    /*
     * Source count returns the number of events aggregating in a leader
     * event including the leader. Check parsing yields an id.
     */
    expr__ctx_clear(ctx);
    TEST_ASSERT_VAL!(
        "source count",
        expr__find_ids(cstr!("source_count(EVENT1)"), core::ptr::null(), ctx) == 0
    );
    TEST_ASSERT_VAL!("source count", hashmap__size((*ctx).ids) == 1);
    TEST_ASSERT_VAL!("source count", hashmap__find((*ctx).ids, cstr!("EVENT1"), &mut val_ptr));

    /* Test no cpuid match */
    ret = test(ctx, cstr!("strcmp_cpuid_str(0x0)"), 0.0);

    /*
     * Test cpuid match with current cpuid. Special chars have to be
     * escaped.
     */
    escaped_cpuid1 = strreplace_chars('-' as c_char, cpuid, cstr!("\\-"));
    free(cpuid as *mut c_void);
    escaped_cpuid2 = strreplace_chars(',' as c_char, escaped_cpuid1, cstr!("\\,"));
    free(escaped_cpuid1 as *mut c_void);
    escaped_cpuid1 = strreplace_chars('=' as c_char, escaped_cpuid2, cstr!("\\="));
    free(escaped_cpuid2 as *mut c_void);
    scnprintf(
        strcmp_cpuid_buf.as_mut_ptr(),
        strcmp_cpuid_buf.len(),
        cstr!("strcmp_cpuid_str(%s)"),
        escaped_cpuid1,
    );
    free(escaped_cpuid1 as *mut c_void);
    ret |= test(ctx, strcmp_cpuid_buf.as_ptr(), 1.0);

    /* has_event returns 1 when an event exists. */
    expr__add_id_val(ctx, strdup(cstr!("cycles")), 2.0);
    ret |= test(ctx, cstr!("has_event(cycles)"), 1.0);

    expr__ctx_free(ctx);

    ret
}

// C source registers this as: DEFINE_SUITE("Simple expression parser", expr);
