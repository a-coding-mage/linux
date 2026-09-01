/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * minimal stdio function definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* Removed C includes/header guard. This translation expects nolibc dependencies
 * such as std, arch, errno, fcntl, types, sys, stdarg, stdlib, string, and
 * compiler symbols to be supplied by surrounding translated files.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_uint, c_void, VaListImpl};

pub type size_t = usize;
pub type ssize_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type off_t = isize;
pub type uintmax_t = u64;
pub type intmax_t = i64;

unsafe extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, mode: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn lseek(fd: c_int, offset: c_long, whence: c_int) -> off_t;
    fn strlen(s: *const c_char) -> size_t;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strtoll(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn isspace(c: c_int) -> c_int;
    fn i64toa_r(in_: c_long, buffer: *mut c_char) -> *mut c_char;
    fn _nolibc_u64toa_base(in_: u64, buffer: *mut c_char, base: c_uint, recip: u64) -> c_int;
}

unsafe fn SET_ERRNO(e: c_int) {
    errno = e;
}

unsafe fn _NOLIBC_OPTIMIZER_HIDE_VAR<T>(_v: T) {}

const STDIN_FILENO: c_int = 0;
const STDOUT_FILENO: c_int = 1;
const STDERR_FILENO: c_int = 2;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const O_CREAT: c_int = 0o100;
const O_TRUNC: c_int = 0o1000;
const O_APPEND: c_int = 0o2000;
const EBADF: c_int = 9;
const EINVAL: c_int = 22;
const EILSEQ: c_int = 84;
const ERANGE: c_int = 34;
const SIZE_MAX: size_t = size_t::MAX;

pub const EOF: c_int = -1;

/* Buffering mode used by setvbuf.  */
pub const _IOFBF: c_int = 0; /* Fully buffered. */
pub const _IOLBF: c_int = 1; /* Line buffered. */
pub const _IONBF: c_int = 2; /* No buffering. */

/* just define FILE as a non-empty type. The value of the pointer gives
 * the FD: FILE=~fd for fd>=0 or NULL for fd<0. This way positive FILE
 * are immediately identified as abnormal entries (i.e. possible copies
 * of valid pointers to something else).
 */
#[repr(C)]
pub struct FILE {
    pub dummy: [c_char; 1],
}

pub const stdin: *mut FILE = !(STDIN_FILENO as intptr_t) as *mut FILE;
pub const stdout: *mut FILE = !(STDOUT_FILENO as intptr_t) as *mut FILE;
pub const stderr: *mut FILE = !(STDERR_FILENO as intptr_t) as *mut FILE;

/* provides a FILE* equivalent of fd. The mode is ignored. */
pub unsafe fn fdopen(fd: c_int, _mode: *const c_char) -> *mut FILE {
    if fd < 0 {
        SET_ERRNO(EBADF);
        return core::ptr::null_mut();
    }
    !(fd as intptr_t) as *mut FILE
}

pub unsafe fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE {
    let mut flags: c_int;
    let fd: c_int;

    match *mode {
        b'r' => flags = O_RDONLY,
        b'w' => flags = O_WRONLY | O_CREAT | O_TRUNC,
        b'a' => flags = O_WRONLY | O_CREAT | O_APPEND,
        _ => {
            SET_ERRNO(EINVAL);
            return core::ptr::null_mut();
        }
    }

    if *mode.add(1) == b'+' as c_char {
        flags = (flags & !(O_RDONLY | O_WRONLY)) | O_RDWR;
    }

    fd = open(pathname, flags, 0o666);
    fdopen(fd, mode)
}

/* provides the fd of stream. */
pub unsafe fn fileno(stream: *mut FILE) -> c_int {
    let i: intptr_t = stream as intptr_t;

    if i >= 0 {
        SET_ERRNO(EBADF);
        return -1;
    }
    !i as c_int
}

/* flush a stream. */
pub unsafe fn fflush(stream: *mut FILE) -> c_int {
    let i: intptr_t = stream as intptr_t;

    /* NULL is valid here. */
    if i > 0 {
        SET_ERRNO(EBADF);
        return -1;
    }

    /* Don't do anything, nolibc does not support buffering. */
    0
}

