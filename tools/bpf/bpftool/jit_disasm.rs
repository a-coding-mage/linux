// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/*
 * Based on:
 *
 * Minimal BPF JIT image disassembler
 *
 * Disassembles BPF JIT compiler emitted opcodes back to asm insn's for
 * debugging or verification purposes.
 *
 * Copyright 2013 Daniel Borkmann <daniel@iogearbox.net>
 * Licensed under the GNU General Public License, version 2.0 (GPLv2)
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type size_t = usize;
type ssize_t = isize;
type __u64 = u64;
type uint64_t = u64;
type uint8_t = u8;
type bfd_vma = u64;
type bool_ = bool;
type va_list = *mut c_void;

const PATH_MAX: usize = 4096;

static mut oper_count: c_int = 0;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog_linfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_line_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct json_writer_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut json_wtr: *mut json_writer_t;
    static mut json_output: bool_;

    fn p_err(fmt: *const c_char, ...);
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn malloc(size: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strtok(str_: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn vasprintf(strp: *mut *mut c_char, fmt: *const c_char, ap: va_list) -> c_int;
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t;

    static mut stdout: *mut c_void;

    fn jsonw_string_field(w: *mut json_writer_t, prop: *const c_char, value: *const c_char);
    fn jsonw_name(w: *mut json_writer_t, name: *const c_char);
    fn jsonw_start_array(w: *mut json_writer_t);
    fn jsonw_end_array(w: *mut json_writer_t);
    fn jsonw_start_object(w: *mut json_writer_t);
    fn jsonw_end_object(w: *mut json_writer_t);
    fn jsonw_string(w: *mut json_writer_t, value: *const c_char);
    fn jsonw_null(w: *mut json_writer_t);
    fn jsonw_printf(w: *mut json_writer_t, fmt: *const c_char, ...);

    fn bpf_prog_linfo__lfind_addr_func(
        prog_linfo: *const bpf_prog_linfo,
        addr: __u64,
        func_idx: c_uint,
        nr_skip: c_uint,
    ) -> *const bpf_line_info;
    fn btf_dump_linfo_json(btf: *const btf, linfo: *const bpf_line_info, linum: bool_);
    fn btf_dump_linfo_plain(
        btf: *const btf,
        linfo: *const bpf_line_info,
        prefix: *const c_char,
        linum: bool_,
    );
}

#[cfg(feature = "llvm_support")]
type LLVMDisasmContextRef = *mut c_void;

#[cfg(feature = "llvm_support")]
type disasm_ctx_t = LLVMDisasmContextRef;

#[cfg(feature = "llvm_support")]
const LLVMDisassembler_ReferenceType_InOut_None: uint64_t = 0;

#[cfg(feature = "llvm_support")]
unsafe extern "C" {
    fn LLVMNormalizeTargetTriple(triple: *const c_char) -> *mut c_char;
    fn LLVMGetDefaultTargetTriple() -> *mut c_char;
    fn LLVMCreateDisasmCPUFeatures(
        triple: *const c_char,
        cpu: *const c_char,
        features: *const c_char,
        dis_info: *mut c_void,
        tag_type: c_int,
        get_op_info: *mut c_void,
        symbol_look_up: Option<
            unsafe extern "C" fn(
                *mut c_void,
                uint64_t,
                *mut uint64_t,
                uint64_t,
                *mut *const c_char,
            ) -> *const c_char,
        >,
    ) -> LLVMDisasmContextRef;
    fn LLVMCreateDisasm(
        triple_name: *const c_char,
        dis_info: *mut c_void,
        tag_type: c_int,
        get_op_info: *mut c_void,
        symbol_look_up: Option<
            unsafe extern "C" fn(
                *mut c_void,
                uint64_t,
                *mut uint64_t,
                uint64_t,
                *mut *const c_char,
            ) -> *const c_char,
        >,
    ) -> LLVMDisasmContextRef;
    fn LLVMDisposeMessage(message: *mut c_char);
    fn LLVMDisasmInstruction(
        dc: LLVMDisasmContextRef,
        bytes: *mut u8,
        bytes_size: u64,
        pc: u64,
        out_string: *mut c_char,
        out_string_size: size_t,
    ) -> size_t;
    fn LLVMInitializeAllTargetInfos();
    fn LLVMInitializeAllTargetMCs();
    fn LLVMInitializeAllDisassemblers();
}

#[cfg(feature = "llvm_support")]
unsafe fn printf_json(mut s: *mut c_char) -> c_int {
    s = strtok(s, c" \t".as_ptr());
    jsonw_string_field(json_wtr, c"operation".as_ptr(), s);

    jsonw_name(json_wtr, c"operands".as_ptr());
    jsonw_start_array(json_wtr);
    oper_count = 1;

    loop {
        s = strtok(core::ptr::null_mut(), c" \t,()".as_ptr());
        if s.is_null() {
            break;
        }
        jsonw_string(json_wtr, s);
        oper_count += 1;
    }
    0
}

/*
 * This callback to set the ref_type is necessary to have the LLVM disassembler
 * print PC-relative addresses instead of byte offsets for branch instruction
 * targets.
 */
#[cfg(feature = "llvm_support")]
unsafe extern "C" fn symbol_lookup_callback(
    _disasm_info: *mut c_void,
    _ref_value: uint64_t,
    ref_type: *mut uint64_t,
    _ref_PC: uint64_t,
    _ref_name: *mut *const c_char,
) -> *const c_char {
    *ref_type = LLVMDisassembler_ReferenceType_InOut_None;
    core::ptr::null()
}

#[cfg(feature = "llvm_support")]
unsafe fn init_context(
    ctx: *mut disasm_ctx_t,
    arch: *const c_char,
    _disassembler_options: *const c_char,
    _image: *mut u8,
    _len: ssize_t,
    _func_ksym: __u64,
) -> c_int {
    let triple: *mut c_char;

    if !arch.is_null() {
        triple = LLVMNormalizeTargetTriple(arch);
    } else {
        triple = LLVMGetDefaultTargetTriple();
    }
    if triple.is_null() {
        p_err(c"Failed to retrieve triple".as_ptr());
        return -1;
    }

    /*
     * Enable all aarch64 ISA extensions so the disassembler can handle any
     * instruction the kernel JIT might emit (e.g. ARM64 LSE atomics).
     */
    if strncmp(triple, c"aarch64".as_ptr(), 7) == 0 {
        *ctx = LLVMCreateDisasmCPUFeatures(
            triple,
            c"".as_ptr(),
            c"+all".as_ptr(),
            core::ptr::null_mut(),
            0,
            core::ptr::null_mut(),
            Some(symbol_lookup_callback),
        );
    } else {
        *ctx = LLVMCreateDisasm(
            triple,
            core::ptr::null_mut(),
            0,
            core::ptr::null_mut(),
            Some(symbol_lookup_callback),
        );
    }
    LLVMDisposeMessage(triple);

    if (*ctx).is_null() {
        p_err(c"Failed to create disassembler".as_ptr());
        return -1;
    }

    0
}

#[cfg(feature = "llvm_support")]
unsafe fn destroy_context(ctx: *mut disasm_ctx_t) {
    LLVMDisposeMessage(*ctx as *mut c_char);
}

#[cfg(feature = "llvm_support")]
unsafe fn disassemble_insn(
    ctx: *mut disasm_ctx_t,
    image: *mut u8,
    len: ssize_t,
    pc: c_int,
    func_ksym: __u64,
) -> c_int {
    let mut buf = [0 as c_char; 256];
    let count: c_int;

    count = LLVMDisasmInstruction(
        *ctx,
        image.offset(pc as isize),
        (len - pc as ssize_t) as u64,
        func_ksym.wrapping_add(pc as __u64),
        buf.as_mut_ptr(),
        core::mem::size_of_val(&buf),
    ) as c_int;
    if json_output {
        printf_json(buf.as_mut_ptr());
    } else {
        printf(c"%s".as_ptr(), buf.as_ptr());
    }

    count
}

#[cfg(feature = "llvm_support")]
#[no_mangle]
pub unsafe extern "C" fn disasm_init() -> c_int {
    LLVMInitializeAllTargetInfos();
    LLVMInitializeAllTargetMCs();
    LLVMInitializeAllDisassemblers();
    0
}

#[cfg(feature = "libbfd_support")]
const DISASM_SPACER: *const c_char = c"\t".as_ptr();
#[cfg(feature = "llvm_support")]
const DISASM_SPACER: *const c_char = c"".as_ptr();

#[cfg(feature = "libbfd_support")]
#[repr(C)]
pub struct disassemble_info {
    pub arch: c_int,
    pub mach: c_ulong,
    pub disassembler_options: *const c_char,
    pub buffer: *mut u8,
    pub buffer_length: ssize_t,
    pub print_address_func: Option<unsafe extern "C" fn(bfd_vma, *mut disassemble_info)>,
}

#[cfg(feature = "libbfd_support")]
#[repr(C)]
pub struct disasm_info {
    pub info: disassemble_info,
    pub func_ksym: __u64,
}

#[cfg(feature = "libbfd_support")]
#[repr(C)]
pub struct bfd_arch_info_type {
    _private: [u8; 0],
}

#[cfg(feature = "libbfd_support")]
#[repr(C)]
pub struct bfd {
    pub arch_info: *const bfd_arch_info_type,
}

#[cfg(feature = "libbfd_support")]
type fprintf_ftype = Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...) -> c_int>;
#[cfg(feature = "libbfd_support")]
type disassembler_ftype = Option<unsafe extern "C" fn(c_int, *mut disassemble_info) -> c_int>;

