// SPDX-License-Identifier: GPL-2.0-only
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type size_t = usize;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type s64 = i64;
type bool_ = bool;

const EF_CSKY_ABIMASK: u32 = 0xF0000000;
const EM_386: u16 = 3;
const EM_MIPS: u16 = 8;
const EM_PPC: u16 = 20;
const EM_PPC64: u16 = 21;
const EM_ARM: u16 = 40;
const EM_SPARC: u16 = 2;
const EM_S390: u16 = 22;
const EM_X86_64: u16 = 62;
const EM_AARCH64: u16 = 183;
const EM_RISCV: u16 = 243;
const EM_CSKY: u16 = 252;
const EM_ARC: u16 = 45;
const EM_LOONGARCH: u16 = 258;
const EM_SPARCV9: u16 = 43;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const ENOTSUP: c_int = 95;
const R_OK: c_int = 4;
const PATH_MAX: usize = 4096;
const SBUILD_ID_SIZE: usize = 41;
const KMOD_DECOMP_LEN: usize = 256;
const DSO__NAME_KALLSYMS: *const c_char = b"kallsyms\0".as_ptr() as *const c_char;
const DSO_BINARY_TYPE__KALLSYMS: c_int = 0;
const DSO_BINARY_TYPE__NOT_FOUND: c_int = 1;
const DSO_BINARY_TYPE__BUILD_ID_CACHE: c_int = 2;
const DSO_BINARY_TYPE__BPF_PROG_INFO: c_int = 3;
const DSO_BINARY_TYPE__BPF_IMAGE: c_int = 4;
const SYMBOL_ANNOTATE_ERRNO__NO_VMLINUX: c_int = -1000;
const SYMBOL_ANNOTATE_ERRNO__NO_LIBOPCODES_FOR_BPF: c_int = -1001;
const SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_REGEXP: c_int = -1002;
const SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_CPUID_PARSING: c_int = -1003;
const SYMBOL_ANNOTATE_ERRNO__BPF_INVALID_FILE: c_int = -1004;
const SYMBOL_ANNOTATE_ERRNO__BPF_MISSING_BTF: c_int = -1005;
const SYMBOL_ANNOTATE_ERRNO__COULDNT_DETERMINE_FILE_TYPE: c_int = -1006;
const PERF_DISASM_UNKNOWN: c_int = 0;
const PERF_DISASM_LLVM: c_int = 1;
const PERF_DISASM_CAPSTONE: c_int = 2;
const PERF_DISASM_OBJDUMP: c_int = 3;
const RAW_BYTES: usize = 11;

#[repr(C)]
pub struct regex_t {
    _data: [usize; 8],
}

