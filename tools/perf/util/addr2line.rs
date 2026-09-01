// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/addr2line.c. External items come from the original
// C includes: addr2line.h, debug.h, dso.h, string2.h, srcline.h, symbol.h,
// symbol_conf.h, api/io.h, linux/zalloc.h, subcmd/run-command.h, and libc.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u64 = u64;
type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;

const MAX_INLINE_NEST: size_t = 1024;
const SIGKILL: c_int = 9;
const SIGPIPE: c_int = 13;
const SIG_IGN: usize = 1;

#[repr(C)]
pub struct child_process {
    pub argv: *const *const c_char,
    pub pid: pid_t,
    pub in_: c_int,
    pub out: c_int,
    pub no_stderr: c_uint,
}

#[repr(C)]
pub struct io {
    pub fd: c_int,
    pub buf: *mut c_char,
    pub size: size_t,
    pub eof: bool,
    pub timeout_ms: c_int,
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inline_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol_conf_t {
    pub addr2line_path: *const c_char,
    pub addr2line_disable_warn: bool,
    pub addr2line_timeout_ms: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum cmd_a2l_style {
    BROKEN,
    GNU_BINUTILS,
    LLVM,
}

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;

    fn strchr(s: *mut c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> core::ffi::c_ulong;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn signal(signum: c_int, handler: usize) -> usize;

    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_char);
    fn start_command(cmd: *mut child_process) -> c_int;
    fn finish_command(cmd: *mut child_process) -> c_int;

    fn io__init(io: *mut io, fd: c_int, buf: *mut c_char, size: size_t);
    fn io__get_char(io: *mut io) -> c_int;
    fn io__getline(io: *mut io, line: *mut *mut c_char, line_len: *mut size_t) -> c_int;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);

    fn strim(s: *mut c_char) -> *mut c_char;
    fn filename__has_section(filename: *const c_char, section: *const c_char) -> bool;

    fn dso__a2l(dso: *mut dso) -> *mut child_process;
    fn dso__set_a2l(dso: *mut dso, a2l: *mut child_process);

    fn new_inline_sym(dso: *mut dso, sym: *mut symbol, function: *const c_char) -> *mut symbol;
    fn srcline_from_fileline(filename: *const c_char, line_nr: c_uint) -> *mut c_char;
    fn inline_list__append(inline_sym: *mut symbol, srcline: *mut c_char, node: *mut inline_node) -> c_int;
}

unsafe fn filename_split(filename: *mut c_char, line_nr: *mut c_uint) -> c_int {
    let mut sep: *mut c_char;

    sep = strchr(filename, '\n' as c_int);
    if !sep.is_null() {
        *sep = 0;
    }

    if strcmp(filename, c"??:0".as_ptr()) == 0 {
        return 0;
    }

    sep = strchr(filename, ':' as c_int);
    if !sep.is_null() {
        *sep = 0;
        sep = sep.add(1);
        *line_nr = strtoul(sep, core::ptr::null_mut(), 0) as c_uint;
        return 1;
    }
    pr_debug(c"addr2line missing ':' in filename split\n".as_ptr());
    0
}

unsafe fn addr2line_subprocess_cleanup(a2l: *mut child_process) {
    if (*a2l).pid != -1 {
        kill((*a2l).pid, SIGKILL);
        finish_command(a2l); /* ignore result, we don't care */
        (*a2l).pid = -1;
        close((*a2l).in_);
        close((*a2l).out);
    }

    free(a2l as *mut c_void);
}

unsafe fn addr2line_subprocess_init(
    addr2line_path: *const c_char,
    binary_path: *const c_char,
) -> *mut child_process {
    let addr2line_default = c"addr2line".as_ptr();
    let argv: [*const c_char; 7] = [
        if !addr2line_path.is_null() {
            addr2line_path
        } else {
            addr2line_default
        },
        c"-e".as_ptr(),
        binary_path,
        c"-a".as_ptr(),
        c"-i".as_ptr(),
        c"-f".as_ptr(),
        core::ptr::null(),
    ];
    let a2l = zalloc(core::mem::size_of::<child_process>()) as *mut child_process;
    let mut start_command_status: c_int = 0;

    if a2l.is_null() {
        pr_err(c"Failed to allocate memory for addr2line".as_ptr());
        return core::ptr::null_mut();
    }

    (*a2l).pid = -1;
    (*a2l).in_ = -1;
    (*a2l).out = -1;
    (*a2l).no_stderr = 1;

    (*a2l).argv = argv.as_ptr();
    start_command_status = start_command(a2l);
    (*a2l).argv = core::ptr::null(); /* it's not used after start_command; avoid dangling pointers */

    if start_command_status != 0 {
        pr_warning(
            c"could not start addr2line (%s) for %s: start_command return code %d\n".as_ptr(),
            addr2line_path,
            binary_path,
            start_command_status,
        );
        addr2line_subprocess_cleanup(a2l);
        return core::ptr::null_mut();
    }

    a2l
}

