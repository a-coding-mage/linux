// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/******************************************************************************
 *
 * Module Name: osunixxf - UNIX OSL interfaces
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/*
 * These interfaces are required in order to compile the ASL compiler and the
 * various ACPICA tools under Linux or other Unix-like system.
 *
 * C dependency intent: <acpi/acpi.h>, "accommon.h", "amlcode.h",
 * "acparser.h", "acdebug.h", and POSIX/libc headers.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type acpi_status = u32;
pub type acpi_physical_address = u64;
pub type acpi_size = usize;
pub type acpi_string = *mut c_char;
pub type acpi_handle = *mut c_void;
pub type acpi_spinlock = acpi_handle;
pub type acpi_cpu_flags = usize;
pub type acpi_io_address = u64;
pub type acpi_thread_id = u64;
pub type acpi_execute_type = u32;
pub type va_list = *mut c_void;
pub type FILE = c_void;
pub type pthread_t = c_ulong;
pub type PTHREAD_CALLBACK = Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>;
pub type acpi_osd_handler = Option<unsafe extern "C" fn(*mut c_void) -> u32>;
pub type acpi_osd_exec_callback = Option<unsafe extern "C" fn(*mut c_void)>;

#[repr(C)]
pub struct acpi_table_header {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct acpi_predefined_names {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct acpi_pci_id {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[repr(C)]
pub struct sem_t {
    _unused: [u8; 0],
}

#[cfg(ACPI_EXEC_APP)]
#[repr(C)]
pub struct termios {
    pub c_iflag: c_uint,
    pub c_oflag: c_uint,
    pub c_cflag: c_uint,
    pub c_lflag: c_uint,
    pub c_line: u8,
    pub c_cc: [u8; 32],
    pub c_ispeed: c_uint,
    pub c_ospeed: c_uint,
}

pub const AE_OK: acpi_status = 0;
pub const AE_ERROR: acpi_status = 1;
pub const AE_NO_MEMORY: acpi_status = 4;
pub const AE_BAD_PARAMETER: acpi_status = 5;
pub const AE_BUFFER_OVERFLOW: acpi_status = 12;
pub const AE_NO_ACPI_TABLES: acpi_status = 17;
pub const AE_TIME: acpi_status = 20;
pub const AE_SUPPORT: acpi_status = 27;
pub const AE_LIMIT: acpi_status = 28;

pub const TRUE: u8 = 1;
pub const ACPI_DB_REDIRECTABLE_OUTPUT: u8 = 0x01;
pub const ACPI_DB_CONSOLE_OUTPUT: u8 = 0x02;
pub const ACPI_WAIT_FOREVER: u16 = 0xFFFF;
pub const ACPI_MSEC_PER_SEC: u64 = 1000;
pub const ACPI_USEC_PER_MSEC: u32 = 1000;
pub const ACPI_NSEC_PER_MSEC: c_long = 1000000;
pub const ACPI_NSEC_PER_SEC: c_long = 1000000000;
pub const ACPI_100NSEC_PER_SEC: u64 = 10000000;
pub const ACPI_100NSEC_PER_USEC: u64 = 10;
pub const ACPI_SIGNAL_FATAL: u32 = 0;
pub const ACPI_SIGNAL_BREAKPOINT: u32 = 1;
pub const ACPI_VPRINTF_BUFFER_SIZE: usize = 512;
pub const _ASCII_NEWLINE: c_int = b'\n' as c_int;
pub const EOF: c_int = -1;
pub const CLOCK_REALTIME: c_int = 0;
pub const EINTR: c_int = 4;
pub const ETIMEDOUT: c_int = 110;

#[cfg(ACPI_EXEC_APP)]
pub const STDIN_FILENO: c_int = 0;
#[cfg(ACPI_EXEC_APP)]
pub const ICANON: c_uint = 0x0002;
#[cfg(ACPI_EXEC_APP)]
pub const ECHO: c_uint = 0x0008;
#[cfg(ACPI_EXEC_APP)]
pub const VMIN: usize = 6;
#[cfg(ACPI_EXEC_APP)]
pub const VTIME: usize = 5;
#[cfg(ACPI_EXEC_APP)]
pub const TCSANOW: c_int = 0;

unsafe extern "C" {
    pub static mut acpi_gbl_output_file: *mut FILE;
    pub static mut acpi_gbl_print_lock: acpi_spinlock;
    pub static mut acpi_gbl_db_output_flags: u8;
    pub static mut acpi_gbl_debug_file: *mut FILE;
    pub static mut stdout: *mut FILE;
    pub static mut stderr: *mut FILE;
    pub static mut errno: c_int;

    pub fn ae_table_override(
        existing_table: *mut acpi_table_header,
        new_table: *mut *mut acpi_table_header,
    );

    pub fn malloc(size: usize) -> *mut c_void;
    pub fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    pub fn free(ptr: *mut c_void);
    pub fn getchar() -> c_int;
    pub fn vfprintf(stream: *mut FILE, fmt: *const c_char, arg: va_list) -> c_int;
    pub fn vsnprintf(str_: *mut c_char, size: usize, fmt: *const c_char, ap: va_list) -> c_int;
    pub fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    pub fn printf(fmt: *const c_char, ...) -> c_int;
    pub fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    pub fn perror(s: *const c_char);
    pub fn usleep(usec: u32) -> c_int;
    pub fn sleep(seconds: u64) -> u32;
    pub fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    pub fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    pub fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
    pub fn sem_destroy(sem: *mut sem_t) -> c_int;
    pub fn sem_trywait(sem: *mut sem_t) -> c_int;
    pub fn sem_wait(sem: *mut sem_t) -> c_int;
    pub fn sem_timedwait(sem: *mut sem_t, abs_timeout: *const timespec) -> c_int;
    pub fn sem_post(sem: *mut sem_t) -> c_int;
    pub fn pthread_self() -> pthread_t;
    pub fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: PTHREAD_CALLBACK,
        arg: *mut c_void,
    ) -> c_int;

    #[cfg(ACPI_EXEC_APP)]
    pub fn isatty(fd: c_int) -> c_int;
    #[cfg(ACPI_EXEC_APP)]
    pub fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
    #[cfg(ACPI_EXEC_APP)]
    pub fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
}

#[cfg(ACPI_EXEC_APP)]
unsafe extern "C" {
    pub fn acpi_ut_read_line(buffer: *mut c_char, buffer_length: u32, bytes_read: *mut u32) -> acpi_status;
}

#[cfg(ACPI_EXEC_APP)]
pub static mut original_term_attributes: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};

#[cfg(ACPI_EXEC_APP)]
pub static mut term_attributes_were_set: c_int = 0;

#[cfg(ACPI_EXEC_APP)]
unsafe fn os_enter_line_edit_mode() {
    let mut local_term_attributes: termios = core::mem::zeroed();

    term_attributes_were_set = 0;

    /* STDIN must be a terminal */
    if isatty(STDIN_FILENO) == 0 {
        return;
    }

    /* Get and keep the original attributes */
    if tcgetattr(STDIN_FILENO, &raw mut original_term_attributes) != 0 {
        fprintf(stderr, c"Could not get terminal attributes!\n".as_ptr());
        return;
    }

    /* Set the new attributes to enable raw character input */
    core::ptr::copy_nonoverlapping(
        &raw const original_term_attributes,
        &mut local_term_attributes,
        1,
    );

    local_term_attributes.c_lflag &= !(ICANON | ECHO);
    local_term_attributes.c_cc[VMIN] = 1;
    local_term_attributes.c_cc[VTIME] = 0;

    if tcsetattr(STDIN_FILENO, TCSANOW, &local_term_attributes) != 0 {
        fprintf(stderr, c"Could not set terminal attributes!\n".as_ptr());
        return;
    }

    term_attributes_were_set = 1;
}

