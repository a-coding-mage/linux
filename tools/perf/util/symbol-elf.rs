// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/symbol-elf.c.
// C include dependencies intentionally remain external to this isolated file:
// compress.h, dso.h, libbfd.h, map.h, maps.h, symbol.h, symsrc.h, machine.h,
// vdso.h, debug.h, util/copyfile.h, linux helpers, kallsyms, libelf/gelf.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type u8 = u8;
type u32 = u32;
type u64 = u64;
type GElf_Addr = u64;
type GElf_Half = u16;
type Elf32_Addr = u32;
type Elf64_Addr = u64;

const EM_AARCH64: c_int = 183; /* ARM 64 bit */
const EM_LOONGARCH: c_int = 258;
const NT_GNU_BUILD_ID: c_int = 3;
const STT_GNU_IFUNC: c_int = 10;
const BUILD_ID_SIZE: usize = 20;
const PATH_MAX: usize = 4096;
const BUFSIZ: usize = 8192;
const EI_DATA: usize = 5;
const EI_NIDENT: usize = 16;
const NR_ADDR: usize = 3;
const SDT_NOTE_IDX_LOC: usize = 0;
const SDT_NOTE_IDX_BASE: usize = 1;
const SDT_NOTE_IDX_REFCTR: usize = 2;

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_EXCL: c_int = 0o200;
const O_CLOEXEC: c_int = 0o2000000;
const SEEK_SET: c_int = 0;

const EFAULT: c_int = 14;
const EWOULDBLOCK: c_int = 11;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EBADF: c_int = 9;

const EM_ARM: GElf_Half = 40;
const EM_RISCV: GElf_Half = 243;
const EM_SPARC: GElf_Half = 2;
const EM_SPARCV9: GElf_Half = 43;
const EM_386: GElf_Half = 3;
const EM_X86_64: GElf_Half = 62;
const EM_PARISC: GElf_Half = 15;
const EM_ALPHA: GElf_Half = 0x9026;
const R_X86_64_IRELATIVE: u32 = 37;

const STT_NOTYPE: u8 = 0;
const STT_OBJECT: u8 = 1;
const STT_FUNC: u8 = 2;
const STB_GLOBAL: u8 = 1;
const STV_INTERNAL: u8 = 1;
const STV_HIDDEN: u8 = 2;
const SHN_UNDEF: u16 = 0;
const SHN_ABS: u16 = 0xfff1;
const SHN_LORESERVE: u16 = 0xff00;
const SHT_PROGBITS: u32 = 1;
const SHT_SYMTAB: u32 = 2;
const SHT_STRTAB: u32 = 3;
const SHT_RELA: u32 = 4;
const SHT_NOBITS: u32 = 8;
const SHT_REL: u32 = 9;
const SHT_DYNSYM: u32 = 11;
const SHT_NOTE: u32 = 7;
const SHF_ALLOC: u64 = 0x2;
const SHF_EXECINSTR: u64 = 0x4;
const PT_LOAD: u32 = 1;
const PF_X: u32 = 1;
const PF_W: u32 = 2;
const PF_R: u32 = 4;
const ET_REL: u16 = 1;
const ET_EXEC: u16 = 2;
const ET_DYN: u16 = 3;
const ELF_K_ELF: c_int = 3;
const ELFCLASSNONE: c_int = 0;
const ELFCLASS32: c_int = 1;
const ELFCLASS64: c_int = 2;
const ELF_C_READ: c_int = 1;
const ELF_C_WRITE: c_int = 2;
const PERF_ELF_C_READ_MMAP: c_int = ELF_C_READ;
const EV_CURRENT: c_uint = 1;
const ELF_T_EHDR: c_int = 1;
const ELF_T_PHDR: c_int = 2;
const ELF_T_ADDR: c_int = 3;
const DSO_LOAD_ERRNO__INVALID_ELF: c_int = -1000;
const DSO_LOAD_ERRNO__INTERNAL_ERROR: c_int = -1001;
const DSO_LOAD_ERRNO__CANNOT_READ_BUILDID: c_int = -1002;
const DSO_LOAD_ERRNO__MISMATCHING_BUILDID: c_int = -1003;
const DSO_BINARY_TYPE__GNU_DEBUGDATA: dso_binary_type = 1;
const DSO_SPACE__USER: c_int = 0;
const DSO__TYPE_UNKNOWN: dso_type = 0;
const DSO__TYPE_64BIT: dso_type = 1;
const DSO__TYPE_X32BIT: dso_type = 2;
const DSO__TYPE_32BIT: dso_type = 3;
const MAPPING_TYPE__DSO: c_int = 1;
const MAPPING_TYPE__IDENTITY: c_int = 2;

static SDT_BASE_SCN: &[u8] = b".stapsdt.base\0";
static SDT_PROBES_SCN: &[u8] = b".probes\0";
static SDT_NOTE_SCN: &[u8] = b".note.stapsdt\0";
static SDT_NOTE_NAME: &[u8] = b"stapsdt\0";
const SDT_NOTE_TYPE: u32 = 3;
static PERF_KCORE_EXTRACT: &[u8] = b"/tmp/perf-kcore-XXXXXX\0";

#[repr(C)] pub struct Elf { _private: [u8; 0] }
#[repr(C)] pub struct Elf_Scn { _private: [u8; 0] }
#[repr(C)] pub struct FILE { _private: [u8; 0] }
#[repr(C)] pub struct dso { _private: [u8; 0] }
#[repr(C)] pub struct map { _private: [u8; 0] }
#[repr(C)] pub struct maps { _private: [u8; 0] }
#[repr(C)] pub struct machine { pub dsos: c_int }
#[repr(C)] pub struct symbols { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct symbol {
    pub start: u64,
    pub end: u64,
    pub name: *mut c_char,
}
#[repr(C)] pub struct ref_reloc_sym {
    pub name: *mut c_char,
    pub addr: u64,
    pub unrelocated_addr: u64,
}
#[repr(C)] pub struct kmap {
    pub ref_reloc_sym: *mut ref_reloc_sym,
    pub kmaps: *mut maps,
}
#[repr(C)] pub struct build_id { pub size: size_t, pub data: [u8; BUILD_ID_SIZE] }
#[repr(C)] pub struct kmod_path { pub name: *mut c_char, pub comp: *mut c_void }
#[repr(C)] pub struct kcore_extract {
    pub kcore_filename: *const c_char,
    pub extract_filename: [c_char; PATH_MAX],
    pub addr: u64,
    pub len: u64,
    pub offs: off_t,
}
#[repr(C)] pub struct sdt_note {
    pub note_list: list_head,
    pub provider: *mut c_char,
    pub name: *mut c_char,
    pub args: *mut c_char,
    pub addr: sdt_addr,
    pub bit32: bool,
}
#[repr(C)] pub union sdt_addr { pub a64: [Elf64_Addr; NR_ADDR], pub a32: [Elf32_Addr; NR_ADDR] }
#[repr(C)] pub struct symbol_conf_t { pub ignore_vmlinux_buildid: bool }

type dso_binary_type = c_int;
type dso_type = c_int;
type Elf_Kind = c_int;
type mapfn_t = Option<unsafe extern "C" fn(u64, u64, u64, *mut c_void) -> c_int>;

#[repr(C)] #[derive(Copy, Clone)] pub struct GElf_Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: GElf_Half,
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
#[repr(C)] #[derive(Copy, Clone)] pub struct GElf_Shdr {
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
#[repr(C)] #[derive(Copy, Clone)] pub struct GElf_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct GElf_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct GElf_Rela { pub r_offset: u64, pub r_info: u64, pub r_addend: i64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct GElf_Rel { pub r_offset: u64, pub r_info: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct GElf_Nhdr { pub n_namesz: u32, pub n_descsz: u32, pub n_type: u32 }
#[repr(C)] pub struct Elf_Data {
    pub d_buf: *mut c_void,
    pub d_type: c_int,
    pub d_version: c_uint,
    pub d_size: size_t,
    pub d_off: i64,
    pub d_align: size_t,
}
#[repr(C)] pub struct symsrc {
    pub name: *mut c_char,
    pub fd: c_int,
    pub elf: *mut Elf,
    pub ehdr: GElf_Ehdr,
    pub symtab_idx: size_t,
    pub symtab: *mut Elf_Scn,
    pub symshdr: GElf_Shdr,
    pub dynsym_idx: size_t,
    pub dynsym: *mut Elf_Scn,
    pub dynshdr: GElf_Shdr,
    pub opdidx: size_t,
    pub opdsec: *mut Elf_Scn,
    pub opdshdr: GElf_Shdr,
    pub is_64_bit: bool,
    pub adjust_symbols: bool,
    pub type_: dso_binary_type,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut page_size: size_t;
    static mut symbol_conf: symbol_conf_t;

    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn unlink(path: *const c_char) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);
    fn qsort_r(base: *mut c_void, nmemb: size_t, size: size_t, compar: Option<unsafe extern "C" fn(*const c_void, *const c_void, *mut c_void) -> c_int>, arg: *mut c_void);
    fn bsearch(key: *const c_void, base: *const c_void, nmemb: size_t, size: size_t, compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>) -> *mut c_void;
    fn mkostemp(template: *mut c_char, flags: c_int) -> c_int;
    fn fmemopen(buf: *mut c_void, size: size_t, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;

    fn elf_begin(fd: c_int, cmd: c_int, ref_: *mut Elf) -> *mut Elf;
    fn elf_end(elf: *mut Elf) -> c_int;
    fn elf_kind(elf: *mut Elf) -> Elf_Kind;
    fn elf_nextscn(elf: *mut Elf, scn: *mut Elf_Scn) -> *mut Elf_Scn;
    fn elf_getscn(elf: *mut Elf, index: size_t) -> *mut Elf_Scn;
    fn elf_getdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
    fn elf_rawdata(scn: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
    fn elf_strptr(elf: *mut Elf, section: size_t, offset: size_t) -> *mut c_char;
    fn elf_errmsg(err: c_int) -> *const c_char;
    fn elf_version(ver: c_uint) -> c_uint;
    fn elf_update(elf: *mut Elf, cmd: c_int) -> off_t;
    fn elf_getident(elf: *mut Elf, nbytes: *mut size_t) -> *mut c_char;

    fn gelf_getehdr(elf: *mut Elf, dst: *mut GElf_Ehdr) -> *mut GElf_Ehdr;
    fn gelf_getshdr(scn: *mut Elf_Scn, dst: *mut GElf_Shdr) -> *mut GElf_Shdr;
    fn gelf_getphdr(elf: *mut Elf, ndx: c_int, dst: *mut GElf_Phdr) -> *mut GElf_Phdr;
    fn gelf_getsym(data: *mut Elf_Data, ndx: c_int, dst: *mut GElf_Sym) -> *mut GElf_Sym;
    fn gelf_getrela(data: *mut Elf_Data, ndx: c_int, dst: *mut GElf_Rela) -> *mut GElf_Rela;
    fn gelf_getrel(data: *mut Elf_Data, ndx: c_int, dst: *mut GElf_Rel) -> *mut GElf_Rel;
    fn gelf_getclass(elf: *mut Elf) -> c_int;
    fn gelf_newehdr(elf: *mut Elf, class: c_int) -> *mut c_void;
    fn gelf_update_ehdr(elf: *mut Elf, src: *mut GElf_Ehdr) -> c_int;
    fn gelf_newphdr(elf: *mut Elf, count: size_t) -> *mut c_void;
    fn gelf_update_phdr(elf: *mut Elf, ndx: c_int, src: *mut GElf_Phdr) -> c_int;
    fn gelf_fsize(elf: *mut Elf, type_: c_int, count: size_t, version: c_uint) -> size_t;
    fn gelf_xlatetom(elf: *mut Elf, dst: *mut Elf_Data, src: *const Elf_Data, encode: c_uint) -> *mut Elf_Data;
    fn gelf_getnote(data: *mut Elf_Data, offset: size_t, nhdr: *mut GElf_Nhdr, name_off: *mut size_t, desc_off: *mut size_t) -> size_t;
    fn elf_getphdrnum(elf: *mut Elf, dst: *mut size_t) -> c_int;
    fn elf_getshdrstrndx(elf: *mut Elf, dst: *mut size_t) -> c_int;

    fn libbfd__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
    fn libbfd_filename__read_debuglink(filename: *const c_char, debuglink: *mut c_char, size: size_t) -> c_int;
    fn is_regular_file(filename: *const c_char) -> bool;
    fn kmod_path__parse(m: *mut kmod_path, filename: *const c_char) -> c_int;
    fn filename__decompress(filename: *const c_char, path: *mut c_char, len: size_t, comp: *mut c_void, error: *mut c_int) -> c_int;
    fn dso__needs_decompress(dso: *mut dso) -> bool;
    fn dso__decompress_kmodule_fd(dso: *mut dso, name: *const c_char) -> c_int;
    fn dso__symtab_type(dso: *mut dso) -> dso_binary_type;
    fn dso__swap_init(dso: *mut dso, data: u8) -> c_int;
    fn dso__has_build_id(dso: *mut dso) -> bool;
    fn build_id__init(bid: *mut build_id, data: *const u8, size: c_int);
    fn dso__build_id_equal(dso: *mut dso, bid: *const build_id) -> bool;
    fn dso__load_errno(dso: *mut dso) -> *mut c_int;
    fn dso__kernel(dso: *mut dso) -> c_int;
    fn dso__rel(dso: *mut dso) -> bool;
    fn dso__adjust_symbols(dso: *mut dso) -> bool;
    fn dso__set_adjust_symbols(dso: *mut dso, v: bool);
    fn dso__set_symtab_type(dso: *mut dso, v: dso_binary_type);
    fn dso__set_is_64_bit(dso: *mut dso, v: bool);
    fn dso__set_rel(dso: *mut dso, v: bool);
    fn dso__set_text_offset(dso: *mut dso, v: u64);
    fn dso__set_text_end(dso: *mut dso, v: u64);
    fn dso__text_offset(dso: *mut dso) -> u64;
    fn dso__is_vdso(dso: *mut dso) -> bool;
    fn dso__find_symbol_nocache(dso: *mut dso, addr: u64) -> *mut symbol;
    fn dso__demangle_sym(dso: *mut dso, kmodule: c_int, name: *const c_char) -> *mut c_char;
    fn dso__symbols(dso: *mut dso) -> *mut symbols;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__long_name_len(dso: *mut dso) -> size_t;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn dso__short_name_len(dso: *mut dso) -> size_t;
    fn dso__delete_symbol(dso: *mut dso, sym: *mut symbol);
    fn dso__new(name: *const c_char) -> *mut dso;
    fn dso__put(dso: *mut dso);
    fn dso__get(dso: *mut dso) -> *mut dso;
    fn dso__set_kernel(dso: *mut dso, v: c_int);
    fn dso__set_binary_type(dso: *mut dso, v: dso_binary_type);
    fn dso__binary_type(dso: *mut dso) -> dso_binary_type;
    fn dso__set_loaded(dso: *mut dso);

    fn symbol__new(start: u64, len: u64, binding: u8, type_: u8, name: *const c_char) -> *mut symbol;
    fn symbol__type(sym: *mut symbol) -> u8;
    fn symbol__ifunc_alias(sym: *mut symbol) -> bool;
    fn symbols__insert(symbols: *mut symbols, sym: *mut symbol);
    fn __symbols__insert(symbols: *mut symbols, sym: *mut symbol);
    fn symbols__delete(symbols: *mut symbols);
    fn symbols__fixup_end(symbols: *mut symbols, is_kallsyms: bool);
    fn symbols__fixup_duplicate(symbols: *mut symbols);

    fn map__set_start(map: *mut map, v: u64);
    fn map__set_end(map: *mut map, v: u64);
    fn map__set_pgoff(map: *mut map, v: u64);
    fn map__set_mapping_type(map: *mut map, v: c_int);
    fn map__start(map: *mut map) -> u64;
    fn map__kmap(map: *mut map) -> *mut kmap;
    fn map__kmaps(map: *mut map) -> *mut maps;
    fn map__set_reloc(map: *mut map, v: u64);
    fn map__new2(start: u64, dso: *mut dso) -> *mut map;
    fn map__put(map: *mut map);
    fn map__dso(map: *mut map) -> *mut dso;

    fn maps__mutate_mapping(kmaps: *mut maps, map: *mut map, cb: Option<unsafe extern "C" fn(*mut map, *mut c_void) -> c_int>, data: *mut c_void) -> c_int;
    fn maps__find_by_name(kmaps: *mut maps, name: *const c_char) -> *mut map;
    fn maps__insert(kmaps: *mut maps, map: *mut map) -> c_int;
    fn maps__machine(kmaps: *mut maps) -> *mut machine;
    fn maps__fixup_end(kmaps: *mut maps);
    fn dsos__add(dsos: *mut c_int, dso: *mut dso);

    fn kallsyms__is_function(type_: c_char) -> bool;
    fn kallsyms__parse(filename: *const c_char, arg: *mut c_void, cb: Option<unsafe extern "C" fn(*mut c_void, *const c_char, c_char, u64) -> c_int>) -> c_int;
    fn modules__parse(filename: *const c_char, arg: *mut c_void, cb: Option<unsafe extern "C" fn(*mut c_void, *const c_char, u64, u64) -> c_int>) -> c_int;
    fn is_livepatch_symbol(name: *const c_char) -> bool;
    fn is_entry_trampoline(name: *const c_char) -> bool;
    fn is_ignored_kernel_symbol(name: *const c_char) -> bool;
    fn symbol__restricted_filename(filename: *const c_char, restricted: *const c_char) -> bool;
    fn copyfile_mode(from: *const c_char, to: *const c_char, mode: c_int) -> c_int;
    fn readn(fd: c_int, buf: *mut c_void, n: size_t) -> c_int;
    fn lzma_decompress_stream_to_file(in_: *mut FILE, out_fd: c_int) -> c_int;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_debug4(fmt: *const c_char, ...);
}

#[inline] unsafe fn GELF_ST_TYPE(val: u8) -> u8 { val & 0x0f }
#[inline] unsafe fn GELF_ST_BIND(val: u8) -> u8 { val >> 4 }
#[inline] unsafe fn ELF32_ST_VISIBILITY(o: u8) -> u8 { o & 0x03 }
#[inline] unsafe fn ELF64_ST_VISIBILITY(o: u8) -> u8 { ELF32_ST_VISIBILITY(o) }
#[inline] unsafe fn GELF_ST_VISIBILITY(val: u8) -> u8 { ELF64_ST_VISIBILITY(val) }
#[inline] unsafe fn GELF_R_SYM(info: u64) -> u32 { (info >> 32) as u32 }
#[inline] unsafe fn GELF_R_TYPE(info: u64) -> u32 { info as u32 }
#[inline] fn NOTE_ALIGN(n: size_t) -> size_t { (n + 3) & !3usize }
#[inline] fn min_u64(a: u64, b: u64) -> u64 { if a < b { a } else { b } }
#[inline] fn max_u64(a: u64, b: u64) -> u64 { if a > b { a } else { b } }
#[inline] fn min_usize(a: usize, b: usize) -> usize { if a < b { a } else { b } }
#[inline] unsafe fn round_down(x: u64, a: size_t) -> u64 { x & !((a as u64) - 1) }
#[inline] unsafe fn round_up(x: u64, a: size_t) -> u64 { (x + (a as u64) - 1) & !((a as u64) - 1) }
#[inline] unsafe fn PERF_ALIGN(x: u64, a: u64) -> u64 { (x + a - 1) & !(a - 1) }
#[inline] unsafe fn le32toh(x: u32) -> u32 { u32::from_le(x) }

unsafe fn zfree<T>(pp: *mut *mut T) {
    if !(*pp).is_null() {
        free(*pp as *mut c_void);
        *pp = ptr::null_mut();
    }
}

unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}
unsafe fn list_empty(head: *const list_head) -> bool { (*head).next == head as *mut list_head }
unsafe fn list_add_tail(new: *mut list_head, head: *mut list_head) {
    (*new).prev = (*head).prev;
    (*new).next = head;
    (*(*head).prev).next = new;
    (*head).prev = new;
}
unsafe fn list_del_init(entry: *mut list_head) {
    (*(*entry).next).prev = (*entry).prev;
    (*(*entry).prev).next = (*entry).next;
    INIT_LIST_HEAD(entry);
}

