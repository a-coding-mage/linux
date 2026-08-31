// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/arch/common.c. C include dependencies are represented
// below as external declarations or local constants where this file uses them.

use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_uint, c_ushort, c_void};
use std::ptr;

const PATH_MAX: usize = 4096;
const F_OK: c_int = 0;

const EM_386: u16 = 3;
const EM_MIPS: u16 = 8;
const EM_PPC: u16 = 20;
const EM_SH: u16 = 42;
const EM_SPARC: u16 = 2;
const EM_S390: u16 = 22;
const EM_ARM: u16 = 40;
const EM_X86_64: u16 = 62;
const EM_AARCH64: u16 = 183;
const EM_RISCV: u16 = 243;
const EM_ARC: u16 = 45;
const EM_PPC64: u16 = 21;
const EM_SPARCV9: u16 = 43;

// Supplied by the target build, as in dwarf-regs.h/common.h in the C source.
extern "C" {
    static EM_HOST: c_ushort;
}

#[repr(C)]
pub struct perf_env {
    pub arch: *mut c_char,
}

extern "C" {
    fn getenv(name: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn free(ptr: *mut c_void);
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;

    fn perf_env__kernel_is_64_bit(env: *mut perf_env) -> bool;
    fn perf_env__e_machine(env: *mut perf_env, e_flags: *mut c_uint) -> c_ushort;
    fn perf_env__arch(env: *mut perf_env) -> *const c_char;
    fn ui__error(fmt: *const c_char, ...);
}

unsafe fn zfree(ptr_: *mut *mut c_char) {
    if !(*ptr_).is_null() {
        free(*ptr_ as *mut c_void);
        *ptr_ = ptr::null_mut();
    }
}

static ARC_TRIPLETS: [&CStr; 3] = [
    unsafe { CStr::from_bytes_with_nul_unchecked(b"arc-linux-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"arc-snps-linux-uclibc-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"arc-snps-linux-gnu-\0") },
];

static ARM_TRIPLETS: [&CStr; 8] = [
    unsafe { CStr::from_bytes_with_nul_unchecked(b"arm-eabi-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"arm-linux-androideabi-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"arm-unknown-linux-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"arm-unknown-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"arm-unknown-linux-gnueabi-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"arm-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"arm-linux-gnueabihf-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"arm-none-eabi-\0") },
];

static ARM64_TRIPLETS: [&CStr; 2] = [
    unsafe { CStr::from_bytes_with_nul_unchecked(b"aarch64-linux-android-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"aarch64-linux-gnu-\0") },
];

static POWERPC_TRIPLETS: [&CStr; 5] = [
    unsafe { CStr::from_bytes_with_nul_unchecked(b"powerpc-unknown-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"powerpc-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"powerpc64-unknown-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"powerpc64-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"powerpc64le-linux-gnu-\0") },
];

static RISCV32_TRIPLETS: [&CStr; 3] = [
    unsafe { CStr::from_bytes_with_nul_unchecked(b"riscv32-unknown-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"riscv32-linux-android-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"riscv32-linux-gnu-\0") },
];

static RISCV64_TRIPLETS: [&CStr; 3] = [
    unsafe { CStr::from_bytes_with_nul_unchecked(b"riscv64-unknown-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"riscv64-linux-android-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"riscv64-linux-gnu-\0") },
];

static S390_TRIPLETS: [&CStr; 2] = [
    unsafe { CStr::from_bytes_with_nul_unchecked(b"s390-ibm-linux-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"s390x-linux-gnu-\0") },
];

static SH_TRIPLETS: [&CStr; 2] = [
    unsafe { CStr::from_bytes_with_nul_unchecked(b"sh-unknown-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"sh-linux-gnu-\0") },
];

static SPARC_TRIPLETS: [&CStr; 3] = [
    unsafe { CStr::from_bytes_with_nul_unchecked(b"sparc-unknown-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"sparc64-unknown-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"sparc64-linux-gnu-\0") },
];

static X86_TRIPLETS: [&CStr; 10] = [
    unsafe { CStr::from_bytes_with_nul_unchecked(b"x86_64-pc-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"x86_64-unknown-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"i686-pc-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"i586-pc-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"i486-pc-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"i386-pc-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"i686-linux-android-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"i686-android-linux-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"x86_64-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"i586-linux-gnu-\0") },
];

static MIPS_TRIPLETS: [&CStr; 7] = [
    unsafe { CStr::from_bytes_with_nul_unchecked(b"mips-unknown-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"mipsel-linux-android-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"mips-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"mips64-linux-gnu-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"mips64el-linux-gnuabi64-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"mips64-linux-gnuabi64-\0") },
    unsafe { CStr::from_bytes_with_nul_unchecked(b"mipsel-linux-gnu-\0") },
];

unsafe fn lookup_path(name: *mut c_char) -> bool {
    let mut found = false;
    let mut tmp: *mut c_char = ptr::null_mut();
    let mut buf = [0 as c_char; PATH_MAX];
    let mut env = getenv(c"PATH".as_ptr());

    if env.is_null() {
        return false;
    }

    env = strdup(env);
    if env.is_null() {
        return false;
    }

    let mut path = strtok_r(env, c":".as_ptr(), &mut tmp);
    while !path.is_null() {
        scnprintf(
            buf.as_mut_ptr(),
            buf.len(),
            c"%s/%s".as_ptr(),
            path,
            name,
        );
        if access(buf.as_ptr(), F_OK) == 0 {
            found = true;
            break;
        }
        path = strtok_r(ptr::null_mut(), c":".as_ptr(), &mut tmp);
    }
    free(env as *mut c_void);
    found
}

unsafe fn lookup_triplets(triplets: &[&CStr], name: *const c_char) -> c_int {
    let mut buf = [0 as c_char; PATH_MAX];

    for (i, triplet) in triplets.iter().enumerate() {
        scnprintf(
            buf.as_mut_ptr(),
            buf.len(),
            c"%s%s".as_ptr(),
            triplet.as_ptr(),
            name,
        );
        if lookup_path(buf.as_mut_ptr()) {
            return i as c_int;
        }
    }
    -1
}

unsafe fn is_native_compatible(env: *mut perf_env, target: u16, host: u16) -> bool {
    if target != host {
        /* A 64-bit host can natively disassemble its 32-bit compat architecture */
        if host == EM_X86_64 && target == EM_386 {
            return true;
        }
        if host == EM_AARCH64 && target == EM_ARM {
            return true;
        }
        if host == EM_PPC64 && target == EM_PPC {
            return true;
        }
        if host == EM_SPARCV9 && target == EM_SPARC {
            return true;
        }
        return false;
    }

    /* target == host case */
    if target == EM_RISCV {
        let target_is_64 = perf_env__kernel_is_64_bit(env);
        let host_is_64 = size_of::<*mut c_void>() == 8;

        /* 32-bit host cannot natively disassemble 64-bit target */
        if !host_is_64 && target_is_64 {
            return false;
        }
    }

    true
}

unsafe fn perf_env__lookup_binutils_path(
    env: *mut perf_env,
    name: *const c_char,
    path: *mut *mut c_char,
) -> c_int {
    let idx: c_int;
    let e_machine = perf_env__e_machine(env, ptr::null_mut());
    let cross_env: *const c_char;
    let path_list: &[&CStr];
    let mut buf: *mut c_char = ptr::null_mut();

    /*
     * We don't need to try to find objdump path for native system.
     * Just use default binutils path (e.g.: "objdump").
     */
    if is_native_compatible(env, e_machine, EM_HOST) {
        *path = buf;
        return 0;
    }

    cross_env = getenv(c"CROSS_COMPILE".as_ptr());
    if !cross_env.is_null() {
        if asprintf(&mut buf, c"%s%s".as_ptr(), cross_env, name) < 0 {
            free(buf as *mut c_void);
            *path = ptr::null_mut();
            return -1;
        }
        if *buf == b'/' as c_char {
            if access(buf, F_OK) == 0 {
                *path = buf;
                return 0;
            }
            free(buf as *mut c_void);
            *path = ptr::null_mut();
            return -1;
        }
        if lookup_path(buf) {
            *path = buf;
            return 0;
        }
        zfree(&mut buf);
    }

    match e_machine {
        EM_ARC => {
            path_list = &ARC_TRIPLETS;
        }
        EM_ARM => {
            path_list = &ARM_TRIPLETS;
        }
        EM_AARCH64 => {
            path_list = &ARM64_TRIPLETS;
        }
        EM_PPC | EM_PPC64 => {
            path_list = &POWERPC_TRIPLETS;
        }
        EM_RISCV => {
            path_list = if perf_env__kernel_is_64_bit(env) {
                &RISCV64_TRIPLETS
            } else {
                &RISCV32_TRIPLETS
            };
        }
        EM_SH => {
            path_list = &SH_TRIPLETS;
        }
        EM_S390 => {
            path_list = &S390_TRIPLETS;
        }
        EM_SPARC | EM_SPARCV9 => {
            path_list = &SPARC_TRIPLETS;
        }
        EM_X86_64 | EM_386 => {
            path_list = &X86_TRIPLETS;
        }
        EM_MIPS => {
            path_list = &MIPS_TRIPLETS;
        }
        _ => {
            ui__error(
                c"binutils for %s not supported.\n".as_ptr(),
                perf_env__arch(env),
            );
            free(buf as *mut c_void);
            *path = ptr::null_mut();
            return -1;
        }
    }

    idx = lookup_triplets(path_list, name);
    if idx < 0 {
        ui__error(
            c"Please install %s for %s.\nYou can add it to PATH, set CROSS_COMPILE or override the default using --%s.\n"
                .as_ptr(),
            name,
            perf_env__arch(env),
            name,
        );
        free(buf as *mut c_void);
        *path = ptr::null_mut();
        return -1;
    }

    if asprintf(
        &mut buf,
        c"%s%s".as_ptr(),
        path_list[idx as usize].as_ptr(),
        name,
    ) < 0
    {
        free(buf as *mut c_void);
        *path = ptr::null_mut();
        return -1;
    }

    *path = buf;
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_env__lookup_objdump(
    env: *mut perf_env,
    path: *mut *mut c_char,
) -> c_int {
    /*
     * For live mode, env->arch will be NULL and we can use
     * the native objdump tool.
     */
    if (*env).arch.is_null() {
        return 0;
    }

    perf_env__lookup_binutils_path(env, c"objdump".as_ptr(), path)
}

/*
 * Some architectures have a single address space for kernel and user addresses,
 * which makes it possible to determine if an address is in kernel space or user
 * space.
 */
#[no_mangle]
pub unsafe extern "C" fn perf_env__single_address_space(env: *mut perf_env) -> bool {
    let e_machine = perf_env__e_machine(env, ptr::null_mut());

    e_machine != EM_SPARC && e_machine != EM_SPARCV9 && e_machine != EM_S390
}