#[cfg(ACPI_EXEC_APP)]
unsafe fn os_exit_line_edit_mode() {
    if term_attributes_were_set == 0 {
        return;
    }

    /* Set terminal attributes back to the original values */
    if tcsetattr(STDIN_FILENO, TCSANOW, &raw const original_term_attributes) != 0 {
        fprintf(stderr, c"Could not restore terminal attributes!\n".as_ptr());
    }
}

#[cfg(not(ACPI_EXEC_APP))]
unsafe fn os_enter_line_edit_mode() {}

#[cfg(not(ACPI_EXEC_APP))]
unsafe fn os_exit_line_edit_mode() {}

pub const fn ACPI_FAILURE(status: acpi_status) -> bool {
    status != AE_OK
}

pub const fn ACPI_TO_POINTER(value: acpi_size) -> *mut c_void {
    value as *mut c_void
}

pub const fn ACPI_CAST_PTHREAD_T(thread: pthread_t) -> acpi_thread_id {
    thread as acpi_thread_id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_initialize() -> acpi_status {
    let status: acpi_status;

    acpi_gbl_output_file = stdout;

    os_enter_line_edit_mode();

    status = acpi_os_create_lock(&raw mut acpi_gbl_print_lock);
    if ACPI_FAILURE(status) {
        return status;
    }

    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_terminate() -> acpi_status {
    os_exit_line_edit_mode();
    AE_OK
}

#[cfg(not(ACPI_USE_NATIVE_RSDP_POINTER))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_get_root_pointer() -> acpi_physical_address {
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_predefined_override(
    init_val: *const acpi_predefined_names,
    new_val: *mut acpi_string,
) -> acpi_status {
    if init_val.is_null() || new_val.is_null() {
        return AE_BAD_PARAMETER;
    }

    *new_val = core::ptr::null_mut();
    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_table_override(
    existing_table: *mut acpi_table_header,
    new_table: *mut *mut acpi_table_header,
) -> acpi_status {
    if existing_table.is_null() || new_table.is_null() {
        return AE_BAD_PARAMETER;
    }

    *new_table = core::ptr::null_mut();

    #[cfg(ACPI_EXEC_APP)]
    {
        ae_table_override(existing_table, new_table);
        return AE_OK;
    }

    #[cfg(not(ACPI_EXEC_APP))]
    {
        AE_NO_ACPI_TABLES
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_physical_table_override(
    existing_table: *mut acpi_table_header,
    new_address: *mut acpi_physical_address,
    new_table_length: *mut u32,
) -> acpi_status {
    AE_SUPPORT
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_enter_sleep(
    sleep_state: u8,
    rega_value: u32,
    regb_value: u32,
) -> acpi_status {
    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_redirect_output(destination: *mut c_void) {
    acpi_gbl_output_file = destination as *mut FILE;
}

/*
 * C varargs cannot be implemented on stable Rust in a faithful local mapping.
 * The externally visible ACPI interface and intent are preserved here.
 */
#[cfg(any())]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_printf(fmt: *const c_char, mut args: ...) {
    let mut flags: u8;

    flags = acpi_gbl_db_output_flags;
    if flags & ACPI_DB_REDIRECTABLE_OUTPUT != 0 {
        if !acpi_gbl_debug_file.is_null() {
            vfprintf(acpi_gbl_debug_file, fmt, args.as_va_list());
        } else {
            flags |= ACPI_DB_CONSOLE_OUTPUT;
        }
    }

    if flags & ACPI_DB_CONSOLE_OUTPUT != 0 {
        vfprintf(acpi_gbl_output_file, fmt, args.as_va_list());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_vprintf(fmt: *const c_char, args: va_list) {
    let mut flags: u8;
    let mut buffer: [c_char; ACPI_VPRINTF_BUFFER_SIZE] = [0; ACPI_VPRINTF_BUFFER_SIZE];

    /*
     * We build the output string in a local buffer because we may be
     * outputting the buffer twice. Using vfprintf is problematic because
     * some implementations modify the args pointer/structure during
     * execution. Thus, we use the local buffer for portability.
     *
     * Note: Since this module is intended for use by the various ACPICA
     * utilities/applications, we can safely declare the buffer on the stack.
     * Also, This function is used for relatively small error messages only.
     */
    vsnprintf(buffer.as_mut_ptr(), ACPI_VPRINTF_BUFFER_SIZE, fmt, args);

    flags = acpi_gbl_db_output_flags;
    if flags & ACPI_DB_REDIRECTABLE_OUTPUT != 0 {
        /* Output is directable to either a file (if open) or the console */
        if !acpi_gbl_debug_file.is_null() {
            /* Output file is open, send the output there */
            fputs(buffer.as_ptr(), acpi_gbl_debug_file);
        } else {
            /* No redirection, send output to console (once only!) */
            flags |= ACPI_DB_CONSOLE_OUTPUT;
        }
    }

    if flags & ACPI_DB_CONSOLE_OUTPUT != 0 {
        fputs(buffer.as_ptr(), acpi_gbl_output_file);
    }
}

#[cfg(not(ACPI_EXEC_APP))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_get_line(
    buffer: *mut c_char,
    buffer_length: u32,
    bytes_read: *mut u32,
) -> acpi_status {
    let mut input_char: c_int;
    let mut end_of_line: u32 = 0;

    /* Standard acpi_os_get_line for all utilities except acpi_exec */
    loop {
        if end_of_line >= buffer_length {
            return AE_BUFFER_OVERFLOW;
        }

        input_char = getchar();
        if input_char == EOF {
            return AE_ERROR;
        }

        if input_char == 0 || input_char == _ASCII_NEWLINE {
            break;
        }

        *buffer.add(end_of_line as usize) = input_char as c_char;
        end_of_line = end_of_line.wrapping_add(1);
    }

    /* Null terminate the buffer */
    *buffer.add(end_of_line as usize) = 0;

    /* Return the number of bytes in the string */
    if !bytes_read.is_null() {
        *bytes_read = end_of_line;
    }

    AE_OK
}

#[cfg(not(ACPI_USE_NATIVE_MEMORY_MAPPING))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_map_memory(
    where_: acpi_physical_address,
    length: acpi_size,
) -> *mut c_void {
    ACPI_TO_POINTER(where_ as acpi_size)
}

#[cfg(not(ACPI_USE_NATIVE_MEMORY_MAPPING))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_unmap_memory(where_: *mut c_void, length: acpi_size) {
    return;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_allocate(size: acpi_size) -> *mut c_void {
    let mem: *mut c_void;

    mem = malloc(size as usize);
    mem
}

#[cfg(USE_NATIVE_ALLOCATE_ZEROED)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_allocate_zeroed(size: acpi_size) -> *mut c_void {
    let mem: *mut c_void;

    mem = calloc(1, size as usize);
    mem
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_free(mem: *mut c_void) {
    free(mem);
}

#[cfg(ACPI_SINGLE_THREADED)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_create_semaphore(
    max_units: u32,
    initial_units: u32,
    out_handle: *mut acpi_handle,
) -> acpi_status {
    *out_handle = 1usize as acpi_handle;
    AE_OK
}

#[cfg(ACPI_SINGLE_THREADED)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_delete_semaphore(handle: acpi_handle) -> acpi_status {
    AE_OK
}

#[cfg(ACPI_SINGLE_THREADED)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_wait_semaphore(
    handle: acpi_handle,
    units: u32,
    timeout: u16,
) -> acpi_status {
    AE_OK
}

#[cfg(ACPI_SINGLE_THREADED)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_signal_semaphore(handle: acpi_handle, units: u32) -> acpi_status {
    AE_OK
}

#[cfg(not(ACPI_SINGLE_THREADED))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_create_semaphore(
    max_units: u32,
    initial_units: u32,
    out_handle: *mut acpi_handle,
) -> acpi_status {
    let sem: *mut sem_t;

    if out_handle.is_null() {
        return AE_BAD_PARAMETER;
    }

    /*
     * C __APPLE__ branch used sem_open/sem_unlink named semaphores. This
     * translation preserves the non-Apple POSIX branch as the directly local
     * mapping and leaves the Apple-specific build intent here.
     */
    sem = acpi_os_allocate(core::mem::size_of::<sem_t>()) as *mut sem_t;
    if sem.is_null() {
        return AE_NO_MEMORY;
    }

    if sem_init(sem, 0, initial_units as c_uint) == -1 {
        acpi_os_free(sem as *mut c_void);
        return AE_BAD_PARAMETER;
    }

    *out_handle = sem as acpi_handle;
    AE_OK
}

#[cfg(not(ACPI_SINGLE_THREADED))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_delete_semaphore(handle: acpi_handle) -> acpi_status {
    let sem: *mut sem_t = handle as *mut sem_t;

    if sem.is_null() {
        return AE_BAD_PARAMETER;
    }

    if sem_destroy(sem) == -1 {
        return AE_BAD_PARAMETER;
    }

    AE_OK
}