#[cfg(feature = "libbfd_support")]
#[repr(C)]
pub enum disassembler_style {
    dis_style_text,
}

#[cfg(feature = "libbfd_support")]
#[repr(C)]
pub struct disasm_ctx_t {
    pub info: *mut disasm_info,
    pub disassemble: disassembler_ftype,
    pub bfdf: *mut bfd,
}

#[cfg(feature = "libbfd_support")]
unsafe extern "C" {
    static bfd_object: c_int;
    fn bfd_openr(filename: *const c_char, target: *const c_char) -> *mut bfd;
    fn bfd_check_format(abfd: *mut bfd, format: c_int) -> c_int;
    fn bfd_scan_arch(string: *const c_char) -> *const bfd_arch_info_type;
    fn bfd_get_arch(abfd: *mut bfd) -> c_int;
    fn bfd_get_mach(abfd: *mut bfd) -> c_ulong;
    fn bfd_big_endian(abfd: *mut bfd) -> c_int;
    fn bfd_close(abfd: *mut bfd) -> c_int;
    fn bfd_init();
    fn generic_print_address(addr: bfd_vma, info: *mut disassemble_info);
    fn init_disassemble_info_compat(
        info: *mut disassemble_info,
        stream: *mut c_void,
        fprintf_func: fprintf_ftype,
        fprintf_styled_func: Option<
            unsafe extern "C" fn(
                *mut c_void,
                disassembler_style,
                *const c_char,
                ...
            ) -> c_int,
        >,
    );
    fn fprintf_styled(out: *mut c_void, style: disassembler_style, fmt: *const c_char, ...) -> c_int;
    fn disassemble_init_for_target(info: *mut disassemble_info);
    #[cfg(feature = "disasm_four_args_signature")]
    fn disassembler(
        arch: c_int,
        big: c_int,
        mach: c_ulong,
        abfd: *mut bfd,
    ) -> disassembler_ftype;
    #[cfg(not(feature = "disasm_four_args_signature"))]
    fn disassembler(abfd: *mut bfd) -> disassembler_ftype;
}

