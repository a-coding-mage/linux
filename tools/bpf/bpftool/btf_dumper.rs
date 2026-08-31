// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (c) 2018 Facebook */

use core::ffi::{c_char, c_int, c_longlong, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type __u8 = u8;
type __s8 = i8;
type __u16 = u16;
type __s16 = i16;
type __u32 = u32;
type __s32 = i32;
type __u64 = u64;
type __s64 = i64;
type size_t = usize;
type json_writer_t = c_void;

const EINVAL: c_int = 22;
const UINT32_MAX: c_ulong = u32::MAX as c_ulong;
const BITS_PER_BYTE: c_int = 8;
const BITS_PER_BYTE_MASK: c_int = BITS_PER_BYTE - 1;

const BTF_INT_SIGNED: __u32 = 1 << 0;
const BTF_INT_CHAR: __u32 = 1 << 1;
const BTF_INT_BOOL: __u32 = 1 << 2;

const BTF_KIND_UNKN: c_int = 0;
const BTF_KIND_INT: c_int = 1;
const BTF_KIND_PTR: c_int = 2;
const BTF_KIND_ARRAY: c_int = 3;
const BTF_KIND_STRUCT: c_int = 4;
const BTF_KIND_UNION: c_int = 5;
const BTF_KIND_ENUM: c_int = 6;
const BTF_KIND_FWD: c_int = 7;
const BTF_KIND_TYPEDEF: c_int = 8;
const BTF_KIND_VOLATILE: c_int = 9;
const BTF_KIND_CONST: c_int = 10;
const BTF_KIND_RESTRICT: c_int = 11;
const BTF_KIND_FUNC: c_int = 12;
const BTF_KIND_FUNC_PROTO: c_int = 13;
const BTF_KIND_VAR: c_int = 14;
const BTF_KIND_DATASEC: c_int = 15;
const BTF_KIND_FLOAT: c_int = 16;
const BTF_KIND_ENUM64: c_int = 19;

const BTF_VAR_STATIC: __u32 = 0;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    pub info: __u32,
    pub size: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct btf_enum {
    pub name_off: __u32,
    pub val: __s32,
}

#[repr(C)]
pub struct btf_enum64 {
    pub name_off: __u32,
    pub val_lo32: __u32,
    pub val_hi32: __u32,
}

#[repr(C)]
pub struct btf_array {
    pub type_: __u32,
    pub index_type: __u32,
    pub nelems: __u32,
}

#[repr(C)]
pub struct btf_member {
    pub name_off: __u32,
    pub type_: __u32,
    pub offset: __u32,
}

#[repr(C)]
pub struct btf_var {
    pub linkage: __u32,
}

#[repr(C)]
pub struct btf_var_secinfo {
    pub type_: __u32,
    pub offset: __u32,
    pub size: __u32,
}

#[repr(C)]
pub struct btf_param {
    pub name_off: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct bpf_func_info {
    pub insn_off: __u32,
    pub type_id: __u32,
}

#[repr(C)]
pub struct bpf_prog_info {
    pub type_: __u32,
    pub id: __u32,
    pub tag: [__u8; 8],
    pub jited_prog_len: __u32,
    pub xlated_prog_len: __u32,
    pub jited_prog_insns: __u64,
    pub xlated_prog_insns: __u64,
    pub load_time: __u64,
    pub created_by_uid: __u32,
    pub nr_map_ids: __u32,
    pub map_ids: __u64,
    pub name: [c_char; 16],
    pub ifindex: __u32,
    pub gpl_compatible: __u32,
    pub netns_dev: __u64,
    pub netns_ino: __u64,
    pub nr_jited_ksyms: __u32,
    pub nr_jited_func_lens: __u32,
    pub jited_ksyms: __u64,
    pub jited_func_lens: __u64,
    pub btf_id: __u32,
    pub func_info_rec_size: __u32,
    pub func_info: __u64,
    pub nr_func_info: __u32,
}

#[repr(C)]
pub struct bpf_line_info {
    pub insn_off: __u32,
    pub file_name_off: __u32,
    pub line_off: __u32,
    pub line_col: __u32,
}

#[repr(C)]
pub struct btf_dumper {
    pub btf: *const btf,
    pub jw: *mut json_writer_t,
    pub is_plain_text: bool,
    pub prog_id_as_func_ptr: bool,
}

unsafe extern "C" {
    static mut json_wtr: *mut json_writer_t;

    fn bpf_prog_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;
    fn btf__load_from_kernel_by_id(id: __u32) -> *mut btf;
    fn btf__type_by_id(btf: *const btf, type_id: __u32) -> *const btf_type;
    fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf__resolve_type(btf: *const btf, type_id: __u32) -> __s32;
    fn btf__resolve_size(btf: *const btf, type_id: __u32) -> c_longlong;
    fn btf__free(btf: *mut btf);
    fn jsonw_string(jw: *mut json_writer_t, s: *const c_char);
    fn jsonw_printf(jw: *mut json_writer_t, fmt: *const c_char, ...);
    fn jsonw_int(jw: *mut json_writer_t, n: __s64);
    fn jsonw_bool(jw: *mut json_writer_t, b: bool);
    fn jsonw_start_array(jw: *mut json_writer_t);
    fn jsonw_end_array(jw: *mut json_writer_t);
    fn jsonw_start_object(jw: *mut json_writer_t);
    fn jsonw_end_object(jw: *mut json_writer_t);
    fn jsonw_name(jw: *mut json_writer_t, name: *const c_char);
    fn jsonw_string_field(jw: *mut json_writer_t, name: *const c_char, s: *const c_char);
    fn jsonw_int_field(jw: *mut json_writer_t, name: *const c_char, n: __s64);
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn putchar(c: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn isspace(c: c_int) -> c_int;
    fn isprint(c: c_int) -> c_int;
}

fn ptr_to_u64<T>(ptr: *const T) -> __u64 {
    ptr as __u64
}

fn BITS_PER_BYTE_MASKED(bits: c_int) -> c_int {
    bits & BITS_PER_BYTE_MASK
}

fn BITS_ROUNDDOWN_BYTES(bits: c_int) -> c_int {
    bits >> 3
}

fn BITS_ROUNDUP_BYTES(bits: c_int) -> c_int {
    BITS_ROUNDDOWN_BYTES(bits) + if BITS_PER_BYTE_MASKED(bits) != 0 { 1 } else { 0 }
}

unsafe fn btf_vlen(t: *const btf_type) -> c_int {
    ((*t).info & 0xffff) as c_int
}

unsafe fn btf_kind(t: *const btf_type) -> c_int {
    (((*t).info >> 24) & 0x1f) as c_int
}

unsafe fn btf_kflag(t: *const btf_type) -> c_int {
    (((*t).info >> 31) & 1) as c_int
}

unsafe fn btf_is_mod(t: *const btf_type) -> bool {
    matches!(btf_kind(t), BTF_KIND_VOLATILE | BTF_KIND_CONST | BTF_KIND_RESTRICT)
}

unsafe fn btf_is_int(t: *const btf_type) -> bool {
    btf_kind(t) == BTF_KIND_INT
}

unsafe fn btf_is_func(t: *const btf_type) -> bool {
    btf_kind(t) == BTF_KIND_FUNC
}

unsafe fn btf_is_func_proto(t: *const btf_type) -> bool {
    btf_kind(t) == BTF_KIND_FUNC_PROTO
}

unsafe fn btf_enum(t: *const btf_type) -> *const btf_enum {
    t.add(1) as *const btf_enum
}

unsafe fn btf_enum64(t: *const btf_type) -> *const btf_enum64 {
    t.add(1) as *const btf_enum64
}

fn BTF_INT_ENCODING(x: __u32) -> __u32 {
    x & 0x0f000000
}

fn BTF_INT_OFFSET(x: __u32) -> __u32 {
    (x & 0x00ff0000) >> 16
}

fn BTF_INT_BITS(x: __u32) -> __u32 {
    x & 0x000000ff
}

fn BTF_MEMBER_BITFIELD_SIZE(x: __u32) -> __u32 {
    x >> 24
}

fn BTF_MEMBER_BIT_OFFSET(x: __u32) -> __u32 {
    x & 0x00ffffff
}

fn BPF_LINE_INFO_LINE_NUM(x: __u32) -> __u32 {
    x >> 10
}

fn BPF_LINE_INFO_LINE_COL(x: __u32) -> __u32 {
    x & 0x3ff
}

unsafe fn btf_dumper_do_type(d: *const btf_dumper, type_id: __u32, bit_offset: __u8, data: *const c_void) -> c_int {
    let t = btf__type_by_id((*d).btf, type_id);

    match btf_kind(t) {
        BTF_KIND_INT => btf_dumper_int(t, bit_offset, data, (*d).jw, (*d).is_plain_text),
        BTF_KIND_STRUCT | BTF_KIND_UNION => btf_dumper_struct(d, type_id, data),
        BTF_KIND_ARRAY => btf_dumper_array(d, type_id, data),
        BTF_KIND_ENUM => btf_dumper_enum(d, t, data),
        BTF_KIND_ENUM64 => btf_dumper_enum64(d, t, data),
        BTF_KIND_PTR => {
            btf_dumper_ptr(d, t, data);
            0
        }
        BTF_KIND_UNKN => {
            jsonw_printf((*d).jw, c"(unknown)".as_ptr());
            0
        }
        BTF_KIND_FWD => {
            /* map key or value can't be forward */
            jsonw_printf((*d).jw, c"(fwd-kind-invalid)".as_ptr());
            -EINVAL
        }
        BTF_KIND_TYPEDEF | BTF_KIND_VOLATILE | BTF_KIND_CONST | BTF_KIND_RESTRICT => {
            btf_dumper_modifier(d, type_id, bit_offset, data)
        }
        BTF_KIND_VAR => btf_dumper_var(d, type_id, bit_offset, data),
        BTF_KIND_DATASEC => btf_dumper_datasec(d, type_id, data),
        _ => {
            jsonw_printf((*d).jw, c"(unsupported-kind)".as_ptr());
            -EINVAL
        }
    }
}

macro_rules! BTF_PRINT_ARG {
    ($func_sig:expr, $pos:expr, $size:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        let mut __pos = $pos;
        __pos += snprintf(
            $func_sig.offset(__pos as isize),
            ($size - __pos) as size_t,
            $fmt
            $(, $arg)*,
        );
        if __pos >= $size {
            -1
        } else {
            __pos
        }
    }};
}

unsafe fn btf_dump_func(
    btf: *const btf,
    func_sig: *mut c_char,
    func_proto: *const btf_type,
    func: *const btf_type,
    pos: c_int,
    size: c_int,
) -> c_int {
    let mut pos = BTF_PRINT_TYPE(btf, (*func_proto).type_, func_sig, pos, size);
    if pos == -1 {
        return -1;
    }
    if !func.is_null() {
        pos = BTF_PRINT_ARG!(func_sig, pos, size, c"%s(".as_ptr(), btf__name_by_offset(btf, (*func).name_off));
    } else {
        pos = BTF_PRINT_ARG!(func_sig, pos, size, c"(".as_ptr());
    }
    if pos == -1 {
        return -1;
    }
    let vlen = btf_vlen(func_proto);
    let mut i = 0;
    while i < vlen {
        let arg = (func_proto.add(1) as *const btf_param).add(i as usize);

        if i != 0 {
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c", ".as_ptr());
            if pos == -1 {
                return -1;
            }
        }
        if (*arg).type_ != 0 {
            pos = BTF_PRINT_TYPE(btf, (*arg).type_, func_sig, pos, size);
            if pos == -1 {
                return -1;
            }
            if (*arg).name_off != 0 {
                pos = BTF_PRINT_ARG!(func_sig, pos, size, c"%s".as_ptr(), btf__name_by_offset(btf, (*arg).name_off));
                if pos == -1 {
                    return -1;
                }
            } else if pos != 0 && *func_sig.offset((pos - 1) as isize) == b' ' as c_char {
                /* Remove unnecessary space for
                 * FUNC_PROTO that does not have
                 * arg->name_off
                 */
                pos -= 1;
                *func_sig.offset(pos as isize) = 0;
            }
        } else {
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c"...".as_ptr());
            if pos == -1 {
                return -1;
            }
        }
        i += 1;
    }
    BTF_PRINT_ARG!(func_sig, pos, size, c")".as_ptr())
}

