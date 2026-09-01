// SPDX-License-Identifier: Apache-2.0 OR MIT

// The contents of this file come from the Rust rustc-demangle library, hosted
// in the <https://github.com/rust-lang/rustc-demangle> repository, licensed
// under "Apache-2.0 OR MIT". For copyright details, see
// <https://github.com/rust-lang/rustc-demangle/blob/main/README.md>.
// Please note that the file should be kept as close as possible to upstream.

// Code for demangling Rust symbols. This code is mostly
// a line-by-line translation of the Rust code in `rustc-demangle`.

// you can find the latest version of this code in https://github.com/rust-lang/rustc-demangle

use core::ffi::{c_char, c_void};
use core::{mem, ptr, slice};

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub const OVERFLOW_MARGIN: size_t = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum overflow_status {
    OverflowOk = 0,
    OverflowOverflow = 1,
}
use overflow_status::*;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DemangleStyle {
    DemangleStyleUnknown = 0,
    DemangleStyleLegacy = 1,
    DemangleStyleV0 = 2,
}
use DemangleStyle::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct demangle {
    pub style: DemangleStyle,
    pub mangled: *const c_char,
    pub mangled_len: size_t,
    pub elements: size_t,
    pub original: *const c_char,
    pub original_len: size_t,
    pub suffix: *const c_char,
    pub suffix_len: size_t,
}

const MAX_DEPTH: uint32_t = 500;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum demangle_status {
    DemangleOk,
    DemangleInvalid,
    DemangleRecursed,
    DemangleBug,
}
use demangle_status::*;