/* flush a stream. */
pub unsafe fn fclose(stream: *mut FILE) -> c_int {
    let i: intptr_t = stream as intptr_t;

    if i >= 0 {
        SET_ERRNO(EBADF);
        return -1;
    }

    if close(!i as c_int) != 0 {
        return EOF;
    }

    0
}

/* getc(), fgetc(), getchar() */

pub unsafe fn getc(stream: *mut FILE) -> c_int {
    fgetc(stream)
}

pub unsafe fn fgetc(stream: *mut FILE) -> c_int {
    let mut ch: u8 = 0;

    if read(fileno(stream), &mut ch as *mut u8 as *mut c_void, 1) <= 0 {
        return EOF;
    }
    ch as c_int
}

pub unsafe fn getchar() -> c_int {
    fgetc(stdin)
}

/* putc(), fputc(), putchar() */

pub unsafe fn putc(c: c_int, stream: *mut FILE) -> c_int {
    fputc(c, stream)
}

pub unsafe fn fputc(c: c_int, stream: *mut FILE) -> c_int {
    let ch: u8 = c as u8;

    if write(fileno(stream), &ch as *const u8 as *const c_void, 1) <= 0 {
        return EOF;
    }
    ch as c_int
}

pub unsafe fn putchar(c: c_int) -> c_int {
    fputc(c, stdout)
}

/* fwrite(), fread(), puts(), fputs(). Note that puts() emits '\n' but not fputs(). */

/* internal fwrite()-like function which only takes a size and returns 0 on
 * success or EOF on error. It automatically retries on short writes.
 */
pub unsafe fn _fwrite(mut buf: *const c_void, mut size: size_t, stream: *mut FILE) -> c_int {
    let mut ret: ssize_t;
    let fd: c_int = fileno(stream);

    while size != 0 {
        ret = write(fd, buf, size);
        if ret <= 0 {
            return EOF;
        }
        size -= ret as size_t;
        buf = (buf as *const u8).add(ret as size_t) as *const c_void;
    }
    0
}

