// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015-2017 Josh Poimboeuf <jpoimboe@redhat.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![feature(c_variadic)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void, VaListImpl};
use core::mem;
use core::ptr;

type size_t = usize;
type bfd_vma = c_ulong;
type disassembler_ftype =
    Option<unsafe extern "C" fn(bfd_vma, *mut disassemble_info) -> c_int>;
type fprintf_ftype = Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...) -> c_int>;
type fprintf_styled_ftype = Option<
    unsafe extern "C" fn(*mut c_void, disassembler_style, *const c_char, ...) -> c_int,
>;

const DISAS_RESULT_SIZE: usize = 1024;
const DISAS_ALT_MAX: usize = 5;
const DISAS_ALT_INSN_MAX: usize = 50;
const ALT_FLAGS_SHIFT: c_uint = 16;
const ALT_FLAG_NOT: c_int = 1 << 0;
const ALT_FLAG_DIRECT_CALL: c_int = 1 << 1;
const ALT_FEATURE_MASK: c_uint = (1 << ALT_FLAGS_SHIFT) - 1;
const DISAS_INSN_OFFSET_SPACE: c_int = 10;
const DISAS_INSN_SPACE: c_int = 60;

const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const ELFCLASS32: u8 = 1;
const ELFCLASS64: u8 = 2;
const ELFDATA2MSB: u8 = 2;
const BFD_ENDIAN_BIG: c_int = 0;
const BFD_ENDIAN_LITTLE: c_int = 1;
const bfd_arch_unknown: c_int = 0;

const ALT_TYPE_INSTRUCTIONS: c_int = 0;
const ALT_TYPE_EX_TABLE: c_int = 1;
const ALT_TYPE_JUMP_TABLE: c_int = 2;
const INSN_NOP: c_int = 0;

#[repr(C)]
pub struct disas_context {
    file: *mut objtool_file,
    insn: *mut instruction,
    alt_applied: bool,
    result: [c_char; DISAS_RESULT_SIZE],
    disassembler: disassembler_ftype,
    info: disassemble_info,
}

#[repr(C)]
struct disas_alt_insn {
    str_: *mut c_char,
    offset: c_int,
    nops: c_int,
}

#[repr(C)]
struct disas_alt {
    orig_insn: *mut instruction, /* original instruction */
    alt: *mut alternative,       /* alternative or NULL if default code */
    name: *mut c_char,           /* name for this alternative */
    width: c_int,                /* formatting width */
    insn: [disas_alt_insn; DISAS_ALT_INSN_MAX], /* alternative instructions */
    insn_idx: c_int,             /* index of the next instruction to print */
}

#[repr(C)]
pub struct objtool_file {
    elf: *mut elf,
}

#[repr(C)]
pub struct elf {
    ehdr: elf_header,
}

#[repr(C)]
pub struct elf_header {
    e_ident: [u8; 16],
}

#[repr(C)]
pub struct section {
    data: *mut section_data,
}

#[repr(C)]
pub struct section_data {
    d_buf: *mut c_void,
}

#[repr(C)]
pub struct symbol {
    name: *mut c_char,
    offset: c_ulong,
    len: c_ulong,
    sec: *mut section,
    warned: bool,
}

#[repr(C)]
pub struct instruction {
    sec: *mut section,
    offset: c_ulong,
    len: c_int,
    type_: c_int,
    alt_group: *mut alt_group,
    jump_dest: *mut instruction,
    alts: *mut alternative,
}

#[repr(C)]
pub struct alt_group {
    first_insn: *mut instruction,
    last_insn: *mut instruction,
    orig_group: *mut alt_group,
    nop: *mut instruction,
    feature: c_int,
}

#[repr(C)]
pub struct alternative {
    type_: c_int,
    insn: *mut instruction,
    next: *mut alternative,
}

#[repr(C)]
pub struct reloc {
    sym: *mut symbol,
}

#[repr(C)]
pub struct disassemble_info {
    fprintf_func: fprintf_ftype,
    fprintf_styled_func: fprintf_styled_ftype,
    stream: *mut c_void,
    application_data: *mut c_void,
    arch: c_int,
    mach: c_ulong,
    disassembler_options: *const c_char,
    read_memory_func: *mut c_void,
    print_address_func: Option<unsafe extern "C" fn(bfd_vma, *mut disassemble_info)>,
    endian: c_int,
    buffer: *mut c_void,
    buffer_vma: bfd_vma,
    buffer_length: c_ulong,
}

#[repr(C)]
pub enum disassembler_style {
    dis_style_text = 0,
}

#[repr(C)]
pub struct opts_t {
    wide: bool,
    verbose: bool,
    disas: *const c_char,
}