unsafe fn BTF_PRINT_TYPE(btf: *const btf, type_id: __u32, func_sig: *mut c_char, pos: c_int, size: c_int) -> c_int {
    __btf_dumper_type_only(btf, type_id, func_sig, pos, size)
}

unsafe fn dump_prog_id_as_func_ptr(d: *const btf_dumper, func_proto: *const btf_type, prog_id: __u32) -> c_int {
    let mut func_type: *const btf_type;
    let mut prog_fd: c_int = -1;
    let func_sig_len: c_int;
    let mut info: bpf_prog_info = zeroed();
    let mut info_len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let mut prog_name: *const c_char = ptr::null();
    let mut prog_btf: *mut btf = ptr::null_mut();
    let mut finfo: bpf_func_info = zeroed();
    let finfo_rec_size: __u32;
    let mut prog_str = [0 as c_char; 1024];
    let mut err: c_int;

    /* Get the ptr's func_proto */
    func_sig_len = btf_dump_func((*d).btf, prog_str.as_mut_ptr(), func_proto, ptr::null(), 0, size_of::<[c_char; 1024]>() as c_int);
    if func_sig_len == -1 {
        return -1;
    }

    if prog_id != 0 {
        /* Get the bpf_prog's name.  Obtain from func_info. */
        prog_fd = bpf_prog_get_fd_by_id(prog_id);
        if prog_fd >= 0 {
            err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len);
            if err == 0 && info.btf_id != 0 && info.nr_func_info != 0 {
                finfo_rec_size = info.func_info_rec_size;
                memset(&mut info as *mut _ as *mut c_void, 0, size_of::<bpf_prog_info>());
                info.nr_func_info = 1;
                info.func_info_rec_size = finfo_rec_size;
                info.func_info = ptr_to_u64(&finfo);

                err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len);
                if err == 0 {
                    prog_btf = btf__load_from_kernel_by_id(info.btf_id);
                    if !prog_btf.is_null() {
                        func_type = btf__type_by_id(prog_btf, finfo.type_id);
                        if !func_type.is_null() && btf_is_func(func_type) {
                            prog_name = btf__name_by_offset(prog_btf, (*func_type).name_off);
                        }
                    }
                }
            }
        }
    }

    if prog_id == 0 {
        snprintf(
            prog_str.as_mut_ptr().offset(func_sig_len as isize),
            size_of::<[c_char; 1024]>() - func_sig_len as usize,
            c" 0".as_ptr(),
        );
    } else if !prog_name.is_null() {
        snprintf(
            prog_str.as_mut_ptr().offset(func_sig_len as isize),
            size_of::<[c_char; 1024]>() - func_sig_len as usize,
            c" %s/prog_id:%u".as_ptr(),
            prog_name,
            prog_id,
        );
    } else {
        snprintf(
            prog_str.as_mut_ptr().offset(func_sig_len as isize),
            size_of::<[c_char; 1024]>() - func_sig_len as usize,
            c" <unknown_prog_name>/prog_id:%u".as_ptr(),
            prog_id,
        );
    }

    prog_str[size_of::<[c_char; 1024]>() - 1] = 0;
    jsonw_string((*d).jw, prog_str.as_ptr());
    btf__free(prog_btf);
    if prog_fd >= 0 {
        close(prog_fd);
    }
    0
}

