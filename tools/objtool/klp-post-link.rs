// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Read the intermediate KLP reloc/symbol representations created by klp diff
 * and convert them to the proper format required by livepatch.  This needs to
 * run last to avoid linker wreckage.  Linkers don't tend to handle the "two
 * rela sections for a single base section" case very well, nor do they like
 * SHN_LIVEPATCH.
 *
 * This is the final tool in the livepatch module generation pipeline:
 *
 *   kernel builds -> objtool klp diff -> module link -> objtool klp post-link
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u64 = u64;

const O_RDWR: c_int = 0x0002;
const SHT_RELA: u32 = 4;
const SHF_ALLOC: u64 = 0x2;
const SHF_INFO_LINK: u64 = 0x40;
const SHF_RELA_LIVEPATCH: u64 = 0x0010_0000;
const SHN_LIVEPATCH: u16 = 0xff20;
const SEC_NAME_LEN: usize = 128;

const KLP_RELOCS_SEC: &[u8] = b"__klp_relocs\0";
const KLP_RELOCS_SEC_DOT: &[u8] = b"__klp_relocs.\0";
const KLP_RELOC_SEC_PREFIX: &[u8] = b".klp.rela.\0";

#[repr(C)]
pub struct Elf_Data {
    pub d_buf: *mut c_void,
}

#[repr(C)]
pub struct GElf_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