#[cfg(feature = "libbfd_support")]
unsafe extern "C" fn disasm_print_addr(addr: bfd_vma, info: *mut disassemble_info) {
    let dinfo = info as *mut disasm_info;
    let addr = addr.wrapping_add((*dinfo).func_ksym);
    generic_print_address(addr, info);
}

#[cfg(feature = "libbfd_support")]
unsafe fn get_exec_path(tpath: *mut c_char, size: size_t) -> c_int {
    let path = c"/proc/self/exe".as_ptr();
    let len: ssize_t;

    len = readlink(path, tpath, size - 1);
    if len <= 0 {
        return -1;
    }

    *tpath.offset(len as isize) = 0;

    0
}

#[cfg(feature = "libbfd_support")]
unsafe fn printf_json(_out: *mut c_void, fmt: *const c_char, ap: va_list) -> c_int {
    let mut s: *mut c_char = core::ptr::null_mut();
    let err: c_int;

    err = vasprintf(&mut s, fmt, ap);
    if err < 0 {
        return -1;
    }

    if oper_count == 0 {
        let mut i: c_int;

        /* Strip trailing spaces */
        i = strlen(s) as c_int - 1;
        while *s.offset(i as isize) == b' ' as c_char {
            *s.offset(i as isize) = 0;
            i -= 1;
        }

        jsonw_string_field(json_wtr, c"operation".as_ptr(), s);
        jsonw_name(json_wtr, c"operands".as_ptr());
        jsonw_start_array(json_wtr);
        oper_count += 1;
    } else if strcmp(fmt, c",".as_ptr()) == 0 {
        /* Skip */
    } else {
        jsonw_string(json_wtr, s);
        oper_count += 1;
    }
    free(s as *mut c_void);
    0
}