unsafe fn btf_dumper_ptr(d: *const btf_dumper, t: *const btf_type, data: *const c_void) {
    let value = *(data as *const c_ulong);
    let ptr_type: *const btf_type;
    let ptr_type_id: __s32;

    if !(*d).prog_id_as_func_ptr || value > UINT32_MAX {
        if (*d).is_plain_text {
            jsonw_printf((*d).jw, c"\"%p\"".as_ptr(), value as *mut c_void);
        } else {
            jsonw_printf((*d).jw, c"%lu".as_ptr(), value);
        }
        return;
    }

    ptr_type_id = btf__resolve_type((*d).btf, (*t).type_);
    if ptr_type_id >= 0 {
        ptr_type = btf__type_by_id((*d).btf, ptr_type_id as __u32);
        if !ptr_type.is_null() && btf_is_func_proto(ptr_type) {
            if dump_prog_id_as_func_ptr(d, ptr_type, value as __u32) == 0 {
                return;
            }
        }
    }

    if (*d).is_plain_text {
        jsonw_printf((*d).jw, c"\"%p\"".as_ptr(), value as *mut c_void);
    } else {
        jsonw_printf((*d).jw, c"%lu".as_ptr(), value);
    }
}

unsafe fn btf_dumper_modifier(d: *const btf_dumper, type_id: __u32, bit_offset: __u8, data: *const c_void) -> c_int {
    let actual_type_id = btf__resolve_type((*d).btf, type_id);
    if actual_type_id < 0 {
        return actual_type_id;
    }

    btf_dumper_do_type(d, actual_type_id as __u32, bit_offset, data)
}