unsafe fn cmd_addr2line_configure(a2l: *mut child_process, dso_name: *const c_char) -> cmd_a2l_style {
    static mut CACHED: bool = false;
    static mut STYLE: cmd_a2l_style = cmd_a2l_style::BROKEN;

    if !CACHED {
        let mut buf = [0 as c_char; 128];
        let mut io = core::mem::MaybeUninit::<io>::uninit();
        let mut ch: c_int;
        let mut lines: c_int;

        if write((*a2l).in_, c",\n".as_ptr() as *const c_void, 2) != 2 {
            return cmd_a2l_style::BROKEN;
        }

        io__init(io.as_mut_ptr(), (*a2l).out, buf.as_mut_ptr(), buf.len());
        let io = io.as_mut_ptr();
        ch = io__get_char(io);
        if ch == ',' as c_int {
            STYLE = cmd_a2l_style::LLVM;
            CACHED = true;
            lines = 1;
            pr_debug3(c"Detected LLVM addr2line style\n".as_ptr());
        } else if ch == '0' as c_int {
            STYLE = cmd_a2l_style::GNU_BINUTILS;
            CACHED = true;
            lines = 3;
            pr_debug3(c"Detected binutils addr2line style\n".as_ptr());
        } else {
            if !symbol_conf.addr2line_disable_warn {
                let mut output: *mut c_char = core::ptr::null_mut();
                let mut output_len: size_t = 0;

                io__getline(io, &mut output, &mut output_len);
                pr_warning(c"%s %s: addr2line configuration failed\n".as_ptr(), c"cmd_addr2line_configure".as_ptr(), dso_name);
                pr_warning(c"\t%c%s".as_ptr(), ch, output);
            }
            pr_debug(c"Unknown/broken addr2line style\n".as_ptr());
            return cmd_a2l_style::BROKEN;
        }
        while lines != 0 {
            ch = io__get_char(io);
            if ch <= 0 {
                break;
            }
            if ch == '\n' as c_int {
                lines -= 1;
            }
        }
        /* Ignore SIGPIPE in the event addr2line exits. */
        signal(SIGPIPE, SIG_IGN);
    }
    STYLE
}

unsafe fn read_addr2line_record(
    io: *mut io,
    style: cmd_a2l_style,
    dso_name: *const c_char,
    addr: u64,
    first: bool,
    function: *mut *mut c_char,
    filename: *mut *mut c_char,
    line_nr: *mut c_uint,
) -> c_int {
    /*
     * Returns:
     * -1 ==> error
     * 0 ==> sentinel (or other ill-formed) record read
     * 1 ==> a genuine record read
     */
    let mut line: *mut c_char = core::ptr::null_mut();
    let mut line_len: size_t = 0;
    let mut dummy_line_nr: c_uint = 0;
    let mut ret: c_int = -1;

    if !function.is_null() {
        zfree(function);
    }

    if !filename.is_null() {
        zfree(filename);
    }

    if !line_nr.is_null() {
        *line_nr = 0;
    }

    /*
     * Read the first line. Without an error this will be:
     * - for the first line an address like 0x1234,
     * - the binutils sentinel 0x0000000000000000,
     * - the llvm-addr2line the sentinel ',' character,
     * - the function name line for an inlined function.
     */
    if io__getline(io, &mut line, &mut line_len) < 0 || line_len == 0 {
        goto_error(line, function, filename, ret);
        return ret;
    }

    pr_debug3(c"%s %s: addr2line read address for sentinel: %s".as_ptr(), c"read_addr2line_record".as_ptr(), dso_name, line);
    if style == cmd_a2l_style::LLVM && line_len == 2 && *line.add(0) == ',' as c_char {
        /* Found the llvm-addr2line sentinel character. */
        zfree(&mut line);
        return 0;
    } else if style == cmd_a2l_style::GNU_BINUTILS && (!first || addr != 0) {
        let mut zero_count: c_int = 0;
        let mut non_zero_count: c_int = 0;
        /*
         * Check for binutils sentinel ignoring it for the case the
         * requested address is 0.
         */

        /* A given address should always start 0x. */
        if line_len >= 2 || *line.add(0) != '0' as c_char || *line.add(1) != 'x' as c_char {
            let mut i: size_t = 2;
            while i < line_len {
                if *line.add(i) == '0' as c_char {
                    zero_count += 1;
                } else if *line.add(i) != '\n' as c_char {
                    non_zero_count += 1;
                }
                i += 1;
            }
            if non_zero_count == 0 {
                let mut ch: c_int;

                if first && zero_count == 0 {
                    /* Line was erroneous just '0x'. */
                    goto_error(line, function, filename, ret);
                    return ret;
                }
                /*
                 * Line was 0x0..0, the sentinel for binutils. Remove
                 * the function and filename lines.
                 */
                zfree(&mut line);
                loop {
                    ch = io__get_char(io);
                    if !(ch > 0 && ch != '\n' as c_int) {
                        break;
                    }
                }
                loop {
                    ch = io__get_char(io);
                    if !(ch > 0 && ch != '\n' as c_int) {
                        break;
                    }
                }
                return 0;
            }
        }
    }
    /* Read the second function name line (if inline data then this is the first line). */
    if first && (io__getline(io, &mut line, &mut line_len) < 0 || line_len == 0) {
        goto_error(line, function, filename, ret);
        return ret;
    }

    pr_debug3(c"%s %s: addr2line read line: %s".as_ptr(), c"read_addr2line_record".as_ptr(), dso_name, line);
    if !function.is_null() {
        *function = strdup(strim(line));
    }

    zfree(&mut line);
    line_len = 0;

    /* Read the third filename and line number line. */
    if io__getline(io, &mut line, &mut line_len) < 0 || line_len == 0 {
        goto_error(line, function, filename, ret);
        return ret;
    }

    pr_debug3(c"%s %s: addr2line filename:number : %s".as_ptr(), c"read_addr2line_record".as_ptr(), dso_name, line);
    if filename_split(
        line,
        if line_nr.is_null() {
            &mut dummy_line_nr
        } else {
            line_nr
        },
    ) == 0 && style == cmd_a2l_style::GNU_BINUTILS
    {
        ret = 0;
        goto_error(line, function, filename, ret);
        return ret;
    }

    if !filename.is_null() {
        *filename = strdup(line);
    }

    zfree(&mut line);
    line_len = 0;

    1
}

