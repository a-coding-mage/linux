// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/capstone.c.
// External declarations below correspond to symbols provided by perf, libc,
// libdl, ELF headers, and capstone headers.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type bool_t = bool;
type size_t = usize;
type ssize_t = isize;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type uint8_t = u8;
type uint16_t = u16;
type uint64_t = u64;
type csh = usize;
type FILE = c_void;

const PRINT_INSN_IMM_HEX: c_int = 1;

const EM_NONE: uint16_t = 0;
const EM_386: uint16_t = 3;
const EM_MIPS: uint16_t = 8;
const EM_PPC: uint16_t = 20;
const EM_PPC64: uint16_t = 21;
const EM_ARM: uint16_t = 40;
const EM_S390: uint16_t = 22;
const EM_X86_64: uint16_t = 62;
const EM_AARCH64: uint16_t = 183;
const EM_RISCV: uint16_t = 243;
const EM_SPARC: uint16_t = 2;
const EM_SPARCV9: uint16_t = 43;

const O_RDONLY: c_int = 0;
const RTLD_LAZY: c_int = 1;

// For Capstone < 5 these constants were supplied locally by macros in C.
const CS_ARCH_RISCV_COMPAT: cs_arch = 15;
const CS_MODE_RISCV32_COMPAT: cs_mode = 1;
const CS_MODE_RISCV64_COMPAT: cs_mode = 2;
const CS_MODE_RISCVC_COMPAT: cs_mode = 4;

type cs_arch = c_int;
type cs_mode = c_int;
type cs_err = c_int;
type cs_opt_type = c_int;

const CS_ERR_OK: cs_err = 0;
const CS_ERR_HANDLE: cs_err = 4;

const CS_ARCH_ARM: cs_arch = 0;
const CS_ARCH_ARM64: cs_arch = 1;
const CS_ARCH_MIPS: cs_arch = 2;
const CS_ARCH_X86: cs_arch = 3;
const CS_ARCH_PPC: cs_arch = 4;
const CS_ARCH_SPARC: cs_arch = 5;
const CS_ARCH_SYSZ: cs_arch = 6;
const CS_ARCH_RISCV: cs_arch = CS_ARCH_RISCV_COMPAT;

const CS_MODE_LITTLE_ENDIAN: cs_mode = 0;
const CS_MODE_ARM: cs_mode = 0;
const CS_MODE_16: cs_mode = 1 << 1;
const CS_MODE_32: cs_mode = 1 << 2;
const CS_MODE_64: cs_mode = 1 << 3;
const CS_MODE_THUMB: cs_mode = 1 << 4;
const CS_MODE_MCLASS: cs_mode = 1 << 5;
const CS_MODE_V8: cs_mode = 1 << 6;
const CS_MODE_MICRO: cs_mode = 1 << 4;
const CS_MODE_MIPS3: cs_mode = 1 << 5;
const CS_MODE_MIPS32R6: cs_mode = 1 << 6;
const CS_MODE_MIPS2: cs_mode = 1 << 7;
const CS_MODE_V9: cs_mode = 1 << 4;
const CS_MODE_BIG_ENDIAN: cs_mode = 1 << 31;
const CS_MODE_MIPS32: cs_mode = CS_MODE_32;
const CS_MODE_MIPS64: cs_mode = CS_MODE_64;
const CS_MODE_RISCV32: cs_mode = CS_MODE_RISCV32_COMPAT;
const CS_MODE_RISCV64: cs_mode = CS_MODE_RISCV64_COMPAT;
const CS_MODE_RISCVC: cs_mode = CS_MODE_RISCVC_COMPAT;

const CS_OPT_SYNTAX: cs_opt_type = 1;
const CS_OPT_DETAIL: cs_opt_type = 2;
const CS_OPT_SYNTAX_ATT: size_t = 2;
const CS_OPT_ON: size_t = 3;

const X86_OP_IMM: c_int = 2;
const X86_OP_MEM: c_int = 3;
const X86_REG_RIP: c_int = 41;

