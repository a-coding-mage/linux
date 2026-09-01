// SPDX-License-Identifier: GPL-2.0
/*
 * User-space helper to filter page_owner output per-fd
 *
 * Example use:
 *   ./page_owner_filter -m handle
 *   ./page_owner_filter -m stack_handle
 *   ./page_owner_filter -n 0,1,2
 *
 * See Documentation/mm/page_owner.rst
 */

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

const MAX_CMD_LEN: usize = 512;
const EOF: c_int = -1;
const F_OK: c_int = 0;
const O_RDWR: c_int = 2;
const EACCES: c_int = 13;
const EPERM: c_int = 1;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const EPIPE: c_int = 32;
const SIGPIPE: c_int = 13;
const SIG_IGN: usize = 1;
const REQUIRED_ARGUMENT: c_int = 1;
const NO_ARGUMENT: c_int = 0;

type FILE = c_void;
type SsizeT = isize;
type SizeT = usize;
type SignalHandler = usize;

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut errno: c_int;
    static mut optarg: *mut c_char;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> SizeT;
    fn snprintf(s: *mut c_char, n: SizeT, format: *const c_char, ...) -> c_int;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: SizeT) -> SsizeT;
    fn write(fd: c_int, buf: *const c_void, count: SizeT) -> SsizeT;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fwrite(ptr: *const c_void, size: SizeT, nmemb: SizeT, stream: *mut FILE) -> SizeT;
    fn fflush(stream: *mut FILE) -> c_int;
    fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn signal(signum: c_int, handler: SignalHandler) -> SignalHandler;
    fn __ctype_b_loc() -> *mut *const u16;
}

unsafe fn isdigit(c: u8) -> bool {
    const _ISDIGIT: u16 = 1 << 11;
    let table = *__ctype_b_loc();
    (*table.add(c as usize) & _ISDIGIT) != 0
}

unsafe fn usage(prog: *const c_char) {
    fprintf(stderr, c"Usage: %s [OPTIONS]\n".as_ptr(), prog);
    fprintf(stderr, c"\nOptions:\n".as_ptr());
    fprintf(stderr, c"  -m, --mode MODE      : print_mode (stack, handle, or stack_handle)\n".as_ptr());
    fprintf(stderr, c"  -n, --nid NID_LIST   : NUMA node IDs (comma-separated or ranges)\n".as_ptr());
    fprintf(stderr, c"  -o, --output FILE    : output file (default: stdout)\n".as_ptr());
    fprintf(stderr, c"  -h, --help           : show this help message\n".as_ptr());
    fprintf(stderr, c"\nExamples:\n".as_ptr());
    fprintf(stderr, c"  %s -m stack\n".as_ptr(), prog);
    fprintf(stderr, c"  %s -m handle\n".as_ptr(), prog);
    fprintf(stderr, c"  %s -m stack_handle\n".as_ptr(), prog);
    fprintf(stderr, c"  %s -m stack -o output.txt\n".as_ptr(), prog);
    fprintf(stderr, c"  %s -n 0,1,2\n".as_ptr(), prog);
    fprintf(stderr, c"  %s -m stack -n 0\n".as_ptr(), prog);
}

unsafe fn validate_mode(mode: *const c_char) -> c_int {
    if strcmp(mode, c"stack".as_ptr()) == 0
        || strcmp(mode, c"handle".as_ptr()) == 0
        || strcmp(mode, c"stack_handle".as_ptr()) == 0
    {
        return 0;
    }

    fprintf(stderr, c"Error: Invalid mode '%s'\n".as_ptr(), mode);
    fprintf(stderr, c"Valid modes: stack, handle, stack_handle\n".as_ptr());
    -1
}

