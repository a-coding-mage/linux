// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Test the powerpc alignment handler on POWER8/POWER9
 *
 * Copyright (C) 2017 IBM Corporation (Michael Neuling, Andrew Donnellan)
 */

/*
 * This selftest exercises the powerpc alignment fault handler.
 *
 * We create two sets of source and destination buffers, one in regular memory,
 * the other cache-inhibited (by default we use /dev/fb0 for this, but an
 * alterative path for cache-inhibited memory may be provided, e.g. memtrace).
 *
 * We initialise the source buffers, then use whichever set of load/store
 * instructions is under test to copy bytes from the source buffers to the
 * destination buffers. For the regular buffers, these instructions will
 * execute normally. For the cache-inhibited buffers, these instructions
 * will trap and cause an alignment fault, and the alignment fault handler
 * will emulate the particular instruction under test. We then compare the
 * destination buffers to ensure that the native and emulated cases give the
 * same result.
 *
 * TODO:
 *   - Any FIXMEs below
 *   - Test VSX regs < 32 and > 32
 *   - Test all loads and stores
 *   - Check update forms do update register
 *   - Test alignment faults over page boundary
 *
 * Some old binutils may not support all the instructions.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type bool_t = bool;
type size_t = usize;
type siginfo_t = c_void;
type sigset_t = c_ulong;
type sighandler_t = unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void);

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: sighandler_t,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
}

#[repr(C)]
pub struct mcontext_t {
    pub gp_regs: [c_ulong; 64],
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_mcontext: mcontext_t,
}

const O_RDWR: c_int = 0x0002;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;
const SA_SIGINFO: c_int = 4;
const SIG_DFL: usize = 0;
const SIGSEGV: c_int = 11;
const SIGBUS: c_int = 7;
const SIGILL: c_int = 4;
const PT_NIP: usize = 32;

extern "C" {
    static mut optind: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn posix_memalign(memptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn signal(signum: c_int, handler: usize) -> usize;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn getopt(argc: c_int, argv: *mut *mut c_char, optstring: *const c_char) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn getpagesize() -> c_int;
    fn exit(status: c_int) -> !;

    fn have_hwcap(feature: c_ulong) -> bool_t;
    fn have_hwcap2(feature: c_ulong) -> bool_t;
    fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

extern "C" {
    static PPC_FEATURE_ARCH_2_05: c_ulong;
    static PPC_FEATURE_ARCH_2_06: c_ulong;
    static PPC_FEATURE_HAS_ALTIVEC: c_ulong;
    static PPC_FEATURE2_ARCH_2_07: c_ulong;
    static PPC_FEATURE2_ARCH_3_00: c_ulong;
    static PPC_FEATURE2_ARCH_3_1: c_ulong;
}

static mut bufsize: c_int = 0;
static mut debug: c_int = 0;
static mut testing: c_int = 0;
static mut gotsig: c_int = 0;
static mut prefixes_enabled: bool = false;
static mut cipath: *mut c_char = b"/dev/fb0\0".as_ptr() as *mut c_char;
static mut cioffset: c_long = 0;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 0;
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn sighandler(sig: c_int, _info: *mut siginfo_t, ctx: *mut c_void) {
    let ucp = ctx as *mut ucontext_t;

    if testing == 0 {
        signal(sig, SIG_DFL);
        kill(0, sig);
    }
    gotsig = sig;

    #[cfg(target_arch = "powerpc64")]
    {
        if prefixes_enabled {
            let nip = (*ucp).uc_mcontext.gp_regs[PT_NIP] as *const u32;
            let inst = ptr::read(nip);
            (*ucp).uc_mcontext.gp_regs[PT_NIP] += if inst >> 26 == 1 { 8 } else { 4 };
        } else {
            (*ucp).uc_mcontext.gp_regs[PT_NIP] += 4;
        }
    }

    #[cfg(not(target_arch = "powerpc64"))]
    {
        (*ucp).uc_mcontext.gp_regs[PT_NIP] += 4;
    }
}

type test_func_t = unsafe extern "C" fn(*mut c_char, *mut c_char);

macro_rules! TEST {
    ($name:ident, $ld_op:literal, $st_op:literal, XFORM, $ld_reg:literal, $st_reg:literal) => {{
        unsafe extern "C" fn $name(s: *mut c_char, d: *mut c_char) {
            unsafe {
                asm!(
                    concat!($ld_op, " ", $ld_reg, " ,{0},{2} ;", $st_op, " ", $st_reg, " ,{1},{2} ;"),
                    in(reg) s,
                    in(reg) d,
                    in(reg) 0usize,
                    lateout("r31") _,
                    lateout("vs0") _,
                    lateout("vs32") _,
                    options(nostack)
                );
            }
        }
        rc |= unsafe { do_test(cstr!(stringify!($name)), $name) };
    }};
    ($name:ident, $ld_op:literal, $st_op:literal, DFORM, $ld_reg:literal, $st_reg:literal) => {{
        unsafe extern "C" fn $name(s: *mut c_char, d: *mut c_char) {
            unsafe {
                asm!(
                    concat!($ld_op, " ", $ld_reg, " ,0({0}) ;", $st_op, " ", $st_reg, " ,0({1}) ;"),
                    in(reg) s,
                    in(reg) d,
                    lateout("r31") _,
                    lateout("vs0") _,
                    lateout("vs32") _,
                    options(nostack)
                );
            }
        }
        rc |= unsafe { do_test(cstr!(stringify!($name)), $name) };
    }};
}

macro_rules! TESTP {
    ($name:ident, $ld_op:ident, $st_op:ident, $ld_reg:literal, $st_reg:literal) => {{
        unsafe extern "C" fn $name(s: *mut c_char, d: *mut c_char) {
            unsafe {
                asm!(
                    concat!(
                        stringify!($ld_op), " ", $ld_reg, ", {0}, 0, 0 ;",
                        stringify!($st_op), " ", $st_reg, ", {1}, 0, 0 ;"
                    ),
                    in(reg) s,
                    in(reg) d,
                    lateout("r31") _,
                    lateout("vs0") _,
                    lateout("vs32") _,
                    options(nostack)
                );
            }
        }
        rc |= unsafe { do_test(cstr!(stringify!($name)), $name) };
    }};
}

macro_rules! LOAD_VSX_XFORM_TEST { ($op:ident) => { TEST!($op, stringify!($op), "stxvd2x", XFORM, "32", "32") }; }
macro_rules! STORE_VSX_XFORM_TEST { ($op:ident) => { TEST!($op, "lxvd2x", stringify!($op), XFORM, "32", "32") }; }
macro_rules! LOAD_VSX_DFORM_TEST { ($op:ident) => { TEST!($op, stringify!($op), "stxv", DFORM, "32", "32") }; }
macro_rules! STORE_VSX_DFORM_TEST { ($op:ident) => { TEST!($op, "lxv", stringify!($op), DFORM, "32", "32") }; }
macro_rules! LOAD_VMX_XFORM_TEST { ($op:ident) => { TEST!($op, stringify!($op), "stxvd2x", XFORM, "0", "32") }; }
macro_rules! STORE_VMX_XFORM_TEST { ($op:ident) => { TEST!($op, "lxvd2x", stringify!($op), XFORM, "32", "0") }; }
macro_rules! LOAD_VMX_DFORM_TEST { ($op:ident) => { TEST!($op, stringify!($op), "stxv", DFORM, "0", "32") }; }
macro_rules! STORE_VMX_DFORM_TEST { ($op:ident) => { TEST!($op, "lxv", stringify!($op), DFORM, "32", "0") }; }
macro_rules! LOAD_XFORM_TEST { ($op:ident) => { TEST!($op, stringify!($op), "stdx", XFORM, "31", "31") }; }
macro_rules! STORE_XFORM_TEST { ($op:ident) => { TEST!($op, "ldx", stringify!($op), XFORM, "31", "31") }; }
macro_rules! LOAD_DFORM_TEST { ($op:ident) => { TEST!($op, stringify!($op), "std", DFORM, "31", "31") }; }
macro_rules! STORE_DFORM_TEST { ($op:ident) => { TEST!($op, "ld", stringify!($op), DFORM, "31", "31") }; }
macro_rules! LOAD_FLOAT_DFORM_TEST { ($op:ident) => { TEST!($op, stringify!($op), "stfd", DFORM, "0", "0") }; }
macro_rules! STORE_FLOAT_DFORM_TEST { ($op:ident) => { TEST!($op, "lfd", stringify!($op), DFORM, "0", "0") }; }
macro_rules! LOAD_FLOAT_XFORM_TEST { ($op:ident) => { TEST!($op, stringify!($op), "stfdx", XFORM, "0", "0") }; }
macro_rules! STORE_FLOAT_XFORM_TEST { ($op:ident) => { TEST!($op, "lfdx", stringify!($op), XFORM, "0", "0") }; }
macro_rules! LOAD_MLS_PREFIX_TEST { ($op:ident) => { TESTP!($op, $op, PSTD, 31, 31) }; }
macro_rules! STORE_MLS_PREFIX_TEST { ($op:ident) => { TESTP!($op, PLD, $op, 31, 31) }; }
macro_rules! LOAD_8LS_PREFIX_TEST { ($op:ident) => { TESTP!($op, $op, PSTD, 31, 31) }; }
macro_rules! STORE_8LS_PREFIX_TEST { ($op:ident) => { TESTP!($op, PLD, $op, 31, 31) }; }
macro_rules! LOAD_FLOAT_MLS_PREFIX_TEST { ($op:ident) => { TESTP!($op, $op, PSTFD, 0, 0) }; }
macro_rules! STORE_FLOAT_MLS_PREFIX_TEST { ($op:ident) => { TESTP!($op, PLFD, $op, 0, 0) }; }
macro_rules! LOAD_VSX_8LS_PREFIX_TEST { ($op:ident, 0) => { TESTP!($op, $op, PSTXV0, 0, 32) }; ($op:ident, 1) => { TESTP!($op, $op, PSTXV1, 0, 32) }; }
macro_rules! STORE_VSX_8LS_PREFIX_TEST { ($op:ident, 0) => { TESTP!($op, PLXV0, $op, 32, 0) }; ($op:ident, 1) => { TESTP!($op, PLXV1, $op, 32, 0) }; }

/* FIXME: Unimplemented tests: */
// STORE_DFORM_TEST(stq)   /* FIXME: need two registers for quad */
// STORE_DFORM_TEST(stswi) /* FIXME: string instruction */

