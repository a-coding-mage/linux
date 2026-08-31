// SPDX-License-Identifier: GPL-2.0-only
/*
 * genelf.c
 * Copyright (C) 2014, Google, Inc
 *
 * Contributed by:
 * 	Stephane Eranian <eranian@gmail.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

/*
 * C dependencies removed from executable Rust:
 * <sys/types.h>, <stddef.h>, <libelf.h>, <string.h>, <stdlib.h>,
 * <inttypes.h>, <err.h>, optionally <dwarf.h>, "blake2s.h",
 * "genelf.h", "../util/jitdump.h", and <linux/compiler.h>.
 */

const NT_GNU_BUILD_ID: c_uint = 3;

type U8 = u8;
type U64 = u64;

#[repr(C)]
pub struct Elf;
#[repr(C)]
pub struct Elf_Scn;

#[repr(C)]
pub struct Elf_Note {
    pub namesz: c_uint, /* Size of entry's owner string */
    pub descsz: c_uint, /* Size of the note descriptor */
    pub type_: c_uint,  /* Interpretation of the descriptor */
    pub name: [c_char; 0], /* Start of the name+desc data */
}

#[repr(C)]
pub struct buildid_note {
    pub desc: Elf_Note,       /* descsz: size of build-id, must be multiple of 4 */
    pub name: [c_char; 4],    /* GNU\0 */
    pub build_id: [U8; 20],
}

#[repr(C)]
pub struct Elf_Data {
    pub d_buf: *mut c_void,
    pub d_type: c_int,
    pub d_size: u64,
    pub d_off: i64,
    pub d_align: u64,
    pub d_version: c_uint,
}

#[repr(C)]
pub struct Elf_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
pub struct Elf_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf_Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Elf_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

#[repr(C)]
pub struct blake2s_ctx {
    _private: [u8; 0],
}

const EI_DATA: usize = 5;
const EI_CLASS: usize = 4;
const EV_NONE: c_uint = 0;
const EV_CURRENT: c_uint = 1;
const ELF_C_WRITE: c_int = 1;
const ELF_T_BYTE: c_int = 0;
const ELF_T_SYM: c_int = 11;
const ET_DYN: u16 = 3;
const PT_LOAD: u32 = 1;
const PF_X: u32 = 1;
const PF_R: u32 = 4;
const SHT_PROGBITS: u32 = 1;
const SHT_SYMTAB: u32 = 2;
const SHT_STRTAB: u32 = 3;
const SHT_NOTE: u32 = 7;
const SHF_ALLOC: u64 = 2;
const SHF_EXECINSTR: u64 = 4;
const STT_NOTYPE: u8 = 0;
const STT_FUNC: u8 = 2;
const STB_LOCAL: u8 = 0;
const STV_DEFAULT: u8 = 0;

const fn ELF_ST_BIND(bind: u8) -> u8 {
    bind << 4
}

const fn ELF_ST_TYPE(type_: u8) -> u8 {
    type_ & 0xf
}

const fn ELF_ST_VIS(visibility: u8) -> u8 {
    visibility & 0x3
}

const fn ALIGN_8(x: u64) -> u64 {
    (x + 7) & !7
}

/* GEN_ELF_* come from genelf.h in the original C translation unit. */
extern "C" {
    static GEN_ELF_ENDIAN: u8;
    static GEN_ELF_CLASS: u8;
    static GEN_ELF_ARCH: u16;
    static GEN_ELF_TEXT_OFFSET: u64;
}

extern "C" {
    fn warnx(fmt: *const c_char, ...);
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn elf_version(ver: c_uint) -> c_uint;
    fn elf_begin(fd: c_int, cmd: c_int, ref_: *mut Elf) -> *mut Elf;
    fn elf_end(elf: *mut Elf) -> c_int;
    fn elf_newehdr(elf: *mut Elf) -> *mut Elf_Ehdr;
    fn elf_newphdr(elf: *mut Elf, count: usize) -> *mut Elf_Phdr;
    fn elf_newscn(elf: *mut Elf) -> *mut Elf_Scn;
    fn elf_newdata(scn: *mut Elf_Scn) -> *mut Elf_Data;
    fn elf_getshdr(scn: *mut Elf_Scn) -> *mut Elf_Shdr;
    fn elf_update(elf: *mut Elf, cmd: c_int) -> i64;

    fn blake2s_init(ctx: *mut blake2s_ctx, outlen: usize);
    fn blake2s_update(ctx: *mut blake2s_ctx, in_: *const U8, inlen: usize);
    fn blake2s_final(ctx: *mut blake2s_ctx, out: *mut U8);

    /*
     * Present only when HAVE_LIBDW_SUPPORT is enabled in the original build.
     * The call site below preserves that conditional intent in a Rust cfg.
     */
    fn jit_add_debug_info(
        e: *mut Elf,
        load_addr: u64,
        debug: *mut c_void,
        nr_debug_entries: c_int,
    ) -> c_int;
}