#[repr(C)]
#[derive(Copy, Clone)]
struct demangle_v0 {
    mangled: *const c_char,
    mangled_len: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct demangle_legacy {
    mangled: *const c_char,
    mangled_len: size_t,
    elements: size_t,
}

unsafe fn strlen(mut s: *const c_char) -> size_t {
    let start = s;
    while *s != 0 {
        s = s.add(1);
    }
    s.offset_from(start) as size_t
}

unsafe fn memchr_(s: *const c_char, c: c_char, n: size_t) -> *const c_char {
    let mut i = 0;
    while i < n {
        if *s.add(i) == c {
            return s.add(i);
        }
        i += 1;
    }
    ptr::null()
}

unsafe fn strstr_(s: *const c_char, needle: &[u8]) -> *const c_char {
    let nlen = needle.len();
    let slen = strlen(s);
    if nlen == 0 {
        return s;
    }
    let bytes = slice::from_raw_parts(s as *const u8, slen);
    let mut i = 0;
    while i + nlen <= slen {
        if &bytes[i..i + nlen] == needle {
            return s.add(i);
        }
        i += 1;
    }
    ptr::null()
}

unsafe fn strncmp_lit(s: *const c_char, lit: &[u8]) -> bool {
    slice::from_raw_parts(s as *const u8, lit.len()) == lit
}

unsafe fn demangle_memrchr(s: *const c_void, c: i32, mut n: size_t) -> *mut c_void {
    let s_ = s as *const uint8_t;
    while n != 0 {
        if *s_.add(n - 1) == c as uint8_t {
            return s_.add(n - 1) as *mut c_void;
        }
        n -= 1;
    }
    ptr::null_mut()
}

fn unicode_iscontrol(ch: uint32_t) -> bool {
    // this is *technically* a unicode table, but
    // some unicode properties are simpler than you might think
    ch < 0x20 || (ch >= 0x7f && ch < 0xa0)
}

// "good enough" tables, the only consequence is that when printing
// *constant strings*, some characters are printed as `\u{abcd}` rather than themselves.
//
// I'm leaving these here to allow easily replacing them with actual
// tables if desired.
fn unicode_isprint(ch: uint32_t) -> bool {
    if ch < 0x20 {
        return false;
    }
    if ch < 0x7f {
        return true;
    }
    false
}

fn unicode_isgraphemextend(_ch: uint32_t) -> bool {
    false
}

unsafe fn str_isascii(s: *const c_char, s_len: size_t) -> bool {
    let mut i = 0;
    while i < s_len {
        if (*s.add(i) as u8) & 0x80 != 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum punycode_status {
    PunycodeOk,
    PunycodeError,
}
use punycode_status::*;

#[repr(C)]
#[derive(Copy, Clone)]
struct parser {
    // the parser assumes that `sym` has a safe "terminating byte". It might be NUL,
    // but it might also be something else if a symbol is "truncated".
    sym: *const c_char,
    sym_len: size_t,
    next: size_t,
    depth: uint32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct printer {
    status: demangle_status, // if status == 0 parser is valid
    parser: parser,
    out: *mut c_char, // NULL for no output [in which case out_len is not decremented]
    out_len: size_t,
    bound_lifetime_depth: uint32_t,
    alternate: bool,
}

macro_rules! print {
    ($e:expr) => {{
        if $e == OverflowOverflow {
            return OverflowOverflow;
        }
    }};
}

macro_rules! print_ch {
    ($printer:expr, $s:expr) => {
        print!(printer_print_ch($printer, $s as c_char))
    };
}

macro_rules! print_str {
    ($printer:expr, $s:expr) => {
        print!(printer_print_str($printer, concat!($s, "\0").as_ptr() as *const c_char))
    };
}

macro_rules! print_u64 {
    ($printer:expr, $s:expr) => {
        print!(printer_print_u64($printer, $s))
    };
}

macro_rules! invalid {
    ($printer:expr) => {{
        print_str!($printer, "{invalid syntax}");
        (*$printer).status = DemangleInvalid;
        return OverflowOk;
    }};
}

macro_rules! parse {
    ($printer:expr, $method:ident $(, $arg:expr)* $(,)?) => {{
        if (*$printer).status != DemangleOk {
            print_str!($printer, "?");
            return OverflowOk;
        } else {
            let parse_status = $method(&mut (*$printer).parser $(, $arg)*);
            if parse_status != DemangleOk {
                print!(printer_print_str($printer, demangle_error_message(parse_status)));
                (*$printer).status = parse_status;
                return OverflowOk;
            }
        }
    }};
}

unsafe fn try_parse_path(p: *mut parser) -> demangle_status {
    let mut pr = printer {
        status: DemangleOk,
        parser: *p,
        out: ptr::null_mut(),
        out_len: usize::MAX,
        bound_lifetime_depth: 0,
        alternate: false,
    };
    let _ignore = printer_print_path(&mut pr, false);
    *p = pr.parser;
    pr.status
}

unsafe fn rust_demangle_v0_demangle(
    s: *const c_char,
    s_len: size_t,
    res: *mut demangle_v0,
    rest: *mut *const c_char,
) -> demangle_status {
    if s_len > strlen(s) {
        // s_len only exists to shorten the string, this is not a buffer API
        return DemangleInvalid;
    }

    let inner: *const c_char;
    let inner_len: size_t;
    if s_len >= 2 && strncmp_lit(s, b"_R") {
        inner = s.add(2);
        inner_len = s_len - 2;
    } else if s_len >= 1 && strncmp_lit(s, b"R") {
        // On Windows, dbghelp strips leading underscores, so we accept "R..."
        // form too.
        inner = s.add(1);
        inner_len = s_len - 1;
    } else if s_len >= 3 && strncmp_lit(s, b"__R") {
        // On OSX, symbols are prefixed with an extra _
        inner = s.add(3);
        inner_len = s_len - 3;
    } else {
        return DemangleInvalid;
    }

    // Paths always start with uppercase characters.
    if *inner < b'A' as c_char || *inner > b'Z' as c_char {
        return DemangleInvalid;
    }
    if !str_isascii(inner, inner_len) {
        return DemangleInvalid;
    }

    let mut p = parser { sym: inner, sym_len: inner_len, next: 0, depth: 0 };
    let mut status = try_parse_path(&mut p);
    if status != DemangleOk {
        return status;
    }
    let next = *p.sym.add(p.next);
    // Instantiating crate (paths always start with uppercase characters).
    if p.next < p.sym_len && next >= b'A' as c_char && next <= b'Z' as c_char {
        status = try_parse_path(&mut p);
        if status != DemangleOk {
            return status;
        }
    }

    (*res).mangled = inner;
    (*res).mangled_len = inner_len;
    if !rest.is_null() {
        *rest = p.sym.add(p.next);
    }
    DemangleOk
}

unsafe fn rust_demangle_v0_display_demangle(
    res: demangle_v0,
    out: *mut c_char,
    len: size_t,
    alternate: bool,
) -> overflow_status {
    let mut pr = printer {
        status: DemangleOk,
        parser: parser { sym: res.mangled, sym_len: res.mangled_len, next: 0, depth: 0 },
        out,
        out_len: len,
        bound_lifetime_depth: 0,
        alternate,
    };
    if printer_print_path(&mut pr, true) == OverflowOverflow {
        return OverflowOverflow;
    }
    if pr.out_len < OVERFLOW_MARGIN {
        return OverflowOverflow;
    }
    *pr.out = 0;
    OverflowOk
}

unsafe fn code_to_utf8(buffer: *mut u8, code: uint32_t) -> size_t {
    if code <= 0x7f {
        *buffer = code as u8;
        return 1;
    }
    if code <= 0x7ff {
        *buffer.add(0) = (0xc0 | (code >> 6)) as u8;
        *buffer.add(1) = (0x80 | (code & 0x3f)) as u8;
        return 2;
    }
    if code <= 0xffff {
        *buffer.add(0) = (0xe0 | (code >> 12)) as u8;
        *buffer.add(1) = (0x80 | ((code >> 6) & 0x3f)) as u8;
        *buffer.add(2) = (0x80 | (code & 0x3f)) as u8;
        return 3;
    }
    if code <= 0x10ffff {
        *buffer.add(0) = (0xf0 | (code >> 18)) as u8;
        *buffer.add(1) = (0x80 | ((code >> 12) & 0x3f)) as u8;
        *buffer.add(2) = (0x80 | ((code >> 6) & 0x3f)) as u8;
        *buffer.add(3) = (0x80 | (code & 0x3f)) as u8;
        return 4;
    }
    0
}

unsafe fn utf8_next_char(s: *mut u8, ch: *mut uint32_t) -> size_t {
    let byte = *s;
    if byte < 0x80 {
        *ch = byte as uint32_t;
        return 1;
    } else if byte < 0xc2 {
        return usize::MAX;
    } else if byte < 0xe0 {
        if *s.add(1) >= 0x80 && *s.add(1) < 0xc0 {
            *ch = (((byte & 0x1f) as uint32_t) << 6) + ((*s.add(1) & 0x3f) as uint32_t);
            return 2;
        }
        return usize::MAX;
    } else if byte < 0xf0 {
        if !(*s.add(1) >= 0x80 && *s.add(1) < 0xc0) || !(*s.add(2) >= 0x80 && *s.add(2) < 0xc0) {
            return usize::MAX; // basic validation
        }
        if byte == 0xe0 && *s.add(1) < 0xa0 {
            return usize::MAX; // overshort
        }
        if byte == 0xed && *s.add(1) >= 0xa0 {
            return usize::MAX; // surrogate
        }
        *ch = (((byte & 0x0f) as uint32_t) << 12)
            + (((*s.add(1) & 0x3f) as uint32_t) << 6)
            + ((*s.add(2) & 0x3f) as uint32_t);
        return 3;
    } else if byte < 0xf5 {
        if !(*s.add(1) >= 0x80 && *s.add(1) < 0xc0)
            || !(*s.add(2) >= 0x80 && *s.add(2) < 0xc0)
            || !(*s.add(3) >= 0x80 && *s.add(3) < 0xc0)
        {
            return usize::MAX; // basic validation
        }
        if byte == 0xf0 && *s.add(1) < 0x90 {
            return usize::MAX; // overshort
        }
        if byte == 0xf4 && *s.add(1) >= 0x90 {
            return usize::MAX; // over max
        }
        *ch = (((byte & 0x07) as uint32_t) << 18)
            + (((*s.add(1) & 0x3f) as uint32_t) << 12)
            + (((*s.add(2) & 0x3f) as uint32_t) << 6)
            + ((*s.add(3) & 0x3f) as uint32_t);
        return 4;
    }
    usize::MAX
}

fn validate_char(n: uint32_t) -> bool {
    ((n ^ 0xd800).wrapping_sub(0x800)) < 0x110000 - 0x800
}

const SMALL_PUNYCODE_LEN: usize = 128;

unsafe fn punycode_decode(
    start: *const c_char,
    ascii_len: size_t,
    mut punycode_start: *const c_char,
    mut punycode_len: size_t,
    out_: *mut [uint32_t; SMALL_PUNYCODE_LEN],
    out_len: *mut size_t,
) -> punycode_status {
    let out = (*out_).as_mut_ptr();
    if punycode_len == 0 || ascii_len > SMALL_PUNYCODE_LEN {
        return PunycodeError;
    }
    let mut i0 = 0;
    while i0 < ascii_len {
        *out.add(i0) = *start.add(i0) as u8 as uint32_t;
        i0 += 1;
    }
    let mut len = ascii_len;
    let base = 36usize;
    let t_min = 1usize;
    let t_max = 26usize;
    let skew = 38usize;
    let mut damp = 700usize;
    let mut bias = 72usize;
    let mut i = 0usize;
    let mut n = 0x80usize;
    loop {
        let mut delta = 0usize;
        let mut w = 1usize;
        let mut k = 0usize;
        loop {
            k += base;
            let biased = if k < bias { 0 } else { k - bias };
            let t = biased.max(t_min).min(t_max);
            let d: usize;
            if punycode_len == 0 {
                return PunycodeError;
            }
            let nx = *punycode_start;
            punycode_start = punycode_start.add(1);
            punycode_len -= 1;
            if nx >= b'a' as c_char && nx <= b'z' as c_char {
                d = (nx - b'a' as c_char) as usize;
            } else if nx >= b'0' as c_char && nx <= b'9' as c_char {
                d = 26 + (nx - b'0' as c_char) as usize;
            } else {
                return PunycodeError;
            }
            if w == 0 || d > usize::MAX / w || d * w > usize::MAX - delta {
                return PunycodeError;
            }
            delta += d * w;
            if d < t {
                break;
            }
            if base < t || w == 0 || (base - t) > usize::MAX / w {
                return PunycodeError;
            }
            w *= base - t;
        }
        len += 1;
        if i > usize::MAX - delta {
            return PunycodeError;
        }
        i += delta;
        if n > usize::MAX - i / len {
            return PunycodeError;
        }
        n += i / len;
        i %= len;
        // char validation
        if n > u32::MAX as usize || !validate_char(n as uint32_t) {
            return PunycodeError;
        }
        // insert new character
        if len > SMALL_PUNYCODE_LEN {
            return PunycodeError;
        }
        ptr::copy(out.add(i), out.add(i + 1), len - i - 1);
        *out.add(i) = n as uint32_t;
        // start i index at incremented position
        i += 1;
        // If there are no more deltas, decoding is complete.
        if punycode_len == 0 {
            *out_len = len;
            return PunycodeOk;
        }
        // Perform bias adaptation.
        delta /= damp;
        damp = 2;
        delta += delta / len;
        k = 0;
        while delta > ((base - t_min) * t_max) / 2 {
            delta /= base - t_min;
            k += base;
        }
        bias = k + ((base - t_min + 1) * delta) / (delta + skew);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ident {
    ascii_start: *const c_char,
    ascii_len: size_t,
    punycode_start: *const c_char,
    punycode_len: size_t,
}

unsafe fn display_ident(
    ascii_start: *const c_char,
    ascii_len: size_t,
    punycode_start: *const c_char,
    punycode_len: size_t,
    out: *mut u8,
    out_len: *mut size_t,
) -> overflow_status {
    let mut outbuf = [0u32; SMALL_PUNYCODE_LEN];
    let mut wide_len = 0usize;
    let out_buflen = *out_len;
    if punycode_len == 0 {
        if ascii_len > out_buflen {
            return OverflowOverflow;
        }
        ptr::copy_nonoverlapping(ascii_start as *const u8, out, ascii_len);
        *out_len = ascii_len;
    } else if punycode_decode(ascii_start, ascii_len, punycode_start, punycode_len, &mut outbuf, &mut wide_len) == PunycodeOk {
        let mut narrow_len = 0usize;
        let mut i = 0;
        while i < wide_len {
            if out_buflen - narrow_len < 4 {
                return OverflowOverflow;
            }
            let pos = out.add(narrow_len);
            narrow_len += code_to_utf8(pos, outbuf[i]);
            i += 1;
        }
        *out_len = narrow_len;
    } else {
        let mut narrow_len = 0usize;
        if out_buflen < 9 {
            return OverflowOverflow;
        }
        ptr::copy_nonoverlapping(b"punycode{".as_ptr(), out, 9);
        narrow_len = 9;
        if ascii_len > 0 {
            if out_buflen - narrow_len < ascii_len || out_buflen - narrow_len - ascii_len < 1 {
                return OverflowOverflow;
            }
            ptr::copy_nonoverlapping(ascii_start as *const u8, out.add(narrow_len), ascii_len);
            narrow_len += ascii_len;
            *out.add(narrow_len) = b'-';
            narrow_len += 1;
        }
        if out_buflen - narrow_len < punycode_len || out_buflen - narrow_len - punycode_len < 1 {
            return OverflowOverflow;
        }
        ptr::copy_nonoverlapping(punycode_start as *const u8, out.add(narrow_len), punycode_len);
        narrow_len += punycode_len;
        *out.add(narrow_len) = b'}';
        narrow_len += 1;
        *out_len = narrow_len;
    }
    OverflowOk
}

unsafe fn try_parse_uint(buf: *const c_char, len: size_t, result: *mut uint64_t) -> bool {
    let mut cur = 0usize;
    while cur < len && *buf.add(cur) == b'0' as c_char {
        cur += 1;
    }
    let mut result_val = 0u64;
    if len - cur > 16 {
        return false;
    }
    while cur < len {
        let c = *buf.add(cur);
        result_val <<= 4;
        if c >= b'0' as c_char && c <= b'9' as c_char {
            result_val += (c - b'0' as c_char) as u64;
        } else if c >= b'a' as c_char && c <= b'f' as c_char {
            result_val += 10 + (c - b'a' as c_char) as u64;
        } else {
            return false;
        }
        cur += 1;
    }
    *result = result_val;
    true
}

unsafe fn dinibble2int(buf: *const c_char, result: *mut u8) -> bool {
    let mut result_val = 0u8;
    let mut i = 0;
    while i < 2 {
        let c = *buf.add(i);
        result_val <<= 4;
        if c >= b'0' as c_char && c <= b'9' as c_char {
            result_val += (c - b'0' as c_char) as u8;
        } else if c >= b'a' as c_char && c <= b'f' as c_char {
            result_val += 10 + (c - b'a' as c_char) as u8;
        } else {
            return false;
        }
        i += 1;
    }
    *result = result_val;
    true
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum nibbles_to_string_status {
    NtsOk = 0,
    NtsOverflow = 1,
    NtsInvalid = 2,
}
use nibbles_to_string_status::*;

const ESCAPED_SIZE: usize = 12;

unsafe fn char_to_string(ch: uint32_t, quote: u8, first: bool, buf: *mut [c_char; ESCAPED_SIZE]) -> size_t {
    let escaped_buf = (*buf).as_mut_ptr();
    *escaped_buf.add(0) = b'\\' as c_char;
    let mut escaped_len = 2usize;
    match ch {
        0 => *escaped_buf.add(1) = b'0' as c_char,
        9 => *escaped_buf.add(1) = b't' as c_char,
        13 => *escaped_buf.add(1) = b'r' as c_char,
        10 => *escaped_buf.add(1) = b'n' as c_char,
        92 => *escaped_buf.add(1) = b'\\' as c_char,
        _ => {
            if ch == quote as uint32_t {
                *escaped_buf.add(1) = ch as c_char;
            } else if !unicode_isprint(ch) || (first && unicode_isgraphemextend(ch)) {
                let s = format!("\\u{{{:x}}}", ch);
                let bytes = s.as_bytes();
                if bytes.len() >= ESCAPED_SIZE {
                    return 0;
                }
                ptr::copy_nonoverlapping(bytes.as_ptr(), escaped_buf as *mut u8, bytes.len());
                escaped_len = bytes.len();
            } else {
                // printable character
                *escaped_buf.add(0) = ch as c_char;
                escaped_len = 1;
            }
        }
    }
    escaped_len
}

unsafe fn nibbles_to_string(mut buf: *const c_char, mut len: size_t, mut out: *mut u8, out_len: *mut size_t) -> nibbles_to_string_status {
    let quote = b'"';
    let mut first = true;
    if (len % 2) != 0 {
        return NtsInvalid; // odd number of nibbles
    }
    let mut cur_out_len = 0usize;
    // write starting quote
    if !out.is_null() {
        cur_out_len = *out_len;
        if cur_out_len == 0 {
            return NtsOverflow;
        }
        *out = quote;
        out = out.add(1);
        cur_out_len -= 1;
    }
    let mut conv_buf = [0u8; 4];
    let mut conv_buf_len = 0usize;
    while len > 1 || conv_buf_len > 0 {
        while len > 1 && conv_buf_len < conv_buf.len() {
            if !dinibble2int(buf, &mut conv_buf[conv_buf_len]) {
                return NtsInvalid;
            }
            conv_buf_len += 1;
            buf = buf.add(2);
            len -= 2;
        }
        // conv_buf is full here if possible, process 1 UTF-8 character
        let mut ch = 0u32;
        let consumed = utf8_next_char(conv_buf.as_mut_ptr(), &mut ch);
        if consumed > conv_buf_len {
            return NtsInvalid;
        }
        ptr::copy(conv_buf.as_ptr().add(consumed), conv_buf.as_mut_ptr(), conv_buf_len - consumed);
        conv_buf_len -= consumed;
        let mut escaped_buf = [0 as c_char; ESCAPED_SIZE];
        let escaped_len = char_to_string(ch, b'"', first, &mut escaped_buf);
        if !out.is_null() {
            if cur_out_len < escaped_len {
                return NtsOverflow;
            }
            ptr::copy_nonoverlapping(escaped_buf.as_ptr() as *const u8, out, escaped_len);
            out = out.add(escaped_len);
            cur_out_len -= escaped_len;
        }
        first = false;
    }
    // write ending quote
    if !out.is_null() {
        if cur_out_len == 0 {
            return NtsOverflow;
        }
        *out = quote;
        cur_out_len -= 1;
        *out_len -= cur_out_len; // subtract remaining space to get used space
    }
    NtsOk
}

fn basic_type(tag: u8) -> *const c_char {
    match tag {
        b'b' => c"bool".as_ptr(),
        b'c' => c"char".as_ptr(),
        b'e' => c"str".as_ptr(),
        b'u' => c"()".as_ptr(),
        b'a' => c"i8".as_ptr(),
        b's' => c"i16".as_ptr(),
        b'l' => c"i32".as_ptr(),
        b'x' => c"i64".as_ptr(),
        b'n' => c"i128".as_ptr(),
        b'i' => c"isize".as_ptr(),
        b'h' => c"u8".as_ptr(),
        b't' => c"u16".as_ptr(),
        b'm' => c"u32".as_ptr(),
        b'y' => c"u64".as_ptr(),
        b'o' => c"u128".as_ptr(),
        b'j' => c"usize".as_ptr(),
        b'f' => c"f32".as_ptr(),
        b'd' => c"f64".as_ptr(),
        b'z' => c"!".as_ptr(),
        b'p' => c"_".as_ptr(),
        b'v' => c"...".as_ptr(),
        _ => ptr::null(),
    }
}

fn parser_push_depth(p: *mut parser) -> demangle_status {
    unsafe {
        (*p).depth += 1;
        if (*p).depth > MAX_DEPTH { DemangleRecursed } else { DemangleOk }
    }
}

fn parser_pop_depth(p: *mut parser) -> demangle_status {
    unsafe { (*p).depth -= 1; }
    DemangleOk
}

unsafe fn parser_peek(p: *const parser) -> u8 {
    if (*p).next == (*p).sym_len {
        0
    } else {
        *(*p).sym.add((*p).next) as u8
    }
}

unsafe fn parser_eat(p: *mut parser, ch: u8) -> bool {
    if parser_peek(p) == ch {
        if ch != 0 {
            (*p).next += 1;
        }
        true
    } else {
        false
    }
}

unsafe fn parser_next(p: *mut parser) -> u8 {
    if (*p).next == (*p).sym_len {
        0
    } else {
        let r = *(*p).sym.add((*p).next) as u8;
        (*p).next += 1;
        r
    }
}

unsafe fn parser_ch(p: *mut parser, next: *mut u8) -> demangle_status {
    if (*p).next == (*p).sym_len {
        DemangleInvalid
    } else {
        *next = *(*p).sym.add((*p).next) as u8;
        (*p).next += 1;
        DemangleOk
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct buf {
    start: *const c_char,
    len: size_t,
}

unsafe fn parser_hex_nibbles(p: *mut parser, b: *mut buf) -> demangle_status {
    let start = (*p).next;
    loop {
        let ch = parser_next(p);
        if ch == b'_' {
            break;
        }
        if !((ch >= b'0' && ch <= b'9') || (ch >= b'a' && ch <= b'f')) {
            return DemangleInvalid;
        }
    }
    (*b).start = (*p).sym.add(start);
    (*b).len = (*p).next - start - 1; // skip final _
    DemangleOk
}

unsafe fn parser_digit_10(p: *mut parser, out: *mut u8) -> demangle_status {
    let ch = parser_peek(p);
    if ch >= b'0' && ch <= b'9' {
        *out = ch - b'0';
        (*p).next += 1;
        DemangleOk
    } else {
        DemangleInvalid
    }
}

unsafe fn parser_digit_62(p: *mut parser, out: *mut u64) -> demangle_status {
    let ch = parser_peek(p);
    if ch >= b'0' && ch <= b'9' {
        *out = (ch - b'0') as u64;
        (*p).next += 1;
        DemangleOk
    } else if ch >= b'a' && ch <= b'z' {
        *out = (10 + (ch - b'a')) as u64;
        (*p).next += 1;
        DemangleOk
    } else if ch >= b'A' && ch <= b'Z' {
        *out = (10 + 26 + (ch - b'A')) as u64;
        (*p).next += 1;
        DemangleOk
    } else {
        DemangleInvalid
    }
}

unsafe fn parser_integer_62(p: *mut parser, out: *mut u64) -> demangle_status {
    if parser_eat(p, b'_') {
        *out = 0;
        return DemangleOk;
    }
    let mut x = 0u64;
    while !parser_eat(p, b'_') {
        let mut d = 0u64;
        let status = parser_digit_62(p, &mut d);
        if status != DemangleOk {
            return status;
        }
        if x > u64::MAX / 62 {
            return DemangleInvalid;
        }
        x *= 62;
        if x > u64::MAX - d {
            return DemangleInvalid;
        }
        x += d;
    }
    if x == u64::MAX {
        return DemangleInvalid;
    }
    *out = x + 1;
    DemangleOk
}

unsafe fn parser_opt_integer_62(p: *mut parser, tag: u8, out: *mut u64) -> demangle_status {
    if !parser_eat(p, tag) {
        *out = 0;
        return DemangleOk;
    }
    let status = parser_integer_62(p, out);
    if status != DemangleOk {
        return status;
    }
    if *out == u64::MAX {
        return DemangleInvalid;
    }
    *out += 1;
    DemangleOk
}

unsafe fn parser_disambiguator(p: *mut parser, out: *mut u64) -> demangle_status {
    parser_opt_integer_62(p, b's', out)
}

type parser_namespace_type = u8;

unsafe fn parser_namespace(p: *mut parser, out: *mut parser_namespace_type) -> demangle_status {
    let next = parser_next(p);
    if next >= b'A' && next <= b'Z' {
        *out = next;
        DemangleOk
    } else if next >= b'a' && next <= b'z' {
        *out = 0;
        DemangleOk
    } else {
        DemangleInvalid
    }
}

unsafe fn parser_backref(p: *mut parser, out: *mut parser) -> demangle_status {
    let start = (*p).next;
    if start == 0 {
        return DemangleBug;
    }
    let s_start = start - 1;
    let mut i = 0u64;
    let mut status = parser_integer_62(p, &mut i);
    if status != DemangleOk {
        return status;
    }
    if i >= s_start as u64 {
        return DemangleInvalid;
    }
    let mut res = parser { sym: (*p).sym, sym_len: (*p).sym_len, next: i as usize, depth: (*p).depth };
    status = parser_push_depth(&mut res);
    if status != DemangleOk {
        return status;
    }
    *out = res;
    DemangleOk
}

unsafe fn parser_ident(p: *mut parser, out: *mut ident) -> demangle_status {
    let is_punycode = parser_eat(p, b'u');
    let mut d = 0u8;
    let mut status = parser_digit_10(p, &mut d);
    let mut len = d as usize;
    if status != DemangleOk {
        return status;
    }
    if len != 0 {
        loop {
            status = parser_digit_10(p, &mut d);
            if status != DemangleOk {
                break;
            }
            if len > usize::MAX / 10 {
                return DemangleInvalid;
            }
            len *= 10;
            if len > usize::MAX - d as usize {
                return DemangleInvalid;
            }
            len += d as usize;
        }
    }
    // Skip past the optional `_` separator.
    parser_eat(p, b'_');
    let start = (*p).next;
    if (*p).sym_len - (*p).next < len {
        return DemangleInvalid;
    }
    (*p).next += len;
    let id = (*p).sym.add(start);
    if is_punycode {
        let underscore = demangle_memrchr(id as *const c_void, b'_' as i32, len) as *const c_char;
        if underscore.is_null() {
            *out = ident { ascii_start: c"".as_ptr(), ascii_len: 0, punycode_start: id, punycode_len: len };
        } else {
            let ascii_len = underscore.offset_from(id) as usize;
            let punycode_len = len - 1 - ascii_len;
            *out = ident { ascii_start: id, ascii_len, punycode_start: underscore.add(1), punycode_len };
        }
        if (*out).punycode_len == 0 {
            return DemangleInvalid;
        }
        DemangleOk
    } else {
        *out = ident { ascii_start: id, ascii_len: len, punycode_start: c"".as_ptr(), punycode_len: 0 };
        DemangleOk
    }
}

const INVALID_SYNTAX: *const c_char = c"{invalid syntax}".as_ptr();

fn demangle_error_message(status: demangle_status) -> *const c_char {
    match status {
        DemangleInvalid => INVALID_SYNTAX,
        DemangleBug => c"{bug}".as_ptr(),
        DemangleRecursed => c"{recursion limit reached}".as_ptr(),
        _ => c"{unknown error}".as_ptr(),
    }
}

unsafe fn printer_eat(pr: *mut printer, b: u8) -> bool {
    if (*pr).status != DemangleOk {
        return false;
    }
    parser_eat(&mut (*pr).parser, b)
}

unsafe fn printer_pop_depth(pr: *mut printer) {
    if (*pr).status == DemangleOk {
        parser_pop_depth(&mut (*pr).parser);
    }
}

unsafe fn printer_print_buf(pr: *mut printer, start: *const c_char, len: size_t) -> overflow_status {
    if (*pr).out.is_null() {
        return OverflowOk;
    }
    if (*pr).out_len < len {
        return OverflowOverflow;
    }
    ptr::copy_nonoverlapping(start, (*pr).out, len);
    (*pr).out = (*pr).out.add(len);
    (*pr).out_len -= len;
    OverflowOk
}

unsafe fn printer_print_str(pr: *mut printer, buf: *const c_char) -> overflow_status {
    printer_print_buf(pr, buf, strlen(buf))
}

unsafe fn printer_print_ch(pr: *mut printer, ch: c_char) -> overflow_status {
    printer_print_buf(pr, &ch, 1)
}

unsafe fn printer_print_u64(pr: *mut printer, n: u64) -> overflow_status {
    let s = n.to_string();
    printer_print_buf(pr, s.as_ptr() as *const c_char, s.len())
}

unsafe fn printer_print_ident(pr: *mut printer, id: *mut ident) -> overflow_status {
    if (*pr).out.is_null() {
        return OverflowOk;
    }
    let mut out_len = (*pr).out_len;
    let status = display_ident((*id).ascii_start, (*id).ascii_len, (*id).punycode_start, (*id).punycode_len, (*pr).out as *mut u8, &mut out_len);
    if status != OverflowOk {
        return status;
    }
    (*pr).out = (*pr).out.add(out_len);
    (*pr).out_len -= out_len;
    OverflowOk
}

type printer_fn = unsafe fn(*mut printer) -> overflow_status;
type backref_fn = unsafe fn(*mut printer, *mut bool) -> overflow_status;

unsafe fn printer_print_backref(pr: *mut printer, func: backref_fn, arg: *mut bool) -> overflow_status {
    let mut backref = mem::zeroed::<parser>();
    parse!(pr, parser_backref, &mut backref);
    if (*pr).out.is_null() {
        return OverflowOk;
    }
    let orig_parser = (*pr).parser;
    let orig_status = (*pr).status;
    (*pr).parser = backref;
    (*pr).status = DemangleOk;
    let status = func(pr, arg);
    (*pr).parser = orig_parser;
    (*pr).status = orig_status;
    status
}

unsafe fn printer_print_lifetime_from_index(pr: *mut printer, lt: u64) -> overflow_status {
    // Bound lifetimes aren't tracked when skipping printing.
    if (*pr).out.is_null() {
        return OverflowOk;
    }
    print_str!(pr, "'");
    if lt == 0 {
        print_str!(pr, "_");
        return OverflowOk;
    }
    if ((*pr).bound_lifetime_depth as u64) < lt {
        invalid!(pr);
    } else {
        let depth = (*pr).bound_lifetime_depth as u64 - lt;
        if depth < 26 {
            print_ch!(pr, b'a' + depth as u8);
        } else {
            print_str!(pr, "_");
            print_u64!(pr, depth);
        }
        OverflowOk
    }
}

unsafe fn printer_in_binder(pr: *mut printer, func: printer_fn) -> overflow_status {
    let mut bound_lifetimes = 0u64;
    parse!(pr, parser_opt_integer_62, b'G', &mut bound_lifetimes);
    // Don't track bound lifetimes when skipping printing.
    if (*pr).out.is_null() {
        return func(pr);
    }
    if bound_lifetimes > 0 {
        print_str!(pr, "for<");
        let mut i = 0;
        while i < bound_lifetimes {
            if i > 0 {
                print_str!(pr, ", ");
            }
            (*pr).bound_lifetime_depth += 1;
            print!(printer_print_lifetime_from_index(pr, 1));
            i += 1;
        }
        print_str!(pr, "> ");
    }
    let r = func(pr);
    (*pr).bound_lifetime_depth -= bound_lifetimes as u32;
    r
}

unsafe fn printer_print_generic_arg(pr: *mut printer) -> overflow_status {
    if printer_eat(pr, b'L') {
        let mut lt = 0u64;
        parse!(pr, parser_integer_62, &mut lt);
        printer_print_lifetime_from_index(pr, lt)
    } else if printer_eat(pr, b'K') {
        printer_print_const(pr, false)
    } else {
        printer_print_type(pr)
    }
}

unsafe fn printer_print_generic_args(pr: *mut printer) -> overflow_status {
    print_str!(pr, "<");
    let mut count = 0usize;
    while (*pr).status == DemangleOk && !printer_eat(pr, b'E') {
        if count > 0 {
            print_str!(pr, ", ");
        }
        print!(printer_print_generic_arg(pr));
        count += 1;
    }
    print_str!(pr, ">");
    OverflowOk
}

unsafe fn printer_print_path_out_of_value(pr: *mut printer, _arg: *mut bool) -> overflow_status { printer_print_path(pr, false) }
unsafe fn printer_print_path_in_value(pr: *mut printer, _arg: *mut bool) -> overflow_status { printer_print_path(pr, true) }

unsafe fn printer_print_path(pr: *mut printer, in_value: bool) -> overflow_status {
    parse!(pr, parser_push_depth);
    let mut tag = 0u8;
    parse!(pr, parser_ch, &mut tag);
    let mut dis = 0u64;
    let mut name = mem::zeroed::<ident>();
    let mut ns = 0u8;
    match tag {
        b'C' => {
            parse!(pr, parser_disambiguator, &mut dis);
            parse!(pr, parser_ident, &mut name);
            print!(printer_print_ident(pr, &mut name));
            if !(*pr).out.is_null() && !(*pr).alternate && dis != 0 {
                print_str!(pr, "[");
                let s = format!("{:x}", dis);
                print!(printer_print_buf(pr, s.as_ptr() as *const c_char, s.len()));
                print_str!(pr, "]");
            }
        }
        b'N' => {
            parse!(pr, parser_namespace, &mut ns);
            let st = printer_print_path(pr, in_value);
            if st != OverflowOk {
                return st;
            }
            if (*pr).status != DemangleOk {
                print_str!(pr, "::");
            }
            parse!(pr, parser_disambiguator, &mut dis);
            parse!(pr, parser_ident, &mut name);
            // Special namespace, like closures and shims
            if ns != 0 {
                print_str!(pr, "::{");
                if ns == b'C' {
                    print_str!(pr, "closure");
                } else if ns == b'S' {
                    print_str!(pr, "shim");
                } else {
                    print_ch!(pr, ns);
                }
                if name.ascii_len != 0 || name.punycode_len != 0 {
                    print_str!(pr, ":");
                    print!(printer_print_ident(pr, &mut name));
                }
                print_str!(pr, "#");
                print_u64!(pr, dis);
                print_str!(pr, "}");
            } else {
                // Implementation-specific/unspecified namespaces
                if name.ascii_len != 0 || name.punycode_len != 0 {
                    print_str!(pr, "::");
                    print!(printer_print_ident(pr, &mut name));
                }
            }
        }
        b'M' | b'X' => {
            parse!(pr, parser_disambiguator, &mut dis);
            let orig_out = (*pr).out;
            (*pr).out = ptr::null_mut();
            print!(printer_print_path(pr, false));
            (*pr).out = orig_out;
            print_str!(pr, "<");
            print!(printer_print_type(pr));
            if tag != b'M' {
                print_str!(pr, " as ");
                print!(printer_print_path(pr, false));
            }
            print_str!(pr, ">");
        }
        b'Y' => {
            print_str!(pr, "<");
            print!(printer_print_type(pr));
            if tag != b'M' {
                print_str!(pr, " as ");
                print!(printer_print_path(pr, false));
            }
            print_str!(pr, ">");
        }
        b'I' => {
            print!(printer_print_path(pr, in_value));
            if in_value {
                print_str!(pr, "::");
            }
            print!(printer_print_generic_args(pr));
        }
        b'B' => {
            print!(printer_print_backref(pr, if in_value { printer_print_path_in_value } else { printer_print_path_out_of_value }, ptr::null_mut()));
        }
        _ => invalid!(pr),
    }
    printer_pop_depth(pr);
    OverflowOk
}

unsafe fn printer_print_const_uint(pr: *mut printer, tag: u8) -> overflow_status {
    let mut hex = mem::zeroed::<buf>();
    parse!(pr, parser_hex_nibbles, &mut hex);
    let mut val = 0u64;
    if try_parse_uint(hex.start, hex.len, &mut val) {
        print_u64!(pr, val);
    } else {
        print_str!(pr, "0x");
        print!(printer_print_buf(pr, hex.start, hex.len));
    }
    if !(*pr).out.is_null() && !(*pr).alternate {
        let ty = basic_type(tag);
        if !ty.is_null() {
            print!(printer_print_str(pr, ty));
        }
    }
    OverflowOk
}

unsafe fn printer_print_const_str_literal(pr: *mut printer) -> overflow_status {
    let mut hex = mem::zeroed::<buf>();
    parse!(pr, parser_hex_nibbles, &mut hex);
    let mut out_len = usize::MAX;
    let mut nts_status = nibbles_to_string(hex.start, hex.len, ptr::null_mut(), &mut out_len);
    match nts_status {
        NtsOk => {
            if !(*pr).out.is_null() {
                out_len = (*pr).out_len;
                nts_status = nibbles_to_string(hex.start, hex.len, (*pr).out as *mut u8, &mut out_len);
                if nts_status != NtsOk {
                    return OverflowOverflow;
                }
                (*pr).out = (*pr).out.add(out_len);
                (*pr).out_len -= out_len;
            }
            OverflowOk
        }
        NtsOverflow => OverflowOverflow,
        NtsInvalid => invalid!(pr),
    }
}

unsafe fn printer_print_const_struct(pr: *mut printer) -> overflow_status {
    let mut dis = 0u64;
    let mut name = mem::zeroed::<ident>();
    parse!(pr, parser_disambiguator, &mut dis);
    parse!(pr, parser_ident, &mut name);
    print!(printer_print_ident(pr, &mut name));
    print_str!(pr, ": ");
    printer_print_const(pr, true)
}

unsafe fn printer_print_const_out_of_value(pr: *mut printer, _arg: *mut bool) -> overflow_status { printer_print_const(pr, false) }
unsafe fn printer_print_const_in_value(pr: *mut printer, _arg: *mut bool) -> overflow_status { printer_print_const(pr, true) }

unsafe fn printer_print_const(pr: *mut printer, in_value: bool) -> overflow_status {
    let mut tag = 0u8;
    parse!(pr, parser_ch, &mut tag);
    parse!(pr, parser_push_depth);
    let mut hex = mem::zeroed::<buf>();
    let mut val = 0u64;
    let mut count: u64;
    let mut opened_brace = false;
    macro_rules! open_brace_if_outside_expr {
        () => {{
            if !in_value {
                opened_brace = true;
                print_str!(pr, "{");
            }
        }};
    }
    match tag {
        b'p' => print_str!(pr, "_"),
        b'a' | b's' | b'l' | b'x' | b'n' | b'i' => {
            if printer_eat(pr, b'n') {
                print_str!(pr, "-");
            }
            print!(printer_print_const_uint(pr, tag));
        }
        b'h' | b't' | b'm' | b'y' | b'o' | b'j' => print!(printer_print_const_uint(pr, tag)),
        b'b' => {
            parse!(pr, parser_hex_nibbles, &mut hex);
            if try_parse_uint(hex.start, hex.len, &mut val) {
                if val == 0 {
                    print_str!(pr, "false");
                } else if val == 1 {
                    print_str!(pr, "true");
                } else {
                    invalid!(pr);
                }
            } else {
                invalid!(pr);
            }
        }
        b'c' => {
            parse!(pr, parser_hex_nibbles, &mut hex);
            if try_parse_uint(hex.start, hex.len, &mut val) && val < u32::MAX as u64 && validate_char(val as u32) {
                let mut escaped_buf = [0 as c_char; ESCAPED_SIZE];
                let escaped_size = char_to_string(val as u32, b'\'', true, &mut escaped_buf);
                print_str!(pr, "'");
                print!(printer_print_buf(pr, escaped_buf.as_ptr(), escaped_size));
                print_str!(pr, "'");
            } else {
                invalid!(pr);
            }
        }
        b'e' => {
            open_brace_if_outside_expr!();
            print_str!(pr, "*");
            print!(printer_print_const_str_literal(pr));
        }
        b'R' | b'Q' => {
            if tag == b'R' && printer_eat(pr, b'e') {
                print!(printer_print_const_str_literal(pr));
            } else {
                open_brace_if_outside_expr!();
                print_str!(pr, "&");
                if tag != b'R' {
                    print_str!(pr, "mut ");
                }
                print!(printer_print_const(pr, true));
            }
        }
        b'A' => {
            open_brace_if_outside_expr!();
            print_str!(pr, "[");
            count = 0;
            while (*pr).status == DemangleOk && !printer_eat(pr, b'E') {
                if count > 0 { print_str!(pr, ", "); }
                print!(printer_print_const(pr, true));
                count += 1;
            }
            print_str!(pr, "]");
        }
        b'T' => {
            open_brace_if_outside_expr!();
            print_str!(pr, "(");
            count = 0;
            while (*pr).status == DemangleOk && !printer_eat(pr, b'E') {
                if count > 0 { print_str!(pr, ", "); }
                print!(printer_print_const(pr, true));
                count += 1;
            }
            if count == 1 { print_str!(pr, ","); }
            print_str!(pr, ")");
        }
        b'V' => {
            open_brace_if_outside_expr!();
            print!(printer_print_path(pr, true));
            parse!(pr, parser_ch, &mut tag);
            match tag {
                b'U' => {}
                b'T' => {
                    print_str!(pr, "(");
                    count = 0;
                    while (*pr).status == DemangleOk && !printer_eat(pr, b'E') {
                        if count > 0 { print_str!(pr, ", "); }
                        print!(printer_print_const(pr, true));
                        count += 1;
                    }
                    print_str!(pr, ")");
                }
                b'S' => {
                    print_str!(pr, " { ");
                    count = 0;
                    while (*pr).status == DemangleOk && !printer_eat(pr, b'E') {
                        if count > 0 { print_str!(pr, ", "); }
                        print!(printer_print_const_struct(pr));
                        count += 1;
                    }
                    print_str!(pr, " }");
                }
                _ => invalid!(pr),
            }
        }
        b'B' => print!(printer_print_backref(pr, if in_value { printer_print_const_in_value } else { printer_print_const_out_of_value }, ptr::null_mut())),
        _ => invalid!(pr),
    }
    if opened_brace {
        print_str!(pr, "}");
    }
    printer_pop_depth(pr);
    OverflowOk
}

/// A trait in a trait object may have some "existential projections"
/// (i.e. associated type bindings) after it, which should be printed
/// in the `<...>` of the trait, e.g. `dyn Trait<T, U, Assoc=X>`.
/// To this end, this method will keep the `<...>` of an 'I' path
/// open, by omitting the `>`, and return `Ok(true)` in that case.
unsafe fn printer_print_maybe_open_generics(pr: *mut printer, open: *mut bool) -> overflow_status {
    if printer_eat(pr, b'B') {
        // NOTE(eddyb) the closure may not run if printing is being skipped,
        // but in that case the returned boolean doesn't matter.
        *open = false;
        printer_print_backref(pr, printer_print_maybe_open_generics, open)
    } else if printer_eat(pr, b'I') {
        print!(printer_print_path(pr, false));
        print_str!(pr, "<");
        let mut count = 0usize;
        while (*pr).status == DemangleOk && !printer_eat(pr, b'E') {
            if count > 0 { print_str!(pr, ", "); }
            print!(printer_print_generic_arg(pr));
            count += 1;
        }
        *open = true;
        OverflowOk
    } else {
        print!(printer_print_path(pr, false));
        *open = false;
        OverflowOk
    }
}

unsafe fn printer_print_dyn_trait(pr: *mut printer) -> overflow_status {
    let mut open = false;
    print!(printer_print_maybe_open_generics(pr, &mut open));
    while printer_eat(pr, b'p') {
        if !open {
            print_str!(pr, "<");
            open = true;
        } else {
            print_str!(pr, ", ");
        }
        let mut name = mem::zeroed::<ident>();
        parse!(pr, parser_ident, &mut name);
        print!(printer_print_ident(pr, &mut name));
        print_str!(pr, " = ");
        print!(printer_print_type(pr));
    }
    if open {
        print_str!(pr, ">");
    }
    OverflowOk
}

unsafe fn printer_print_object_bounds(pr: *mut printer) -> overflow_status {
    let mut count = 0usize;
    while (*pr).status == DemangleOk && !printer_eat(pr, b'E') {
        if count > 0 { print_str!(pr, " + "); }
        print!(printer_print_dyn_trait(pr));
        count += 1;
    }
    OverflowOk
}

unsafe fn printer_print_function_type(pr: *mut printer) -> overflow_status {
    let is_unsafe = printer_eat(pr, b'U');
    let abi: *const c_char;
    let mut abi_len: usize;
    if printer_eat(pr, b'K') {
        if printer_eat(pr, b'C') {
            abi = c"C".as_ptr();
            abi_len = 1;
        } else {
            let mut abi_ident = mem::zeroed::<ident>();
            parse!(pr, parser_ident, &mut abi_ident);
            if abi_ident.ascii_len == 0 || abi_ident.punycode_len != 0 {
                invalid!(pr);
            }
            abi = abi_ident.ascii_start;
            abi_len = abi_ident.ascii_len;
        }
    } else {
        abi = ptr::null();
        abi_len = 0;
    }
    if is_unsafe {
        print_str!(pr, "unsafe ");
    }
    if !abi.is_null() {
        print_str!(pr, "extern \"");
        // replace _ with -
        let mut abi_cur = abi;
        while abi_len > 0 {
            let minus = memchr_(abi_cur, b'_' as c_char, abi_len);
            if minus.is_null() {
                print!(printer_print_buf(pr, abi_cur, abi_len));
                break;
            } else {
                let space_to_minus = minus.offset_from(abi_cur) as usize;
                print!(printer_print_buf(pr, abi_cur, space_to_minus));
                print_str!(pr, "-");
                abi_cur = minus.add(1);
                abi_len -= space_to_minus + 1;
            }
        }
        print_str!(pr, "\" ");
    }
    print_str!(pr, "fn(");
    let mut count = 0usize;
    while (*pr).status == DemangleOk && !printer_eat(pr, b'E') {
        if count > 0 { print_str!(pr, ", "); }
        print!(printer_print_type(pr));
        count += 1;
    }
    print_str!(pr, ")");
    if printer_eat(pr, b'u') {
        // Skip printing the return type if it's 'u', i.e. `()`.
    } else {
        print_str!(pr, " -> ");
        print!(printer_print_type(pr));
    }
    OverflowOk
}

unsafe fn printer_print_type_backref(pr: *mut printer, _arg: *mut bool) -> overflow_status { printer_print_type(pr) }

unsafe fn printer_print_type(pr: *mut printer) -> overflow_status {
    let mut tag = 0u8;
    parse!(pr, parser_ch, &mut tag);
    let basic_ty = basic_type(tag);
    if !basic_ty.is_null() {
        return printer_print_str(pr, basic_ty);
    }
    let mut count = 0u64;
    let mut lt = 0u64;
    parse!(pr, parser_push_depth);
    match tag {
        b'R' | b'Q' => {
            print_str!(pr, "&");
            if printer_eat(pr, b'L') {
                parse!(pr, parser_integer_62, &mut lt);
                if lt != 0 {
                    print!(printer_print_lifetime_from_index(pr, lt));
                    print_str!(pr, " ");
                }
            }
            if tag != b'R' {
                print_str!(pr, "mut ");
            }
            print!(printer_print_type(pr));
        }
        b'P' | b'O' => {
            print_str!(pr, "*");
            if tag != b'P' { print_str!(pr, "mut "); } else { print_str!(pr, "const "); }
            print!(printer_print_type(pr));
        }
        b'A' | b'S' => {
            print_str!(pr, "[");
            print!(printer_print_type(pr));
            if tag == b'A' {
                print_str!(pr, "; ");
                print!(printer_print_const(pr, true));
            }
            print_str!(pr, "]");
        }
        b'T' => {
            print_str!(pr, "(");
            count = 0;
            while (*pr).status == DemangleOk && !printer_eat(pr, b'E') {
                if count > 0 { print_str!(pr, ", "); }
                print!(printer_print_type(pr));
                count += 1;
            }
            if count == 1 { print_str!(pr, ","); }
            print_str!(pr, ")");
        }
        b'F' => print!(printer_in_binder(pr, printer_print_function_type)),
        b'D' => {
            print_str!(pr, "dyn ");
            print!(printer_in_binder(pr, printer_print_object_bounds));
            if !printer_eat(pr, b'L') {
                invalid!(pr);
            }
            parse!(pr, parser_integer_62, &mut lt);
            if lt != 0 {
                print_str!(pr, " + ");
                print!(printer_print_lifetime_from_index(pr, lt));
            }
        }
        b'B' => print!(printer_print_backref(pr, printer_print_type_backref, ptr::null_mut())),
        _ => {
            // Go back to the tag, so `print_path` also sees it.
            if (*pr).status == DemangleOk && (*pr).parser.next > 0 {
                (*pr).parser.next -= 1;
            }
            print!(printer_print_path(pr, false));
        }
    }
    printer_pop_depth(pr);
    OverflowOk
}

unsafe fn rust_demangle_legacy_demangle(
    s: *const c_char,
    s_len: size_t,
    res: *mut demangle_legacy,
    rest: *mut *const c_char,
) -> demangle_status {
    if s_len > strlen(s) {
        // s_len only exists to shorten the string, this is not a buffer API
        return DemangleInvalid;
    }
    let inner: *const c_char;
    let inner_len: size_t;
    if s_len >= 3 && strncmp_lit(s, b"_ZN") {
        inner = s.add(3);
        inner_len = s_len - 3;
    } else if s_len >= 2 && strncmp_lit(s, b"ZN") {
        // On Windows, dbghelp strips leading underscores, so we accept "ZN...E"
        // form too.
        inner = s.add(2);
        inner_len = s_len - 2;
    } else if s_len >= 4 && strncmp_lit(s, b"__ZN") {
        // On OSX, symbols are prefixed with an extra _
        inner = s.add(4);
        inner_len = s_len - 4;
    } else {
        return DemangleInvalid;
    }
    if !str_isascii(inner, inner_len) {
        return DemangleInvalid;
    }
    let mut elements = 0usize;
    let mut chars = inner;
    let mut chars_len = inner_len;
    if chars_len == 0 {
        return DemangleInvalid;
    }
    let mut c = *chars;
    while c != b'E' as c_char {
        // Decode an identifier element's length
        if c < b'0' as c_char || c > b'9' as c_char {
            return DemangleInvalid;
        }
        let mut len = 0usize;
        while c >= b'0' as c_char && c <= b'9' as c_char {
            let d = (c - b'0' as c_char) as usize;
            if len > usize::MAX / 10 {
                return DemangleInvalid;
            }
            len *= 10;
            if len > usize::MAX - d {
                return DemangleInvalid;
            }
            len += d;
            chars = chars.add(1);
            chars_len -= 1;
            if chars_len == 0 {
                return DemangleInvalid;
            }
            c = *chars;
        }
        // Advance by the length
        if chars_len <= len {
            return DemangleInvalid;
        }
        chars = chars.add(len);
        chars_len -= len;
        elements += 1;
        c = *chars;
    }
    *res = demangle_legacy { mangled: inner, mangled_len: inner_len, elements };
    *rest = chars.add(1);
    DemangleOk
}

unsafe fn is_rust_hash(s: *const c_char, len: size_t) -> bool {
    if len == 0 || *s != b'h' as c_char {
        return false;
    }
    let mut i = 1usize;
    while i < len {
        let ch = *s.add(i);
        if !((ch >= b'0' as c_char && ch <= b'9' as c_char)
            || (ch >= b'a' as c_char && ch <= b'f' as c_char)
            || (ch >= b'A' as c_char && ch <= b'F' as c_char))
        {
            return false;
        }
        i += 1;
    }
    true
}

unsafe fn rust_demangle_legacy_display_demangle(
    res: demangle_legacy,
    out: *mut c_char,
    mut len: size_t,
    alternate: bool,
) -> overflow_status {
    let mut pr = printer {
        // not actually using the parser part of the printer, just keeping it to share the format functions
        status: DemangleOk,
        parser: parser { sym: ptr::null(), sym_len: 0, next: 0, depth: 0 },
        out,
        out_len: len,
        bound_lifetime_depth: 0,
        alternate,
    };
    let mut inner = res.mangled;
    let mut element = 0usize;
    while element < res.elements {
        let mut i = 0usize;
        let mut rest = inner;
        while rest < res.mangled.add(res.mangled_len) && *rest >= b'0' as c_char && *rest <= b'9' as c_char {
            i *= 10;
            i += (*rest - b'0' as c_char) as usize;
            rest = rest.add(1);
        }
        if res.mangled.add(res.mangled_len).offset_from(rest) as usize as size_t < i {
            // safety: shouldn't reach this place if the input string is validated. bail out.
            // safety: we knwo rest <= res.mangled + res.mangled_len from the for-loop above
            break;
        }
        len = i;
        inner = rest.add(len);
        // From here on, inner contains a pointer to the next element, rest[:len] to the current one
        if alternate && element + 1 == res.elements && is_rust_hash(rest, i) {
            break;
        }
        if element != 0 {
            print_str!(&mut pr, "::");
        }
        if len >= 2 && *rest == b'_' as c_char && *rest.add(1) == b'$' as c_char {
            rest = rest.add(1);
            len -= 1;
        }
        while len > 0 {
            if *rest == b'.' as c_char {
                if len >= 2 && *rest.add(1) == b'.' as c_char {
                    print_str!(&mut pr, "::");
                    rest = rest.add(2);
                    len -= 2;
                } else {
                    print_str!(&mut pr, ".");
                    rest = rest.add(1);
                    len -= 1;
                }
            } else if *rest == b'$' as c_char {
                let escape = memchr_(rest.add(1), b'$' as c_char, len - 1);
                if escape.is_null() {
                    break;
                }
                let mut escape_start = rest.add(1);
                let mut escape_len = escape.offset_from(rest.add(1)) as usize;
                let next_len = len - (escape.add(1).offset_from(rest) as usize);
                let next_rest = escape.add(1);
                let ch: c_char;
                if escape_len == 2 && *escape_start == b'S' as c_char && *escape_start.add(1) == b'P' as c_char {
                    ch = b'@' as c_char;
                } else if escape_len == 2 && *escape_start == b'B' as c_char && *escape_start.add(1) == b'P' as c_char {
                    ch = b'*' as c_char;
                } else if escape_len == 2 && *escape_start == b'R' as c_char && *escape_start.add(1) == b'F' as c_char {
                    ch = b'&' as c_char;
                } else if escape_len == 2 && *escape_start == b'L' as c_char && *escape_start.add(1) == b'T' as c_char {
                    ch = b'<' as c_char;
                } else if escape_len == 2 && *escape_start == b'G' as c_char && *escape_start.add(1) == b'T' as c_char {
                    ch = b'>' as c_char;
                } else if escape_len == 2 && *escape_start == b'L' as c_char && *escape_start.add(1) == b'P' as c_char {
                    ch = b'(' as c_char;
                } else if escape_len == 2 && *escape_start == b'R' as c_char && *escape_start.add(1) == b'P' as c_char {
                    ch = b')' as c_char;
                } else if escape_len == 1 && *escape_start == b'C' as c_char {
                    ch = b',' as c_char;
                } else {
                    if escape_len > 1 && *escape_start == b'u' as c_char {
                        escape_start = escape_start.add(1);
                        escape_len -= 1;
                        let mut val = 0u64;
                        if try_parse_uint(escape_start, escape_len, &mut val)
                            && val < u32::MAX as u64
                            && validate_char(val as u32)
                        {
                            if !unicode_iscontrol(val as u32) {
                                let mut wchr = [0u8; 4];
                                let wchr_len = code_to_utf8(wchr.as_mut_ptr(), val as u32);
                                print!(printer_print_buf(&mut pr, wchr.as_ptr() as *const c_char, wchr_len));
                                len = next_len;
                                rest = next_rest;
                                continue;
                            }
                        }
                    }
                    break; // print the rest of this element raw
                }
                print_ch!(&mut pr, ch as u8);
                len = next_len;
                rest = next_rest;
            } else {
                let mut j = 0usize;
                while j < len && *rest.add(j) != b'$' as c_char && *rest.add(j) != b'.' as c_char {
                    j += 1;
                }
                if j == len {
                    break;
                }
                print!(printer_print_buf(&mut pr, rest, j));
                rest = rest.add(j);
                len -= j;
            }
        }
        print!(printer_print_buf(&mut pr, rest, len));
        element += 1;
    }
    if pr.out_len < OVERFLOW_MARGIN {
        return OverflowOverflow;
    }
    *pr.out = 0;
    OverflowOk
}

unsafe fn is_symbol_like(s: *const c_char, len: size_t) -> bool {
    // rust-demangle definition of symbol like: control characters and space are not symbol-like, all else is
    let mut i = 0usize;
    while i < len {
        let ch = *s.add(i);
        if !(ch >= 0x21 && ch <= 0x7e) {
            return false;
        }
        i += 1;
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn rust_demangle_demangle(s: *const c_char, res: *mut demangle) {
    // During ThinLTO LLVM may import and rename internal symbols, so strip out
    // those endings first as they're one of the last manglings applied to symbol
    // names.
    let llvm = b".llvm.";
    let found_llvm = strstr_(s, llvm);
    let mut s_len = strlen(s);
    if !found_llvm.is_null() {
        let mut all_hex_ptr = found_llvm.add(llvm.len());
        let mut all_hex = true;
        while *all_hex_ptr != 0 {
            if !((*all_hex_ptr >= b'0' as c_char && *all_hex_ptr <= b'9' as c_char)
                || (*all_hex_ptr >= b'A' as c_char && *all_hex_ptr <= b'F' as c_char)
                || *all_hex_ptr == b'@' as c_char)
            {
                all_hex = false;
                break;
            }
            all_hex_ptr = all_hex_ptr.add(1);
        }
        if all_hex {
            s_len = found_llvm.offset_from(s) as usize;
        }
    }
    let mut suffix: *const c_char = ptr::null();
    let mut legacy = mem::zeroed::<demangle_legacy>();
    let mut st = rust_demangle_legacy_demangle(s, s_len, &mut legacy, &mut suffix);
    if st == DemangleOk {
        *res = demangle {
            style: DemangleStyleLegacy,
            mangled: legacy.mangled,
            mangled_len: legacy.mangled_len,
            elements: legacy.elements,
            original: s,
            original_len: s_len,
            suffix,
            suffix_len: s_len - suffix.offset_from(s) as usize,
        };
    } else {
        let mut v0 = mem::zeroed::<demangle_v0>();
        st = rust_demangle_v0_demangle(s, s_len, &mut v0, &mut suffix);
        if st == DemangleOk {
            *res = demangle {
                style: DemangleStyleV0,
                mangled: v0.mangled,
                mangled_len: v0.mangled_len,
                elements: 0,
                original: s,
                original_len: s_len,
                suffix,
                suffix_len: s_len - suffix.offset_from(s) as usize,
            };
        } else {
            *res = demangle {
                style: DemangleStyleUnknown,
                mangled: ptr::null(),
                mangled_len: 0,
                elements: 0,
                original: s,
                original_len: s_len,
                suffix: s,
                suffix_len: 0,
            };
        }
    }
    // Output like LLVM IR adds extra period-delimited words. See if
    // we are in that case and save the trailing words if so.
    if (*res).suffix_len != 0 {
        if *(*res).suffix == b'.' as c_char && is_symbol_like((*res).suffix, (*res).suffix_len) {
            // Keep the suffix
        } else {
            // Reset the suffix and invalidate the demangling
            (*res).style = DemangleStyleUnknown;
            (*res).suffix_len = 0;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn rust_demangle_is_known(res: *mut demangle) -> bool {
    (*res).style != DemangleStyleUnknown
}

#[no_mangle]
pub unsafe extern "C" fn rust_demangle_display_demangle(
    res: *const demangle,
    mut out: *mut c_char,
    mut len: size_t,
    alternate: bool,
) -> overflow_status {
    let original_len = (*res).original_len;
    let out_len: size_t;
    match (*res).style {
        DemangleStyleUnknown => {
            if len < original_len {
                return OverflowOverflow;
            } else {
                ptr::copy_nonoverlapping((*res).original, out, original_len);
                out = out.add(original_len);
                len -= original_len;
            }
        }
        DemangleStyleLegacy => {
            let legacy = demangle_legacy { mangled: (*res).mangled, mangled_len: (*res).mangled_len, elements: (*res).elements };
            if rust_demangle_legacy_display_demangle(legacy, out, len, alternate) == OverflowOverflow {
                return OverflowOverflow;
            }
            out_len = strlen(out);
            out = out.add(out_len);
            len -= out_len;
        }
        DemangleStyleV0 => {
            let v0 = demangle_v0 { mangled: (*res).mangled, mangled_len: (*res).mangled_len };
            if rust_demangle_v0_display_demangle(v0, out, len, alternate) == OverflowOverflow {
                return OverflowOverflow;
            }
            out_len = strlen(out);
            out = out.add(out_len);
            len -= out_len;
        }
    }
    let suffix_len = (*res).suffix_len;
    if len < suffix_len || len - suffix_len < OVERFLOW_MARGIN {
        return OverflowOverflow;
    }
    ptr::copy_nonoverlapping((*res).suffix, out, suffix_len);
    *out.add(suffix_len) = 0;
    OverflowOk
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