// STORE_XFORM_TEST(stwat) /* AMO can't emulate or run on CI */
// STORE_XFORM_TEST(stdat) /* ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ */

/* preload byte by byte */
#[no_mangle]
pub unsafe extern "C" fn preload_data(dst: *mut c_void, offset: c_int, width: c_int) {
    let mut c = dst as *mut c_char;
    let mut i: c_int;

    c = c.add(offset as usize);

    i = 0;
    while i < width {
        *c.add(i as usize) = i as c_char;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_memcpy(
    dst: *mut c_void,
    src: *mut c_void,
    size: c_int,
    offset: c_int,
    test_func: test_func_t,
) -> c_int {
    let mut s: *mut c_char;
    let mut d: *mut c_char;

    s = src as *mut c_char;
    s = s.add(offset as usize);
    d = dst as *mut c_char;
    d = d.add(offset as usize);

    assert!(size == 16);
    gotsig = 0;
    testing = 1;

    test_func(s, d); /* run the actual test */

    testing = 0;
    if gotsig != 0 {
        if debug != 0 {
            printf(cstr!("  Got signal %i\n"), gotsig);
        }
        return 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn dumpdata(s1: *mut c_char, s2: *mut c_char, n: c_int, test_name: *mut c_char) {
    let mut i: c_int;

    printf(cstr!("  %s: unexpected result:\n"), test_name);
    printf(cstr!("    mem:"));
    i = 0;
    while i < n {
        printf(cstr!(" %02x"), *s1.add(i as usize) as c_int);
        i += 1;
    }
    printf(cstr!("\n"));
    printf(cstr!("    ci: "));
    i = 0;
    while i < n {
        printf(cstr!(" %02x"), *s2.add(i as usize) as c_int);
        i += 1;
    }
    printf(cstr!("\n"));
}

#[no_mangle]
pub unsafe extern "C" fn test_memcmp(
    s1: *mut c_void,
    s2: *mut c_void,
    n: c_int,
    offset: c_int,
    test_name: *mut c_char,
) -> c_int {
    let mut s1c: *mut c_char;
    let mut s2c: *mut c_char;

    s1c = s1 as *mut c_char;
    s1c = s1c.add(offset as usize);
    s2c = s2 as *mut c_char;
    s2c = s2c.add(offset as usize);

    if memcmp(s1c as *const c_void, s2c as *const c_void, n as size_t) != 0 {
        if debug != 0 {
            printf(cstr!("\n  Compare failed. Offset:%i length:%i\n"), offset, n);
            dumpdata(s1c, s2c, n, test_name);
        }
        return 1;
    }
    0
}

/*
 * Do two memcpy tests using the same instructions. One cachable
 * memory and the other doesn't.
 */
#[no_mangle]
pub unsafe extern "C" fn do_test(test_name: *mut c_char, test_func: test_func_t) -> c_int {
    let mut offset: c_int;
    let mut width: c_int;
    let fd: c_int;
    let mut rc: c_int;
    let mut r: c_int;
    let mut mem0: *mut c_void = ptr::null_mut();
    let mut mem1: *mut c_void = ptr::null_mut();
    let ci0: *mut c_void;
    let ci1: *mut c_void;

    printf(cstr!("\tDoing %s:\t"), test_name);

    fd = open(cipath, O_RDWR);
    if fd < 0 {
        printf(cstr!("\n"));
        perror(cstr!("Can't open ci file now?"));
        return 1;
    }

    ci0 = mmap(ptr::null_mut(), bufsize as size_t, PROT_WRITE | PROT_READ, MAP_SHARED, fd, cioffset);
    ci1 = mmap(
        ptr::null_mut(),
        bufsize as size_t,
        PROT_WRITE | PROT_READ,
        MAP_SHARED,
        fd,
        cioffset + bufsize as c_long,
    );

    if ci0 == MAP_FAILED || ci1 == MAP_FAILED {
        printf(cstr!("\n"));
        perror(cstr!("mmap failed"));
        SKIP_IF!(true);
    }

    rc = posix_memalign(&mut mem0, bufsize as size_t, bufsize as size_t);
    if rc != 0 {
        printf(cstr!("\n"));
        return rc;
    }

    rc = posix_memalign(&mut mem1, bufsize as size_t, bufsize as size_t);
    if rc != 0 {
        printf(cstr!("\n"));
        free(mem0);
        return rc;
    }

    rc = 0;
    /*
     * offset = 0 is aligned but tests the workaround for the P9N
     * DD2.1 vector CI load issue (see 5080332c2c89 "powerpc/64s:
     * Add workaround for P9 vector CI load issue")
     */
    offset = 0;
    while offset < 16 {
        width = 16; /* vsx == 16 bytes */
        r = 0;

        /* load pattern into memory byte by byte */
        preload_data(ci0, offset, width);
        preload_data(mem0, offset, width); // FIXME: remove??
        memcpy(ci0, mem0, bufsize as size_t);
        memcpy(ci1, mem1, bufsize as size_t); /* initialise output to the same */

        /* sanity check */
        test_memcmp(mem0, ci0, width, offset, test_name);

        r |= test_memcpy(ci1, ci0, width, offset, test_func);
        r |= test_memcpy(mem1, mem0, width, offset, test_func);
        if r != 0 && debug == 0 {
            printf(cstr!("FAILED: Got signal"));
            rc = 1;
            break;
        }

        r |= test_memcmp(mem1, ci1, width, offset, test_name);
        if r != 0 && debug == 0 {
            printf(cstr!("FAILED: Wrong Data"));
            rc = 1;
            break;
        }

        offset += 1;
    }

    if rc == 0 {
        printf(cstr!("PASSED"));
    }

    printf(cstr!("\n"));

    munmap(ci0, bufsize as size_t);
    munmap(ci1, bufsize as size_t);
    free(mem0);
    free(mem1);
    close(fd);

    rc
}

unsafe extern "C" fn can_open_cifile() -> bool {
    let fd: c_int;

    fd = open(cipath, O_RDWR);
    if fd < 0 {
        return false;
    }

    close(fd);
    true
}

#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_vsx_206() -> c_int {
    let mut rc: c_int = 0;

    SKIP_IF!(!can_open_cifile());
    SKIP_IF!(!have_hwcap(PPC_FEATURE_ARCH_2_06));

    printf(cstr!("VSX: 2.06B\n"));
    LOAD_VSX_XFORM_TEST!(lxvd2x);
    LOAD_VSX_XFORM_TEST!(lxvw4x);
    LOAD_VSX_XFORM_TEST!(lxsdx);
    LOAD_VSX_XFORM_TEST!(lxvdsx);
    STORE_VSX_XFORM_TEST!(stxvd2x);
    STORE_VSX_XFORM_TEST!(stxvw4x);
    STORE_VSX_XFORM_TEST!(stxsdx);
    rc
}

#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_vsx_207() -> c_int {
    let mut rc: c_int = 0;

    SKIP_IF!(!can_open_cifile());
    SKIP_IF!(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));

    printf(cstr!("VSX: 2.07B\n"));
    LOAD_VSX_XFORM_TEST!(lxsspx);
    LOAD_VSX_XFORM_TEST!(lxsiwax);
    LOAD_VSX_XFORM_TEST!(lxsiwzx);
    STORE_VSX_XFORM_TEST!(stxsspx);
    STORE_VSX_XFORM_TEST!(stxsiwx);
    rc
}

#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_vsx_300() -> c_int {
    let mut rc: c_int = 0;

    SKIP_IF!(!can_open_cifile());

    SKIP_IF!(!have_hwcap2(PPC_FEATURE2_ARCH_3_00));
    printf(cstr!("VSX: 3.00B\n"));
    LOAD_VMX_DFORM_TEST!(lxsd);
    LOAD_VSX_XFORM_TEST!(lxsibzx);
    LOAD_VSX_XFORM_TEST!(lxsihzx);
    LOAD_VMX_DFORM_TEST!(lxssp);
    LOAD_VSX_DFORM_TEST!(lxv);
    LOAD_VSX_XFORM_TEST!(lxvb16x);
    LOAD_VSX_XFORM_TEST!(lxvh8x);
    LOAD_VSX_XFORM_TEST!(lxvx);
    LOAD_VSX_XFORM_TEST!(lxvwsx);
    LOAD_VSX_XFORM_TEST!(lxvl);
    LOAD_VSX_XFORM_TEST!(lxvll);
    STORE_VMX_DFORM_TEST!(stxsd);
    STORE_VSX_XFORM_TEST!(stxsibx);
    STORE_VSX_XFORM_TEST!(stxsihx);
    STORE_VMX_DFORM_TEST!(stxssp);
    STORE_VSX_DFORM_TEST!(stxv);
    STORE_VSX_XFORM_TEST!(stxvb16x);
    STORE_VSX_XFORM_TEST!(stxvh8x);
    STORE_VSX_XFORM_TEST!(stxvx);
    STORE_VSX_XFORM_TEST!(stxvl);
    STORE_VSX_XFORM_TEST!(stxvll);
    rc
}