static mut SHD_STRING_TABLE: [c_char; 114] = [
    0,
    b'.' as c_char, b't' as c_char, b'e' as c_char, b'x' as c_char, b't' as c_char, 0, /*  1 */
    b'.' as c_char, b's' as c_char, b'h' as c_char, b's' as c_char, b't' as c_char, b'r' as c_char, b't' as c_char, b'a' as c_char, b'b' as c_char, 0, /*  7 */
    b'.' as c_char, b's' as c_char, b'y' as c_char, b'm' as c_char, b't' as c_char, b'a' as c_char, b'b' as c_char, 0, /* 17 */
    b'.' as c_char, b's' as c_char, b't' as c_char, b'r' as c_char, b't' as c_char, b'a' as c_char, b'b' as c_char, 0, /* 25 */
    b'.' as c_char, b'n' as c_char, b'o' as c_char, b't' as c_char, b'e' as c_char, b'.' as c_char, b'g' as c_char, b'n' as c_char, b'u' as c_char, b'.' as c_char, b'b' as c_char, b'u' as c_char, b'i' as c_char, b'l' as c_char, b'd' as c_char, b'-' as c_char, b'i' as c_char, b'd' as c_char, 0, /* 33 */
    b'.' as c_char, b'd' as c_char, b'e' as c_char, b'b' as c_char, b'u' as c_char, b'g' as c_char, b'_' as c_char, b'l' as c_char, b'i' as c_char, b'n' as c_char, b'e' as c_char, 0, /* 52 */
    b'.' as c_char, b'd' as c_char, b'e' as c_char, b'b' as c_char, b'u' as c_char, b'g' as c_char, b'_' as c_char, b'i' as c_char, b'n' as c_char, b'f' as c_char, b'o' as c_char, 0, /* 64 */
    b'.' as c_char, b'd' as c_char, b'e' as c_char, b'b' as c_char, b'u' as c_char, b'g' as c_char, b'_' as c_char, b'a' as c_char, b'b' as c_char, b'b' as c_char, b'r' as c_char, b'e' as c_char, b'v' as c_char, 0, /* 76 */
    b'.' as c_char, b'e' as c_char, b'h' as c_char, b'_' as c_char, b'f' as c_char, b'r' as c_char, b'a' as c_char, b'm' as c_char, b'e' as c_char, b'_' as c_char, b'h' as c_char, b'd' as c_char, b'r' as c_char, 0, /* 90 */
    b'.' as c_char, b'e' as c_char, b'h' as c_char, b'_' as c_char, b'f' as c_char, b'r' as c_char, b'a' as c_char, b'm' as c_char, b'e' as c_char, 0, /* 104 */
];

static mut BNOTE: buildid_note = buildid_note {
    desc: Elf_Note {
        namesz: 0,
        descsz: 0,
        type_: 0,
        name: [],
    },
    name: [0; 4],
    build_id: [0; 20],
};

static mut SYMTAB: [Elf_Sym; 2] = [
    /* symbol 0 MUST be the undefined symbol */
    Elf_Sym {
        st_name: 0, /* index in sym_string table */
        st_info: ELF_ST_TYPE(STT_NOTYPE),
        st_shndx: 0, /* for now */
        st_value: 0x0,
        st_other: ELF_ST_VIS(STV_DEFAULT),
        st_size: 0,
    },
    Elf_Sym {
        st_name: 1, /* index in sym_string table */
        st_info: ELF_ST_BIND(STB_LOCAL) | ELF_ST_TYPE(STT_FUNC),
        st_shndx: 1,
        st_value: 0, /* for now */
        st_other: ELF_ST_VIS(STV_DEFAULT),
        st_size: 0, /* for now */
    },
];