unsafe fn validate_nid_list(nid_list: *const c_char) -> c_int {
    let mut p: *const c_char;
    let mut i: c_int = 0;
    let mut has_digit: c_int = 0;
    let mut in_range: c_int = 0;
    let mut prev_num: c_int = 0;
    let mut curr_num: c_int = 0;

    if nid_list.is_null() || strlen(nid_list) == 0 {
        return 0;
    }

    p = nid_list;
    while *p != 0 {
        if *p as c_int == ',' as c_int {
            if has_digit == 0 {
                fprintf(stderr, c"Error: Invalid nid_list format\n".as_ptr());
                return -1;
            }
            if in_range != 0 && prev_num > curr_num {
                fprintf(
                    stderr,
                    c"Error: Invalid range %d-%d (start must be <= end)\n".as_ptr(),
                    prev_num,
                    curr_num,
                );
                return -1;
            }
            i = 0;
            has_digit = 0;
            in_range = 0;
            prev_num = 0;
            curr_num = 0;
            p = p.add(1);
            continue;
        }

        if *p as c_int == '-' as c_int {
            if has_digit == 0 {
                fprintf(stderr, c"Error: Invalid nid_list format ".as_ptr());
                fprintf(stderr, c"(dash without preceding number)\n".as_ptr());
                return -1;
            }
            if in_range != 0 {
                fprintf(stderr, c"Error: Multiple dashes in nid_list\n".as_ptr());
                return -1;
            }
            prev_num = curr_num;
            curr_num = 0;
            i = 0;
            has_digit = 0;
            in_range = 1;
            p = p.add(1);
            continue;
        }

        if !isdigit(*p as u8) {
            fprintf(stderr, c"Error: Invalid character '%c' in nid_list\n".as_ptr(), *p as c_int);
            return -1;
        }

        if i > 5 {
            fprintf(stderr, c"Error: NID too long (max 65536)\n".as_ptr());
            return -1;
        }
        curr_num = curr_num * 10 + (*p as c_int - '0' as c_int);
        i += 1;
        has_digit = 1;
        p = p.add(1);
    }

    if has_digit == 0 {
        fprintf(stderr, c"Error: Invalid nid_list format\n".as_ptr());
        return -1;
    }

    if in_range != 0 && prev_num > curr_num {
        fprintf(
            stderr,
            c"Error: Invalid range %d-%d (start must be <= end)\n".as_ptr(),
            prev_num,
            curr_num,
        );
        return -1;
    }

    0
}