#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_vsx_prefix() -> c_int {
    let mut rc: c_int = 0;

    SKIP_IF!(!can_open_cifile());
    SKIP_IF!(!have_hwcap2(PPC_FEATURE2_ARCH_3_1));

    printf(cstr!("VSX: PREFIX\n"));
    LOAD_VSX_8LS_PREFIX_TEST!(PLXSD, 0);
    LOAD_VSX_8LS_PREFIX_TEST!(PLXSSP, 0);
    LOAD_VSX_8LS_PREFIX_TEST!(PLXV0, 0);
    LOAD_VSX_8LS_PREFIX_TEST!(PLXV1, 1);
    STORE_VSX_8LS_PREFIX_TEST!(PSTXSD, 0);
    STORE_VSX_8LS_PREFIX_TEST!(PSTXSSP, 0);
    STORE_VSX_8LS_PREFIX_TEST!(PSTXV0, 0);
    STORE_VSX_8LS_PREFIX_TEST!(PSTXV1, 1);
    rc
}

#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_integer() -> c_int {
    let mut rc: c_int = 0;

    SKIP_IF!(!can_open_cifile());

    printf(cstr!("Integer\n"));
    LOAD_DFORM_TEST!(lbz);
    LOAD_DFORM_TEST!(lbzu);
    LOAD_XFORM_TEST!(lbzx);
    LOAD_XFORM_TEST!(lbzux);
    LOAD_DFORM_TEST!(lhz);
    LOAD_DFORM_TEST!(lhzu);
    LOAD_XFORM_TEST!(lhzx);
    LOAD_XFORM_TEST!(lhzux);
    LOAD_DFORM_TEST!(lha);
    LOAD_DFORM_TEST!(lhau);
    LOAD_XFORM_TEST!(lhax);
    LOAD_XFORM_TEST!(lhaux);
    LOAD_XFORM_TEST!(lhbrx);
    LOAD_DFORM_TEST!(lwz);
    LOAD_DFORM_TEST!(lwzu);
    LOAD_XFORM_TEST!(lwzx);
    LOAD_XFORM_TEST!(lwzux);
    LOAD_DFORM_TEST!(lwa);
    LOAD_XFORM_TEST!(lwax);
    LOAD_XFORM_TEST!(lwaux);
    LOAD_XFORM_TEST!(lwbrx);
    LOAD_DFORM_TEST!(ld);
    LOAD_DFORM_TEST!(ldu);
    LOAD_XFORM_TEST!(ldx);
    LOAD_XFORM_TEST!(ldux);
    STORE_DFORM_TEST!(stb);
    STORE_XFORM_TEST!(stbx);
    STORE_DFORM_TEST!(stbu);
    STORE_XFORM_TEST!(stbux);
    STORE_DFORM_TEST!(sth);
    STORE_XFORM_TEST!(sthx);
    STORE_DFORM_TEST!(sthu);
    STORE_XFORM_TEST!(sthux);
    STORE_XFORM_TEST!(sthbrx);
    STORE_DFORM_TEST!(stw);
    STORE_XFORM_TEST!(stwx);
    STORE_DFORM_TEST!(stwu);
    STORE_XFORM_TEST!(stwux);
    STORE_XFORM_TEST!(stwbrx);
    STORE_DFORM_TEST!(std);
    STORE_XFORM_TEST!(stdx);
    STORE_DFORM_TEST!(stdu);
    STORE_XFORM_TEST!(stdux);

    #[cfg(target_endian = "big")]
    {
        LOAD_DFORM_TEST!(lmw);
        STORE_DFORM_TEST!(stmw);
    }

    rc
}

