// Translated from elf-parse.c.
// The declarations below are supplied by the corresponding ELF parser header
// and other translation units.

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};

extern "C" {
    static mut elf_parser: elf_funcs;

    fn rle(p: *const c_void) -> u8;
    fn r2le(p: *const c_void) -> u16;
    fn r8le(p: *const c_void) -> u64;
    fn wle(p: *mut c_void, v: u32);
    fn w8le(p: *mut c_void, v: u64);
    fn rbe(p: *const c_void) -> u8;
    fn r2be(p: *const c_void) -> u16;
    fn r8be(p: *const c_void) -> u64;
    fn wbe(p: *mut c_void, v: u32);
    fn w8be(p: *mut c_void, v: u64);

    fn ehdr32_shoff(ehdr: *const c_void) -> u32;
    fn ehdr32_shentsize(ehdr: *const c_void) -> u16;
    fn ehdr32_shstrndx(ehdr: *const c_void) -> u16;
    fn ehdr32_shnum(ehdr: *const c_void) -> u16;
    fn shdr32_addr(shdr: *const c_void) -> u32;
    fn shdr32_offset(shdr: *const c_void) -> u32;
    fn shdr32_link(shdr: *const c_void) -> u32;
    fn shdr32_size(shdr: *const c_void) -> u32;
    fn shdr32_name(shdr: *const c_void) -> u32;
    fn shdr32_type(shdr: *const c_void) -> u32;
    fn shdr32_entsize(shdr: *const c_void) -> u32;
    fn sym32_type(sym: *const c_void) -> u8;
    fn sym32_name(sym: *const c_void) -> u32;
    fn sym32_value(sym: *const c_void) -> u32;
    fn sym32_shndx(sym: *const c_void) -> u16;
    fn rela32_offset(rela: *const c_void) -> u32;
    fn rela32_info(rela: *const c_void) -> u32;
    fn rela32_addend(rela: *const c_void) -> i32;
    fn rela32_write_addend(rela: *mut c_void, value: i32);

    fn ehdr64_shoff(ehdr: *const c_void) -> u64;
    fn ehdr64_shentsize(ehdr: *const c_void) -> u16;
    fn ehdr64_shstrndx(ehdr: *const c_void) -> u16;
    fn ehdr64_shnum(ehdr: *const c_void) -> u16;
    fn shdr64_addr(shdr: *const c_void) -> u64;
    fn shdr64_offset(shdr: *const c_void) -> u64;
    fn shdr64_link(shdr: *const c_void) -> u32;
    fn shdr64_size(shdr: *const c_void) -> u64;
    fn shdr64_name(shdr: *const c_void) -> u32;
    fn shdr64_type(shdr: *const c_void) -> u32;
    fn shdr64_entsize(shdr: *const c_void) -> u64;
    fn sym64_type(sym: *const c_void) -> u8;
    fn sym64_name(sym: *const c_void) -> u32;
    fn sym64_value(sym: *const c_void) -> u64;
    fn sym64_shndx(sym: *const c_void) -> u16;
    fn rela64_offset(rela: *const c_void) -> u64;
    fn rela64_info(rela: *const c_void) -> u64;
    fn rela64_addend(rela: *const c_void) -> i64;
    fn rela64_write_addend(rela: *mut c_void, value: i64);

    fn elf_unmap(addr: *mut c_void, size: usize);
}

#[repr(C)]
struct elf_funcs {
    r: unsafe extern "C" fn(*const c_void) -> u8,
    r2: unsafe extern "C" fn(*const c_void) -> u16,
    r8: unsafe extern "C" fn(*const c_void) -> u64,
    w: unsafe extern "C" fn(*mut c_void, u32),
    w8: unsafe extern "C" fn(*mut c_void, u64),
    ehdr_shoff: unsafe extern "C" fn(*const c_void) -> u64,
    ehdr_shentsize: unsafe extern "C" fn(*const c_void) -> u16,
    ehdr_shstrndx: unsafe extern "C" fn(*const c_void) -> u16,
    ehdr_shnum: unsafe extern "C" fn(*const c_void) -> u16,
    shdr_addr: unsafe extern "C" fn(*const c_void) -> u64,
    shdr_offset: unsafe extern "C" fn(*const c_void) -> u64,
    shdr_link: unsafe extern "C" fn(*const c_void) -> u32,
    shdr_size: unsafe extern "C" fn(*const c_void) -> u64,
    shdr_name: unsafe extern "C" fn(*const c_void) -> u32,
    shdr_type: unsafe extern "C" fn(*const c_void) -> u32,
    shdr_entsize: unsafe extern "C" fn(*const c_void) -> u64,
    sym_type: unsafe extern "C" fn(*const c_void) -> u8,
    sym_name: unsafe extern "C" fn(*const c_void) -> u32,
    sym_value: unsafe extern "C" fn(*const c_void) -> u64,
    sym_shndx: unsafe extern "C" fn(*const c_void) -> u16,
    rela_offset: unsafe extern "C" fn(*const c_void) -> u64,
    rela_info: unsafe extern "C" fn(*const c_void) -> u64,
    rela_addend: unsafe extern "C" fn(*const c_void) -> i64,
    rela_write_addend: unsafe extern "C" fn(*mut c_void, i64),
}