unsafe extern "C" {
    static mut stdout: *mut FILE;
    static mut opts: opts_t;
    static mut sym_name_max_len: c_int;

    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn vasprintf(strp: *mut *mut c_char, fmt: *const c_char, ap: VaListImpl) -> c_int;
    fn sprintf(str: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(str: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn vsnprintf(str: *mut c_char, size: size_t, fmt: *const c_char, ap: VaListImpl) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, fmt: *const c_char, ap: VaListImpl) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn fnmatch(pattern: *const c_char, string: *const c_char, flags: c_int) -> c_int;

    fn WARN(fmt: *const c_char, ...);
    fn WARN_FUNC(sec: *mut section, offset: c_ulong, fmt: *const c_char, ...);
    fn offstr(sec: *mut section, offset: c_ulong) -> *mut c_char;
    fn insn_sym(insn: *mut instruction) -> *mut symbol;
    fn find_reloc_by_dest_range(
        elf: *mut elf,
        sec: *mut section,
        offset: c_ulong,
        len: c_int,
    ) -> *mut reloc;
    fn arch_insn_adjusted_addend(insn: *mut instruction, reloc: *mut reloc) -> c_ulong;
    fn is_sec_sym(sym: *mut symbol) -> bool;
    fn insn_call_dest(insn: *mut instruction) -> *mut symbol;
    fn init_disassemble_info_compat(
        dinfo: *mut disassemble_info,
        stream: *mut c_void,
        fprintf_func: fprintf_ftype,
        fprintf_styled_func: fprintf_styled_ftype,
    );
    fn buffer_read_memory();
    fn arch_disas_info_init(dinfo: *mut disassemble_info) -> c_int;
    fn disassemble_init_for_target(dinfo: *mut disassemble_info);
    fn disassembler(
        arch: c_int,
        big: bool,
        mach: c_ulong,
        abfd: *mut c_void,
    ) -> disassembler_ftype;
    fn sec_size(sec: *mut section) -> c_ulong;
    fn next_insn_same_sec(file: *mut objtool_file, insn: *mut instruction) -> *mut instruction;
    fn arch_cpu_feature_name(num: c_int) -> *const c_char;
    fn is_text_sec(sec: *mut section) -> bool;
    fn first_insn_for_sym(file: *mut objtool_file, sym: *mut symbol) -> *mut instruction;
    fn next_insn_for_sym(file: *mut objtool_file, sym: *mut symbol, insn: *mut instruction)
        -> *mut instruction;
    fn first_symbol(elf: *mut elf) -> *mut symbol;
    fn next_symbol(elf: *mut elf, sym: *mut symbol) -> *mut symbol;
    fn first_section(elf: *mut elf) -> *mut section;
    fn next_section(elf: *mut elf, sec: *mut section) -> *mut section;
    fn first_symbol_for_sec(sec: *mut section) -> *mut symbol;
    fn next_symbol_for_sec(sec: *mut section, sym: *mut symbol) -> *mut symbol;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

fn DALT_DEFAULT(dalt: *mut disas_alt) -> bool {
    unsafe { (*dalt).alt.is_null() }
}

unsafe fn DALT_INSN(dalt: *mut disas_alt) -> *mut instruction {
    if DALT_DEFAULT(dalt) {
        (*dalt).orig_insn
    } else {
        (*(*dalt).alt).insn
    }
}

unsafe fn DALT_GROUP(dalt: *mut disas_alt) -> *mut alt_group {
    (*DALT_INSN(dalt)).alt_group
}

unsafe fn DALT_ALTID(dalt: *mut disas_alt) -> c_ulong {
    (*(*dalt).orig_insn).offset
}

fn alt_feature(ft_flags: c_uint) -> c_int {
    (ft_flags & ALT_FEATURE_MASK) as c_int
}

fn alt_flags(ft_flags: c_uint) -> c_int {
    (ft_flags >> ALT_FLAGS_SHIFT) as c_int
}

/*
 * Wrapper around asprintf() to allocate and format a string.
 * Return the allocated string or NULL on error.
 */
unsafe extern "C" fn strfmt(fmt: *const c_char, mut ap: ...) -> *mut c_char {
    let mut str_: *mut c_char = ptr::null_mut();
    let rv = vasprintf(&mut str_, fmt, ap.as_va_list());
    if rv == -1 { ptr::null_mut() } else { str_ }
}

unsafe fn sprint_name(str_: *mut c_char, name: *const c_char, offset: c_ulong) -> c_int {
    if offset != 0 {
        sprintf(str_, c"%s+0x%lx".as_ptr(), name, offset)
    } else {
        sprintf(str_, c"%s".as_ptr(), name)
    }
}

unsafe fn DINFO_FPRINTF(dinfo: *mut disassemble_info, fmt: *const c_char, mut ap: ...) -> c_int {
    ((*dinfo).fprintf_func.unwrap())((*dinfo).stream, fmt, ap.as_va_list())
}

unsafe fn disas_result_fprintf(
    dctx: *mut disas_context,
    fmt: *const c_char,
    ap: VaListImpl,
) -> c_int {
    let buf = (*dctx).result.as_mut_ptr();
    let mut len = strlen(buf) as c_int;
    if len >= (DISAS_RESULT_SIZE as c_int - 1) {
        WARN_FUNC((*(*dctx).insn).sec, (*(*dctx).insn).offset, c"disassembly buffer is full".as_ptr());
        return -1;
    }
    let avail = DISAS_RESULT_SIZE as c_int - len;

    len = vsnprintf(buf.add(len as usize), avail as size_t, fmt, ap);
    if len < 0 || len >= avail {
        WARN_FUNC(
            (*(*dctx).insn).sec,
            (*(*dctx).insn).offset,
            c"disassembly buffer is truncated".as_ptr(),
        );
        return -1;
    }

    0
}

unsafe extern "C" fn disas_fprintf(stream: *mut c_void, fmt: *const c_char, mut arg: ...) -> c_int {
    disas_result_fprintf(stream as *mut disas_context, fmt, arg.as_va_list())
}

/*
 * For init_disassemble_info_compat().
 */
unsafe extern "C" fn disas_fprintf_styled(
    stream: *mut c_void,
    _style: disassembler_style,
    fmt: *const c_char,
    mut arg: ...
) -> c_int {
    disas_result_fprintf(stream as *mut disas_context, fmt, arg.as_va_list())
}

unsafe fn disas_print_addr_sym(
    sec: *mut section,
    sym: *mut symbol,
    addr: bfd_vma,
    dinfo: *mut disassemble_info,
) {
    let mut symstr = [0 as c_char; 1024];
    if !sym.is_null() {
        sprint_name(symstr.as_mut_ptr(), (*sym).name, addr - (*sym).offset);
        DINFO_FPRINTF(dinfo, c"%#lx <%s>".as_ptr(), addr, symstr.as_mut_ptr());
    } else {
        let str_ = offstr(sec, addr);
        DINFO_FPRINTF(dinfo, c"%#lx <%s>".as_ptr(), addr, str_);
        free(str_ as *mut c_void);
    }
}

unsafe extern "C" fn disas_print_addr_alt(addr: bfd_vma, dinfo: *mut disassemble_info) -> bool {
    let dctx = (*dinfo).application_data as *mut disas_context;
    let alt_group: *mut alt_group;
    let orig_first_insn: *mut instruction;
    let offset: c_ulong;
    let sym: *mut symbol;

    /*
     * Check if we are processing an alternative at the original
     * instruction address (i.e. if alt_applied is true) and if
     * we are referencing an address inside the alternative.
     *
     * For example, this happens if there is a branch inside an
     * alternative. In that case, the address should be updated
     * to a reference inside the original instruction flow.
     */
    if !(*dctx).alt_applied {
        return false;
    }

    alt_group = (*(*dctx).insn).alt_group;
    if alt_group.is_null()
        || (*alt_group).orig_group.is_null()
        || addr < (*(*alt_group).first_insn).offset
        || addr > (*(*alt_group).last_insn).offset
    {
        return false;
    }

    orig_first_insn = (*(*alt_group).orig_group).first_insn;
    offset = addr - (*(*alt_group).first_insn).offset;

    let addr = (*orig_first_insn).offset + offset;
    sym = insn_sym(orig_first_insn);

    disas_print_addr_sym((*orig_first_insn).sec, sym, addr, dinfo);

    true
}

unsafe extern "C" fn disas_print_addr_noreloc(addr: bfd_vma, dinfo: *mut disassemble_info) {
    let dctx = (*dinfo).application_data as *mut disas_context;
    let insn = (*dctx).insn;
    let mut sym = insn_sym(insn);

    if disas_print_addr_alt(addr, dinfo) {
        return;
    }

    if !sym.is_null() && (addr < (*sym).offset || addr >= (*sym).offset + (*sym).len) {
        sym = ptr::null_mut();
    }

    disas_print_addr_sym((*insn).sec, sym, addr, dinfo);
}

unsafe extern "C" fn disas_print_addr_reloc(addr: bfd_vma, dinfo: *mut disassemble_info) {
    let dctx = (*dinfo).application_data as *mut disas_context;
    let insn = (*dctx).insn;
    let mut symstr = [0 as c_char; 1024];

    let reloc = find_reloc_by_dest_range((*(*dctx).file).elf, (*insn).sec, (*insn).offset, (*insn).len);
    if reloc.is_null() {
        /*
         * There is no relocation for this instruction although
         * the address to resolve points to the next instruction.
         * So this is an effective reference to the next IP, for
         * example: "lea 0x0(%rip),%rdi". The kernel can reference
         * the next IP with _THIS_IP_ macro.
         */
        DINFO_FPRINTF(dinfo, c"%#lx <%s>".as_ptr(), addr, c"_THIS_IP_".as_ptr());
        return;
    }

    let offset = arch_insn_adjusted_addend(insn, reloc);

    /*
     * If the relocation symbol is a section name (for example ".bss")
     * then we try to further resolve the name.
     */
    if is_sec_sym((*reloc).sym) {
        let str_ = offstr((*(*reloc).sym).sec, (*(*reloc).sym).offset + offset);
        DINFO_FPRINTF(dinfo, c"%#lx <%s>".as_ptr(), addr, str_);
        free(str_ as *mut c_void);
    } else {
        sprint_name(symstr.as_mut_ptr(), (*(*reloc).sym).name, offset);
        DINFO_FPRINTF(dinfo, c"%#lx <%s>".as_ptr(), addr, symstr.as_mut_ptr());
    }
}

/*
 * Resolve an address into a "<symbol>+<offset>" string.
 */
unsafe extern "C" fn disas_print_address(addr: bfd_vma, dinfo: *mut disassemble_info) {
    let dctx = (*dinfo).application_data as *mut disas_context;
    let insn = (*dctx).insn;

    /*
     * If the instruction is a call/jump and it references a
     * destination then this is likely the address we are looking
     * up. So check it first.
     */
    let jump_dest = (*insn).jump_dest;
    if !jump_dest.is_null() && !insn_sym(jump_dest).is_null() && (*jump_dest).offset == addr {
        if !disas_print_addr_alt(addr, dinfo) {
            disas_print_addr_sym((*jump_dest).sec, insn_sym(jump_dest), addr, dinfo);
        }
        return;
    }

    /*
     * If the address points to the next instruction then there is
     * probably a relocation. It can be a false positive when the
     * current instruction is referencing the address of the next
     * instruction. This particular case will be handled in
     * disas_print_addr_reloc().
     */
    let is_reloc = addr == (*insn).offset + (*insn).len as c_ulong;

    /*
     * The call destination offset can be the address we are looking
     * up, or 0 if there is a relocation.
     */
    let sym = insn_call_dest(insn);
    if !sym.is_null() && ((*sym).offset == addr || ((*sym).offset == 0 && is_reloc)) {
        DINFO_FPRINTF(dinfo, c"%#lx <%s>".as_ptr(), addr, (*sym).name);
        return;
    }

    if !is_reloc {
        disas_print_addr_noreloc(addr, dinfo);
    } else {
        disas_print_addr_reloc(addr, dinfo);
    }
}

/*
 * Initialize disassemble info arch, mach (32 or 64-bit) and options.
 */
#[no_mangle]
pub unsafe extern "C" fn disas_info_init(
    dinfo: *mut disassemble_info,
    arch: c_int,
    mach32: c_int,
    mach64: c_int,
    options: *const c_char,
) -> c_int {
    let dctx = (*dinfo).application_data as *mut disas_context;
    let file = (*dctx).file;

    (*dinfo).arch = arch;

    match (*(*file).elf).ehdr.e_ident[EI_CLASS] {
        ELFCLASS32 => (*dinfo).mach = mach32 as c_ulong,
        ELFCLASS64 => (*dinfo).mach = mach64 as c_ulong,
        _ => return -1,
    }

    (*dinfo).disassembler_options = options;

    0
}

#[no_mangle]
pub unsafe extern "C" fn disas_context_create(file: *mut objtool_file) -> *mut disas_context {
    let dctx = malloc(mem::size_of::<disas_context>()) as *mut disas_context;
    if dctx.is_null() {
        WARN(c"failed to allocate disassembly context".as_ptr());
        return ptr::null_mut();
    }

    (*dctx).file = file;
    let dinfo = &mut (*dctx).info as *mut disassemble_info;

    init_disassemble_info_compat(
        dinfo,
        dctx as *mut c_void,
        Some(disas_fprintf),
        Some(disas_fprintf_styled),
    );

    (*dinfo).read_memory_func = buffer_read_memory as *mut c_void;
    (*dinfo).print_address_func = Some(disas_print_address);
    (*dinfo).application_data = dctx as *mut c_void;

    /*
     * bfd_openr() is not used to avoid doing ELF data processing
     * and caching that has already being done. Here, we just need
     * to identify the target file so we call an arch specific
     * function to fill some disassemble info (arch, mach).
     */

    (*dinfo).arch = bfd_arch_unknown;
    (*dinfo).mach = 0;

    let err = arch_disas_info_init(dinfo);
    if err != 0 || (*dinfo).arch == bfd_arch_unknown || (*dinfo).mach == 0 {
        WARN(c"failed to init disassembly arch".as_ptr());
        free(dctx as *mut c_void);
        return ptr::null_mut();
    }

    (*dinfo).endian = if (*(*file).elf).ehdr.e_ident[EI_DATA] == ELFDATA2MSB {
        BFD_ENDIAN_BIG
    } else {
        BFD_ENDIAN_LITTLE
    };

    disassemble_init_for_target(dinfo);

    (*dctx).disassembler = disassembler(
        (*dinfo).arch,
        (*dinfo).endian == BFD_ENDIAN_BIG,
        (*dinfo).mach,
        ptr::null_mut(),
    );
    if (*dctx).disassembler.is_none() {
        WARN(c"failed to create disassembler function".as_ptr());
        free(dctx as *mut c_void);
        return ptr::null_mut();
    }

    dctx
}

#[no_mangle]
pub unsafe extern "C" fn disas_context_destroy(dctx: *mut disas_context) {
    free(dctx as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn disas_result(dctx: *mut disas_context) -> *mut c_char {
    (*dctx).result.as_mut_ptr()
}

unsafe fn disas_vprint(
    stream: *mut FILE,
    sec: *mut section,
    offset: c_ulong,
    mut depth: c_int,
    format: *const c_char,
    ap: VaListImpl,
) -> c_int {
    let mut len = sym_name_max_len + DISAS_INSN_OFFSET_SPACE;
    if depth < 0 {
        len += depth;
        depth = 0;
    }

    let mut n = 0;

    if !sec.is_null() {
        let addr_str = offstr(sec, offset);
        n += fprintf(stream, c"%6lx:  %-*s  ".as_ptr(), offset, len, addr_str);
        free(addr_str as *mut c_void);
    } else {
        len += DISAS_INSN_OFFSET_SPACE + 1;
        n += fprintf(stream, c"%-*s".as_ptr(), len, c"".as_ptr());
    }

    /* print vertical bars to show the code flow */
    for _ in 0..depth {
        n += fprintf(stream, c"| ".as_ptr());
    }

    if !format.is_null() {
        n += vfprintf(stream, format, ap);
    }

    n
}

unsafe extern "C" fn disas_print(
    stream: *mut FILE,
    sec: *mut section,
    offset: c_ulong,
    depth: c_int,
    format: *const c_char,
    mut args: ...
) -> c_int {
    disas_vprint(stream, sec, offset, depth, format, args.as_va_list())
}

/*
 * Print a message in the instruction flow. If insn is not NULL then
 * the instruction address is printed in addition of the message,
 * otherwise only the message is printed. In all cases, the instruction
 * itself is not printed.
 */
#[no_mangle]
pub unsafe extern "C" fn disas_print_info(
    stream: *mut FILE,
    insn: *mut instruction,
    depth: c_int,
    format: *const c_char,
    mut args: ...
) {
    let (sec, off) = if !insn.is_null() {
        ((*insn).sec, (*insn).offset)
    } else {
        (ptr::null_mut(), 0)
    };

    disas_vprint(stream, sec, off, depth, format, args.as_va_list());
}

/*
 * Print an instruction address (offset and function), the instruction itself
 * and an optional message.
 */
#[no_mangle]
pub unsafe extern "C" fn disas_print_insn(
    stream: *mut FILE,
    dctx: *mut disas_context,
    insn: *mut instruction,
    depth: c_int,
    format: *const c_char,
    mut args: ...
) {
    let mut fake_nop_insn = [0 as c_char; 32];

    /*
     * Alternative can insert a fake nop, sometimes with no
     * associated section so nothing to disassemble.
     */
    let fake_nop = (*insn).sec.is_null() && (*insn).type_ == INSN_NOP;
    let insn_str = if fake_nop {
        snprintf(
            fake_nop_insn.as_mut_ptr(),
            32,
            c"<fake nop> (%d bytes)".as_ptr(),
            (*insn).len,
        );
        fake_nop_insn.as_mut_ptr()
    } else {
        disas_insn(dctx, insn);
        disas_result(dctx)
    };

    /* print the instruction */
    let len = if (depth + 1) * 2 < DISAS_INSN_SPACE {
        DISAS_INSN_SPACE - (depth + 1) * 2
    } else {
        1
    };
    disas_print_info(stream, insn, depth, c"%-*s".as_ptr(), len, insn_str);

    /* print message if any */
    if format.is_null() {
        return;
    }

    if strcmp(format, c"\n".as_ptr()) == 0 {
        fprintf(stream, c"\n".as_ptr());
        return;
    }

    fprintf(stream, c" - ".as_ptr());
    vfprintf(stream, format, args.as_va_list());
}

/*
 * Disassemble a single instruction. Return the size of the instruction.
 *
 * If alt_applied is true then insn should be an instruction from of an
 * alternative (i.e. insn->alt_group != NULL), and it is disassembled
 * at the location of the original code it is replacing. When the
 * instruction references any address inside the alternative then
 * these references will be re-adjusted to replace the original code.
 */
unsafe fn disas_insn_common(
    dctx: *mut disas_context,
    insn: *mut instruction,
    alt_applied: bool,
) -> size_t {
    let disasm = (*dctx).disassembler.unwrap();
    let dinfo = &mut (*dctx).info as *mut disassemble_info;

    (*dctx).insn = insn;
    (*dctx).alt_applied = alt_applied;
    (*dctx).result[0] = 0;

    if (*insn).type_ == INSN_NOP {
        DINFO_FPRINTF(dinfo, c"nop%d".as_ptr(), (*insn).len);
        return (*insn).len as size_t;
    }

    /*
     * Set the disassembler buffer to read data from the section
     * containing the instruction to disassemble.
     */
    (*dinfo).buffer = (*(*insn).sec).data.as_mut().unwrap().d_buf;
    (*dinfo).buffer_vma = 0;
    (*dinfo).buffer_length = sec_size((*insn).sec);

    disasm((*insn).offset, &mut (*dctx).info) as size_t
}

#[no_mangle]
pub unsafe extern "C" fn disas_insn(dctx: *mut disas_context, insn: *mut instruction) -> size_t {
    disas_insn_common(dctx, insn, false)
}

unsafe fn disas_insn_alt(dctx: *mut disas_context, insn: *mut instruction) -> size_t {
    disas_insn_common(dctx, insn, true)
}

unsafe fn next_insn_same_alt(
    file: *mut objtool_file,
    alt_grp: *mut alt_group,
    insn: *mut instruction,
) -> *mut instruction {
    if (*alt_grp).last_insn == insn || (*alt_grp).nop == insn {
        return ptr::null_mut();
    }

    next_insn_same_sec(file, insn)
}

/*
 * Provide a name for the type of alternatives present at the
 * specified instruction.
 *
 * An instruction can have alternatives with different types, for
 * example alternative instructions and an exception table. In that
 * case the name for the alternative instructions type is used.
 *
 * Return NULL if the instruction as no alternative.
 */
#[no_mangle]
pub unsafe extern "C" fn disas_alt_type_name(insn: *mut instruction) -> *const c_char {
    let mut name: *const c_char = ptr::null();
    let mut alt = (*insn).alts;
    while !alt.is_null() {
        if (*alt).type_ == ALT_TYPE_INSTRUCTIONS {
            name = c"alternative".as_ptr();
            break;
        }

        match (*alt).type_ {
            ALT_TYPE_EX_TABLE => name = c"ex_table".as_ptr(),
            ALT_TYPE_JUMP_TABLE => name = c"jump_table".as_ptr(),
            _ => name = c"unknown".as_ptr(),
        }
        alt = (*alt).next;
    }

    name
}

/*
 * Provide a name for an alternative.
 */
#[no_mangle]
pub unsafe extern "C" fn disas_alt_name(alt: *mut alternative) -> *mut c_char {
    let mut pfx = [0 as c_char; 4];
    let mut str_: *mut c_char = ptr::null_mut();

    match (*alt).type_ {
        ALT_TYPE_EX_TABLE => {
            str_ = strdup(c"EXCEPTION".as_ptr());
        }
        ALT_TYPE_JUMP_TABLE => {
            str_ = strdup(c"JUMP".as_ptr());
        }
        ALT_TYPE_INSTRUCTIONS => {
            /*
             * This is a non-default group alternative. Create a name
             * based on the feature and flags associated with this
             * alternative. Use either the feature name (it is available)
             * or the feature number. And add a prefix to show the flags
             * used.
             *
             * Prefix flags characters:
             *
             *   '!'  alternative used when feature not enabled
             *   '+'  direct call alternative
             *   '?'  unknown flag
             */
            if (*(*alt).insn).alt_group.is_null() {
                return ptr::null_mut();
            }

            let feature = (*(*(*alt).insn).alt_group).feature as c_uint;
            let num = alt_feature(feature);
            let flags = alt_flags(feature);
            let mut p = pfx.as_mut_ptr();

            if flags & !(ALT_FLAG_NOT | ALT_FLAG_DIRECT_CALL) != 0 {
                *p = b'?' as c_char;
                p = p.add(1);
            }
            if flags & ALT_FLAG_DIRECT_CALL != 0 {
                *p = b'+' as c_char;
                p = p.add(1);
            }
            if flags & ALT_FLAG_NOT != 0 {
                *p = b'!' as c_char;
            }

            let name = arch_cpu_feature_name(num);
            if name.is_null() {
                str_ = strfmt(c"%sFEATURE 0x%X".as_ptr(), pfx.as_mut_ptr(), num);
            } else {
                str_ = strfmt(c"%s%s".as_ptr(), pfx.as_mut_ptr(), name);
            }
        }
        _ => {}
    }

    str_
}

unsafe fn disas_alt_init(
    dalt: *mut disas_alt,
    orig_insn: *mut instruction,
    alt: *mut alternative,
) -> c_int {
    (*dalt).orig_insn = orig_insn;
    (*dalt).alt = alt;
    (*dalt).insn_idx = 0;
    (*dalt).name = if !alt.is_null() {
        disas_alt_name(alt)
    } else {
        strdup(c"DEFAULT".as_ptr())
    };
    if (*dalt).name.is_null() {
        return -1;
    }
    (*dalt).width = strlen((*dalt).name) as c_int;

    0
}

unsafe fn disas_alt_add_insn(
    dalt: *mut disas_alt,
    index: c_int,
    insn_str: *mut c_char,
    offset: c_int,
    nops: c_int,
) -> c_int {
    if index >= DISAS_ALT_INSN_MAX as c_int {
        WARN(
            c"Alternative %lx.%s has more instructions than supported".as_ptr(),
            DALT_ALTID(dalt),
            (*dalt).name,
        );
        return -1;
    }

    let len = strlen(insn_str) as c_int;
    (*dalt).insn[index as usize].str_ = insn_str;
    (*dalt).insn[index as usize].offset = offset;
    (*dalt).insn[index as usize].nops = nops;
    if len > (*dalt).width {
        (*dalt).width = len;
    }

    0
}

unsafe fn disas_alt_jump(dalt: *mut disas_alt) -> c_int {
    let mut suffix = [0 as c_char; 2];

    let orig_insn = (*dalt).orig_insn;
    let dest_insn = (*(*dalt).alt).insn;

    let (str_, nops) = if (*orig_insn).type_ == INSN_NOP {
        if (*orig_insn).len == 5 {
            suffix[0] = b'q' as c_char;
        }
        (
            strfmt(
                c"jmp%-3s %lx <%s+0x%lx>".as_ptr(),
                suffix.as_mut_ptr(),
                (*dest_insn).offset,
                (*insn_sym(dest_insn)).name,
                (*dest_insn).offset - (*insn_sym(dest_insn)).offset,
            ),
            0,
        )
    } else {
        (strfmt(c"nop%d".as_ptr(), (*orig_insn).len), (*orig_insn).len)
    };

    if str_.is_null() {
        return -1;
    }

    disas_alt_add_insn(dalt, 0, str_, 0, nops);

    1
}

/*
 * Disassemble an exception table alternative.
 */
unsafe fn disas_alt_extable(dalt: *mut disas_alt) -> c_int {
    let alt_insn = (*(*dalt).alt).insn;
    let str_ = strfmt(
        c"resume at 0x%lx <%s+0x%lx>".as_ptr(),
        (*alt_insn).offset,
        (*insn_sym(alt_insn)).name,
        (*alt_insn).offset - (*insn_sym(alt_insn)).offset,
    );
    if str_.is_null() {
        return -1;
    }

    disas_alt_add_insn(dalt, 0, str_, 0, 0);

    1
}

/*
 * Disassemble an alternative and store instructions in the disas_alt
 * structure. Return the number of instructions in the alternative.
 */
unsafe fn disas_alt_group(dctx: *mut disas_context, dalt: *mut disas_alt) -> c_int {
    let file = (*dctx).file;
    let mut count = 0;
    let mut offset = 0;
    let mut insn = (*DALT_GROUP(dalt)).first_insn;

    while !insn.is_null() {
        disas_insn_alt(dctx, insn);
        let str_ = strdup(disas_result(dctx));
        if str_.is_null() {
            return -1;
        }

        let nops = if (*insn).type_ == INSN_NOP { (*insn).len } else { 0 };
        let err = disas_alt_add_insn(dalt, count, str_, offset, nops);
        if err != 0 {
            break;
        }
        offset += (*insn).len;
        count += 1;
        insn = next_insn_same_alt(file, DALT_GROUP(dalt), insn);
    }

    count
}

/*
 * Disassemble the default alternative.
 */
unsafe fn disas_alt_default(dctx: *mut disas_context, dalt: *mut disas_alt) -> c_int {
    if !DALT_GROUP(dalt).is_null() {
        return disas_alt_group(dctx, dalt);
    }

    /*
     * Default alternative with no alt_group: this is the default
     * code associated with either a jump table or an exception
     * table and no other instruction alternatives. In that case
     * the default alternative is made of a single instruction.
     */
    disas_insn(dctx, (*dalt).orig_insn);
    let str_ = strdup(disas_result(dctx));
    if str_.is_null() {
        return -1;
    }
    let nops = if (*(*dalt).orig_insn).type_ == INSN_NOP {
        (*(*dalt).orig_insn).len
    } else {
        0
    };
    let err = disas_alt_add_insn(dalt, 0, str_, 0, nops);
    if err != 0 {
        return -1;
    }

    1
}

/*
 * For each alternative, if there is an instruction at the specified
 * offset then print this instruction, otherwise print a blank entry.
 * The offset is an offset from the start of the alternative.
 *
 * Return the offset for the next instructions to print, or -1 if all
 * instructions have been printed.
 */
unsafe fn disas_alt_print_insn(
    dalts: *mut disas_alt,
    alt_count: c_int,
    insn_count: c_int,
    offset: c_int,
) -> c_int {
    let mut offset_next = -1;

    for i in 0..alt_count {
        let dalt = dalts.add(i as usize);
        let mut j = (*dalt).insn_idx;
        if j == -1 {
            printf(c"| %-*s ".as_ptr(), (*dalt).width, c"".as_ptr());
            continue;
        }

        if (*dalt).insn[j as usize].offset == offset {
            let str_ = (*dalt).insn[j as usize].str_;
            printf(
                c"| %-*s ".as_ptr(),
                (*dalt).width,
                if !str_.is_null() { str_ } else { c"".as_ptr() as *mut c_char },
            );
            j += 1;
            if j < insn_count {
                (*dalt).insn_idx = j;
            } else {
                (*dalt).insn_idx = -1;
                continue;
            }
        } else {
            printf(c"| %-*s ".as_ptr(), (*dalt).width, c"".as_ptr());
        }

        if (*dalt).insn[j as usize].offset > 0
            && (offset_next == -1 || (*dalt).insn[j as usize].offset < offset_next)
        {
            offset_next = (*dalt).insn[j as usize].offset;
        }
    }
    printf(c"\n".as_ptr());

    offset_next
}

/*
 * Print all alternatives side-by-side.
 */
unsafe fn disas_alt_print_wide(
    alt_name: *mut c_char,
    dalts: *mut disas_alt,
    alt_count: c_int,
    insn_count: c_int,
) {
    let orig_insn = (*dalts).orig_insn;

    /*
     * Print an header with the name of each alternative.
     */
    disas_print_info(stdout, orig_insn, -2, ptr::null());

    if strlen(alt_name) as c_int > (*dalts).width {
        (*dalts).width = strlen(alt_name) as c_int;
    }
    printf(c"| %-*s ".as_ptr(), (*dalts).width, alt_name);

    for i in 1..alt_count {
        printf(
            c"| %-*s ".as_ptr(),
            (*dalts.add(i as usize)).width,
            (*dalts.add(i as usize)).name,
        );
    }

    printf(c"\n".as_ptr());

    /*
     * Print instructions for each alternative.
     */
    let mut offset_next = 0;
    loop {
        let offset = offset_next;
        disas_print(
            stdout,
            (*orig_insn).sec,
            (*orig_insn).offset + offset as c_ulong,
            -2,
            ptr::null(),
        );
        offset_next = disas_alt_print_insn(dalts, alt_count, insn_count, offset);
        if offset_next <= offset {
            break;
        }
    }
}

/*
 * Print all alternatives one above the other.
 */
unsafe fn disas_alt_print_compact(
    alt_name: *mut c_char,
    dalts: *mut disas_alt,
    alt_count: c_int,
    insn_count: c_int,
) {
    let orig_insn = (*dalts).orig_insn;

    let len = disas_print(stdout, (*orig_insn).sec, (*orig_insn).offset, 0, ptr::null());
    printf(c"%s\n".as_ptr(), alt_name);

    /*
     * If all alternatives have a single instruction then print each
     * alternative on a single line. Otherwise, print alternatives
     * one above the other with a clear separation.
     */

    if insn_count == 1 {
        let mut width = 0;
        for i in 0..alt_count {
            if (*dalts.add(i as usize)).width > width {
                width = (*dalts.add(i as usize)).width;
            }
        }

        for i in 0..alt_count {
            printf(
                c"%*s= %-*s    (if %s)\n".as_ptr(),
                len,
                c"".as_ptr(),
                width,
                (*dalts.add(i as usize)).insn[0].str_,
                (*dalts.add(i as usize)).name,
            );
        }

        return;
    }

    for i in 0..alt_count {
        printf(c"%*s= %s\n".as_ptr(), len, c"".as_ptr(), (*dalts.add(i as usize)).name);
        for j in 0..insn_count {
            if (*dalts.add(i as usize)).insn[j as usize].str_.is_null() {
                break;
            }
            disas_print(
                stdout,
                (*orig_insn).sec,
                (*orig_insn).offset + (*dalts.add(i as usize)).insn[j as usize].offset as c_ulong,
                0,
                c"| %s\n".as_ptr(),
                (*dalts.add(i as usize)).insn[j as usize].str_,
            );
        }
        printf(c"%*s|\n".as_ptr(), len, c"".as_ptr());
    }
}

/*
 * Trim NOPs in alternatives. This replaces trailing NOPs in alternatives
 * with a single indication of the number of bytes covered with NOPs.
 *
 * Return the maximum numbers of instructions in all alternatives after
 * trailing NOPs have been trimmed.
 */
unsafe fn disas_alt_trim_nops(
    dalts: *mut disas_alt,
    alt_count: c_int,
    insn_count: c_int,
) -> c_int {
    let mut count = 0;
    for i in 0..alt_count {
        let mut offset = 0;
        let mut nops = 0;
        let mut nops_count = 0;
        let dalt = dalts.add(i as usize);
        let mut j = insn_count - 1;
        while j >= 0 {
            if (*dalt).insn[j as usize].str_.is_null() || (*dalt).insn[j as usize].nops == 0 {
                break;
            }
            offset = (*dalt).insn[j as usize].offset;
            free((*dalt).insn[j as usize].str_ as *mut c_void);
            (*dalt).insn[j as usize].offset = 0;
            (*dalt).insn[j as usize].str_ = ptr::null_mut();
            nops += (*dalt).insn[j as usize].nops;
            nops_count += 1;
            j -= 1;
        }

        /*
         * All trailing NOPs have been removed. If there was a single
         * NOP instruction then re-add it. If there was a block of
         * NOPs then indicate the number of bytes than the block
         * covers (nop*<number-of-bytes>).
         */
        if nops_count != 0 {
            let s = if nops_count == 1 { c"".as_ptr() } else { c"*".as_ptr() };
            (*dalt).insn[(j + 1) as usize].str_ = strfmt(c"nop%s%d".as_ptr(), s, nops);
            (*dalt).insn[(j + 1) as usize].offset = offset;
            (*dalt).insn[(j + 1) as usize].nops = nops;
            j += 1;
        }

        if j > count {
            count = j;
        }
    }

    count + 1
}

/*
 * Disassemble an alternative.
 *
 * Return the last instruction in the default alternative so that
 * disassembly can continue with the next instruction. Return NULL
 * on error.
 */
unsafe fn disas_alt(dctx: *mut disas_context, orig_insn: *mut instruction) -> *mut c_void {
    let mut dalts: [disas_alt; DISAS_ALT_MAX] = mem::zeroed();
    let mut last_insn: *mut instruction = ptr::null_mut();
    let mut insn_count = 0;
    let mut alt_count = 0;

    let alt_name = strfmt(
        c"<%s.%lx>".as_ptr(),
        disas_alt_type_name(orig_insn),
        (*orig_insn).offset,
    );
    if alt_name.is_null() {
        WARN(
            c"Failed to define name for alternative at instruction 0x%lx".as_ptr(),
            (*orig_insn).offset,
        );
        return ptr::null_mut();
    }

    /*
     * Initialize and disassemble the default alternative.
     */
    let mut err = disas_alt_init(&mut dalts[0], orig_insn, ptr::null_mut());
    if err != 0 {
        WARN(c"%s: failed to initialize default alternative".as_ptr(), alt_name);
        free(alt_name as *mut c_void);
        return ptr::null_mut();
    }

    insn_count = disas_alt_default(dctx, &mut dalts[0]);
    if insn_count < 0 {
        WARN(c"%s: failed to disassemble default alternative".as_ptr(), alt_name);
        free(dalts[0].name as *mut c_void);
        free(alt_name as *mut c_void);
        return ptr::null_mut();
    }

    /*
     * Initialize and disassemble all other alternatives.
     */
    let mut i = 1;
    let mut alt = (*orig_insn).alts;
    while !alt.is_null() {
        if i >= DISAS_ALT_MAX as c_int {
            WARN(c"%s has more alternatives than supported".as_ptr(), alt_name);
            break;
        }

        let dalt = &mut dalts[i as usize] as *mut disas_alt;
        err = disas_alt_init(dalt, orig_insn, alt);
        if err != 0 {
            WARN(c"%s: failed to disassemble alternative".as_ptr(), alt_name);
            break;
        }

        let mut count = -1;
        match (*(*dalt).alt).type_ {
            ALT_TYPE_INSTRUCTIONS => count = disas_alt_group(dctx, dalt),
            ALT_TYPE_EX_TABLE => count = disas_alt_extable(dalt),
            ALT_TYPE_JUMP_TABLE => count = disas_alt_jump(dalt),
            _ => {}
        }
        if count < 0 {
            WARN(c"%s: failed to disassemble alternative %s".as_ptr(), alt_name, (*dalt).name);
            break;
        }

        insn_count = if count > insn_count { count } else { insn_count };
        i += 1;
        alt = (*alt).next;
    }
    alt_count = i;

    /*
     * Print default and non-default alternatives.
     */

    insn_count = disas_alt_trim_nops(dalts.as_mut_ptr(), alt_count, insn_count);

    if opts.wide {
        disas_alt_print_wide(alt_name, dalts.as_mut_ptr(), alt_count, insn_count);
    } else {
        disas_alt_print_compact(alt_name, dalts.as_mut_ptr(), alt_count, insn_count);
    }

    last_insn = if !(*orig_insn).alt_group.is_null() {
        (*(*orig_insn).alt_group).last_insn
    } else {
        orig_insn
    };

    for i in 0..alt_count {
        free(dalts[i as usize].name as *mut c_void);
        for j in 0..insn_count {
            free(dalts[i as usize].insn[j as usize].str_ as *mut c_void);
        }
    }

    free(alt_name as *mut c_void);

    last_insn as *mut c_void
}

/*
 * Disassemble a function.
 */
unsafe fn disas_func(dctx: *mut disas_context, func: *mut symbol) {
    printf(c"%s:\n".as_ptr(), (*func).name);
    let mut insn = first_insn_for_sym((*dctx).file, func);
    while !insn.is_null() {
        if !(*insn).alts.is_null() {
            let insn_start = insn;
            insn = disas_alt(dctx, insn) as *mut instruction;
            if !insn.is_null() {
                insn = next_insn_for_sym((*dctx).file, func, insn);
                continue;
            }
            /*
             * There was an error with disassembling
             * the alternative. Resume disassembling
             * at the current instruction, this will
             * disassemble the default alternative
             * only and continue with the code after
             * the alternative.
             */
            insn = insn_start;
        }

        disas_print_insn(stdout, dctx, insn, 0, c"\n".as_ptr());
        insn = next_insn_for_sym((*dctx).file, func, insn);
    }
    printf(c"\n".as_ptr());
}

/*
 * Disassemble all warned functions.
 */
#[no_mangle]
pub unsafe extern "C" fn disas_warned_funcs(dctx: *mut disas_context) {
    if dctx.is_null() {
        return;
    }

    let mut sym = first_symbol((*(*dctx).file).elf);
    while !sym.is_null() {
        if (*sym).warned {
            disas_func(dctx, sym);
        }
        sym = next_symbol((*(*dctx).file).elf, sym);
    }
}

#[no_mangle]
pub unsafe extern "C" fn disas_funcs(dctx: *mut disas_context) {
    let disas_all = strcmp(opts.disas, c"*".as_ptr()) == 0;

    let mut sec = first_section((*(*dctx).file).elf);
    while !sec.is_null() {
        if !is_text_sec(sec) {
            sec = next_section((*(*dctx).file).elf, sec);
            continue;
        }

        let mut sym = first_symbol_for_sec(sec);
        while !sym.is_null() {
            /*
             * If the function had a warning and the verbose
             * option is used then the function was already
             * disassemble.
             */
            if opts.verbose && (*sym).warned {
                sym = next_symbol_for_sec(sec, sym);
                continue;
            }

            if disas_all || fnmatch(opts.disas, (*sym).name, 0) == 0 {
                disas_func(dctx, sym);
            }
            sym = next_symbol_for_sec(sec, sym);
        }
        sec = next_section((*(*dctx).file).elf, sec);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