pub unsafe fn fwrite(mut s: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t {
    let mut written: size_t = 0;

    while written < nmemb {
        if _fwrite(s, size, stream) != 0 {
            break;
        }
        s = (s as *const u8).add(size) as *const c_void;
        written += 1;
    }
    written
}

/* internal fread()-like function which only takes a size and returns 0 on
 * success or EOF on error. It automatically retries on short reads.
 */
pub unsafe fn _fread(mut buf: *mut c_void, mut size: size_t, stream: *mut FILE) -> c_int {
    let fd: c_int = fileno(stream);
    let mut ret: ssize_t;

    while size != 0 {
        ret = read(fd, buf, size);
        if ret <= 0 {
            return EOF;
        }
        size -= ret as size_t;
        buf = (buf as *mut u8).add(ret as size_t) as *mut c_void;
    }
    0
}

pub unsafe fn fread(mut s: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t {
    let mut nread: size_t = 0;

    while nread < nmemb {
        if _fread(s, size, stream) != 0 {
            break;
        }
        s = (s as *mut u8).add(size) as *mut c_void;
        nread += 1;
    }
    nread
}

pub unsafe fn fputs(s: *const c_char, stream: *mut FILE) -> c_int {
    _fwrite(s as *const c_void, strlen(s), stream)
}

pub unsafe fn puts(s: *const c_char) -> c_int {
    if fputs(s, stdout) == EOF {
        return EOF;
    }
    putchar(b'\n' as c_int)
}

/* fgets() */
pub unsafe fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char {
    let mut ofs: c_int = 0;
    let mut c: c_int;

    while ofs + 1 < size {
        c = fgetc(stream);
        if c == EOF {
            break;
        }
        *s.add(ofs as usize) = c as c_char;
        ofs += 1;
        if c == b'\n' as c_int {
            break;
        }
    }
    if ofs < size {
        *s.add(ofs as usize) = 0;
    }
    if ofs != 0 { s } else { core::ptr::null_mut() }
}

/* fseek */
pub unsafe fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int {
    let fd: c_int = fileno(stream);
    let ret: off_t;

    ret = lseek(fd, offset, whence);

    /* lseek() and fseek() differ in that lseek returns the new
     * position or -1, fseek() returns either 0 or -1.
     */
    if ret >= 0 {
        return 0;
    }

    -1
}

/* printf(). Supports most of the normal integer and string formats.
 *  - %[#0-+ ][width|*[.precision|*}][{l,t,z,ll,L,j,q}]{c,d,i,u,o,x,X,p,s,m,%}
 *  - %% generates a single %
 *  - %m outputs strerror(errno).
 *  - %X outputs a..f the same as %x.
 *  - No support for floating point or wide characters.
 *  - Invalid formats are copied to the output buffer.
 *
 * Called by vfprintf() and snprintf() to do the actual formatting.
 * The callers provide a callback function to save the formatted data.
 * The callback function is called multiple times:
 *  - for each group of literal characters in the format string.
 *  - for field padding.
 *  - for each conversion specifier.
 *  - with (NULL, 0) at the end of the __nolibc_printf.
 * If the callback returns non-zero __nolibc_printf() immediately returns -1.
 */

pub type __nolibc_printf_cb = unsafe fn(state: *mut c_void, buf: *const c_char, size: size_t) -> c_int;

unsafe fn _NOLIBC_PF_FLAG(ch: c_int) -> c_uint {
    1u32 << ((ch as c_uint) & 0x1f)
}

unsafe fn _NOLIBC_PF_FLAG_NZ(ch: c_int) -> c_uint {
    if ch != 0 { _NOLIBC_PF_FLAG(ch) } else { 0 }
}

unsafe fn _NOLIBC_PF_FLAGS_CONTAIN_8(flags: c_uint, cmps: [c_int; 8]) -> c_uint {
    flags & (_NOLIBC_PF_FLAG_NZ(cmps[0])
        | _NOLIBC_PF_FLAG_NZ(cmps[1])
        | _NOLIBC_PF_FLAG_NZ(cmps[2])
        | _NOLIBC_PF_FLAG_NZ(cmps[3])
        | _NOLIBC_PF_FLAG_NZ(cmps[4])
        | _NOLIBC_PF_FLAG_NZ(cmps[5])
        | _NOLIBC_PF_FLAG_NZ(cmps[6])
        | _NOLIBC_PF_FLAG_NZ(cmps[7]))
}

unsafe fn _NOLIBC_PF_CHAR_IS_ONE_OF(ch: c_int, cmp_1: c_int, rest: [c_int; 7]) -> c_uint {
    if (ch as c_uint).wrapping_sub((cmp_1 & 0xe0) as c_uint) > 0x1f {
        0
    } else {
        _NOLIBC_PF_FLAGS_CONTAIN_8(_NOLIBC_PF_FLAG(ch), [cmp_1, rest[0], rest[1], rest[2], rest[3], rest[4], rest[5], rest[6]])
    }
}

unsafe fn _NOLIBC_U64TOA_RECIP(base: c_uint) -> u64 {
    0u64.wrapping_sub(1) / base as u64 + 1
}

pub unsafe fn __nolibc_printf(
    cb: __nolibc_printf_cb,
    state: *mut c_void,
    mut fmt: *const c_char,
    mut args: VaListImpl,
) -> c_int {
    let mut ch: c_char;
    let mut v: u64;
    let mut signed_v: i64;
    let mut written: c_int;
    let mut width: c_int;
    let mut precision: c_int = 0;
    let mut len: c_int;
    let mut flags: c_uint;
    let mut ch_flag: c_uint;
    let mut outbuf = [0 as c_char; 2 + 31 + 22 + 1];
    let mut out: *mut c_char;
    let mut outstr: *const c_char;
    let mut sign_prefix: c_uint;
    let mut got_width: c_int;

    written = 0;
    'printf_loop: loop {
        outstr = fmt;
        ch = *fmt;
        fmt = fmt.add(1);
        if ch == 0 {
            break;
        }

        width = 0;
        flags = 0;
        if ch != b'%' as c_char {
            while *fmt != 0 && *fmt != b'%' as c_char {
                fmt = fmt.add(1);
            }
            /* Output characters from the format string. */
            len = fmt.offset_from(outstr) as c_int;
            output(cb, state, outstr, len, &mut written, &mut width, &mut flags)?;
            continue 'printf_loop;
        }

        /* we're in a format sequence */

        /* Conversion flag characters */
        loop {
            ch = *fmt;
            fmt = fmt.add(1);
            ch_flag = _NOLIBC_PF_CHAR_IS_ONE_OF(ch as c_int, b' ' as c_int, [b'#' as c_int, b'+' as c_int, b'-' as c_int, b'0' as c_int, 0, 0, 0]);
            if ch_flag == 0 {
                break;
            }
            flags |= ch_flag;
        }

        /* Width and precision */
        got_width = 0;
        loop {
            if ch == b'*' as c_char {
                precision = args.arg::<c_int>();
                ch = *fmt;
                fmt = fmt.add(1);
            } else {
                precision = 0;
                while ch >= b'0' as c_char && ch <= b'9' as c_char {
                    precision = precision * 10 + (ch as c_int - b'0' as c_int);
                    ch = *fmt;
                    fmt = fmt.add(1);
                }
            }
            if got_width != 0 {
                break;
            }
            width = precision;
            if ch != b'.' as c_char {
                /* Default precision for strings */
                precision = -1;
                break;
            }
            got_width = 1;
        }
        /* A negative width (e.g. from "%*s") requests left justify. */
        if width < 0 {
            width = -width;
            flags |= _NOLIBC_PF_FLAG(b'-' as c_int);
        }

        /* Length modifier.
         * They miss the conversion flags characters " #+-0" so can go into flags.
         * Change both L and ll to j (all always 64bit).
         */
        if ch == b'L' as c_char {
            ch = b'j' as c_char;
        }
        ch_flag = _NOLIBC_PF_CHAR_IS_ONE_OF(ch as c_int, b'l' as c_int, [b't' as c_int, b'z' as c_int, b'j' as c_int, b'q' as c_int, 0, 0, 0]);
        if ch_flag != 0 {
            if ch == b'l' as c_char && *fmt == b'l' as c_char {
                fmt = fmt.add(1);
                ch_flag = _NOLIBC_PF_FLAG(b'j' as c_int);
            }
            flags |= ch_flag;
            ch = *fmt;
            fmt = fmt.add(1);
        }

        ch_flag = _NOLIBC_PF_FLAG(ch as c_int) | ((flags & _NOLIBC_PF_FLAG(b'#' as c_int)) >> 1);
        if (((ch >= b'a' as c_char && ch <= b'z' as c_char) || ch == b'X' as c_char)
            && _NOLIBC_PF_FLAGS_CONTAIN_8(ch_flag, [b'c' as c_int, b'd' as c_int, b'i' as c_int, b'u' as c_int, b'o' as c_int, b'x' as c_int, b'p' as c_int, b's' as c_int]) != 0)
        {
            if _NOLIBC_PF_FLAGS_CONTAIN_8(
                ch_flag | (flags & !_NOLIBC_PF_FLAG(b'p' as c_int)),
                [b'p' as c_int, b's' as c_int, b'l' as c_int, b't' as c_int, b'z' as c_int, 0, 0, 0],
            ) != 0 {
                v = args.arg::<c_ulong>() as u64;
                signed_v = v as c_long as i64;
            } else if _NOLIBC_PF_FLAGS_CONTAIN_8(flags, [b'j' as c_int, b'q' as c_int, 0, 0, 0, 0, 0, 0]) != 0 {
                v = args.arg::<u64>();
                signed_v = v as i64;
            } else {
                v = args.arg::<c_uint>() as u64;
                signed_v = v as c_int as i64;
            }

            if ch == b'c' as c_char {
                /* "%c" - single character. */
                outbuf[0] = v as c_char;
                len = 1;
                outstr = outbuf.as_ptr();
                output(cb, state, outstr, len, &mut written, &mut width, &mut flags)?;
                continue 'printf_loop;
            }

            if ch == b's' as c_char {
                /* "%s" - character string. */
                outstr = v as uintptr_t as *const c_char;
                if outstr.is_null() {
                    outstr = c"(null)".as_ptr();
                    /* Match glibc, nothing output if precision too small */
                    len = if precision < 0 || precision >= 6 { 6 } else { 0 };
                    output(cb, state, outstr, len, &mut written, &mut width, &mut flags)?;
                    continue 'printf_loop;
                }
                len = strnlen_for_printf(outstr, precision);
                output(cb, state, outstr, len, &mut written, &mut width, &mut flags)?;
                continue 'printf_loop;
            }

            sign_prefix = 0;

            if _NOLIBC_PF_FLAGS_CONTAIN_8(ch_flag, [b'd' as c_int, b'i' as c_int, 0, 0, 0, 0, 0, 0]) != 0 {
                /* "%d" and "%i" - signed decimal numbers. */
                if signed_v < 0 {
                    sign_prefix = b'-' as c_uint;
                    v = (-(signed_v + 1)) as u64;
                    v += 1;
                } else if _NOLIBC_PF_FLAGS_CONTAIN_8(flags, [b'+' as c_int, 0, 0, 0, 0, 0, 0, 0]) != 0 {
                    sign_prefix = b'+' as c_uint;
                } else if _NOLIBC_PF_FLAGS_CONTAIN_8(flags, [b' ' as c_int, 0, 0, 0, 0, 0, 0, 0]) != 0 {
                    sign_prefix = b' ' as c_uint;
                }
            } else if _NOLIBC_PF_FLAGS_CONTAIN_8(ch_flag, [b'o' as c_int, 0, 0, 0, 0, 0, 0, 0]) != 0
                && _NOLIBC_PF_FLAGS_CONTAIN_8(ch_flag, [(b'#' - 1) as c_int, 0, 0, 0, 0, 0, 0, 0]) != 0
            {
                sign_prefix = b'0' as c_uint;
            }

            out = outbuf.as_mut_ptr().add(2 + 31);

            if v == 0 {
                if _NOLIBC_PF_FLAGS_CONTAIN_8(ch_flag, [b'p' as c_int, 0, 0, 0, 0, 0, 0, 0]) != 0 {
                    /* "%p" match glibc, precision is ignored */
                    outstr = c"(nil)".as_ptr();
                    len = 5;
                    output(cb, state, outstr, len, &mut written, &mut width, &mut flags)?;
                    continue 'printf_loop;
                }
                if precision == 0 {
                    /* Explicit %nn.0d, no digits output (except for %#.0o) */
                    len = 0;
                } else {
                    /* All other formats (including "%#x") just output "0". */
                    *out = b'0' as c_char;
                    len = 1;
                }
            } else {
                /* Convert the number to ascii in the required base. */
                let recip: u64;
                let base: c_uint;
                if _NOLIBC_PF_FLAGS_CONTAIN_8(ch_flag, [b'd' as c_int, b'i' as c_int, b'u' as c_int, 0, 0, 0, 0, 0]) != 0 {
                    base = 10;
                    recip = _NOLIBC_U64TOA_RECIP(10);
                } else if _NOLIBC_PF_FLAGS_CONTAIN_8(ch_flag, [b'o' as c_int, 0, 0, 0, 0, 0, 0, 0]) != 0 {
                    base = 8;
                    recip = _NOLIBC_U64TOA_RECIP(8);
                } else {
                    base = 16;
                    recip = _NOLIBC_U64TOA_RECIP(16);
                    if _NOLIBC_PF_FLAGS_CONTAIN_8(ch_flag, [b'p' as c_int, (b'#' - 1) as c_int, 0, 0, 0, 0, 0, 0]) != 0 {
                        /* "%p" and "%#x" need "0x" prepending. */
                        sign_prefix = ((b'0' as c_uint) << 8) | b'x' as c_uint;
                    }
                }
                len = _nolibc_u64toa_base(v, out, base, recip);
            }

            if precision < 0 {
                if _NOLIBC_PF_FLAGS_CONTAIN_8(flags, [b'0' as c_int, 0, 0, 0, 0, 0, 0, 0]) == 0
                    || _NOLIBC_PF_FLAGS_CONTAIN_8(flags, [b'-' as c_int, 0, 0, 0, 0, 0, 0, 0]) != 0
                {
                    // no_zero_padding
                } else {
                    precision = width;
                    if sign_prefix != 0 {
                        precision -= 1;
                        if sign_prefix >= 256 {
                            precision -= 1;
                        }
                    }
                }
            }
            if precision > 31 {
                precision = 31;
            }
            while len < precision {
                _NOLIBC_OPTIMIZER_HIDE_VAR(len);
                out = out.sub(1);
                *out = b'0' as c_char;
                len += 1;
            }

            if sign_prefix.wrapping_sub(*out as c_uint) != 0 {
                while sign_prefix != 0 {
                    _NOLIBC_OPTIMIZER_HIDE_VAR(len);
                    len += 1;
                    out = out.sub(1);
                    *out = sign_prefix as c_char;
                    sign_prefix >>= 8;
                }
            }
            outstr = out;
            output(cb, state, outstr, len, &mut written, &mut width, &mut flags)?;
            continue 'printf_loop;
        }

        if ch == b'm' as c_char {
            /* If NOLIBC_IGNORE_ERRNO is defined, use "unknown error"; otherwise strerror(errno). */
            outstr = strerror(errno);
            len = strnlen_for_printf(outstr, precision);
            output(cb, state, outstr, len, &mut written, &mut width, &mut flags)?;
            continue 'printf_loop;
        }

        if ch != b'%' as c_char {
            /* Invalid format: back up to output the format characters */
            fmt = outstr.add(1);
            /* and output a '%' now. */
        }
        len = 1;
        width = 0;
        outstr = fmt.sub(1);
        output(cb, state, outstr, len, &mut written, &mut width, &mut flags)?;
    }

    /* Request a final '\0' be added to the snprintf() output.
     * This may be the only call of the cb() function.
     */
    if cb(state, core::ptr::null(), 0) != 0 {
        return -1;
    }

    written
}

unsafe fn strnlen_for_printf(outstr: *const c_char, precision: c_int) -> c_int {
    let mut len: c_int = 0;
    while precision < 0 || len < precision {
        if *outstr.add(len as usize) == 0 {
            break;
        }
        len += 1;
    }
    len
}

unsafe fn output(
    cb: __nolibc_printf_cb,
    state: *mut c_void,
    outstr: *const c_char,
    len: c_int,
    written: &mut c_int,
    width: &mut c_int,
    flags: &mut c_uint,
) -> Result<(), c_int> {
    *written += len;
    _NOLIBC_OPTIMIZER_HIDE_VAR(len);
    *width -= len;
    *flags = _NOLIBC_PF_FLAGS_CONTAIN_8(*flags, [b'-' as c_int, 0, 0, 0, 0, 0, 0, 0]);
    if *flags != 0 && cb(state, outstr, len as size_t) != 0 {
        return Err(-1);
    }
    while *width > 0 {
        let pad_len: c_int = ((*width - 1) & 15) + 1;
        *width -= pad_len;
        *written += pad_len;
        if cb(state, c"                ".as_ptr(), pad_len as size_t) != 0 {
            return Err(-1);
        }
    }
    if *flags == 0 && cb(state, outstr, len as size_t) != 0 {
        return Err(-1);
    }
    Ok(())
}

unsafe fn __nolibc_fprintf_cb(stream: *mut c_void, buf: *const c_char, size: size_t) -> c_int {
    _fwrite(buf as *const c_void, size, stream as *mut FILE)
}

pub unsafe fn vfprintf(stream: *mut FILE, fmt: *const c_char, args: VaListImpl) -> c_int {
    __nolibc_printf(__nolibc_fprintf_cb, stream as *mut c_void, fmt, args)
}

pub unsafe fn vprintf(fmt: *const c_char, args: VaListImpl) -> c_int {
    vfprintf(stdout, fmt, args)
}

pub unsafe extern "C" fn fprintf(stream: *mut FILE, fmt: *const c_char, mut args: ...) -> c_int {
    vfprintf(stream, fmt, args.as_va_list())
}

pub unsafe extern "C" fn printf(fmt: *const c_char, mut args: ...) -> c_int {
    vfprintf(stdout, fmt, args.as_va_list())
}

pub unsafe fn vdprintf(fd: c_int, fmt: *const c_char, args: VaListImpl) -> c_int {
    let stream: *mut FILE;

    stream = fdopen(fd, core::ptr::null());
    if stream.is_null() {
        return -1;
    }
    /* Technically 'stream' is leaked, but as it's only a wrapper around 'fd' that is fine */
    vfprintf(stream, fmt, args)
}

pub unsafe extern "C" fn dprintf(fd: c_int, fmt: *const c_char, mut args: ...) -> c_int {
    vdprintf(fd, fmt, args.as_va_list())
}

#[repr(C)]
pub struct __nolibc_sprintf_cb_state {
    pub buf: *mut c_char,
    pub space: size_t,
}

unsafe fn __nolibc_sprintf_cb(v_state: *mut c_void, buf: *const c_char, mut size: size_t) -> c_int {
    let state: *mut __nolibc_sprintf_cb_state = v_state as *mut __nolibc_sprintf_cb_state;
    let space: size_t = (*state).space;
    let tgt: *mut c_char;

    if size >= space {
        if space <= 1 {
            return 0;
        }
        size = space - 1;
    }
    tgt = (*state).buf;

    *tgt = b'\0' as c_char;
    if size != 0 {
        (*state).space = space - size;
        (*state).buf = tgt.add(size);
        memcpy(tgt as *mut c_void, buf as *const c_void, size);
    }

    0
}

pub unsafe fn vsnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, args: VaListImpl) -> c_int {
    let mut state = __nolibc_sprintf_cb_state { buf, space: size };

    __nolibc_printf(__nolibc_sprintf_cb, &mut state as *mut _ as *mut c_void, fmt, args)
}

