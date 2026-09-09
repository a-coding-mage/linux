/*
 * PROM library initialisation code, supports YAMON and U-Boot.
 *
 * Translated from the corresponding C implementation.  Linux, MIPS, and
 * PROM declarations supplied by the surrounding build are intentionally
 * referenced as external symbols here.
 */

use core::ffi::c_char;

extern "C" {
    static mut fw_arg0: usize;
    static mut fw_arg1: usize;
    static mut fw_arg2: usize;
    static mut arcs_cmdline: [c_char; COMMAND_LINE_SIZE];

    fn strlcat(dst: *mut c_char, src: *const c_char, size: usize) -> usize;
    fn strlen(s: *const c_char) -> usize;
    fn strchr(s: *const c_char, c: i32) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> i32;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> i32;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn kstrtoul(s: *const c_char, base: u32, result: *mut usize) -> i32;
    fn memblock_add(base: usize, size: usize);
}

const COMMAND_LINE_SIZE: usize = 4096;
const SZ_64M: usize = 64 * 1024 * 1024;

pub static mut prom_argc: i32 = 0;
pub static mut prom_argv: *mut *mut c_char = core::ptr::null_mut();
pub static mut prom_envp: *mut *mut c_char = core::ptr::null_mut();

pub unsafe extern "C" fn prom_init_cmdline() {
    let mut i: i32 = 1;

    while i < prom_argc {
        strlcat(
            arcs_cmdline.as_mut_ptr(),
            *prom_argv.add(i as usize),
            COMMAND_LINE_SIZE,
        );
        if i < prom_argc - 1 {
            strlcat(
                arcs_cmdline.as_mut_ptr(),
                b" \0".as_ptr() as *const c_char,
                COMMAND_LINE_SIZE,
            );
        }
        i += 1;
    }
}

pub unsafe extern "C" fn prom_getenv(envname: *mut c_char) -> *mut c_char {
    /*
     * Return a pointer to the given environment variable.
     * YAMON uses "name", "value" pairs, while U-Boot uses "name=value".
     */
    let mut env = prom_envp;
    let i = strlen(envname);
    let yamon = !(*env).is_null() && strchr(*env, b'=' as i32).is_null();

    while !(*env).is_null() {
        if yamon {
            if strcmp(envname, *env) == 0 {
                return *env.add(1);
            }
            env = env.add(1);
        } else if strncmp(envname, *env, i) == 0 && (*(*env).add(i) as u8) == b'=' {
            return (*env).add(i + 1);
        }
        env = env.add(1);
    }

    core::ptr::null_mut()
}

pub unsafe extern "C" fn prom_init() {
    let memsize_str: *mut c_char;
    let mut memsize: usize;

    prom_argc = fw_arg0 as i32;
    prom_argv = fw_arg1 as *mut *mut c_char;
    prom_envp = fw_arg2 as *mut *mut c_char;

    prom_init_cmdline();

    memsize_str = prom_getenv(b"memsize\0".as_ptr() as *mut c_char);
    if memsize_str.is_null() || kstrtoul(memsize_str, 0, &mut memsize) != 0 {
        memsize = SZ_64M; /* minimum memsize is 64MB RAM */
    }

    memblock_add(0, memsize);
}

#[inline]
unsafe fn str2hexnum(c: u8) -> u8 {
    if c >= b'0' && c <= b'9' {
        return c - b'0';
    }
    if c >= b'a' && c <= b'f' {
        return c - b'a' + 10;
    }
    if c >= b'A' && c <= b'F' {
        return c - b'A' + 10;
    }

    0 /* foo */
}

#[inline]
unsafe fn str2eaddr(ea: *mut u8, mut str_: *mut u8) {
    let mut i = 0;

    while i < 6 {
        let mut num: u8;

        if *str_ == b'.' || *str_ == b':' {
            str_ = str_.add(1);
        }
        num = str2hexnum(*str_) << 4;
        str_ = str_.add(1);
        num |= str2hexnum(*str_);
        str_ = str_.add(1);
        *ea.add(i) = num;
        i += 1;
    }
}

pub unsafe extern "C" fn prom_get_ethernet_addr(ethernet_addr: *mut c_char) -> i32 {
    let mut ethaddr_str: *mut c_char;

    /* Check the environment variables first */
    ethaddr_str = prom_getenv(b"ethaddr\0".as_ptr() as *mut c_char);
    if ethaddr_str.is_null() {
        /* Check command line */
        ethaddr_str = strstr(
            arcs_cmdline.as_ptr(),
            b"ethaddr=\0".as_ptr() as *const c_char,
        );
        if ethaddr_str.is_null() {
            return -1;
        }

        ethaddr_str = ethaddr_str.add(strlen(b"ethaddr=\0".as_ptr() as *const c_char));
    }

    str2eaddr(ethernet_addr as *mut u8, ethaddr_str as *mut u8);

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