unsafe fn jit_add_eh_frame_info(
    e: *mut Elf,
    unwinding: *mut c_void,
    unwinding_header_size: u64,
    unwinding_size: u64,
    base_offset: u64,
) -> c_int {
    let mut d: *mut Elf_Data;
    let mut scn: *mut Elf_Scn;
    let mut shdr: *mut Elf_Shdr;
    let unwinding_table_size: u64 = unwinding_size - unwinding_header_size;

    /*
     * setup eh_frame section
     */
    scn = elf_newscn(e);
    if scn.is_null() {
        warnx(c"cannot create section".as_ptr());
        return -1;
    }

    d = elf_newdata(scn);
    if d.is_null() {
        warnx(c"cannot get new data".as_ptr());
        return -1;
    }

    (*d).d_align = 8;
    (*d).d_off = 0;
    (*d).d_buf = unwinding;
    (*d).d_type = ELF_T_BYTE;
    (*d).d_size = unwinding_table_size;
    (*d).d_version = EV_CURRENT;

    shdr = elf_getshdr(scn);
    if shdr.is_null() {
        warnx(c"cannot get section header".as_ptr());
        return -1;
    }

    (*shdr).sh_name = 104;
    (*shdr).sh_type = SHT_PROGBITS;
    (*shdr).sh_addr = base_offset;
    (*shdr).sh_flags = SHF_ALLOC;
    (*shdr).sh_entsize = 0;

    /*
     * setup eh_frame_hdr section
     */
    scn = elf_newscn(e);
    if scn.is_null() {
        warnx(c"cannot create section".as_ptr());
        return -1;
    }

    d = elf_newdata(scn);
    if d.is_null() {
        warnx(c"cannot get new data".as_ptr());
        return -1;
    }

    (*d).d_align = 4;
    (*d).d_off = 0;
    (*d).d_buf = (unwinding as *mut u8).add(unwinding_table_size as usize) as *mut c_void;
    (*d).d_type = ELF_T_BYTE;
    (*d).d_size = unwinding_header_size;
    (*d).d_version = EV_CURRENT;

    shdr = elf_getshdr(scn);
    if shdr.is_null() {
        warnx(c"cannot get section header".as_ptr());
        return -1;
    }

    (*shdr).sh_name = 90;
    (*shdr).sh_type = SHT_PROGBITS;
    (*shdr).sh_addr = base_offset + unwinding_table_size;
    (*shdr).sh_flags = SHF_ALLOC;
    (*shdr).sh_entsize = 0;

    0
}

const TAG_CODE: c_int = 0;
const TAG_SYMTAB: c_int = 1;
const TAG_STRSYM: c_int = 2;

/*
 * Update the hash using the given data, also prepending a (tag, len) prefix to
 * ensure that distinct input tuples reliably result in distinct hashes.
 */
unsafe fn blake2s_update_tagged(
    ctx: *mut blake2s_ctx,
    tag: c_int,
    data: *const c_void,
    len: usize,
) {
    let prefix: U64 = ((tag as U64) << 56) | (len as U64);

    blake2s_update(
        ctx,
        &prefix as *const U64 as *const U8,
        size_of::<U64>(),
    );
    blake2s_update(ctx, data as *const U8, len);
}

/*
 * fd: file descriptor open for writing for the output file
 * load_addr: code load address (could be zero)
 * sym: function name (for native code - used as the symbol)
 * code: the native code
 * csize: the code size in bytes
 */