unsafe fn btf_dumper_enum(d: *const btf_dumper, t: *const btf_type, data: *const c_void) -> c_int {
    let enums = btf_enum(t);
    let value: __s64;

    match (*t).size {
        8 => value = *(data as *const __s64),
        4 => value = *(data as *const __s32) as __s64,
        2 => value = *(data as *const __s16) as __s64,
        1 => value = *(data as *const __s8) as __s64,
        _ => return -EINVAL,
    }

    let mut i: __u32 = 0;
    while i < btf_vlen(t) as __u32 {
        if value == (*enums.add(i as usize)).val as __s64 {
            jsonw_string((*d).jw, btf__name_by_offset((*d).btf, (*enums.add(i as usize)).name_off));
            return 0;
        }
        i += 1;
    }

    jsonw_int((*d).jw, value);
    0
}

unsafe fn btf_dumper_enum64(d: *const btf_dumper, t: *const btf_type, data: *const c_void) -> c_int {
    let enums = btf_enum64(t);
    let value = *(data as *const __u64);
    let val_lo32 = value as __u32;
    let val_hi32 = (value >> 32) as __u32;

    let mut i: __u32 = 0;
    while i < btf_vlen(t) as __u32 {
        if val_lo32 == (*enums.add(i as usize)).val_lo32 && val_hi32 == (*enums.add(i as usize)).val_hi32 {
            jsonw_string((*d).jw, btf__name_by_offset((*d).btf, (*enums.add(i as usize)).name_off));
            return 0;
        }
        i += 1;
    }

    jsonw_int((*d).jw, value as __s64);
    0
}

unsafe fn is_str_array(btf: *const btf, arr: *const btf_array, mut s: *const c_char) -> bool {
    let mut elem_type: *const btf_type;
    let end_s: *const c_char;

    if (*arr).nelems == 0 {
        return false;
    }

    elem_type = btf__type_by_id(btf, (*arr).type_);
    /* Not skipping typedef.  typedef to char does not count as
     * a string now.
     */
    while !elem_type.is_null() && btf_is_mod(elem_type) {
        elem_type = btf__type_by_id(btf, (*elem_type).type_);
    }

    if elem_type.is_null() || !btf_is_int(elem_type) || (*elem_type).size != 1 {
        return false;
    }

    if BTF_INT_ENCODING(*(elem_type.add(1) as *const __u32)) != BTF_INT_CHAR
        && strcmp(c"char".as_ptr(), btf__name_by_offset(btf, (*elem_type).name_off)) != 0
    {
        return false;
    }

    end_s = s.add((*arr).nelems as usize);
    while s < end_s {
        if *s == 0 {
            return true;
        }
        if *s <= 0x1f || *s >= 0x7f {
            return false;
        }
        s = s.add(1);
    }

    /* '\0' is not found */
    false
}

unsafe fn btf_dumper_array(d: *const btf_dumper, type_id: __u32, data: *const c_void) -> c_int {
    let t = btf__type_by_id((*d).btf, type_id);
    let arr = t.add(1) as *const btf_array;
    let elem_size: c_longlong;
    let mut ret: c_int = 0;

    if is_str_array((*d).btf, arr, data as *const c_char) {
        jsonw_string((*d).jw, data as *const c_char);
        return 0;
    }

    elem_size = btf__resolve_size((*d).btf, (*arr).type_);
    if elem_size < 0 {
        return elem_size as c_int;
    }

    jsonw_start_array((*d).jw);
    let mut i: __u32 = 0;
    while i < (*arr).nelems {
        ret = btf_dumper_do_type(d, (*arr).type_, 0, (data as *const u8).offset((i as isize) * elem_size as isize) as *const c_void);
        if ret != 0 {
            break;
        }
        i += 1;
    }

    jsonw_end_array((*d).jw);
    ret
}