#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_integer_206() -> c_int {
    let mut rc: c_int = 0;

    SKIP_IF!(!can_open_cifile());
    SKIP_IF!(!have_hwcap(PPC_FEATURE_ARCH_2_06));

    printf(cstr!("Integer: 2.06\n"));

    LOAD_XFORM_TEST!(ldbrx);
    STORE_XFORM_TEST!(stdbrx);

    rc
}

#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_integer_prefix() -> c_int {
    let mut rc: c_int = 0;

    SKIP_IF!(!can_open_cifile());
    SKIP_IF!(!have_hwcap2(PPC_FEATURE2_ARCH_3_1));

    printf(cstr!("Integer: PREFIX\n"));
    LOAD_MLS_PREFIX_TEST!(PLBZ);
    LOAD_MLS_PREFIX_TEST!(PLHZ);
    LOAD_MLS_PREFIX_TEST!(PLHA);
    LOAD_MLS_PREFIX_TEST!(PLWZ);
    LOAD_8LS_PREFIX_TEST!(PLWA);
    LOAD_8LS_PREFIX_TEST!(PLD);
    STORE_MLS_PREFIX_TEST!(PSTB);
    STORE_MLS_PREFIX_TEST!(PSTH);
    STORE_MLS_PREFIX_TEST!(PSTW);
    STORE_8LS_PREFIX_TEST!(PSTD);
    rc
}