#[repr(C)]
pub struct regmatch_t {
    pub rm_so: c_long,
    pub rm_eo: c_long,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct e_machine_and_e_flags {
    pub e_machine: u16,
    pub e_flags: u32,
}

#[repr(C)]
pub struct objdump_info {
    pub skip_functions_char: c_char,
    pub comment_char: c_char,
    pub register_char: c_char,
    pub memory_ref_char: c_char,
}

#[repr(C)]
pub struct ins {
    pub name: *const c_char,
    pub ops: *const ins_ops,
}

#[repr(C)]
pub struct operand {
    pub raw: *mut c_char,
    pub name: *mut c_char,
    pub addr: u64,
    pub sym: *mut symbol,
    pub offset: s64,
    pub offset_avail: bool,
    pub outside: bool,
    pub multi_regs: bool,
}

#[repr(C)]
pub struct jump_operand {
    pub raw_comment: *const c_char,
    pub raw_func_start: *const c_char,
}

#[repr(C)]
pub struct locked_operand {
    pub ins: ins,
    pub ops: *mut ins_operands,
}

#[repr(C)]
pub struct ins_operands {
    pub raw: *mut c_char,
    pub source: operand,
    pub target: operand,
    pub jump: jump_operand,
    pub locked: locked_operand,
}

#[repr(C)]
pub struct ins_ops {
    pub free: Option<unsafe extern "C" fn(*mut ins_operands)>,
    pub parse: Option<unsafe extern "C" fn(*const arch, *mut ins_operands, *mut map_symbol, *mut disasm_line) -> c_int>,
    pub scnprintf: Option<unsafe extern "C" fn(*const ins, *mut c_char, size_t, *mut ins_operands, c_int) -> c_int>,
    pub is_jump: bool,
    pub is_call: bool,
}

#[repr(C)]
pub struct arch {
    pub id: e_machine_and_e_flags,
    pub instructions: *const ins,
    pub nr_instructions: c_int,
    pub nr_instructions_allocated: c_int,
    pub sorted_instructions: bool,
    pub objdump: objdump_info,
    pub insn_suffix: *const c_char,
    pub ins_is_fused: Option<unsafe extern "C" fn(*const arch, *const c_char, *const c_char) -> bool>,
    pub associate_instruction_ops: Option<unsafe extern "C" fn(*mut arch, *const c_char) -> *const ins_ops>,
}

#[repr(C)]
pub struct annotation_line {
    pub node: list_head,
    pub offset: s64,
    pub line: *mut c_char,
    pub line_nr: c_int,
    pub fileloc: *mut c_char,
    pub data_nr: c_int,
    pub path: *mut c_char,
    pub cycles: *mut c_void,
    pub br_cntr: *mut c_void,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct raw_disasm {
    pub raw_insn: u32,
}

#[repr(C)]
pub struct disasm_line {
    pub ins: ins,
    pub ops: ins_operands,
    pub raw: raw_disasm,
    pub al: annotation_line,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
    pub thread: *mut thread,
}

#[repr(C)]
pub struct addr_map_symbol {
    pub addr: u64,
    pub al_addr: u64,
    pub ms: map_symbol,
}

#[repr(C)]
pub struct symbol {
    pub start: u64,
    pub end: u64,
    pub name: *const c_char,
}

#[repr(C)]
pub struct annotation_source {
    pub nr_events: c_int,
    pub source: list_head,
}

#[repr(C)]
pub struct annotation {
    pub src: *mut annotation_source,
}

#[repr(C)]
pub struct annotation_options {
    pub objdump_path: *const c_char,
    pub disassembler_style: *const c_char,
    pub show_linenr: bool,
    pub show_asm_raw: bool,
    pub annotate_src: bool,
    pub hide_src_code: bool,
    pub prefix: *const c_char,
    pub prefix_strip: *const c_char,
    pub disassembler_used: c_int,
    pub disassemblers: [c_int; 8],
}

#[repr(C)]
pub struct annotate_args {
    pub arch: *const arch,
    pub ms: *mut map_symbol,
    pub options: *mut annotation_options,
    pub offset: s64,
    pub line: *mut c_char,
    pub line_nr: c_int,
    pub fileloc: *mut c_char,
}

#[repr(C)]
pub struct child_process {
    pub argv: *const *const c_char,
    pub out: c_int,
    pub err: c_int,
    pub no_stderr: c_int,
}

#[repr(C)]
pub struct kcore_extract {
    pub addr: u64,
    pub kcore_filename: *mut c_char,
    pub len: u64,
    pub offs: u64,
    pub extract_filename: *mut c_char,
}

pub enum map {}
pub enum maps {}
pub enum dso {}
pub enum thread {}
pub enum FILE {}

unsafe extern "C" {
    static mut errno: c_int;
    static mut annotate_opts: annotation_options;
    static perf_disassembler__strs: [*const c_char; 4];
    static sort_order: *const c_char;

    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(preg: *const regex_t, string: *const c_char, nmatch: size_t, pmatch: *mut regmatch_t, eflags: c_int) -> c_int;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn atoi(nptr: *const c_char) -> c_int;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
    fn bsearch(key: *const c_void, base: *const c_void, nmemb: size_t, size: size_t, compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int) -> *mut c_void;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: size_t) -> isize;
    fn dirname(path: *mut c_char) -> *mut c_char;
    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn feof(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> isize;
    fn isspace(c: c_int) -> c_int;

    fn skip_spaces(str_: *const c_char) -> *mut c_char;
    fn strim(s: *mut c_char) -> *mut c_char;
    fn remove_spaces(s: *mut c_char);
    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn zfree_srcline(ptr: *mut *mut c_char);
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    fn strlcpy(dest: *mut c_char, src: *const c_char, size: size_t) -> size_t;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn BUG_ON(cond: bool);
    fn start_command(cmd: *mut child_process) -> c_int;
    fn finish_command(cmd: *mut child_process) -> c_int;

    fn arch__new_x86(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    fn arch__new_arc(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    fn arch__new_arm(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    fn arch__new_arm64(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    fn arch__new_csky(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    fn arch__new_loongarch(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    fn arch__new_mips(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    fn arch__new_powerpc(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    fn arch__new_riscv64(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    fn arch__new_s390(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    fn arch__new_sparc(id: *const e_machine_and_e_flags, cpuid: *const c_char) -> *const arch;
    fn check_ppc_insn(dl: *mut disasm_line) -> *const ins_ops;
    fn be32_to_cpu(x: u32) -> u32;
    fn map__get(map: *mut map) -> *mut map;
    fn map__objdump_2mem(map: *mut map, ip: u64) -> u64;
    fn map__rip_2objdump(map: *mut map, ip: u64) -> u64;
    fn map__map_ip(map: *mut map, ip: u64) -> u64;
    fn map__unmap_ip(map: *mut map, ip: u64) -> u64;
    fn map__dso(map: *mut map) -> *mut dso;
    fn maps__find_ams(maps: *mut maps, target: *mut addr_map_symbol) -> c_int;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn addr_map_symbol__exit(target: *mut addr_map_symbol);
    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
    fn annotation_line__add(al: *mut annotation_line, list: *mut list_head);
    fn disasm_line__has_local_offset(dl: *mut disasm_line) -> bool;
    fn dso__has_build_id(dso: *mut dso) -> bool;
    fn dso__bid(dso: *mut dso) -> *const c_void;
    fn build_id__snprintf(bid: *const c_void, bf: *mut c_char, size: size_t) -> c_int;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__symtab_type(dso: *mut dso) -> c_int;
    fn dso__binary_type(dso: *mut dso) -> c_int;
    fn dso__set_binary_type(dso: *mut dso, ty: c_int);
    fn dso__is_kcore(dso: *mut dso) -> bool;
    fn dso__kernel(dso: *mut dso) -> bool;
    fn dso__build_id_filename(dso: *mut dso, bf: *mut c_char, size: size_t, is_debug: bool) -> *mut c_char;
    fn __symbol__join_symfs(filename: *mut c_char, size: size_t, path: *const c_char);
    fn dso__lock(dso: *mut dso) -> *mut c_void;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn dso__nsinfo(dso: *mut dso) -> *mut c_void;
    fn dso__filename_with_chroot(dso: *mut dso, filename: *const c_char) -> *mut c_char;
    fn dso__data_read_offset(dso: *mut dso, machine: *mut c_void, offset: u64, buf: *mut u8, size: u64) -> c_int;
    fn symbol__disassemble_bpf_libbfd(sym: *mut symbol, args: *mut annotate_args) -> c_int;
    fn dso__needs_decompress(dso: *mut dso) -> bool;
    fn dso__decompress_kmodule_path(dso: *mut dso, path: *mut c_char, tmp: *mut c_char, size: size_t) -> c_int;
    fn kcore_extract__create(kce: *mut kcore_extract) -> c_int;
    fn kcore_extract__delete(kce: *mut kcore_extract);
    fn symbol__disassemble_capstone_powerpc(filename: *mut c_char, sym: *mut symbol, args: *mut annotate_args) -> c_int;
    fn symbol__disassemble_capstone(filename: *mut c_char, sym: *mut symbol, args: *mut annotate_args) -> c_int;
    fn symbol__disassemble_llvm(filename: *mut c_char, sym: *mut symbol, args: *mut annotate_args) -> c_int;
}

static mut file_lineno: regex_t = regex_t { _data: [0; 8] };
static mut archs: *mut *const arch = ptr::null_mut();
static mut num_archs: size_t = 0;

#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
static INIT_REGEXPR: unsafe extern "C" fn() = symbol__init_regexpr;

unsafe extern "C" fn symbol__init_regexpr() {
    unsafe {
        regcomp(&raw mut file_lineno, c"^/[^:]+:([0-9]+)".as_ptr(), 1);
    }
}

unsafe fn cstr_byte_at(p: *const c_char, idx: isize) -> c_char {
    unsafe { *p.offset(idx) }
}

unsafe fn zfree_char(p: *mut *mut c_char) {
    unsafe { zfree(p as *mut *mut c_void) }
}

unsafe fn list_empty(head: *const list_head) -> bool {
    unsafe { (*head).next == head as *mut list_head }
}

unsafe fn list_del_init(entry: *mut list_head) {
    unsafe {
        (*(*entry).next).prev = (*entry).prev;
        (*(*entry).prev).next = (*entry).next;
        (*entry).next = entry;
        (*entry).prev = entry;
    }
}

unsafe fn list_entry_disasm_line(node: *mut list_head) -> *mut disasm_line {
    let off = core::mem::offset_of!(disasm_line, al) + core::mem::offset_of!(annotation_line, node);
    (node as *mut u8).wrapping_sub(off) as *mut disasm_line
}

unsafe fn arch__grow_instructions(archp: *mut arch) -> c_int {
    unsafe {
        let new_nr_allocated: c_int;
        let new_instructions: *mut ins;
        if (*archp).nr_instructions_allocated == 0 && !(*archp).instructions.is_null() {
            new_nr_allocated = (*archp).nr_instructions + 128;
            new_instructions = calloc(new_nr_allocated as size_t, size_of::<ins>()) as *mut ins;
            if new_instructions.is_null() {
                return -1;
            }
            memcpy(new_instructions as *mut c_void, (*archp).instructions as *const c_void, (*archp).nr_instructions as size_t * size_of::<ins>());
        } else {
            new_nr_allocated = (*archp).nr_instructions_allocated + 128;
            new_instructions = realloc((*archp).instructions as *mut c_void, new_nr_allocated as size_t * size_of::<ins>()) as *mut ins;
            if new_instructions.is_null() {
                return -1;
            }
        }
        (*archp).instructions = new_instructions;
        (*archp).nr_instructions_allocated = new_nr_allocated;
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__associate_ins_ops(archp: *mut arch, name: *const c_char, ops: *const ins_ops) -> c_int {
    unsafe {
        if (*archp).nr_instructions == (*archp).nr_instructions_allocated && arch__grow_instructions(archp) != 0 {
            return -1;
        }
        let insp = ((*archp).instructions as *mut ins).add((*archp).nr_instructions as usize);
        (*insp).name = strdup(name);
        if (*insp).name.is_null() {
            return -1;
        }
        (*insp).ops = ops;
        (*archp).nr_instructions += 1;
        ins__sort(archp);
        0
    }
}

unsafe fn e_machine_and_eflags__cmp(v1: *const e_machine_and_e_flags, v2: *const e_machine_and_e_flags) -> c_int {
    unsafe {
        if (*v1).e_machine == (*v2).e_machine {
            if (*v1).e_machine != EM_CSKY {
                return 0;
            }
            if ((*v1).e_flags & EF_CSKY_ABIMASK) < ((*v2).e_flags & EF_CSKY_ABIMASK) {
                return -1;
            }
            return (((*v1).e_flags & EF_CSKY_ABIMASK) > ((*v2).e_flags & EF_CSKY_ABIMASK)) as c_int;
        }
        if (*v1).e_machine < (*v2).e_machine { -1 } else { 1 }
    }
}

unsafe extern "C" fn arch__key_cmp(key: *const c_void, archp: *const c_void) -> c_int {
    unsafe {
        let archpp = archp as *const *const arch;
        e_machine_and_eflags__cmp(key as *const e_machine_and_e_flags, &(*(*archpp)).id)
    }
}

unsafe extern "C" fn arch__cmp(a: *const c_void, b: *const c_void) -> c_int {
    unsafe {
        let aa = a as *const *const arch;
        let ab = b as *const *const arch;
        e_machine_and_eflags__cmp(&(*(*aa)).id, &(*(*ab)).id)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__find(e_machine: u16, e_flags: u32, cpuid: *const c_char) -> *const arch {
    unsafe {
        let mut fns: [Option<unsafe extern "C" fn(*const e_machine_and_e_flags, *const c_char) -> *const arch>; 259] = [None; 259];
        fns[EM_386 as usize] = Some(arch__new_x86);
        fns[EM_ARC as usize] = Some(arch__new_arc);
        fns[EM_ARM as usize] = Some(arch__new_arm);
        fns[EM_AARCH64 as usize] = Some(arch__new_arm64);
        fns[EM_CSKY as usize] = Some(arch__new_csky);
        fns[EM_LOONGARCH as usize] = Some(arch__new_loongarch);
        fns[EM_MIPS as usize] = Some(arch__new_mips);
        fns[EM_PPC64 as usize] = Some(arch__new_powerpc);
        fns[EM_PPC as usize] = Some(arch__new_powerpc);
        fns[EM_RISCV as usize] = Some(arch__new_riscv64);
        fns[EM_S390 as usize] = Some(arch__new_s390);
        fns[EM_SPARC as usize] = Some(arch__new_sparc);
        fns[EM_SPARCV9 as usize] = Some(arch__new_sparc);
        fns[EM_X86_64 as usize] = Some(arch__new_x86);
        let key = e_machine_and_e_flags { e_machine, e_flags };
        let mut result: *const arch = ptr::null();
        if num_archs > 0 {
            let tmp = bsearch(&key as *const _ as *const c_void, archs as *const c_void, num_archs, size_of::<*const arch>(), arch__key_cmp) as *mut *const arch;
            if !tmp.is_null() {
                result = *tmp;
            }
        }
        if !result.is_null() {
            return result;
        }
        if e_machine as usize >= fns.len() || fns[e_machine as usize].is_none() {
            errno = ENOTSUP;
            return ptr::null();
        }
        let tmp = reallocarray(archs as *mut c_void, num_archs + 1, size_of::<*const arch>()) as *mut *const arch;
        if tmp.is_null() {
            return ptr::null();
        }
        archs = tmp;
        result = fns[e_machine as usize].unwrap()(&key, cpuid);
        if result.is_null() {
            pr_err(c"%s: failed to initialize %u arch priv area\n".as_ptr(), c"arch__find".as_ptr(), e_machine as c_uint);
            return ptr::null();
        }
        *archs.add(num_archs) = result;
        num_archs += 1;
        qsort(archs as *mut c_void, num_archs, size_of::<*const arch>(), arch__cmp);
        result
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__is_x86(archp: *const arch) -> bool {
    unsafe { (*archp).id.e_machine == EM_386 || (*archp).id.e_machine == EM_X86_64 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__is_powerpc(archp: *const arch) -> bool {
    unsafe { (*archp).id.e_machine == EM_PPC || (*archp).id.e_machine == EM_PPC64 }
}

unsafe fn ins_ops__delete(ops: *mut ins_operands) {
    unsafe {
        if ops.is_null() {
            return;
        }
        zfree_char(&mut (*ops).source.raw);
        zfree_char(&mut (*ops).source.name);
        zfree_char(&mut (*ops).target.raw);
        zfree_char(&mut (*ops).target.name);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ins__raw_scnprintf(insn: *const ins, bf: *mut c_char, size: size_t, ops: *mut ins_operands, max_ins_name: c_int) -> c_int {
    unsafe { scnprintf(bf, size, c"%-*s %s".as_ptr(), max_ins_name, (*insn).name, (*ops).raw) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ins__scnprintf(insn: *const ins, bf: *mut c_char, size: size_t, ops: *mut ins_operands, max_ins_name: c_int) -> c_int {
    unsafe {
        if let Some(f) = (*(*insn).ops).scnprintf {
            return f(insn, bf, size, ops, max_ins_name);
        }
        ins__raw_scnprintf(insn, bf, size, ops, max_ins_name)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ins__is_fused(archp: *const arch, ins1: *const c_char, ins2: *const c_char) -> bool {
    unsafe {
        if archp.is_null() || (*archp).ins_is_fused.is_none() {
            return false;
        }
        (*archp).ins_is_fused.unwrap()(archp, ins1, ins2)
    }
}

unsafe extern "C" fn call__parse(archp: *const arch, ops: *mut ins_operands, ms: *mut map_symbol, _dl: *mut disasm_line) -> c_int {
    unsafe {
        let map_p = (*ms).map;
        let mut endptr: *mut c_char = ptr::null_mut();
        (*ops).target.addr = strtoull((*ops).raw, &mut endptr, 16) as u64;
        let mut name = strchr(endptr, '<' as c_int);
        if name.is_null() {
            let tok = strchr(endptr, '*' as c_int);
            if !tok.is_null() {
                endptr = endptr.add(1);
                if strstr(endptr, c"(%r".as_ptr()).is_null() {
                    (*ops).target.addr = strtoull(endptr, ptr::null_mut(), 16) as u64;
                }
            }
        } else {
            name = name.add(1);
            if (*archp).objdump.skip_functions_char != 0 && !strchr(name, (*archp).objdump.skip_functions_char as c_int).is_null() {
                return -1;
            }
            let tok = strchr(name, '>' as c_int);
            if tok.is_null() {
                return -1;
            }
            *tok = 0;
            (*ops).target.name = strdup(name);
            *tok = '>' as c_char;
            if (*ops).target.name.is_null() {
                return -1;
            }
        }
        let mut target: addr_map_symbol = zeroed();
        target.ms.map = map__get(map_p);
        target.addr = map__objdump_2mem(map_p, (*ops).target.addr);
        if maps__find_ams(thread__maps((*ms).thread), &mut target) == 0
            && map__rip_2objdump(target.ms.map, map__map_ip(target.ms.map, target.addr)) == (*ops).target.addr {
            (*ops).target.sym = target.ms.sym;
        }
        addr_map_symbol__exit(&mut target);
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn call__scnprintf(insn: *const ins, bf: *mut c_char, size: size_t, ops: *mut ins_operands, max_ins_name: c_int) -> c_int {
    unsafe {
        if !(*ops).target.sym.is_null() {
            return scnprintf(bf, size, c"%-*s %s".as_ptr(), max_ins_name, (*insn).name, (*(*ops).target.sym).name);
        }
        if (*ops).target.addr == 0 {
            return ins__raw_scnprintf(insn, bf, size, ops, max_ins_name);
        }
        if !(*ops).target.name.is_null() {
            return scnprintf(bf, size, c"%-*s %s".as_ptr(), max_ins_name, (*insn).name, (*ops).target.name);
        }
        scnprintf(bf, size, c"%-*s *%llx".as_ptr(), max_ins_name, (*insn).name, (*ops).target.addr as c_ulonglong)
    }
}

#[unsafe(no_mangle)]
pub static call_ops: ins_ops = ins_ops { free: None, parse: Some(call__parse), scnprintf: Some(call__scnprintf), is_jump: false, is_call: true };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ins__is_call(insn: *const ins) -> bool {
    unsafe { !(*insn).ops.is_null() && (*(*insn).ops).is_call }
}

unsafe fn validate_comma(c: *const c_char, ops: *mut ins_operands) -> *const c_char {
    unsafe {
        if !(*ops).jump.raw_comment.is_null() && c > (*ops).jump.raw_comment {
            return ptr::null();
        }
        if !(*ops).jump.raw_func_start.is_null() && c > (*ops).jump.raw_func_start {
            return ptr::null();
        }
        c
    }
}

unsafe extern "C" fn jump__parse(archp: *const arch, ops: *mut ins_operands, ms: *mut map_symbol, _dl: *mut disasm_line) -> c_int {
    unsafe {
        let map_p = (*ms).map;
        let sym = (*ms).sym;
        let mut target: addr_map_symbol = zeroed();
        target.ms.map = map__get(map_p);
        let mut c = strchr((*ops).raw, ',' as c_int);
        (*ops).jump.raw_comment = strchr((*ops).raw, (*archp).objdump.comment_char as c_int);
        (*ops).jump.raw_func_start = strchr((*ops).raw, '<' as c_int);
        c = validate_comma(c, ops) as *mut c_char;
        if !c.is_null() {
            c = c.add(1);
            (*ops).target.addr = strtoull(c, ptr::null_mut(), 16) as u64;
            if (*ops).target.addr == 0 {
                c = strchr(c, ',' as c_int);
                c = validate_comma(c, ops) as *mut c_char;
                if !c.is_null() {
                    c = c.add(1);
                    (*ops).target.addr = strtoull(c, ptr::null_mut(), 16) as u64;
                }
            }
        } else {
            (*ops).target.addr = strtoull((*ops).raw, ptr::null_mut(), 16) as u64;
        }
        target.addr = map__objdump_2mem(map_p, (*ops).target.addr);
        let start = map__unmap_ip(map_p, (*sym).start);
        let end = map__unmap_ip(map_p, (*sym).end);
        (*ops).target.outside = target.addr < start || target.addr >= end;
        if maps__find_ams(thread__maps((*ms).thread), &mut target) == 0
            && map__rip_2objdump(target.ms.map, map__map_ip(target.ms.map, target.addr)) == (*ops).target.addr {
            (*ops).target.sym = target.ms.sym;
        }
        if !(*ops).target.outside {
            (*ops).target.offset = target.addr.wrapping_sub(start) as s64;
            (*ops).target.offset_avail = true;
        } else {
            (*ops).target.offset_avail = false;
        }
        addr_map_symbol__exit(&mut target);
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jump__scnprintf(insn: *const ins, bf: *mut c_char, size: size_t, ops: *mut ins_operands, max_ins_name: c_int) -> c_int {
    unsafe {
        if (*ops).target.addr == 0 || (*ops).target.offset < 0 {
            return ins__raw_scnprintf(insn, bf, size, ops, max_ins_name);
        }
        if (*ops).target.outside && !(*ops).target.sym.is_null() {
            return scnprintf(bf, size, c"%-*s %s".as_ptr(), max_ins_name, (*insn).name, (*(*ops).target.sym).name);
        }
        let mut c = strchr((*ops).raw, ',' as c_int);
        c = validate_comma(c, ops) as *mut c_char;
        if !c.is_null() {
            let mut c2 = strchr(c.add(1), ',' as c_int);
            c2 = validate_comma(c2, ops) as *mut c_char;
            if !c2.is_null() {
                c = c2;
            }
            c = c.add(1);
            if *c == ' ' as c_char {
                c = c.add(1);
            }
        }
        let precision = if c.is_null() { 0 } else { c.offset_from((*ops).raw) as c_int };
        scnprintf(bf, size, c"%-*s %.*s%llx".as_ptr(), max_ins_name, (*insn).name, precision, (*ops).raw, (*ops).target.offset as c_ulonglong)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jump__delete(_ops: *mut ins_operands) {}

#[unsafe(no_mangle)]
pub static jump_ops: ins_ops = ins_ops { free: Some(jump__delete), parse: Some(jump__parse), scnprintf: Some(jump__scnprintf), is_jump: true, is_call: false };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ins__is_jump(insn: *const ins) -> bool {
    unsafe { !(*insn).ops.is_null() && (*(*insn).ops).is_jump }
}

unsafe fn comment__symbol(raw: *mut c_char, comment: *mut c_char, addrp: *mut u64, namep: *mut *mut c_char) -> c_int {
    unsafe {
        if strstr(raw, c"(%rip)".as_ptr()).is_null() {
            return 0;
        }
        let mut endptr: *mut c_char = ptr::null_mut();
        *addrp = strtoull(comment, &mut endptr, 16) as u64;
        if endptr == comment {
            return 0;
        }
        let mut name = strchr(endptr, '<' as c_int);
        if name.is_null() {
            return -1;
        }
        name = name.add(1);
        let t = strchr(name, '>' as c_int);
        if t.is_null() {
            return 0;
        }
        *t = 0;
        *namep = strdup(name);
        *t = '>' as c_char;
        0
    }
}

unsafe extern "C" fn lock__parse(archp: *const arch, ops: *mut ins_operands, ms: *mut map_symbol, _dl: *mut disasm_line) -> c_int {
    unsafe {
        (*ops).locked.ops = zalloc(size_of::<ins_operands>()) as *mut ins_operands;
        if (*ops).locked.ops.is_null() {
            return 0;
        }
        if disasm_line__parse((*ops).raw, &mut (*ops).locked.ins.name, &mut (*(*ops).locked.ops).raw) < 0 {
            zfree(&mut (*ops).locked.ops as *mut *mut ins_operands as *mut *mut c_void);
            return 0;
        }
        (*ops).locked.ins.ops = ins__find(archp, (*ops).locked.ins.name, ptr::null_mut());
        if (*ops).locked.ins.ops.is_null() {
            zfree(&mut (*ops).locked.ops as *mut *mut ins_operands as *mut *mut c_void);
            return 0;
        }
        if let Some(parse) = (*(*ops).locked.ins.ops).parse {
            if parse(archp, (*ops).locked.ops, ms, ptr::null_mut()) < 0 {
                zfree(&mut (*ops).locked.ops as *mut *mut ins_operands as *mut *mut c_void);
            }
        }
        0
    }
}

unsafe extern "C" fn lock__scnprintf(insn: *const ins, bf: *mut c_char, size: size_t, ops: *mut ins_operands, max_ins_name: c_int) -> c_int {
    unsafe {
        if (*ops).locked.ins.ops.is_null() {
            return ins__raw_scnprintf(insn, bf, size, ops, max_ins_name);
        }
        let printed = scnprintf(bf, size, c"%-*s ".as_ptr(), max_ins_name, (*insn).name);
        printed + ins__scnprintf(&mut (*ops).locked.ins, bf.add(printed as usize), size - printed as usize, (*ops).locked.ops, max_ins_name)
    }
}

unsafe extern "C" fn lock__delete(ops: *mut ins_operands) {
    unsafe {
        let insn = &mut (*ops).locked.ins;
        if !insn.ops.is_null() {
            if let Some(free_fn) = (*insn.ops).free {
                free_fn((*ops).locked.ops);
            } else {
                ins_ops__delete((*ops).locked.ops);
            }
        }
        zfree(&mut (*ops).locked.ops as *mut *mut ins_operands as *mut *mut c_void);
        zfree_char(&mut (*ops).locked.ins.name as *mut *const c_char as *mut *mut c_char);
        zfree_char(&mut (*ops).target.raw);
        zfree_char(&mut (*ops).target.name);
    }
}

#[unsafe(no_mangle)]
pub static lock_ops: ins_ops = ins_ops { free: Some(lock__delete), parse: Some(lock__parse), scnprintf: Some(lock__scnprintf), is_jump: false, is_call: false };

unsafe fn check_multi_regs(archp: *const arch, mut op: *const c_char) -> bool {
    unsafe {
        let mut count = 0;
        if (*archp).objdump.register_char == 0 {
            return false;
        }
        if (*archp).objdump.memory_ref_char != 0 {
            op = strchr(op, (*archp).objdump.memory_ref_char as c_int);
            if op.is_null() {
                return false;
            }
        }
        loop {
            op = strchr(op, (*archp).objdump.register_char as c_int);
            if op.is_null() {
                break;
            }
            count += 1;
            op = op.add(1);
        }
        count > 1
    }
}

unsafe extern "C" fn mov__parse(archp: *const arch, ops: *mut ins_operands, _ms: *mut map_symbol, _dl: *mut disasm_line) -> c_int {
    unsafe {
        let mut s = strchr((*ops).raw, ',' as c_int);
        if s.is_null() {
            return -1;
        }
        *s = 0;
        if !strchr((*ops).raw, '(' as c_int).is_null() {
            *s = ',' as c_char;
            s = strchr((*ops).raw, ')' as c_int);
            if s.is_null() || *s.add(1) != ',' as c_char {
                return -1;
            }
            s = s.add(1);
            *s = 0;
        }
        (*ops).source.raw = strdup((*ops).raw);
        *s = ',' as c_char;
        if (*ops).source.raw.is_null() {
            return -1;
        }
        (*ops).source.multi_regs = check_multi_regs(archp, (*ops).source.raw);
        s = s.add(1);
        let target = skip_spaces(s);
        let comment = strchr(s, (*archp).objdump.comment_char as c_int);
        if !comment.is_null() {
            s = comment.sub(1);
        } else {
            s = strchr(s, 0).sub(1);
        }
        while s > target && isspace(*s as c_int) != 0 {
            s = s.sub(1);
        }
        s = s.add(1);
        let prev = *s;
        *s = 0;
        (*ops).target.raw = strdup(target);
        *s = prev;
        if (*ops).target.raw.is_null() {
            zfree_char(&mut (*ops).source.raw);
            return -1;
        }
        (*ops).target.multi_regs = check_multi_regs(archp, (*ops).target.raw);
        if comment.is_null() {
            return 0;
        }
        let comment2 = skip_spaces(comment);
        comment__symbol((*ops).source.raw, comment2.add(1), &mut (*ops).source.addr, &mut (*ops).source.name);
        comment__symbol((*ops).target.raw, comment2.add(1), &mut (*ops).target.addr, &mut (*ops).target.name);
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mov__scnprintf(insn: *const ins, bf: *mut c_char, size: size_t, ops: *mut ins_operands, max_ins_name: c_int) -> c_int {
    unsafe {
        let src = if !(*ops).source.name.is_null() { (*ops).source.name } else { (*ops).source.raw };
        let dst = if !(*ops).target.name.is_null() { (*ops).target.name } else { (*ops).target.raw };
        scnprintf(bf, size, c"%-*s %s,%s".as_ptr(), max_ins_name, (*insn).name, src, dst)
    }
}

#[unsafe(no_mangle)]
pub static mov_ops: ins_ops = ins_ops { free: None, parse: Some(mov__parse), scnprintf: Some(mov__scnprintf), is_jump: false, is_call: false };

unsafe extern "C" fn dec__parse(archp: *const arch, ops: *mut ins_operands, _ms: *mut map_symbol, _dl: *mut disasm_line) -> c_int {
    unsafe {
        let target = (*ops).raw;
        let mut s = (*ops).raw;
        while *s != 0 && isspace(*s as c_int) == 0 {
            s = s.add(1);
        }
        let prev = *s;
        *s = 0;
        (*ops).target.raw = strdup(target);
        *s = prev;
        if (*ops).target.raw.is_null() {
            return -1;
        }
        let comment = strchr(s, (*archp).objdump.comment_char as c_int);
        if comment.is_null() {
            return 0;
        }
        let comment = skip_spaces(comment);
        comment__symbol((*ops).target.raw, comment.add(1), &mut (*ops).target.addr, &mut (*ops).target.name);
        0
    }
}

unsafe extern "C" fn dec__scnprintf(insn: *const ins, bf: *mut c_char, size: size_t, ops: *mut ins_operands, max_ins_name: c_int) -> c_int {
    unsafe {
        let dst = if !(*ops).target.name.is_null() { (*ops).target.name } else { (*ops).target.raw };
        scnprintf(bf, size, c"%-*s %s".as_ptr(), max_ins_name, (*insn).name, dst)
    }
}

#[unsafe(no_mangle)]
pub static dec_ops: ins_ops = ins_ops { free: None, parse: Some(dec__parse), scnprintf: Some(dec__scnprintf), is_jump: false, is_call: false };

unsafe extern "C" fn nop__scnprintf(_insn: *const ins, bf: *mut c_char, size: size_t, _ops: *mut ins_operands, max_ins_name: c_int) -> c_int {
    unsafe { scnprintf(bf, size, c"%-*s".as_ptr(), max_ins_name, c"nop".as_ptr()) }
}

#[unsafe(no_mangle)]
pub static nop_ops: ins_ops = ins_ops { free: None, parse: None, scnprintf: Some(nop__scnprintf), is_jump: false, is_call: false };
#[unsafe(no_mangle)]
pub static ret_ops: ins_ops = ins_ops { free: None, parse: None, scnprintf: Some(ins__raw_scnprintf), is_jump: false, is_call: false };
#[unsafe(no_mangle)]
pub static load_store_ops: ins_ops = ins_ops { free: None, parse: None, scnprintf: None, is_jump: false, is_call: false };
#[unsafe(no_mangle)]
pub static arithmetic_ops: ins_ops = ins_ops { free: None, parse: None, scnprintf: None, is_jump: false, is_call: false };

unsafe fn ins__is_nop(insn: *const ins) -> bool {
    unsafe { ptr::addr_eq((*insn).ops, &raw const nop_ops) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ins__is_ret(insn: *const ins) -> bool {
    unsafe { ptr::addr_eq((*insn).ops, &raw const ret_ops) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ins__is_lock(insn: *const ins) -> bool {
    unsafe { ptr::addr_eq((*insn).ops, &raw const lock_ops) }
}

unsafe extern "C" fn ins__key_cmp(name: *const c_void, insp: *const c_void) -> c_int {
    unsafe { strcmp(name as *const c_char, (*(insp as *const ins)).name) }
}

unsafe extern "C" fn ins__cmp(a: *const c_void, b: *const c_void) -> c_int {
    unsafe { strcmp((*(a as *const ins)).name, (*(b as *const ins)).name) }
}

unsafe fn ins__sort(archp: *mut arch) {
    unsafe { qsort((*archp).instructions as *mut c_void, (*archp).nr_instructions as size_t, size_of::<ins>(), ins__cmp) }
}

unsafe fn __ins__find(archp: *const arch, name: *const c_char, dl: *mut disasm_line) -> *const ins_ops {
    unsafe {
        if arch__is_powerpc(archp) {
            let ops = check_ppc_insn(dl);
            if !ops.is_null() {
                return ops;
            }
        }
        if !(*archp).sorted_instructions {
            ins__sort(archp as *mut arch);
            (*(archp as *mut arch)).sorted_instructions = true;
        }
        let mut found = bsearch(name as *const c_void, (*archp).instructions as *const c_void, (*archp).nr_instructions as size_t, size_of::<ins>(), ins__key_cmp) as *const ins;
        if !found.is_null() {
            return (*found).ops;
        }
        if !(*archp).insn_suffix.is_null() {
            let mut tmp = [0 as c_char; 32];
            let len = strlen(name);
            if len == 0 || len >= tmp.len() {
                return ptr::null();
            }
            let suffix = *name.add(len - 1);
            if strchr((*archp).insn_suffix, suffix as c_int).is_null() {
                return ptr::null();
            }
            strcpy(tmp.as_mut_ptr(), name);
            tmp[len - 1] = 0;
            found = bsearch(tmp.as_ptr() as *const c_void, (*archp).instructions as *const c_void, (*archp).nr_instructions as size_t, size_of::<ins>(), ins__key_cmp) as *const ins;
        }
        if !found.is_null() { (*found).ops } else { ptr::null() }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ins__find(archp: *const arch, name: *const c_char, dl: *mut disasm_line) -> *const ins_ops {
    unsafe {
        let mut ops = __ins__find(archp, name, dl);
        if ops.is_null() {
            if let Some(f) = (*archp).associate_instruction_ops {
                ops = f(archp as *mut arch, name);
            }
        }
        ops
    }
}

unsafe fn disasm_line__init_ins(dl: *mut disasm_line, archp: *const arch, ms: *mut map_symbol) {
    unsafe {
        (*dl).ins.ops = ins__find(archp, (*dl).ins.name, dl);
        if (*dl).ins.ops.is_null() {
            return;
        }
        if let Some(parse) = (*(*dl).ins.ops).parse {
            if parse(archp, &mut (*dl).ops, ms, dl) < 0 {
                (*dl).ins.ops = ptr::null();
            }
        }
    }
}

unsafe fn disasm_line__parse(line: *mut c_char, namep: *mut *const c_char, rawp: *mut *mut c_char) -> c_int {
    unsafe {
        let name = skip_spaces(line);
        if *name == 0 {
            return -1;
        }
        *rawp = name.add(1);
        while **rawp != 0 && isspace(**rawp as c_int) == 0 {
            *rawp = (*rawp).add(1);
        }
        let tmp = **rawp;
        **rawp = 0;
        *namep = strdup(name);
        if (*namep).is_null() {
            return -1;
        }
        **rawp = tmp;
        *rawp = strim(*rawp);
        0
    }
}

unsafe fn PPC_OP(op: u32) -> u32 {
    (op >> 26) & 0x3F
}

unsafe fn disasm_line__parse_powerpc(dl: *mut disasm_line, args: *mut annotate_args) -> c_int {
    unsafe {
        let line = (*dl).al.line;
        let namep = &mut (*dl).ins.name as *mut *const c_char;
        let rawp = &mut (*dl).ops.raw as *mut *mut c_char;
        let name_raw_insn = skip_spaces(line);
        let name = skip_spaces(name_raw_insn.add(RAW_BYTES));
        let mut disasm = 0;
        let mut ret = 0;
        if (*(*args).options).disassembler_used != 0 {
            disasm = 1;
        }
        if *name_raw_insn == 0 {
            return -1;
        }
        if disasm != 0 {
            ret = disasm_line__parse(name, namep, rawp);
        } else {
            *namep = c"".as_ptr();
        }
        let tmp_raw_insn = strndup(name_raw_insn, 11);
        if tmp_raw_insn.is_null() {
            return -1;
        }
        remove_spaces(tmp_raw_insn);
        sscanf(tmp_raw_insn, c"%x".as_ptr(), &mut (*dl).raw.raw_insn);
        if disasm != 0 {
            (*dl).raw.raw_insn = be32_to_cpu((*dl).raw.raw_insn);
        }
        ret
    }
}

unsafe fn annotation_line__init(al: *mut annotation_line, args: *mut annotate_args, nr: c_int) {
    unsafe {
        (*al).offset = (*args).offset;
        (*al).line = strdup((*args).line);
        (*al).line_nr = (*args).line_nr;
        (*al).fileloc = if !(*args).fileloc.is_null() { strdup((*args).fileloc) } else { ptr::null_mut() };
        (*al).data_nr = nr;
    }
}

unsafe fn annotation_line__exit(al: *mut annotation_line) {
    unsafe {
        zfree_srcline(&mut (*al).path);
        zfree_char(&mut (*al).fileloc);
        zfree_char(&mut (*al).line);
        zfree(&mut (*al).cycles as *mut *mut c_void);
        zfree(&mut (*al).br_cntr as *mut *mut c_void);
    }
}

unsafe fn disasm_line_size(nr: c_int) -> size_t {
    size_of::<disasm_line>() + (nr as size_t)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn disasm_line__new(args: *mut annotate_args) -> *mut disasm_line {
    unsafe {
        let notes = symbol__annotation((*(*args).ms).sym);
        let nr = (*(*notes).src).nr_events;
        let dl = zalloc(disasm_line_size(nr)) as *mut disasm_line;
        if dl.is_null() {
            return ptr::null_mut();
        }
        annotation_line__init(&mut (*dl).al, args, nr);
        if (*dl).al.line.is_null() {
            annotation_line__exit(&mut (*dl).al);
            free(dl as *mut c_void);
            return ptr::null_mut();
        }
        if (*args).offset != -1 {
            if arch__is_powerpc((*args).arch) {
                if disasm_line__parse_powerpc(dl, args) < 0 {
                    annotation_line__exit(&mut (*dl).al);
                    free(dl as *mut c_void);
                    return ptr::null_mut();
                }
            } else if disasm_line__parse((*dl).al.line, &mut (*dl).ins.name, &mut (*dl).ops.raw) < 0 {
                annotation_line__exit(&mut (*dl).al);
                free(dl as *mut c_void);
                return ptr::null_mut();
            }
            disasm_line__init_ins(dl, (*args).arch, (*args).ms);
        }
        dl
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn disasm_line__free(dl: *mut disasm_line) {
    unsafe {
        if !(*dl).ins.ops.is_null() {
            if let Some(free_fn) = (*(*dl).ins.ops).free {
                free_fn(&mut (*dl).ops);
            } else {
                ins_ops__delete(&mut (*dl).ops);
            }
        } else {
            ins_ops__delete(&mut (*dl).ops);
        }
        zfree_char(&mut (*dl).ins.name as *mut *const c_char as *mut *mut c_char);
        annotation_line__exit(&mut (*dl).al);
        free(dl as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn disasm_line__scnprintf(dl: *mut disasm_line, bf: *mut c_char, size: size_t, raw: bool, max_ins_name: c_int) -> c_int {
    unsafe {
        if raw || (*dl).ins.ops.is_null() {
            return scnprintf(bf, size, c"%-*s %s".as_ptr(), max_ins_name, (*dl).ins.name, (*dl).ops.raw);
        }
        ins__scnprintf(&mut (*dl).ins, bf, size, &mut (*dl).ops, max_ins_name)
    }
}

unsafe fn symbol__parse_objdump_line(sym: *mut symbol, args: *mut annotate_args, mut parsed_line: *mut c_char, line_nr: *mut c_int, fileloc: *mut *mut c_char) -> c_int {
    unsafe {
        let map_p = (*(*args).ms).map;
        let notes = symbol__annotation(sym);
        let mut match_: [regmatch_t; 2] = zeroed();
        if regexec(&raw const file_lineno, parsed_line, 2, match_.as_mut_ptr(), 0) == 0 {
            *line_nr = atoi(parsed_line.offset(match_[1].rm_so as isize));
            free(*fileloc as *mut c_void);
            *fileloc = strdup(parsed_line);
            return 0;
        }
        let mut tmp: *mut c_char = ptr::null_mut();
        let line_ip = strtoull(parsed_line, &mut tmp, 16) as s64;
        let mut offset: s64 = -1;
        if parsed_line != tmp && *tmp == ':' as c_char && *tmp.add(1) != 0 {
            let start = map__rip_2objdump(map_p, (*sym).start);
            let end = map__rip_2objdump(map_p, (*sym).end);
            offset = line_ip - start as s64;
            if (line_ip as u64) < start || (line_ip as u64) >= end {
                offset = -1;
            } else {
                parsed_line = tmp.add(1);
            }
        }
        (*args).offset = offset;
        (*args).line = parsed_line;
        (*args).line_nr = *line_nr;
        (*args).fileloc = *fileloc;
        (*(*args).ms).sym = sym;
        let dl = disasm_line__new(args);
        *line_nr += 1;
        if dl.is_null() {
            return -1;
        }
        if !disasm_line__has_local_offset(dl) {
            (*dl).ops.target.offset = ((*dl).ops.target.addr - map__rip_2objdump(map_p, (*sym).start)) as s64;
            (*dl).ops.target.offset_avail = true;
        }
        if !(*dl).ins.ops.is_null() && ins__is_call(&mut (*dl).ins) && (*dl).ops.target.sym.is_null() {
            let mut target: addr_map_symbol = zeroed();
            target.addr = (*dl).ops.target.addr;
            target.ms.map = map__get(map_p);
            if maps__find_ams(thread__maps((*(*args).ms).thread), &mut target) == 0 && (*target.ms.sym).start == target.al_addr {
                (*dl).ops.target.sym = target.ms.sym;
            }
            addr_map_symbol__exit(&mut target);
        }
        annotation_line__add(&mut (*dl).al, &mut (*(*notes).src).source);
        0
    }
}

unsafe fn delete_last_nop(sym: *mut symbol) {
    unsafe {
        let notes = symbol__annotation(sym);
        let list = &mut (*(*notes).src).source as *mut list_head;
        while !list_empty(list) {
            let dl = list_entry_disasm_line((*list).prev);
            if !(*dl).ins.ops.is_null() {
                if !ins__is_nop(&mut (*dl).ins) {
                    return;
                }
            } else if strstr((*dl).al.line, c" nop ".as_ptr()).is_null()
                && strstr((*dl).al.line, c" nopl ".as_ptr()).is_null()
                && strstr((*dl).al.line, c" nopw ".as_ptr()).is_null() {
                return;
            }
            list_del_init(&mut (*dl).al.node);
            disasm_line__free(dl);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__strerror_disassemble(ms: *mut map_symbol, errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int {
    unsafe {
        let dso_p = map__dso((*ms).map);
        BUG_ON(buflen == 0);
        if errnum >= 0 {
            str_error_r(errnum, buf, buflen);
            return 0;
        }
        match errnum {
            SYMBOL_ANNOTATE_ERRNO__NO_VMLINUX => {
                let mut bf = [0 as c_char; SBUILD_ID_SIZE + 15];
                strcpy(bf.as_mut_ptr(), c" with build id ".as_ptr());
                let mut build_id_msg: *mut c_char = ptr::null_mut();
                if dso__has_build_id(dso_p) {
                    build_id__snprintf(dso__bid(dso_p), bf.as_mut_ptr().add(15), bf.len() - 15);
                    build_id_msg = bf.as_mut_ptr();
                }
                scnprintf(buf, buflen, c"No vmlinux file%s\nwas found in the path.\n\nNote that annotation using /proc/kcore requires CAP_SYS_RAWIO capability.\n\nPlease use:\n\n  perf buildid-cache -vu vmlinux\n\nor:\n\n  --vmlinux vmlinux\n".as_ptr(), if build_id_msg.is_null() { c"".as_ptr() } else { build_id_msg });
            }
            SYMBOL_ANNOTATE_ERRNO__NO_LIBOPCODES_FOR_BPF => { scnprintf(buf, buflen, c"Please link with binutils's libopcode to enable BPF annotation".as_ptr()); }
            SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_REGEXP => { scnprintf(buf, buflen, c"Problems with arch specific instruction name regular expressions.".as_ptr()); }
            SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_CPUID_PARSING => { scnprintf(buf, buflen, c"Problems while parsing the CPUID in the arch specific initialization.".as_ptr()); }
            SYMBOL_ANNOTATE_ERRNO__BPF_INVALID_FILE => { scnprintf(buf, buflen, c"Invalid BPF file: %s.".as_ptr(), dso__long_name(dso_p)); }
            SYMBOL_ANNOTATE_ERRNO__BPF_MISSING_BTF => { scnprintf(buf, buflen, c"The %s BPF file has no BTF section, compile with -g or use pahole -J.".as_ptr(), dso__long_name(dso_p)); }
            SYMBOL_ANNOTATE_ERRNO__COULDNT_DETERMINE_FILE_TYPE => { scnprintf(buf, buflen, c"Couldn't determine the file %s type.".as_ptr(), dso__long_name(dso_p)); }
            _ => { scnprintf(buf, buflen, c"Internal error: Invalid %d error code\n".as_ptr(), errnum); }
        }
        0
    }
}

unsafe fn dso__disassemble_filename(dso_p: *mut dso, filename: *mut c_char, filename_size: size_t) -> c_int {
    unsafe {
        let mut linkname = [0 as c_char; PATH_MAX];
        if dso__symtab_type(dso_p) == DSO_BINARY_TYPE__KALLSYMS && !dso__is_kcore(dso_p) {
            return SYMBOL_ANNOTATE_ERRNO__NO_VMLINUX;
        }
        let build_id_filename = dso__build_id_filename(dso_p, ptr::null_mut(), 0, false);
        if !build_id_filename.is_null() {
            __symbol__join_symfs(filename, filename_size, build_id_filename);
            free(build_id_filename as *mut c_void);
        } else {
            if dso__has_build_id(dso_p) {
                return ENOMEM;
            }
            if dso__kernel(dso_p) && *dso__long_name(dso_p) == '/' as c_char {
                snprintf(filename, filename_size, c"%s".as_ptr(), dso__long_name(dso_p));
            } else {
                __symbol__join_symfs(filename, filename_size, dso__long_name(dso_p));
            }
            return 0;
        }
        let build_id_path = strdup(filename);
        if build_id_path.is_null() {
            return ENOMEM;
        }
        let pos = strrchr(build_id_path, '/' as c_int);
        if !pos.is_null() && strlen(pos) < SBUILD_ID_SIZE - 2 {
            dirname(build_id_path);
        }
        let mut fallback = dso__is_kcore(dso_p);
        if !fallback {
            let len = readlink(build_id_path, linkname.as_mut_ptr(), linkname.len() - 1);
            if len < 0 {
                fallback = true;
            } else {
                linkname[len as usize] = 0;
                if !strstr(linkname.as_ptr(), DSO__NAME_KALLSYMS).is_null() || access(filename, R_OK) != 0 {
                    fallback = true;
                }
            }
        }
        if fallback {
            if dso__kernel(dso_p) && *dso__long_name(dso_p) == '/' as c_char {
                snprintf(filename, filename_size, c"%s".as_ptr(), dso__long_name(dso_p));
            } else {
                __symbol__join_symfs(filename, filename_size, dso__long_name(dso_p));
            }
            mutex_lock(dso__lock(dso_p));
            if access(filename, R_OK) != 0 && errno == ENOENT && !dso__nsinfo(dso_p).is_null() {
                let new_name = dso__filename_with_chroot(dso_p, filename);
                if !new_name.is_null() {
                    strlcpy(filename, new_name, filename_size);
                    free(new_name as *mut c_void);
                }
            }
            mutex_unlock(dso__lock(dso_p));
        } else if dso__binary_type(dso_p) == DSO_BINARY_TYPE__NOT_FOUND {
            dso__set_binary_type(dso_p, DSO_BINARY_TYPE__BUILD_ID_CACHE);
        }
        free(build_id_path as *mut c_void);
        0
    }
}

unsafe fn symbol__disassemble_raw(filename: *mut c_char, sym: *mut symbol, args: *mut annotate_args) -> c_int {
    unsafe {
        let notes = symbol__annotation(sym);
        let map_p = (*(*args).ms).map;
        let dso_p = map__dso(map_p);
        let start = map__rip_2objdump(map_p, (*sym).start);
        let end = map__rip_2objdump(map_p, (*sym).end);
        let len = end - start;
        if !(*(*args).options).objdump_path.is_null() {
            return -1;
        }
        pr_debug(c"Reading raw instruction from : %s using dso__data_read_offset\n".as_ptr(), filename);
        let buf = malloc(len as size_t) as *mut u8;
        if buf.is_null() {
            return -1;
        }
        let mut count = dso__data_read_offset(dso_p, ptr::null_mut(), (*sym).start, buf, len);
        let line = buf as *mut u32;
        if count as u64 != len {
            free(buf as *mut c_void);
            return -1;
        }
        let mut disasm_buf = [0 as c_char; 512];
        scnprintf(disasm_buf.as_mut_ptr(), disasm_buf.len(), c"%#llx <%s>:".as_ptr(), start as c_ulonglong, (*sym).name);
        (*args).offset = -1;
        (*args).line = disasm_buf.as_mut_ptr();
        (*args).line_nr = 0;
        (*args).fileloc = ptr::null_mut();
        (*(*args).ms).sym = sym;
        let mut dl = disasm_line__new(args);
        if dl.is_null() {
            free(buf as *mut c_void);
            return -1;
        }
        annotation_line__add(&mut (*dl).al, &mut (*(*notes).src).source);
        count = (len / 4) as c_int;
        let mut offset: u64 = 0;
        let mut i = 0;
        while i < count {
            (*args).offset = offset as s64;
            sprintf((*args).line, c"%x".as_ptr(), *line.add(i as usize));
            dl = disasm_line__new(args);
            if dl.is_null() {
                break;
            }
            annotation_line__add(&mut (*dl).al, &mut (*(*notes).src).source);
            offset += 4;
            i += 1;
        }
        if offset != len {
            let list = &mut (*(*notes).src).source as *mut list_head;
            while !list_empty(list) {
                dl = list_entry_disasm_line((*list).next);
                list_del_init(&mut (*dl).al.node);
                disasm_line__free(dl);
            }
            count = -1;
        }
        free(buf as *mut c_void);
        if count < 0 { count } else { 0 }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn expand_tabs(line: *mut c_char, storage: *mut *mut c_char, storage_len: *mut size_t) -> *mut c_char {
    unsafe {
        let line_len = strlen(line);
        let mut num_tabs = 0usize;
        for i in 0..line_len {
            if *line.add(i) == '\t' as c_char {
                num_tabs += 1;
            }
        }
        if num_tabs == 0 {
            return line;
        }
        let new_storage_len = line_len + 1 + (num_tabs * 7);
        let new_line = malloc(new_storage_len) as *mut c_char;
        if new_line.is_null() {
            pr_err(c"Failure allocating memory for tab expansion\n".as_ptr());
            return ptr::null_mut();
        }
        let mut src = 0usize;
        let mut dst = 0usize;
        let mut tabs_left = num_tabs;
        let mut i = 0usize;
        while i < line_len && tabs_left != 0 {
            if *line.add(i) == '\t' as c_char {
                let len = i - src;
                memcpy(new_line.add(dst) as *mut c_void, line.add(src) as *const c_void, len);
                dst += len;
                *new_line.add(dst) = ' ' as c_char;
                dst += 1;
                while dst % 8 != 0 {
                    *new_line.add(dst) = ' ' as c_char;
                    dst += 1;
                }
                src = i + 1;
                tabs_left -= 1;
            }
            i += 1;
        }
        let len = line_len - src;
        memcpy(new_line.add(dst) as *mut c_void, line.add(src) as *const c_void, len);
        dst += len;
        *new_line.add(dst) = 0;
        free(*storage as *mut c_void);
        *storage = new_line;
        *storage_len = new_storage_len;
        new_line
    }
}

unsafe fn symbol__disassemble_bpf_image(sym: *mut symbol, args: *mut annotate_args) -> c_int {
    unsafe {
        let notes = symbol__annotation(sym);
        (*args).offset = -1;
        (*args).line = strdup(c"to be implemented".as_ptr());
        (*args).line_nr = 0;
        (*args).fileloc = ptr::null_mut();
        let dl = disasm_line__new(args);
        if !dl.is_null() {
            annotation_line__add(&mut (*dl).al, &mut (*(*notes).src).source);
        }
        zfree_char(&mut (*args).line);
        0
    }
}

unsafe fn symbol__disassemble_objdump(filename: *const c_char, sym: *mut symbol, args: *mut annotate_args) -> c_int {
    unsafe {
        let opts = &mut annotate_opts as *mut annotation_options;
        let map_p = (*(*args).ms).map;
        let dso_p = map__dso(map_p);
        if dso__binary_type(dso_p) == DSO_BINARY_TYPE__BPF_PROG_INFO {
            return symbol__disassemble_bpf_libbfd(sym, args);
        }
        if dso__binary_type(dso_p) == DSO_BINARY_TYPE__BPF_IMAGE {
            return symbol__disassemble_bpf_image(sym, args);
        }
        let mut command: *mut c_char = ptr::null_mut();
        let err0 = asprintf(&mut command, c"%s %s%s --start-address=0x%016llx --stop-address=0x%016llx %s -d %s %s %s %c%s%c %s%s -C \"$1\"".as_ptr(),
            if !(*opts).objdump_path.is_null() { (*opts).objdump_path } else { c"objdump".as_ptr() },
            if !(*opts).disassembler_style.is_null() { c"-M ".as_ptr() } else { c"".as_ptr() },
            if !(*opts).disassembler_style.is_null() { (*opts).disassembler_style } else { c"".as_ptr() },
            map__rip_2objdump(map_p, (*sym).start) as c_ulonglong,
            map__rip_2objdump(map_p, (*sym).end) as c_ulonglong,
            if (*opts).show_linenr { c"-l".as_ptr() } else { c"".as_ptr() },
            if (*opts).show_asm_raw { c"".as_ptr() } else { c"--no-show-raw-insn".as_ptr() },
            if (*opts).annotate_src { c"-S".as_ptr() } else { c"".as_ptr() },
            if !(*opts).prefix.is_null() { c"--prefix ".as_ptr() } else { c"".as_ptr() },
            if !(*opts).prefix.is_null() { '"' as c_int } else { ' ' as c_int },
            if !(*opts).prefix.is_null() { (*opts).prefix } else { c"".as_ptr() },
            if !(*opts).prefix.is_null() { '"' as c_int } else { ' ' as c_int },
            if !(*opts).prefix_strip.is_null() { c"--prefix-strip=".as_ptr() } else { c"".as_ptr() },
            if !(*opts).prefix_strip.is_null() { (*opts).prefix_strip } else { c"".as_ptr() });
        if err0 < 0 {
            pr_err(c"Failure allocating memory for the command to run\n".as_ptr());
            return err0;
        }
        pr_debug(c"Executing: %s\n".as_ptr(), command);
        let mut objdump_argv = [c"/bin/sh".as_ptr(), c"-c".as_ptr(), command as *const c_char, c"--".as_ptr(), filename, ptr::null()];
        let mut objdump_process: child_process = zeroed();
        objdump_process.argv = objdump_argv.as_mut_ptr();
        objdump_process.out = -1;
        objdump_process.err = -1;
        objdump_process.no_stderr = 1;
        if start_command(&mut objdump_process) != 0 {
            pr_err(c"Failure starting to run %s\n".as_ptr(), command);
            free(command as *mut c_void);
            return -1;
        }
        let file = fdopen(objdump_process.out, c"r".as_ptr());
        if file.is_null() {
            pr_err(c"Failure creating FILE stream for %s\n".as_ptr(), command);
            close(objdump_process.out);
            free(command as *mut c_void);
            return -1;
        }
        let mut line: *mut c_char = ptr::null_mut();
        let mut line_len: size_t = 0;
        let mut lineno = 0;
        let mut fileloc: *mut c_char = ptr::null_mut();
        let mut nline = 0;
        while feof(file) == 0 {
            if getline(&mut line, &mut line_len, file) < 0 || line.is_null() {
                break;
            }
            let matchp = strstr(line, filename);
            if !matchp.is_null() && *matchp.add(strlen(filename)) == ':' as c_char {
                continue;
            }
            let mut expanded_line = strim(line);
            expanded_line = expand_tabs(expanded_line, &mut line, &mut line_len);
            if expanded_line.is_null() {
                break;
            }
            if symbol__parse_objdump_line(sym, args, expanded_line, &mut lineno, &mut fileloc) < 0 {
                break;
            }
            nline += 1;
        }
        free(line as *mut c_void);
        free(fileloc as *mut c_void);
        let mut err = finish_command(&mut objdump_process);
        if err != 0 {
            pr_err(c"Error running %s\n".as_ptr(), command);
        }
        if nline == 0 {
            err = -1;
            pr_err(c"No output from %s\n".as_ptr(), command);
        }
        if dso__is_kcore(dso_p) {
            delete_last_nop(sym);
        }
        fclose(file);
        close(objdump_process.out);
        free(command as *mut c_void);
        err
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__disassemble(sym: *mut symbol, args: *mut annotate_args) -> c_int {
    unsafe {
        let options = (*args).options;
        let map_p = (*(*args).ms).map;
        let dso_p = map__dso(map_p);
        let mut symfs_filename = [0 as c_char; PATH_MAX];
        let mut delete_extract = false;
        let mut kce: kcore_extract = zeroed();
        let mut decomp = false;
        let mut err = dso__disassemble_filename(dso_p, symfs_filename.as_mut_ptr(), symfs_filename.len());
        if err != 0 {
            return err;
        }
        pr_debug(c"%s: filename=%s, sym=%s, start=%#llx, end=%#llx\n".as_ptr(), c"symbol__disassemble".as_ptr(), symfs_filename.as_mut_ptr(), (*sym).name, map__unmap_ip(map_p, (*sym).start) as c_ulonglong, map__unmap_ip(map_p, (*sym).end) as c_ulonglong);
        pr_debug(c"annotating [%p] %30s : [%p] %30s\n".as_ptr(), dso_p, dso__long_name(dso_p), sym, (*sym).name);
        if dso__binary_type(dso_p) == DSO_BINARY_TYPE__NOT_FOUND {
            return SYMBOL_ANNOTATE_ERRNO__COULDNT_DETERMINE_FILE_TYPE;
        } else if dso__is_kcore(dso_p) {
            kce.addr = map__rip_2objdump(map_p, (*sym).start);
            kce.kcore_filename = symfs_filename.as_mut_ptr();
            kce.len = (*sym).end - (*sym).start;
            kce.offs = (*sym).start;
            if kcore_extract__create(&mut kce) == 0 {
                delete_extract = true;
                strlcpy(symfs_filename.as_mut_ptr(), kce.extract_filename, symfs_filename.len());
            }
        } else if dso__needs_decompress(dso_p) {
            let mut tmp = [0 as c_char; KMOD_DECOMP_LEN];
            if dso__decompress_kmodule_path(dso_p, symfs_filename.as_mut_ptr(), tmp.as_mut_ptr(), tmp.len()) < 0 {
                return -1;
            }
            if tmp[0] != 0 {
                decomp = true;
                strcpy(symfs_filename.as_mut_ptr(), tmp.as_ptr());
            }
        }
        if arch__is_powerpc((*args).arch) {
            if !sort_order.is_null() && strstr(sort_order, c"sym".as_ptr()).is_null() {
                err = symbol__disassemble_raw(symfs_filename.as_mut_ptr(), sym, args);
                if err == 0 {
                    if decomp { unlink(symfs_filename.as_mut_ptr()); }
                    if delete_extract { kcore_extract__delete(&mut kce); }
                    return err;
                }
                err = symbol__disassemble_capstone_powerpc(symfs_filename.as_mut_ptr(), sym, args);
                if err == 0 {
                    if decomp { unlink(symfs_filename.as_mut_ptr()); }
                    if delete_extract { kcore_extract__delete(&mut kce); }
                    return err;
                }
            }
        }
        if (*options).annotate_src && !(*options).hide_src_code {
            err = symbol__disassemble_objdump(symfs_filename.as_ptr(), sym, args);
            if err == 0 {
                if decomp { unlink(symfs_filename.as_mut_ptr()); }
                if delete_extract { kcore_extract__delete(&mut kce); }
                return err;
            }
        }
        err = -1;
        let mut i: u8 = 0;
        while (i as usize) < (*options).disassemblers.len() && err != 0 {
            let dis = (*options).disassemblers[i as usize];
            match dis {
                PERF_DISASM_LLVM => {
                    (*(*args).options).disassembler_used = PERF_DISASM_LLVM;
                    err = symbol__disassemble_llvm(symfs_filename.as_mut_ptr(), sym, args);
                }
                PERF_DISASM_CAPSTONE => {
                    (*(*args).options).disassembler_used = PERF_DISASM_CAPSTONE;
                    err = symbol__disassemble_capstone(symfs_filename.as_mut_ptr(), sym, args);
                }
                PERF_DISASM_OBJDUMP => {
                    (*(*args).options).disassembler_used = PERF_DISASM_OBJDUMP;
                    err = symbol__disassemble_objdump(symfs_filename.as_ptr(), sym, args);
                }
                PERF_DISASM_UNKNOWN | _ => {
                    (*(*args).options).disassembler_used = PERF_DISASM_UNKNOWN;
                    break;
                }
            }
            if err == 0 {
                pr_debug(c"Disassembled with %s\n".as_ptr(), perf_disassembler__strs[dis as usize]);
            }
            i = i.wrapping_add(1);
        }
        if decomp {
            unlink(symfs_filename.as_mut_ptr());
        }
        if delete_extract {
            kcore_extract__delete(&mut kce);
        }
        err
    }
}