unsafe fn btf_int128_print(jw: *mut json_writer_t, data: *const c_void, is_plain_text: bool) {
    /* data points to a __int128 number.
     * Suppose
     *     int128_num = *(__int128 *)data;
     * The below formulas shows what upper_num and lower_num represents:
     *     upper_num = int128_num >> 64;
     *     lower_num = int128_num & 0xffffffffFFFFFFFFULL;
     */
    let upper_num: __u64;
    let lower_num: __u64;

    /* Mirrors the original __BIG_ENDIAN_BITFIELD / little-endian conditional. */
    if cfg!(target_endian = "big") {
        upper_num = *(data as *const __u64);
        lower_num = *((data as *const u8).add(8) as *const __u64);
    } else {
        upper_num = *((data as *const u8).add(8) as *const __u64);
        lower_num = *(data as *const __u64);
    }

    if is_plain_text {
        if upper_num == 0 {
            jsonw_printf(jw, c"0x%llx".as_ptr(), lower_num);
        } else {
            jsonw_printf(jw, c"0x%llx%016llx".as_ptr(), upper_num, lower_num);
        }
    } else if upper_num == 0 {
        jsonw_printf(jw, c"\"0x%llx\"".as_ptr(), lower_num);
    } else {
        jsonw_printf(jw, c"\"0x%llx%016llx\"".as_ptr(), upper_num, lower_num);
    }
}

unsafe fn btf_int128_shift(print_num: *mut __u64, left_shift_bits: __u16, right_shift_bits: __u16) {
    let mut upper_num: __u64;
    let mut lower_num: __u64;

    /* Mirrors the original __BIG_ENDIAN_BITFIELD / little-endian conditional. */
    if cfg!(target_endian = "big") {
        upper_num = *print_num.add(0);
        lower_num = *print_num.add(1);
    } else {
        upper_num = *print_num.add(1);
        lower_num = *print_num.add(0);
    }

    /* shake out un-needed bits by shift/or operations */
    if left_shift_bits >= 64 {
        upper_num = lower_num << (left_shift_bits - 64);
        lower_num = 0;
    } else {
        upper_num = (upper_num << left_shift_bits) | (lower_num >> (64 - left_shift_bits));
        lower_num <<= left_shift_bits;
    }

    if right_shift_bits >= 64 {
        lower_num = upper_num >> (right_shift_bits - 64);
        upper_num = 0;
    } else {
        lower_num = (lower_num >> right_shift_bits) | (upper_num << (64 - right_shift_bits));
        upper_num >>= right_shift_bits;
    }

    if cfg!(target_endian = "big") {
        *print_num.add(0) = upper_num;
        *print_num.add(1) = lower_num;
    } else {
        *print_num.add(0) = lower_num;
        *print_num.add(1) = upper_num;
    }
}

unsafe fn btf_dumper_bitfield(nr_bits: __u32, bit_offset: __u8, data: *const c_void, jw: *mut json_writer_t, is_plain_text: bool) {
    let left_shift_bits: c_int;
    let right_shift_bits: c_int;
    let mut print_num = [0_u64; 2];
    let bytes_to_copy: c_int;
    let bits_to_copy: c_int;

    bits_to_copy = bit_offset as c_int + nr_bits as c_int;
    bytes_to_copy = BITS_ROUNDUP_BYTES(bits_to_copy);

    memcpy(print_num.as_mut_ptr() as *mut c_void, data, bytes_to_copy as size_t);
    if cfg!(target_endian = "big") {
        left_shift_bits = bit_offset as c_int;
    } else {
        left_shift_bits = 128 - bits_to_copy;
    }
    right_shift_bits = 128 - nr_bits as c_int;

    btf_int128_shift(print_num.as_mut_ptr(), left_shift_bits as __u16, right_shift_bits as __u16);
    btf_int128_print(jw, print_num.as_ptr() as *const c_void, is_plain_text);
}

unsafe fn btf_dumper_int_bits(int_type: __u32, bit_offset: __u8, mut data: *const c_void, jw: *mut json_writer_t, is_plain_text: bool) {
    let nr_bits = BTF_INT_BITS(int_type);
    let total_bits_offset: c_int;

    /* bits_offset is at most 7.
     * BTF_INT_OFFSET() cannot exceed 128 bits.
     */
    total_bits_offset = bit_offset as c_int + BTF_INT_OFFSET(int_type) as c_int;
    data = (data as *const u8).offset(BITS_ROUNDDOWN_BYTES(total_bits_offset) as isize) as *const c_void;
    let bit_offset = BITS_PER_BYTE_MASKED(total_bits_offset) as __u8;
    btf_dumper_bitfield(nr_bits, bit_offset, data, jw, is_plain_text);
}