#[cfg(feature = "libbfd_support")]
unsafe extern "C" fn fprintf_json(out: *mut c_void, fmt: *const c_char, mut args: ...) -> c_int {
    let r: c_int;

    r = printf_json(out, fmt, args.as_va_list() as va_list);

    r
}

#[cfg(feature = "libbfd_support")]
unsafe extern "C" fn fprintf_json_styled(
    out: *mut c_void,
    _style: disassembler_style,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    let r: c_int;

    r = printf_json(out, fmt, args.as_va_list() as va_list);

    r
}

#[cfg(feature = "libbfd_support")]
unsafe fn init_context(
    ctx: *mut disasm_ctx_t,
    arch: *const c_char,
    disassembler_options: *const c_char,
    image: *mut u8,
    len: ssize_t,
    func_ksym: __u64,
) -> c_int {
    let mut info: *mut disassemble_info;
    let mut tpath = [0 as c_char; PATH_MAX];
    let bfdf: *mut bfd;

    memset(
        tpath.as_mut_ptr() as *mut c_void,
        0,
        core::mem::size_of_val(&tpath),
    );
    if get_exec_path(tpath.as_mut_ptr(), core::mem::size_of_val(&tpath)) != 0 {
        p_err(c"failed to create disassembler (get_exec_path)".as_ptr());
        return -1;
    }

    (*ctx).bfdf = bfd_openr(tpath.as_ptr(), core::ptr::null());
    if (*ctx).bfdf.is_null() {
        p_err(c"failed to create disassembler (bfd_openr)".as_ptr());
        return -1;
    }
    if bfd_check_format((*ctx).bfdf, bfd_object) == 0 {
        p_err(c"failed to create disassembler (bfd_check_format)".as_ptr());
        bfd_close((*ctx).bfdf);
        return -1;
    }
    bfdf = (*ctx).bfdf;

    (*ctx).info = malloc(core::mem::size_of::<disasm_info>()) as *mut disasm_info;
    if (*ctx).info.is_null() {
        p_err(c"mem alloc failed".as_ptr());
        bfd_close((*ctx).bfdf);
        return -1;
    }
    (*(*ctx).info).func_ksym = func_ksym;
    info = &mut (*(*ctx).info).info;

    if json_output {
        init_disassemble_info_compat(
            info,
            stdout,
            Some(fprintf_json),
            Some(fprintf_json_styled),
        );
    } else {
        init_disassemble_info_compat(info, stdout, Some(fprintf), Some(fprintf_styled));
    }

    /* Update architecture info for offload. */
    if !arch.is_null() {
        let inf: *const bfd_arch_info_type = bfd_scan_arch(arch);

        if !inf.is_null() {
            (*bfdf).arch_info = inf;
        } else {
            p_err(c"No libbfd support for %s".as_ptr(), arch);
            free(info as *mut c_void);
            bfd_close((*ctx).bfdf);
            return -1;
        }
    }

    (*info).arch = bfd_get_arch(bfdf);
    (*info).mach = bfd_get_mach(bfdf);
    if !disassembler_options.is_null() {
        (*info).disassembler_options = disassembler_options;
    }
    (*info).buffer = image;
    (*info).buffer_length = len;
    (*info).print_address_func = Some(disasm_print_addr);

    disassemble_init_for_target(info);

    #[cfg(feature = "disasm_four_args_signature")]
    {
        (*ctx).disassemble = disassembler((*info).arch, bfd_big_endian(bfdf), (*info).mach, bfdf);
    }
    #[cfg(not(feature = "disasm_four_args_signature"))]
    {
        (*ctx).disassemble = disassembler(bfdf);
    }
    if (*ctx).disassemble.is_none() {
        p_err(c"failed to create disassembler".as_ptr());
        free(info as *mut c_void);
        bfd_close((*ctx).bfdf);
        return -1;
    }
    0
}