#[inline] unsafe fn elf_sym__type(sym: *const GElf_Sym) -> u8 { GELF_ST_TYPE((*sym).st_info) }
#[inline] unsafe fn elf_sym__visibility(sym: *const GElf_Sym) -> u8 { GELF_ST_VISIBILITY((*sym).st_other) }

#[inline] unsafe fn elf_sym__is_function(sym: *const GElf_Sym) -> c_int {
    (((elf_sym__type(sym) == STT_FUNC || elf_sym__type(sym) == STT_GNU_IFUNC)
        && (*sym).st_name != 0
        && (*sym).st_shndx != SHN_UNDEF) as c_int)
}

#[inline] unsafe fn elf_sym__is_object(sym: *const GElf_Sym) -> bool {
    elf_sym__type(sym) == STT_OBJECT && (*sym).st_name != 0 && (*sym).st_shndx != SHN_UNDEF
}

#[inline] unsafe fn elf_sym__is_label(sym: *const GElf_Sym) -> c_int {
    (elf_sym__type(sym) == STT_NOTYPE
        && (*sym).st_name != 0
        && (*sym).st_shndx != SHN_UNDEF
        && (*sym).st_shndx != SHN_ABS
        && elf_sym__visibility(sym) != STV_HIDDEN
        && elf_sym__visibility(sym) != STV_INTERNAL) as c_int
}

unsafe fn elf_sym__filter(sym: *mut GElf_Sym) -> bool {
    elf_sym__is_function(sym) != 0 || elf_sym__is_object(sym)
}

#[inline] unsafe fn elf_sym__name(sym: *const GElf_Sym, symstrs: *const Elf_Data) -> *const c_char {
    ((*symstrs).d_buf as *const c_char).add((*sym).st_name as usize)
}

#[inline] unsafe fn elf_sec__name(shdr: *const GElf_Shdr, secstrs: *const Elf_Data) -> *const c_char {
    ((*secstrs).d_buf as *const c_char).add((*shdr).sh_name as usize)
}

#[inline] unsafe fn elf_sec__is_text(shdr: *const GElf_Shdr, secstrs: *const Elf_Data) -> c_int {
    (!strstr(elf_sec__name(shdr, secstrs), b"text\0".as_ptr() as *const c_char).is_null()) as c_int
}

#[inline] unsafe fn elf_sec__is_data(shdr: *const GElf_Shdr, secstrs: *const Elf_Data) -> bool {
    !strstr(elf_sec__name(shdr, secstrs), b"data\0".as_ptr() as *const c_char).is_null()
}

unsafe fn elf_sec__filter(shdr: *mut GElf_Shdr, secstrs: *mut Elf_Data) -> bool {
    elf_sec__is_text(shdr, secstrs) != 0 || elf_sec__is_data(shdr, secstrs)
}

unsafe fn elf_addr_to_index(elf: *mut Elf, addr: GElf_Addr) -> size_t {
    let mut sec: *mut Elf_Scn = ptr::null_mut();
    let mut shdr: GElf_Shdr = zeroed();
    let mut cnt: size_t = 1;
    loop {
        sec = elf_nextscn(elf, sec);
        if sec.is_null() { break; }
        gelf_getshdr(sec, &mut shdr);
        if addr >= shdr.sh_addr && addr < shdr.sh_addr.wrapping_add(shdr.sh_size) {
            return cnt;
        }
        cnt += 1;
    }
    usize::MAX
}