unsafe fn btf_dumper_int(t: *const btf_type, bit_offset: __u8, data: *const c_void, jw: *mut json_writer_t, is_plain_text: bool) -> c_int {
    let int_type = t.add(1) as *const __u32;
    let nr_bits = BTF_INT_BITS(*int_type);
    /* if this is bit field */
    if bit_offset != 0 || BTF_INT_OFFSET(*int_type) != 0 || BITS_PER_BYTE_MASKED(nr_bits as c_int) != 0 {
        btf_dumper_int_bits(*int_type, bit_offset, data, jw, is_plain_text);
        return 0;
    }

    if nr_bits == 128 {
        btf_int128_print(jw, data, is_plain_text);
        return 0;
    }

    match BTF_INT_ENCODING(*int_type) {
        0 => {
            if BTF_INT_BITS(*int_type) == 64 {
                jsonw_printf(jw, c"%llu".as_ptr(), *(data as *const __u64));
            } else if BTF_INT_BITS(*int_type) == 32 {
                jsonw_printf(jw, c"%u".as_ptr(), *(data as *const __u32));
            } else if BTF_INT_BITS(*int_type) == 16 {
                jsonw_printf(jw, c"%hu".as_ptr(), *(data as *const __u16) as c_int);
            } else if BTF_INT_BITS(*int_type) == 8 {
                jsonw_printf(jw, c"%hhu".as_ptr(), *(data as *const __u8) as c_int);
            } else {
                btf_dumper_int_bits(*int_type, bit_offset, data, jw, is_plain_text);
            }
        }
        BTF_INT_SIGNED => {
            if BTF_INT_BITS(*int_type) == 64 {
                jsonw_printf(jw, c"%lld".as_ptr(), *(data as *const c_longlong));
            } else if BTF_INT_BITS(*int_type) == 32 {
                jsonw_printf(jw, c"%d".as_ptr(), *(data as *const c_int));
            } else if BTF_INT_BITS(*int_type) == 16 {
                jsonw_printf(jw, c"%hd".as_ptr(), *(data as *const i16) as c_int);
            } else if BTF_INT_BITS(*int_type) == 8 {
                jsonw_printf(jw, c"%hhd".as_ptr(), *(data as *const c_char) as c_int);
            } else {
                btf_dumper_int_bits(*int_type, bit_offset, data, jw, is_plain_text);
            }
        }
        BTF_INT_CHAR => {
            if isprint(*(data as *const c_char) as c_int) != 0 {
                jsonw_printf(jw, c"\"%c\"".as_ptr(), *(data as *const c_char) as c_int);
            } else if is_plain_text {
                jsonw_printf(jw, c"0x%hhx".as_ptr(), *(data as *const c_char) as c_int);
            } else {
                jsonw_printf(jw, c"\"\\u00%02hhx\"".as_ptr(), *(data as *const c_char) as c_int);
            }
        }
        BTF_INT_BOOL => {
            jsonw_bool(jw, *(data as *const bool));
        }
        _ => {
            /* shouldn't happen */
            return -EINVAL;
        }
    }

    0
}

unsafe fn btf_dumper_struct(d: *const btf_dumper, type_id: __u32, data: *const c_void) -> c_int {
    let t: *const btf_type;
    let m: *const btf_member;
    let data_off: *const c_void;
    let kind_flag: c_int;
    let mut ret: c_int = 0;
    let vlen: c_int;

    t = btf__type_by_id((*d).btf, type_id);
    if t.is_null() {
        return -EINVAL;
    }

    kind_flag = btf_kflag(t);
    vlen = btf_vlen(t);
    jsonw_start_object((*d).jw);
    m = t.add(1) as *const btf_member;

    let mut i: c_int = 0;
    while i < vlen {
        let mut bit_offset = (*m.add(i as usize)).offset;
        let mut bitfield_size: __u32 = 0;

        if kind_flag != 0 {
            bitfield_size = BTF_MEMBER_BITFIELD_SIZE(bit_offset);
            bit_offset = BTF_MEMBER_BIT_OFFSET(bit_offset);
        }

        jsonw_name((*d).jw, btf__name_by_offset((*d).btf, (*m.add(i as usize)).name_off));
        data_off = (data as *const u8).offset(BITS_ROUNDDOWN_BYTES(bit_offset as c_int) as isize) as *const c_void;
        if bitfield_size != 0 {
            btf_dumper_bitfield(bitfield_size, BITS_PER_BYTE_MASKED(bit_offset as c_int) as __u8, data_off, (*d).jw, (*d).is_plain_text);
        } else {
            ret = btf_dumper_do_type(d, (*m.add(i as usize)).type_, BITS_PER_BYTE_MASKED(bit_offset as c_int) as __u8, data_off);
            if ret != 0 {
                break;
            }
        }
        i += 1;
    }

    jsonw_end_object((*d).jw);

    ret
}

unsafe fn btf_dumper_var(d: *const btf_dumper, type_id: __u32, bit_offset: __u8, data: *const c_void) -> c_int {
    let t = btf__type_by_id((*d).btf, type_id);
    let ret: c_int;

    jsonw_start_object((*d).jw);
    jsonw_name((*d).jw, btf__name_by_offset((*d).btf, (*t).name_off));
    ret = btf_dumper_do_type(d, (*t).type_, bit_offset, data);
    jsonw_end_object((*d).jw);

    ret
}