#[cfg(feature = "libbfd_support")]
unsafe fn destroy_context(ctx: *mut disasm_ctx_t) {
    free((*ctx).info as *mut c_void);
    bfd_close((*ctx).bfdf);
}

#[cfg(feature = "libbfd_support")]
unsafe fn disassemble_insn(
    ctx: *mut disasm_ctx_t,
    _image: *mut u8,
    _len: ssize_t,
    pc: c_int,
    _func_ksym: __u64,
) -> c_int {
    ((*ctx).disassemble.unwrap())(pc, &mut (*(*ctx).info).info)
}

#[cfg(feature = "libbfd_support")]
#[no_mangle]
pub unsafe extern "C" fn disasm_init() -> c_int {
    bfd_init();
    0
}

#[no_mangle]
pub unsafe extern "C" fn disasm_print_insn(
    image: *mut u8,
    len: ssize_t,
    opcodes: c_int,
    arch: *const c_char,
    disassembler_options: *const c_char,
    btf: *const btf,
    prog_linfo: *const bpf_prog_linfo,
    func_ksym: __u64,
    func_idx: c_uint,
    linum: bool_,
) -> c_int {
    let mut linfo: *const bpf_line_info = core::ptr::null();
    let mut nr_skip: c_uint = 0;
    let mut count: c_int;
    let mut i: c_int;
    let mut pc: c_uint = 0;
    let mut ctx: disasm_ctx_t = core::mem::zeroed();

    if len == 0 {
        return -1;
    }

    if init_context(&mut ctx, arch, disassembler_options, image, len, func_ksym) != 0 {
        return -1;
    }

    if json_output {
        jsonw_start_array(json_wtr);
    }
    loop {
        if !prog_linfo.is_null() {
            linfo = bpf_prog_linfo__lfind_addr_func(
                prog_linfo,
                func_ksym.wrapping_add(pc as __u64),
                func_idx,
                nr_skip,
            );
            if !linfo.is_null() {
                nr_skip += 1;
            }
        }

        if json_output {
            jsonw_start_object(json_wtr);
            oper_count = 0;
            if !linfo.is_null() {
                btf_dump_linfo_json(btf, linfo, linum);
            }
            jsonw_name(json_wtr, c"pc".as_ptr());
            jsonw_printf(json_wtr, c"\"0x%x\"".as_ptr(), pc);
        } else {
            if !linfo.is_null() {
                btf_dump_linfo_plain(btf, linfo, c"; ".as_ptr(), linum);
            }
            printf(c"%4x:%s".as_ptr(), pc, DISASM_SPACER);
        }

        count = disassemble_insn(&mut ctx, image, len, pc as c_int, func_ksym);

        if json_output {
            /*
             * Operand array, was started in fprintf_json. Before
             * that, make sure we have a _null_ value if no operand
             * other than operation code was present.
             */
            if oper_count == 1 {
                jsonw_null(json_wtr);
            }
            jsonw_end_array(json_wtr);
        }

        if opcodes != 0 {
            if json_output {
                jsonw_name(json_wtr, c"opcodes".as_ptr());
                jsonw_start_array(json_wtr);
                i = 0;
                while i < count {
                    jsonw_printf(
                        json_wtr,
                        c"\"0x%02hhx\"".as_ptr(),
                        *image.offset(pc as isize + i as isize) as uint8_t as c_int,
                    );
                    i += 1;
                }
                jsonw_end_array(json_wtr);
            } else {
                printf(c"\n\t".as_ptr());
                i = 0;
                while i < count {
                    printf(
                        c"%02x ".as_ptr(),
                        *image.offset(pc as isize + i as isize) as uint8_t as c_int,
                    );
                    i += 1;
                }
            }
        }
        if json_output {
            jsonw_end_object(json_wtr);
        } else {
            printf(c"\n".as_ptr());
        }

        pc = pc.wrapping_add(count as c_uint);
        if !(count > 0 && (pc as ssize_t) < len) {
            break;
        }
    }
    if json_output {
        jsonw_end_array(json_wtr);
    }

    destroy_context(&mut ctx);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
