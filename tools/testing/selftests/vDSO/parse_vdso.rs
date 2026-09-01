/*
 * parse_vdso.c: Linux reference vDSO parser
 * Written by Andrew Lutomirski, 2011-2014.
 *
 * This code is meant to be linked in to various programs that run on Linux.
 * As such, it is available with as few restrictions as possible.  This file
 * is licensed under the Creative Commons Zero License, version 1.0,
 * available at http://creativecommons.org/publicdomain/zero/1.0/legalcode
 *
 * The vDSO is a regular ELF DSO that the kernel maps into user space when
 * it starts a program.  It works equally well in statically and dynamically
 * linked binaries.
 *
 * This code is tested on x86.  In principle it should work on any
 * architecture that has a vDSO.
 */

use core::ffi::{c_char, c_ulong, c_void};

/* Includes in the C source: stdbool.h, stdint.h, string.h, limits.h, elf.h,
 * and "parse_vdso.h".
 */

/* And here's the code. */
/* The C source allows ELF_BITS to be overridden.  File-locally, it defaults
 * from __SIZEOF_LONG__; use the Rust target pointer width for the same intent.
 */
#[cfg(target_pointer_width = "64")]
type ElfAddr = u64;
#[cfg(target_pointer_width = "32")]
type ElfAddr = u32;
#[cfg(target_pointer_width = "64")]
type ElfOff = u64;
#[cfg(target_pointer_width = "32")]
type ElfOff = u32;
#[cfg(target_pointer_width = "64")]
type ElfWord = u32;
#[cfg(target_pointer_width = "32")]
type ElfWord = u32;
#[cfg(target_pointer_width = "64")]
type ElfSword = i32;
#[cfg(target_pointer_width = "32")]
type ElfSword = i32;
#[cfg(target_pointer_width = "64")]
type ElfXword = u64;
#[cfg(target_pointer_width = "32")]
type ElfXword = u32;
#[cfg(target_pointer_width = "64")]
type ElfSxword = i64;
#[cfg(target_pointer_width = "32")]
type ElfSxword = i32;
#[cfg(target_pointer_width = "64")]
type ElfHalf = u16;
#[cfg(target_pointer_width = "32")]
type ElfHalf = u16;
type ElfVersym = ElfHalf;

#[cfg(target_arch = "s390x")]
type ElfHashEntry = ElfXword;
#[cfg(not(target_arch = "s390x"))]
type ElfHashEntry = ElfWord;

const EI_NIDENT: usize = 16;
const EI_CLASS: usize = 4;
const ELFCLASS32: u8 = 1;
const ELFCLASS64: u8 = 2;
#[cfg(target_pointer_width = "64")]
const ELF_CLASS: u8 = ELFCLASS64;
#[cfg(target_pointer_width = "32")]
const ELF_CLASS: u8 = ELFCLASS32;

const PT_LOAD: ElfWord = 1;
const PT_DYNAMIC: ElfWord = 2;

const DT_NULL: ElfSxword = 0;
const DT_HASH: ElfSxword = 4;
const DT_STRTAB: ElfSxword = 5;
const DT_SYMTAB: ElfSxword = 6;
const DT_GNU_HASH: ElfSxword = 0x6ffffef5;
const DT_VERSYM: ElfSxword = 0x6ffffff0;
const DT_VERDEF: ElfSxword = 0x6ffffffc;

const STB_GLOBAL: u8 = 1;
const STB_WEAK: u8 = 2;
const STT_FUNC: u8 = 2;
const SHN_UNDEF: ElfHalf = 0;
const VER_FLG_BASE: ElfHalf = 0x1;

