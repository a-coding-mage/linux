// SPDX-License-Identifier: GPL-2.0-only
/*
 * genelf_debug.c
 * Copyright (C) 2015, Google, Inc
 *
 * Contributed by:
 * 	Stephane Eranian <eranian@google.com>
 *
 * based on GPLv2 source code from Oprofile
 * @remark Copyright 2007 OProfile authors
 * @author Philippe Elie
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

const BUFFER_EXT_DFL_SIZE: usize = 4 * 1024;

type uword = u32;
type uhalf = u16;
type sword = i32;
type shalf = i16;
type ubyte = u8;
type sbyte = i8;

#[repr(C)]
struct buffer_ext {
    cur_pos: usize,
    max_sz: usize,
    data: *mut c_void,
}

#[repr(C)]
pub struct Elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf_Scn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf_Data {
    d_buf: *mut c_void,
    d_type: u32,
    d_version: u32,
    d_size: usize,
    d_off: i64,
    d_align: usize,
}

#[repr(C)]
pub struct Elf_Shdr {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

#[repr(C)]
pub struct debug_entry {
    addr: u64,
    lineno: c_int,
    name: *const c_char,
}

unsafe extern "C" {
    fn warnx(fmt: *const c_char, ...);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

    fn elf_newscn(elf: *mut Elf) -> *mut Elf_Scn;
    fn elf_newdata(scn: *mut Elf_Scn) -> *mut Elf_Data;
    fn elf_getshdr(scn: *mut Elf_Scn) -> *mut Elf_Shdr;
    fn elf_update(elf: *mut Elf, cmd: c_int) -> isize;

    fn debug_entry_next(ent: *mut debug_entry) -> *mut debug_entry;

    static GEN_ELF_TEXT_OFFSET: c_ulong;
}

/* Values normally supplied by <dwarf.h>, <libelf.h>, and related headers. */
const DW_LNS_copy: ubyte = 1;
const DW_LNS_advance_pc: ubyte = 2;
const DW_LNS_advance_line: ubyte = 3;
const DW_LNS_set_file: ubyte = 4;
const DW_LNS_set_column: ubyte = 5;
const DW_LNS_negate_stmt: ubyte = 6;
const DW_LNS_set_basic_block: ubyte = 7;
const DW_LNS_const_add_pc: ubyte = 8;
const DW_LNS_fixed_advance_pc: ubyte = 9;
const DW_LNS_set_prologue_end: ubyte = 10;
const DW_LNS_set_epilogue_begin: ubyte = 11;
const DW_LNS_set_isa: ubyte = 12;
const DW_LNE_end_sequence: ubyte = 1;
const DW_LNE_set_address: ubyte = 2;
const DW_LNE_define_file: ubyte = 3;
const DW_TAG_compile_unit: c_ulong = 0x11;
const DW_CHILDREN_yes: c_ulong = 1;
const DW_AT_stmt_list: c_ulong = 0x10;
const DW_FORM_data4: c_ulong = 0x06;
const ELF_T_BYTE: u32 = 0;
const EV_CURRENT: u32 = 1;
const SHT_PROGBITS: u32 = 1;
const ELF_C_WRITE: c_int = 6;

#[repr(C, packed)]
struct debug_line_header {
    // Not counting this field
    total_length: uword,
    // version number (2 currently)
    version: uhalf,
    // relative offset from next field to
    // program statement
    prolog_length: uword,
    minimum_instruction_length: ubyte,
    default_is_stmt: ubyte,
    // line_base - see DWARF 2 specs
    line_base: sbyte,
    // line_range - see DWARF 2 specs
    line_range: ubyte,
    // number of opcode + 1
    opcode_base: ubyte,
    /* follow the array of opcode args nr: ubytes [nr_opcode_base] */
    /* follow the search directories index, zero terminated string
     * terminated by an empty string.
     */
    /* follow an array of { filename, LEB128, LEB128, LEB128 }, first is
     * the directory index entry, 0 means current directory, then mtime
     * and filesize, last entry is followed by en empty string.
     */
    /* follow the first program statement */
}

/* DWARF 2 spec talk only about one possible compilation unit header while
 * binutils can handle two flavours of dwarf 2, 32 and 64 bits, this is not
 * related to the used arch, an ELF 32 can hold more than 4 Go of debug
 * information. For now we handle only DWARF 2 32 bits comp unit. It'll only
 * become a problem if we generate more than 4GB of debug information.
 */