#[no_mangle]
pub unsafe extern "C" fn elf_section_by_name(
    elf: *mut Elf,
    ep: *mut GElf_Ehdr,
    shp: *mut GElf_Shdr,
    name: *const c_char,
    idx: *mut size_t,
) -> *mut Elf_Scn {
    let mut sec: *mut Elf_Scn = ptr::null_mut();
    let mut cnt: size_t = 1;

    /* ELF is corrupted/truncated, avoid calling elf_strptr. */
    if elf_rawdata(elf_getscn(elf, (*ep).e_shstrndx as size_t), ptr::null_mut()).is_null() {
        return ptr::null_mut();
    }

    loop {
        sec = elf_nextscn(elf, sec);
        if sec.is_null() { break; }
        gelf_getshdr(sec, shp);
        let str_ = elf_strptr(elf, (*ep).e_shstrndx as size_t, (*shp).sh_name as size_t);
        if !str_.is_null() && strcmp(name, str_) == 0 {
            if !idx.is_null() { *idx = cnt; }
            return sec;
        }
        cnt += 1;
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn filename__has_section(filename: *const c_char, sec: *const c_char) -> bool {
    let fd = open(filename, O_RDONLY | O_CLOEXEC);
    if fd < 0 { return false; }
    let elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, ptr::null_mut());
    let mut found = false;
    if !elf.is_null() {
        let mut ehdr: GElf_Ehdr = zeroed();
        let mut shdr: GElf_Shdr = zeroed();
        if !gelf_getehdr(elf, &mut ehdr).is_null() {
            found = !elf_section_by_name(elf, &mut ehdr, &mut shdr, sec, ptr::null_mut()).is_null();
        }
        elf_end(elf);
    }
    close(fd);
    found
}

unsafe fn elf_read_program_header(elf: *mut Elf, vaddr: u64, phdr: *mut GElf_Phdr) -> c_int {
    let mut phdrnum: size_t = 0;
    if elf_getphdrnum(elf, &mut phdrnum) != 0 { return -1; }
    for i in 0..phdrnum {
        if gelf_getphdr(elf, i as c_int, phdr).is_null() { return -1; }
        if (*phdr).p_type != PT_LOAD { continue; }
        let sz = max_u64((*phdr).p_memsz, (*phdr).p_filesz);
        if sz == 0 { continue; }
        if vaddr >= (*phdr).p_vaddr && vaddr < (*phdr).p_vaddr.wrapping_add(sz) { return 0; }
    }
    /* Not found any valid program header */
    -1
}

#[repr(C)]
struct rel_info {
    nr_entries: u32,
    sorted: *mut u32,
    is_rela: bool,
    reldata: *mut Elf_Data,
    rela: GElf_Rela,
    rel: GElf_Rel,
}

unsafe fn get_rel_symidx(ri: *mut rel_info, mut idx: u32) -> u32 {
    if !(*ri).sorted.is_null() { idx = *(*ri).sorted.add(idx as usize); }
    if (*ri).is_rela {
        gelf_getrela((*ri).reldata, idx as c_int, &mut (*ri).rela);
        return GELF_R_SYM((*ri).rela.r_info);
    }
    gelf_getrel((*ri).reldata, idx as c_int, &mut (*ri).rel);
    GELF_R_SYM((*ri).rel.r_info)
}

unsafe fn get_rel_offset(ri: *mut rel_info, x: u32) -> u64 {
    if (*ri).is_rela {
        let mut rela: GElf_Rela = zeroed();
        gelf_getrela((*ri).reldata, x as c_int, &mut rela);
        rela.r_offset
    } else {
        let mut rel: GElf_Rel = zeroed();
        gelf_getrel((*ri).reldata, x as c_int, &mut rel);
        rel.r_offset
    }
}

unsafe extern "C" fn rel_cmp(a: *const c_void, b: *const c_void, r: *mut c_void) -> c_int {
    let ri = r as *mut rel_info;
    let a_offset = get_rel_offset(ri, *(a as *const u32));
    let b_offset = get_rel_offset(ri, *(b as *const u32));
    if a_offset < b_offset { -1 } else if a_offset > b_offset { 1 } else { 0 }
}

unsafe fn sort_rel(ri: *mut rel_info) -> c_int {
    let sz = size_of::<u32>();
    (*ri).sorted = calloc((*ri).nr_entries as size_t, sz) as *mut u32;
    if (*ri).sorted.is_null() { return -1; }
    for i in 0..(*ri).nr_entries {
        *(*ri).sorted.add(i as usize) = i;
    }
    qsort_r((*ri).sorted as *mut c_void, (*ri).nr_entries as size_t, sz, Some(rel_cmp), ri as *mut c_void);
    0
}

/*
 * For x86_64, the GNU linker is putting IFUNC information in the relocation
 * addend.
 */
unsafe fn addend_may_be_ifunc(ehdr: *mut GElf_Ehdr, ri: *mut rel_info) -> bool {
    (*ehdr).e_machine == EM_X86_64 && (*ri).is_rela && GELF_R_TYPE((*ri).rela.r_info) == R_X86_64_IRELATIVE
}

unsafe fn get_ifunc_name(elf: *mut Elf, dso_: *mut dso, ehdr: *mut GElf_Ehdr, ri: *mut rel_info, buf: *mut c_char, buf_sz: size_t) -> bool {
    let mut addr = (*ri).rela.r_addend as u64;
    let mut phdr: GElf_Phdr = zeroed();
    if !addend_may_be_ifunc(ehdr, ri) { return false; }
    if elf_read_program_header(elf, addr, &mut phdr) != 0 { return false; }
    addr = addr.wrapping_sub(phdr.p_vaddr.wrapping_sub(phdr.p_offset));
    let sym = dso__find_symbol_nocache(dso_, addr);
    /* Expecting the address to be an IFUNC or IFUNC alias */
    if sym.is_null() || (*sym).start != addr || (symbol__type(sym) != STT_GNU_IFUNC && !symbol__ifunc_alias(sym)) {
        return false;
    }
    snprintf(buf, buf_sz, b"%s@plt\0".as_ptr() as *const c_char, (*sym).name);
    true
}

unsafe fn exit_rel(ri: *mut rel_info) { zfree(&mut (*ri).sorted); }

unsafe fn get_plt_sizes(dso_: *mut dso, ehdr: *mut GElf_Ehdr, shdr_plt: *mut GElf_Shdr, plt_header_size: *mut u64, plt_entry_size: *mut u64) -> bool {
    match (*ehdr).e_machine {
        EM_ARM => { *plt_header_size = 20; *plt_entry_size = 12; return true; }
        EM_AARCH64 | EM_LOONGARCH | EM_RISCV => { *plt_header_size = 32; *plt_entry_size = 16; return true; }
        EM_SPARC => { *plt_header_size = 48; *plt_entry_size = 12; return true; }
        EM_SPARCV9 => { *plt_header_size = 128; *plt_entry_size = 32; return true; }
        EM_386 | EM_X86_64 => {
            *plt_entry_size = (*shdr_plt).sh_entsize;
            /* Size is 8 or 16, if not, assume alignment indicates size */
            if *plt_entry_size != 8 && *plt_entry_size != 16 {
                *plt_entry_size = if (*shdr_plt).sh_addralign == 8 { 8 } else { 16 };
            }
            *plt_header_size = *plt_entry_size;
        }
        _ => {
            /* FIXME: s390/alpha/mips/parisc/poperpc/sh/xtensa need to be checked */
            *plt_header_size = (*shdr_plt).sh_entsize;
            *plt_entry_size = (*shdr_plt).sh_entsize;
        }
    }
    if *plt_entry_size != 0 { return true; }
    pr_debug(b"Missing PLT entry size for %s\n\0".as_ptr() as *const c_char, dso__long_name(dso_));
    false
}

unsafe fn machine_is_x86(e_machine: GElf_Half) -> bool { e_machine == EM_386 || e_machine == EM_X86_64 }

#[repr(C)] struct rela_dyn { offset: GElf_Addr, sym_idx: u32 }
#[repr(C)] struct rela_dyn_info {
    dso: *mut dso,
    plt_got_data: *mut Elf_Data,
    nr_entries: u32,
    sorted: *mut rela_dyn,
    dynsym_data: *mut Elf_Data,
    dynstr_data: *mut Elf_Data,
    rela_dyn_data: *mut Elf_Data,
}
unsafe fn exit_rela_dyn(di: *mut rela_dyn_info) { zfree(&mut (*di).sorted); }
unsafe extern "C" fn cmp_offset(a: *const c_void, b: *const c_void) -> c_int {
    let va = a as *const rela_dyn;
    let vb = b as *const rela_dyn;
    if (*va).offset < (*vb).offset { -1 } else if (*va).offset > (*vb).offset { 1 } else { 0 }
}
unsafe fn sort_rela_dyn(di: *mut rela_dyn_info) -> c_int {
    (*di).sorted = calloc((*di).nr_entries as size_t, size_of::<rela_dyn>()) as *mut rela_dyn;
    if (*di).sorted.is_null() { return -1; }
    let mut n: u32 = 0;
    for i in 0..(*di).nr_entries {
        let mut rela: GElf_Rela = zeroed();
        gelf_getrela((*di).rela_dyn_data, i as c_int, &mut rela);
        let sym_idx = GELF_R_SYM(rela.r_info);
        if sym_idx != 0 {
            (*(*di).sorted.add(n as usize)).sym_idx = sym_idx;
            (*(*di).sorted.add(n as usize)).offset = rela.r_offset;
            n += 1;
        }
    }
    (*di).nr_entries = n;
    qsort((*di).sorted as *mut c_void, n as size_t, size_of::<rela_dyn>(), Some(cmp_offset));
    0
}

unsafe fn get_rela_dyn_info(elf: *mut Elf, ehdr: *mut GElf_Ehdr, di: *mut rela_dyn_info, mut scn: *mut Elf_Scn) {
    let mut rela_dyn_shdr: GElf_Shdr = zeroed();
    let mut shdr: GElf_Shdr = zeroed();
    (*di).plt_got_data = elf_getdata(scn, ptr::null_mut());
    scn = elf_section_by_name(elf, ehdr, &mut rela_dyn_shdr, b".rela.dyn\0".as_ptr() as *const c_char, ptr::null_mut());
    if scn.is_null() || rela_dyn_shdr.sh_link == 0 || rela_dyn_shdr.sh_entsize == 0 { return; }
    (*di).nr_entries = (rela_dyn_shdr.sh_size / rela_dyn_shdr.sh_entsize) as u32;
    (*di).rela_dyn_data = elf_getdata(scn, ptr::null_mut());
    scn = elf_getscn(elf, rela_dyn_shdr.sh_link as size_t);
    if scn.is_null() || gelf_getshdr(scn, &mut shdr).is_null() || shdr.sh_link == 0 { return; }
    (*di).dynsym_data = elf_getdata(scn, ptr::null_mut());
    (*di).dynstr_data = elf_getdata(elf_getscn(elf, shdr.sh_link as size_t), ptr::null_mut());
    if (*di).plt_got_data.is_null() || (*di).dynstr_data.is_null() || (*di).dynsym_data.is_null() || (*di).rela_dyn_data.is_null() { return; }
    /* Sort into offset order */
    sort_rela_dyn(di);
}

/* Get instruction displacement from a plt entry for x86_64 */
unsafe fn get_x86_64_plt_disp(p: *const u8) -> u32 {
    let endbr64: [u8; 4] = [0xf3, 0x0f, 0x1e, 0xfa];
    let mut n: usize = 0;
    /* Skip endbr64 */
    if memcmp(p as *const c_void, endbr64.as_ptr() as *const c_void, endbr64.len()) == 0 { n += endbr64.len(); }
    /* Skip bnd prefix */
    if *p.add(n) == 0xf2 { n += 1; }
    /* jmp with 4-byte displacement */
    if *p.add(n) == 0xff && *p.add(n + 1) == 0x25 {
        let mut disp: u32 = 0;
        n += 2;
        /* Also add offset from start of entry to end of instruction */
        memcpy(&mut disp as *mut _ as *mut c_void, p.add(n) as *const c_void, size_of::<u32>());
        return (n as u32).wrapping_add(4).wrapping_add(le32toh(disp));
    }
    0
}

unsafe fn get_plt_got_name(shdr: *mut GElf_Shdr, i: size_t, di: *mut rela_dyn_info, buf: *mut c_char, buf_sz: size_t) -> bool {
    let mut vi = rela_dyn { offset: 0, sym_idx: 0 };
    let mut sym: GElf_Sym = zeroed();
    if (*di).sorted.is_null() { return false; }
    let disp = get_x86_64_plt_disp(((*(*di).plt_got_data).d_buf as *const u8).add(i));
    if disp == 0 { return false; }
    /* Compute target offset of the .plt.got entry */
    vi.offset = (*shdr).sh_offset + (*(*di).plt_got_data).d_off as u64 + i as u64 + disp as u64;
    /* Find that offset in .rela.dyn (sorted by offset) */
    let vr = bsearch(&vi as *const _ as *const c_void, (*di).sorted as *const c_void, (*di).nr_entries as size_t, size_of::<rela_dyn>(), Some(cmp_offset)) as *mut rela_dyn;
    if vr.is_null() { return false; }
    /* Get the associated symbol */
    gelf_getsym((*di).dynsym_data, (*vr).sym_idx as c_int, &mut sym);
    let mut sym_name = elf_sym__name(&sym, (*di).dynstr_data);
    let demangled = dso__demangle_sym((*di).dso, 0, sym_name);
    if !demangled.is_null() { sym_name = demangled; }
    snprintf(buf, buf_sz, b"%s@plt\0".as_ptr() as *const c_char, sym_name);
    let result = *sym_name != 0;
    free(demangled as *mut c_void);
    result
}

unsafe fn dso__synthesize_plt_got_symbols(dso_: *mut dso, elf: *mut Elf, ehdr: *mut GElf_Ehdr, buf: *mut c_char, buf_sz: size_t) -> c_int {
    let mut di: rela_dyn_info = zeroed();
    di.dso = dso_;
    let mut shdr: GElf_Shdr = zeroed();
    let scn = elf_section_by_name(elf, ehdr, &mut shdr, b".plt.got\0".as_ptr() as *const c_char, ptr::null_mut());
    if scn.is_null() || shdr.sh_entsize == 0 { return 0; }
    if (*ehdr).e_machine == EM_X86_64 { get_rela_dyn_info(elf, ehdr, &mut di, scn); }
    let mut err = -1;
    let mut i: size_t = 0;
    while i < shdr.sh_size as size_t {
        if !get_plt_got_name(&mut shdr, i, &mut di, buf, buf_sz) {
            snprintf(buf, buf_sz, b"offset_%#lx@plt\0".as_ptr() as *const c_char, shdr.sh_offset + i as u64);
        }
        let sym = symbol__new(shdr.sh_offset + i as u64, shdr.sh_entsize, STB_GLOBAL, STT_FUNC, buf);
        if sym.is_null() { break; }
        symbols__insert(dso__symbols(dso_), sym);
        i += shdr.sh_entsize as usize;
    }
    if i >= shdr.sh_size as size_t { err = 0; }
    exit_rela_dyn(&mut di);
    err
}

#[no_mangle]
pub unsafe extern "C" fn dso__synthesize_plt_symbols(dso_: *mut dso, ss: *mut symsrc) -> c_int {
    let mut idx: u32;
    let mut sym: GElf_Sym = zeroed();
    let mut plt_offset: u64;
    let mut plt_header_size: u64 = 0;
    let mut plt_entry_size: u64 = 0;
    let mut shdr_plt: GElf_Shdr = zeroed();
    let mut plt_sec_shdr: GElf_Shdr = zeroed();
    let mut shdr_rel_plt: GElf_Shdr = zeroed();
    let mut shdr_dynsym: GElf_Shdr;
    let mut sympltname = [0 as c_char; 1024];
    let elf = (*ss).elf;
    let mut ehdr = (*ss).ehdr;
    let mut nr = 0;
    let mut err = -1;
    let mut ri: rel_info = zeroed();

    if elf_section_by_name(elf, &mut ehdr, &mut shdr_plt, b".plt\0".as_ptr() as *const c_char, ptr::null_mut()).is_null() { return 0; }

    /*
     * A symbol from a previous section (e.g. .init) can have been expanded
     * by symbols__fixup_end() to overlap .plt. Truncate it before adding
     * a symbol for .plt header.
     */
    let f0 = dso__find_symbol_nocache(dso_, shdr_plt.sh_offset);
    if !f0.is_null() && (*f0).start < shdr_plt.sh_offset && (*f0).end > shdr_plt.sh_offset { (*f0).end = shdr_plt.sh_offset; }
    if !get_plt_sizes(dso_, &mut ehdr, &mut shdr_plt, &mut plt_header_size, &mut plt_entry_size) { return 0; }
    let plt_sym = symbol__new(shdr_plt.sh_offset, plt_header_size, STB_GLOBAL, STT_FUNC, b".plt\0".as_ptr() as *const c_char);
    if plt_sym.is_null() { goto_out_plt(dso_, &mut ri, err); return 0; }
    symbols__insert(dso__symbols(dso_), plt_sym);
    if machine_is_x86(ehdr.e_machine) && dso__synthesize_plt_got_symbols(dso_, elf, &mut ehdr, sympltname.as_mut_ptr(), sympltname.len()) != 0 {
        exit_rel(&mut ri); return 0;
    }
    let lazy_plt: bool;
    if machine_is_x86(ehdr.e_machine) && !elf_section_by_name(elf, &mut ehdr, &mut plt_sec_shdr, b".plt.sec\0".as_ptr() as *const c_char, ptr::null_mut()).is_null() {
        if !get_plt_sizes(dso_, &mut ehdr, &mut plt_sec_shdr, &mut plt_header_size, &mut plt_entry_size) { return 0; }
        (*plt_sym).end = (*plt_sym).start + shdr_plt.sh_size;
        plt_offset = plt_sec_shdr.sh_offset;
        lazy_plt = false;
    } else {
        plt_offset = shdr_plt.sh_offset;
        lazy_plt = true;
    }
    let mut scn_plt_rel = elf_section_by_name(elf, &mut ehdr, &mut shdr_rel_plt, b".rela.plt\0".as_ptr() as *const c_char, ptr::null_mut());
    if scn_plt_rel.is_null() {
        scn_plt_rel = elf_section_by_name(elf, &mut ehdr, &mut shdr_rel_plt, b".rel.plt\0".as_ptr() as *const c_char, ptr::null_mut());
        if scn_plt_rel.is_null() { return 0; }
    }
    if shdr_rel_plt.sh_type != SHT_RELA && shdr_rel_plt.sh_type != SHT_REL { return 0; }
    if shdr_rel_plt.sh_link == 0 { return 0; }
    let scn_dynsym: *mut Elf_Scn;
    if shdr_rel_plt.sh_link as size_t == (*ss).dynsym_idx {
        scn_dynsym = (*ss).dynsym; shdr_dynsym = (*ss).dynshdr;
    } else if shdr_rel_plt.sh_link as size_t == (*ss).symtab_idx {
        scn_dynsym = (*ss).symtab; shdr_dynsym = (*ss).symshdr;
    } else { exit_rel(&mut ri); return 0; }
    if scn_dynsym.is_null() { return 0; }
    ri.reldata = elf_getdata(scn_plt_rel, ptr::null_mut());
    if ri.reldata.is_null() { exit_rel(&mut ri); return 0; }
    let syms = elf_getdata(scn_dynsym, ptr::null_mut());
    if syms.is_null() { exit_rel(&mut ri); return 0; }
    let scn_symstrs = elf_getscn(elf, shdr_dynsym.sh_link as size_t);
    if scn_symstrs.is_null() { exit_rel(&mut ri); return 0; }
    let symstrs = elf_getdata(scn_symstrs, ptr::null_mut());
    if symstrs.is_null() || (*symstrs).d_size == 0 { exit_rel(&mut ri); return 0; }
    ri.nr_entries = (shdr_rel_plt.sh_size / shdr_rel_plt.sh_entsize) as u32;
    ri.is_rela = shdr_rel_plt.sh_type == SHT_RELA;
    if lazy_plt {
        if ri.nr_entries as u64 * plt_entry_size == shdr_plt.sh_size {
            dso__delete_symbol(dso_, plt_sym);
        } else {
            plt_offset += plt_header_size;
        }
    }
    if machine_is_x86(ehdr.e_machine) && sort_rel(&mut ri) != 0 { exit_rel(&mut ri); return 0; }
    idx = 0;
    while idx < ri.nr_entries {
        gelf_getsym(syms, get_rel_symidx(&mut ri, idx) as c_int, &mut sym);
        let mut elf_name = elf_sym__name(&sym, symstrs);
        let demangled = dso__demangle_sym(dso_, 0, elf_name);
        if !demangled.is_null() { elf_name = demangled; }
        if *elf_name != 0 {
            snprintf(sympltname.as_mut_ptr(), sympltname.len(), b"%s@plt\0".as_ptr() as *const c_char, elf_name);
        } else if !get_ifunc_name(elf, dso_, &mut ehdr, &mut ri, sympltname.as_mut_ptr(), sympltname.len()) {
            snprintf(sympltname.as_mut_ptr(), sympltname.len(), b"offset_%#lx@plt\0".as_ptr() as *const c_char, plt_offset);
        }
        free(demangled as *mut c_void);
        let f = symbol__new(plt_offset, plt_entry_size, STB_GLOBAL, STT_FUNC, sympltname.as_ptr());
        if f.is_null() { exit_rel(&mut ri); return 0; }
        plt_offset += plt_entry_size;
        symbols__insert(dso__symbols(dso_), f);
        nr += 1;
        idx += 1;
    }
    err = 0;
    exit_rel(&mut ri);
    if err == 0 { return nr; }
    pr_debug(b"%s: problems reading %s PLT info.\n\0".as_ptr() as *const c_char, b"dso__synthesize_plt_symbols\0".as_ptr() as *const c_char, dso__long_name(dso_));
    0
}

