/* SPDX-License-Identifier: GPL-2.0 */

/* 32-bit ELF base types. */
pub type Elf32_Addr = u32;
pub type Elf32_Half = u16;
pub type Elf32_Off = u32;
pub type Elf32_Sword = i32;
pub type Elf32_Word = u32;

/* 64-bit ELF base types. */
pub type Elf64_Addr = u64;
pub type Elf64_Half = u16;
pub type Elf64_SHalf = i16;
pub type Elf64_Off = u64;
pub type Elf64_Sword = i32;
pub type Elf64_Word = u32;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;

/* These constants are for the segment types stored in the image headers */
pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7; /* Thread local storage segment */
pub const PT_LOOS: u32 = 0x60000000; /* OS-specific */
pub const PT_HIOS: u32 = 0x6fffffff; /* OS-specific */
pub const PT_LOPROC: u32 = 0x70000000;
pub const PT_HIPROC: u32 = 0x7fffffff;
pub const PT_GNU_EH_FRAME: u32 = 0x6474e550;
pub const PT_GNU_STACK: u32 = PT_LOOS + 0x474e551;

/* These constants define the different elf file types */
pub const ET_NONE: u32 = 0;
pub const ET_REL: u32 = 1;
pub const ET_EXEC: u32 = 2;
pub const ET_DYN: u32 = 3;
pub const ET_CORE: u32 = 4;
pub const ET_LOPROC: u32 = 0xff00;
pub const ET_HIPROC: u32 = 0xffff;

/* These constants define the various ELF target machines */
pub const EM_NONE: u32 = 0;
pub const EM_PPC: u32 = 20; /* PowerPC */
pub const EM_PPC64: u32 = 21; /* PowerPC64 */

pub const EI_NIDENT: usize = 16;

#[repr(C)]
pub struct elf32_hdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: Elf32_Half,
    pub e_machine: Elf32_Half,
    pub e_version: Elf32_Word,
    pub e_entry: Elf32_Addr, /* Entry point */
    pub e_phoff: Elf32_Off,
    pub e_shoff: Elf32_Off,
    pub e_flags: Elf32_Word,
    pub e_ehsize: Elf32_Half,
    pub e_phentsize: Elf32_Half,
    pub e_phnum: Elf32_Half,
    pub e_shentsize: Elf32_Half,
    pub e_shnum: Elf32_Half,
    pub e_shstrndx: Elf32_Half,
}
pub type Elf32_Ehdr = elf32_hdr;

#[repr(C)]
pub struct elf64_hdr {
    pub e_ident: [u8; 16], /* ELF "magic number" */
    pub e_type: Elf64_Half,
    pub e_machine: Elf64_Half,
    pub e_version: Elf64_Word,
    pub e_entry: Elf64_Addr, /* Entry point virtual address */
    pub e_phoff: Elf64_Off, /* Program header table file offset */
    pub e_shoff: Elf64_Off, /* Section header table file offset */
    pub e_flags: Elf64_Word,
    pub e_ehsize: Elf64_Half,
    pub e_phentsize: Elf64_Half,
    pub e_phnum: Elf64_Half,
    pub e_shentsize: Elf64_Half,
    pub e_shnum: Elf64_Half,
    pub e_shstrndx: Elf64_Half,
}
pub type Elf64_Ehdr = elf64_hdr;

/* These constants define the permissions on sections in the program
   header, p_flags. */
pub const PF_R: u32 = 0x4;
pub const PF_W: u32 = 0x2;
pub const PF_X: u32 = 0x1;

#[repr(C)]
pub struct elf32_phdr {
    pub p_type: Elf32_Word,
    pub p_offset: Elf32_Off,
    pub p_vaddr: Elf32_Addr,
    pub p_paddr: Elf32_Addr,
    pub p_filesz: Elf32_Word,
    pub p_memsz: Elf32_Word,
    pub p_flags: Elf32_Word,
    pub p_align: Elf32_Word,
}

#[repr(C)]
pub struct elf64_phdr {
    pub p_type: Elf64_Word,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off, /* Segment file offset */
    pub p_vaddr: Elf64_Addr, /* Segment virtual address */
    pub p_paddr: Elf64_Addr, /* Segment physical address */
    pub p_filesz: Elf64_Xword, /* Segment size in file */
    pub p_memsz: Elf64_Xword, /* Segment size in memory */
    pub p_align: Elf64_Xword, /* Segment alignment, file & memory */
}

pub const EI_MAG0: usize = 0; /* e_ident[] indexes */
pub const EI_MAG1: usize = 1;
pub const EI_MAG2: usize = 2;
pub const EI_MAG3: usize = 3;
pub const EI_CLASS: usize = 4;
pub const EI_DATA: usize = 5;
pub const EI_VERSION: usize = 6;
pub const EI_OSABI: usize = 7;
pub const EI_PAD: usize = 8;

pub const ELFMAG0: u8 = 0x7f; /* EI_MAG */
pub const ELFMAG1: u8 = b'E';
pub const ELFMAG2: u8 = b'L';
pub const ELFMAG3: u8 = b'F';
pub const ELFMAG: &[u8; 4] = b"\x7fELF";
pub const SELFMAG: usize = 4;

pub const ELFCLASSNONE: u32 = 0; /* EI_CLASS */
pub const ELFCLASS32: u32 = 1;
pub const ELFCLASS64: u32 = 2;
pub const ELFCLASSNUM: u32 = 3;

pub const ELFDATANONE: u32 = 0; /* e_ident[EI_DATA] */
pub const ELFDATA2LSB: u32 = 1;
pub const ELFDATA2MSB: u32 = 2;

pub const EV_NONE: u32 = 0; /* e_version, EI_VERSION */
pub const EV_CURRENT: u32 = 1;
pub const EV_NUM: u32 = 2;

pub const ELFOSABI_NONE: u32 = 0;
pub const ELFOSABI_LINUX: u32 = 3;

#[repr(C)]
pub struct elf_info {
    pub loadsize: usize,
    pub memsize: usize,
    pub elfoffset: usize,
}

unsafe extern "C" {
    pub fn parse_elf64(hdr: *mut core::ffi::c_void, info: *mut elf_info) -> i32;
    pub fn parse_elf32(hdr: *mut core::ffi::c_void, info: *mut elf_info) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