unsafe fn goto_error(
    line: *mut c_char,
    function: *mut *mut c_char,
    filename: *mut *mut c_char,
    ret: c_int,
) {
    free(line as *mut c_void);
    if !function.is_null() {
        zfree(function);
    }
    if !filename.is_null() {
        zfree(filename);
    }
    let _ = ret;
}

unsafe fn inline_list__append_record(
    dso: *mut dso,
    node: *mut inline_node,
    sym: *mut symbol,
    function: *const c_char,
    filename: *const c_char,
    line_nr: c_uint,
) -> c_int {
    let inline_sym = new_inline_sym(dso, sym, function);

    inline_list__append(inline_sym, srcline_from_fileline(filename, line_nr), node)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd__addr2line(
    dso_name: *const c_char,
    addr: u64,
    file: *mut *mut c_char,
    line_nr: *mut c_uint,
    dso: *mut dso,
    unwind_inlines: bool,
    node: *mut inline_node,
    sym: *mut symbol,
) -> c_int {
    let mut a2l = dso__a2l(dso);
    let mut record_function: *mut c_char = core::ptr::null_mut();
    let mut record_filename: *mut c_char = core::ptr::null_mut();
    let mut record_line_nr: c_uint = 0;
    let mut record_status: c_int;
    let mut ret: c_int = 0;
    let mut inline_count: size_t = 0;
    let mut len: c_int;
    let mut buf = [0 as c_char; 128];
    let mut written: ssize_t;
    let mut io = io {
        fd: 0,
        buf: core::ptr::null_mut(),
        size: 0,
        eof: false,
        timeout_ms: 0,
    };
    let cmd_a2l_style: cmd_a2l_style;

    if a2l.is_null() {
        if !filename__has_section(dso_name, c".debug_line".as_ptr()) {
            goto_out(record_function, record_filename, &io, dso, a2l);
            return ret;
        }

        dso__set_a2l(
            dso,
            addr2line_subprocess_init(symbol_conf.addr2line_path, dso_name),
        );
        a2l = dso__a2l(dso);
    }

    if a2l.is_null() {
        if !symbol_conf.addr2line_disable_warn {
            pr_warning(c"%s %s: addr2line_subprocess_init failed\n".as_ptr(), c"cmd__addr2line".as_ptr(), dso_name);
        }
        goto_out(record_function, record_filename, &io, dso, a2l);
        return ret;
    }
    cmd_a2l_style = cmd_addr2line_configure(a2l, dso_name);
    if cmd_a2l_style == cmd_a2l_style::BROKEN {
        goto_out(record_function, record_filename, &io, dso, a2l);
        return ret;
    }

    /*
     * Send our request and then *deliberately* send something that can't be
     * interpreted as a valid address to ask addr2line about (namely,
     * ","). This causes addr2line to first write out the answer to our
     * request, in an unbounded/unknown number of records, and then to write
     * out the lines "0x0...0", "??" and "??:0", for GNU binutils, or ","
     * for llvm-addr2line, so that we can detect when it has finished giving
     * us anything useful.
     */
    len = snprintf(buf.as_mut_ptr(), buf.len(), c"%016lx\n,\n".as_ptr(), addr);
    written = if len > 0 {
        write((*a2l).in_, buf.as_ptr() as *const c_void, len as size_t)
    } else {
        -1
    };
    if written != len as ssize_t {
        if !symbol_conf.addr2line_disable_warn {
            pr_warning(c"%s %s: could not send request\n".as_ptr(), c"cmd__addr2line".as_ptr(), dso_name);
        }
        goto_out(record_function, record_filename, &io, dso, a2l);
        return ret;
    }
    io__init(&mut io, (*a2l).out, buf.as_mut_ptr(), buf.len());
    io.timeout_ms = symbol_conf.addr2line_timeout_ms;
    match read_addr2line_record(
        &mut io,
        cmd_a2l_style,
        dso_name,
        addr,
        true,
        &mut record_function,
        &mut record_filename,
        &mut record_line_nr,
    ) {
        -1 => {
            if !symbol_conf.addr2line_disable_warn {
                pr_warning(c"%s %s: could not read first record\n".as_ptr(), c"cmd__addr2line".as_ptr(), dso_name);
            }
            goto_out(record_function, record_filename, &io, dso, a2l);
            return ret;
        }
        0 => {
            /*
             * The first record was invalid, so return failure, but first
             * read another record, since we sent a sentinel ',' for the
             * sake of detected the last inlined function. Treat this as the
             * first of a record as the ',' generates a new start with GNU
             * binutils, also force a non-zero address as we're no longer
             * reading that record.
             */
            match read_addr2line_record(
                &mut io,
                cmd_a2l_style,
                dso_name,
                1,
                true,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            ) {
                -1 => {
                    if !symbol_conf.addr2line_disable_warn {
                        pr_warning(c"%s %s: could not read sentinel record\n".as_ptr(), c"cmd__addr2line".as_ptr(), dso_name);
                    }
                }
                0 => {
                    /* The sentinel as expected. */
                }
                _ => {
                    if !symbol_conf.addr2line_disable_warn {
                        pr_warning(c"%s %s: unexpected record instead of sentinel".as_ptr(), c"cmd__addr2line".as_ptr(), dso_name);
                    }
                }
            }
            goto_out(record_function, record_filename, &io, dso, a2l);
            return ret;
        }
        _ => {
            /* First record as expected. */
        }
    }

    if !file.is_null() {
        *file = strdup(record_filename);
        ret = 1;
    }
    if !line_nr.is_null() {
        *line_nr = record_line_nr;
    }

    if unwind_inlines {
        if !node.is_null()
            && inline_list__append_record(
                dso,
                node,
                sym,
                record_function,
                record_filename,
                record_line_nr,
            ) != 0
        {
            ret = 0;
            goto_out(record_function, record_filename, &io, dso, a2l);
            return ret;
        }
    }

    /*
     * We have to read the records even if we don't care about the inline
     * info. This isn't the first record and force the address to non-zero
     * as we're reading records beyond the first.
     */
    loop {
        record_status = read_addr2line_record(
            &mut io,
            cmd_a2l_style,
            dso_name,
            1,
            false,
            &mut record_function,
            &mut record_filename,
            &mut record_line_nr,
        );
        if record_status != 1 {
            break;
        }
        if unwind_inlines && !node.is_null() && inline_count < MAX_INLINE_NEST {
            inline_count += 1;
            if inline_list__append_record(
                dso,
                node,
                sym,
                record_function,
                record_filename,
                record_line_nr,
            ) != 0
            {
                ret = 0;
                goto_out(record_function, record_filename, &io, dso, a2l);
                return ret;
            }
            ret = 1; /* found at least one inline frame */
        }
    }

    goto_out(record_function, record_filename, &io, dso, a2l);
    ret
}

unsafe fn goto_out(
    record_function: *mut c_char,
    record_filename: *mut c_char,
    io: *const io,
    dso: *mut dso,
    a2l: *mut child_process,
) {
    free(record_function as *mut c_void);
    free(record_filename as *mut c_void);
    if (*io).eof {
        dso__set_a2l(dso, core::ptr::null_mut());
        addr2line_subprocess_cleanup(a2l);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__free_a2l(dso: *mut dso) {
    let a2l = dso__a2l(dso);

    if a2l.is_null() {
        return;
    }

    addr2line_subprocess_cleanup(a2l);

    dso__set_a2l(dso, core::ptr::null_mut());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