unsafe fn btf_dumper_datasec(d: *const btf_dumper, type_id: __u32, data: *const c_void) -> c_int {
    let vsi: *const btf_var_secinfo;
    let t: *const btf_type;
    let mut ret: c_int = 0;
    let vlen: c_int;

    t = btf__type_by_id((*d).btf, type_id);
    if t.is_null() {
        return -EINVAL;
    }

    vlen = btf_vlen(t);
    vsi = t.add(1) as *const btf_var_secinfo;

    jsonw_start_object((*d).jw);
    jsonw_name((*d).jw, btf__name_by_offset((*d).btf, (*t).name_off));
    jsonw_start_array((*d).jw);
    let mut i: c_int = 0;
    while i < vlen {
        ret = btf_dumper_do_type(d, (*vsi.add(i as usize)).type_, 0, (data as *const u8).add((*vsi.add(i as usize)).offset as usize) as *const c_void);
        if ret != 0 {
            break;
        }
        i += 1;
    }
    jsonw_end_array((*d).jw);
    jsonw_end_object((*d).jw);

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn btf_dumper_type(d: *const btf_dumper, type_id: __u32, data: *const c_void) -> c_int {
    btf_dumper_do_type(d, type_id, 0, data)
}

unsafe fn __btf_dumper_type_only(btf: *const btf, type_id: __u32, func_sig: *mut c_char, mut pos: c_int, size: c_int) -> c_int {
    let proto_type: *const btf_type;
    let array: *const btf_array;
    let var: *const btf_var;
    let t: *const btf_type;

    if type_id == 0 {
        return BTF_PRINT_ARG!(func_sig, pos, size, c"void ".as_ptr());
    }

    t = btf__type_by_id(btf, type_id);

    match btf_kind(t) {
        BTF_KIND_INT | BTF_KIND_TYPEDEF | BTF_KIND_FLOAT => {
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c"%s ".as_ptr(), btf__name_by_offset(btf, (*t).name_off));
        }
        BTF_KIND_STRUCT => {
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c"struct %s ".as_ptr(), btf__name_by_offset(btf, (*t).name_off));
        }
        BTF_KIND_UNION => {
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c"union %s ".as_ptr(), btf__name_by_offset(btf, (*t).name_off));
        }
        BTF_KIND_ENUM | BTF_KIND_ENUM64 => {
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c"enum %s ".as_ptr(), btf__name_by_offset(btf, (*t).name_off));
        }
        BTF_KIND_ARRAY => {
            array = t.add(1) as *const btf_array;
            pos = BTF_PRINT_TYPE(btf, (*array).type_, func_sig, pos, size);
            if pos == -1 {
                return -1;
            }
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c"[%u]".as_ptr(), (*array).nelems);
        }
        BTF_KIND_PTR => {
            pos = BTF_PRINT_TYPE(btf, (*t).type_, func_sig, pos, size);
            if pos == -1 {
                return -1;
            }
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c"* ".as_ptr());
        }
        BTF_KIND_FWD => {
            pos = BTF_PRINT_ARG!(
                func_sig,
                pos,
                size,
                c"%s %s ".as_ptr(),
                if btf_kflag(t) != 0 { c"union".as_ptr() } else { c"struct".as_ptr() },
                btf__name_by_offset(btf, (*t).name_off),
            );
        }
        BTF_KIND_VOLATILE => {
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c"volatile ".as_ptr());
            if pos == -1 {
                return -1;
            }
            pos = BTF_PRINT_TYPE(btf, (*t).type_, func_sig, pos, size);
        }
        BTF_KIND_CONST => {
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c"const ".as_ptr());
            if pos == -1 {
                return -1;
            }
            pos = BTF_PRINT_TYPE(btf, (*t).type_, func_sig, pos, size);
        }
        BTF_KIND_RESTRICT => {
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c"restrict ".as_ptr());
            if pos == -1 {
                return -1;
            }
            pos = BTF_PRINT_TYPE(btf, (*t).type_, func_sig, pos, size);
        }
        BTF_KIND_FUNC_PROTO => {
            pos = btf_dump_func(btf, func_sig, t, ptr::null(), pos, size);
        }
        BTF_KIND_FUNC => {
            proto_type = btf__type_by_id(btf, (*t).type_);
            pos = btf_dump_func(btf, func_sig, proto_type, t, pos, size);
        }
        BTF_KIND_VAR => {
            var = t.add(1) as *const btf_var;
            if (*var).linkage == BTF_VAR_STATIC {
                pos = BTF_PRINT_ARG!(func_sig, pos, size, c"static ".as_ptr());
                if pos == -1 {
                    return -1;
                }
            }
            pos = BTF_PRINT_TYPE(btf, (*t).type_, func_sig, pos, size);
            if pos == -1 {
                return -1;
            }
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c" %s".as_ptr(), btf__name_by_offset(btf, (*t).name_off));
        }
        BTF_KIND_DATASEC => {
            pos = BTF_PRINT_ARG!(func_sig, pos, size, c"section (\"%s\") ".as_ptr(), btf__name_by_offset(btf, (*t).name_off));
        }
        BTF_KIND_UNKN | _ => return -1,
    }

    if pos == -1 {
        return -1;
    }
    pos
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn btf_dumper_type_only(btf: *const btf, type_id: __u32, func_sig: *mut c_char, size: c_int) {
    let err: c_int;

    *func_sig = 0;
    if btf.is_null() {
        return;
    }

    err = __btf_dumper_type_only(btf, type_id, func_sig, 0, size);
    if err < 0 {
        *func_sig = 0;
    }
}