#[repr(C)]
pub struct GElf_Shdr {
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
pub struct elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct section {
    pub name: *const c_char,
    pub idx: u32,
    pub data: *mut Elf_Data,
    pub sh: GElf_Shdr,
    pub base: *mut section,
    pub rsec: *mut section,
}

#[repr(C)]
pub struct symbol {
    pub idx: u32,
    pub sec: *mut section,
    pub sym: GElf_Sym,
}

#[repr(C)]
pub struct reloc {
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct klp_reloc {
    pub sym: u64,
    pub offset: u64,
    pub type_: u32,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;

    fn sec_size(sec: *mut section) -> c_ulong;
    fn find_reloc_by_dest(elf: *mut elf, sec: *mut section, offset: c_ulong) -> *mut reloc;
    fn reloc_addend(reloc: *mut reloc) -> u64;
    fn snprintf_check(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn find_section_by_name(elf: *mut elf, name: *const c_char) -> *mut section;
    fn elf_create_section(
        elf: *mut elf,
        name: *const c_char,
        idx: c_int,
        entsize: c_ulong,
        sh_type: u32,
        addralign: c_ulong,
        flags: u64,
    ) -> *mut section;
    fn elf_rela_size(elf: *mut elf) -> c_ulong;
    fn elf_addr_size(elf: *mut elf) -> c_ulong;
    fn elf_create_reloc(
        elf: *mut elf,
        sec: *mut section,
        offset: c_ulong,
        sym: *mut symbol,
        addend: u64,
        type_: u32,
    ) -> *mut reloc;
    fn gelf_update_sym(data: *mut Elf_Data, ndx: c_int, sym: *mut GElf_Sym) -> c_int;
    fn set_reloc_type(elf: *mut elf, reloc: *mut reloc, type_: u32);
    fn elf_open_read(name: *const c_char, flags: c_int) -> *mut elf;
    fn elf_write(elf: *mut elf) -> c_int;
    fn elf_close(elf: *mut elf) -> c_int;

    fn ERROR(format: *const c_char, ...);
    fn ERROR_ELF(format: *const c_char, ...);
}

const fn offset_of_klp_reloc_offset() -> c_ulong {
    core::mem::offset_of!(klp_reloc, offset) as c_ulong
}

const fn offset_of_klp_reloc_sym() -> c_ulong {
    core::mem::offset_of!(klp_reloc, sym) as c_ulong
}

unsafe fn fix_klp_reloc_sec(
    elf: *mut elf,
    symtab: *mut section,
    klp_relocs: *mut section,
) -> c_int {
    /* section format: __klp_relocs.sec_objname */
    let sec_objname =
        unsafe { (*klp_relocs).name.add(strlen(KLP_RELOCS_SEC_DOT.as_ptr() as *const c_char)) };
    let nr_relocs = unsafe { sec_size(klp_relocs) as usize / size_of::<klp_reloc>() };

    for i in 0..nr_relocs {
        let klp_reloc: *mut klp_reloc;
        let klp_reloc_off: c_ulong;
        let mut sec: *mut section;
        let tmp: *mut section;
        let klp_rsec: *mut section;
        let offset: c_ulong;
        let mut reloc: *mut reloc;
        let mut rsec_name = [0 as c_char; SEC_NAME_LEN];
        let addend: u64;
        let sym: *mut symbol;
        let klp_sym: *mut symbol;

        klp_reloc_off = (i * size_of::<klp_reloc>()) as c_ulong;
        klp_reloc = unsafe { ((*(*klp_relocs).data).d_buf as *mut u8).add(klp_reloc_off as usize) }
            as *mut klp_reloc;

        /*
         * Read __klp_relocs[i]:
         */

        /* klp_reloc.sec_offset */
        reloc = unsafe {
            find_reloc_by_dest(
                elf,
                klp_relocs,
                klp_reloc_off.wrapping_add(offset_of_klp_reloc_offset()),
            )
        };
        if reloc.is_null() {
            unsafe {
                ERROR(
                    b"malformed %s section\0".as_ptr() as *const c_char,
                    (*klp_relocs).name,
                );
            }
            return -1;
        }

        sec = unsafe { (*(*reloc).sym).sec };
        offset = unsafe { reloc_addend(reloc) as c_ulong };

        /* klp_reloc.sym */
        reloc = unsafe {
            find_reloc_by_dest(
                elf,
                klp_relocs,
                klp_reloc_off.wrapping_add(offset_of_klp_reloc_sym()),
            )
        };
        if reloc.is_null() {
            unsafe {
                ERROR(
                    b"malformed %s section\0".as_ptr() as *const c_char,
                    (*klp_relocs).name,
                );
            }
            return -1;
        }

        klp_sym = unsafe { (*reloc).sym };
        addend = unsafe { reloc_addend(reloc) };

        /*
         * Create the KLP rela:
         */

        /* section format: .klp.rela.sec_objname.section_name */
        if unsafe {
            snprintf_check(
                rsec_name.as_mut_ptr(),
                SEC_NAME_LEN,
                b"%s%s.%s\0".as_ptr() as *const c_char,
                KLP_RELOC_SEC_PREFIX.as_ptr() as *const c_char,
                sec_objname,
                (*sec).name,
            )
        } != 0
        {
            return -1;
        }

        let mut found_klp_rsec = unsafe { find_section_by_name(elf, rsec_name.as_ptr()) };
        if found_klp_rsec.is_null() {
            found_klp_rsec = unsafe {
                elf_create_section(
                    elf,
                    rsec_name.as_ptr(),
                    0,
                    elf_rela_size(elf),
                    SHT_RELA,
                    elf_addr_size(elf),
                    SHF_ALLOC | SHF_INFO_LINK | SHF_RELA_LIVEPATCH,
                )
            };
            if found_klp_rsec.is_null() {
                return -1;
            }

            unsafe {
                (*found_klp_rsec).sh.sh_link = (*symtab).idx;
                (*found_klp_rsec).sh.sh_info = (*sec).idx;
                (*found_klp_rsec).base = sec;
            }
        }
        klp_rsec = found_klp_rsec;

        tmp = unsafe { (*sec).rsec };
        unsafe {
            (*sec).rsec = klp_rsec;
        }
        if unsafe {
            elf_create_reloc(
                elf,
                sec,
                offset,
                klp_sym,
                addend,
                (*klp_reloc).type_,
            )
        }
        .is_null()
        {
            return -1;
        }
        unsafe {
            (*sec).rsec = tmp;
        }

        /*
         * Fix up the corresponding KLP symbol:
         */

        unsafe {
            (*klp_sym).sym.st_shndx = SHN_LIVEPATCH;
        }
        if unsafe { gelf_update_sym((*symtab).data, (*klp_sym).idx as c_int, &mut (*klp_sym).sym) }
            == 0
        {
            unsafe {
                ERROR_ELF(b"gelf_update_sym\0".as_ptr() as *const c_char);
            }
            return -1;
        }

        /*
         * Disable the original non-KLP reloc by converting it to R_*_NONE:
         */

        reloc = unsafe { find_reloc_by_dest(elf, sec, offset) };
        sym = unsafe { (*reloc).sym };
        unsafe {
            (*sym).sym.st_shndx = SHN_LIVEPATCH;
            set_reloc_type(elf, reloc, 0);
        }
        if unsafe { gelf_update_sym((*symtab).data, (*sym).idx as c_int, &mut (*sym).sym) } == 0 {
            unsafe {
                ERROR_ELF(b"gelf_update_sym\0".as_ptr() as *const c_char);
            }
            return -1;
        }
    }

    0
}

unsafe fn fix_klp_relocs(elf: *mut elf) -> c_int {
    let symtab: *mut section;
    let mut sec: *mut section;

    symtab = unsafe { find_section_by_name(elf, b".symtab\0".as_ptr() as *const c_char) };
    if symtab.is_null() {
        unsafe {
            ERROR(b"missing .symtab\0".as_ptr() as *const c_char);
        }
        return -1;
    }

    /*
     * C source uses for_each_sec(elf, sec).  The iterator implementation is
     * provided by objtool headers outside this isolated translation unit.
     */
    unsafe {
        sec = ptr::null_mut();
        while {
            sec = for_each_sec_next(elf, sec);
            !sec.is_null()
        } {
            if strncmp(
                (*sec).name,
                KLP_RELOCS_SEC_DOT.as_ptr() as *const c_char,
                strlen(KLP_RELOCS_SEC_DOT.as_ptr() as *const c_char),
            ) != 0
            {
                continue;
            }

            if fix_klp_reloc_sec(elf, symtab, sec) != 0 {
                return -1;
            }
        }
    }

    0
}

unsafe extern "C" {
    fn for_each_sec_next(elf: *mut elf, previous: *mut section) -> *mut section;
}

/*
 * This runs on the livepatch module after all other linking has been done.  It
 * converts the intermediate __klp_relocs.* sections into proper KLP relocs to
 * be processed by livepatch.  This needs to run last to avoid linker wreckage.
 * Linkers don't tend to handle the "two rela sections for a single base
 * section" case very well, nor do they appreciate SHN_LIVEPATCH.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_klp_post_link(
    mut argc: c_int,
    mut argv: *mut *const c_char,
) -> c_int {
    let elf: *mut elf;

    argc -= 1;
    argv = unsafe { argv.add(1) };

    if argc != 1 {
        unsafe {
            fprintf(stderr, b"%d\n\0".as_ptr() as *const c_char, argc);
            fprintf(
                stderr,
                b"usage: objtool link <file.ko>\n\0".as_ptr() as *const c_char,
            );
        }
        return -1;
    }

    elf = unsafe { elf_open_read(*argv, O_RDWR) };
    if elf.is_null() {
        return -1;
    }

    if unsafe { fix_klp_relocs(elf) } != 0 {
        return -1;
    }

    if unsafe { elf_write(elf) } != 0 {
        return -1;
    }

    unsafe { elf_close(elf) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