pub unsafe extern "C" fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, mut args: ...) -> c_int {
    vsnprintf(buf, size, fmt, args.as_va_list())
}

pub unsafe fn vsprintf(buf: *mut c_char, fmt: *const c_char, args: VaListImpl) -> c_int {
    vsnprintf(buf, SIZE_MAX, fmt, args)
}

pub unsafe extern "C" fn sprintf(buf: *mut c_char, fmt: *const c_char, mut args: ...) -> c_int {
    vsprintf(buf, fmt, args.as_va_list())
}

pub unsafe fn __nolibc_vasprintf(strp: *mut *mut c_char, fmt: *const c_char, args1: VaListImpl, args2: VaListImpl) -> c_int {
    let len1: c_int;
    let len2: c_int;
    let buf: *mut c_char;

    len1 = vsnprintf(core::ptr::null_mut(), 0, fmt, args1);
    if len1 < 0 {
        return -1;
    }

    buf = malloc(len1 as size_t + 1) as *mut c_char;
    if buf.is_null() {
        return -1;
    }

    len2 = vsnprintf(buf, len1 as size_t + 1, fmt, args2);
    if len2 < 0 {
        free(buf as *mut c_void);
        return -1;
    }

    *strp = buf;
    len1
}

pub unsafe fn vasprintf(strp: *mut *mut c_char, fmt: *const c_char, args: VaListImpl) -> c_int {
    /* va_copy(args2, args) is required by the C source; Rust VaList copying is represented source-locally here. */
    let args2 = args.clone();
    __nolibc_vasprintf(strp, fmt, args, args2)
}