unsafe fn goto_out_plt(_dso: *mut dso, ri: *mut rel_info, _err: c_int) { exit_rel(ri); }

unsafe fn elf_read_build_id(elf: *mut Elf, bf: *mut c_void, size: size_t) -> c_int {
    let mut err = -1;
    if size < BUILD_ID_SIZE { return err; }
    if elf_kind(elf) != ELF_K_ELF { return err; }
    let mut ehdr: GElf_Ehdr = zeroed();
    let mut shdr: GElf_Shdr = zeroed();
    if gelf_getehdr(elf, &mut ehdr).is_null() {
        pr_err(b"%s: cannot get elf header.\n\0".as_ptr() as *const c_char, b"elf_read_build_id\0".as_ptr() as *const c_char);
        return err;
    }
    let mut sec = elf_section_by_name(elf, &mut ehdr, &mut shdr, b".note.gnu.build-id\0".as_ptr() as *const c_char, ptr::null_mut());
    if sec.is_null() { sec = elf_section_by_name(elf, &mut ehdr, &mut shdr, b".notes\0".as_ptr() as *const c_char, ptr::null_mut()); }
    if sec.is_null() { sec = elf_section_by_name(elf, &mut ehdr, &mut shdr, b".note\0".as_ptr() as *const c_char, ptr::null_mut()); }
    if sec.is_null() { return err; }
    let data = elf_getdata(sec, ptr::null_mut());
    if data.is_null() { return err; }
    let mut ptr_ = (*data).d_buf as *mut u8;
    let end = ptr_.add((*data).d_size);
    while ptr_ < end {
        let nhdr = ptr_ as *mut GElf_Nhdr;
        if ptr_.add(size_of::<GElf_Nhdr>()) > end { break; }
        let namesz = NOTE_ALIGN((*nhdr).n_namesz as size_t);
        let descsz = NOTE_ALIGN((*nhdr).n_descsz as size_t);
        let remaining = end as usize - ptr_ as usize - size_of::<GElf_Nhdr>();
        if namesz > remaining || descsz > remaining - namesz {
            pr_warning(b"%s: oversized note: n_namesz=%u, n_descsz=%u\n\0".as_ptr() as *const c_char, b"elf_read_build_id\0".as_ptr() as *const c_char, (*nhdr).n_namesz, (*nhdr).n_descsz);
            break;
        }
        ptr_ = ptr_.add(size_of::<GElf_Nhdr>());
        let name = ptr_ as *const c_char;
        ptr_ = ptr_.add(namesz);
        if (*nhdr).n_type == NT_GNU_BUILD_ID as u32 && (*nhdr).n_namesz as usize == b"GNU\0".len() {
            if memcmp(name as *const c_void, b"GNU\0".as_ptr() as *const c_void, b"GNU\0".len()) == 0 {
                let sz = min_usize(size, descsz);
                memcpy(bf, ptr_ as *const c_void, sz);
                memset((bf as *mut u8).add(sz) as *mut c_void, 0, size - sz);
                err = sz as c_int;
                break;
            }
        }
        ptr_ = ptr_.add(descsz);
    }
    err
}

unsafe fn read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int {
    let size = (*bid).data.len();
    let mut err = libbfd__read_build_id(filename, bid);
    if err >= 0 { return err; }
    if size < BUILD_ID_SIZE { return err; }
    let fd = open(filename, O_RDONLY | O_CLOEXEC);
    if fd < 0 { return err; }
    let elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, ptr::null_mut());
    if elf.is_null() {
        pr_debug2(b"%s: cannot read %s ELF file.\n\0".as_ptr() as *const c_char, b"read_build_id\0".as_ptr() as *const c_char, filename);
    } else {
        err = elf_read_build_id(elf, (*bid).data.as_mut_ptr() as *mut c_void, size);
        if err > 0 { (*bid).size = err as size_t; }
        elf_end(elf);
    }
    close(fd);
    err
}

#[no_mangle]
pub unsafe extern "C" fn filename__read_build_id(mut filename: *const c_char, bid: *mut build_id) -> c_int {
    let mut m = kmod_path { name: ptr::null_mut(), comp: ptr::null_mut() };
    let mut path = [0 as c_char; PATH_MAX];
    if filename.is_null() { return -EFAULT; }
    errno = 0;
    if !is_regular_file(filename) { return if errno == 0 { -EWOULDBLOCK } else { -errno }; }
    if kmod_path__parse(&mut m, filename) != 0 { return -1; }
    if !m.comp.is_null() {
        let mut error = 0;
        let fd = filename__decompress(filename, path.as_mut_ptr(), path.len(), m.comp, &mut error);
        if fd < 0 {
            pr_debug(b"Failed to decompress (error %d) %s\n\0".as_ptr() as *const c_char, error, filename);
            return -1;
        }
        close(fd);
        /* non-empty path means a temp file was created */
        if path[0] != 0 { filename = path.as_ptr(); }
    }
    let err = read_build_id(filename, bid);
    if !m.comp.is_null() && filename == path.as_ptr() { unlink(filename); }
    err
}

#[no_mangle]
pub unsafe extern "C" fn sysfs__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int {
    let size = (*bid).data.len();
    let fd = open(filename, O_RDONLY | O_CLOEXEC);
    if fd < 0 { return -1; }
    let mut err = -1;
    loop {
        let mut bf = [0 as c_char; BUFSIZ];
        let mut nhdr: GElf_Nhdr = zeroed();
        if read(fd, &mut nhdr as *mut _ as *mut c_void, size_of::<GElf_Nhdr>()) != size_of::<GElf_Nhdr>() as ssize_t { break; }
        let namesz = NOTE_ALIGN(nhdr.n_namesz as size_t);
        let descsz = NOTE_ALIGN(nhdr.n_descsz as size_t);
        if nhdr.n_type == NT_GNU_BUILD_ID as u32 && nhdr.n_namesz as usize == b"GNU\0".len() {
            if read(fd, bf.as_mut_ptr() as *mut c_void, namesz) != namesz as ssize_t { break; }
            if memcmp(bf.as_ptr() as *const c_void, b"GNU\0".as_ptr() as *const c_void, b"GNU\0".len()) == 0 {
                let sz = min_usize(descsz, size);
                if read(fd, (*bid).data.as_mut_ptr() as *mut c_void, sz) == sz as ssize_t {
                    memset((*bid).data.as_mut_ptr().add(sz) as *mut c_void, 0, size - sz);
                    (*bid).size = sz;
                    err = 0;
                    break;
                }
            } else {
                /* descsz from untrusted file — clamp to buffer */
                if descsz > bf.len() { break; }
                if read(fd, bf.as_mut_ptr() as *mut c_void, descsz) != descsz as ssize_t { break; }
            }
        } else {
            let n;
            /* int sum of namesz+descsz can overflow negative, bypassing size check */
            if namesz > bf.len() || descsz > bf.len() - namesz {
                n = bf.len();
                pr_debug(b"%s: truncating reading of build id in sysfs file %s: n_namesz=%u, n_descsz=%u.\n\0".as_ptr() as *const c_char, b"sysfs__read_build_id\0".as_ptr() as *const c_char, filename, nhdr.n_namesz, nhdr.n_descsz);
            } else { n = namesz + descsz; }
            /* no valid note has both namesz and descsz zero */
            if n == 0 { break; }
            if read(fd, bf.as_mut_ptr() as *mut c_void, n) != n as ssize_t { break; }
        }
    }
    close(fd);
    err
}

#[no_mangle]
pub unsafe extern "C" fn filename__read_debuglink(filename: *const c_char, debuglink: *mut c_char, size: size_t) -> c_int {
    let mut err = libbfd_filename__read_debuglink(filename, debuglink, size);
    if err >= 0 { return err; }
    let fd = open(filename, O_RDONLY | O_CLOEXEC);
    if fd < 0 { return err; }
    let elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, ptr::null_mut());
    if elf.is_null() {
        pr_debug2(b"%s: cannot read %s ELF file.\n\0".as_ptr() as *const c_char, b"filename__read_debuglink\0".as_ptr() as *const c_char, filename);
        close(fd);
        return err;
    }
    if elf_kind(elf) == ELF_K_ELF {
        let mut ehdr: GElf_Ehdr = zeroed();
        let mut shdr: GElf_Shdr = zeroed();
        if !gelf_getehdr(elf, &mut ehdr).is_null() {
            let sec = elf_section_by_name(elf, &mut ehdr, &mut shdr, b".gnu_debuglink\0".as_ptr() as *const c_char, ptr::null_mut());
            if !sec.is_null() {
                let data = elf_getdata(sec, ptr::null_mut());
                if !data.is_null() {
                    if (*data).d_size > 0 {
                        let len = min_usize(size - 1, (*data).d_size);
                        memcpy(debuglink as *mut c_void, (*data).d_buf, len);
                        *debuglink.add(len) = 0;
                    } else {
                        *debuglink = 0;
                    }
                    err = 0;
                }
            }
        } else {
            pr_err(b"%s: cannot get elf header.\n\0".as_ptr() as *const c_char, b"filename__read_debuglink\0".as_ptr() as *const c_char);
        }
    }
    elf_end(elf);
    close(fd);
    err
}

#[no_mangle] pub unsafe extern "C" fn symsrc__possibly_runtime(ss: *mut symsrc) -> bool { !(*ss).dynsym.is_null() || !(*ss).opdsec.is_null() }
#[no_mangle] pub unsafe extern "C" fn symsrc__has_symtab(ss: *mut symsrc) -> bool { !(*ss).symtab.is_null() }
#[no_mangle] pub unsafe extern "C" fn symsrc__destroy(ss: *mut symsrc) { zfree(&mut (*ss).name); elf_end((*ss).elf); close((*ss).fd); }

unsafe fn elf__needs_adjust_symbols(ehdr: *const GElf_Ehdr) -> bool {
    /*
     * Usually vmlinux is an ELF file with type ET_EXEC for most
     * architectures; except Arm64 kernel is linked with option
     * '-share', so need to check type ET_DYN.
     */
    (*ehdr).e_type == ET_EXEC || (*ehdr).e_type == ET_REL || (*ehdr).e_type == ET_DYN
}

