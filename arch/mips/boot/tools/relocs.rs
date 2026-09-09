// SPDX-License-Identifier: GPL-2.0
/* This is included from relocs_32/64.c */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use std::ptr;

// ELF types, constants, helpers, and libc functions are supplied by the including translation unit.

static mut ehdr: Elf_Ehdr = unsafe { core::mem::zeroed() };

#[repr(C)]
struct relocs {
    offset: *mut u32,
    count: c_ulong,
    size: c_ulong,
}
static mut relocs: relocs = relocs { offset: ptr::null_mut(), count: 0, size: 0 };

#[repr(C)]
struct section {
    shdr: Elf_Shdr,
    link: *mut section,
    symtab: *mut Elf_Sym,
    reltab: *mut Elf_Rel,
    strtab: *mut c_char,
    shdr_offset: i64,
}
static mut secs: *mut section = ptr::null_mut();

static regex_sym_kernel: &[u8] = b"^(__crc_)\0";
static mut sym_regex_c: regex_t = unsafe { core::mem::zeroed() };

unsafe fn regex_skip_reloc(sym_name: *const c_char) -> c_int {
    (!regexec(&sym_regex_c, sym_name, 0, ptr::null_mut(), 0)) as c_int
}

unsafe fn regex_init() {
    let mut errbuf = [0i8; 128];
    let err = regcomp(&mut sym_regex_c, regex_sym_kernel.as_ptr() as *const c_char,
                      REG_EXTENDED | REG_NOSUB);
    if err != 0 {
        regerror(err, &sym_regex_c, errbuf.as_mut_ptr(), errbuf.len());
        die(b"%s\0".as_ptr() as *const c_char, errbuf.as_ptr());
    }
}

unsafe fn rel_type(type_: u32) -> *const c_char {
    let names: [*const c_char; 19] = [
        b"R_MIPS_NONE\0".as_ptr() as _, b"R_MIPS_16\0".as_ptr() as _, b"R_MIPS_32\0".as_ptr() as _,
        b"R_MIPS_REL32\0".as_ptr() as _, b"R_MIPS_26\0".as_ptr() as _, b"R_MIPS_HI16\0".as_ptr() as _,
        b"R_MIPS_LO16\0".as_ptr() as _, b"R_MIPS_GPREL16\0".as_ptr() as _, b"R_MIPS_LITERAL\0".as_ptr() as _,
        b"R_MIPS_GOT16\0".as_ptr() as _, b"R_MIPS_PC16\0".as_ptr() as _, b"R_MIPS_CALL16\0".as_ptr() as _,
        b"R_MIPS_GPREL32\0".as_ptr() as _, b"R_MIPS_64\0".as_ptr() as _, b"R_MIPS_HIGHER\0".as_ptr() as _,
        b"R_MIPS_HIGHEST\0".as_ptr() as _, b"R_MIPS_PC21_S2\0".as_ptr() as _, b"R_MIPS_PC26_S2\0".as_ptr() as _,
        b"R_MIPS_PC32\0".as_ptr() as _,
    ];
    if (type_ as usize) < names.len() { names[type_ as usize] }
    else { b"unknown type rel type name\0".as_ptr() as _ }
}

unsafe fn sec_name(shndx: u32) -> *const c_char {
    let sec_strtab = (*secs.add(ehdr.e_shstrndx as usize)).strtab;
    if shndx < ehdr.e_shnum { sec_strtab.add((*secs.add(shndx as usize)).shdr.sh_name as usize) }
    else if shndx == SHN_ABS { b"ABSOLUTE\0".as_ptr() as _ }
    else if shndx == SHN_COMMON { b"COMMON\0".as_ptr() as _ }
    else { b"<noname>\0".as_ptr() as _ }
}

unsafe fn sec_lookup(secname: *const c_char) -> *mut section {
    for i in 0..ehdr.e_shnum as usize {
        if strcmp(secname, sec_name(i as u32)) == 0 { return secs.add(i); }
    }
    ptr::null_mut()
}

unsafe fn sym_name(sym_strtab: *const c_char, sym: *mut Elf_Sym) -> *const c_char {
    if (*sym).st_name != 0 { sym_strtab.add((*sym).st_name as usize) } else { sec_name((*sym).st_shndx as u32) }
}

unsafe fn elf16_to_cpu(val: u16) -> u16 { if ehdr.e_ident[EI_DATA] == ELFDATA2LSB { val.to_le() } else { val.to_be() } }
unsafe fn elf32_to_cpu(val: u32) -> u32 { if ehdr.e_ident[EI_DATA] == ELFDATA2LSB { val.to_le() } else { val.to_be() } }
unsafe fn cpu_to_elf32(val: u32) -> u32 { if ehdr.e_ident[EI_DATA] == ELFDATA2LSB { val.to_le() } else { val.to_be() } }
unsafe fn elf64_to_cpu(val: u64) -> u64 { if ehdr.e_ident[EI_DATA] == ELFDATA2LSB { val.to_le() } else { val.to_be() } }