pub unsafe extern "C" fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, mut args: ...) -> c_int {
    vasprintf(strp, fmt, args.as_va_list())
}

pub unsafe fn vsscanf(mut str_: *const c_char, mut format: *const c_char, mut args: VaListImpl) -> c_int {
    let mut uval: uintmax_t;
    let mut ival: intmax_t;
    let mut base: c_int;
    let mut endptr: *mut c_char = core::ptr::null_mut();
    let mut matches: c_int;
    let mut lpref: c_int;

    matches = 0;

    loop {
        if *format == b'%' as c_char {
            /* start of pattern */
            lpref = 0;
            format = format.add(1);

            if *format == b'l' as c_char {
                /* same as in printf() */
                lpref = 1;
                format = format.add(1);
                if *format == b'l' as c_char {
                    lpref = 2;
                    format = format.add(1);
                }
            }

            if *format == b'%' as c_char {
                /* literal % */
                if b'%' as c_char != *str_ {
                    break;
                }
                str_ = str_.add(1);
                format = format.add(1);
                continue;
            } else if *format == b'd' as c_char {
                ival = strtoll(str_, &mut endptr, 10);
                if lpref == 0 {
                    *args.arg::<*mut c_int>() = ival as c_int;
                } else if lpref == 1 {
                    *args.arg::<*mut c_long>() = ival as c_long;
                } else if lpref == 2 {
                    *args.arg::<*mut i64>() = ival as i64;
                }
            } else if *format == b'u' as c_char || *format == b'x' as c_char || *format == b'X' as c_char {
                base = if *format == b'u' as c_char { 10 } else { 16 };
                uval = strtoull(str_, &mut endptr, base);
                if lpref == 0 {
                    *args.arg::<*mut c_uint>() = uval as c_uint;
                } else if lpref == 1 {
                    *args.arg::<*mut c_ulong>() = uval as c_ulong;
                } else if lpref == 2 {
                    *args.arg::<*mut u64>() = uval as u64;
                }
            } else if *format == b'p' as c_char {
                *args.arg::<*mut *mut c_void>() = strtoul(str_, &mut endptr, 16) as *mut c_void;
            } else {
                SET_ERRNO(EILSEQ);
                break;
            }

            format = format.add(1);
            str_ = endptr;
            matches += 1;
        } else if *format == b'\0' as c_char {
            break;
        } else if isspace(*format as c_int) != 0 {
            /* skip spaces in format and str */
            while isspace(*format as c_int) != 0 {
                format = format.add(1);
            }
            while isspace(*str_ as c_int) != 0 {
                str_ = str_.add(1);
            }
        } else if *format == *str_ {
            /* literal match */
            format = format.add(1);
            str_ = str_.add(1);
        } else {
            if matches == 0 {
                matches = EOF;
            }
            break;
        }
    }

    matches
}