unsafe fn read_gnu_debugdata(dso_: *mut dso, elf: *mut Elf, name: *const c_char, fd_ret: *mut c_int) -> *mut Elf {
    let mut ehdr: GElf_Ehdr = zeroed();
    let mut shdr: GElf_Shdr = zeroed();
    let mut shndx: size_t = 0;
    let mut temp_filename = *b"/tmp/perf.gnu_debugdata.elf.XXXXXX\0";
    if gelf_getehdr(elf, &mut ehdr).is_null() {
        pr_debug(b"%s: cannot read %s ELF file.\n\0".as_ptr() as *const c_char, b"read_gnu_debugdata\0".as_ptr() as *const c_char, name);
        *dso__load_errno(dso_) = DSO_LOAD_ERRNO__INVALID_ELF;
        return ptr::null_mut();
    }
    let scn = elf_section_by_name(elf, &mut ehdr, &mut shdr, b".gnu_debugdata\0".as_ptr() as *const c_char, &mut shndx);
    if scn.is_null() { *dso__load_errno(dso_) = -ENOENT; return ptr::null_mut(); }
    if shdr.sh_type == SHT_NOBITS {
        pr_debug(b"%s: .gnu_debugdata of ELF file %s has no data.\n\0".as_ptr() as *const c_char, b"read_gnu_debugdata\0".as_ptr() as *const c_char, name);
        *dso__load_errno(dso_) = DSO_LOAD_ERRNO__INVALID_ELF;
        return ptr::null_mut();
    }
    let scn_data = elf_rawdata(scn, ptr::null_mut());
    if scn_data.is_null() {
        pr_debug(b"%s: error reading .gnu_debugdata of %s: %s\n\0".as_ptr() as *const c_char, b"read_gnu_debugdata\0".as_ptr() as *const c_char, name, elf_errmsg(-1));
        *dso__load_errno(dso_) = DSO_LOAD_ERRNO__INVALID_ELF;
        return ptr::null_mut();
    }
    let wrapped = fmemopen((*scn_data).d_buf, (*scn_data).d_size, b"r\0".as_ptr() as *const c_char);
    if wrapped.is_null() { *dso__load_errno(dso_) = -errno; return ptr::null_mut(); }
    let temp_fd = mkostemp(temp_filename.as_mut_ptr() as *mut c_char, O_CLOEXEC);
    if temp_fd < 0 {
        *dso__load_errno(dso_) = -errno;
        fclose(wrapped);
        return ptr::null_mut();
    }
    unlink(temp_filename.as_ptr() as *const c_char);
    let ret = lzma_decompress_stream_to_file(wrapped, temp_fd);
    fclose(wrapped);
    if ret < 0 {
        *dso__load_errno(dso_) = -errno;
        close(temp_fd);
        return ptr::null_mut();
    }
    let elf_embedded = elf_begin(temp_fd, PERF_ELF_C_READ_MMAP, ptr::null_mut());
    if elf_embedded.is_null() {
        pr_debug(b"%s: error reading .gnu_debugdata of %s: %s\n\0".as_ptr() as *const c_char, b"read_gnu_debugdata\0".as_ptr() as *const c_char, name, elf_errmsg(-1));
        *dso__load_errno(dso_) = DSO_LOAD_ERRNO__INVALID_ELF;
        close(temp_fd);
        return ptr::null_mut();
    }
    pr_debug(b"%s: using .gnu_debugdata of %s\n\0".as_ptr() as *const c_char, b"read_gnu_debugdata\0".as_ptr() as *const c_char, name);
    *fd_ret = temp_fd;
    elf_embedded
}

#[no_mangle]
pub unsafe extern "C" fn symsrc__init(ss: *mut symsrc, dso_: *mut dso, name: *const c_char, mut type_: dso_binary_type) -> c_int {
    let fd: c_int;
    if dso__needs_decompress(dso_) {
        fd = dso__decompress_kmodule_fd(dso_, name);
        if fd < 0 { return -1; }
        type_ = dso__symtab_type(dso_);
    } else {
        fd = open(name, O_RDONLY | O_CLOEXEC);
        if fd < 0 { *dso__load_errno(dso_) = errno; return -1; }
    }
    let mut elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, ptr::null_mut());
    if elf.is_null() {
        pr_debug(b"%s: cannot read %s ELF file.\n\0".as_ptr() as *const c_char, b"symsrc__init\0".as_ptr() as *const c_char, name);
        *dso__load_errno(dso_) = DSO_LOAD_ERRNO__INVALID_ELF;
        close(fd);
        return -1;
    }
    let mut fd = fd;
    if type_ == DSO_BINARY_TYPE__GNU_DEBUGDATA {
        let mut new_fd = 0;
        let embedded = read_gnu_debugdata(dso_, elf, name, &mut new_fd);
        if embedded.is_null() { elf_end(elf); close(fd); return -1; }
        elf_end(elf);
        close(fd);
        fd = new_fd;
        elf = embedded;
    }
    let mut ehdr: GElf_Ehdr = zeroed();
    if gelf_getehdr(elf, &mut ehdr).is_null() {
        *dso__load_errno(dso_) = DSO_LOAD_ERRNO__INVALID_ELF;
        pr_debug(b"%s: cannot get elf header.\n\0".as_ptr() as *const c_char, b"symsrc__init\0".as_ptr() as *const c_char);
        elf_end(elf); close(fd); return -1;
    }
    if dso__swap_init(dso_, ehdr.e_ident[EI_DATA]) != 0 {
        *dso__load_errno(dso_) = DSO_LOAD_ERRNO__INTERNAL_ERROR;
        elf_end(elf); close(fd); return -1;
    }
    /* Always reject images with a mismatched build-id: */
    if dso__has_build_id(dso_) && !symbol_conf.ignore_vmlinux_buildid {
        let mut build_id = [0u8; BUILD_ID_SIZE];
        let mut bid: build_id = zeroed();
        let size = elf_read_build_id(elf, build_id.as_mut_ptr() as *mut c_void, BUILD_ID_SIZE);
        if size <= 0 { *dso__load_errno(dso_) = DSO_LOAD_ERRNO__CANNOT_READ_BUILDID; elf_end(elf); close(fd); return -1; }
        build_id__init(&mut bid, build_id.as_ptr(), size);
        if !dso__build_id_equal(dso_, &bid) {
            pr_debug(b"%s: build id mismatch for %s.\n\0".as_ptr() as *const c_char, b"symsrc__init\0".as_ptr() as *const c_char, name);
            *dso__load_errno(dso_) = DSO_LOAD_ERRNO__MISMATCHING_BUILDID;
            elf_end(elf); close(fd); return -1;
        }
    }
    (*ss).is_64_bit = gelf_getclass(elf) == ELFCLASS64;
    (*ss).symtab_idx = 0;
    (*ss).symtab = elf_section_by_name(elf, &mut ehdr, &mut (*ss).symshdr, b".symtab\0".as_ptr() as *const c_char, &mut (*ss).symtab_idx);
    if (*ss).symshdr.sh_type != SHT_SYMTAB { (*ss).symtab = ptr::null_mut(); }
    (*ss).dynsym_idx = 0;
    (*ss).dynsym = elf_section_by_name(elf, &mut ehdr, &mut (*ss).dynshdr, b".dynsym\0".as_ptr() as *const c_char, &mut (*ss).dynsym_idx);
    if (*ss).dynshdr.sh_type != SHT_DYNSYM { (*ss).dynsym = ptr::null_mut(); }
    (*ss).opdidx = 0;
    (*ss).opdsec = elf_section_by_name(elf, &mut ehdr, &mut (*ss).opdshdr, b".opd\0".as_ptr() as *const c_char, &mut (*ss).opdidx);
    if (*ss).opdshdr.sh_type != SHT_PROGBITS { (*ss).opdsec = ptr::null_mut(); }
    if dso__kernel(dso_) == DSO_SPACE__USER { (*ss).adjust_symbols = true; } else { (*ss).adjust_symbols = elf__needs_adjust_symbols(&ehdr); }
    (*ss).name = strdup(name);
    if (*ss).name.is_null() { *dso__load_errno(dso_) = errno; elf_end(elf); close(fd); return -1; }
    (*ss).elf = elf; (*ss).fd = fd; (*ss).ehdr = ehdr; (*ss).type_ = type_;
    0
}

unsafe fn is_exe_text(flags: c_int) -> bool { ((flags as u64) & (SHF_ALLOC | SHF_EXECINSTR)) == (SHF_ALLOC | SHF_EXECINSTR) }

unsafe fn max_text_section(elf: *mut Elf, ehdr: *mut GElf_Ehdr) -> u64 {
    let mut sec: *mut Elf_Scn = ptr::null_mut();
    let mut shdr: GElf_Shdr = zeroed();
    let mut offs = 0u64;
    if (*ehdr).e_machine == EM_PARISC || (*ehdr).e_machine == EM_ALPHA { return 0; }
    if elf_rawdata(elf_getscn(elf, (*ehdr).e_shstrndx as size_t), ptr::null_mut()).is_null() { return 0; }
    loop {
        sec = elf_nextscn(elf, sec);
        if sec.is_null() { break; }
        if gelf_getshdr(sec, &mut shdr).is_null() { break; }
        if !is_exe_text(shdr.sh_flags as c_int) { continue; }
        let sec_name = elf_strptr(elf, (*ehdr).e_shstrndx as size_t, shdr.sh_name as size_t);
        if sec_name.is_null()
            || strstarts(sec_name, b".init\0".as_ptr() as *const c_char)
            || strstarts(sec_name, b".exit\0".as_ptr() as *const c_char) { break; }
        let align = if shdr.sh_addralign != 0 { shdr.sh_addralign } else { 1 };
        if offs != 0 && PERF_ALIGN(offs, align) != shdr.sh_offset { break; }
        offs = shdr.sh_offset + shdr.sh_size;
    }
    offs
}