#[repr(C, packed)]
struct compilation_unit_header {
    total_length: uword,
    version: uhalf,
    debug_abbrev_offset: uword,
    pointer_size: ubyte,
}

const DW_LNS_num_opcode: ubyte = DW_LNS_set_isa + 1;

/* field filled at run time are marked with -1 */
static default_debug_line_header: debug_line_header = debug_line_header {
    total_length: !0u32,
    version: 2,
    prolog_length: !0u32,
    minimum_instruction_length: 1, /* could be better when min instruction size != 1 */
    default_is_stmt: 1,           /* we don't take care about basic block */
    line_base: -5,                /* sensible value for line base ... */
    line_range: (-14i8) as u8,    /* ... and line range are guessed statically */
    opcode_base: DW_LNS_num_opcode,
};

static mut standard_opcode_length: [ubyte; 12] = [0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1];

/*
{
	[DW_LNS_advance_pc]   = 1,
	[DW_LNS_advance_line] = 1,
	[DW_LNS_set_file] =  1,
	[DW_LNS_set_column] = 1,
	[DW_LNS_fixed_advance_pc] = 1,
	[DW_LNS_set_isa] = 1,
};
*/

/* field filled at run time are marked with -1 */
static mut default_comp_unit_header: compilation_unit_header = compilation_unit_header {
    total_length: !0u32,
    version: 2,
    debug_abbrev_offset: 0, /* we reuse the same abbrev entries for all comp unit */
    pointer_size: size_of::<*mut c_void>() as u8,
};

unsafe fn buffer_ext_dump(be: *mut buffer_ext, msg: *const c_char) {
    let mut i: usize;

    unsafe {
        warnx(c"DUMP for %s".as_ptr(), msg);
        i = 0;
        while i < (*be).cur_pos {
            warnx(
                c"%4zu 0x%02x".as_ptr(),
                i,
                *((*be).data as *mut c_char).add(i) as c_int & 0xff,
            );
            i += 1;
        }
    }
}

unsafe fn buffer_ext_add(be: *mut buffer_ext, addr: *mut c_void, sz: usize) -> c_int {
    let mut tmp: *mut c_void;
    let mut be_sz: usize;

    unsafe {
        be_sz = (*be).max_sz;

        loop {
            if ((*be).cur_pos).wrapping_add(sz) < be_sz {
                memcpy(((*be).data as *mut u8).add((*be).cur_pos) as *mut c_void, addr, sz);
                (*be).cur_pos = ((*be).cur_pos).wrapping_add(sz);
                return 0;
            }

            if be_sz == 0 {
                be_sz = BUFFER_EXT_DFL_SIZE;
            } else {
                be_sz <<= 1;
            }

            tmp = realloc((*be).data, be_sz);
            if tmp.is_null() {
                return -1;
            }

            (*be).data = tmp;
            (*be).max_sz = be_sz;
        }
    }
}

unsafe fn buffer_ext_init(be: *mut buffer_ext) {
    unsafe {
        (*be).data = ptr::null_mut();
        (*be).cur_pos = 0;
        (*be).max_sz = 0;
    }
}

unsafe fn buffer_ext_exit(be: *mut buffer_ext) {
    unsafe {
        if !(*be).data.is_null() {
            free((*be).data);
            (*be).data = ptr::null_mut();
        }
    }
}

unsafe fn buffer_ext_size(be: *mut buffer_ext) -> usize {
    unsafe { (*be).cur_pos }
}

unsafe fn buffer_ext_addr(be: *mut buffer_ext) -> *mut c_void {
    unsafe { (*be).data }
}

unsafe fn emit_uword(be: *mut buffer_ext, mut data: uword) {
    unsafe {
        buffer_ext_add(be, &mut data as *mut uword as *mut c_void, size_of::<uword>());
    }
}

unsafe fn emit_string(be: *mut buffer_ext, s: *const c_char) {
    unsafe {
        buffer_ext_add(be, s as *mut c_void, strlen(s) + 1);
    }
}

unsafe fn emit_unsigned_LEB128(be: *mut buffer_ext, mut data: c_ulong) {
    unsafe {
        loop {
            let mut cur: ubyte = (data & 0x7f) as ubyte;
            data >>= 7;
            if data != 0 {
                cur |= 0x80;
            }
            buffer_ext_add(be, &mut cur as *mut ubyte as *mut c_void, 1);
            if data == 0 {
                break;
            }
        }
    }
}