#[cfg(target_pointer_width = "64")]
#[repr(C)]
struct ElfEhdr {
    e_ident: [u8; EI_NIDENT],
    e_type: ElfHalf,
    e_machine: ElfHalf,
    e_version: ElfWord,
    e_entry: ElfAddr,
    e_phoff: ElfOff,
    e_shoff: ElfOff,
    e_flags: ElfWord,
    e_ehsize: ElfHalf,
    e_phentsize: ElfHalf,
    e_phnum: ElfHalf,
    e_shentsize: ElfHalf,
    e_shnum: ElfHalf,
    e_shstrndx: ElfHalf,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
struct ElfEhdr {
    e_ident: [u8; EI_NIDENT],
    e_type: ElfHalf,
    e_machine: ElfHalf,
    e_version: ElfWord,
    e_entry: ElfAddr,
    e_phoff: ElfOff,
    e_shoff: ElfOff,
    e_flags: ElfWord,
    e_ehsize: ElfHalf,
    e_phentsize: ElfHalf,
    e_phnum: ElfHalf,
    e_shentsize: ElfHalf,
    e_shnum: ElfHalf,
    e_shstrndx: ElfHalf,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
struct ElfPhdr {
    p_type: ElfWord,
    p_flags: ElfWord,
    p_offset: ElfOff,
    p_vaddr: ElfAddr,
    p_paddr: ElfAddr,
    p_filesz: ElfXword,
    p_memsz: ElfXword,
    p_align: ElfXword,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
struct ElfPhdr {
    p_type: ElfWord,
    p_offset: ElfOff,
    p_vaddr: ElfAddr,
    p_paddr: ElfAddr,
    p_filesz: ElfWord,
    p_memsz: ElfWord,
    p_flags: ElfWord,
    p_align: ElfWord,
}

#[cfg(target_pointer_width = "64")]
#[repr(C)]
struct ElfSym {
    st_name: ElfWord,
    st_info: u8,
    st_other: u8,
    st_shndx: ElfHalf,
    st_value: ElfAddr,
    st_size: ElfXword,
}

#[cfg(target_pointer_width = "32")]
#[repr(C)]
struct ElfSym {
    st_name: ElfWord,
    st_value: ElfAddr,
    st_size: ElfWord,
    st_info: u8,
    st_other: u8,
    st_shndx: ElfHalf,
}

#[repr(C)]
union ElfDynUn {
    d_val: ElfXword,
    d_ptr: ElfAddr,
}

#[repr(C)]
struct ElfDyn {
    d_tag: ElfSxword,
    d_un: ElfDynUn,
}

#[repr(C)]
struct ElfVerdef {
    vd_version: ElfHalf,
    vd_flags: ElfHalf,
    vd_ndx: ElfHalf,
    vd_cnt: ElfHalf,
    vd_hash: ElfWord,
    vd_aux: ElfWord,
    vd_next: ElfWord,
}

#[repr(C)]
struct ElfVerdaux {
    vda_name: ElfWord,
    vda_next: ElfWord,
}

#[repr(C)]
struct vdso_info {
    valid: bool,

    /* Load information */
    load_addr: usize,
    load_offset: usize, /* load_addr - recorded vaddr */

    /* Symbol table */
    symtab: *mut ElfSym,
    symstrings: *const c_char,
    gnu_hash: *mut ElfWord,
    gnu_bucket: *mut ElfWord,
    bucket: *mut ElfHashEntry,
    chain: *mut ElfHashEntry,
    nbucket: ElfHashEntry,
    nchain: ElfHashEntry,

