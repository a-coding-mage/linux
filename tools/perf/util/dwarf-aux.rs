// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * dwarf-aux.c : libdw auxiliary interfaces
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ptrdiff_t = isize;
type bool_ = bool;
type u64 = u64;
type s64 = i64;
type Dwarf_Addr = u64;
type Dwarf_Word = u64;
type Dwarf_Off = u64;

#[repr(C)]
pub struct Dwarf;
#[repr(C)]
pub struct Dwarf_Files;
#[repr(C)]
pub struct Dwarf_Lines;
#[repr(C)]
pub struct Dwarf_Line;
#[repr(C)]
pub struct Dwarf_CFI;
#[repr(C)]
pub struct Dwarf_Frame;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dwarf_Die {
    pub addr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dwarf_Attribute {
    pub cu: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dwarf_Op {
    pub atom: c_uint,
    pub number: Dwarf_Word,
    pub number2: Dwarf_Word,
}

#[repr(C)]
pub struct strbuf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct die_var_type {
    pub die_off: Dwarf_Off,
    pub addr: Dwarf_Addr,
    pub end: Dwarf_Addr,
    pub has_range: bool,
    pub reg: c_int,
    pub offset: c_int,
    pub is_reg_var_addr: bool,
    pub next: *mut die_var_type,
}

type line_walk_callback_t =
    Option<unsafe extern "C" fn(*const c_char, c_int, Dwarf_Addr, *mut c_void) -> c_int>;
type die_find_callback_t = Option<unsafe extern "C" fn(*mut Dwarf_Die, *mut c_void) -> c_int>;
type dwarf_func_callback_t = Option<unsafe extern "C" fn(*mut Dwarf_Die, *mut c_void) -> c_int>;

const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENOTSUP: c_int = 95;

const DIE_FIND_CB_END: c_int = 0;
const DIE_FIND_CB_CONTINUE: c_int = 3;
const DIE_FIND_CB_CHILD: c_int = 1;
const DIE_FIND_CB_SIBLING: c_int = 2;
const DWARF_CB_OK: c_int = 0;
const DWARF_CB_ABORT: c_int = 1;

const DWARF_REG_FB: c_int = -1;

extern "C" {
    static DW_AT_comp_dir: c_uint;
    static DW_AT_linkage_name: c_uint;
    static DW_AT_call_line: c_uint;
    static DW_AT_type: c_uint;
    static DW_AT_encoding: c_uint;
    static DW_AT_declaration: c_uint;
    static DW_AT_inline: c_uint;
    static DW_AT_ranges: c_uint;
    static DW_AT_data_member_location: c_uint;
    static DW_AT_call_file: c_uint;
    static DW_AT_decl_file: c_uint;
    static DW_AT_abstract_origin: c_uint;
    static DW_AT_external: c_uint;
    static DW_AT_location: c_uint;
    static DW_AT_const_value: c_uint;
    static DW_AT_data_bit_offset: c_uint;

    static DW_TAG_const_type: c_int;
    static DW_TAG_restrict_type: c_int;
    static DW_TAG_volatile_type: c_int;
    static DW_TAG_shared_type: c_int;
    static DW_TAG_typedef: c_int;
    static DW_TAG_pointer_type: c_int;
    static DW_TAG_array_type: c_int;
    static DW_TAG_subprogram: c_int;
    static DW_TAG_inlined_subroutine: c_int;
    static DW_TAG_formal_parameter: c_int;
    static DW_TAG_variable: c_int;
    static DW_TAG_member: c_int;
    static DW_TAG_union_type: c_int;
    static DW_TAG_structure_type: c_int;
    static DW_TAG_enumeration_type: c_int;
    static DW_TAG_subroutine_type: c_int;
    static DW_TAG_compile_unit: c_int;
    static DW_TAG_namespace: c_int;

    static DW_ATE_signed_char: Dwarf_Word;
    static DW_ATE_signed: Dwarf_Word;
    static DW_ATE_signed_fixed: Dwarf_Word;

    static DW_OP_reg0: c_uint;
    static DW_OP_reg31: c_uint;
    static DW_OP_breg0: c_uint;
    static DW_OP_breg31: c_uint;
    static DW_OP_regx: c_uint;
    static DW_OP_bregx: c_uint;
    static DW_OP_fbreg: c_uint;
    static DW_OP_plus_uconst: c_uint;
    static DW_OP_stack_value: c_uint;
    static DW_OP_deref_size: c_uint;
    static DW_OP_deref: c_uint;
    static DW_OP_piece: c_uint;
    static DW_OP_addr: c_uint;
    static DW_FORM_sec_offset: c_uint;

    fn dwarf_getsrcfiles(cu_die: *mut Dwarf_Die, files: *mut *mut Dwarf_Files, nfiles: *mut size_t) -> c_int;
    fn dwarf_filesrc(files: *mut Dwarf_Files, idx: size_t, mtime: *mut c_void, length: *mut c_void) -> *const c_char;
    fn strtailcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn dwarf_attr(die: *mut Dwarf_Die, attr: c_uint, result: *mut Dwarf_Attribute) -> *mut Dwarf_Attribute;
    fn dwarf_attr_integrate(die: *mut Dwarf_Die, attr: c_uint, result: *mut Dwarf_Attribute) -> *mut Dwarf_Attribute;
    fn dwarf_formstring(attr: *mut Dwarf_Attribute) -> *const c_char;
    fn dwarf_getsrclines(cu_die: *mut Dwarf_Die, lines: *mut *mut Dwarf_Lines, nlines: *mut size_t) -> c_int;
    fn dwarf_onesrcline(lines: *mut Dwarf_Lines, idx: size_t) -> *mut Dwarf_Line;
    fn dwarf_lineaddr(line: *mut Dwarf_Line, addr: *mut Dwarf_Addr) -> c_int;
    fn dwarf_linebeginstatement(line: *mut Dwarf_Line, flag: *mut bool) -> c_int;
    fn dwarf_lineno(line: *mut Dwarf_Line, lineno: *mut c_int) -> c_int;
    fn dwarf_linesrc(line: *mut Dwarf_Line, mtime: *mut c_void, length: *mut c_void) -> *const c_char;
    fn dwarf_decl_line(die: *mut Dwarf_Die, lineno: *mut c_int) -> c_int;
    fn dwarf_diename(die: *mut Dwarf_Die) -> *const c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strglobmatch(str_: *const c_char, pat: *const c_char) -> bool;
    fn dwarf_formudata(attr: *mut Dwarf_Attribute, return_uval: *mut Dwarf_Word) -> c_int;
    fn dwarf_formref_die(attr: *mut Dwarf_Attribute, die: *mut Dwarf_Die) -> *mut Dwarf_Die;
    fn dwarf_tag(die: *mut Dwarf_Die) -> c_int;
    fn dwarf_entrypc(die: *mut Dwarf_Die, addr: *mut Dwarf_Addr) -> c_int;
    fn dwarf_ranges(die: *mut Dwarf_Die, offset: size_t, basep: *mut Dwarf_Addr, startp: *mut Dwarf_Addr, endp: *mut Dwarf_Addr) -> ptrdiff_t;
    fn dwarf_highpc(die: *mut Dwarf_Die, addr: *mut Dwarf_Addr) -> c_int;
    fn dwarf_haspc(die: *mut Dwarf_Die, pc: Dwarf_Addr) -> bool;
    fn dwarf_getlocation(attr: *mut Dwarf_Attribute, expr: *mut *mut Dwarf_Op, nexpr: *mut size_t) -> c_int;
    fn dwarf_cu_die(cu: *mut c_void, result: *mut Dwarf_Die, a: *mut c_void, b: *mut c_void, c: *mut c_void, d: *mut c_void, e: *mut c_void, f: *mut c_void) -> *mut Dwarf_Die;
    fn dwarf_child(die: *mut Dwarf_Die, result: *mut Dwarf_Die) -> c_int;
    fn dwarf_siblingof(die: *mut Dwarf_Die, result: *mut Dwarf_Die) -> c_int;
    fn dwarf_getfuncs(cu_die: *mut Dwarf_Die, callback: dwarf_func_callback_t, data: *mut c_void, offset: ptrdiff_t) -> ptrdiff_t;
    fn dwarf_diecu(die: *mut Dwarf_Die, result: *mut Dwarf_Die, a: *mut c_void, b: *mut c_void) -> *mut Dwarf_Die;
    fn dwarf_lineendsequence(line: *mut Dwarf_Line, flag: *mut bool) -> c_int;
    fn dwarf_lineprologueend(line: *mut Dwarf_Line, flag: *mut bool) -> c_int;
    fn strbuf_add(buf: *mut strbuf, data: *const c_void, len: size_t) -> c_int;
    fn strbuf_addf(buf: *mut strbuf, fmt: *const c_char, ...) -> c_int;
    fn strbuf_addstr(buf: *mut strbuf, s: *const c_char) -> c_int;
    fn dwarf_getscopes_die(die: *mut Dwarf_Die, scopes: *mut *mut Dwarf_Die) -> c_int;
    fn free(ptr: *mut c_void);
    fn dwarf_getlocations(attr: *mut Dwarf_Attribute, offset: ptrdiff_t, basep: *mut Dwarf_Addr, startp: *mut Dwarf_Addr, endp: *mut Dwarf_Addr, expr: *mut *mut Dwarf_Op, nexpr: *mut size_t) -> ptrdiff_t;
    fn dwarf_aggregate_size(die: *mut Dwarf_Die, size: *mut Dwarf_Word) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn dwarf_dieoffset(die: *mut Dwarf_Die) -> Dwarf_Off;
    fn dwarf_getcfi(dwarf: *mut Dwarf) -> *mut Dwarf_CFI;
    fn dwarf_cfi_addrframe(cfi: *mut Dwarf_CFI, pc: Dwarf_Addr, frame: *mut *mut Dwarf_Frame) -> c_int;
    fn dwarf_frame_cfa(frame: *mut Dwarf_Frame, ops: *mut *mut Dwarf_Op, nops: *mut size_t) -> c_int;
    fn dwarf_whatform(attr: *mut Dwarf_Attribute) -> c_uint;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
}

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

/**
 * cu_find_realpath - Find the realpath of the target file
 * @cu_die: A DIE(dwarf information entry) of CU(compilation Unit)
 * @fname:  The tail filename of the target file
 *
 * Find the real(long) path of @fname in @cu_die.
 */
#[no_mangle]
pub unsafe extern "C" fn cu_find_realpath(cu_die: *mut Dwarf_Die, fname: *const c_char) -> *const c_char {
    let mut files: *mut Dwarf_Files = ptr::null_mut();
    let mut nfiles: size_t = 0;
    let mut i: size_t;
    let mut src: *const c_char = ptr::null();
    let ret: c_int;

    if fname.is_null() {
        return ptr::null();
    }

    ret = dwarf_getsrcfiles(cu_die, &mut files, &mut nfiles);
    if ret != 0 {
        return ptr::null();
    }

    i = 0;
    while i < nfiles {
        src = dwarf_filesrc(files, i, ptr::null_mut(), ptr::null_mut());
        if strtailcmp(src, fname) == 0 {
            break;
        }
        i += 1;
    }
    if i == nfiles {
        return ptr::null();
    }
    src
}

/**
 * cu_get_comp_dir - Get the path of compilation directory
 * @cu_die: a CU DIE
 *
 * Get the path of compilation directory of given @cu_die.
 * Since this depends on DW_AT_comp_dir, older gcc will not
 * embedded it. In that case, this returns NULL.
 */
#[no_mangle]
pub unsafe extern "C" fn cu_get_comp_dir(cu_die: *mut Dwarf_Die) -> *const c_char {
    let mut attr: Dwarf_Attribute = mem::zeroed();
    if dwarf_attr(cu_die, DW_AT_comp_dir, &mut attr).is_null() {
        return ptr::null();
    }
    dwarf_formstring(&mut attr)
}

/* Unlike dwarf_getsrc_die(), cu_getsrc_die() only returns statement line */
unsafe extern "C" fn cu_getsrc_die(cu_die: *mut Dwarf_Die, addr: Dwarf_Addr) -> *mut Dwarf_Line {
    let mut laddr: Dwarf_Addr = 0;
    let mut lines: *mut Dwarf_Lines = ptr::null_mut();
    let mut line: *mut Dwarf_Line;
    let mut nlines: size_t = 0;
    let mut l: size_t;
    let mut u: size_t;
    let mut n: size_t;
    let mut flag: bool = false;

    if dwarf_getsrclines(cu_die, &mut lines, &mut nlines) != 0 || nlines == 0 {
        return ptr::null_mut();
    }

    /* Lines are sorted by address, use binary search */
    l = 0;
    u = nlines - 1;
    while l < u {
        n = u - (u - l) / 2;
        line = dwarf_onesrcline(lines, n);
        if line.is_null() || dwarf_lineaddr(line, &mut laddr) != 0 {
            return ptr::null_mut();
        }
        if addr < laddr {
            u = n - 1;
        } else {
            l = n;
        }
    }
    /* Going backward to find the lowest line */
    loop {
        l -= 1;
        line = dwarf_onesrcline(lines, l);
        if line.is_null() || dwarf_lineaddr(line, &mut laddr) != 0 {
            return ptr::null_mut();
        }
        if laddr != addr {
            break;
        }
    }
    l += 1;
    /* Going forward to find the statement line */
    loop {
        line = dwarf_onesrcline(lines, l);
        l += 1;
        if line.is_null() || dwarf_lineaddr(line, &mut laddr) != 0 || dwarf_linebeginstatement(line, &mut flag) != 0 {
            return ptr::null_mut();
        }
        if laddr > addr {
            return ptr::null_mut();
        }
        if flag {
            break;
        }
    }

    line
}

/**
 * cu_find_lineinfo - Get a line number and file name for given address
 * @cu_die: a CU DIE
 * @addr: An address
 * @fname: a pointer which returns the file name string
 * @lineno: a pointer which returns the line number
 *
 * Find a line number and file name for @addr in @cu_die.
 */
#[no_mangle]
pub unsafe extern "C" fn cu_find_lineinfo(cu_die: *mut Dwarf_Die, addr: Dwarf_Addr, fname: *mut *const c_char, lineno: *mut c_int) -> c_int {
    let mut line: *mut Dwarf_Line;
    let mut die_mem: Dwarf_Die = mem::zeroed();
    let mut faddr: Dwarf_Addr = 0;

    if !die_find_realfunc(cu_die, addr, &mut die_mem).is_null()
        && die_entrypc(&mut die_mem, &mut faddr) == 0
        && faddr == addr
    {
        *fname = die_get_decl_file(&mut die_mem);
        if dwarf_decl_line(&mut die_mem, lineno) != 0 {
            return -ENOENT;
        }
    } else {
        line = cu_getsrc_die(cu_die, addr);
        if !line.is_null() && dwarf_lineno(line, lineno) == 0 {
            *fname = dwarf_linesrc(line, ptr::null_mut(), ptr::null_mut());
            if (*fname).is_null() {
                /* line number is useless without filename */
                *lineno = 0;
            }
        }
    }

    if *lineno != 0 && !(*fname).is_null() { *lineno } else { -ENOENT }
}

/**
 * cu_walk_functions_at - Walk on function DIEs at given address
 * @cu_die: A CU DIE
 * @addr: An address
 * @callback: A callback which called with found DIEs
 * @data: A user data
 *
 * Walk on function DIEs at given @addr in @cu_die. Passed DIEs
 * should be subprogram or inlined-subroutines.
 */
#[no_mangle]
pub unsafe extern "C" fn cu_walk_functions_at(cu_die: *mut Dwarf_Die, addr: Dwarf_Addr, callback: die_find_callback_t, data: *mut c_void) -> c_int {
    let mut die_mem: Dwarf_Die = mem::zeroed();
    let mut sc_die: *mut Dwarf_Die;
    let mut ret: c_int = -ENOENT;

    /* Inlined function could be recursive. Trace it until fail */
    sc_die = die_find_realfunc(cu_die, addr, &mut die_mem);
    while !sc_die.is_null() {
        ret = callback.unwrap()(sc_die, data);
        if ret != 0 {
            break;
        }
        sc_die = die_find_child(sc_die, Some(__die_find_inline_cb), &addr as *const _ as *mut c_void, &mut die_mem);
    }

    ret
}

/**
 * die_get_linkage_name - Get the linkage name of the object
 * @dw_die: A DIE of the object
 *
 * Get the linkage name attribute of given @dw_die.
 * For C++ binary, the linkage name will be the mangled symbol.
 */
#[no_mangle]
pub unsafe extern "C" fn die_get_linkage_name(dw_die: *mut Dwarf_Die) -> *const c_char {
    let mut attr: Dwarf_Attribute = mem::zeroed();

    if dwarf_attr_integrate(dw_die, DW_AT_linkage_name, &mut attr).is_null() {
        return ptr::null();
    }
    dwarf_formstring(&mut attr)
}

/**
 * die_compare_name - Compare diename and tname
 * @dw_die: a DIE
 * @tname: a string of target name
 *
 * Compare the name of @dw_die and @tname. Return false if @dw_die has no name.
 */
#[no_mangle]
pub unsafe extern "C" fn die_compare_name(dw_die: *mut Dwarf_Die, tname: *const c_char) -> bool {
    let name = dwarf_diename(dw_die);
    if !name.is_null() { strcmp(tname, name) == 0 } else { false }
}

/**
 * die_match_name - Match diename/linkage name and glob
 * @dw_die: a DIE
 * @glob: a string of target glob pattern
 *
 * Glob matching the name of @dw_die and @glob. Return false if matching fail.
 * This also match linkage name.
 */
#[no_mangle]
pub unsafe extern "C" fn die_match_name(dw_die: *mut Dwarf_Die, glob: *const c_char) -> bool {
    let mut name: *const c_char;

    name = dwarf_diename(dw_die);
    if !name.is_null() && strglobmatch(name, glob) {
        return true;
    }
    /* fall back to check linkage name */
    name = die_get_linkage_name(dw_die);
    if !name.is_null() && strglobmatch(name, glob) {
        return true;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn die_get_call_lineno(in_die: *mut Dwarf_Die) -> c_int {
    let mut attr: Dwarf_Attribute = mem::zeroed();
    let mut ret: Dwarf_Word = 0;

    if dwarf_attr(in_die, DW_AT_call_line, &mut attr).is_null() {
        return -ENOENT;
    }

    dwarf_formudata(&mut attr, &mut ret);
    ret as c_int
}

#[no_mangle]
pub unsafe extern "C" fn die_get_type(vr_die: *mut Dwarf_Die, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut attr: Dwarf_Attribute = mem::zeroed();

    if !dwarf_attr_integrate(vr_die, DW_AT_type, &mut attr).is_null() && !dwarf_formref_die(&mut attr, die_mem).is_null() {
        die_mem
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn __die_get_real_type(mut vr_die: *mut Dwarf_Die, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut tag: c_int;

    loop {
        vr_die = die_get_type(vr_die, die_mem);
        if vr_die.is_null() {
            break;
        }
        tag = dwarf_tag(vr_die);
        if !(tag == DW_TAG_const_type || tag == DW_TAG_restrict_type || tag == DW_TAG_volatile_type || tag == DW_TAG_shared_type) {
            break;
        }
    }

    vr_die
}

#[no_mangle]
pub unsafe extern "C" fn die_get_real_type(mut vr_die: *mut Dwarf_Die, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    loop {
        vr_die = __die_get_real_type(vr_die, die_mem);
        if vr_die.is_null() || dwarf_tag(vr_die) != DW_TAG_typedef {
            break;
        }
    }

    vr_die
}

#[no_mangle]
pub unsafe extern "C" fn die_get_pointer_type(mut type_die: *mut Dwarf_Die, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut tag: c_int;

    loop {
        tag = dwarf_tag(type_die);
        if tag == DW_TAG_pointer_type || tag == DW_TAG_array_type {
            return type_die;
        }
        if tag != DW_TAG_typedef && tag != DW_TAG_const_type && tag != DW_TAG_restrict_type && tag != DW_TAG_volatile_type && tag != DW_TAG_shared_type {
            return ptr::null_mut();
        }
        type_die = die_get_type(type_die, die_mem);
        if type_die.is_null() {
            break;
        }
    }

    ptr::null_mut()
}

unsafe extern "C" fn die_get_attr_udata(tp_die: *mut Dwarf_Die, attr_name: c_uint, result: *mut Dwarf_Word) -> c_int {
    let mut attr: Dwarf_Attribute = mem::zeroed();

    if dwarf_attr_integrate(tp_die, attr_name, &mut attr).is_null() || dwarf_formudata(&mut attr, result) != 0 {
        return -ENOENT;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn die_is_signed_type(tp_die: *mut Dwarf_Die) -> bool {
    let mut ret: Dwarf_Word = 0;

    if die_get_attr_udata(tp_die, DW_AT_encoding, &mut ret) != 0 {
        return false;
    }

    ret == DW_ATE_signed_char || ret == DW_ATE_signed || ret == DW_ATE_signed_fixed
}

#[no_mangle]
pub unsafe extern "C" fn die_is_func_def(dw_die: *mut Dwarf_Die) -> bool {
    let mut attr: Dwarf_Attribute = mem::zeroed();
    let mut addr: Dwarf_Addr = 0;

    if dwarf_tag(dw_die) != DW_TAG_subprogram {
        return false;
    }

    if !dwarf_attr(dw_die, DW_AT_declaration, &mut attr).is_null() {
        return false;
    }

    /*
     * DW_AT_declaration can be lost from function declaration
     * by gcc's bug #97060.
     * So we need to check this subprogram DIE has DW_AT_inline
     * or an entry address.
     */
    if dwarf_attr(dw_die, DW_AT_inline, &mut attr).is_null() && die_entrypc(dw_die, &mut addr) < 0 {
        return false;
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn die_entrypc(dw_die: *mut Dwarf_Die, addr: *mut Dwarf_Addr) -> c_int {
    let mut base: Dwarf_Addr = 0;
    let mut end: Dwarf_Addr = 0;
    let mut attr: Dwarf_Attribute = mem::zeroed();

    if addr.is_null() {
        return -EINVAL;
    }

    if dwarf_entrypc(dw_die, addr) == 0 {
        return 0;
    }

    /*
     *  Since the dwarf_ranges() will return 0 if there is no
     * DW_AT_ranges attribute, we should check it first.
     */
    if dwarf_attr(dw_die, DW_AT_ranges, &mut attr).is_null() {
        return -ENOENT;
    }

    if dwarf_ranges(dw_die, 0, &mut base, addr, &mut end) < 0 { -ENOENT } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn die_is_func_instance(dw_die: *mut Dwarf_Die) -> bool {
    let mut tmp: Dwarf_Addr = 0;
    let mut attr_mem: Dwarf_Attribute = mem::zeroed();
    let tag = dwarf_tag(dw_die);

    if tag != DW_TAG_subprogram && tag != DW_TAG_inlined_subroutine {
        return false;
    }

    dwarf_entrypc(dw_die, &mut tmp) == 0 || !dwarf_attr(dw_die, DW_AT_ranges, &mut attr_mem).is_null()
}

#[no_mangle]
pub unsafe extern "C" fn die_get_data_member_location(mb_die: *mut Dwarf_Die, offs: *mut Dwarf_Word) -> c_int {
    let mut attr: Dwarf_Attribute = mem::zeroed();
    let mut expr: *mut Dwarf_Op = ptr::null_mut();
    let mut nexpr: size_t = 0;
    let mut ret: c_int;

    if dwarf_attr_integrate(mb_die, DW_AT_data_member_location, &mut attr).is_null() {
        return -ENOENT;
    }

    if dwarf_formudata(&mut attr, offs) != 0 {
        /* DW_AT_data_member_location should be DW_OP_plus_uconst */
        ret = dwarf_getlocation(&mut attr, &mut expr, &mut nexpr);
        if ret < 0 || nexpr == 0 {
            return -ENOENT;
        }

        if (*expr).atom != DW_OP_plus_uconst || nexpr != 1 {
            pr_debug(cstr(b"Unable to get offset:Unexpected OP %x (%zd)\n\0"), (*expr).atom, nexpr);
            return -ENOTSUP;
        }
        *offs = (*expr).number as Dwarf_Word;
    }
    0
}

unsafe extern "C" fn die_get_call_fileno(in_die: *mut Dwarf_Die) -> c_int {
    let mut idx: Dwarf_Word = 0;
    if die_get_attr_udata(in_die, DW_AT_call_file, &mut idx) == 0 { idx as c_int } else { -ENOENT }
}

unsafe extern "C" fn die_get_decl_fileno(pdie: *mut Dwarf_Die) -> c_int {
    let mut idx: Dwarf_Word = 0;
    if die_get_attr_udata(pdie, DW_AT_decl_file, &mut idx) == 0 { idx as c_int } else { -ENOENT }
}

unsafe extern "C" fn die_get_file_name(dw_die: *mut Dwarf_Die, idx: c_int) -> *const c_char {
    let mut cu_die: Dwarf_Die = mem::zeroed();
    let mut files: *mut Dwarf_Files = ptr::null_mut();
    let mut attr_mem: Dwarf_Attribute = mem::zeroed();

    if idx < 0
        || dwarf_attr_integrate(dw_die, DW_AT_decl_file, &mut attr_mem).is_null()
        || dwarf_cu_die(attr_mem.cu, &mut cu_die, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut()).is_null()
        || dwarf_getsrcfiles(&mut cu_die, &mut files, ptr::null_mut()) != 0
    {
        return ptr::null();
    }

    dwarf_filesrc(files, idx as size_t, ptr::null_mut(), ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn die_get_call_file(in_die: *mut Dwarf_Die) -> *const c_char {
    die_get_file_name(in_die, die_get_call_fileno(in_die))
}

#[no_mangle]
pub unsafe extern "C" fn die_get_decl_file(dw_die: *mut Dwarf_Die) -> *const c_char {
    die_get_file_name(dw_die, die_get_decl_fileno(dw_die))
}

#[no_mangle]
pub unsafe extern "C" fn die_find_child(rt_die: *mut Dwarf_Die, callback: die_find_callback_t, data: *mut c_void, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut child_die: Dwarf_Die = mem::zeroed();
    let mut ret: c_int;

    ret = dwarf_child(rt_die, die_mem);
    if ret != 0 {
        return ptr::null_mut();
    }

    loop {
        ret = callback.unwrap()(die_mem, data);
        if ret == DIE_FIND_CB_END {
            return die_mem;
        }

        if (ret & DIE_FIND_CB_CHILD) != 0 && !die_find_child(die_mem, callback, data, &mut child_die).is_null() {
            ptr::copy_nonoverlapping(&child_die, die_mem, 1);
            return die_mem;
        }
        if !((ret & DIE_FIND_CB_SIBLING) != 0 && dwarf_siblingof(die_mem, die_mem) == 0) {
            break;
        }
    }

    ptr::null_mut()
}

#[repr(C)]
struct __addr_die_search_param {
    addr: Dwarf_Addr,
    die_mem: *mut Dwarf_Die,
}

unsafe extern "C" fn __die_search_func_tail_cb(fn_die: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let ad = data as *mut __addr_die_search_param;
    let mut addr: Dwarf_Addr = 0;

    if dwarf_tag(fn_die) == DW_TAG_subprogram && dwarf_highpc(fn_die, &mut addr) == 0 && addr == (*ad).addr {
        ptr::copy_nonoverlapping(fn_die, (*ad).die_mem, 1);
        return DWARF_CB_ABORT;
    }
    DWARF_CB_OK
}

#[no_mangle]
pub unsafe extern "C" fn die_find_tailfunc(cu_die: *mut Dwarf_Die, addr: Dwarf_Addr, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut ad = __addr_die_search_param { addr, die_mem };
    /* dwarf_getscopes can't find subprogram. */
    if dwarf_getfuncs(cu_die, Some(__die_search_func_tail_cb), &mut ad as *mut _ as *mut c_void, 0) <= 0 {
        ptr::null_mut()
    } else {
        die_mem
    }
}

unsafe extern "C" fn __die_search_func_cb(fn_die: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let ad = data as *mut __addr_die_search_param;

    /*
     * Since a declaration entry doesn't has given pc, this always returns
     * function definition entry.
     */
    if dwarf_tag(fn_die) == DW_TAG_subprogram && dwarf_haspc(fn_die, (*ad).addr) {
        ptr::copy_nonoverlapping(fn_die, (*ad).die_mem, 1);
        return DWARF_CB_ABORT;
    }
    DWARF_CB_OK
}

#[no_mangle]
pub unsafe extern "C" fn die_find_realfunc(cu_die: *mut Dwarf_Die, addr: Dwarf_Addr, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut ad = __addr_die_search_param { addr, die_mem };
    /* dwarf_getscopes can't find subprogram. */
    if dwarf_getfuncs(cu_die, Some(__die_search_func_cb), &mut ad as *mut _ as *mut c_void, 0) <= 0 {
        ptr::null_mut()
    } else {
        die_mem
    }
}

unsafe extern "C" fn __die_find_inline_cb(die_mem: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let addr = data as *mut Dwarf_Addr;

    if dwarf_tag(die_mem) == DW_TAG_inlined_subroutine && dwarf_haspc(die_mem, *addr) {
        return DIE_FIND_CB_END;
    }

    DIE_FIND_CB_CONTINUE
}

#[no_mangle]
pub unsafe extern "C" fn die_find_top_inlinefunc(sp_die: *mut Dwarf_Die, addr: Dwarf_Addr, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    die_find_child(sp_die, Some(__die_find_inline_cb), &addr as *const _ as *mut c_void, die_mem)
}

#[no_mangle]
pub unsafe extern "C" fn die_find_inlinefunc(mut sp_die: *mut Dwarf_Die, addr: Dwarf_Addr, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut tmp_die: Dwarf_Die = mem::zeroed();

    sp_die = die_find_child(sp_die, Some(__die_find_inline_cb), &addr as *const _ as *mut c_void, &mut tmp_die);
    if sp_die.is_null() {
        return ptr::null_mut();
    }

    /* Inlined function could be recursive. Trace it until fail */
    while !sp_die.is_null() {
        ptr::copy_nonoverlapping(sp_die, die_mem, 1);
        sp_die = die_find_child(sp_die, Some(__die_find_inline_cb), &addr as *const _ as *mut c_void, &mut tmp_die);
    }

    die_mem
}

unsafe extern "C" fn __die_find_func_rettype_cb(die_mem: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let func_name: *const c_char;

    if dwarf_tag(die_mem) != DW_TAG_subprogram {
        return DIE_FIND_CB_SIBLING;
    }

    func_name = dwarf_diename(die_mem);
    if !func_name.is_null() && strcmp(func_name, data as *const c_char) == 0 {
        return DIE_FIND_CB_END;
    }

    DIE_FIND_CB_SIBLING
}

#[no_mangle]
pub unsafe extern "C" fn die_find_func_rettype(mut cu_die: *mut Dwarf_Die, name: *const c_char, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut tmp_die: Dwarf_Die = mem::zeroed();

    cu_die = die_find_child(cu_die, Some(__die_find_func_rettype_cb), name as *mut c_void, &mut tmp_die);
    if cu_die.is_null() {
        return ptr::null_mut();
    }

    if die_get_real_type(&mut tmp_die, die_mem).is_null() {
        return ptr::null_mut();
    }

    die_mem
}

#[repr(C)]
struct __instance_walk_param {
    addr: *mut c_void,
    callback: die_find_callback_t,
    data: *mut c_void,
    retval: c_int,
}

unsafe extern "C" fn __die_walk_instances_cb(inst: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let iwp = data as *mut __instance_walk_param;
    let mut attr_mem: Dwarf_Attribute = mem::zeroed();
    let mut origin_mem: Dwarf_Die = mem::zeroed();
    let mut tmp: c_int = 0;

    if !die_is_func_instance(inst) {
        return DIE_FIND_CB_CONTINUE;
    }

    let attr = dwarf_attr(inst, DW_AT_abstract_origin, &mut attr_mem);
    if attr.is_null() {
        return DIE_FIND_CB_CONTINUE;
    }

    let origin = dwarf_formref_die(attr, &mut origin_mem);
    if origin.is_null() || (*origin).addr != (*iwp).addr {
        return DIE_FIND_CB_CONTINUE;
    }

    /* Ignore redundant instances */
    if dwarf_tag(inst) == DW_TAG_inlined_subroutine {
        if dwarf_decl_line(origin, &mut tmp) == 0 && die_get_call_lineno(inst) == tmp {
            tmp = die_get_decl_fileno(origin);
            if die_get_call_fileno(inst) == tmp {
                return DIE_FIND_CB_CONTINUE;
            }
        }
    }

    (*iwp).retval = (*iwp).callback.unwrap()(inst, (*iwp).data);

    if (*iwp).retval != 0 { DIE_FIND_CB_END } else { DIE_FIND_CB_CONTINUE }
}

#[no_mangle]
pub unsafe extern "C" fn die_walk_instances(or_die: *mut Dwarf_Die, callback: die_find_callback_t, data: *mut c_void) -> c_int {
    let mut cu_die: Dwarf_Die = mem::zeroed();
    let mut die_mem: Dwarf_Die = mem::zeroed();
    let mut iwp = __instance_walk_param {
        addr: (*or_die).addr,
        callback,
        data,
        retval: -ENOENT,
    };

    if dwarf_diecu(or_die, &mut cu_die, ptr::null_mut(), ptr::null_mut()).is_null() {
        return -ENOENT;
    }

    die_find_child(&mut cu_die, Some(__die_walk_instances_cb), &mut iwp as *mut _ as *mut c_void, &mut die_mem);

    iwp.retval
}

/* Line walker internal parameters */
#[repr(C)]
struct __line_walk_param {
    recursive: bool,
    callback: line_walk_callback_t,
    data: *mut c_void,
    retval: c_int,
}

unsafe extern "C" fn __die_walk_funclines_cb(in_die: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let lw = data as *mut __line_walk_param;
    let mut addr: Dwarf_Addr = 0;
    let mut fname: *const c_char;
    let mut lineno: c_int = 0;

    if dwarf_tag(in_die) == DW_TAG_inlined_subroutine {
        fname = die_get_call_file(in_die);
        lineno = die_get_call_lineno(in_die);
        if !fname.is_null() && lineno > 0 && die_entrypc(in_die, &mut addr) == 0 {
            (*lw).retval = (*lw).callback.unwrap()(fname, lineno, addr, (*lw).data);
            if (*lw).retval != 0 {
                return DIE_FIND_CB_END;
            }
        }
        if !(*lw).recursive {
            return DIE_FIND_CB_SIBLING;
        }
    }

    if addr != 0 {
        fname = die_get_decl_file(in_die);
        if !fname.is_null() && dwarf_decl_line(in_die, &mut lineno) == 0 {
            (*lw).retval = (*lw).callback.unwrap()(fname, lineno, addr, (*lw).data);
            if (*lw).retval != 0 {
                return DIE_FIND_CB_END;
            }
        }
    }

    /* Continue to search nested inlined function call-sites */
    DIE_FIND_CB_CONTINUE
}

unsafe extern "C" fn __die_walk_funclines(sp_die: *mut Dwarf_Die, recursive: bool, callback: line_walk_callback_t, data: *mut c_void) -> c_int {
    let mut lw = __line_walk_param { recursive, callback, data, retval: 0 };
    let mut die_mem: Dwarf_Die = mem::zeroed();
    let mut addr: Dwarf_Addr = 0;
    let mut lineno: c_int = 0;

    /* Handle function declaration line */
    let fname = die_get_decl_file(sp_die);
    if !fname.is_null() && dwarf_decl_line(sp_die, &mut lineno) == 0 && die_entrypc(sp_die, &mut addr) == 0 {
        lw.retval = callback.unwrap()(fname, lineno, addr, data);
        if lw.retval != 0 {
            return lw.retval;
        }
    }
    die_find_child(sp_die, Some(__die_walk_funclines_cb), &mut lw as *mut _ as *mut c_void, &mut die_mem);
    lw.retval
}

unsafe extern "C" fn __die_walk_culines_cb(sp_die: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let lw = data as *mut __line_walk_param;

    /*
     * Since inlined function can include another inlined function in
     * the same file, we need to walk in it recursively.
     */
    (*lw).retval = __die_walk_funclines(sp_die, true, (*lw).callback, (*lw).data);
    if (*lw).retval != 0 {
        return DWARF_CB_ABORT;
    }

    DWARF_CB_OK
}

#[no_mangle]
pub unsafe extern "C" fn die_walk_lines(rt_die: *mut Dwarf_Die, callback: line_walk_callback_t, data: *mut c_void) -> c_int {
    let mut lines: *mut Dwarf_Lines = ptr::null_mut();
    let mut line: *mut Dwarf_Line;
    let mut addr: Dwarf_Addr = 0;
    let mut fname: *const c_char;
    let mut decf: *const c_char = ptr::null();
    let mut inf: *const c_char = ptr::null();
    let mut lineno: c_int = 0;
    let mut ret: c_int = 0;
    let mut decl: c_int = 0;
    let mut inl: c_int = 0;
    let mut die_mem: Dwarf_Die = mem::zeroed();
    let mut cu_die: *mut Dwarf_Die;
    let mut nlines: size_t = 0;
    let mut i: size_t;
    let mut flag: bool = false;

    /* Get the CU die */
    if dwarf_tag(rt_die) != DW_TAG_compile_unit {
        cu_die = dwarf_diecu(rt_die, &mut die_mem, ptr::null_mut(), ptr::null_mut());
        dwarf_decl_line(rt_die, &mut decl);
        decf = die_get_decl_file(rt_die);
    } else {
        cu_die = rt_die;
    }
    if cu_die.is_null() {
        pr_debug2(cstr(b"Failed to get CU from given DIE.\n\0"));
        return -EINVAL;
    }

    /* Get lines list in the CU */
    if dwarf_getsrclines(cu_die, &mut lines, &mut nlines) != 0 {
        pr_debug2(cstr(b"Failed to get source lines on this CU.\n\0"));
        return -ENOENT;
    }
    pr_debug2(cstr(b"Get %zd lines from this CU\n\0"), nlines);

    /* Walk on the lines on lines list */
    i = 0;
    while i < nlines {
        line = dwarf_onesrcline(lines, i);
        if line.is_null() || dwarf_lineno(line, &mut lineno) != 0 || dwarf_lineaddr(line, &mut addr) != 0 {
            pr_debug2(cstr(b"Failed to get line info. Possible error in debuginfo.\n\0"));
            i += 1;
            continue;
        }
        /* Skip end-of-sequence */
        if dwarf_lineendsequence(line, &mut flag) != 0 || flag {
            i += 1;
            continue;
        }
        /* Skip Non statement line-info */
        if dwarf_linebeginstatement(line, &mut flag) != 0 || !flag {
            i += 1;
            continue;
        }
        /* Filter lines based on address */
        if rt_die != cu_die {
            /*
             * Address filtering
             * The line is included in given function, and
             * no inline block includes it.
             */
            if !dwarf_haspc(rt_die, addr) {
                i += 1;
                continue;
            }

            if !die_find_inlinefunc(rt_die, addr, &mut die_mem).is_null() {
                /* Call-site check */
                inf = die_get_call_file(&mut die_mem);
                if (inf == decf || (!inf.is_null() && !decf.is_null() && strcmp(inf, decf) == 0))
                    && die_get_call_lineno(&mut die_mem) == lineno
                {
                } else {
                    if dwarf_decl_line(&mut die_mem, &mut inl) != 0 {
                        inl = 0;
                    }
                    if inl != decl || decf != die_get_decl_file(&mut die_mem) {
                        i += 1;
                        continue;
                    }
                }
            }
        }
        /* Get source line */
        fname = dwarf_linesrc(line, ptr::null_mut(), ptr::null_mut());

        ret = callback.unwrap()(fname, lineno, addr, data);
        if ret != 0 {
            return ret;
        }
        i += 1;
    }

    /*
     * Dwarf lines doesn't include function declarations and inlined
     * subroutines. We have to check functions list or given function.
     */
    if rt_die != cu_die {
        ret = __die_walk_funclines(rt_die, false, callback, data);
    } else {
        let mut param = __line_walk_param { recursive: false, callback, data, retval: 0 };
        if dwarf_getfuncs(cu_die, Some(__die_walk_culines_cb), &mut param as *mut _ as *mut c_void, 0) < 0 {
            ret = -EINVAL;
        } else {
            ret = param.retval;
        }
    }

    ret
}

#[repr(C)]
struct __find_variable_param {
    name: *const c_char,
    addr: Dwarf_Addr,
}

unsafe extern "C" fn __die_find_variable_cb(die_mem: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let fvp = data as *mut __find_variable_param;
    let mut attr: Dwarf_Attribute = mem::zeroed();
    let tag = dwarf_tag(die_mem);

    if (tag == DW_TAG_formal_parameter || tag == DW_TAG_variable)
        && die_compare_name(die_mem, (*fvp).name)
        /*
         * Does the DIE have location information or const value
         * or external instance?
         */
        && (!dwarf_attr(die_mem, DW_AT_external, &mut attr).is_null()
            || !dwarf_attr(die_mem, DW_AT_location, &mut attr).is_null()
            || !dwarf_attr(die_mem, DW_AT_const_value, &mut attr).is_null())
    {
        return DIE_FIND_CB_END;
    }
    if dwarf_haspc(die_mem, (*fvp).addr) {
        DIE_FIND_CB_CONTINUE
    } else {
        DIE_FIND_CB_SIBLING
    }
}

#[no_mangle]
pub unsafe extern "C" fn die_find_variable_at(sp_die: *mut Dwarf_Die, name: *const c_char, addr: Dwarf_Addr, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut fvp = __find_variable_param { name, addr };

    die_find_child(sp_die, Some(__die_find_variable_cb), &mut fvp as *mut _ as *mut c_void, die_mem)
}

unsafe extern "C" fn __die_find_member_cb(die_mem: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let name = data as *const c_char;

    if dwarf_tag(die_mem) == DW_TAG_member {
        if die_compare_name(die_mem, name) {
            return DIE_FIND_CB_END;
        } else if dwarf_diename(die_mem).is_null() {
            /* Unnamed structure */
            let mut type_die: Dwarf_Die = mem::zeroed();
            let mut tmp_die: Dwarf_Die = mem::zeroed();
            if !die_get_type(die_mem, &mut type_die).is_null() && !die_find_member(&mut type_die, name, &mut tmp_die).is_null() {
                return DIE_FIND_CB_END;
            }
        }
    }
    DIE_FIND_CB_SIBLING
}

#[no_mangle]
pub unsafe extern "C" fn die_find_member(st_die: *mut Dwarf_Die, name: *const c_char, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    die_find_child(st_die, Some(__die_find_member_cb), name as *mut c_void, die_mem)
}

#[no_mangle]
pub unsafe extern "C" fn die_get_typename_from_type(type_die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int {
    let mut ret: c_int;
    let mut tmp = cstr(b"\0");

    let tag = dwarf_tag(type_die);
    if tag == DW_TAG_pointer_type {
        tmp = cstr(b"*\0");
    } else if tag == DW_TAG_array_type {
        tmp = cstr(b"[]\0");
    } else if tag == DW_TAG_subroutine_type {
        /* Function pointer */
        return strbuf_add(buf, cstr(b"(function_type)\0") as *const c_void, 15);
    } else {
        let name = dwarf_diename(type_die);

        if tag == DW_TAG_union_type {
            tmp = cstr(b"union \0");
        } else if tag == DW_TAG_structure_type {
            tmp = cstr(b"struct \0");
        } else if tag == DW_TAG_enumeration_type {
            tmp = cstr(b"enum \0");
        } else if name.is_null() {
            return -ENOENT;
        }
        /* Write a base name */
        return strbuf_addf(buf, cstr(b"%s%s\0"), tmp, if name.is_null() { cstr(b"\0") } else { name });
    }
    ret = die_get_typename(type_die, buf);
    if ret < 0 {
        /* void pointer has no type attribute */
        if tag == DW_TAG_pointer_type && ret == -ENOENT {
            return strbuf_addf(buf, cstr(b"void*\0"));
        }

        return ret;
    }
    strbuf_addstr(buf, tmp)
}

#[no_mangle]
pub unsafe extern "C" fn die_get_typename(vr_die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int {
    let mut type_: Dwarf_Die = mem::zeroed();

    if __die_get_real_type(vr_die, &mut type_).is_null() {
        return -ENOENT;
    }

    die_get_typename_from_type(&mut type_, buf)
}

#[no_mangle]
pub unsafe extern "C" fn die_get_varname(vr_die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int {
    let mut ret: c_int;

    ret = die_get_typename(vr_die, buf);
    if ret < 0 {
        pr_debug(cstr(b"Failed to get type, make it unknown.\n\0"));
        ret = strbuf_add(buf, cstr(b"(unknown_type)\0") as *const c_void, 14);
    }

    if ret < 0 { ret } else { strbuf_addf(buf, cstr(b"\t%s\0"), dwarf_diename(vr_die)) }
}

unsafe extern "C" fn reg_from_dwarf_op(op: *mut Dwarf_Op) -> c_int {
    if (*op).atom >= DW_OP_reg0 && (*op).atom <= DW_OP_reg31 {
        return ((*op).atom - DW_OP_reg0) as c_int;
    }
    if (*op).atom >= DW_OP_breg0 && (*op).atom <= DW_OP_breg31 {
        return ((*op).atom - DW_OP_breg0) as c_int;
    }
    if (*op).atom == DW_OP_regx || (*op).atom == DW_OP_bregx {
        return (*op).number as c_int;
    }
    if (*op).atom == DW_OP_fbreg {
        return DWARF_REG_FB;
    }
    -1
}

unsafe extern "C" fn offset_from_dwarf_op(op: *mut Dwarf_Op) -> c_int {
    if ((*op).atom >= DW_OP_reg0 && (*op).atom <= DW_OP_reg31) || (*op).atom == DW_OP_regx {
        return 0;
    }
    if ((*op).atom >= DW_OP_breg0 && (*op).atom <= DW_OP_breg31) || (*op).atom == DW_OP_fbreg {
        return (*op).number as c_int;
    }
    if (*op).atom == DW_OP_bregx {
        return (*op).number2 as c_int;
    }
    -1
}

unsafe extern "C" fn check_allowed_ops(mut ops: *mut Dwarf_Op, mut nops: size_t) -> bool {
    /* The first op is checked separately */
    ops = ops.add(1);
    nops -= 1;

    /*
     * It needs to make sure if the location expression matches to the given
     * register and offset exactly.  Thus it rejects any complex expressions
     * and only allows a few of selected operators that doesn't change the
     * location.
     */
    while nops != 0 {
        if !((*ops).atom == DW_OP_stack_value
            || (*ops).atom == DW_OP_deref_size
            || (*ops).atom == DW_OP_deref
            || (*ops).atom == DW_OP_piece)
        {
            return false;
        }
        ops = ops.add(1);
        nops -= 1;
    }
    true
}

unsafe extern "C" fn die_get_var_innermost_scope(sp_die: *mut Dwarf_Die, vr_die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int {
    let mut scopes: *mut Dwarf_Die = ptr::null_mut();
    let count: c_int;
    let mut offset: size_t = 0;
    let mut base: Dwarf_Addr = 0;
    let mut start: Dwarf_Addr = 0;
    let mut end: Dwarf_Addr = 0;
    let mut entry: Dwarf_Addr = 0;
    let mut ret: c_int;
    let mut first = true;
    let name: *const c_char;

    ret = die_entrypc(sp_die, &mut entry);
    if ret != 0 {
        return ret;
    }

    name = dwarf_diename(sp_die);
    if name.is_null() {
        return -ENOENT;
    }

    count = dwarf_getscopes_die(vr_die, &mut scopes);

    /* (*SCOPES)[1] is the DIE for the scope containing that scope */
    if count <= 1 {
        ret = -EINVAL;
        free(scopes as *mut c_void);
        return ret;
    }

    loop {
        let next = dwarf_ranges(scopes.add(1), offset, &mut base, &mut start, &mut end);
        if next <= 0 {
            break;
        }
        offset = next as size_t;
        start -= entry;
        end -= entry;

        if first {
            ret = strbuf_addf(buf, cstr(b"@<%s+[%lu-%lu\0"), name, start, end);
            first = false;
        } else {
            ret = strbuf_addf(buf, cstr(b",%lu-%lu\0"), start, end);
        }
        if ret < 0 {
            free(scopes as *mut c_void);
            return ret;
        }
    }

    if !first {
        ret = strbuf_add(buf, cstr(b"]>\0") as *const c_void, 2);
    }

    free(scopes as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn die_get_var_range(sp_die: *mut Dwarf_Die, vr_die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int {
    let mut ret: c_int = 0;
    let mut base: Dwarf_Addr = 0;
    let mut start: Dwarf_Addr = 0;
    let mut end: Dwarf_Addr = 0;
    let mut entry: Dwarf_Addr = 0;
    let mut op: *mut Dwarf_Op = ptr::null_mut();
    let mut nops: size_t = 0;
    let mut offset: size_t = 0;
    let mut attr: Dwarf_Attribute = mem::zeroed();
    let mut first = true;
    let name: *const c_char;

    ret = die_entrypc(sp_die, &mut entry);
    if ret != 0 {
        return ret;
    }

    name = dwarf_diename(sp_die);
    if name.is_null() {
        return -ENOENT;
    }

    if dwarf_attr(vr_die, DW_AT_location, &mut attr).is_null() {
        return -EINVAL;
    }

    loop {
        let next = dwarf_getlocations(&mut attr, offset as ptrdiff_t, &mut base, &mut start, &mut end, &mut op, &mut nops);
        if next <= 0 {
            break;
        }
        offset = next as size_t;
        if start == 0 {
            /* Single Location Descriptions */
            ret = die_get_var_innermost_scope(sp_die, vr_die, buf);
            return ret;
        }

        /* Location Lists */
        start -= entry;
        end -= entry;
        if first {
            ret = strbuf_addf(buf, cstr(b"@<%s+[%lu-%lu\0"), name, start, end);
            first = false;
        } else {
            ret = strbuf_addf(buf, cstr(b",%lu-%lu\0"), start, end);
        }
        if ret < 0 {
            return ret;
        }
    }

    if !first {
        ret = strbuf_add(buf, cstr(b"]>\0") as *const c_void, 2);
    }
    ret
}

#[repr(C)]
struct find_var_data {
    /* Target instruction address */
    pc: Dwarf_Addr,
    /* Target memory address (for global data) */
    addr: Dwarf_Addr,
    /* Target register */
    reg: c_uint,
    /* Access data type */
    type_: Dwarf_Die,
    /* Access offset, set for global data */
    offset: c_int,
    /* True if the current register is the frame base */
    is_fbreg: bool,
}

/* Max number of registers DW_OP_regN supports */
const DWARF_OP_DIRECT_REGS: c_uint = 32;

unsafe extern "C" fn match_var_offset(die_mem: *mut Dwarf_Die, data: *mut find_var_data, addr_offset: s64, addr_type: s64, is_pointer: bool) -> bool {
    let mut size: Dwarf_Word = 0;
    let mut ptr_die: Dwarf_Die = mem::zeroed();
    let mut offset: s64 = addr_offset - addr_type;

    if offset < 0 {
        return false;
    }

    if __die_get_real_type(die_mem, &mut (*data).type_).is_null() {
        return false;
    }

    let ptr_type = die_get_pointer_type(&mut (*data).type_, &mut ptr_die);
    if is_pointer && !ptr_type.is_null() {
        /* Get the target type of the pointer */
        if __die_get_real_type(ptr_type, &mut (*data).type_).is_null() {
            return false;
        }
    }

    if offset == 0 {
        /* Update offset relative to the start of the variable */
        (*data).offset = 0;
        return true;
    }

    if dwarf_aggregate_size(&mut (*data).type_, &mut size) < 0 {
        return false;
    }

    if (offset as u64) >= size {
        return false;
    }

    /* Update offset relative to the start of the variable */
    (*data).offset = offset as c_int;
    true
}

unsafe extern "C" fn is_breg_access_indirect(ops: *mut Dwarf_Op, nops: size_t) -> bool {
    /* only the base register */
    if nops == 1 {
        return false;
    }

    if nops == 2 && (*ops.add(1)).atom == DW_OP_stack_value {
        return true;
    }

    if nops == 3
        && ((*ops.add(1)).atom == DW_OP_deref || (*ops.add(1)).atom == DW_OP_deref_size)
        && (*ops.add(2)).atom == DW_OP_stack_value
    {
        return false;
    }
    /* unreachable, OP not supported */
    false
}

unsafe extern "C" fn __die_find_var_reg_cb(die_mem: *mut Dwarf_Die, arg: *mut c_void) -> c_int {
    let data = arg as *mut find_var_data;
    let tag = dwarf_tag(die_mem);
    let mut off: ptrdiff_t = 0;
    let mut attr: Dwarf_Attribute = mem::zeroed();
    let mut base: Dwarf_Addr = 0;
    let mut start: Dwarf_Addr = 0;
    let mut end: Dwarf_Addr = 0;
    let mut ops: *mut Dwarf_Op = ptr::null_mut();
    let mut nops: size_t = 0;

    if tag != DW_TAG_variable && tag != DW_TAG_formal_parameter {
        return DIE_FIND_CB_SIBLING;
    }

    if dwarf_attr(die_mem, DW_AT_location, &mut attr).is_null() {
        return DIE_FIND_CB_SIBLING;
    }

    loop {
        off = dwarf_getlocations(&mut attr, off, &mut base, &mut start, &mut end, &mut ops, &mut nops);
        if off <= 0 {
            break;
        }
        /* Assuming the location list is sorted by address */
        if end <= (*data).pc {
            continue;
        }
        if start > (*data).pc {
            break;
        }

        /* Local variables accessed using frame base register */
        if (*data).is_fbreg
            && (*ops).atom == DW_OP_fbreg
            && check_allowed_ops(ops, nops)
            && match_var_offset(die_mem, data, (*data).offset as s64, (*ops).number as s64, is_breg_access_indirect(ops, nops))
        {
            return DIE_FIND_CB_END;
        }

        /* Only match with a simple case */
        if (*data).reg < DWARF_OP_DIRECT_REGS {
            /* pointer variables saved in a register 0 to 31 */
            if (*ops).atom == DW_OP_reg0 + (*data).reg
                && check_allowed_ops(ops, nops)
                && match_var_offset(die_mem, data, (*data).offset as s64, 0, true)
            {
                return DIE_FIND_CB_END;
            }

            /* variables accessed by a register + offset */
            if (*ops).atom == DW_OP_breg0 + (*data).reg
                && check_allowed_ops(ops, nops)
                && match_var_offset(die_mem, data, (*data).offset as s64, (*ops).number as s64, is_breg_access_indirect(ops, nops))
            {
                return DIE_FIND_CB_END;
            }
        } else {
            /* pointer variables saved in a register 32 or above */
            if (*ops).atom == DW_OP_regx
                && (*ops).number == (*data).reg as Dwarf_Word
                && check_allowed_ops(ops, nops)
                && match_var_offset(die_mem, data, (*data).offset as s64, 0, true)
            {
                return DIE_FIND_CB_END;
            }

            /* variables accessed by a register + offset */
            if (*ops).atom == DW_OP_bregx
                && (*data).reg as Dwarf_Word == (*ops).number
                && check_allowed_ops(ops, nops)
                && match_var_offset(die_mem, data, (*data).offset as s64, (*ops).number2 as s64, is_breg_access_indirect(ops, nops))
            {
                return DIE_FIND_CB_END;
            }
        }
    }
    DIE_FIND_CB_SIBLING
}

#[no_mangle]
pub unsafe extern "C" fn die_find_variable_by_reg(sc_die: *mut Dwarf_Die, pc: Dwarf_Addr, reg: c_int, type_die: *mut Dwarf_Die, poffset: *mut c_int, is_fbreg: bool, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut data: find_var_data = mem::zeroed();
    data.pc = pc;
    data.reg = reg as c_uint;
    data.offset = *poffset;
    data.is_fbreg = is_fbreg;
    let result = die_find_child(sc_die, Some(__die_find_var_reg_cb), &mut data as *mut _ as *mut c_void, die_mem);
    if !result.is_null() {
        *poffset = data.offset;
        *type_die = data.type_;
    }
    result
}

unsafe extern "C" fn __die_find_var_addr_cb(die_mem: *mut Dwarf_Die, arg: *mut c_void) -> c_int {
    let data = arg as *mut find_var_data;
    let tag = dwarf_tag(die_mem);
    let mut off: ptrdiff_t = 0;
    let mut attr: Dwarf_Attribute = mem::zeroed();
    let mut base: Dwarf_Addr = 0;
    let mut start: Dwarf_Addr = 0;
    let mut end: Dwarf_Addr = 0;
    let mut ops: *mut Dwarf_Op = ptr::null_mut();
    let mut nops: size_t = 0;

    if tag != DW_TAG_variable {
        return DIE_FIND_CB_SIBLING;
    }

    if dwarf_attr(die_mem, DW_AT_location, &mut attr).is_null() {
        return DIE_FIND_CB_SIBLING;
    }

    loop {
        off = dwarf_getlocations(&mut attr, off, &mut base, &mut start, &mut end, &mut ops, &mut nops);
        if off <= 0 {
            break;
        }
        if (*ops).atom != DW_OP_addr {
            continue;
        }

        if check_allowed_ops(ops, nops) && match_var_offset(die_mem, data, (*data).addr as s64, (*ops).number as s64, false) {
            return DIE_FIND_CB_END;
        }
    }
    DIE_FIND_CB_SIBLING
}

#[no_mangle]
pub unsafe extern "C" fn die_find_variable_by_addr(sc_die: *mut Dwarf_Die, addr: Dwarf_Addr, die_mem: *mut Dwarf_Die, type_die: *mut Dwarf_Die, offset: *mut c_int) -> *mut Dwarf_Die {
    let mut data: find_var_data = mem::zeroed();
    data.addr = addr;
    let result = die_find_child(sc_die, Some(__die_find_var_addr_cb), &mut data as *mut _ as *mut c_void, die_mem);
    if !result.is_null() {
        *offset = data.offset;
        *type_die = data.type_;
    }
    result
}

unsafe extern "C" fn __die_collect_vars_cb(die_mem: *mut Dwarf_Die, arg: *mut c_void) -> c_int {
    let var_types = arg as *mut *mut die_var_type;
    let mut type_die: Dwarf_Die = mem::zeroed();
    let tag = dwarf_tag(die_mem);
    let mut attr: Dwarf_Attribute = mem::zeroed();
    let mut base: Dwarf_Addr = 0;
    let mut start: Dwarf_Addr = 0;
    let mut end: Dwarf_Addr = 0;
    let mut ops: *mut Dwarf_Op = ptr::null_mut();
    let mut nops: size_t = 0;
    let mut off: ptrdiff_t;

    if tag != DW_TAG_variable && tag != DW_TAG_formal_parameter {
        return DIE_FIND_CB_SIBLING;
    }

    if dwarf_attr(die_mem, DW_AT_location, &mut attr).is_null() {
        return DIE_FIND_CB_SIBLING;
    }

    if __die_get_real_type(die_mem, &mut type_die).is_null() {
        return DIE_FIND_CB_SIBLING;
    }

    /*
     * Collect all location entries as variables may have different
     * locations across different address ranges.
     */
    off = 0;
    loop {
        off = dwarf_getlocations(&mut attr, off, &mut base, &mut start, &mut end, &mut ops, &mut nops);
        if off <= 0 {
            break;
        }
        if !check_allowed_ops(ops, nops) {
            continue;
        }

        let vt = malloc(mem::size_of::<die_var_type>()) as *mut die_var_type;
        if vt.is_null() {
            return DIE_FIND_CB_END;
        }

        /* Usually a register holds the value of a variable */
        (*vt).is_reg_var_addr = false;

        if (((*ops).atom >= DW_OP_breg0 && (*ops).atom <= DW_OP_breg31) || (*ops).atom == DW_OP_bregx || (*ops).atom == DW_OP_fbreg)
            && !is_breg_access_indirect(ops, nops)
        {
            /* The register contains an address of the variable. */
            (*vt).is_reg_var_addr = true;
        }

        (*vt).die_off = dwarf_dieoffset(&mut type_die);
        (*vt).addr = start;
        (*vt).end = end;
        (*vt).has_range = end != 0 || start != 0;
        (*vt).reg = reg_from_dwarf_op(ops);
        (*vt).offset = offset_from_dwarf_op(ops);
        (*vt).next = *var_types;
        *var_types = vt;
    }

    DIE_FIND_CB_SIBLING
}

#[no_mangle]
pub unsafe extern "C" fn die_collect_vars(sc_die: *mut Dwarf_Die, var_types: *mut *mut die_var_type) {
    let mut die_mem: Dwarf_Die = mem::zeroed();

    die_find_child(sc_die, Some(__die_collect_vars_cb), var_types as *mut c_void, &mut die_mem);
}

unsafe extern "C" fn __die_collect_global_vars_cb(die_mem: *mut Dwarf_Die, arg: *mut c_void) -> c_int {
    let var_types = arg as *mut *mut die_var_type;
    let mut type_die: Dwarf_Die = mem::zeroed();
    let tag = dwarf_tag(die_mem);
    let mut attr: Dwarf_Attribute = mem::zeroed();
    let mut base: Dwarf_Addr = 0;
    let mut start: Dwarf_Addr = 0;
    let mut end: Dwarf_Addr = 0;
    let mut ops: *mut Dwarf_Op = ptr::null_mut();
    let mut nops: size_t = 0;

    if tag != DW_TAG_variable {
        return DIE_FIND_CB_SIBLING;
    }

    if dwarf_attr(die_mem, DW_AT_location, &mut attr).is_null() {
        return DIE_FIND_CB_SIBLING;
    }

    /* Only collect the location with an absolute address. */
    if dwarf_getlocations(&mut attr, 0, &mut base, &mut start, &mut end, &mut ops, &mut nops) <= 0 {
        return DIE_FIND_CB_SIBLING;
    }

    if (*ops).atom != DW_OP_addr {
        return DIE_FIND_CB_SIBLING;
    }

    if !check_allowed_ops(ops, nops) {
        return DIE_FIND_CB_SIBLING;
    }

    if die_get_real_type(die_mem, &mut type_die).is_null() {
        return DIE_FIND_CB_SIBLING;
    }

    let vt = malloc(mem::size_of::<die_var_type>()) as *mut die_var_type;
    if vt.is_null() {
        return DIE_FIND_CB_END;
    }

    (*vt).die_off = dwarf_dieoffset(&mut type_die);
    (*vt).addr = (*ops).number;
    (*vt).end = 0;
    (*vt).has_range = false;
    (*vt).reg = -1;
    (*vt).offset = 0;
    (*vt).next = *var_types;
    *var_types = vt;

    DIE_FIND_CB_SIBLING
}

#[no_mangle]
pub unsafe extern "C" fn die_collect_global_vars(cu_die: *mut Dwarf_Die, var_types: *mut *mut die_var_type) {
    let mut die_mem: Dwarf_Die = mem::zeroed();

    die_find_child(cu_die, Some(__die_collect_global_vars_cb), var_types as *mut c_void, &mut die_mem);
}

#[no_mangle]
pub unsafe extern "C" fn die_get_cfa(dwarf: *mut Dwarf, pc: u64, preg: *mut c_int, poffset: *mut c_int) -> c_int {
    let mut frame: *mut Dwarf_Frame = ptr::null_mut();
    let mut ops: *mut Dwarf_Op = ptr::null_mut();
    let mut nops: size_t = 0;

    let cfi = dwarf_getcfi(dwarf);
    if cfi.is_null() {
        return -1;
    }

    if dwarf_cfi_addrframe(cfi, pc, &mut frame) == 0 && dwarf_frame_cfa(frame, &mut ops, &mut nops) == 0 && check_allowed_ops(ops, nops) {
        *preg = reg_from_dwarf_op(ops);
        *poffset = offset_from_dwarf_op(ops);
        return 0;
    }
    -1
}

unsafe extern "C" fn die_has_loclist(vr_die: *mut Dwarf_Die) -> bool {
    let mut loc: Dwarf_Attribute = mem::zeroed();
    let tag = dwarf_tag(vr_die);

    if tag != DW_TAG_formal_parameter && tag != DW_TAG_variable {
        return false;
    }

    !dwarf_attr_integrate(vr_die, DW_AT_location, &mut loc).is_null() && dwarf_whatform(&mut loc) == DW_FORM_sec_offset
}

#[no_mangle]
pub unsafe extern "C" fn die_is_optimized_target(cu_die: *mut Dwarf_Die) -> bool {
    let mut tmp_die: Dwarf_Die = mem::zeroed();

    if die_has_loclist(cu_die) {
        return true;
    }

    if dwarf_child(cu_die, &mut tmp_die) == 0 && die_is_optimized_target(&mut tmp_die) {
        return true;
    }

    if dwarf_siblingof(cu_die, &mut tmp_die) == 0 && die_is_optimized_target(&mut tmp_die) {
        return true;
    }

    false
}

unsafe extern "C" fn die_search_idx(lines: *mut Dwarf_Lines, nr_lines: c_ulong, addr: Dwarf_Addr, idx: *mut c_ulong) -> bool {
    let mut i: c_ulong = 0;
    let mut tmp: Dwarf_Addr = 0;

    while i < nr_lines {
        if dwarf_lineaddr(dwarf_onesrcline(lines, i as size_t), &mut tmp) != 0 {
            return false;
        }

        if tmp == addr {
            *idx = i;
            return true;
        }
        i += 1;
    }
    false
}

unsafe extern "C" fn die_get_postprologue_addr(entrypc_idx: c_ulong, lines: *mut Dwarf_Lines, nr_lines: c_ulong, highpc: Dwarf_Addr, postprologue_addr: *mut Dwarf_Addr) -> bool {
    let mut i: c_ulong;
    let mut entrypc_lno: c_int = 0;
    let mut lno: c_int = 0;
    let mut line: *mut Dwarf_Line;
    let mut addr: Dwarf_Addr = 0;
    let mut p_end: bool = false;

    /* entrypc_lno is actual source line number */
    line = dwarf_onesrcline(lines, entrypc_idx as size_t);
    if dwarf_lineno(line, &mut entrypc_lno) != 0 {
        return false;
    }

    i = entrypc_idx;
    while i < nr_lines {
        line = dwarf_onesrcline(lines, i as size_t);

        if dwarf_lineaddr(line, &mut addr) != 0 || dwarf_lineno(line, &mut lno) != 0 || dwarf_lineprologueend(line, &mut p_end) != 0 {
            return false;
        }

        /* highpc is exclusive. [entrypc,highpc) */
        if addr >= highpc {
            break;
        }

        /* clang supports prologue-end marker */
        if p_end {
            break;
        }

        /* Actual next line in source */
        if lno != entrypc_lno {
            break;
        }

        /*
         * Single source line can have multiple line records.
         * For Example,
         *     void foo() { printf("hello\n"); }
         * contains two line records. One points to declaration and
         * other points to printf() line. Variable 'lno' won't get
         * incremented in this case but 'i' will.
         */
        if i != entrypc_idx {
            break;
        }
        i += 1;
    }

    if dwarf_lineaddr(line, postprologue_addr) != 0 {
        return false;
    }
    if *postprologue_addr >= highpc {
        if dwarf_lineaddr(dwarf_onesrcline(lines, (i - 1) as size_t), postprologue_addr) != 0 {
            return false;
        }
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn die_skip_prologue(sp_die: *mut Dwarf_Die, cu_die: *mut Dwarf_Die, entrypc: *mut Dwarf_Addr) {
    let mut nr_lines: size_t = 0;
    let mut entrypc_idx: c_ulong = 0;
    let mut lines: *mut Dwarf_Lines = ptr::null_mut();
    let mut postprologue_addr: Dwarf_Addr = 0;
    let mut highpc: Dwarf_Addr = 0;

    if dwarf_highpc(sp_die, &mut highpc) != 0 {
        return;
    }

    if dwarf_getsrclines(cu_die, &mut lines, &mut nr_lines) != 0 {
        return;
    }

    if !die_search_idx(lines, nr_lines as c_ulong, *entrypc, &mut entrypc_idx) {
        return;
    }

    if !die_get_postprologue_addr(entrypc_idx, lines, nr_lines as c_ulong, highpc, &mut postprologue_addr) {
        return;
    }

    *entrypc = postprologue_addr;
}

#[repr(C)]
struct find_scope_data {
    /* Target instruction address */
    pc: Dwarf_Addr,
    /* Number of scopes found [output] */
    nr: c_int,
    /* Array of scopes found, 0 for the outermost one. [output] */
    scopes: *mut Dwarf_Die,
}

unsafe extern "C" fn __die_find_scope_cb(die_mem: *mut Dwarf_Die, arg: *mut c_void) -> c_int {
    let data = arg as *mut find_scope_data;
    let tag = dwarf_tag(die_mem);

    if dwarf_haspc(die_mem, (*data).pc) {
        let tmp = realloc((*data).scopes as *mut c_void, (((*data).nr + 1) as size_t) * mem::size_of::<Dwarf_Die>()) as *mut Dwarf_Die;
        if tmp.is_null() {
            return DIE_FIND_CB_END;
        }

        ptr::copy_nonoverlapping(die_mem, tmp.add((*data).nr as size_t), 1);
        (*data).scopes = tmp;
        (*data).nr += 1;
        return DIE_FIND_CB_CHILD;
    }

    /*
     * If the DIE doesn't have the PC, we still need to check its children
     * and siblings if it's a container like a namespace.
     */
    if tag == DW_TAG_namespace {
        return DIE_FIND_CB_CONTINUE;
    }

    DIE_FIND_CB_SIBLING
}

#[no_mangle]
pub unsafe extern "C" fn die_get_scopes(cu_die: *mut Dwarf_Die, pc: Dwarf_Addr, scopes: *mut *mut Dwarf_Die) -> c_int {
    let mut data: find_scope_data = mem::zeroed();
    let mut die_mem: Dwarf_Die = mem::zeroed();
    data.pc = pc;

    die_find_child(cu_die, Some(__die_find_scope_cb), &mut data as *mut _ as *mut c_void, &mut die_mem);

    *scopes = data.scopes;
    data.nr
}

unsafe extern "C" fn __die_find_member_offset_cb(die_mem: *mut Dwarf_Die, arg: *mut c_void) -> c_int {
    let mut type_die: Dwarf_Die = mem::zeroed();
    let mut size: Dwarf_Word = 0;
    let mut loc: Dwarf_Word = 0;
    let offset: Dwarf_Word = arg as isize as Dwarf_Word;
    let tag = dwarf_tag(die_mem);

    if tag != DW_TAG_member {
        return DIE_FIND_CB_SIBLING;
    }

    /* Unions might not have location */
    if die_get_data_member_location(die_mem, &mut loc) < 0 {
        let mut attr: Dwarf_Attribute = mem::zeroed();

        if !dwarf_attr_integrate(die_mem, DW_AT_data_bit_offset, &mut attr).is_null() && dwarf_formudata(&mut attr, &mut loc) == 0 {
            loc /= 8;
        } else {
            loc = 0;
        }
    }

    if offset == loc {
        return DIE_FIND_CB_END;
    }

    if die_get_real_type(die_mem, &mut type_die).is_null() {
        // TODO: add a pr_debug_dtp() later for this unlikely failure
        return DIE_FIND_CB_SIBLING;
    }

    if dwarf_aggregate_size(&mut type_die, &mut size) < 0 {
        size = 0;
    }

    if loc < offset && offset < loc + size {
        return DIE_FIND_CB_END;
    }

    DIE_FIND_CB_SIBLING
}

#[no_mangle]
pub unsafe extern "C" fn die_get_member_type(type_die: *mut Dwarf_Die, mut offset: c_int, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut mb_type: Dwarf_Die;
    let mut tag: c_int;

    tag = dwarf_tag(type_die);
    /* If it's not a compound type, return the type directly */
    if tag != DW_TAG_structure_type && tag != DW_TAG_union_type {
        let mut size: Dwarf_Word = 0;

        if dwarf_aggregate_size(type_die, &mut size) < 0 {
            size = 0;
        }

        if (offset as c_uint) as Dwarf_Word >= size {
            return ptr::null_mut();
        }

        *die_mem = *type_die;
        return die_mem;
    }

    mb_type = *type_die;
    /* TODO: Handle union types better? */
    while tag == DW_TAG_structure_type || tag == DW_TAG_union_type {
        let member = die_find_child(&mut mb_type, Some(__die_find_member_offset_cb), offset as isize as *mut c_void, die_mem);
        if member.is_null() {
            return ptr::null_mut();
        }

        if die_get_real_type(member, &mut mb_type).is_null() {
            return ptr::null_mut();
        }

        tag = dwarf_tag(&mut mb_type);

        if tag == DW_TAG_structure_type || tag == DW_TAG_union_type || tag == DW_TAG_array_type {
            let mut loc: Dwarf_Word = 0;

            /* Update offset for the start of the member struct */
            if die_get_data_member_location(member, &mut loc) == 0 {
                offset -= loc as c_int;
            }
        }

        /* Handle array types: resolve to the element type by one level */
        if tag == DW_TAG_array_type {
            let mut size: Dwarf_Word = 0;

            if die_get_real_type(&mut mb_type, &mut mb_type).is_null() {
                return ptr::null_mut();
            }

            if dwarf_aggregate_size(&mut mb_type, &mut size) < 0 {
                return ptr::null_mut();
            }

            offset %= size as c_int;
            tag = dwarf_tag(&mut mb_type);
        }
    }
    *die_mem = mb_type;
    die_mem
}

#[no_mangle]
pub unsafe extern "C" fn die_deref_ptr_type(ptr_die: *mut Dwarf_Die, offset: c_int, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut type_die: Dwarf_Die = mem::zeroed();

    if dwarf_tag(ptr_die) != DW_TAG_pointer_type {
        return ptr::null_mut();
    }

    if die_get_real_type(ptr_die, &mut type_die).is_null() {
        return ptr::null_mut();
    }

    die_get_member_type(&mut type_die, offset, die_mem)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