#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_vmx() -> c_int {
    let mut rc: c_int = 0;

    SKIP_IF!(!can_open_cifile());
    SKIP_IF!(!have_hwcap(PPC_FEATURE_HAS_ALTIVEC));

    printf(cstr!("VMX\n"));
    LOAD_VMX_XFORM_TEST!(lvx);

    /*
     * FIXME: These loads only load part of the register, so our
     * testing method doesn't work. Also they don't take alignment
     * faults, so it's kinda pointless anyway
     *
     LOAD_VMX_XFORM_TEST(lvebx)
     LOAD_VMX_XFORM_TEST(lvehx)
     LOAD_VMX_XFORM_TEST(lvewx)
     LOAD_VMX_XFORM_TEST(lvxl)
    */
    STORE_VMX_XFORM_TEST!(stvx);
    STORE_VMX_XFORM_TEST!(stvebx);
    STORE_VMX_XFORM_TEST!(stvehx);
    STORE_VMX_XFORM_TEST!(stvewx);
    STORE_VMX_XFORM_TEST!(stvxl);
    rc
}

#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_fp() -> c_int {
    let mut rc: c_int = 0;

    SKIP_IF!(!can_open_cifile());

    printf(cstr!("Floating point\n"));
    LOAD_FLOAT_DFORM_TEST!(lfd);
    LOAD_FLOAT_XFORM_TEST!(lfdx);
    LOAD_FLOAT_DFORM_TEST!(lfdu);
    LOAD_FLOAT_XFORM_TEST!(lfdux);
    LOAD_FLOAT_DFORM_TEST!(lfs);
    LOAD_FLOAT_XFORM_TEST!(lfsx);
    LOAD_FLOAT_DFORM_TEST!(lfsu);
    LOAD_FLOAT_XFORM_TEST!(lfsux);
    STORE_FLOAT_DFORM_TEST!(stfd);
    STORE_FLOAT_XFORM_TEST!(stfdx);
    STORE_FLOAT_DFORM_TEST!(stfdu);
    STORE_FLOAT_XFORM_TEST!(stfdux);
    STORE_FLOAT_DFORM_TEST!(stfs);
    STORE_FLOAT_XFORM_TEST!(stfsx);
    STORE_FLOAT_DFORM_TEST!(stfsu);
    STORE_FLOAT_XFORM_TEST!(stfsux);
    STORE_FLOAT_XFORM_TEST!(stfiwx);

    rc
}