unsafe fn emit_signed_LEB128(be: *mut buffer_ext, mut data: c_long) {
    let mut more: c_int = 1;
    let negative: c_int = (data < 0) as c_int;
    let size: c_int = (size_of::<c_long>() * 8) as c_int;

    unsafe {
        while more != 0 {
            let mut cur: ubyte = (data & 0x7f) as ubyte;
            data >>= 7;
            if negative != 0 {
                data |= -((1 as c_long) << (size - 7));
            }
            if (data == 0 && (cur & 0x40) == 0) || (data == -1 && (cur & 0x40) != 0) {
                more = 0;
            } else {
                cur |= 0x80;
            }
            buffer_ext_add(be, &mut cur as *mut ubyte as *mut c_void, 1);
        }
    }
}

unsafe fn emit_extended_opcode(
    be: *mut buffer_ext,
    mut opcode: ubyte,
    data: *mut c_void,
    data_len: usize,
) {
    unsafe {
        buffer_ext_add(be, c"".as_ptr() as *mut c_void, 1);

        emit_unsigned_LEB128(be, (data_len + 1) as c_ulong);

        buffer_ext_add(be, &mut opcode as *mut ubyte as *mut c_void, 1);
        buffer_ext_add(be, data, data_len);
    }
}

unsafe fn emit_opcode(be: *mut buffer_ext, mut opcode: ubyte) {
    unsafe {
        buffer_ext_add(be, &mut opcode as *mut ubyte as *mut c_void, 1);
    }
}

unsafe fn emit_opcode_signed(be: *mut buffer_ext, mut opcode: ubyte, data: c_long) {
    unsafe {
        buffer_ext_add(be, &mut opcode as *mut ubyte as *mut c_void, 1);
        emit_signed_LEB128(be, data);
    }
}

unsafe fn emit_opcode_unsigned(be: *mut buffer_ext, mut opcode: ubyte, data: c_ulong) {
    unsafe {
        buffer_ext_add(be, &mut opcode as *mut ubyte as *mut c_void, 1);
        emit_unsigned_LEB128(be, data);
    }
}

unsafe fn emit_advance_pc(be: *mut buffer_ext, delta_pc: c_ulong) {
    unsafe {
        emit_opcode_unsigned(be, DW_LNS_advance_pc, delta_pc);
    }
}

unsafe fn emit_advance_lineno(be: *mut buffer_ext, delta_lineno: c_long) {
    unsafe {
        emit_opcode_signed(be, DW_LNS_advance_line, delta_lineno);
    }
}

unsafe fn emit_lne_end_of_sequence(be: *mut buffer_ext) {
    unsafe {
        emit_extended_opcode(be, DW_LNE_end_sequence, ptr::null_mut(), 0);
    }
}

unsafe fn emit_set_file(be: *mut buffer_ext, idx: c_ulong) {
    unsafe {
        emit_opcode_unsigned(be, DW_LNS_set_file, idx);
    }
}

unsafe fn emit_lne_define_filename(be: *mut buffer_ext, filename: *const c_char) {
    unsafe {
        buffer_ext_add(be, c"".as_ptr() as *mut c_void, 1);

        /* LNE field, strlen(filename) + zero termination, 3 bytes for: the dir entry, timestamp, filesize */
        emit_unsigned_LEB128(be, (strlen(filename) + 5) as c_ulong);
        emit_opcode(be, DW_LNE_define_file);
        emit_string(be, filename);
        /* directory index 0=do not know */
        emit_unsigned_LEB128(be, 0);
        /* last modification date on file 0=do not know */
        emit_unsigned_LEB128(be, 0);
        /* filesize 0=do not know */
        emit_unsigned_LEB128(be, 0);
    }
}

unsafe fn emit_lne_set_address(be: *mut buffer_ext, mut address: *mut c_void) {
    unsafe {
        emit_extended_opcode(
            be,
            DW_LNE_set_address,
            &mut address as *mut *mut c_void as *mut c_void,
            size_of::<c_ulong>(),
        );
    }
}

