// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2013-2015, Michael Ellerman, IBM Corp.
 */

/* _GNU_SOURCE was defined in C for CPU_ZERO etc. */

use core::ffi::c_void;
use core::ptr;

static mut auxv: [libc::c_char; 4096] = [0; 4096];

unsafe extern "C" {
    fn CPU_ALLOC(count: libc::c_int) -> *mut libc::cpu_set_t;
    fn CPU_ALLOC_SIZE(count: libc::c_int) -> libc::size_t;
    fn CPU_FREE(set: *mut libc::cpu_set_t);
    fn CPU_ZERO_S(setsize: libc::size_t, set: *mut libc::cpu_set_t);
    fn CPU_ISSET_S(cpu: libc::c_int, setsize: libc::size_t, set: *const libc::cpu_set_t) -> libc::c_int;
    fn CPU_ZERO(set: *mut libc::cpu_set_t);
    fn CPU_SET(cpu: libc::c_int, set: *mut libc::cpu_set_t);
}

const BIND_CPU_ANY: libc::c_int = -1;
const AT_NULL: libc::c_ulong = 0;

#[repr(C)]
union Elf64_auxv_t_un {
    a_val: libc::c_ulong,
}

#[repr(C)]
struct Elf64_auxv_t {
    a_type: libc::c_ulong,
    a_un: Elf64_auxv_t_un,
}

unsafe fn errno_location() -> *mut libc::c_int {
    libc::__errno_location()
}