unsafe fn ref_reloc_sym_not_found(kmap_: *mut kmap) -> bool {
    !kmap_.is_null() && !(*kmap_).ref_reloc_sym.is_null() && !(*(*kmap_).ref_reloc_sym).name.is_null() && (*(*kmap_).ref_reloc_sym).unrelocated_addr == 0
}
unsafe fn ref_reloc(kmap_: *mut kmap) -> u64 {
    if !kmap_.is_null() && !(*kmap_).ref_reloc_sym.is_null() && (*(*kmap_).ref_reloc_sym).unrelocated_addr != 0 {
        return (*(*kmap_).ref_reloc_sym).addr - (*(*kmap_).ref_reloc_sym).unrelocated_addr;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn arch__sym_update(_s: *mut symbol, _sym: *mut GElf_Sym) {}

#[repr(C)] struct remap_kernel_ctx { sh_addr: u64, sh_size: u64, sh_offset: u64, kmap: *mut kmap }
unsafe extern "C" fn remap_kernel_cb(map_: *mut map, data: *mut c_void) -> c_int {
    let ctx = data as *mut remap_kernel_ctx;
    map__set_start(map_, (*ctx).sh_addr + ref_reloc((*ctx).kmap));
    map__set_end(map_, map__start(map_) + (*ctx).sh_size);
    map__set_pgoff(map_, (*ctx).sh_offset);
    map__set_mapping_type(map_, MAPPING_TYPE__DSO);
    0
}

unsafe fn dso__process_kernel_symbol(
    dso_: *mut dso, map_: *mut map, sym: *mut GElf_Sym, shdr: *mut GElf_Shdr,
    kmaps: *mut maps, kmap_: *mut kmap, curr_dsop: *mut *mut dso,
    section_name: *const c_char, adjust_kernel_syms: bool, kmodule: bool,
    remap_kernel: *mut bool, max_text_sh_offset: u64,
) -> c_int {
    let mut dso_name = [0 as c_char; PATH_MAX];
    if adjust_kernel_syms {
        if dso__rel(dso_) { (*sym).st_value += (*shdr).sh_offset; }
        else { (*sym).st_value -= (*shdr).sh_addr - (*shdr).sh_offset; }
    }
    if strcmp(section_name, dso__short_name(*curr_dsop).add(dso__short_name_len(dso_))) == 0 { return 0; }
    if strcmp(section_name, b".text\0".as_ptr() as *const c_char) == 0 {
        if *remap_kernel && dso__kernel(dso_) != 0 && !kmodule {
            let mut ctx = remap_kernel_ctx { sh_addr: (*shdr).sh_addr, sh_size: (*shdr).sh_size, sh_offset: (*shdr).sh_offset, kmap: kmap_ };
            *remap_kernel = false;
            maps__mutate_mapping(kmaps, map_, Some(remap_kernel_cb), &mut ctx as *mut _ as *mut c_void);
        }
        if *remap_kernel && kmodule {
            *remap_kernel = false;
            map__set_pgoff(map_, (*shdr).sh_offset);
        }
        dso__put(*curr_dsop);
        *curr_dsop = dso__get(dso_);
        return 0;
    }
    if kmap_.is_null() { return 0; }
    if kmodule && adjust_kernel_syms && is_exe_text((*shdr).sh_flags as c_int) && (*shdr).sh_offset <= max_text_sh_offset {
        dso__put(*curr_dsop);
        *curr_dsop = dso__get(dso_);
        return 0;
    }
    snprintf(dso_name.as_mut_ptr(), dso_name.len(), b"%s%s\0".as_ptr() as *const c_char, dso__short_name(dso_), section_name);
    let mut curr_map = maps__find_by_name(kmaps, dso_name.as_ptr());
    if curr_map.is_null() {
        let mut start = (*sym).st_value;
        if kmodule { start += map__start(map_) + (*shdr).sh_offset; }
        let curr_dso = dso__new(dso_name.as_ptr());
        if curr_dso.is_null() { return -1; }
        dso__set_kernel(curr_dso, dso__kernel(dso_));
        dso__set_binary_type(curr_dso, dso__binary_type(dso_));
        dso__set_adjust_symbols(curr_dso, dso__adjust_symbols(dso_));
        curr_map = map__new2(start, curr_dso);
        if curr_map.is_null() { dso__put(curr_dso); return -1; }
        if dso__kernel(curr_dso) != 0 { (*map__kmap(curr_map)).kmaps = kmaps; }
        if adjust_kernel_syms {
            map__set_start(curr_map, (*shdr).sh_addr + ref_reloc(kmap_));
            map__set_end(curr_map, map__start(curr_map) + (*shdr).sh_size);
            map__set_pgoff(curr_map, (*shdr).sh_offset);
        } else {
            map__set_mapping_type(curr_map, MAPPING_TYPE__IDENTITY);
        }
        dso__set_symtab_type(curr_dso, dso__symtab_type(dso_));
        if maps__insert(kmaps, curr_map) != 0 {
            dso__put(curr_dso); map__put(curr_map); return -1;
        }
        dsos__add(&mut (*maps__machine(kmaps)).dsos, curr_dso);
        dso__set_loaded(curr_dso);
        dso__put(*curr_dsop);
        *curr_dsop = curr_dso;
    } else {
        dso__put(*curr_dsop);
        *curr_dsop = dso__get(map__dso(curr_map));
    }
    map__put(curr_map);
    0
}

unsafe fn dso__load_sym_internal(dso_: *mut dso, map_: *mut map, syms_ss: *mut symsrc, runtime_ss: *mut symsrc, kmodule: c_int, dynsym: c_int) -> c_int {
    let kmap_ = if dso__kernel(dso_) != 0 { map__kmap(map_) } else { ptr::null_mut() };
    let kmaps = if !kmap_.is_null() { map__kmaps(map_) } else { ptr::null_mut() };
    if !kmap_.is_null() && kmaps.is_null() { return -1; }
    let elf = (*syms_ss).elf;
    let ehdr = (*syms_ss).ehdr;
    let (mut sec, mut shdr) = if dynsym != 0 { ((*syms_ss).dynsym, (*syms_ss).dynshdr) } else { ((*syms_ss).symtab, (*syms_ss).symshdr) };
    let mut tshdr: GElf_Shdr = zeroed();
    if !elf_section_by_name((*runtime_ss).elf, &mut (*runtime_ss).ehdr, &mut tshdr, b".text\0".as_ptr() as *const c_char, ptr::null_mut()).is_null() {
        dso__set_text_offset(dso_, tshdr.sh_addr - tshdr.sh_offset);
        dso__set_text_end(dso_, tshdr.sh_offset + tshdr.sh_size);
    }
    let opddata = if !(*runtime_ss).opdsec.is_null() { elf_rawdata((*runtime_ss).opdsec, ptr::null_mut()) } else { ptr::null_mut() };
    let syms = elf_getdata(sec, ptr::null_mut());
    if syms.is_null() { return -1; }
    sec = elf_getscn(elf, shdr.sh_link as size_t);
    if sec.is_null() { return -1; }
    let symstrs = elf_getdata(sec, ptr::null_mut());
    if symstrs.is_null() { return -1; }
    let mut sec_strndx = elf_getscn((*runtime_ss).elf, (*runtime_ss).ehdr.e_shstrndx as size_t);
    if sec_strndx.is_null() { return -1; }
    let secstrs_run = elf_getdata(sec_strndx, ptr::null_mut());
    if secstrs_run.is_null() { return -1; }
    sec_strndx = elf_getscn(elf, ehdr.e_shstrndx as size_t);
    if sec_strndx.is_null() { return -1; }
    let secstrs_sym = elf_getdata(sec_strndx, ptr::null_mut());
    if secstrs_sym.is_null() { return -1; }
    let nr_syms = (shdr.sh_size / shdr.sh_entsize) as u32;
    let mut sym: GElf_Sym = zeroed();
    if ref_reloc_sym_not_found(kmap_) {
        for idx in 0..nr_syms {
            gelf_getsym(syms, idx as c_int, &mut sym);
            let elf_name = elf_sym__name(&sym, symstrs);
            if strcmp(elf_name, (*(*kmap_).ref_reloc_sym).name) != 0 { continue; }
            (*(*kmap_).ref_reloc_sym).unrelocated_addr = sym.st_value;
            map__set_reloc(map_, (*(*kmap_).ref_reloc_sym).addr - (*(*kmap_).ref_reloc_sym).unrelocated_addr);
            break;
        }
    }
    if dso__is_vdso(dso_) { map__set_reloc(map_, map__start(map_) - dso__text_offset(dso_)); }
    dso__set_adjust_symbols(dso_, (*runtime_ss).adjust_symbols || ref_reloc(kmap_) != 0);
    let mut remap_kernel = false;
    let mut adjust_kernel_syms = false;
    if dso__kernel(dso_) != 0 { remap_kernel = true; adjust_kernel_syms = dso__adjust_symbols(dso_); }
    let max_text_sh_offset = if kmodule != 0 && adjust_kernel_syms { max_text_section((*runtime_ss).elf, &mut (*runtime_ss).ehdr) } else { 0 };
    let mut curr_dso = dso__get(dso_);
    let mut nr = 0;
    for idx in 0..nr_syms {
        gelf_getsym(syms, idx as c_int, &mut sym);
        let mut elf_name = elf_sym__name(&sym, symstrs);
        let is_label = elf_sym__is_label(&sym);
        let mut used_opd = false;
        if is_label == 0 && !elf_sym__filter(&mut sym) { continue; }
        if ehdr.e_machine == EM_ARM || ehdr.e_machine == EM_AARCH64 {
            if *elf_name == b'$' as c_char && !strchr(b"adtx\0".as_ptr() as *const c_char, *elf_name.add(1) as c_int).is_null()
                && (*elf_name.add(2) == 0 || *elf_name.add(2) == b'.' as c_char) { continue; }
        }
        if ehdr.e_machine == EM_RISCV {
            if *elf_name == b'$' as c_char && !strchr(b"dx\0".as_ptr() as *const c_char, *elf_name.add(1) as c_int).is_null() { continue; }
        }
        if dso__kernel(dso_) != 0 && is_ignored_kernel_symbol(elf_name) { continue; }
        if !(*runtime_ss).opdsec.is_null() && sym.st_shndx as size_t == (*runtime_ss).opdidx {
            let offset = (sym.st_value - (*syms_ss).opdshdr.sh_addr) as usize;
            let opd = ((*opddata).d_buf as *mut u64).add(offset / size_of::<u64>());
            sym.st_value = *opd;
            sym.st_shndx = elf_addr_to_index((*runtime_ss).elf, sym.st_value) as u16;
            used_opd = true;
        }
        if sym.st_shndx == SHN_ABS { continue; }
        sec = elf_getscn((*syms_ss).elf, sym.st_shndx as size_t);
        if sec.is_null() {
            if dynsym != 0 && ehdr.e_shnum != 0 && sym.st_shndx < SHN_LORESERVE && sym.st_shndx >= ehdr.e_shnum { continue; }
            dso__put(curr_dso); return -1;
        }
        gelf_getshdr(sec, &mut shdr);
        if (shdr.sh_flags & SHF_ALLOC) == 0 { continue; }
        let mut secstrs = secstrs_sym;
        if shdr.sh_type == SHT_NOBITS {
            sec = elf_getscn((*runtime_ss).elf, sym.st_shndx as size_t);
            if sec.is_null() { dso__put(curr_dso); return -1; }
            gelf_getshdr(sec, &mut shdr);
            secstrs = secstrs_run;
        }
        if is_label != 0 && !elf_sec__filter(&mut shdr, secstrs) { continue; }
        let section_name = elf_sec__name(&shdr, secstrs);
        if ehdr.e_machine == EM_ARM && GELF_ST_TYPE(sym.st_info) == STT_FUNC && (sym.st_value & 1) != 0 { sym.st_value -= 1; }
        if dso__kernel(dso_) != 0 {
            if dso__process_kernel_symbol(dso_, map_, &mut sym, &mut shdr, kmaps, kmap_, &mut curr_dso, section_name, adjust_kernel_syms, kmodule != 0, &mut remap_kernel, max_text_sh_offset) != 0 {
                dso__put(curr_dso); return -1;
            }
        } else if (used_opd && (*runtime_ss).adjust_symbols) || (!used_opd && (*syms_ss).adjust_symbols) {
            let mut phdr: GElf_Phdr = zeroed();
            if elf_read_program_header((*runtime_ss).elf, sym.st_value, &mut phdr) != 0 {
                sym.st_value -= shdr.sh_addr - shdr.sh_offset;
            } else {
                sym.st_value -= phdr.p_vaddr - phdr.p_offset;
            }
        }
        let demangled = dso__demangle_sym(dso_, kmodule, elf_name);
        if !demangled.is_null() { elf_name = demangled; }
        let f = symbol__new(sym.st_value, sym.st_size, GELF_ST_BIND(sym.st_info), GELF_ST_TYPE(sym.st_info), elf_name);
        free(demangled as *mut c_void);
        if f.is_null() { dso__put(curr_dso); return -1; }
        arch__sym_update(f, &mut sym);
        __symbols__insert(dso__symbols(curr_dso), f);
        nr += 1;
    }
    dso__put(curr_dso);
    if nr > 0 {
        symbols__fixup_end(dso__symbols(dso_), false);
        symbols__fixup_duplicate(dso__symbols(dso_));
        if !kmap_.is_null() { maps__fixup_end(kmaps); }
    }
    nr
}

#[no_mangle]
pub unsafe extern "C" fn dso__load_sym(dso_: *mut dso, map_: *mut map, syms_ss: *mut symsrc, runtime_ss: *mut symsrc, kmodule: c_int) -> c_int {
    let mut nr = 0;
    let mut err = -1;
    dso__set_symtab_type(dso_, (*syms_ss).type_);
    dso__set_is_64_bit(dso_, (*syms_ss).is_64_bit);
    dso__set_rel(dso_, (*syms_ss).ehdr.e_type == ET_REL);
    if kmodule != 0 && !(*syms_ss).symtab.is_null() { symbols__delete(dso__symbols(dso_)); }
    if (*syms_ss).symtab.is_null() {
        if dso__kernel(dso_) != 0 { return err; }
    } else {
        err = dso__load_sym_internal(dso_, map_, syms_ss, runtime_ss, kmodule, 0);
        if err < 0 { return err; }
        nr = err;
    }
    if !(*syms_ss).dynsym.is_null() {
        err = dso__load_sym_internal(dso_, map_, syms_ss, runtime_ss, kmodule, 1);
        if err < 0 { return err; }
        nr += err;
    }
    if (*syms_ss).type_ == DSO_BINARY_TYPE__GNU_DEBUGDATA && !(*runtime_ss).dynsym.is_null() {
        err = dso__load_sym_internal(dso_, map_, runtime_ss, runtime_ss, kmodule, 1);
        if err < 0 { return err; }
        nr += err;
    }
    nr
}

unsafe fn elf_read_maps(elf: *mut Elf, exe: bool, mapfn: mapfn_t, data: *mut c_void) -> c_int {
    let mut phdr: GElf_Phdr = zeroed();
    let mut phdrnum: size_t = 0;
    if elf_getphdrnum(elf, &mut phdrnum) != 0 { return -1; }
    for i in 0..phdrnum {
        if gelf_getphdr(elf, i as c_int, &mut phdr).is_null() { return -1; }
        if phdr.p_type != PT_LOAD { continue; }
        if exe { if (phdr.p_flags & PF_X) == 0 { continue; } }
        else if (phdr.p_flags & PF_R) == 0 { continue; }
        let sz = min_u64(phdr.p_memsz, phdr.p_filesz);
        if sz == 0 { continue; }
        let err = mapfn.unwrap()(phdr.p_vaddr, sz, phdr.p_offset, data);
        if err != 0 { return err; }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn file__read_maps(fd: c_int, exe: bool, mapfn: mapfn_t, data: *mut c_void, is_64_bit: *mut bool) -> c_int {
    let elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, ptr::null_mut());
    if elf.is_null() { return -1; }
    if !is_64_bit.is_null() { *is_64_bit = gelf_getclass(elf) == ELFCLASS64; }
    let err = elf_read_maps(elf, exe, mapfn, data);
    elf_end(elf);
    err
}

#[no_mangle]
pub unsafe extern "C" fn dso__type_fd(fd: c_int) -> dso_type {
    let mut dso_type_ = DSO__TYPE_UNKNOWN;
    let elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, ptr::null_mut());
    if elf.is_null() { return dso_type_; }
    if elf_kind(elf) == ELF_K_ELF {
        if gelf_getclass(elf) == ELFCLASS64 { dso_type_ = DSO__TYPE_64BIT; }
        else {
            let mut ehdr: GElf_Ehdr = zeroed();
            if !gelf_getehdr(elf, &mut ehdr).is_null() {
                dso_type_ = if ehdr.e_machine == EM_X86_64 { DSO__TYPE_X32BIT } else { DSO__TYPE_32BIT };
            }
        }
    }
    elf_end(elf);
    dso_type_
}

unsafe fn copy_bytes(from: c_int, from_offs: off_t, to: c_int, to_offs: off_t, mut len: u64) -> c_int {
    let mut err = -1;
    let buf = malloc(page_size) as *mut c_char;
    if buf.is_null() { return -1; }
    if lseek(to, to_offs, SEEK_SET) != to_offs { free(buf as *mut c_void); return err; }
    if lseek(from, from_offs, SEEK_SET) != from_offs { free(buf as *mut c_void); return err; }
    while len != 0 {
        let mut n = page_size;
        if len < n as u64 { n = len as size_t; }
        /* Use read because mmap won't work on proc files */
        let r = read(from, buf as *mut c_void, n);
        if r < 0 { break; }
        if r == 0 { err = 0; break; }
        n = r as size_t;
        let wr = write(to, buf as *const c_void, n);
        if wr < 0 || wr as size_t != n { break; }
        len -= n as u64;
        err = 0;
    }
    free(buf as *mut c_void);
    err
}

#[repr(C)] struct kcore { fd: c_int, elfclass: c_int, elf: *mut Elf, ehdr: GElf_Ehdr }
unsafe fn kcore__open(kcore_: *mut kcore, filename: *const c_char) -> c_int {
    (*kcore_).fd = open(filename, O_RDONLY | O_CLOEXEC);
    if (*kcore_).fd == -1 { return -1; }
    (*kcore_).elf = elf_begin((*kcore_).fd, ELF_C_READ, ptr::null_mut());
    if (*kcore_).elf.is_null() { close((*kcore_).fd); return -1; }
    (*kcore_).elfclass = gelf_getclass((*kcore_).elf);
    if (*kcore_).elfclass == ELFCLASSNONE { elf_end((*kcore_).elf); close((*kcore_).fd); return -1; }
    if gelf_getehdr((*kcore_).elf, &mut (*kcore_).ehdr).is_null() { elf_end((*kcore_).elf); close((*kcore_).fd); return -1; }
    0
}
unsafe fn kcore__init(kcore_: *mut kcore, filename: *mut c_char, elfclass: c_int, temp: bool) -> c_int {
    (*kcore_).elfclass = elfclass;
    (*kcore_).fd = if temp { mkostemp(filename, O_CLOEXEC) } else { open(filename, O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC, 0o400) };
    if (*kcore_).fd == -1 { return -1; }
    (*kcore_).elf = elf_begin((*kcore_).fd, ELF_C_WRITE, ptr::null_mut());
    if (*kcore_).elf.is_null() { close((*kcore_).fd); unlink(filename); return -1; }
    if gelf_newehdr((*kcore_).elf, elfclass).is_null() { elf_end((*kcore_).elf); close((*kcore_).fd); unlink(filename); return -1; }
    memset(&mut (*kcore_).ehdr as *mut _ as *mut c_void, 0, size_of::<GElf_Ehdr>());
    0
}
unsafe fn kcore__close(kcore_: *mut kcore) { elf_end((*kcore_).elf); close((*kcore_).fd); }
unsafe fn kcore__copy_hdr(from: *mut kcore, to: *mut kcore, count: size_t) -> c_int {
    let ehdr = &mut (*to).ehdr;
    let kehdr = &(*from).ehdr;
    memcpy(ehdr.e_ident.as_mut_ptr() as *mut c_void, kehdr.e_ident.as_ptr() as *const c_void, EI_NIDENT);
    ehdr.e_type = kehdr.e_type; ehdr.e_machine = kehdr.e_machine; ehdr.e_version = kehdr.e_version; ehdr.e_entry = 0;
    ehdr.e_shoff = 0; ehdr.e_flags = kehdr.e_flags; ehdr.e_phnum = count as u16; ehdr.e_shentsize = 0; ehdr.e_shnum = 0; ehdr.e_shstrndx = 0;
    if (*from).elfclass == ELFCLASS32 {
        ehdr.e_phoff = 52; ehdr.e_ehsize = 52; ehdr.e_phentsize = 32;
    } else {
        ehdr.e_phoff = 64; ehdr.e_ehsize = 64; ehdr.e_phentsize = 56;
    }
    if gelf_update_ehdr((*to).elf, ehdr) == 0 { return -1; }
    if gelf_newphdr((*to).elf, count).is_null() { return -1; }
    0
}
unsafe fn kcore__add_phdr(kcore_: *mut kcore, idx: c_int, offset: off_t, addr: u64, len: u64) -> c_int {
    let mut phdr = GElf_Phdr { p_type: PT_LOAD, p_flags: PF_R | PF_W | PF_X, p_offset: offset as u64, p_vaddr: addr, p_paddr: 0, p_filesz: len, p_memsz: len, p_align: page_size as u64 };
    if gelf_update_phdr((*kcore_).elf, idx, &mut phdr) == 0 { return -1; }
    0
}
unsafe fn kcore__write(kcore_: *mut kcore) -> off_t { elf_update((*kcore_).elf, ELF_C_WRITE) }

#[repr(C)] struct phdr_data { offset: off_t, rel: off_t, addr: u64, len: u64, node: list_head, remaps: *mut phdr_data }
#[repr(C)] struct sym_data { addr: u64, node: list_head }
#[repr(C)] struct kcore_copy_info {
    stext: u64, etext: u64, first_symbol: u64, last_symbol: u64, first_module: u64,
    first_module_symbol: u64, last_module_symbol: u64, phnum: size_t, phdrs: list_head, syms: list_head,
}

unsafe fn phdr_data__new(addr: u64, len: u64, offset: off_t) -> *mut phdr_data {
    let p = calloc(1, size_of::<phdr_data>()) as *mut phdr_data;
    if !p.is_null() { (*p).addr = addr; (*p).len = len; (*p).offset = offset; }
    p
}
unsafe fn kcore_copy_info__addnew(kci: *mut kcore_copy_info, addr: u64, len: u64, offset: off_t) -> *mut phdr_data {
    let p = phdr_data__new(addr, len, offset);
    if !p.is_null() { list_add_tail(&mut (*p).node, &mut (*kci).phdrs); }
    p
}
unsafe fn kcore_copy__free_phdrs(kci: *mut kcore_copy_info) {
    let mut node = (*kci).phdrs.next;
    while node != &mut (*kci).phdrs {
        let next = (*node).next;
        let p = node as *mut phdr_data;
        list_del_init(node);
        free(p as *mut c_void);
        node = next;
    }
}
unsafe fn kcore_copy__new_sym(kci: *mut kcore_copy_info, addr: u64) -> *mut sym_data {
    let s = calloc(1, size_of::<sym_data>()) as *mut sym_data;
    if !s.is_null() { (*s).addr = addr; list_add_tail(&mut (*s).node, &mut (*kci).syms); }
    s
}
unsafe fn kcore_copy__free_syms(kci: *mut kcore_copy_info) {
    let mut node = (*kci).syms.next;
    while node != &mut (*kci).syms {
        let next = (*node).next;
        let s = node as *mut sym_data;
        list_del_init(node);
        free(s as *mut c_void);
        node = next;
    }
}

unsafe extern "C" fn kcore_copy__process_kallsyms(arg: *mut c_void, name: *const c_char, type_: c_char, start: u64) -> c_int {
    let kci = arg as *mut kcore_copy_info;
    if !kallsyms__is_function(type_) { return 0; }
    /* Ignore livepatch symbols */
    if is_livepatch_symbol(name) { return 0; }
    if !strchr(name, b'[' as c_int).is_null() {
        if (*kci).first_module_symbol == 0 || start < (*kci).first_module_symbol { (*kci).first_module_symbol = start; }
        if start > (*kci).last_module_symbol { (*kci).last_module_symbol = start; }
        return 0;
    }
    if (*kci).first_symbol == 0 || start < (*kci).first_symbol { (*kci).first_symbol = start; }
    if (*kci).last_symbol == 0 || start > (*kci).last_symbol { (*kci).last_symbol = start; }
    if strcmp(name, b"_stext\0".as_ptr() as *const c_char) == 0 { (*kci).stext = start; return 0; }
    if strcmp(name, b"_etext\0".as_ptr() as *const c_char) == 0 { (*kci).etext = start; return 0; }
    if is_entry_trampoline(name) && kcore_copy__new_sym(kci, start).is_null() { return -1; }
    0
}
unsafe fn kcore_copy__parse_kallsyms(kci: *mut kcore_copy_info, dir: *const c_char) -> c_int {
    let mut filename = [0 as c_char; PATH_MAX];
    scnprintf(filename.as_mut_ptr(), PATH_MAX, b"%s/kallsyms\0".as_ptr() as *const c_char, dir);
    if symbol__restricted_filename(filename.as_ptr(), b"/proc/kallsyms\0".as_ptr() as *const c_char) { return -1; }
    if kallsyms__parse(filename.as_ptr(), kci as *mut c_void, Some(kcore_copy__process_kallsyms)) < 0 { return -1; }
    0
}
unsafe extern "C" fn kcore_copy__process_modules(arg: *mut c_void, _name: *const c_char, start: u64, _size: u64) -> c_int {
    let kci = arg as *mut kcore_copy_info;
    if (*kci).first_module == 0 || start < (*kci).first_module { (*kci).first_module = start; }
    0
}
unsafe fn kcore_copy__parse_modules(kci: *mut kcore_copy_info, dir: *const c_char) -> c_int {
    let mut filename = [0 as c_char; PATH_MAX];
    scnprintf(filename.as_mut_ptr(), PATH_MAX, b"%s/modules\0".as_ptr() as *const c_char, dir);
    if symbol__restricted_filename(filename.as_ptr(), b"/proc/modules\0".as_ptr() as *const c_char) { return -1; }
    if modules__parse(filename.as_ptr(), kci as *mut c_void, Some(kcore_copy__process_modules)) < 0 { return -1; }
    0
}
unsafe fn kcore_copy__map(kci: *mut kcore_copy_info, start: u64, end: u64, pgoff: u64, s: u64, e: u64) -> c_int {
    if s < start || s >= end { return 0; }
    let offset = (s - start) + pgoff;
    let len = if e < end { e - s } else { end - s };
    if !kcore_copy_info__addnew(kci, s, len, offset as off_t).is_null() { 0 } else { -1 }
}
unsafe extern "C" fn kcore_copy__read_map(start: u64, len: u64, pgoff: u64, data: *mut c_void) -> c_int {
    let kci = data as *mut kcore_copy_info;
    let end = start + len;
    if kcore_copy__map(kci, start, end, pgoff, (*kci).stext, (*kci).etext) != 0 { return -1; }
    if kcore_copy__map(kci, start, end, pgoff, (*kci).first_module, (*kci).last_module_symbol) != 0 { return -1; }
    let mut node = (*kci).syms.next;
    while node != &mut (*kci).syms {
        let sdat = node as *mut sym_data;
        let s = round_down((*sdat).addr, page_size);
        if kcore_copy__map(kci, start, end, pgoff, s, s + len) != 0 { return -1; }
        node = (*node).next;
    }
    0
}
unsafe fn kcore_copy__read_maps(kci: *mut kcore_copy_info, elf: *mut Elf) -> c_int {
    if elf_read_maps(elf, true, Some(kcore_copy__read_map), kci as *mut c_void) < 0 { return -1; }
    0
}
unsafe fn kcore_copy__find_remaps(kci: *mut kcore_copy_info) {
    if (*kci).stext == 0 { return; }
    let mut k: *mut phdr_data = ptr::null_mut();
    let mut node = (*kci).phdrs.next;
    while node != &mut (*kci).phdrs {
        let p = node as *mut phdr_data;
        let pend = (*p).addr + (*p).len - 1;
        if (*p).addr <= (*kci).stext && pend >= (*kci).stext { k = p; break; }
        node = (*node).next;
    }
    if k.is_null() { return; }
    let kend = (*k).offset + (*k).len as off_t;
    node = (*kci).phdrs.next;
    while node != &mut (*kci).phdrs {
        let p = node as *mut phdr_data;
        let pend = (*p).offset + (*p).len as off_t;
        if p != k && (*p).offset >= (*k).offset && pend <= kend { (*p).remaps = k; }
        node = (*node).next;
    }
}
unsafe fn kcore_copy__layout(kci: *mut kcore_copy_info) {
    kcore_copy__find_remaps(kci);
    let mut rel: off_t = 0;
    let mut node = (*kci).phdrs.next;
    while node != &mut (*kci).phdrs {
        let p = node as *mut phdr_data;
        if (*p).remaps.is_null() { (*p).rel = rel; rel += (*p).len as off_t; }
        (*kci).phnum += 1;
        node = (*node).next;
    }
    node = (*kci).phdrs.next;
    while node != &mut (*kci).phdrs {
        let p = node as *mut phdr_data;
        let k = (*p).remaps;
        if !k.is_null() { (*p).rel = (*p).offset - (*k).offset + (*k).rel; }
        node = (*node).next;
    }
}
unsafe fn kcore_copy__calc_maps(kci: *mut kcore_copy_info, dir: *const c_char, elf: *mut Elf) -> c_int {
    if kcore_copy__parse_kallsyms(kci, dir) != 0 { return -1; }
    if kcore_copy__parse_modules(kci, dir) != 0 { return -1; }
    if (*kci).stext != 0 { (*kci).stext = round_down((*kci).stext, page_size); } else { (*kci).stext = round_down((*kci).first_symbol, page_size); }
    if (*kci).etext != 0 { (*kci).etext = round_up((*kci).etext, page_size); } else if (*kci).last_symbol != 0 { (*kci).etext = round_up((*kci).last_symbol, page_size); (*kci).etext += page_size as u64; }
    if (*kci).first_module_symbol != 0 && ((*kci).first_module == 0 || (*kci).first_module_symbol < (*kci).first_module) { (*kci).first_module = (*kci).first_module_symbol; }
    (*kci).first_module = round_down((*kci).first_module, page_size);
    if (*kci).last_module_symbol != 0 { (*kci).last_module_symbol = round_up((*kci).last_module_symbol, page_size); (*kci).last_module_symbol += page_size as u64; }
    if (*kci).stext == 0 || (*kci).etext == 0 { return -1; }
    if (*kci).first_module != 0 && (*kci).last_module_symbol == 0 { return -1; }
    if kcore_copy__read_maps(kci, elf) != 0 { return -1; }
    kcore_copy__layout(kci);
    0
}
unsafe fn kcore_copy__copy_file(from_dir: *const c_char, to_dir: *const c_char, name: *const c_char) -> c_int {
    let mut from_filename = [0 as c_char; PATH_MAX];
    let mut to_filename = [0 as c_char; PATH_MAX];
    scnprintf(from_filename.as_mut_ptr(), PATH_MAX, b"%s/%s\0".as_ptr() as *const c_char, from_dir, name);
    scnprintf(to_filename.as_mut_ptr(), PATH_MAX, b"%s/%s\0".as_ptr() as *const c_char, to_dir, name);
    copyfile_mode(from_filename.as_ptr(), to_filename.as_ptr(), 0o400)
}
unsafe fn kcore_copy__unlink(dir: *const c_char, name: *const c_char) -> c_int {
    let mut filename = [0 as c_char; PATH_MAX];
    scnprintf(filename.as_mut_ptr(), PATH_MAX, b"%s/%s\0".as_ptr() as *const c_char, dir, name);
    unlink(filename.as_ptr())
}
unsafe fn kcore_copy__compare_fds(from: c_int, to: c_int) -> c_int {
    let buf_from = malloc(page_size);
    let buf_to = malloc(page_size);
    let mut err = -1;
    if buf_from.is_null() || buf_to.is_null() { free(buf_to); free(buf_from); return err; }
    loop {
        let ret = read(from, buf_from, page_size);
        if ret < 0 { break; }
        if ret == 0 { err = 0; break; }
        let len = ret as size_t;
        if readn(to, buf_to, len) != len as c_int { break; }
        if memcmp(buf_from, buf_to, len) != 0 { break; }
    }
    free(buf_to); free(buf_from); err
}
unsafe fn kcore_copy__compare_files(from_filename: *const c_char, to_filename: *const c_char) -> c_int {
    let from = open(from_filename, O_RDONLY | O_CLOEXEC);
    if from < 0 { return -1; }
    let to = open(to_filename, O_RDONLY | O_CLOEXEC);
    if to < 0 { close(from); return -1; }
    let err = kcore_copy__compare_fds(from, to);
    close(to); close(from); err
}
unsafe fn kcore_copy__compare_file(from_dir: *const c_char, to_dir: *const c_char, name: *const c_char) -> c_int {
    let mut from_filename = [0 as c_char; PATH_MAX];
    let mut to_filename = [0 as c_char; PATH_MAX];
    scnprintf(from_filename.as_mut_ptr(), PATH_MAX, b"%s/%s\0".as_ptr() as *const c_char, from_dir, name);
    scnprintf(to_filename.as_mut_ptr(), PATH_MAX, b"%s/%s\0".as_ptr() as *const c_char, to_dir, name);
    kcore_copy__compare_files(from_filename.as_ptr(), to_filename.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn kcore_copy(from_dir: *const c_char, to_dir: *const c_char) -> c_int {
    let mut kcore_: kcore = zeroed();
    let mut extract: kcore = zeroed();
    let mut idx = 0;
    let mut err = -1;
    let mut kci: kcore_copy_info = zeroed();
    let mut kcore_filename = [0 as c_char; PATH_MAX];
    let mut extract_filename = [0 as c_char; PATH_MAX];
    INIT_LIST_HEAD(&mut kci.phdrs);
    INIT_LIST_HEAD(&mut kci.syms);
    if kcore_copy__copy_file(from_dir, to_dir, b"kallsyms\0".as_ptr() as *const c_char) != 0 { return -1; }
    if kcore_copy__copy_file(from_dir, to_dir, b"modules\0".as_ptr() as *const c_char) != 0 { kcore_copy__unlink(to_dir, b"kallsyms\0".as_ptr() as *const c_char); return -1; }
    scnprintf(kcore_filename.as_mut_ptr(), PATH_MAX, b"%s/kcore\0".as_ptr() as *const c_char, from_dir);
    scnprintf(extract_filename.as_mut_ptr(), PATH_MAX, b"%s/kcore\0".as_ptr() as *const c_char, to_dir);
    if kcore__open(&mut kcore_, kcore_filename.as_ptr()) != 0 { err = -1; }
    else if kcore_copy__calc_maps(&mut kci, from_dir, kcore_.elf) == 0 && kcore__init(&mut extract, extract_filename.as_mut_ptr(), kcore_.elfclass, false) == 0 {
        if kcore__copy_hdr(&mut kcore_, &mut extract, kci.phnum) == 0 {
            let mut offset = (gelf_fsize(extract.elf, ELF_T_EHDR, 1, EV_CURRENT) + gelf_fsize(extract.elf, ELF_T_PHDR, kci.phnum, EV_CURRENT)) as off_t;
            offset = round_up(offset as u64, page_size) as off_t;
            let mut node = kci.phdrs.next;
            while node != &mut kci.phdrs {
                let p = node as *mut phdr_data;
                let offs = (*p).rel + offset;
                if kcore__add_phdr(&mut extract, idx, offs, (*p).addr, (*p).len) != 0 { break; }
                idx += 1;
                node = (*node).next;
            }
            let sz = kcore__write(&mut extract);
            if sz >= 0 && sz <= offset {
                err = 0;
                node = kci.phdrs.next;
                while node != &mut kci.phdrs {
                    let p = node as *mut phdr_data;
                    let offs = (*p).rel + offset;
                    if (*p).remaps.is_null() && copy_bytes(kcore_.fd, (*p).offset, extract.fd, offs, (*p).len) != 0 { err = -1; break; }
                    node = (*node).next;
                }
                if err == 0 && kcore_copy__compare_file(from_dir, to_dir, b"kallsyms\0".as_ptr() as *const c_char) != 0 { err = -1; }
            }
        }
        kcore__close(&mut extract);
        if err != 0 { unlink(extract_filename.as_ptr()); }
        kcore__close(&mut kcore_);
    } else if kcore_.fd != 0 { kcore__close(&mut kcore_); }
    if err != 0 {
        kcore_copy__unlink(to_dir, b"modules\0".as_ptr() as *const c_char);
        kcore_copy__unlink(to_dir, b"kallsyms\0".as_ptr() as *const c_char);
    }
    kcore_copy__free_phdrs(&mut kci);
    kcore_copy__free_syms(&mut kci);
    err
}

#[no_mangle]
pub unsafe extern "C" fn kcore_extract__create(kce: *mut kcore_extract) -> c_int {
    let mut kcore_: kcore = zeroed();
    let mut extract: kcore = zeroed();
    let count: size_t = 1;
    let idx = 0;
    let mut err = -1;
    let offset: off_t = page_size as off_t;
    if kcore__open(&mut kcore_, (*kce).kcore_filename) != 0 { return -1; }
    strcpy_rs((*kce).extract_filename.as_mut_ptr(), PERF_KCORE_EXTRACT.as_ptr() as *const c_char);
    if kcore__init(&mut extract, (*kce).extract_filename.as_mut_ptr(), kcore_.elfclass, true) == 0 {
        if kcore__copy_hdr(&mut kcore_, &mut extract, count) == 0 && kcore__add_phdr(&mut extract, idx, offset, (*kce).addr, (*kce).len) == 0 {
            let sz = kcore__write(&mut extract);
            if sz >= 0 && sz <= offset && copy_bytes(kcore_.fd, (*kce).offs, extract.fd, offset, (*kce).len) == 0 { err = 0; }
        }
        kcore__close(&mut extract);
        if err != 0 { unlink((*kce).extract_filename.as_ptr()); }
    }
    kcore__close(&mut kcore_);
    err
}
unsafe fn strcpy_rs(dst: *mut c_char, src: *const c_char) -> *mut c_char {
    let len = strlen(src) + 1;
    memcpy(dst as *mut c_void, src as *const c_void, len);
    dst
}
#[no_mangle] pub unsafe extern "C" fn kcore_extract__delete(kce: *mut kcore_extract) { unlink((*kce).extract_filename.as_ptr()); }

/* HAVE_GELF_GETNOTE_SUPPORT: SDT note support translated from the conditional C block. */
unsafe fn sdt_adjust_loc(tmp: *mut sdt_note, base_off: GElf_Addr) {
    if base_off == 0 { return; }
    if (*tmp).bit32 {
        (*tmp).addr.a32[SDT_NOTE_IDX_LOC] = (*tmp).addr.a32[SDT_NOTE_IDX_LOC].wrapping_add(base_off as u32).wrapping_sub((*tmp).addr.a32[SDT_NOTE_IDX_BASE]);
    } else {
        (*tmp).addr.a64[SDT_NOTE_IDX_LOC] = (*tmp).addr.a64[SDT_NOTE_IDX_LOC].wrapping_add(base_off).wrapping_sub((*tmp).addr.a64[SDT_NOTE_IDX_BASE]);
    }
}
unsafe fn sdt_adjust_refctr(tmp: *mut sdt_note, base_addr: GElf_Addr, base_off: GElf_Addr) {
    if base_off == 0 { return; }
    if (*tmp).bit32 && (*tmp).addr.a32[SDT_NOTE_IDX_REFCTR] != 0 {
        (*tmp).addr.a32[SDT_NOTE_IDX_REFCTR] = (*tmp).addr.a32[SDT_NOTE_IDX_REFCTR].wrapping_sub((base_addr - base_off) as u32);
    } else if (*tmp).addr.a64[SDT_NOTE_IDX_REFCTR] != 0 {
        (*tmp).addr.a64[SDT_NOTE_IDX_REFCTR] = (*tmp).addr.a64[SDT_NOTE_IDX_REFCTR].wrapping_sub(base_addr - base_off);
    }
}

unsafe fn populate_sdt_note(elf: *mut *mut Elf, data: *const c_char, len: size_t, sdt_notes: *mut list_head) -> c_int {
    let mut ehdr: GElf_Ehdr = zeroed();
    let mut shdr: GElf_Shdr = zeroed();
    let mut ret = -EINVAL;
    let mut buf64 = [0u64; NR_ADDR];
    let mut dst = Elf_Data { d_buf: buf64.as_mut_ptr() as *mut c_void, d_type: ELF_T_ADDR, d_version: EV_CURRENT, d_size: gelf_fsize(*elf, ELF_T_ADDR, NR_ADDR, EV_CURRENT), d_off: 0, d_align: 0 };
    let src = Elf_Data { d_buf: data as *mut c_void, d_type: ELF_T_ADDR, d_version: EV_CURRENT, d_size: dst.d_size, d_off: 0, d_align: 0 };
    let tmp = calloc(1, size_of::<sdt_note>()) as *mut sdt_note;
    if tmp.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(&mut (*tmp).note_list);
    if len < dst.d_size + 3 { free(tmp as *mut c_void); return ret; }
    if gelf_xlatetom(*elf, &mut dst, &src, *elf_getident(*elf, ptr::null_mut()).add(EI_DATA) as c_uint).is_null() {
        pr_err(b"gelf_xlatetom : %s\n\0".as_ptr() as *const c_char, elf_errmsg(-1));
        free(tmp as *mut c_void);
        return ret;
    }
    let provider = data.add(dst.d_size);
    let mut name = memchr(provider as *const c_void, 0, data.add(len) as usize - provider as usize) as *const c_char;
    if name.is_null() { free(tmp as *mut c_void); return ret; }
    name = name.add(1);
    (*tmp).provider = strdup(provider);
    if (*tmp).provider.is_null() { free(tmp as *mut c_void); return -ENOMEM; }
    (*tmp).name = strdup(name);
    if (*tmp).name.is_null() { zfree(&mut (*tmp).provider); free(tmp as *mut c_void); return -ENOMEM; }
    let args0 = memchr(name as *const c_void, 0, data.add(len) as usize - name as usize) as *const c_char;
    if args0.is_null() || data.add(len) as usize - args0 as usize < 2 || *args0.add(1) == b':' as c_char || *args0.add(1) == 0 {
        (*tmp).args = ptr::null_mut();
    } else {
        (*tmp).args = strdup(args0.add(1));
        if (*tmp).args.is_null() { zfree(&mut (*tmp).name); zfree(&mut (*tmp).provider); free(tmp as *mut c_void); return -ENOMEM; }
    }
    if gelf_getclass(*elf) == ELFCLASS32 {
        memcpy(&mut (*tmp).addr as *mut _ as *mut c_void, buf64.as_ptr() as *const c_void, 3 * size_of::<Elf32_Addr>());
        (*tmp).bit32 = true;
    } else {
        memcpy(&mut (*tmp).addr as *mut _ as *mut c_void, buf64.as_ptr() as *const c_void, 3 * size_of::<Elf64_Addr>());
        (*tmp).bit32 = false;
    }
    if gelf_getehdr(*elf, &mut ehdr).is_null() {
        ret = -EBADF;
        zfree(&mut (*tmp).args); zfree(&mut (*tmp).name); zfree(&mut (*tmp).provider); free(tmp as *mut c_void);
        return ret;
    }
    if !elf_section_by_name(*elf, &mut ehdr, &mut shdr, SDT_BASE_SCN.as_ptr() as *const c_char, ptr::null_mut()).is_null() { sdt_adjust_loc(tmp, shdr.sh_offset); }
    if !elf_section_by_name(*elf, &mut ehdr, &mut shdr, SDT_PROBES_SCN.as_ptr() as *const c_char, ptr::null_mut()).is_null() { sdt_adjust_refctr(tmp, shdr.sh_addr, shdr.sh_offset); }
    list_add_tail(&mut (*tmp).note_list, sdt_notes);
    0
}

unsafe fn construct_sdt_notes_list(elf: *mut Elf, sdt_notes: *mut list_head) -> c_int {
    let mut ehdr: GElf_Ehdr = zeroed();
    let mut shdr: GElf_Shdr = zeroed();
    let mut shstrndx: size_t = 0;
    let mut ret = 0;
    if gelf_getehdr(elf, &mut ehdr).is_null() { return -EBADF; }
    if elf_getshdrstrndx(elf, &mut shstrndx) != 0 { return -EBADF; }
    let scn = elf_section_by_name(elf, &mut ehdr, &mut shdr, SDT_NOTE_SCN.as_ptr() as *const c_char, ptr::null_mut());
    if scn.is_null() { return -ENOENT; }
    if shdr.sh_type != SHT_NOTE || (shdr.sh_flags & SHF_ALLOC) != 0 { return -ENOENT; }
    let data = elf_getdata(scn, ptr::null_mut());
    let mut offset: size_t = 0;
    loop {
        let mut nhdr: GElf_Nhdr = zeroed();
        let mut name_off: size_t = 0;
        let mut desc_off: size_t = 0;
        let next = gelf_getnote(data, offset, &mut nhdr, &mut name_off, &mut desc_off);
        if next <= 0 { break; }
        if nhdr.n_namesz as usize == SDT_NOTE_NAME.len() && memcmp(((*data).d_buf as *mut u8).add(name_off) as *const c_void, SDT_NOTE_NAME.as_ptr() as *const c_void, SDT_NOTE_NAME.len()) == 0 {
            if nhdr.n_type != SDT_NOTE_TYPE { return ret; }
            let mut elf_mut = elf;
            ret = populate_sdt_note(&mut elf_mut, ((*data).d_buf as *const c_char).add(desc_off), nhdr.n_descsz as size_t, sdt_notes);
            if ret < 0 { return ret; }
        }
        offset = next;
    }
    if list_empty(sdt_notes) { ret = -ENOENT; }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn get_sdt_note_list(head: *mut list_head, target: *const c_char) -> c_int {
    let fd = open(target, O_RDONLY | O_CLOEXEC);
    if fd < 0 { return -EBADF; }
    let elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, ptr::null_mut());
    let ret;
    if elf.is_null() { ret = -EBADF; }
    else { ret = construct_sdt_notes_list(elf, head); elf_end(elf); }
    close(fd);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_sdt_note_list(sdt_notes: *mut list_head) -> c_int {
    let mut nr_free = 0;
    let mut node = (*sdt_notes).next;
    while node != sdt_notes {
        let next = (*node).next;
        let pos = node as *mut sdt_note;
        list_del_init(&mut (*pos).note_list);
        zfree(&mut (*pos).args);
        zfree(&mut (*pos).name);
        zfree(&mut (*pos).provider);
        free(pos as *mut c_void);
        nr_free += 1;
        node = next;
    }
    nr_free
}

#[no_mangle]
pub unsafe extern "C" fn sdt_notes__get_count(start: *mut list_head) -> c_int {
    let mut count = 0;
    let mut node = (*start).next;
    while node != start {
        count += 1;
        node = (*node).next;
    }
    count
}

#[no_mangle]
pub unsafe extern "C" fn symbol__elf_init() {
    elf_version(EV_CURRENT);
}