    /* Version table */
    versym: *mut ElfVersym,
    verdef: *mut ElfVerdef,
}

static mut vdso_info: vdso_info = vdso_info {
    valid: false,
    load_addr: 0,
    load_offset: 0,
    symtab: core::ptr::null_mut(),
    symstrings: core::ptr::null(),
    gnu_hash: core::ptr::null_mut(),
    gnu_bucket: core::ptr::null_mut(),
    bucket: core::ptr::null_mut(),
    chain: core::ptr::null_mut(),
    nbucket: 0,
    nchain: 0,
    versym: core::ptr::null_mut(),
    verdef: core::ptr::null_mut(),
};

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> i32;
}

fn elf64_st_type(val: u8) -> u8 {
    val & 0xf
}

fn elf64_st_bind(val: u8) -> u8 {
    val >> 4
}

/*
 * Straight from the ELF specification...and then tweaked slightly, in order to
 * avoid a few clang warnings.
 */
unsafe fn elf_hash(name: *const c_char) -> c_ulong {
    let mut h: c_ulong = 0;
    let mut g: c_ulong;
    let mut uch_name = name as *const u8;

    while *uch_name != 0 {
        h = (h << 4).wrapping_add(*uch_name as c_ulong);
        uch_name = uch_name.add(1);
        g = h & 0xf0000000;
        if g != 0 {
            h ^= g >> 24;
        }
        h &= !g;
    }
    h
}

unsafe fn gnu_hash(name: *const c_char) -> u32 {
    let mut s = name as *const u8;
    let mut h: u32 = 5381;

    while *s != 0 {
        h = h.wrapping_add(h.wrapping_mul(32).wrapping_add(*s as u32));
        s = s.add(1);
    }
    h
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vdso_init_from_sysinfo_ehdr(base: usize) {
    let mut i: usize;
    let mut found_vaddr = false;

    vdso_info.valid = false;

    vdso_info.load_addr = base;

    let hdr = base as *mut ElfEhdr;
    if (*hdr).e_ident[EI_CLASS] != ELF_CLASS {
        return; /* Wrong ELF class -- check ELF_BITS */
    }

    let pt = (vdso_info.load_addr + (*hdr).e_phoff as usize) as *mut ElfPhdr;
    let mut dyn_: *mut ElfDyn = core::ptr::null_mut();

    /*
     * We need two things from the segment table: the load offset
     * and the dynamic table.
     */
    i = 0;
    while i < (*hdr).e_phnum as usize {
        if (*pt.add(i)).p_type == PT_LOAD && !found_vaddr {
            found_vaddr = true;
            vdso_info.load_offset = base
                .wrapping_add((*pt.add(i)).p_offset as usize)
                .wrapping_sub((*pt.add(i)).p_vaddr as usize);
        } else if (*pt.add(i)).p_type == PT_DYNAMIC {
            dyn_ = (base + (*pt.add(i)).p_offset as usize) as *mut ElfDyn;
        }
        i += 1;
    }

    if !found_vaddr || dyn_.is_null() {
        return; /* Failed */
    }

    /*
     * Fish out the useful bits of the dynamic table.
     */
    let mut hash: *mut ElfHashEntry = core::ptr::null_mut();
    vdso_info.symstrings = core::ptr::null();
    vdso_info.gnu_hash = core::ptr::null_mut();
    vdso_info.symtab = core::ptr::null_mut();
    vdso_info.versym = core::ptr::null_mut();
    vdso_info.verdef = core::ptr::null_mut();
    i = 0;
    while (*dyn_.add(i)).d_tag != DT_NULL {
        match (*dyn_.add(i)).d_tag {
            DT_STRTAB => {
                vdso_info.symstrings =
                    ((*dyn_.add(i)).d_un.d_ptr as usize + vdso_info.load_offset) as *const c_char;
            }
            DT_SYMTAB => {
                vdso_info.symtab =
                    ((*dyn_.add(i)).d_un.d_ptr as usize + vdso_info.load_offset) as *mut ElfSym;
            }
            DT_HASH => {
                hash = ((*dyn_.add(i)).d_un.d_ptr as usize + vdso_info.load_offset)
                    as *mut ElfHashEntry;
            }
            DT_GNU_HASH => {
                vdso_info.gnu_hash =
                    ((*dyn_.add(i)).d_un.d_ptr as usize + vdso_info.load_offset) as *mut ElfWord;
            }
            DT_VERSYM => {
                vdso_info.versym =
                    ((*dyn_.add(i)).d_un.d_ptr as usize + vdso_info.load_offset) as *mut ElfVersym;
            }
            DT_VERDEF => {
                vdso_info.verdef =
                    ((*dyn_.add(i)).d_un.d_ptr as usize + vdso_info.load_offset) as *mut ElfVerdef;
            }
            _ => {}
        }
        i += 1;
    }
    if vdso_info.symstrings.is_null()
        || vdso_info.symtab.is_null()
        || (hash.is_null() && vdso_info.gnu_hash.is_null())
    {
        return; /* Failed */
    }

    if vdso_info.verdef.is_null() {
        vdso_info.versym = core::ptr::null_mut();
    }

    /* Parse the hash table header. */
    if !vdso_info.gnu_hash.is_null() {
        vdso_info.nbucket = *vdso_info.gnu_hash.add(0) as ElfHashEntry;
        /* The bucket array is located after the header (4 uint32) and the bloom
         * filter (size_t array of gnu_hash[2] elements).
         */
        vdso_info.gnu_bucket = vdso_info
            .gnu_hash
            .add(4 + core::mem::size_of::<usize>() / 4 * *vdso_info.gnu_hash.add(2) as usize);
    } else {
        vdso_info.nbucket = *hash.add(0);
        vdso_info.nchain = *hash.add(1);
        vdso_info.bucket = hash.add(2);
        vdso_info.chain = hash.add(vdso_info.nbucket as usize + 2);
    }

    /* That's all we need. */
    vdso_info.valid = true;
}

unsafe fn vdso_match_version(ver: ElfVersym, name: *const c_char, hash: ElfWord) -> bool {
    /*
     * This is a helper function to check if the version indexed by
     * ver matches name (which hashes to hash).
     *
     * The version definition table is a mess, and I don't know how
     * to do this in better than linear time without allocating memory
     * to build an index.  I also don't know why the table has
     * variable size entries in the first place.
     *
     * For added fun, I can't find a comprehensible specification of how
     * to parse all the weird flags in the table.
     *
     * So I just parse the whole table every time.
     */

    /* First step: find the version definition */
    let ver = ver & 0x7fff; /* Apparently bit 15 means "hidden" */
    let mut def = vdso_info.verdef;
    loop {
        if ((*def).vd_flags & VER_FLG_BASE) == 0 && ((*def).vd_ndx & 0x7fff) == ver {
            break;
        }

        if (*def).vd_next == 0 {
            return false; /* No definition. */
        }

        def = (def as *mut c_char).add((*def).vd_next as usize) as *mut ElfVerdef;
    }

    /* Now figure out whether it matches. */
    let aux = (def as *mut c_char).add((*def).vd_aux as usize) as *mut ElfVerdaux;
    (*def).vd_hash == hash
        && strcmp(name, vdso_info.symstrings.add((*aux).vda_name as usize)) == 0
}

unsafe fn check_sym(
    sym: *mut ElfSym,
    i: ElfWord,
    name: *const c_char,
    version: *const c_char,
    ver_hash: c_ulong,
) -> bool {
    /* Check for a defined global or weak function w/ right name. */
    if elf64_st_type((*sym).st_info) != STT_FUNC {
        return false;
    }
    if elf64_st_bind((*sym).st_info) != STB_GLOBAL && elf64_st_bind((*sym).st_info) != STB_WEAK {
        return false;
    }
    if strcmp(name, vdso_info.symstrings.add((*sym).st_name as usize)) != 0 {
        return false;
    }

    /* Check symbol version. */
    if !vdso_info.versym.is_null()
        && !vdso_match_version(*vdso_info.versym.add(i as usize), version, ver_hash as ElfWord)
    {
        return false;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vdso_sym(version: *const c_char, name: *const c_char) -> *mut c_void {
    let ver_hash: c_ulong;
    if !vdso_info.valid {
        return core::ptr::null_mut();
    }

    ver_hash = elf_hash(version);
    let mut i: ElfWord;

    if !vdso_info.gnu_hash.is_null() {
        let mut h1 = gnu_hash(name);
        let mut h2: u32;
        let mut hashval: *mut u32;

        i = *vdso_info
            .gnu_bucket
            .add((h1 as ElfHashEntry % vdso_info.nbucket) as usize);
        if i == 0 {
            return core::ptr::null_mut();
        }
        h1 |= 1;
        hashval = vdso_info
            .gnu_bucket
            .add(vdso_info.nbucket as usize)
            .add(i.wrapping_sub(*vdso_info.gnu_hash.add(1)) as usize);
        loop {
            let sym = vdso_info.symtab.add(i as usize);
            h2 = *hashval;
            hashval = hashval.add(1);
            if h1 == (h2 | 1) && check_sym(sym, i, name, version, ver_hash) {
                return (vdso_info.load_offset + (*sym).st_value as usize) as *mut c_void;
            }
            if (h2 & 1) != 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
    } else {
        i = *vdso_info
            .bucket
            .add((elf_hash(name) as ElfHashEntry % vdso_info.nbucket) as usize)
            as ElfWord;
        while i != 0 {
            let sym = vdso_info.symtab.add(i as usize);
            if (*sym).st_shndx != SHN_UNDEF && check_sym(sym, i, name, version, ver_hash) {
                return (vdso_info.load_offset + (*sym).st_value as usize) as *mut c_void;
            }
            i = *vdso_info.chain.add(i as usize) as ElfWord;
        }
    }

    core::ptr::null_mut()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