pub unsafe extern "C" fn sscanf(str_: *const c_char, format: *const c_char, mut args: ...) -> c_int {
    vsscanf(str_, format, args.as_va_list())
}

pub unsafe fn perror(msg: *const c_char) {
    /* If NOLIBC_IGNORE_ERRNO is defined:
     * fprintf(stderr, "%s%sunknown error\n", (msg && *msg) ? msg : "", (msg && *msg) ? ": " : "");
     * Otherwise:
     */
    fprintf(
        stderr,
        c"%s%serrno=%d\n".as_ptr(),
        if !msg.is_null() && *msg != 0 { msg } else { c"".as_ptr() },
        if !msg.is_null() && *msg != 0 { c": ".as_ptr() } else { c"".as_ptr() },
        errno,
    );
}

pub unsafe fn setvbuf(_stream: *mut FILE, _buf: *mut c_char, mode: c_int, _size: size_t) -> c_int {
    /*
     * nolibc does not support buffering so this is a nop. Just check mode
     * is valid as required by the spec.
     */
    match mode {
        _IOFBF | _IOLBF | _IONBF => {}
        _ => return EOF,
    }

    0
}

pub unsafe fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int {
    if buflen < 18 {
        return ERANGE;
    }

    memcpy(buf as *mut c_void, c"errno=".as_ptr() as *const c_void, 6);
    i64toa_r(errnum as c_long, buf.add(6));
    0
}

pub unsafe fn strerror(errnum: c_int) -> *const c_char {
    static mut BUF: [c_char; 18] = [0; 18];
    let mut b: *mut c_char = BUF.as_mut_ptr();

    /* Force gcc to use 'register offset' to access buf[]. */
    _NOLIBC_OPTIMIZER_HIDE_VAR(b);

    /* Use strerror_r() to avoid having the only .data in small programs. */
    strerror_r(errnum, b, core::mem::size_of_val(&BUF));

    b
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