unsafe fn set_errno(value: libc::c_int) {
    *errno_location() = value;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_file(
    path: *const libc::c_char,
    buf: *mut libc::c_char,
    count: libc::size_t,
    len: *mut libc::size_t,
) -> libc::c_int {
    let mut rc: libc::ssize_t;
    let fd: libc::c_int;
    let mut err: libc::c_int;
    let mut eof: libc::c_char = 0;

    fd = libc::open(path, libc::O_RDONLY);
    if fd < 0 {
        return -*errno_location();
    }

    rc = libc::read(fd, buf as *mut c_void, count);
    if rc < 0 {
        err = -*errno_location();
        goto_out_read_file(fd, err);
        return err;
    }

    if !len.is_null() {
        *len = rc as libc::size_t;
    }

    /* Overflow if there are still more bytes after filling the buffer */
    if rc == count as libc::ssize_t {
        rc = libc::read(fd, &mut eof as *mut _ as *mut c_void, 1);
        if rc != 0 {
            err = -libc::EOVERFLOW;
            goto_out_read_file(fd, err);
            return err;
        }
    }

    err = 0;

    goto_out_read_file(fd, err);
    err
}

unsafe fn goto_out_read_file(fd: libc::c_int, err: libc::c_int) {
    libc::close(fd);
    set_errno(-err);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_file_alloc(
    path: *const libc::c_char,
    buf: *mut *mut libc::c_char,
    len: *mut libc::size_t,
) -> libc::c_int {
    let mut read_offset: libc::size_t = 0;
    let mut buffer_len: libc::size_t = 0;
    let mut buffer: *mut libc::c_char = ptr::null_mut();
    let mut err: libc::c_int;
    let fd: libc::c_int;

    fd = libc::open(path, libc::O_RDONLY);
    if fd < 0 {
        return -*errno_location();
    }

    /*
     * We don't use stat & preallocate st_size because some non-files
     * report 0 file size. Instead just dynamically grow the buffer
     * as needed.
     */
    loop {
        let rc: libc::ssize_t;

        if read_offset >= buffer_len / 2 {
            let next_buffer: *mut libc::c_char;

            buffer_len = if buffer_len != 0 { buffer_len * 2 } else { 4096 };
            next_buffer = libc::realloc(buffer as *mut c_void, buffer_len) as *mut libc::c_char;
            if next_buffer.is_null() {
                err = -*errno_location();
                goto_out_read_file_alloc(fd, buffer, err);
                return err;
            }
            buffer = next_buffer;
        }

        rc = libc::read(
            fd,
            buffer.add(read_offset) as *mut c_void,
            buffer_len - read_offset,
        );
        if rc < 0 {
            err = -*errno_location();
            goto_out_read_file_alloc(fd, buffer, err);
            return err;
        }

        if rc == 0 {
            break;
        }

        read_offset += rc as libc::size_t;
    }

    *buf = buffer;
    if !len.is_null() {
        *len = read_offset;
    }

    err = 0;

    goto_out_read_file_alloc(fd, buffer, err);
    err
}

unsafe fn goto_out_read_file_alloc(fd: libc::c_int, buffer: *mut libc::c_char, err: libc::c_int) {
    libc::close(fd);
    if err != 0 {
        libc::free(buffer as *mut c_void);
    }
    set_errno(-err);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_file(
    path: *const libc::c_char,
    buf: *const libc::c_char,
    count: libc::size_t,
) -> libc::c_int {
    let fd: libc::c_int;
    let mut err: libc::c_int;
    let rc: libc::ssize_t;

    fd = libc::open(path, libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC, 0o644);
    if fd < 0 {
        return -*errno_location();
    }

    rc = libc::write(fd, buf as *const c_void, count);
    if rc < 0 {
        err = -*errno_location();
        goto_out_write_file(fd, err);
        return err;
    }

    if rc != count as libc::ssize_t {
        err = -libc::EOVERFLOW;
        goto_out_write_file(fd, err);
        return err;
    }

    err = 0;

    goto_out_write_file(fd, err);
    err
}

unsafe fn goto_out_write_file(fd: libc::c_int, err: libc::c_int) {
    libc::close(fd);
    set_errno(-err);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_auxv(buf: *mut libc::c_char, buf_size: libc::ssize_t) -> libc::c_int {
    let err: libc::c_int;

    err = read_file(c"/proc/self/auxv".as_ptr(), buf, buf_size as libc::size_t, ptr::null_mut());
    if err != 0 {
        libc::perror(c"Error reading /proc/self/auxv".as_ptr());
        return err;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_debugfs_file(
    subpath: *const libc::c_char,
    buf: *mut libc::c_char,
    count: libc::size_t,
) -> libc::c_int {
    let mut path: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];

    libc::strcpy(path.as_mut_ptr(), c"/sys/kernel/debug/".as_ptr());
    libc::strncat(
        path.as_mut_ptr(),
        subpath,
        core::mem::size_of_val(&path) - libc::strlen(path.as_ptr()) - 1,
    );

    read_file(path.as_ptr(), buf, count, ptr::null_mut())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_debugfs_file(
    subpath: *const libc::c_char,
    buf: *const libc::c_char,
    count: libc::size_t,
) -> libc::c_int {
    let mut path: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];

    libc::strcpy(path.as_mut_ptr(), c"/sys/kernel/debug/".as_ptr());
    libc::strncat(
        path.as_mut_ptr(),
        subpath,
        core::mem::size_of_val(&path) - libc::strlen(path.as_ptr()) - 1,
    );

    write_file(path.as_ptr(), buf, count)
}

unsafe fn validate_int_parse(
    buffer: *const libc::c_char,
    count: libc::size_t,
    mut end: *mut libc::c_char,
) -> libc::c_int {
    let mut err: libc::c_int = 0;

    /* Require at least one digit */
    if end == buffer as *mut libc::c_char {
        err = -libc::EINVAL;
        set_errno(-err);
        return err;
    }

    /* Require all remaining characters be whitespace-ish */
    while end < buffer.add(count) as *mut libc::c_char {
        if *end == 0 {
            break;
        }

        if *end != b' ' as libc::c_char && *end != b'\n' as libc::c_char {
            err = -libc::EINVAL;
            set_errno(-err);
            return err;
        }
        end = end.add(1);
    }

    set_errno(-err);
    err
}

unsafe fn parse_bounded_int(
    buffer: *const libc::c_char,
    count: libc::size_t,
    result: *mut libc::intmax_t,
    base: libc::c_int,
    min: libc::intmax_t,
    max: libc::intmax_t,
) -> libc::c_int {
    let mut err: libc::c_int;
    let mut end: *mut libc::c_char = ptr::null_mut();

    set_errno(0);
    *result = libc::strtoimax(buffer, &mut end, base);

    if *errno_location() != 0 {
        return -*errno_location();
    }

    err = validate_int_parse(buffer, count, end);
    if err != 0 {
        set_errno(-err);
        return err;
    }

    if *result < min || *result > max {
        err = -libc::EOVERFLOW;
    } else {
        err = 0;
    }

    set_errno(-err);
    err
}

unsafe fn parse_bounded_uint(
    buffer: *const libc::c_char,
    count: libc::size_t,
    result: *mut libc::uintmax_t,
    base: libc::c_int,
    max: libc::uintmax_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut end: *mut libc::c_char = ptr::null_mut();

    set_errno(0);
    *result = libc::strtoumax(buffer, &mut end, base);

    if *errno_location() != 0 {
        return -*errno_location();
    }

    err = validate_int_parse(buffer, count, end);
    if err != 0 {
        set_errno(-err);
        return err;
    }

    if *result > max {
        err = -libc::EOVERFLOW;
    }

    set_errno(-err);
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_intmax(
    buffer: *const libc::c_char,
    count: libc::size_t,
    result: *mut libc::intmax_t,
    base: libc::c_int,
) -> libc::c_int {
    parse_bounded_int(buffer, count, result, base, libc::INTMAX_MIN, libc::INTMAX_MAX)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_uintmax(
    buffer: *const libc::c_char,
    count: libc::size_t,
    result: *mut libc::uintmax_t,
    base: libc::c_int,
) -> libc::c_int {
    parse_bounded_uint(buffer, count, result, base, libc::UINTMAX_MAX)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_int(
    buffer: *const libc::c_char,
    count: libc::size_t,
    result: *mut libc::c_int,
    base: libc::c_int,
) -> libc::c_int {
    let mut parsed: libc::intmax_t = 0;
    let err = parse_bounded_int(
        buffer,
        count,
        &mut parsed,
        base,
        libc::INT_MIN as libc::intmax_t,
        libc::INT_MAX as libc::intmax_t,
    );

    *result = parsed as libc::c_int;
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_uint(
    buffer: *const libc::c_char,
    count: libc::size_t,
    result: *mut libc::c_uint,
    base: libc::c_int,
) -> libc::c_int {
    let mut parsed: libc::uintmax_t = 0;
    let err = parse_bounded_uint(buffer, count, &mut parsed, base, libc::UINT_MAX as libc::uintmax_t);

    *result = parsed as libc::c_uint;
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_long(
    buffer: *const libc::c_char,
    count: libc::size_t,
    result: *mut libc::c_long,
    base: libc::c_int,
) -> libc::c_int {
    let mut parsed: libc::intmax_t = 0;
    let err = parse_bounded_int(
        buffer,
        count,
        &mut parsed,
        base,
        libc::LONG_MIN as libc::intmax_t,
        libc::LONG_MAX as libc::intmax_t,
    );

    *result = parsed as libc::c_long;
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn parse_ulong(
    buffer: *const libc::c_char,
    count: libc::size_t,
    result: *mut libc::c_ulong,
    base: libc::c_int,
) -> libc::c_int {
    let mut parsed: libc::uintmax_t = 0;
    let err = parse_bounded_uint(buffer, count, &mut parsed, base, libc::ULONG_MAX as libc::uintmax_t);

    *result = parsed as libc::c_ulong;
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_long(
    path: *const libc::c_char,
    result: *mut libc::c_long,
    base: libc::c_int,
) -> libc::c_int {
    let err: libc::c_int;
    let mut buffer: [libc::c_char; 32] = [0; 32];

    err = read_file(path, buffer.as_mut_ptr(), core::mem::size_of_val(&buffer) - 1, ptr::null_mut());
    if err != 0 {
        return err;
    }

    parse_long(buffer.as_ptr(), core::mem::size_of_val(&buffer), result, base)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_ulong(
    path: *const libc::c_char,
    result: *mut libc::c_ulong,
    base: libc::c_int,
) -> libc::c_int {
    let err: libc::c_int;
    let mut buffer: [libc::c_char; 32] = [0; 32];

    err = read_file(path, buffer.as_mut_ptr(), core::mem::size_of_val(&buffer) - 1, ptr::null_mut());
    if err != 0 {
        return err;
    }

    parse_ulong(buffer.as_ptr(), core::mem::size_of_val(&buffer), result, base)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_long(
    path: *const libc::c_char,
    result: libc::c_long,
    base: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int;
    let len: libc::c_int;
    let mut buffer: [libc::c_char; 32] = [0; 32];

    /* Decimal only for now: no format specifier for signed hex values */
    if base != 10 {
        err = -libc::EINVAL;
        set_errno(-err);
        return err;
    }

    len = libc::snprintf(buffer.as_mut_ptr(), core::mem::size_of_val(&buffer), c"%ld".as_ptr(), result);
    if len < 0 || len as usize >= core::mem::size_of_val(&buffer) {
        err = -libc::EOVERFLOW;
        set_errno(-err);
        return err;
    }

    err = write_file(path, buffer.as_ptr(), len as libc::size_t);

    set_errno(-err);
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_ulong(
    path: *const libc::c_char,
    result: libc::c_ulong,
    base: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int;
    let len: libc::c_int;
    let mut buffer: [libc::c_char; 32] = [0; 32];
    let fmt: *const libc::c_char;

    match base {
        10 => {
            fmt = c"%lu".as_ptr();
        }
        16 => {
            fmt = c"%lx".as_ptr();
        }
        _ => {
            err = -libc::EINVAL;
            set_errno(-err);
            return err;
        }
    }

    len = libc::snprintf(buffer.as_mut_ptr(), core::mem::size_of_val(&buffer), fmt, result);
    if len < 0 || len as usize >= core::mem::size_of_val(&buffer) {
        err = -*errno_location();
        set_errno(-err);
        return err;
    }

    err = write_file(path, buffer.as_ptr(), len as libc::size_t);

    set_errno(-err);
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn find_auxv_entry(
    type_: libc::c_int,
    auxv: *mut libc::c_char,
) -> *mut c_void {
    let mut p: *mut Elf64_auxv_t;

    p = auxv as *mut Elf64_auxv_t;

    while (*p).a_type != AT_NULL {
        if (*p).a_type == type_ as libc::c_ulong {
            return p as *mut c_void;
        }

        p = p.add(1);
    }

    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_auxv_entry(type_: libc::c_int) -> *mut c_void {
    let p: *mut Elf64_auxv_t;

    if read_auxv(core::ptr::addr_of_mut!(auxv) as *mut libc::c_char, core::mem::size_of_val(&auxv) as libc::ssize_t) != 0 {
        return ptr::null_mut();
    }

    p = find_auxv_entry(type_, core::ptr::addr_of_mut!(auxv) as *mut libc::c_char) as *mut Elf64_auxv_t;
    if !p.is_null() {
        return (*p).a_un.a_val as *mut c_void;
    }

    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pick_online_cpu() -> libc::c_int {
    let ncpus: libc::c_int;
    let mut cpu: libc::c_int = -1;
    let mask: *mut libc::cpu_set_t;
    let size: libc::size_t;

    ncpus = libc::get_nprocs_conf();
    size = CPU_ALLOC_SIZE(ncpus);
    mask = CPU_ALLOC(ncpus);
    if mask.is_null() {
        libc::perror(c"malloc".as_ptr());
        return -1;
    }

    CPU_ZERO_S(size, mask);

    if libc::sched_getaffinity(0, size, mask) != 0 {
        libc::perror(c"sched_getaffinity".as_ptr());
        CPU_FREE(mask);
        return cpu;
    }

    /* We prefer a primary thread, but skip 0 */
    cpu = 8;
    while cpu < ncpus {
        if CPU_ISSET_S(cpu, size, mask) != 0 {
            CPU_FREE(mask);
            return cpu;
        }
        cpu += 8;
    }

    /* Search for anything, but in reverse */
    cpu = ncpus - 1;
    while cpu >= 0 {
        if CPU_ISSET_S(cpu, size, mask) != 0 {
            CPU_FREE(mask);
            return cpu;
        }
        cpu -= 1;
    }

    libc::printf(c"No cpus in affinity mask?!\n".as_ptr());

    CPU_FREE(mask);
    cpu
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn bind_to_cpu(mut cpu: libc::c_int) -> libc::c_int {
    let mut mask: libc::cpu_set_t = core::mem::zeroed();
    let err: libc::c_int;

    if cpu == BIND_CPU_ANY {
        cpu = pick_online_cpu();
        if cpu < 0 {
            return cpu;
        }
    }

    libc::printf(c"Binding to cpu %d\n".as_ptr(), cpu);

    CPU_ZERO(&mut mask);
    CPU_SET(cpu, &mut mask);

    err = libc::sched_setaffinity(0, core::mem::size_of_val(&mask), &mask);
    if err != 0 {
        return err;
    }

    cpu
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn is_ppc64le() -> bool {
    let mut uts: libc::utsname = core::mem::zeroed();
    let rc: libc::c_int;

    set_errno(0);
    rc = libc::uname(&mut uts);
    if rc != 0 {
        libc::perror(c"uname".as_ptr());
        return false;
    }

    libc::strcmp(uts.machine.as_ptr(), c"ppc64le".as_ptr()) == 0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_sysfs_file(
    fpath: *mut libc::c_char,
    result: *mut libc::c_char,
    result_size: libc::size_t,
) -> libc::c_int {
    let mut path: [libc::c_char; libc::PATH_MAX as usize] = [0; libc::PATH_MAX as usize];

    libc::strcpy(path.as_mut_ptr(), c"/sys/".as_ptr());
    libc::strncat(
        path.as_mut_ptr(),
        fpath,
        libc::PATH_MAX as libc::size_t - libc::strlen(path.as_ptr()) - 1,
    );

    read_file(path.as_ptr(), result, result_size, ptr::null_mut())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_debugfs_int(
    debugfs_file: *const libc::c_char,
    result: *mut libc::c_int,
) -> libc::c_int {
    let err: libc::c_int;
    let mut value: [libc::c_char; 16] = [0; 16];

    err = read_debugfs_file(debugfs_file, value.as_mut_ptr(), core::mem::size_of_val(&value) - 1);
    if err != 0 {
        return err;
    }

    parse_int(value.as_ptr(), core::mem::size_of_val(&value), result, 10)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn write_debugfs_int(
    debugfs_file: *const libc::c_char,
    result: libc::c_int,
) -> libc::c_int {
    let mut value: [libc::c_char; 16] = [0; 16];

    libc::snprintf(value.as_mut_ptr(), 16, c"%d".as_ptr(), result);

    write_debugfs_file(debugfs_file, value.as_ptr(), libc::strlen(value.as_ptr()))
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