#[repr(C)]
pub struct cs_insn {
    id: c_uint,
    address: uint64_t,
    size: uint16_t,
    bytes: [uint8_t; 24],
    mnemonic: [c_char; 32],
    op_str: [c_char; 160],
    detail: *mut cs_detail,
}

#[repr(C)]
pub struct cs_detail {
    regs_read: [uint16_t; 20],
    regs_read_count: uint8_t,
    regs_write: [uint16_t; 20],
    regs_write_count: uint8_t,
    groups: [uint8_t; 8],
    groups_count: uint8_t,
    x86: cs_x86,
}

#[repr(C)]
pub struct cs_x86 {
    prefix: [uint8_t; 4],
    opcode: [uint8_t; 4],
    rex: uint8_t,
    addr_size: uint8_t,
    modrm: uint8_t,
    sib: uint8_t,
    disp: i64,
    sib_index: c_int,
    sib_scale: c_int,
    sib_base: c_int,
    xop_cc: c_int,
    sse_cc: c_int,
    avx_cc: c_int,
    avx_sae: bool_t,
    avx_rm: c_int,
    eflags: u64,
    op_count: uint8_t,
    operands: [cs_x86_op; 8],
    encoding: cs_x86_encoding,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_x86_op {
    type_: c_int,
    value: cs_x86_op_value,
    size: uint8_t,
    access: uint8_t,
    avx_bcast: c_int,
    avx_zero_opmask: bool_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cs_x86_op_value {
    reg: c_int,
    imm: i64,
    mem: x86_op_mem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_op_mem {
    segment: c_int,
    base: c_int,
    index: c_int,
    scale: c_int,
    disp: i64,
}

#[repr(C)]
pub struct cs_x86_encoding {
    modrm_offset: uint8_t,
    disp_offset: uint8_t,
    disp_size: uint8_t,
    imm_offset: uint8_t,
    imm_size: uint8_t,
}

#[repr(C)]
pub struct addr_location {
    _private: [u8; 0],
}
#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}
#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}
#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}
#[repr(C)]
pub struct nscookie {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    start: u64,
    end: u64,
    name: *const c_char,
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map_symbol {
    map: *mut map,
    sym: *mut symbol,
    thread: *mut thread,
}

#[repr(C)]
pub struct annotation_options {
    objdump_path: *const c_char,
    disassembler_style: *const c_char,
}

#[repr(C)]
pub struct arch {
    _private: [u8; 0],
}

#[repr(C)]
pub struct annotate_args {
    ms: *mut map_symbol,
    options: *mut annotation_options,
    arch: *mut arch,
    offset: i64,
    line: *mut c_char,
    line_nr: c_int,
    fileloc: *mut c_void,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct annotation_source {
    source: list_head,
}

#[repr(C)]
pub struct annotation {
    src: *mut annotation_source,
}

#[repr(C)]
pub struct annotation_line {
    node: list_head,
}

#[repr(C)]
pub struct disasm_line {
    al: annotation_line,
}

#[repr(C)]
struct find_file_offset_data {
    ip: u64,
    offset: u64,
}

unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;

    fn cs_open(arch: cs_arch, mode: cs_mode, handle: *mut csh) -> cs_err;
    fn cs_option(handle: csh, type_: cs_opt_type, value: size_t) -> cs_err;
    fn cs_disasm(
        handle: csh,
        code: *const uint8_t,
        code_size: size_t,
        address: uint64_t,
        count: size_t,
        insn: *mut *mut cs_insn,
    ) -> size_t;
    fn cs_free(insn: *mut cs_insn, count: size_t);
    fn cs_close(handle: *mut csh) -> cs_err;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warning_once(fmt: *const c_char, ...);
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn sprintf(str_: *mut c_char, format: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: u64) -> ssize_t;
    static mut errno: c_int;

    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn thread__find_symbol(
        thread: *mut thread,
        cpumode: u8,
        addr: u64,
        al: *mut addr_location,
    ) -> bool_t;
    fn symbol__fprintf_symname_offs(sym: *mut symbol, al: *mut addr_location, fp: *mut FILE)
        -> size_t;
    fn thread__e_machine_endian(
        thread: *mut thread,
        machine: *mut machine,
        e_flags: *mut c_void,
        is_big_endian: *mut bool_t,
    ) -> uint16_t;
    fn arch__is_x86(arch: *mut arch) -> bool_t;
    fn map__objdump_2mem(map: *mut map, ip: u64) -> u64;
    fn dso__kernel(dso: *mut dso) -> bool_t;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__kmaps(map: *mut map) -> *mut maps;
    fn maps__find(maps: *mut maps, addr: u64) -> *mut map;
    fn map__map_ip(map: *mut map, ip: u64) -> u64;
    fn map__find_symbol(map: *mut map, addr: u64) -> *mut symbol;
    fn map__put(map: *mut map);
    fn map__rip_2objdump(map: *mut map, ip: u64) -> u64;
    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
    fn dso__read_symbol(
        dso: *mut dso,
        filename: *const c_char,
        map: *mut map,
        sym: *mut symbol,
        code_buf: *mut *mut u8,
        buf_len: *mut u64,
        is_64bit: *mut bool_t,
    ) -> *const u8;
    fn disasm_line__new(args: *mut annotate_args) -> *mut disasm_line;
    fn annotation_line__add(al: *mut annotation_line, list: *mut list_head);
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn dso__e_machine_endian(
        dso: *mut dso,
        machine: *mut machine,
        e_flags: *mut c_void,
        is_big_endian: *mut bool_t,
    ) -> uint16_t;
    fn list_empty(head: *const list_head) -> bool_t;
    fn list_del_init(entry: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn disasm_line__free(dl: *mut disasm_line);
    fn nsinfo__mountns_enter(nsinfo: *mut c_void, nsc: *mut nscookie);
    fn nsinfo__mountns_exit(nsc: *mut nscookie);
    fn dso__nsinfo(dso: *mut dso) -> *mut c_void;
    fn file__read_maps(
        fd: c_int,
        exe: bool_t,
        cb: unsafe extern "C" fn(u64, u64, u64, *mut c_void) -> c_int,
        arg: *mut c_void,
        is_64bit: *mut bool_t,
    ) -> c_int;
}

unsafe fn list_first_disasm_line(list: *mut list_head) -> *mut disasm_line {
    (*list).next as *mut disasm_line
}

// Original C has an alternative LIBCAPSTONE_DLOPEN implementation compiled by
// preprocessor condition. This Rust translation preserves the direct-link path.
unsafe fn perf_cs_open(arch: cs_arch, mode: cs_mode, handle: *mut csh) -> cs_err {
    unsafe { cs_open(arch, mode, handle) }
}

unsafe fn perf_cs_option(handle: csh, type_: cs_opt_type, value: size_t) -> cs_err {
    unsafe { cs_option(handle, type_, value) }
}

unsafe fn perf_cs_disasm(
    handle: csh,
    code: *const uint8_t,
    code_size: size_t,
    address: uint64_t,
    count: size_t,
    insn: *mut *mut cs_insn,
) -> size_t {
    unsafe { cs_disasm(handle, code, code_size, address, count, insn) }
}

unsafe fn perf_cs_free(insn: *mut cs_insn, count: size_t) {
    unsafe { cs_free(insn, count) };
}

unsafe fn perf_cs_close(handle: *mut csh) -> cs_err {
    unsafe { cs_close(handle) }
}

unsafe fn e_machine_to_capstone(
    e_machine: uint16_t,
    is64: bool_t,
    is_big_endian: bool_t,
    arch: *mut cs_arch,
    mode: *mut cs_mode,
) -> bool_t {
    unsafe {
        *mode = if is_big_endian {
            CS_MODE_BIG_ENDIAN
        } else {
            CS_MODE_LITTLE_ENDIAN
        };

        match e_machine {
            EM_X86_64 | EM_386 => {
                *arch = CS_ARCH_X86;
                *mode |= if is64 { CS_MODE_64 } else { CS_MODE_32 };
                true
            }
            EM_AARCH64 => {
                *arch = CS_ARCH_ARM64;
                *mode |= CS_MODE_ARM;
                true
            }
            EM_ARM => {
                *arch = CS_ARCH_ARM;
                *mode |= CS_MODE_ARM | CS_MODE_V8;
                true
            }
            EM_S390 => {
                *arch = CS_ARCH_SYSZ;
                true
            }
            EM_MIPS => {
                *arch = CS_ARCH_MIPS;
                *mode |= if is64 { CS_MODE_MIPS64 } else { CS_MODE_MIPS32 };
                true
            }
            EM_PPC => {
                *arch = CS_ARCH_PPC;
                true
            }
            EM_PPC64 => {
                *arch = CS_ARCH_PPC;
                *mode |= CS_MODE_64;
                true
            }
            EM_SPARC => {
                *arch = CS_ARCH_SPARC;
                true
            }
            EM_SPARCV9 => {
                *arch = CS_ARCH_SPARC;
                *mode |= CS_MODE_V9;
                true
            }
            EM_RISCV => {
                *arch = CS_ARCH_RISCV;
                *mode |= (if is64 { CS_MODE_RISCV64 } else { CS_MODE_RISCV32 })
                    | CS_MODE_RISCVC;
                true
            }
            _ => false,
        }
    }
}

unsafe fn capstone_init(
    e_machine: uint16_t,
    cs_handle: *mut csh,
    is64: bool_t,
    is_big_endian: bool_t,
    disassembler_style: bool_t,
) -> c_int {
    let mut arch: cs_arch = 0;
    let mut mode: cs_mode = 0;

    unsafe {
        if !e_machine_to_capstone(e_machine, is64, is_big_endian, &mut arch, &mut mode) {
            return -1;
        }

        if perf_cs_open(arch, mode, cs_handle) != CS_ERR_OK {
            pr_warning_once(c"cs_open failed\n".as_ptr());
            return -1;
        }

        if arch == CS_ARCH_X86 {
            /*
             * In case of using capstone_init while symbol__disassemble
             * setting CS_OPT_SYNTAX_ATT depends if disassembler_style opts
             * is set via annotation args
             */
            if disassembler_style {
                perf_cs_option(*cs_handle, CS_OPT_SYNTAX, CS_OPT_SYNTAX_ATT);
            }
            /*
             * Resolving address operands to symbols is implemented
             * on x86 by investigating instruction details.
             */
            perf_cs_option(*cs_handle, CS_OPT_DETAIL, CS_OPT_ON);
        }
    }

    0
}

unsafe fn print_insn_x86(
    thread: *mut thread,
    cpumode: u8,
    insn: *mut cs_insn,
    print_opts: c_int,
    fp: *mut FILE,
) -> size_t {
    let mut al = MaybeUninit::<addr_location>::uninit();
    let mut printed: size_t = 0;

    unsafe {
        if !(*insn).detail.is_null() && (*(*insn).detail).x86.op_count == 1 {
            let op = &mut (*(*insn).detail).x86.operands[0] as *mut cs_x86_op;

            addr_location__init(al.as_mut_ptr());
            if (*op).type_ == X86_OP_IMM
                && thread__find_symbol(thread, cpumode, (*op).value.imm as u64, al.as_mut_ptr())
            {
                printed += fprintf(fp, c"%s ".as_ptr(), (*insn).mnemonic.as_ptr()) as size_t;
                printed += symbol__fprintf_symname_offs(ptr::null_mut(), al.as_mut_ptr(), fp);
                if (print_opts & PRINT_INSN_IMM_HEX) != 0 {
                    printed += fprintf(fp, c" [%#lx]".as_ptr(), (*op).value.imm as u64) as size_t;
                }
                addr_location__exit(al.as_mut_ptr());
                return printed;
            }
            addr_location__exit(al.as_mut_ptr());
        }

        printed += fprintf(
            fp,
            c"%s %s".as_ptr(),
            (*insn).mnemonic.as_ptr(),
            (*insn).op_str.as_ptr(),
        ) as size_t;
    }
    printed
}

#[no_mangle]
pub unsafe extern "C" fn capstone__fprintf_insn_asm(
    machine: *mut machine,
    thread: *mut thread,
    cpumode: u8,
    is64bit: bool_t,
    code: *const uint8_t,
    code_size: size_t,
    ip: uint64_t,
    lenp: *mut c_int,
    print_opts: c_int,
    fp: *mut FILE,
) -> ssize_t {
    let printed: size_t;
    let mut insn: *mut cs_insn = ptr::null_mut();
    let mut cs_handle: csh = 0;
    let count: size_t;
    let mut is_big_endian = false;
    let e_machine: uint16_t;
    let ret: c_int;

    unsafe {
        e_machine = thread__e_machine_endian(thread, machine, ptr::null_mut(), &mut is_big_endian);

        /* TODO: Try to initiate capstone only once but need a proper place. */
        ret = capstone_init(e_machine, &mut cs_handle, is64bit, is_big_endian, true);
        if ret < 0 {
            return ret as ssize_t;
        }

        count = perf_cs_disasm(cs_handle, code, code_size, ip, 1, &mut insn);
        if count > 0 {
            if e_machine == EM_X86_64 || e_machine == EM_386 {
                printed = print_insn_x86(thread, cpumode, &mut *insn, print_opts, fp);
            } else {
                printed = fprintf(
                    fp,
                    c"%s %s".as_ptr(),
                    (*insn).mnemonic.as_ptr(),
                    (*insn).op_str.as_ptr(),
                ) as size_t;
            }
            if !lenp.is_null() {
                *lenp = (*insn).size as c_int;
            }
            perf_cs_free(insn, count);
        } else {
            printed = (-1isize) as size_t;
        }

        perf_cs_close(&mut cs_handle);
    }
    printed as ssize_t
}

unsafe fn print_capstone_detail(
    insn: *mut cs_insn,
    buf: *mut c_char,
    len: size_t,
    args: *mut annotate_args,
    mut addr: u64,
) {
    let mut map = unsafe { (*(*args).ms).map };
    let mut sym: *mut symbol;

    /* TODO: support more architectures */
    unsafe {
        if !arch__is_x86((*args).arch) {
            return;
        }

        if (*insn).detail.is_null() {
            return;
        }

        for i in 0..(*(*insn).detail).x86.op_count as usize {
            let op = &mut (*(*insn).detail).x86.operands[i] as *mut cs_x86_op;
            let orig_addr: u64;
            let mut found_map: *mut map = ptr::null_mut();

            if (*op).type_ != X86_OP_MEM {
                continue;
            }

            /* only print RIP-based global symbols for now */
            if (*op).value.mem.base != X86_REG_RIP {
                continue;
            }

            /* get the target address */
            orig_addr = addr
                .wrapping_add((*insn).size as u64)
                .wrapping_add((*op).value.mem.disp as u64);
            addr = map__objdump_2mem(map, orig_addr);

            if dso__kernel(map__dso(map)) {
                /*
                 * The kernel maps can be split into sections, let's
                 * find the map first and then search the symbol.
                 */
                found_map = maps__find(map__kmaps(map), addr);
                if found_map.is_null() {
                    continue;
                }
                map = found_map;
            }

            /* convert it to map-relative address for search */
            addr = map__map_ip(map, addr);

            sym = map__find_symbol(map, addr);
            if sym.is_null() {
                map__put(found_map);
                continue;
            }

            if addr == (*sym).start {
                scnprintf(
                    buf,
                    len,
                    c"\t# %lx <%s>".as_ptr(),
                    orig_addr,
                    (*sym).name,
                );
            } else {
                scnprintf(
                    buf,
                    len,
                    c"\t# %lx <%s+%#lx>".as_ptr(),
                    orig_addr,
                    (*sym).name,
                    addr.wrapping_sub((*sym).start),
                );
            }
            map__put(found_map);
            break;
        }
    }
}

/* This will be called for each PHDR in an ELF binary */
unsafe extern "C" fn find_file_offset(start: u64, len: u64, pgoff: u64, arg: *mut c_void) -> c_int {
    let data = arg as *mut find_file_offset_data;

    unsafe {
        if start <= (*data).ip && (*data).ip < start.wrapping_add(len) {
            (*data).offset = pgoff.wrapping_add((*data).ip).wrapping_sub(start);
            return 1;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn symbol__disassemble_capstone(
    filename: *const c_char,
    sym: *mut symbol,
    args: *mut annotate_args,
) -> c_int {
    let notes: *mut annotation;
    let map: *mut map;
    let dso: *mut dso;
    let start: u64;
    let mut offset: u64;
    let mut i: c_int;
    let mut count: c_int;
    let mut free_count: c_int;
    let mut is_64bit = false;
    let mut needs_cs_close = false;
    /* Malloc-ed buffer containing instructions read from disk. */
    let mut code_buf: *mut u8 = ptr::null_mut();
    /* Pointer to code to be disassembled. */
    let buf: *const u8;
    let mut buf_len: u64 = 0;
    let mut handle: csh = 0;
    let mut insn: *mut cs_insn = ptr::null_mut();
    let mut disasm_buf = [0 as c_char; 512];
    let mut dl: *mut disasm_line;
    let mut disassembler_style = false;
    let mut e_machine: uint16_t = EM_NONE;
    let mut is_big_endian = false;

    unsafe {
        notes = symbol__annotation(sym);
        map = (*(*args).ms).map;
        dso = map__dso(map);
        start = map__rip_2objdump(map, (*sym).start);

        if !(*(*args).options).objdump_path.is_null() {
            return -1;
        }

        buf = dso__read_symbol(
            dso,
            filename,
            map,
            sym,
            &mut code_buf,
            &mut buf_len,
            &mut is_64bit,
        );
        if buf.is_null() {
            return errno;
        }

        /* add the function address and name */
        scnprintf(
            disasm_buf.as_mut_ptr(),
            disasm_buf.len(),
            c"%#lx <%s>:".as_ptr(),
            start,
            (*sym).name,
        );

        (*args).offset = -1;
        (*args).line = disasm_buf.as_mut_ptr();
        (*args).line_nr = 0;
        (*args).fileloc = ptr::null_mut();
        (*(*args).ms).sym = sym;

        dl = disasm_line__new(args);
        if dl.is_null() {
            count = -1;
            free(code_buf as *mut c_void);
            return count;
        }

        annotation_line__add(&mut (*dl).al, &mut (*(*notes).src).source);

        if (*(*args).options).disassembler_style.is_null()
            || strcmp((*(*args).options).disassembler_style, c"att".as_ptr()) == 0
        {
            disassembler_style = true;
        }

        if !(*(*args).ms).thread.is_null() {
            e_machine = thread__e_machine_endian(
                (*(*args).ms).thread,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut is_big_endian,
            );
        } else if !dso.is_null() {
            let kmaps = if !map.is_null() && dso__kernel(dso) {
                map__kmaps(map)
            } else {
                ptr::null_mut()
            };
            let kmap_machine = if !kmaps.is_null() {
                maps__machine(kmaps)
            } else {
                ptr::null_mut()
            };

            e_machine = dso__e_machine_endian(dso, kmap_machine, ptr::null_mut(), &mut is_big_endian);
        }
        if e_machine == 0 || e_machine == EM_NONE {
            e_machine = thread__e_machine_endian(
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut is_big_endian,
            );
        }
        if capstone_init(e_machine, &mut handle, is_64bit, is_big_endian, disassembler_style) < 0 {
            count = -1;
            free(code_buf as *mut c_void);
            return count;
        }

        needs_cs_close = true;

        free_count = perf_cs_disasm(handle, buf, buf_len as size_t, start, buf_len as size_t, &mut insn)
            as c_int;
        count = free_count;
        i = 0;
        offset = 0;
        while i < count {
            let printed: c_int;

            printed = scnprintf(
                disasm_buf.as_mut_ptr(),
                disasm_buf.len(),
                c"       %-7s %s".as_ptr(),
                (*insn.add(i as usize)).mnemonic.as_ptr(),
                (*insn.add(i as usize)).op_str.as_ptr(),
            );
            print_capstone_detail(
                insn.add(i as usize),
                disasm_buf.as_mut_ptr().add(printed as usize),
                disasm_buf.len() - printed as usize,
                args,
                start.wrapping_add(offset),
            );

            (*args).offset = offset as i64;
            (*args).line = disasm_buf.as_mut_ptr();

            dl = disasm_line__new(args);
            if dl.is_null() {
                let mut tmp: *mut disasm_line;
                tmp = ptr::null_mut();
                while !list_empty(&mut (*(*notes).src).source) {
                    dl = list_first_disasm_line(&mut (*(*notes).src).source);
                    list_del(&mut (*dl).al.node);
                    disasm_line__free(dl);
                }
                count = -1;
                break;
            }

            annotation_line__add(&mut (*dl).al, &mut (*(*notes).src).source);

            offset = offset.wrapping_add((*insn.add(i as usize)).size as u64);
            i += 1;
        }

        /* It failed in the middle: probably due to unknown instructions */
        if count >= 0 && offset != buf_len {
            let list = &mut (*(*notes).src).source as *mut list_head;

            /* Discard all lines and fallback to objdump */
            while !list_empty(list) {
                dl = list_first_disasm_line(list);

                list_del_init(&mut (*dl).al.node);
                disasm_line__free(dl);
            }
            count = -1;
        }

        if needs_cs_close {
            perf_cs_close(&mut handle);
            if free_count > 0 {
                perf_cs_free(insn, free_count as size_t);
            }
        }
        free(code_buf as *mut c_void);
    }
    if count < 0 { count } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn symbol__disassemble_capstone_powerpc(
    filename: *const c_char,
    sym: *mut symbol,
    args: *mut annotate_args,
) -> c_int {
    let notes: *mut annotation;
    let map: *mut map;
    let dso: *mut dso;
    let start: u64;
    let end: u64;
    let len: u64;
    let mut offset: u64;
    let mut i: c_int;
    let mut fd: c_int;
    let mut count: c_int = -1;
    let mut is_64bit = false;
    let mut needs_cs_close = false;
    let mut buf: *mut u8 = ptr::null_mut();
    let mut data = find_file_offset_data { ip: 0, offset: 0 };
    let mut handle: csh = 0;
    let mut disasm_buf = [0 as c_char; 512];
    let mut dl: *mut disasm_line;
    let mut line: *mut u32;
    let mut disassembler_style = false;
    let mut e_machine: uint16_t = EM_NONE;
    let mut is_big_endian = false;
    let mut nsc = MaybeUninit::<nscookie>::uninit();

    unsafe {
        notes = symbol__annotation(sym);
        map = (*(*args).ms).map;
        dso = map__dso(map);
        start = map__rip_2objdump(map, (*sym).start);
        end = map__rip_2objdump(map, (*sym).end);
        len = end.wrapping_sub(start);
        data.ip = start;

        if !(*(*args).options).objdump_path.is_null() {
            return -1;
        }

        nsinfo__mountns_enter(dso__nsinfo(dso), nsc.as_mut_ptr());
        fd = open(filename, O_RDONLY);
        nsinfo__mountns_exit(nsc.as_mut_ptr());
        if fd < 0 {
            return -1;
        }

        if file__read_maps(
            fd,
            true,
            find_file_offset,
            &mut data as *mut find_file_offset_data as *mut c_void,
            &mut is_64bit,
        ) == 0
        {
            goto_powerpc_err(
                fd,
                needs_cs_close,
                &mut handle,
                buf,
                count,
            );
            return -1;
        }

        if (*(*args).options).disassembler_style.is_null()
            || strcmp((*(*args).options).disassembler_style, c"att".as_ptr()) == 0
        {
            disassembler_style = true;
        }

        if !(*(*args).ms).thread.is_null() {
            e_machine = thread__e_machine_endian(
                (*(*args).ms).thread,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut is_big_endian,
            );
        } else if !dso.is_null() {
            let kmaps = if !map.is_null() && dso__kernel(dso) {
                map__kmaps(map)
            } else {
                ptr::null_mut()
            };
            let kmap_machine = if !kmaps.is_null() {
                maps__machine(kmaps)
            } else {
                ptr::null_mut()
            };

            e_machine = dso__e_machine_endian(dso, kmap_machine, ptr::null_mut(), &mut is_big_endian);
        }
        if e_machine == 0 || e_machine == EM_NONE {
            e_machine = thread__e_machine_endian(
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut is_big_endian,
            );
        }
        if capstone_init(e_machine, &mut handle, is_64bit, is_big_endian, disassembler_style) < 0 {
            close(fd);
            return -1;
        }

        needs_cs_close = true;

        buf = malloc(len as size_t) as *mut u8;
        if buf.is_null() {
            close(fd);
            if needs_cs_close {
                perf_cs_close(&mut handle);
            }
            return -1;
        }

        count = pread(fd, buf as *mut c_void, len as size_t, data.offset) as c_int;
        close(fd);
        fd = -1;

        if count as u64 != len {
            if needs_cs_close {
                perf_cs_close(&mut handle);
            }
            free(buf as *mut c_void);
            return -1;
        }

        line = buf as *mut u32;

        /* add the function address and name */
        scnprintf(
            disasm_buf.as_mut_ptr(),
            disasm_buf.len(),
            c"%#lx <%s>:".as_ptr(),
            start,
            (*sym).name,
        );

        (*args).offset = -1;
        (*args).line = disasm_buf.as_mut_ptr();
        (*args).line_nr = 0;
        (*args).fileloc = ptr::null_mut();
        (*(*args).ms).sym = sym;

        dl = disasm_line__new(args);
        if dl.is_null() {
            if needs_cs_close {
                perf_cs_close(&mut handle);
            }
            free(buf as *mut c_void);
            return -1;
        }

        annotation_line__add(&mut (*dl).al, &mut (*(*notes).src).source);

        /*
         * TODO: enable disassm for powerpc
         * count = cs_disasm(handle, buf, len, start, len, &insn);
         *
         * For now, only binary code is saved in disassembled line
         * to be used in "type" and "typeoff" sort keys. Each raw code
         * is 32 bit instruction. So use "len/4" to get the number of
         * entries.
         */
        count = (len / 4) as c_int;

        i = 0;
        offset = 0;
        while i < count {
            (*args).offset = offset as i64;
            sprintf((*args).line, c"%x".as_ptr(), *line.add(i as usize));

            dl = disasm_line__new(args);
            if dl.is_null() {
                break;
            }

            annotation_line__add(&mut (*dl).al, &mut (*(*notes).src).source);

            offset = offset.wrapping_add(4);
            i += 1;
        }

        /* It failed in the middle */
        if offset != len {
            let list = &mut (*(*notes).src).source as *mut list_head;

            /* Discard all lines and fallback to objdump */
            while !list_empty(list) {
                dl = list_first_disasm_line(list);

                list_del_init(&mut (*dl).al.node);
                disasm_line__free(dl);
            }
            count = -1;
        }

        if needs_cs_close {
            perf_cs_close(&mut handle);
        }
        free(buf as *mut c_void);
    }
    if count < 0 { count } else { 0 }
}

unsafe fn goto_powerpc_err(
    fd: c_int,
    needs_cs_close: bool_t,
    handle: *mut csh,
    buf: *mut u8,
    mut count: c_int,
) -> c_int {
    unsafe {
        if fd >= 0 {
            close(fd);
        }
        count = -1;
        if needs_cs_close {
            perf_cs_close(handle);
        }
        free(buf as *mut c_void);
    }
    count
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