#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_fp_205() -> c_int {
    let mut rc: c_int = 0;

    SKIP_IF!(!can_open_cifile());
    SKIP_IF!(!have_hwcap(PPC_FEATURE_ARCH_2_05));

    printf(cstr!("Floating point: 2.05\n"));

    LOAD_FLOAT_DFORM_TEST!(lfdp);
    LOAD_FLOAT_XFORM_TEST!(lfdpx);
    LOAD_FLOAT_XFORM_TEST!(lfiwax);
    STORE_FLOAT_DFORM_TEST!(stfdp);
    STORE_FLOAT_XFORM_TEST!(stfdpx);

    rc
}

#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_fp_206() -> c_int {
    let mut rc: c_int = 0;

    SKIP_IF!(!can_open_cifile());
    SKIP_IF!(!have_hwcap(PPC_FEATURE_ARCH_2_06));

    printf(cstr!("Floating point: 2.06\n"));

    LOAD_FLOAT_XFORM_TEST!(lfiwzx);

    rc
}

#[no_mangle]
pub unsafe extern "C" fn test_alignment_handler_fp_prefix() -> c_int {
    let mut rc: c_int = 0;

    SKIP_IF!(!can_open_cifile());
    SKIP_IF!(!have_hwcap2(PPC_FEATURE2_ARCH_3_1));

    printf(cstr!("Floating point: PREFIX\n"));
    LOAD_FLOAT_DFORM_TEST!(lfs);
    LOAD_FLOAT_MLS_PREFIX_TEST!(PLFS);
    LOAD_FLOAT_MLS_PREFIX_TEST!(PLFD);
    STORE_FLOAT_MLS_PREFIX_TEST!(PSTFS);
    STORE_FLOAT_MLS_PREFIX_TEST!(PSTFD);
    rc
}