#[repr(C)]
struct Elf32Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u32,
    e_phoff: u32,
    e_shoff: u32,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
struct Elf64Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
union ElfEhdr {
    e32: Elf32Ehdr,
    e64: Elf64Ehdr,
}

const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const ELFDATA2LSB: u8 = 1;
const ELFDATA2MSB: u8 = 2;
const ELFCLASS32: u8 = 1;
const ELFCLASS64: u8 = 2;
const EV_CURRENT: u8 = 1;
const SELFMAG: usize = 4;
const ELFMAG: [u8; 4] = [0x7f, b'E', b'L', b'F'];

unsafe fn map_file(fname: *const c_char, size: *mut usize) -> *mut c_void {
    let fd = libc::open(fname, libc::O_RDWR);
    if fd < 0 {
        libc::perror(fname);
        return std::ptr::null_mut();
    }
    let mut sb: libc::stat = std::mem::zeroed();
    if libc::fstat(fd, &mut sb) < 0 {
        libc::perror(fname);
        libc::close(fd);
        return std::ptr::null_mut();
    }
    if (sb.st_mode & libc::S_IFMT) != libc::S_IFREG {
        libc::fprintf(libc::stderr, b"not a regular file: %s\0".as_ptr() as *const c_char, fname);
        libc::close(fd);
        return std::ptr::null_mut();
    }
    let addr = libc::mmap(std::ptr::null_mut(), sb.st_size as usize,
        libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0);
    libc::close(fd);
    if addr == libc::MAP_FAILED {
        libc::fprintf(libc::stderr, b"Could not mmap file: %s\n\0".as_ptr() as *const c_char, fname);
        return std::ptr::null_mut();
    }
    *size = sb.st_size as usize;
    addr
}

unsafe fn elf_parse(fname: *const c_char, addr: *mut c_void, types: u32) -> c_int {
    let ehdr = addr as *mut ElfEhdr;
    let data = (*ehdr).e32.e_ident[EI_DATA];
    match data {
        ELFDATA2LSB => { elf_parser.r = rle; elf_parser.r2 = r2le; elf_parser.r8 = r8le; elf_parser.w = wle; elf_parser.w8 = w8le; }
        ELFDATA2MSB => { elf_parser.r = rbe; elf_parser.r2 = r2be; elf_parser.r8 = r8be; elf_parser.w = wbe; elf_parser.w8 = w8be; }
        _ => { libc::fprintf(libc::stderr, b"unrecognized ELF data encoding %d: %s\n\0".as_ptr() as *const c_char, data as c_int, fname); return -1; }
    }
    if ELFMAG != (*ehdr).e32.e_ident[..SELFMAG] || (*ehdr).e32.e_ident[EI_VERSION] != EV_CURRENT { return -1; }
    let typ = (elf_parser.r2)(&(*ehdr).e32.e_type as *const _ as *const c_void);
    if ((1u32 << typ) & types) == 0 { return -1; }
    if (*ehdr).e32.e_ident[EI_CLASS] == ELFCLASS32 {
        if (elf_parser.r2)(&(*ehdr).e32.e_ehsize as *const _ as *const c_void) as usize != std::mem::size_of::<Elf32Ehdr>() || (elf_parser.r2)(&(*ehdr).e32.e_shentsize as *const _ as *const c_void) as usize != 40 { return -1; }
    } else if (*ehdr).e32.e_ident[EI_CLASS] == ELFCLASS64 {
        if (elf_parser.r2)(&(*ehdr).e64.e_ehsize as *const _ as *const c_void) as usize != std::mem::size_of::<Elf64Ehdr>() || (elf_parser.r2)(&(*ehdr).e64.e_shentsize as *const _ as *const c_void) as usize != 64 { return -1; }
    } else { return -1; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn elf_map_machine(addr: *mut c_void) -> c_int { (elf_parser.r2)(&(*(addr as *mut ElfEhdr)).e32.e_machine as *const _ as *const c_void) as c_int }

#[no_mangle]
pub unsafe extern "C" fn elf_map_long_size(addr: *mut c_void) -> c_int { if (*(addr as *mut ElfEhdr)).e32.e_ident[EI_CLASS] == ELFCLASS32 { 4 } else { 8 } }

#[no_mangle]
pub unsafe extern "C" fn elf_map(fname: *const c_char, size: *mut usize, types: u32) -> *mut c_void {
    let addr = map_file(fname, size);
    if addr.is_null() { return std::ptr::null_mut(); }
    if elf_parse(fname, addr, types) < 0 { elf_unmap(addr, *size); return std::ptr::null_mut(); }
    addr
}

#[no_mangle]
pub unsafe extern "C" fn elf_unmap(addr: *mut c_void, size: usize) { libc::munmap(addr, size); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
