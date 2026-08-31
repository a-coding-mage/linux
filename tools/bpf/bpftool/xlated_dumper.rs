// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2018 Netronome Systems, Inc. */

/* Translated from xlated_dumper.c. C include dependencies are represented as
 * external declarations and opaque/partial C-layout types below.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type size_t = usize;
type __u8 = u8;
type __u32 = u32;
type __u64 = u64;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct va_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog_linfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct json_writer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kernel_sym {
    pub address: c_ulong,
    pub name: [c_char; 128],
    pub module: [c_char; 128],
}

#[repr(C)]
pub struct bpf_func_info {
    pub insn_off: __u32,
    pub type_id: __u32,
}

#[repr(C)]
pub struct bpf_line_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_insn {
    pub code: __u8,
    pub dst_reg: __u8,
    pub src_reg: __u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
pub struct bpf_insn_cbs {
    pub cb_print: Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>,
    pub cb_call: Option<unsafe extern "C" fn(*mut c_void, *const bpf_insn) -> *const c_char>,
    pub cb_imm: Option<unsafe extern "C" fn(*mut c_void, *const bpf_insn, __u64) -> *const c_char>,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct dump_data {
    pub sym_mapping: *mut kernel_sym,
    pub sym_count: size_t,
    pub address_call_base: c_ulong,
    pub nr_jited_ksyms: __u32,
    pub jited_ksyms: *mut c_ulong,
    pub scratch_buff: [c_char; 128],
    pub prog_linfo: *const bpf_prog_linfo,
    pub func_info: *mut bpf_func_info,
    pub btf: *mut btf,
    pub finfo_rec_size: size_t,
}

pub const BPF_LD: c_int = 0x00;
pub const BPF_IMM: c_int = 0x00;
pub const BPF_DW: c_int = 0x18;
pub const BPF_PSEUDO_MAP_FD: __u8 = 1;
pub const BPF_PSEUDO_MAP_VALUE: __u8 = 2;
pub const BPF_PSEUDO_CALL: __u8 = 1;
pub const BPF_PSEUDO_MAP_IDX_VALUE: __u8 = 6;
pub const BPF_PSEUDO_FUNC: __u8 = 4;

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut json_wtr: *mut json_writer;

    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn qsort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    );
    fn bsearch(
        key: *const c_void,
        base: *const c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn vprintf(format: *const c_char, arg: va_list) -> c_int;
    fn vsnprintf(s: *mut c_char, n: size_t, format: *const c_char, arg: va_list) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;

    fn libbpf_reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn jsonw_vprintf_enquote(wtr: *mut json_writer, fmt: *const c_char, args: va_list);
    fn jsonw_start_array(wtr: *mut json_writer);
    fn jsonw_end_array(wtr: *mut json_writer);
    fn jsonw_start_object(wtr: *mut json_writer);
    fn jsonw_end_object(wtr: *mut json_writer);
    fn jsonw_name(wtr: *mut json_writer, name: *const c_char);
    fn jsonw_string(wtr: *mut json_writer, value: *const c_char);
    fn jsonw_printf(wtr: *mut json_writer, fmt: *const c_char, ...);
    fn print_hex_data_json(data: *mut u8, len: c_uint);
    fn print_bpf_insn(cbs: *const bpf_insn_cbs, insn: *const bpf_insn, allow_ptr_leaks: bool);
    fn btf_dumper_type_only(btf: *mut btf, type_id: __u32, func_sig: *mut c_char, len: size_t);
    fn bpf_prog_linfo__lfind(
        prog_linfo: *const bpf_prog_linfo,
        insn_off: c_uint,
        nr_skip: c_uint,
    ) -> *const bpf_line_info;
    fn btf_dump_linfo_json(btf: *mut btf, linfo: *const bpf_line_info, linum: bool);
    fn btf_dump_linfo_plain(
        btf: *mut btf,
        linfo: *const bpf_line_info,
        prefix: *const c_char,
        linum: bool,
    );
    fn btf_dump_linfo_dotlabel(btf: *mut btf, linfo: *const bpf_line_info, linum: bool);
    fn fprint_hex(stream: *mut FILE, data: *const c_void, len: size_t, sep: *const c_char);
}

unsafe extern "C" fn kernel_syms_cmp(sym_a: *const c_void, sym_b: *const c_void) -> c_int {
    ((*((sym_a as *const kernel_sym))).address).wrapping_sub((*((sym_b as *const kernel_sym))).address)
        as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_syms_load(dd: *mut dump_data) {
    let mut sym: *mut kernel_sym;
    let mut buff: [c_char; 256] = [0; 256];
    let mut tmp: *mut c_void;
    let mut address: *mut c_void = core::ptr::null_mut();
    let fp: *mut FILE;

    fp = fopen(c"/proc/kallsyms".as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        return;
    }

    while !fgets(buff.as_mut_ptr(), buff.len() as c_int, fp).is_null() {
        tmp = libbpf_reallocarray(
            (*dd).sym_mapping as *mut c_void,
            (*dd).sym_count + 1,
            core::mem::size_of::<kernel_sym>(),
        );
        if tmp.is_null() {
            free((*dd).sym_mapping as *mut c_void);
            (*dd).sym_mapping = core::ptr::null_mut();
            fclose(fp);
            return;
        }
        (*dd).sym_mapping = tmp as *mut kernel_sym;
        sym = (*dd).sym_mapping.add((*dd).sym_count);

        /* module is optional */
        (*sym).module[0] = b'\0' as c_char;
        /* trim the square brackets around the module name */
        if sscanf(
            buff.as_ptr(),
            c"%p %*c %s [%[^]]s".as_ptr(),
            &mut address as *mut *mut c_void,
            (*sym).name.as_mut_ptr(),
            (*sym).module.as_mut_ptr(),
        ) < 2
        {
            continue;
        }
        (*sym).address = address as c_ulong;
        if strcmp((*sym).name.as_ptr(), c"__bpf_call_base".as_ptr()) == 0 {
            (*dd).address_call_base = (*sym).address;
            /* sysctl kernel.kptr_restrict was set */
            if (*sym).address == 0 {
                free((*dd).sym_mapping as *mut c_void);
                (*dd).sym_mapping = core::ptr::null_mut();
                fclose(fp);
                return;
            }
        }
        if (*sym).address != 0 {
            (*dd).sym_count += 1;
        }
    }

    fclose(fp);

    qsort(
        (*dd).sym_mapping as *mut c_void,
        (*dd).sym_count,
        core::mem::size_of::<kernel_sym>(),
        Some(kernel_syms_cmp),
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_syms_destroy(dd: *mut dump_data) {
    free((*dd).sym_mapping as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_syms_search(dd: *mut dump_data, key: c_ulong) -> *mut kernel_sym {
    let sym = kernel_sym {
        address: key,
        name: [0; 128],
        module: [0; 128],
    };

    if !(*dd).sym_mapping.is_null() {
        bsearch(
            &sym as *const kernel_sym as *const c_void,
            (*dd).sym_mapping as *const c_void,
            (*dd).sym_count,
            core::mem::size_of::<kernel_sym>(),
            Some(kernel_syms_cmp),
        ) as *mut kernel_sym
    } else {
        core::ptr::null_mut()
    }
}

unsafe extern "C" fn print_insn(_private_data: *mut c_void, fmt: *const c_char, args: ...) {
    vprintf(fmt, args);
}

unsafe extern "C" fn print_insn_for_graph(
    _private_data: *mut c_void,
    fmt: *const c_char,
    args: ...
) {
    let mut buf: [c_char; 64] = [0; 64];
    let mut p: *mut c_char;

    vsnprintf(buf.as_mut_ptr(), buf.len(), fmt, args);

    p = buf.as_mut_ptr();
    while *p != b'\0' as c_char {
        if *p == b'<' as c_char
            || *p == b'>' as c_char
            || *p == b'|' as c_char
            || *p == b'&' as c_char
        {
            memmove(
                p.add(1) as *mut c_void,
                p as *const c_void,
                strlen(buf.as_ptr()) + 1 - p.offset_from(buf.as_mut_ptr()) as size_t,
            );
            /* Escape special character. */
            *p = b'\\' as c_char;
            p = p.add(1);
        }

        p = p.add(1);
    }

    printf(c"%s".as_ptr(), buf.as_ptr());
}

unsafe extern "C" fn print_insn_json(_private_data: *mut c_void, fmt: *const c_char, args: ...) {
    jsonw_vprintf_enquote(json_wtr, fmt, args);
}

unsafe extern "C" fn print_call_pcrel(
    dd: *mut dump_data,
    sym: *mut kernel_sym,
    address: c_ulong,
    insn: *const bpf_insn,
) -> *const c_char {
    if (*dd).nr_jited_ksyms == 0 {
        /* Do not show address for interpreted programs */
        snprintf(
            (*dd).scratch_buff.as_mut_ptr(),
            (*dd).scratch_buff.len(),
            c"%+d".as_ptr(),
            (*insn).off as c_int,
        );
    } else if !sym.is_null() {
        snprintf(
            (*dd).scratch_buff.as_mut_ptr(),
            (*dd).scratch_buff.len(),
            c"%+d#%s".as_ptr(),
            (*insn).off as c_int,
            (*sym).name.as_ptr(),
        );
    } else {
        snprintf(
            (*dd).scratch_buff.as_mut_ptr(),
            (*dd).scratch_buff.len(),
            c"%+d#0x%lx".as_ptr(),
            (*insn).off as c_int,
            address,
        );
    }
    (*dd).scratch_buff.as_ptr()
}

unsafe extern "C" fn print_call_helper(
    dd: *mut dump_data,
    sym: *mut kernel_sym,
    address: c_ulong,
) -> *const c_char {
    if !sym.is_null() {
        snprintf(
            (*dd).scratch_buff.as_mut_ptr(),
            (*dd).scratch_buff.len(),
            c"%s".as_ptr(),
            (*sym).name.as_ptr(),
        );
    } else {
        snprintf(
            (*dd).scratch_buff.as_mut_ptr(),
            (*dd).scratch_buff.len(),
            c"0x%lx".as_ptr(),
            address,
        );
    }
    (*dd).scratch_buff.as_ptr()
}

unsafe extern "C" fn print_call(
    private_data: *mut c_void,
    insn: *const bpf_insn,
) -> *const c_char {
    let dd = private_data as *mut dump_data;
    let mut address: c_ulong = (*dd).address_call_base.wrapping_add((*insn).imm as c_ulong);
    let sym: *mut kernel_sym;

    if (*insn).src_reg == BPF_PSEUDO_CALL
        && ((*insn).imm as __u32) < (*dd).nr_jited_ksyms
        && !(*dd).jited_ksyms.is_null()
    {
        address = *(*dd).jited_ksyms.add((*insn).imm as usize);
    }

    sym = kernel_syms_search(dd, address);
    if (*insn).src_reg == BPF_PSEUDO_CALL {
        print_call_pcrel(dd, sym, address, insn)
    } else {
        print_call_helper(dd, sym, address)
    }
}

unsafe extern "C" fn print_imm(
    private_data: *mut c_void,
    insn: *const bpf_insn,
    full_imm: __u64,
) -> *const c_char {
    let dd = private_data as *mut dump_data;

    if (*insn).src_reg == BPF_PSEUDO_MAP_FD {
        snprintf(
            (*dd).scratch_buff.as_mut_ptr(),
            (*dd).scratch_buff.len(),
            c"map[id:%d]".as_ptr(),
            (*insn).imm,
        );
    } else if (*insn).src_reg == BPF_PSEUDO_MAP_VALUE {
        snprintf(
            (*dd).scratch_buff.as_mut_ptr(),
            (*dd).scratch_buff.len(),
            c"map[id:%d][0]+%d".as_ptr(),
            (*insn).imm,
            (*insn.add(1)).imm,
        );
    } else if (*insn).src_reg == BPF_PSEUDO_MAP_IDX_VALUE {
        snprintf(
            (*dd).scratch_buff.as_mut_ptr(),
            (*dd).scratch_buff.len(),
            c"map[idx:%d]+%d".as_ptr(),
            (*insn).imm,
            (*insn.add(1)).imm,
        );
    } else if (*insn).src_reg == BPF_PSEUDO_FUNC {
        snprintf(
            (*dd).scratch_buff.as_mut_ptr(),
            (*dd).scratch_buff.len(),
            c"subprog[%+d]".as_ptr(),
            (*insn).imm,
        );
    } else {
        snprintf(
            (*dd).scratch_buff.as_mut_ptr(),
            (*dd).scratch_buff.len(),
            c"0x%llx".as_ptr(),
            full_imm as u64,
        );
    }
    (*dd).scratch_buff.as_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_xlated_json(
    dd: *mut dump_data,
    buf: *mut c_void,
    len: c_uint,
    opcodes: bool,
    linum: bool,
) {
    let prog_linfo: *const bpf_prog_linfo = (*dd).prog_linfo;
    let cbs = bpf_insn_cbs {
        cb_print: Some(print_insn_json),
        cb_call: Some(print_call),
        cb_imm: Some(print_imm),
        private_data: dd as *mut c_void,
    };
    let mut record: *mut bpf_func_info;
    let insn = buf as *mut bpf_insn;
    let btf = (*dd).btf;
    let mut double_insn = false;
    let mut nr_skip: c_uint = 0;
    let mut func_sig: [c_char; 1024] = [0; 1024];
    let mut i: c_uint;

    jsonw_start_array(json_wtr);
    record = (*dd).func_info;
    i = 0;
    while (i as usize) < (len as usize) / core::mem::size_of::<bpf_insn>() {
        if double_insn {
            double_insn = false;
            i += 1;
            continue;
        }
        double_insn = (*insn.add(i as usize)).code as c_int == (BPF_LD | BPF_IMM | BPF_DW);

        jsonw_start_object(json_wtr);

        if !btf.is_null() && !record.is_null() {
            if (*record).insn_off == i {
                btf_dumper_type_only(btf, (*record).type_id, func_sig.as_mut_ptr(), func_sig.len());
                if func_sig[0] != b'\0' as c_char {
                    jsonw_name(json_wtr, c"proto".as_ptr());
                    jsonw_string(json_wtr, func_sig.as_ptr());
                }
                record = (record as *mut u8).add((*dd).finfo_rec_size) as *mut bpf_func_info;
            }
        }

        if !prog_linfo.is_null() {
            let linfo: *const bpf_line_info;

            linfo = bpf_prog_linfo__lfind(prog_linfo, i, nr_skip);
            if !linfo.is_null() {
                btf_dump_linfo_json(btf, linfo, linum);
                nr_skip += 1;
            }
        }

        jsonw_name(json_wtr, c"disasm".as_ptr());
        print_bpf_insn(&cbs, insn.add(i as usize), true);

        if opcodes {
            jsonw_name(json_wtr, c"opcodes".as_ptr());
            jsonw_start_object(json_wtr);

            jsonw_name(json_wtr, c"code".as_ptr());
            jsonw_printf(json_wtr, c"\"0x%02hhx\"".as_ptr(), (*insn.add(i as usize)).code as c_int);

            jsonw_name(json_wtr, c"src_reg".as_ptr());
            jsonw_printf(
                json_wtr,
                c"\"0x%hhx\"".as_ptr(),
                (*insn.add(i as usize)).src_reg as c_int,
            );

            jsonw_name(json_wtr, c"dst_reg".as_ptr());
            jsonw_printf(
                json_wtr,
                c"\"0x%hhx\"".as_ptr(),
                (*insn.add(i as usize)).dst_reg as c_int,
            );

            jsonw_name(json_wtr, c"off".as_ptr());
            print_hex_data_json(&mut (*insn.add(i as usize)).off as *mut i16 as *mut u8, 2);

            jsonw_name(json_wtr, c"imm".as_ptr());
            if double_insn && i < len - 1 {
                print_hex_data_json(&mut (*insn.add(i as usize)).imm as *mut i32 as *mut u8, 12);
            } else {
                print_hex_data_json(&mut (*insn.add(i as usize)).imm as *mut i32 as *mut u8, 4);
            }
            jsonw_end_object(json_wtr);
        }
        jsonw_end_object(json_wtr);
        i += 1;
    }
    jsonw_end_array(json_wtr);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_xlated_plain(
    dd: *mut dump_data,
    buf: *mut c_void,
    len: c_uint,
    opcodes: bool,
    linum: bool,
) {
    let prog_linfo: *const bpf_prog_linfo = (*dd).prog_linfo;
    let cbs = bpf_insn_cbs {
        cb_print: Some(print_insn),
        cb_call: Some(print_call),
        cb_imm: Some(print_imm),
        private_data: dd as *mut c_void,
    };
    let mut record: *mut bpf_func_info;
    let insn = buf as *mut bpf_insn;
    let btf = (*dd).btf;
    let mut nr_skip: c_uint = 0;
    let mut double_insn = false;
    let mut func_sig: [c_char; 1024] = [0; 1024];
    let mut i: c_uint;

    record = (*dd).func_info;
    i = 0;
    while (i as usize) < (len as usize) / core::mem::size_of::<bpf_insn>() {
        if double_insn {
            double_insn = false;
            i += 1;
            continue;
        }

        if !btf.is_null() && !record.is_null() {
            if (*record).insn_off == i {
                btf_dumper_type_only(btf, (*record).type_id, func_sig.as_mut_ptr(), func_sig.len());
                if func_sig[0] != b'\0' as c_char {
                    printf(c"%s:\n".as_ptr(), func_sig.as_ptr());
                }
                record = (record as *mut u8).add((*dd).finfo_rec_size) as *mut bpf_func_info;
            }
        }

        if !prog_linfo.is_null() {
            let linfo: *const bpf_line_info;

            linfo = bpf_prog_linfo__lfind(prog_linfo, i, nr_skip);
            if !linfo.is_null() {
                btf_dump_linfo_plain(btf, linfo, c"; ".as_ptr(), linum);
                nr_skip += 1;
            }
        }

        double_insn = (*insn.add(i as usize)).code as c_int == (BPF_LD | BPF_IMM | BPF_DW);

        printf(c"%4u: ".as_ptr(), i);
        print_bpf_insn(&cbs, insn.add(i as usize), true);
        printf(c"\n".as_ptr());

        if opcodes {
            printf(c"       ".as_ptr());
            fprint_hex(stdout, insn.add(i as usize) as *const c_void, 8, c" ".as_ptr());
            if double_insn && i < len - 1 {
                printf(c" ".as_ptr());
                fprint_hex(stdout, insn.add(i as usize + 1) as *const c_void, 8, c" ".as_ptr());
            }
            printf(c"\n".as_ptr());
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dump_xlated_for_graph(
    dd: *mut dump_data,
    buf_start: *mut c_void,
    buf_end: *mut c_void,
    start_idx: c_uint,
    opcodes: bool,
    linum: bool,
) {
    let cbs = bpf_insn_cbs {
        cb_print: Some(print_insn_for_graph),
        cb_call: Some(print_call),
        cb_imm: Some(print_imm),
        private_data: dd as *mut c_void,
    };
    let prog_linfo: *const bpf_prog_linfo = (*dd).prog_linfo;
    let mut last_linfo: *const bpf_line_info = core::ptr::null();
    let mut record: *mut bpf_func_info = (*dd).func_info;
    let insn_start = buf_start as *mut bpf_insn;
    let insn_end = buf_end as *mut bpf_insn;
    let mut cur = insn_start;
    let btf = (*dd).btf;
    let mut double_insn = false;
    let mut func_sig: [c_char; 1024] = [0; 1024];

    while cur <= insn_end {
        let insn_off: c_uint;

        if double_insn {
            double_insn = false;
            cur = cur.add(1);
            continue;
        }
        double_insn = (*cur).code as c_int == (BPF_LD | BPF_IMM | BPF_DW);

        insn_off = cur.offset_from(insn_start) as c_uint + start_idx;
        if !btf.is_null() && !record.is_null() {
            if (*record).insn_off == insn_off {
                btf_dumper_type_only(btf, (*record).type_id, func_sig.as_mut_ptr(), func_sig.len());
                if func_sig[0] != b'\0' as c_char {
                    printf(c"; %s:\\l\\\n".as_ptr(), func_sig.as_ptr());
                }
                record = (record as *mut u8).add((*dd).finfo_rec_size) as *mut bpf_func_info;
            }
        }

        if !prog_linfo.is_null() {
            let linfo: *const bpf_line_info;

            linfo = bpf_prog_linfo__lfind(prog_linfo, insn_off, 0);
            if !linfo.is_null() && linfo != last_linfo {
                btf_dump_linfo_dotlabel(btf, linfo, linum);
                last_linfo = linfo;
            }
        }

        printf(c"%u: ".as_ptr(), insn_off);
        print_bpf_insn(&cbs, cur, true);
        printf(c"\\l\\\n".as_ptr());

        if opcodes {
            printf(c"\\ \\ \\ \\ ".as_ptr());
            fprint_hex(stdout, cur as *const c_void, 8, c" ".as_ptr());
            if double_insn && cur <= insn_end.sub(1) {
                printf(c" ".as_ptr());
                fprint_hex(stdout, cur.add(1) as *const c_void, 8, c" ".as_ptr());
            }
            printf(c"\\l\\\n".as_ptr());
        }

        if cur != insn_end {
            printf(c"| ".as_ptr());
        }
        cur = cur.add(1);
    }
}