unsafe fn ltrim(mut s: *const c_char) -> *const c_char {
    while isspace(*s as c_int) != 0 {
        s = s.add(1);
    }

    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn btf_dump_linfo_plain(btf: *const btf, linfo: *const bpf_line_info, mut prefix: *const c_char, linum: bool) {
    let mut line = btf__name_by_offset(btf, (*linfo).line_off);

    if line.is_null() {
        return;
    }
    line = ltrim(line);

    if prefix.is_null() {
        prefix = c"".as_ptr();
    }

    if linum {
        let mut file = btf__name_by_offset(btf, (*linfo).file_name_off);

        /* More forgiving on file because linum option is
         * expected to provide more info than the already
         * available src line.
         */
        if file.is_null() {
            file = c"".as_ptr();
        }

        printf(
            c"%s%s [file:%s line_num:%u line_col:%u]\n".as_ptr(),
            prefix,
            line,
            file,
            BPF_LINE_INFO_LINE_NUM((*linfo).line_col),
            BPF_LINE_INFO_LINE_COL((*linfo).line_col),
        );
    } else {
        printf(c"%s%s\n".as_ptr(), prefix, line);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn btf_dump_linfo_json(btf: *const btf, linfo: *const bpf_line_info, linum: bool) {
    let line = btf__name_by_offset(btf, (*linfo).line_off);

    if !line.is_null() {
        jsonw_string_field(json_wtr, c"src".as_ptr(), ltrim(line));
    }

    if linum {
        let file = btf__name_by_offset(btf, (*linfo).file_name_off);

        if !file.is_null() {
            jsonw_string_field(json_wtr, c"file".as_ptr(), file);
        }

        if BPF_LINE_INFO_LINE_NUM((*linfo).line_col) != 0 {
            jsonw_int_field(json_wtr, c"line_num".as_ptr(), BPF_LINE_INFO_LINE_NUM((*linfo).line_col) as __s64);
        }

        if BPF_LINE_INFO_LINE_COL((*linfo).line_col) != 0 {
            jsonw_int_field(json_wtr, c"line_col".as_ptr(), BPF_LINE_INFO_LINE_COL((*linfo).line_col) as __s64);
        }
    }
}

unsafe fn dotlabel_puts(mut s: *const c_char) {
    while *s != 0 {
        match *s as u8 as char {
            '\\' | '"' | '{' | '}' | '<' | '>' | '|' | ' ' => {
                putchar('\\' as c_int);
                putchar(*s as c_int);
            }
            _ => {
                putchar(*s as c_int);
            }
        }
        s = s.add(1);
    }
}

unsafe fn shorten_path(path: *const c_char) -> *const c_char {
    const MAX_PATH_LEN: c_uint = 32;
    let len = strlen(path);
    let mut shortpath: *const c_char;

    if len <= MAX_PATH_LEN as usize {
        return path;
    }

    /* Search for last '/' under the MAX_PATH_LEN limit */
    shortpath = strchr(path.add(len - MAX_PATH_LEN as usize), '/' as c_int) as *const c_char;
    if !shortpath.is_null() {
        if shortpath < path.add(strlen(c"...".as_ptr())) {
            /* We removed a very short prefix, e.g. "/w", and we'll
             * make the path longer by prefixing with the ellipsis.
             * Not worth it, keep initial path.
             */
            return path;
        }
        return shortpath;
    }

    /* File base name length is > MAX_PATH_LEN, search for last '/' */
    shortpath = strrchr(path, '/' as c_int) as *const c_char;
    if !shortpath.is_null() {
        return shortpath;
    }

    path
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn btf_dump_linfo_dotlabel(btf: *const btf, linfo: *const bpf_line_info, linum: bool) {
    let mut line = btf__name_by_offset(btf, (*linfo).line_off);

    if line.is_null() || strlen(line) == 0 {
        return;
    }
    line = ltrim(line);

    if linum {
        let file = btf__name_by_offset(btf, (*linfo).file_name_off);
        let shortfile: *const c_char;

        /* More forgiving on file because linum option is
         * expected to provide more info than the already
         * available src line.
         */
        if file.is_null() {
            shortfile = c"".as_ptr();
        } else {
            shortfile = shorten_path(file);
        }

        printf(c"; [%s".as_ptr(), if shortfile > file { c"...".as_ptr() } else { c"".as_ptr() });
        dotlabel_puts(shortfile);
        printf(
            c" line:%u col:%u]\\l\\\n".as_ptr(),
            BPF_LINE_INFO_LINE_NUM((*linfo).line_col),
            BPF_LINE_INFO_LINE_COL((*linfo).line_col),
        );
    }

    printf(c"; ".as_ptr());
    dotlabel_puts(line);
    printf(c"\\l\\\n".as_ptr());
}