unsafe fn get_special_opcode(
    ent: *mut debug_entry,
    last_line: c_uint,
    last_vma: c_ulong,
) -> ubyte {
    let temp: c_uint;
    let delta_addr: c_ulong;

    unsafe {
        /*
         * delta from line_base
         */
        temp = ((*ent).lineno as c_uint)
            .wrapping_sub(last_line)
            .wrapping_sub(default_debug_line_header.line_base as c_uint);

        if temp >= default_debug_line_header.line_range as c_uint {
            return 0;
        }

        /*
         * delta of addresses
         */
        delta_addr =
            ((*ent).addr as c_ulong).wrapping_sub(last_vma) / default_debug_line_header.minimum_instruction_length as c_ulong;

        /* This is not sufficient to ensure opcode will be in [0-256] but
         * sufficient to ensure when summing with the delta lineno we will
         * not overflow the unsigned long opcode */

        if delta_addr <= 256 / default_debug_line_header.line_range as c_ulong {
            let opcode: c_ulong = temp as c_ulong
                + (delta_addr * default_debug_line_header.line_range as c_ulong)
                + default_debug_line_header.opcode_base as c_ulong;

            return if opcode <= 255 { opcode as ubyte } else { 0 };
        }
        0
    }
}

type c_uint = u32;

unsafe fn emit_lineno_info(
    be: *mut buffer_ext,
    mut ent: *mut debug_entry,
    nr_entry: usize,
    code_addr: c_ulong,
) {
    let mut i: usize;

    /* as described in the jitdump format */
    let repeated_name_marker: [c_char; 2] = [-1i8 as c_char, 0];

    /*
     * Machine state at start of a statement program
     * address = 0
     * file    = 1
     * line    = 1
     * column  = 0
     * is_stmt = default_is_stmt as given in the debug_line_header
     * basic block = 0
     * end sequence = 0
     */

    /* start state of the state machine we take care of */
    let mut last_vma: c_ulong = 0;
    let mut cur_filename: *const c_char = ptr::null();
    let mut cur_file_idx: c_ulong = 0;
    let mut last_line: c_int = 1;

    unsafe {
        emit_lne_set_address(be, code_addr as *mut c_void);

        i = 0;
        while i < nr_entry {
            let mut need_copy: c_int = 0;
            let special_opcode: ubyte;

            /*
             * check if filename changed, if so add it
             */
            if (cur_filename.is_null() || strcmp(cur_filename, (*ent).name) != 0)
                && strcmp(repeated_name_marker.as_ptr(), (*ent).name) != 0
            {
                emit_lne_define_filename(be, (*ent).name);
                cur_filename = (*ent).name;
                cur_file_idx += 1;
                emit_set_file(be, cur_file_idx);
                need_copy = 1;
            }

            special_opcode = get_special_opcode(ent, last_line as c_uint, last_vma);
            if special_opcode != 0 {
                last_line = (*ent).lineno;
                last_vma = (*ent).addr as c_ulong;
                emit_opcode(be, special_opcode);
            } else {
                /*
                 * lines differ, emit line delta
                 */
                if last_line != (*ent).lineno {
                    emit_advance_lineno(be, ((*ent).lineno - last_line) as c_long);
                    last_line = (*ent).lineno;
                    need_copy = 1;
                }
                /*
                 * addresses differ, emit address delta
                 */
                if last_vma != (*ent).addr as c_ulong {
                    emit_advance_pc(be, ((*ent).addr as c_ulong).wrapping_sub(last_vma));
                    last_vma = (*ent).addr as c_ulong;
                    need_copy = 1;
                }
                /*
                 * add new row to matrix
                 */
                if need_copy != 0 {
                    emit_opcode(be, DW_LNS_copy);
                }
            }

            i += 1;
            ent = debug_entry_next(ent);
        }
    }
}

unsafe fn add_debug_line(
    be: *mut buffer_ext,
    ent: *mut debug_entry,
    nr_entry: usize,
    code_addr: c_ulong,
) {
    let mut dbg_header: *mut debug_line_header;
    let old_size: usize;

    unsafe {
        old_size = buffer_ext_size(be);

        buffer_ext_add(
            be,
            &default_debug_line_header as *const debug_line_header as *mut c_void,
            size_of::<debug_line_header>(),
        );

        buffer_ext_add(
            be,
            core::ptr::addr_of_mut!(standard_opcode_length) as *mut c_void,
            size_of::<[ubyte; 12]>(),
        );

        // empty directory entry
        buffer_ext_add(be, c"".as_ptr() as *mut c_void, 1);

        // empty filename directory
        buffer_ext_add(be, c"".as_ptr() as *mut c_void, 1);

        dbg_header = (buffer_ext_addr(be) as *mut u8).add(old_size) as *mut debug_line_header;
        ptr::addr_of_mut!((*dbg_header).prolog_length).write_unaligned(
            (buffer_ext_size(be) - old_size
                - offset_of!(debug_line_header, minimum_instruction_length)) as uword,
        );

        emit_lineno_info(be, ent, nr_entry, code_addr);

        emit_lne_end_of_sequence(be);

        dbg_header = (buffer_ext_addr(be) as *mut u8).add(old_size) as *mut debug_line_header;
        ptr::addr_of_mut!((*dbg_header).total_length).write_unaligned(
            (buffer_ext_size(be) - old_size - offset_of!(debug_line_header, version)) as uword,
        );
    }
}