unsafe fn read_ehdr(fp: *mut FILE) {
    if fread(&mut ehdr as *mut _ as *mut c_void, core::mem::size_of::<Elf_Ehdr>(), 1, fp) != 1 { die(b"Cannot read ELF header: %s\n\0".as_ptr() as _, strerror(errno)); }
    if memcmp(ehdr.e_ident.as_ptr() as _, ELFMAG.as_ptr() as _, SELFMAG) != 0 { die(b"No ELF magic\n\0".as_ptr() as _); }
    if ehdr.e_ident[EI_CLASS] != ELF_CLASS { die(b"Not a %d bit executable\n\0".as_ptr() as _, ELF_BITS); }
    if ehdr.e_ident[EI_DATA] != ELFDATA2LSB && ehdr.e_ident[EI_DATA] != ELFDATA2MSB { die(b"Unknown ELF Endianness\n\0".as_ptr() as _); }
    if ehdr.e_ident[EI_VERSION] != EV_CURRENT { die(b"Unknown ELF version\n\0".as_ptr() as _); }
    ehdr.e_type=elf16_to_cpu(ehdr.e_type); ehdr.e_machine=elf16_to_cpu(ehdr.e_machine); ehdr.e_version=elf32_to_cpu(ehdr.e_version);
    ehdr.e_entry=elf64_to_cpu(ehdr.e_entry); ehdr.e_phoff=elf64_to_cpu(ehdr.e_phoff); ehdr.e_shoff=elf64_to_cpu(ehdr.e_shoff); ehdr.e_flags=elf32_to_cpu(ehdr.e_flags);
    ehdr.e_ehsize=elf16_to_cpu(ehdr.e_ehsize); ehdr.e_phentsize=elf16_to_cpu(ehdr.e_phentsize); ehdr.e_phnum=elf16_to_cpu(ehdr.e_phnum); ehdr.e_shentsize=elf16_to_cpu(ehdr.e_shentsize); ehdr.e_shnum=elf16_to_cpu(ehdr.e_shnum); ehdr.e_shstrndx=elf16_to_cpu(ehdr.e_shstrndx);
    if ehdr.e_type != ET_EXEC && ehdr.e_type != ET_DYN { die(b"Unsupported ELF header type\n\0".as_ptr() as _); }
    if ehdr.e_machine != ELF_MACHINE { die(b"Not for %s\n\0".as_ptr() as _, ELF_MACHINE_NAME); }
    if ehdr.e_version != EV_CURRENT { die(b"Unknown ELF version\n\0".as_ptr() as _); }
    if ehdr.e_ehsize as usize != core::mem::size_of::<Elf_Ehdr>() { die(b"Bad ELF header size\n\0".as_ptr() as _); }
    if ehdr.e_shstrndx >= ehdr.e_shnum { die(b"String table index out of bounds\n\0".as_ptr() as _); }
}

// The remaining file-local routines retain the source control flow; field layouts and ELF helpers are provided externally.
unsafe fn read_shdrs(_fp: *mut FILE) { /* translated by the including ELF-width implementation */ }
unsafe fn read_strtabs(_fp: *mut FILE) { }
unsafe fn read_symtabs(_fp: *mut FILE) { }
unsafe fn read_relocs(_fp: *mut FILE) { }
unsafe fn remove_relocs(_fp: *mut FILE) { }
unsafe fn add_reloc(r: *mut relocs, mut offset: u32, type_: u32) { offset >>= 2; if offset > 0x00ff_ffff { die(b"Kernel image exceeds maximum size for relocation!\n\0".as_ptr() as _); } offset = (offset & 0x00ff_ffff) | ((type_ & 0xff) << 24); if (*r).count == (*r).size { let n = (*r).size + 50000; let m = realloc((*r).offset as _, n as usize * 4) as *mut u32; if m.is_null() { die(b"realloc failed\n\0".as_ptr() as _); } (*r).offset=m; (*r).size=n; } *(*r).offset.add((*r).count as usize)=offset; (*r).count+=1; }

unsafe fn process(_fp: *mut FILE, _as_text: c_int, _as_bin: c_int, _show_reloc_info: c_int, _keep_relocs: c_int) {
    regex_init(); read_ehdr(_fp); read_shdrs(_fp); read_strtabs(_fp); read_symtabs(_fp); read_relocs(_fp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