unsafe fn c_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut output_file: *const c_char = ptr::null();
    let mut filter_cmd = [0 as c_char; MAX_CMD_LEN];
    let mut output: *mut FILE = ptr::null_mut();
    let mut fd: c_int = -1;
    let mut ret: SsizeT;
    let mut buf = [0 as c_char; 4096];
    let mut opt: c_int;
    let mut cmd_len: SizeT = 0;

    signal(SIGPIPE, SIG_IGN);

    let long_options = [
        option {
            name: c"mode".as_ptr(),
            has_arg: REQUIRED_ARGUMENT,
            flag: ptr::null_mut(),
            val: 'm' as c_int,
        },
        option {
            name: c"nid".as_ptr(),
            has_arg: REQUIRED_ARGUMENT,
            flag: ptr::null_mut(),
            val: 'n' as c_int,
        },
        option {
            name: c"output".as_ptr(),
            has_arg: REQUIRED_ARGUMENT,
            flag: ptr::null_mut(),
            val: 'o' as c_int,
        },
        option {
            name: c"help".as_ptr(),
            has_arg: NO_ARGUMENT,
            flag: ptr::null_mut(),
            val: 'h' as c_int,
        },
        option {
            name: ptr::null(),
            has_arg: 0,
            flag: ptr::null_mut(),
            val: 0,
        },
    ];

    filter_cmd[0] = 0;

    if argc > 1 {
        let mut i: c_int = 1;
        while i < argc {
            if strcmp(*argv.add(i as usize), c"-h".as_ptr()) == 0
                || strcmp(*argv.add(i as usize), c"--help".as_ptr()) == 0
            {
                usage(*argv);
                return 0;
            }
            i += 1;
        }
    }

    /* Check if page_owner exists and is readable */
    if access(c"/sys/kernel/debug/page_owner".as_ptr(), F_OK) != 0 {
        if errno == ENOENT {
            fprintf(stderr, c"Error: /sys/kernel/debug/page_owner does not exist\n".as_ptr());
        } else {
            perror(c"Error accessing /sys/kernel/debug/page_owner".as_ptr());
        }
        fprintf(stderr, c"Make sure page_owner is enabled in kernel\n".as_ptr());
        return 1;
    }

    loop {
        opt = getopt_long(
            argc,
            argv,
            c"m:n:o:h".as_ptr(),
            long_options.as_ptr(),
            ptr::null_mut(),
        );
        if opt == EOF {
            break;
        }
        let len: c_int;

        match opt {
            x if x == 'm' as c_int => {
                let mode = optarg as *const c_char;

                if validate_mode(mode) < 0 {
                    return 1;
                }
                len = snprintf(
                    filter_cmd.as_mut_ptr().add(cmd_len),
                    MAX_CMD_LEN - cmd_len,
                    c"%smode=%s".as_ptr(),
                    if cmd_len > 0 { c" ".as_ptr() } else { c"".as_ptr() },
                    mode,
                );
                if len < 0 || cmd_len + len as SizeT >= MAX_CMD_LEN {
                    fprintf(stderr, c"Error: Command too long\n".as_ptr());
                    return 1;
                }
                cmd_len += len as SizeT;
            }
            x if x == 'n' as c_int => {
                let nid_list = optarg as *const c_char;

                if validate_nid_list(nid_list) < 0 {
                    return 1;
                }
                len = snprintf(
                    filter_cmd.as_mut_ptr().add(cmd_len),
                    MAX_CMD_LEN - cmd_len,
                    c"%snid=%s".as_ptr(),
                    if cmd_len > 0 { c" ".as_ptr() } else { c"".as_ptr() },
                    nid_list,
                );
                if len < 0 || cmd_len + len as SizeT >= MAX_CMD_LEN {
                    fprintf(stderr, c"Error: Command too long\n".as_ptr());
                    return 1;
                }
                cmd_len += len as SizeT;
            }
            x if x == 'o' as c_int => {
                output_file = optarg as *const c_char;
            }
            x if x == 'h' as c_int => {
                /* Already handled above */
            }
            _ => {
                usage(*argv);
                return 1;
            }
        }
    }

    /* At least one filter must be specified */
    if cmd_len == 0 {
        fprintf(
            stderr,
            c"Error: At least one filter (-m or -n) must be specified\n\n".as_ptr(),
        );
        usage(*argv);
        return 1;
    }

    /* Open page_owner for read-write - this will fail if kernel doesn't support write */
    fd = open(c"/sys/kernel/debug/page_owner".as_ptr(), O_RDWR);
    if fd < 0 {
        if errno == EACCES || errno == EPERM {
            fprintf(stderr, c"Error: /sys/kernel/debug/page_owner ".as_ptr());
            fprintf(stderr, c"does not support write access\n".as_ptr());
            fprintf(stderr, c"This kernel does not support ".as_ptr());
            fprintf(stderr, c"per-fd filtering.\n".as_ptr());
            fprintf(stderr, c"Please ensure you have a kernel with ".as_ptr());
            fprintf(stderr, c"per-fd filtering support.\n".as_ptr());
        } else {
            perror(c"Error opening /sys/kernel/debug/page_owner".as_ptr());
        }
        return 1;
    }

    if !output_file.is_null() {
        output = fopen(output_file, c"w".as_ptr());
        if output.is_null() {
            perror(c"open output file".as_ptr());
            close(fd);
            return 1;
        }
    } else {
        output = stdout;
    }

    ret = write(
        fd,
        filter_cmd.as_ptr() as *const c_void,
        strlen(filter_cmd.as_ptr()),
    );

    if ret < 0 {
        if errno == EINVAL {
            fprintf(stderr, c"Error: Kernel rejected the filter command.\n".as_ptr());
            fprintf(stderr, c"Possible causes:\n".as_ptr());
            fprintf(stderr, c"  - Kernel does not support per-fd filtering\n".as_ptr());
            fprintf(stderr, c"  - NUMA node has no memory\n".as_ptr());
            fprintf(stderr, c"  - Unknown reason\n".as_ptr());
        } else {
            perror(c"write filter command".as_ptr());
        }
        close(fd);
        if output != stdout {
            fclose(output);
        }
        return if ret < 0 { 1 } else { 0 };
    }

    if ret as SizeT != strlen(filter_cmd.as_ptr()) {
        fprintf(
            stderr,
            c"Warning: Partial write (%zd/%zu)\n".as_ptr(),
            ret,
            strlen(filter_cmd.as_ptr()),
        );
    }

    /* Read and display filtered output */
    ret = 0;
    loop {
        ret = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
        if ret <= 0 {
            break;
        }
        let written = fwrite(buf.as_ptr() as *const c_void, 1, ret as SizeT, output);

        if written != ret as SizeT {
            if errno == EPIPE {
                /* Pipe closed, treat as success */
                ret = 0;
                close(fd);
                if output != stdout {
                    fclose(output);
                }
                return if ret < 0 { 1 } else { 0 };
            }
            perror(c"write output".as_ptr());
            ret = -1;
            close(fd);
            if output != stdout {
                fclose(output);
            }
            return if ret < 0 { 1 } else { 0 };
        }
    }

    if ret < 0 {
        perror(c"read page_owner".as_ptr());
        close(fd);
        if output != stdout {
            fclose(output);
        }
        return if ret < 0 { 1 } else { 0 };
    }

    if fflush(output) != 0 {
        if errno == EPIPE {
            /* Pipe closed, treat as success */
            ret = 0;
        } else {
            perror(c"flush output".as_ptr());
            ret = -1;
        }
    }

    close(fd);
    if output != stdout {
        fclose(output);
    }
    if ret < 0 { 1 } else { 0 }
}

fn main() {
    let args: Vec<*mut c_char> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
        .collect();
    let mut argv = args;
    argv.push(ptr::null_mut());

    let ret = unsafe { c_main((argv.len() - 1) as c_int, argv.as_mut_ptr()) };

    for arg in argv.into_iter().take_while(|arg| !arg.is_null()) {
        unsafe {
            let _ = std::ffi::CString::from_raw(arg);
        }
    }

    std::process::exit(ret);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