unsafe fn add_debug_abbrev(be: *mut buffer_ext) {
    unsafe {
        emit_unsigned_LEB128(be, 1);
        emit_unsigned_LEB128(be, DW_TAG_compile_unit);
        emit_unsigned_LEB128(be, DW_CHILDREN_yes);
        emit_unsigned_LEB128(be, DW_AT_stmt_list);
        emit_unsigned_LEB128(be, DW_FORM_data4);
        emit_unsigned_LEB128(be, 0);
        emit_unsigned_LEB128(be, 0);
        emit_unsigned_LEB128(be, 0);
    }
}

unsafe fn add_compilation_unit(be: *mut buffer_ext, offset_debug_line: usize) {
    let comp_unit_header: *mut compilation_unit_header;
    let old_size: usize;

    unsafe {
        old_size = buffer_ext_size(be);

        buffer_ext_add(
            be,
            core::ptr::addr_of_mut!(default_comp_unit_header) as *mut c_void,
            size_of::<compilation_unit_header>(),
        );

        emit_unsigned_LEB128(be, 1);
        emit_uword(be, offset_debug_line as uword);

        comp_unit_header =
            (buffer_ext_addr(be) as *mut u8).add(old_size) as *mut compilation_unit_header;
        ptr::addr_of_mut!((*comp_unit_header).total_length).write_unaligned(
            (buffer_ext_size(be) - old_size - offset_of!(compilation_unit_header, version))
                as uword,
        );
    }
}