#[no_mangle]
pub unsafe extern "C" fn jit_write_elf(
    fd: c_int,
    load_addr: u64,
    sym: *const c_char,
    code: *const c_void,
    csize: c_int,
    debug: *mut c_void,
    nr_debug_entries: c_int,
    unwinding: *mut c_void,
    unwinding_header_size: u64,
    unwinding_size: u64,
) -> c_int {
    let mut e: *mut Elf;
    let mut d: *mut Elf_Data;
    let mut scn: *mut Elf_Scn;
    let mut ehdr: *mut Elf_Ehdr;
    let mut phdr: *mut Elf_Phdr;
    let mut shdr: *mut Elf_Shdr;
    let mut eh_frame_base_offset: u64;
    let mut strsym: *mut c_char = ptr::null_mut();
    let mut ctx: blake2s_ctx = zeroed();
    let mut symlen: c_int;
    let mut retval: c_int = -1;

    if elf_version(EV_CURRENT) == EV_NONE {
        warnx(c"ELF initialization failed".as_ptr());
        return -1;
    }

    e = elf_begin(fd, ELF_C_WRITE, ptr::null_mut());
    if e.is_null() {
        warnx(c"elf_begin failed".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    /*
     * setup ELF header
     */
    ehdr = elf_newehdr(e);
    if ehdr.is_null() {
        warnx(c"cannot get ehdr".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    (*ehdr).e_ident[EI_DATA] = GEN_ELF_ENDIAN;
    (*ehdr).e_ident[EI_CLASS] = GEN_ELF_CLASS;
    (*ehdr).e_machine = GEN_ELF_ARCH;
    (*ehdr).e_type = ET_DYN;
    (*ehdr).e_entry = GEN_ELF_TEXT_OFFSET;
    (*ehdr).e_version = EV_CURRENT;
    (*ehdr).e_shstrndx = if !unwinding.is_null() { 4 } else { 2 }; /* shdr index for section name */

    /*
     * setup program header
     */
    phdr = elf_newphdr(e, 1);
    (*phdr.add(0)).p_type = PT_LOAD;
    (*phdr.add(0)).p_offset = GEN_ELF_TEXT_OFFSET;
    (*phdr.add(0)).p_vaddr = GEN_ELF_TEXT_OFFSET;
    (*phdr.add(0)).p_paddr = GEN_ELF_TEXT_OFFSET;
    (*phdr.add(0)).p_filesz = csize as u64;
    (*phdr.add(0)).p_memsz = csize as u64;
    (*phdr.add(0)).p_flags = PF_X | PF_R;
    (*phdr.add(0)).p_align = 8;

    /*
     * setup text section
     */
    scn = elf_newscn(e);
    if scn.is_null() {
        warnx(c"cannot create section".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    d = elf_newdata(scn);
    if d.is_null() {
        warnx(c"cannot get new data".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    (*d).d_align = 16;
    (*d).d_off = 0;
    (*d).d_buf = code as *mut c_void;
    (*d).d_type = ELF_T_BYTE;
    (*d).d_size = csize as u64;
    (*d).d_version = EV_CURRENT;

    shdr = elf_getshdr(scn);
    if shdr.is_null() {
        warnx(c"cannot get section header".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    (*shdr).sh_name = 1;
    (*shdr).sh_type = SHT_PROGBITS;
    (*shdr).sh_addr = GEN_ELF_TEXT_OFFSET;
    (*shdr).sh_flags = SHF_EXECINSTR | SHF_ALLOC;
    (*shdr).sh_entsize = 0;

    blake2s_init(&mut ctx, size_of_val(&BNOTE.build_id));
    blake2s_update_tagged(&mut ctx, TAG_CODE, code, csize as usize);

    /*
     * Setup .eh_frame_hdr and .eh_frame
     */
    if !unwinding.is_null() {
        eh_frame_base_offset = ALIGN_8(GEN_ELF_TEXT_OFFSET + csize as u64);
        retval = jit_add_eh_frame_info(
            e,
            unwinding,
            unwinding_header_size,
            unwinding_size,
            eh_frame_base_offset,
        );
        if retval != 0 {
            goto_error(e, strsym, retval);
            return retval;
        }
        retval = -1;
    }

    /*
     * setup section headers string table
     */
    scn = elf_newscn(e);
    if scn.is_null() {
        warnx(c"cannot create section".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    d = elf_newdata(scn);
    if d.is_null() {
        warnx(c"cannot get new data".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    (*d).d_align = 1;
    (*d).d_off = 0;
    (*d).d_buf = SHD_STRING_TABLE.as_mut_ptr() as *mut c_void;
    (*d).d_type = ELF_T_BYTE;
    (*d).d_size = size_of_val(&SHD_STRING_TABLE) as u64;
    (*d).d_version = EV_CURRENT;

    shdr = elf_getshdr(scn);
    if shdr.is_null() {
        warnx(c"cannot get section header".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    (*shdr).sh_name = 7; /* offset of '.shstrtab' in shd_string_table */
    (*shdr).sh_type = SHT_STRTAB;
    (*shdr).sh_flags = 0;
    (*shdr).sh_entsize = 0;

    /*
     * setup symtab section
     */
    SYMTAB[1].st_size = csize as u64;
    SYMTAB[1].st_value = GEN_ELF_TEXT_OFFSET;

    scn = elf_newscn(e);
    if scn.is_null() {
        warnx(c"cannot create section".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    d = elf_newdata(scn);
    if d.is_null() {
        warnx(c"cannot get new data".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    (*d).d_align = 8;
    (*d).d_off = 0;
    (*d).d_buf = SYMTAB.as_mut_ptr() as *mut c_void;
    (*d).d_type = ELF_T_SYM;
    (*d).d_size = size_of_val(&SYMTAB) as u64;
    (*d).d_version = EV_CURRENT;

    shdr = elf_getshdr(scn);
    if shdr.is_null() {
        warnx(c"cannot get section header".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    (*shdr).sh_name = 17; /* offset of '.symtab' in shd_string_table */
    (*shdr).sh_type = SHT_SYMTAB;
    (*shdr).sh_flags = 0;
    (*shdr).sh_entsize = size_of::<Elf_Sym>() as u64;
    (*shdr).sh_link = if !unwinding.is_null() { 6 } else { 4 }; /* index of .strtab section */

    blake2s_update_tagged(
        &mut ctx,
        TAG_SYMTAB,
        SYMTAB.as_mut_ptr() as *const c_void,
        size_of_val(&SYMTAB),
    );

    /*
     * setup symbols string table
     * 2 = 1 for 0 in 1st entry, 1 for the 0 at end of symbol for 2nd entry
     */
    symlen = (2 + strlen(sym)) as c_int;
    strsym = calloc(1, symlen as usize) as *mut c_char;
    if strsym.is_null() {
        warnx(c"cannot allocate strsym".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }
    strcpy(strsym.add(1), sym);

    scn = elf_newscn(e);
    if scn.is_null() {
        warnx(c"cannot create section".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    d = elf_newdata(scn);
    if d.is_null() {
        warnx(c"cannot get new data".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    (*d).d_align = 1;
    (*d).d_off = 0;
    (*d).d_buf = strsym as *mut c_void;
    (*d).d_type = ELF_T_BYTE;
    (*d).d_size = symlen as u64;
    (*d).d_version = EV_CURRENT;

    shdr = elf_getshdr(scn);
    if shdr.is_null() {
        warnx(c"cannot get section header".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    (*shdr).sh_name = 25; /* offset in shd_string_table */
    (*shdr).sh_type = SHT_STRTAB;
    (*shdr).sh_flags = 0;
    (*shdr).sh_entsize = 0;

    blake2s_update_tagged(&mut ctx, TAG_STRSYM, strsym as *const c_void, symlen as usize);

    /*
     * setup build-id section
     */
    scn = elf_newscn(e);
    if scn.is_null() {
        warnx(c"cannot create section".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    d = elf_newdata(scn);
    if d.is_null() {
        warnx(c"cannot get new data".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    /*
     * build-id generation
     */
    blake2s_final(&mut ctx, BNOTE.build_id.as_mut_ptr());
    BNOTE.desc.namesz = size_of_val(&BNOTE.name) as c_uint; /* must include 0 termination */
    BNOTE.desc.descsz = size_of_val(&BNOTE.build_id) as c_uint;
    BNOTE.desc.type_ = NT_GNU_BUILD_ID;
    strcpy(BNOTE.name.as_mut_ptr(), c"GNU".as_ptr());

    (*d).d_align = 4;
    (*d).d_off = 0;
    (*d).d_buf = &mut BNOTE as *mut buildid_note as *mut c_void;
    (*d).d_type = ELF_T_BYTE;
    (*d).d_size = size_of_val(&BNOTE) as u64;
    (*d).d_version = EV_CURRENT;

    shdr = elf_getshdr(scn);
    if shdr.is_null() {
        warnx(c"cannot get section header".as_ptr());
        goto_error(e, strsym, retval);
        return retval;
    }

    (*shdr).sh_name = 33; /* offset in shd_string_table */
    (*shdr).sh_type = SHT_NOTE;
    (*shdr).sh_addr = 0x0;
    (*shdr).sh_flags = SHF_ALLOC;
    (*shdr).sh_size = size_of_val(&BNOTE) as u64;
    (*shdr).sh_entsize = 0;

    #[cfg(HAVE_LIBDW_SUPPORT)]
    {
        if !debug.is_null() && nr_debug_entries != 0 {
            retval = jit_add_debug_info(e, load_addr, debug, nr_debug_entries);
            if retval != 0 {
                goto_error(e, strsym, retval);
                return retval;
            }
        } else if elf_update(e, ELF_C_WRITE) < 0 {
            warnx(c"elf_update 4 failed".as_ptr());
            goto_error(e, strsym, retval);
            return retval;
        }
    }
    #[cfg(not(HAVE_LIBDW_SUPPORT))]
    {
        let _ = load_addr;
        let _ = debug;
        let _ = nr_debug_entries;
        if elf_update(e, ELF_C_WRITE) < 0 {
            warnx(c"elf_update 4 failed".as_ptr());
            goto_error(e, strsym, retval);
            return retval;
        }
    }

    retval = 0;
    goto_error(e, strsym, retval);
    retval
}

unsafe fn size_of_val<T>(val: &T) -> usize {
    let _ = val;
    size_of::<T>()
}

unsafe fn goto_error(e: *mut Elf, strsym: *mut c_char, _retval: c_int) {
    void_elf_end(e);
    free(strsym as *mut c_void);
}

unsafe fn void_elf_end(e: *mut Elf) {
    let _ = elf_end(e);
}