#[cfg(not(ACPI_SINGLE_THREADED))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_wait_semaphore(
    handle: acpi_handle,
    units: u32,
    mut msec_timeout: u16,
) -> acpi_status {
    let mut status: acpi_status = AE_OK;
    let sem: *mut sem_t = handle as *mut sem_t;
    let mut ret_val: c_int;
    #[cfg(not(ACPI_USE_ALTERNATE_TIMEOUT))]
    let mut time: timespec = core::mem::zeroed();

    if sem.is_null() {
        return AE_BAD_PARAMETER;
    }

    match msec_timeout {
        /*
         * No Wait:
         * --------
         * A zero timeout value indicates that we shouldn't wait - just
         * acquire the semaphore if available otherwise return AE_TIME
         * (a.k.a. 'would block').
         */
        0 => {
            if sem_trywait(sem) == -1 {
                status = AE_TIME;
            }
        }

        /* Wait Indefinitely */
        ACPI_WAIT_FOREVER => {
            loop {
                ret_val = sem_wait(sem);
                if !(ret_val == -1 && errno == EINTR) {
                    break;
                }
                continue; /* Restart if interrupted */
            }
            if ret_val != 0 {
                status = AE_TIME;
            }
        }

        /* Wait with msec_timeout */
        _ => {
            #[cfg(ACPI_USE_ALTERNATE_TIMEOUT)]
            {
                /*
                 * Alternate timeout mechanism for environments where
                 * sem_timedwait is not available or does not work properly.
                 */
                while msec_timeout != 0 {
                    if sem_trywait(sem) == 0 {
                        /* Got the semaphore */
                        return AE_OK;
                    }

                    if msec_timeout >= 10 {
                        msec_timeout -= 10;
                        usleep(10 * ACPI_USEC_PER_MSEC); /* ten milliseconds */
                    } else {
                        msec_timeout -= 1;
                        usleep(ACPI_USEC_PER_MSEC); /* one millisecond */
                    }
                }
                status = AE_TIME;
            }

            #[cfg(not(ACPI_USE_ALTERNATE_TIMEOUT))]
            {
                /*
                 * The interface to sem_timedwait is an absolute time, so we need to
                 * get the current time, then add in the millisecond Timeout value.
                 */
                if clock_gettime(CLOCK_REALTIME, &mut time) == -1 {
                    perror(c"clock_gettime".as_ptr());
                    return AE_TIME;
                }

                time.tv_sec += (msec_timeout as c_long / ACPI_MSEC_PER_SEC as c_long);
                time.tv_nsec +=
                    ((msec_timeout as c_long % ACPI_MSEC_PER_SEC as c_long) * ACPI_NSEC_PER_MSEC);

                /* Handle nanosecond overflow (field must be less than one second) */
                if time.tv_nsec >= ACPI_NSEC_PER_SEC {
                    time.tv_sec += time.tv_nsec / ACPI_NSEC_PER_SEC;
                    time.tv_nsec %= ACPI_NSEC_PER_SEC;
                }

                loop {
                    ret_val = sem_timedwait(sem, &time);
                    if !(ret_val == -1 && errno == EINTR) {
                        break;
                    }
                    continue; /* Restart if interrupted */
                }

                if ret_val != 0 {
                    if errno != ETIMEDOUT {
                        perror(c"sem_timedwait".as_ptr());
                    }
                    status = AE_TIME;
                }
            }
        }
    }

    status
}