unsafe fn jit_process_debug_info(
    code_addr: u64,
    debug: *mut c_void,
    nr_debug_entries: c_int,
    dl: *mut buffer_ext,
    da: *mut buffer_ext,
    di: *mut buffer_ext,
) -> c_int {
    let mut ent: *mut debug_entry = debug as *mut debug_entry;
    let mut i: c_int;

    unsafe {
        i = 0;
        while i < nr_debug_entries {
            (*ent).addr = (*ent).addr.wrapping_sub(code_addr);
            ent = debug_entry_next(ent);
            i += 1;
        }
        add_compilation_unit(di, buffer_ext_size(dl));
        add_debug_line(
            dl,
            debug as *mut debug_entry,
            nr_debug_entries as usize,
            GEN_ELF_TEXT_OFFSET,
        );
        add_debug_abbrev(da);
        if false {
            buffer_ext_dump(da, c"abbrev".as_ptr());
        }

        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jit_add_debug_info(
    e: *mut Elf,
    code_addr: u64,
    debug: *mut c_void,
    nr_debug_entries: c_int,
) -> c_int {
    let mut d: *mut Elf_Data;
    let mut scn: *mut Elf_Scn;
    let mut shdr: *mut Elf_Shdr;
    let mut dl: buffer_ext = buffer_ext {
        cur_pos: 0,
        max_sz: 0,
        data: ptr::null_mut(),
    };
    let mut di: buffer_ext = buffer_ext {
        cur_pos: 0,
        max_sz: 0,
        data: ptr::null_mut(),
    };
    let mut da: buffer_ext = buffer_ext {
        cur_pos: 0,
        max_sz: 0,
        data: ptr::null_mut(),
    };
    let mut ret: c_int = -1;

    unsafe {
        buffer_ext_init(&mut dl);
        buffer_ext_init(&mut di);
        buffer_ext_init(&mut da);

        if jit_process_debug_info(
            code_addr,
            debug,
            nr_debug_entries,
            &mut dl,
            &mut da,
            &mut di,
        ) != 0
        {
            buffer_ext_exit(&mut dl);
            buffer_ext_exit(&mut di);
            buffer_ext_exit(&mut da);
            return ret;
        }

        /*
         * setup .debug_line section
         */
        scn = elf_newscn(e);
        if scn.is_null() {
            warnx(c"cannot create section".as_ptr());
            buffer_ext_exit(&mut dl);
            buffer_ext_exit(&mut di);
            buffer_ext_exit(&mut da);
            return ret;
        }

        d = elf_newdata(scn);
        if d.is_null() {
            warnx(c"cannot get new data".as_ptr());
            buffer_ext_exit(&mut dl);
            buffer_ext_exit(&mut di);
            buffer_ext_exit(&mut da);
            return ret;
        }

        (*d).d_align = 1;
        (*d).d_off = 0i64;
        (*d).d_buf = buffer_ext_addr(&mut dl);
        (*d).d_type = ELF_T_BYTE;
        (*d).d_size = buffer_ext_size(&mut dl);
        (*d).d_version = EV_CURRENT;

        shdr = elf_getshdr(scn);
        if shdr.is_null() {
            warnx(c"cannot get section header".as_ptr());
            buffer_ext_exit(&mut dl);
            buffer_ext_exit(&mut di);
            buffer_ext_exit(&mut da);
            return ret;
        }

        (*shdr).sh_name = 52; /* .debug_line */
        (*shdr).sh_type = SHT_PROGBITS;
        (*shdr).sh_addr = 0; /* must be zero or == sh_offset -> dynamic object */
        (*shdr).sh_flags = 0;
        (*shdr).sh_entsize = 0;

        /*
         * setup .debug_info section
         */
        scn = elf_newscn(e);
        if scn.is_null() {
            warnx(c"cannot create section".as_ptr());
            buffer_ext_exit(&mut dl);
            buffer_ext_exit(&mut di);
            buffer_ext_exit(&mut da);
            return ret;
        }

        d = elf_newdata(scn);
        if d.is_null() {
            warnx(c"cannot get new data".as_ptr());
            buffer_ext_exit(&mut dl);
            buffer_ext_exit(&mut di);
            buffer_ext_exit(&mut da);
            return ret;
        }

        (*d).d_align = 1;
        (*d).d_off = 0i64;
        (*d).d_buf = buffer_ext_addr(&mut di);
        (*d).d_type = ELF_T_BYTE;
        (*d).d_size = buffer_ext_size(&mut di);
        (*d).d_version = EV_CURRENT;

        shdr = elf_getshdr(scn);
        if shdr.is_null() {
            warnx(c"cannot get section header".as_ptr());
            buffer_ext_exit(&mut dl);
            buffer_ext_exit(&mut di);
            buffer_ext_exit(&mut da);
            return ret;
        }

        (*shdr).sh_name = 64; /* .debug_info */
        (*shdr).sh_type = SHT_PROGBITS;
        (*shdr).sh_addr = 0; /* must be zero or == sh_offset -> dynamic object */
        (*shdr).sh_flags = 0;
        (*shdr).sh_entsize = 0;

        /*
         * setup .debug_abbrev section
         */
        scn = elf_newscn(e);
        if scn.is_null() {
            warnx(c"cannot create section".as_ptr());
            buffer_ext_exit(&mut dl);
            buffer_ext_exit(&mut di);
            buffer_ext_exit(&mut da);
            return ret;
        }

        d = elf_newdata(scn);
        if d.is_null() {
            warnx(c"cannot get new data".as_ptr());
            buffer_ext_exit(&mut dl);
            buffer_ext_exit(&mut di);
            buffer_ext_exit(&mut da);
            return ret;
        }

        (*d).d_align = 1;
        (*d).d_off = 0i64;
        (*d).d_buf = buffer_ext_addr(&mut da);
        (*d).d_type = ELF_T_BYTE;
        (*d).d_size = buffer_ext_size(&mut da);
        (*d).d_version = EV_CURRENT;

        shdr = elf_getshdr(scn);
        if shdr.is_null() {
            warnx(c"cannot get section header".as_ptr());
            buffer_ext_exit(&mut dl);
            buffer_ext_exit(&mut di);
            buffer_ext_exit(&mut da);
            return ret;
        }

        (*shdr).sh_name = 76; /* .debug_info */
        (*shdr).sh_type = SHT_PROGBITS;
        (*shdr).sh_addr = 0; /* must be zero or == sh_offset -> dynamic object */
        (*shdr).sh_flags = 0;
        (*shdr).sh_entsize = 0;

        /*
         * now we update the ELF image with all the sections
         */
        if elf_update(e, ELF_C_WRITE) < 0 {
            warnx(c"elf_update debug failed".as_ptr());
        } else {
            ret = 0;
        }

        buffer_ext_exit(&mut dl);
        buffer_ext_exit(&mut di);
        buffer_ext_exit(&mut da);
        ret
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