#[no_mangle]
pub unsafe extern "C" fn usage(prog: *mut c_char) {
    printf(cstr!("Usage: %s [options] [path [offset]]\n"), prog);
    printf(cstr!("  -d\tEnable debug error output\n"));
    printf(cstr!("\n"));
    printf(cstr!("This test requires a POWER8, POWER9 or POWER10 CPU "));
    printf(cstr!("and either a usable framebuffer at /dev/fb0 or "));
    printf(cstr!("the path to usable cache inhibited memory and optional "));
    printf(cstr!("offset to be provided\n"));
}

#[no_mangle]
pub unsafe extern "C" fn main(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut sa: sigaction = core::mem::zeroed();
    let mut rc: c_int = 0;
    let mut option: c_int = 0;

    loop {
        option = getopt(argc, argv, cstr!("d"));
        if option == -1 {
            break;
        }
        match option {
            x if x == b'd' as c_int => {
                debug += 1;
            }
            _ => {
                usage(*argv);
                exit(1);
            }
        }
    }
    argc -= optind;
    argv = argv.add(optind as usize);

    if argc > 0 {
        cipath = *argv;
    }
    if argc > 1 {
        cioffset = strtol(*argv.add(1), ptr::null_mut(), 0x10);
    }

    bufsize = getpagesize();

    sa.sa_sigaction = sighandler;
    sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = SA_SIGINFO;
    if sigaction(SIGSEGV, &sa, ptr::null_mut()) == -1
        || sigaction(SIGBUS, &sa, ptr::null_mut()) == -1
        || sigaction(SIGILL, &sa, ptr::null_mut()) == -1
    {
        perror(cstr!("sigaction"));
        exit(1);
    }

    prefixes_enabled = have_hwcap2(PPC_FEATURE2_ARCH_3_1);

    rc |= test_harness(test_alignment_handler_vsx_206, cstr!("test_alignment_handler_vsx_206"));
    rc |= test_harness(test_alignment_handler_vsx_207, cstr!("test_alignment_handler_vsx_207"));
    rc |= test_harness(test_alignment_handler_vsx_300, cstr!("test_alignment_handler_vsx_300"));
    rc |= test_harness(test_alignment_handler_vsx_prefix, cstr!("test_alignment_handler_vsx_prefix"));
    rc |= test_harness(test_alignment_handler_integer, cstr!("test_alignment_handler_integer"));
    rc |= test_harness(test_alignment_handler_integer_206, cstr!("test_alignment_handler_integer_206"));
    rc |= test_harness(test_alignment_handler_integer_prefix, cstr!("test_alignment_handler_integer_prefix"));
    rc |= test_harness(test_alignment_handler_vmx, cstr!("test_alignment_handler_vmx"));
    rc |= test_harness(test_alignment_handler_fp, cstr!("test_alignment_handler_fp"));
    rc |= test_harness(test_alignment_handler_fp_205, cstr!("test_alignment_handler_fp_205"));
    rc |= test_harness(test_alignment_handler_fp_206, cstr!("test_alignment_handler_fp_206"));
    rc |= test_harness(test_alignment_handler_fp_prefix, cstr!("test_alignment_handler_fp_prefix"));
    rc
}