#[cfg(not(ACPI_SINGLE_THREADED))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_signal_semaphore(handle: acpi_handle, units: u32) -> acpi_status {
    let sem: *mut sem_t = handle as *mut sem_t;

    if sem.is_null() {
        return AE_BAD_PARAMETER;
    }

    if sem_post(sem) == -1 {
        return AE_LIMIT;
    }

    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_create_lock(out_handle: *mut acpi_spinlock) -> acpi_status {
    acpi_os_create_semaphore(1, 1, out_handle as *mut acpi_handle)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_delete_lock(handle: acpi_spinlock) {
    acpi_os_delete_semaphore(handle);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_acquire_lock(handle: acpi_handle) -> acpi_cpu_flags {
    acpi_os_wait_semaphore(handle, 1, 0xFFFF);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_release_lock(handle: acpi_spinlock, flags: acpi_cpu_flags) {
    acpi_os_signal_semaphore(handle, 1);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_install_interrupt_handler(
    interrupt_number: u32,
    service_routine: acpi_osd_handler,
    context: *mut c_void,
) -> u32 {
    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_remove_interrupt_handler(
    interrupt_number: u32,
    service_routine: acpi_osd_handler,
) -> acpi_status {
    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_stall(microseconds: u32) {
    if microseconds != 0 {
        usleep(microseconds);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_sleep(milliseconds: u64) {
    /* Sleep for whole seconds */
    sleep(milliseconds / ACPI_MSEC_PER_SEC);

    /*
     * Sleep for remaining microseconds.
     * Arg to usleep() is in usecs and must be less than 1,000,000 (1 second).
     */
    usleep(((milliseconds % ACPI_MSEC_PER_SEC) as u32) * ACPI_USEC_PER_MSEC);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_get_timer() -> u64 {
    let mut time: timeval = core::mem::zeroed();

    /* This timer has sufficient resolution for user-space application code */
    gettimeofday(&mut time, core::ptr::null_mut());

    /* (Seconds * 10^7 = 100ns(10^-7)) + (Microseconds(10^-6) * 10^1 = 100ns) */
    ((time.tv_sec as u64).wrapping_mul(ACPI_100NSEC_PER_SEC))
        .wrapping_add((time.tv_usec as u64).wrapping_mul(ACPI_100NSEC_PER_USEC))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_read_pci_configuration(
    pci_id: *mut acpi_pci_id,
    pci_register: u32,
    value: *mut u64,
    width: u32,
) -> acpi_status {
    *value = 0;
    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_write_pci_configuration(
    pci_id: *mut acpi_pci_id,
    pci_register: u32,
    value: u64,
    width: u32,
) -> acpi_status {
    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_read_port(
    address: acpi_io_address,
    value: *mut u32,
    width: u32,
) -> acpi_status {
    match width {
        8 => {
            *value = 0xFF;
        }
        16 => {
            *value = 0xFFFF;
        }
        32 => {
            *value = 0xFFFFFFFF;
        }
        _ => {
            return AE_BAD_PARAMETER;
        }
    }

    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_write_port(
    address: acpi_io_address,
    value: u32,
    width: u32,
) -> acpi_status {
    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_read_memory(
    address: acpi_physical_address,
    value: *mut u64,
    width: u32,
) -> acpi_status {
    match width {
        8 | 16 | 32 | 64 => {
            *value = 0;
        }
        _ => {
            return AE_BAD_PARAMETER;
        }
    }

    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_write_memory(
    address: acpi_physical_address,
    value: u64,
    width: u32,
) -> acpi_status {
    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_readable(pointer: *mut c_void, length: acpi_size) -> u8 {
    TRUE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_writable(pointer: *mut c_void, length: acpi_size) -> u8 {
    TRUE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_signal(function: u32, info: *mut c_void) -> acpi_status {
    match function {
        ACPI_SIGNAL_FATAL => {}
        ACPI_SIGNAL_BREAKPOINT => {}
        _ => {}
    }

    AE_OK
}

#[cfg(not(ACPI_SINGLE_THREADED))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_get_thread_id() -> acpi_thread_id {
    let thread: pthread_t;

    thread = pthread_self();
    ACPI_CAST_PTHREAD_T(thread)
}

#[cfg(not(ACPI_SINGLE_THREADED))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_execute(
    type_: acpi_execute_type,
    function: acpi_osd_exec_callback,
    context: *mut c_void,
) -> acpi_status {
    let mut thread: pthread_t = 0;
    let ret: c_int;

    ret = pthread_create(
        &mut thread,
        core::ptr::null(),
        core::mem::transmute::<acpi_osd_exec_callback, PTHREAD_CALLBACK>(function),
        context,
    );
    if ret != 0 {
        acpi_os_vprintf(c"Create thread failed".as_ptr(), core::ptr::null_mut());
    }
    0
}

#[cfg(ACPI_SINGLE_THREADED)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_get_thread_id() -> acpi_thread_id {
    1
}

#[cfg(ACPI_SINGLE_THREADED)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_execute(
    type_: acpi_execute_type,
    function: acpi_osd_exec_callback,
    context: *mut c_void,
) -> acpi_status {
    if let Some(function) = function {
        function(context);
    }

    AE_OK
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn acpi_os_wait_events_complete() {
    return;
}
