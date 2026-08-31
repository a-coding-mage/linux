// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/unwind-libunwind.c.
// Original C dependencies:
// callchain.h, debug.h, dso.h, env.h, map.h, perf_regs.h, session.h,
// symbol.h, thread.h, unwind.h, libunwind-arch/libunwind-arch.h,
// dwarf-regs.h, elf.h, fcntl.h, gelf.h, inttypes.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type s32 = i32;
type s64 = i64;
type ssize_t = isize;
type size_t = usize;
type uint64_t = u64;
type uint16_t = u16;
type uintptr_t = usize;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const O_RDONLY: c_int = 0;
const PATH_MAX: usize = 4096;
const PERF_ELF_C_READ_MMAP: c_int = 0;
const PT_LOAD: u32 = 1;
const ET_EXEC: u16 = 2;
const EM_NONE: u16 = 0;
const PERF_RECORD_MISC_USER: c_uint = 0;
const DSO_BINARY_TYPE__DEBUGLINK: c_int = 0;
const ORDER_CALLEE: c_int = 0;

const DW_EH_PE_FORMAT_MASK: u8 = 0x0f; /* format of the encoded value */
const DW_EH_PE_APPL_MASK: u8 = 0x70; /* how the value is to be applied */

/* Pointer-encoding formats: */
const DW_EH_PE_omit: u8 = 0xff;
const DW_EH_PE_ptr: u8 = 0x00; /* pointer-sized unsigned value */
const DW_EH_PE_udata4: u8 = 0x03; /* unsigned 32-bit value */
const DW_EH_PE_udata8: u8 = 0x04; /* unsigned 64-bit value */
const DW_EH_PE_sdata4: u8 = 0x0b; /* signed 32-bit value */
const DW_EH_PE_sdata8: u8 = 0x0c; /* signed 64-bit value */

/* Pointer-encoding application: */
const DW_EH_PE_absptr: u8 = 0x00; /* absolute value */
const DW_EH_PE_pcrel: u8 = 0x10; /* rel. to addr. of encoded value */

/*
 * The following are not documented by LSB v1.3, yet they are used by
 * GCC, presumably they aren't documented by LSB since they aren't
 * used on Linux:
 */
const DW_EH_PE_funcrel: u8 = 0x40; /* start-of-procedure-relative */
const DW_EH_PE_aligned: u8 = 0x50; /* aligned pointer */

/* Flags intentionally not handled, since they're not needed:
 * #define DW_EH_PE_indirect      0x80
 * #define DW_EH_PE_uleb128       0x01
 * #define DW_EH_PE_udata2        0x02
 * #define DW_EH_PE_sleb128       0x09
 * #define DW_EH_PE_sdata2        0x0a
 * #define DW_EH_PE_textrel       0x20
 * #define DW_EH_PE_datarel       0x30
 */

#[repr(C)]
pub struct Elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    root_dir: *const c_char,
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    name: *const c_char,
}

#[repr(C)]
pub struct perf_sample {
    user_stack: stack_dump,
    user_regs: *mut regs_dump,
}

#[repr(C)]
pub struct regs_dump {
    regs: *mut c_void,
}

#[repr(C)]
pub struct stack_dump {
    data: *mut u8,
    size: u64,
}

#[repr(C)]
pub struct map_symbol {
    sym: *mut symbol,
    map: *mut map,
    thread: *mut thread,
}

#[repr(C)]
pub struct unwind_entry {
    ms: map_symbol,
    ip: u64,
}

#[repr(C)]
pub struct addr_location {
    map: *mut map,
    sym: *mut symbol,
    thread: *mut thread,
}

#[repr(C)]
pub struct unwind_info {
    machine: *mut machine,
    thread: *mut thread,
    sample: *mut perf_sample,
    unw_word_t_size: size_t,
    e_machine: u16,
    best_effort: bool,
    cur_ip: c_int,
    ips: *mut u64,
}

#[repr(C)]
pub struct dso_data {
    eh_frame_hdr_offset: u64,
    eh_frame_hdr_addr: u64,
    elf_base_addr: u64,
    debug_frame_offset: u64,
}

#[repr(C)]
pub struct GElf_Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

#[repr(C)]
pub struct GElf_Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
}

#[repr(C)]
pub struct GElf_Shdr {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
}

#[repr(C)]
pub struct libarch_unwind__dyn_info {
    start_ip: u64,
    end_ip: u64,
    segbase: u64,
    table_data: u64,
    table_len: u64,
}

#[repr(C)]
pub struct callchain_param_t {
    order: c_int,
}

type unwind_entry_cb_t = Option<unsafe extern "C" fn(*mut unwind_entry, *mut c_void) -> c_int>;

unsafe extern "C" {
    static dwarf_callchain_users: bool;
    static callchain_param: callchain_param_t;

    fn elf_begin(fd: c_int, cmd: c_int, ref_: *mut c_void) -> *mut Elf;
    fn elf_getphdrnum(elf: *mut Elf, phdrnum: *mut size_t) -> c_int;
    fn gelf_getphdr(elf: *mut Elf, ndx: size_t, dst: *mut GElf_Phdr) -> *mut GElf_Phdr;
    fn gelf_getehdr(elf: *mut Elf, dst: *mut GElf_Ehdr) -> *mut GElf_Ehdr;
    fn elf_end(elf: *mut Elf) -> c_int;
    fn elf_section_by_name(
        elf: *mut Elf,
        ehdr: *mut GElf_Ehdr,
        shdr: *mut GElf_Shdr,
        name: *const c_char,
        idx: *mut c_void,
    ) -> bool;
    fn getpagesize() -> c_int;

    fn dso__data(dso: *mut dso) -> *mut dso_data;
    fn dso__data_read_offset(
        dso: *mut dso,
        machine: *mut machine,
        offset: u64,
        data: *mut u8,
        size: size_t,
    ) -> ssize_t;
    fn dso__data_get_fd(dso: *mut dso, machine: *mut machine, fd: *mut c_int) -> bool;
    fn dso__data_put_fd(dso: *mut dso);
    fn dso__data_read_addr(
        dso: *mut dso,
        map: *mut map,
        machine: *mut machine,
        addr: u64,
        data: *mut u8,
        size: size_t,
    ) -> ssize_t;
    fn dso__name(dso: *mut dso) -> *const c_char;
    fn dso__symsrc_filename(dso: *mut dso) -> *const c_char;
    fn dso__free_symsrc_filename(dso: *mut dso);
    fn dso__set_symsrc_filename(dso: *mut dso, filename: *mut c_char);
    fn dso__read_binary_type_filename(
        dso: *mut dso,
        binary_type: c_int,
        root_dir: *const c_char,
        filename: *mut c_char,
        size: size_t,
    ) -> c_int;

    fn maps__for_each_map(
        maps: *mut maps,
        cb: Option<unsafe extern "C" fn(*mut map, *mut c_void) -> c_int>,
        data: *mut c_void,
    );
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn thread__find_map(thread: *mut thread, misc: c_uint, ip: u64, al: *mut addr_location);
    fn thread__find_symbol(
        thread: *mut thread,
        misc: c_uint,
        ip: u64,
        al: *mut addr_location,
    ) -> *mut symbol;
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn thread__e_machine(thread: *mut thread, machine: *mut c_void, e_flags: *mut c_void) -> u16;

    fn map__dso(map: *mut map) -> *mut dso;
    fn map__start(map: *mut map) -> u64;
    fn map__end(map: *mut map) -> u64;
    fn map__pgoff(map: *mut map) -> u64;
    fn map__get(map: *mut map) -> *mut map;
    fn map__put(map: *mut map);
    fn map__map_ip(map: *mut map, ip: u64) -> u64;

    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);

    fn perf_sample__user_regs(sample: *mut perf_sample) -> *mut c_void;
    fn perf_arch_reg_sp(e_machine: u16) -> c_int;
    fn perf_arch_reg_ip(e_machine: u16) -> c_int;
    fn perf_reg_value(val: *mut u64, regs: *mut c_void, id: c_int) -> c_int;
    fn get_perf_regnum_for_unw_regnum(e_machine: u16, regnum: c_int) -> c_int;

    fn maps__addr_space(maps: *mut maps) -> *mut c_void;
    fn maps__set_e_machine(maps: *mut maps, e_machine: u16);
    fn maps__set_addr_space(maps: *mut maps, addr_space: *mut c_void);
    fn libunwind_arch__create_addr_space(e_machine: u16) -> *mut c_void;
    fn libunwind_arch__flush_access(maps: *mut maps);
    fn libunwind_arch__finish_access(maps: *mut maps);
    fn libunwind_arch_unwind_info__new(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        best_effort: bool,
        e_machine: u16,
        first_ip: u64,
    ) -> *mut unwind_info;
    fn libunwind_arch_unwind_info__delete(ui: *mut unwind_info);
    fn libunwind_arch__unwind_step(ui: *mut unwind_info) -> c_int;
    fn libunwind_arch__dwarf_search_unwind_table(
        e_machine: u16,
        as_: *mut c_void,
        ip: u64,
        di: *mut libarch_unwind__dyn_info,
        pi: *mut c_void,
        need_unwind_info: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn libunwind_arch__dwarf_find_debug_frame(
        e_machine: u16,
        found: c_int,
        di: *mut libarch_unwind__dyn_info,
        ip: u64,
        base: u64,
        symfile: *const c_char,
        start: u64,
        end: u64,
    ) -> bool;

    fn open(path: *const c_char, flags: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_warning_once(fmt: *const c_char, ...);
}

unsafe fn dw_read<T: Copy>(ptr_: &mut *mut u8, end: *mut u8) -> Result<T, c_int> {
    let p = *ptr_ as *mut T;
    if p.add(1) as *mut T > end as *mut T {
        return Err(-EINVAL);
    }
    let v = ptr::read_unaligned(p);
    *ptr_ = p.add(1) as *mut u8;
    Ok(v)
}

unsafe fn __dw_read_encoded_value(
    p: *mut *mut u8,
    end: *mut u8,
    val: *mut u64,
    mut encoding: u8,
) -> c_int {
    let mut cur = *p;
    *val = 0;

    match encoding {
        DW_EH_PE_omit => {
            *val = 0;
            *p = cur;
            return 0;
        }
        DW_EH_PE_ptr => {
            match dw_read::<c_ulong>(&mut cur, end) {
                Ok(v) => *val = v as u64,
                Err(e) => return e,
            }
            *p = cur;
            return 0;
        }
        _ => {}
    }

    match encoding & DW_EH_PE_APPL_MASK {
        DW_EH_PE_absptr => {}
        DW_EH_PE_pcrel => {
            *val = cur as c_ulong as u64;
        }
        _ => return -EINVAL,
    }

    if (encoding & 0x07) == 0x00 {
        encoding |= DW_EH_PE_udata4;
    }

    match encoding & DW_EH_PE_FORMAT_MASK {
        DW_EH_PE_sdata4 => match dw_read::<s32>(&mut cur, end) {
            Ok(v) => *val = (*val).wrapping_add(v as u64),
            Err(e) => return e,
        },
        DW_EH_PE_udata4 => match dw_read::<u32>(&mut cur, end) {
            Ok(v) => *val = (*val).wrapping_add(v as u64),
            Err(e) => return e,
        },
        DW_EH_PE_sdata8 => match dw_read::<s64>(&mut cur, end) {
            Ok(v) => *val = (*val).wrapping_add(v as u64),
            Err(e) => return e,
        },
        DW_EH_PE_udata8 => match dw_read::<u64>(&mut cur, end) {
            Ok(v) => *val = (*val).wrapping_add(v),
            Err(e) => return e,
        },
        _ => return -EINVAL,
    }

    *p = cur;
    0
}

unsafe fn dw_read_encoded_value(ptr_: &mut *mut u8, end: *mut u8, enc: u8) -> Result<u64, c_int> {
    let mut v: u64 = 0;
    if __dw_read_encoded_value(ptr_ as *mut *mut u8, end, &mut v, enc) != 0 {
        return Err(-EINVAL);
    }
    Ok(v)
}

unsafe fn elf_base_address(fd: c_int) -> u64 {
    let elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, ptr::null_mut());
    let mut phdr: GElf_Phdr = core::mem::zeroed();
    let mut retval: u64 = 0;
    let mut phdrnum: size_t = 0;

    if elf.is_null() {
        return 0;
    }
    let _ = elf_getphdrnum(elf, &mut phdrnum);
    /* PT_LOAD segments are sorted by p_vaddr, so the first has the minimum p_vaddr. */
    for i in 0..phdrnum {
        if !gelf_getphdr(elf, i, &mut phdr).is_null() && phdr.p_type == PT_LOAD {
            retval = phdr.p_vaddr & (-(getpagesize() as c_long) as u64);
            break;
        }
    }

    elf_end(elf);
    retval
}

#[repr(C, packed)]
struct eh_frame_hdr {
    version: c_uchar,
    eh_frame_ptr_enc: c_uchar,
    fde_count_enc: c_uchar,
    table_enc: c_uchar,

    /*
     * The rest of the header is variable-length and consists of the
     * following members:
     *
     *	encoded_t eh_frame_ptr;
     *	encoded_t fde_count;
     */

    /* A single encoded pointer should not be more than 8 bytes. */
    enc: [u64; 2],

    /*
     * struct {
     *    encoded_t start_ip;
     *    encoded_t fde_addr;
     * } binary_search_table[fde_count];
     */
}

type c_uchar = u8;

unsafe fn unwind_spec_ehframe(
    dso: *mut dso,
    machine: *mut machine,
    offset: u64,
    table_data_offset: *mut u64,
    fde_count: *mut u64,
) -> c_int {
    let mut hdr: eh_frame_hdr = core::mem::zeroed();
    let mut enc = (&mut hdr.enc as *mut [u64; 2]) as *mut u8;
    let end = ((&mut hdr.enc as *mut [u64; 2]) as *mut u8).add(size_of::<[u64; 2]>());
    let r: ssize_t;

    r = dso__data_read_offset(
        dso,
        machine,
        offset,
        (&mut hdr as *mut eh_frame_hdr) as *mut u8,
        size_of::<eh_frame_hdr>(),
    );
    if r != size_of::<eh_frame_hdr>() as ssize_t {
        return -EINVAL;
    }

    /* We dont need eh_frame_ptr, just skip it. */
    if dw_read_encoded_value(&mut enc, end, hdr.eh_frame_ptr_enc).is_err() {
        return -EINVAL;
    }

    match dw_read_encoded_value(&mut enc, end, hdr.fde_count_enc) {
        Ok(v) => *fde_count = v,
        Err(_) => return -EINVAL,
    }
    *table_data_offset = enc as u64 - (&hdr as *const eh_frame_hdr as *const u8 as u64);
    0
}

#[repr(C)]
struct read_unwind_spec_eh_frame_maps_cb_args {
    dso: *mut dso,
    base_addr: u64,
}

unsafe extern "C" fn read_unwind_spec_eh_frame_maps_cb(map: *mut map, data: *mut c_void) -> c_int {
    let args = data as *mut read_unwind_spec_eh_frame_maps_cb_args;

    if map__dso(map) == (*args).dso
        && map__start(map).wrapping_sub(map__pgoff(map)) < (*args).base_addr
    {
        (*args).base_addr = map__start(map).wrapping_sub(map__pgoff(map));
    }

    0
}

unsafe fn elf_section_address_and_offset(
    fd: c_int,
    name: *const c_char,
    address: *mut u64,
    offset: *mut u64,
) -> c_int {
    let elf: *mut Elf;
    let mut ehdr: GElf_Ehdr = core::mem::zeroed();
    let mut shdr: GElf_Shdr = core::mem::zeroed();
    let mut ret: c_int = -1;

    elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, ptr::null_mut());
    if elf.is_null() {
        return -1;
    }

    if gelf_getehdr(elf, &mut ehdr).is_null() {
        elf_end(elf);
        return ret;
    }

    if !elf_section_by_name(elf, &mut ehdr, &mut shdr, name, ptr::null_mut()) {
        elf_end(elf);
        return ret;
    }

    *address = shdr.sh_addr;
    *offset = shdr.sh_offset;
    ret = 0;
    elf_end(elf);
    ret
}

unsafe fn read_unwind_spec_eh_frame(
    dso: *mut dso,
    ui: *mut unwind_info,
    table_data: *mut u64,
    segbase: *mut u64,
    fde_count: *mut u64,
) -> c_int {
    let mut args = read_unwind_spec_eh_frame_maps_cb_args {
        dso,
        base_addr: u64::MAX,
    };
    let mut ret: c_int;
    let mut fd: c_int = 0;

    if (*dso__data(dso)).eh_frame_hdr_offset == 0 {
        if !dso__data_get_fd(dso, (*ui).machine, &mut fd) {
            return -EINVAL;
        }

        /* Check the .eh_frame section for unwinding info */
        ret = elf_section_address_and_offset(
            fd,
            b".eh_frame_hdr\0".as_ptr() as *const c_char,
            &mut (*dso__data(dso)).eh_frame_hdr_addr,
            &mut (*dso__data(dso)).eh_frame_hdr_offset,
        );
        (*dso__data(dso)).elf_base_addr = elf_base_address(fd);
        dso__data_put_fd(dso);
        if ret != 0 || (*dso__data(dso)).eh_frame_hdr_offset == 0 {
            return -EINVAL;
        }
    }

    maps__for_each_map(
        thread__maps((*ui).thread),
        Some(read_unwind_spec_eh_frame_maps_cb),
        &mut args as *mut _ as *mut c_void,
    );

    args.base_addr = args.base_addr.wrapping_sub((*dso__data(dso)).elf_base_addr);
    /* Address of .eh_frame_hdr */
    *segbase = args.base_addr.wrapping_add((*dso__data(dso)).eh_frame_hdr_addr);
    ret = unwind_spec_ehframe(
        dso,
        (*ui).machine,
        (*dso__data(dso)).eh_frame_hdr_offset,
        table_data,
        fde_count,
    );
    if ret != 0 {
        return ret;
    }
    /* binary_search_table offset plus .eh_frame_hdr address */
    *table_data = (*table_data).wrapping_add(*segbase);
    0
}

unsafe fn elf_section_offset(fd: c_int, name: *const c_char) -> u64 {
    let mut address: u64 = 0;
    let mut offset: u64 = 0;

    if elf_section_address_and_offset(fd, name, &mut address, &mut offset) != 0 {
        return 0;
    }

    offset
}

unsafe fn read_unwind_spec_debug_frame(
    dso: *mut dso,
    machine: *mut machine,
    offset: *mut u64,
) -> c_int {
    let mut fd: c_int;
    let mut ofs: u64 = (*dso__data(dso)).debug_frame_offset;

    /* debug_frame can reside in:
     *  - dso
     *  - debug pointed by symsrc_filename
     *  - gnu_debuglink, which doesn't necessary
     *    has to be pointed by symsrc_filename
     */
    if ofs == 0 {
        fd = 0;
        if dso__data_get_fd(dso, machine, &mut fd) {
            ofs = elf_section_offset(fd, b".debug_frame\0".as_ptr() as *const c_char);
            dso__data_put_fd(dso);
        }

        if ofs <= 0 {
            fd = open(dso__symsrc_filename(dso), O_RDONLY);
            if fd >= 0 {
                ofs = elf_section_offset(fd, b".debug_frame\0".as_ptr() as *const c_char);
                close(fd);
            }
        }

        if ofs <= 0 {
            let debuglink = malloc(PATH_MAX) as *mut c_char;
            let mut ret: c_int = 0;

            if debuglink.is_null() {
                pr_err(b"unwind: Can't read unwind spec debug frame.\n\0".as_ptr() as *const c_char);
                return -ENOMEM;
            }

            ret = dso__read_binary_type_filename(
                dso,
                DSO_BINARY_TYPE__DEBUGLINK,
                (*machine).root_dir,
                debuglink,
                PATH_MAX,
            );
            if ret == 0 {
                fd = open(debuglink, O_RDONLY);
                if fd >= 0 {
                    ofs = elf_section_offset(fd, b".debug_frame\0".as_ptr() as *const c_char);
                    close(fd);
                }
            }
            if ofs > 0 {
                if !dso__symsrc_filename(dso).is_null() {
                    pr_warning(
                        b"%s: overwrite symsrc(%s,%s)\n\0".as_ptr() as *const c_char,
                        b"read_unwind_spec_debug_frame\0".as_ptr() as *const c_char,
                        dso__symsrc_filename(dso),
                        debuglink,
                    );
                    dso__free_symsrc_filename(dso);
                }
                dso__set_symsrc_filename(dso, debuglink);
            } else {
                free(debuglink as *mut c_void);
            }
        }

        (*dso__data(dso)).debug_frame_offset = ofs;
    }

    *offset = ofs;
    if *offset != 0 {
        return 0;
    }

    -EINVAL
}

unsafe fn find_map(ip: uint64_t, ui: *mut unwind_info) -> *mut map {
    let mut al: addr_location = core::mem::zeroed();
    let ret: *mut map;

    addr_location__init(&mut al);
    thread__find_map((*ui).thread, PERF_RECORD_MISC_USER, ip, &mut al);
    ret = map__get(al.map);
    addr_location__exit(&mut al);
    ret
}

unsafe fn elf_is_exec(fd: c_int, name: *const c_char) -> c_int {
    let elf: *mut Elf;
    let mut ehdr: GElf_Ehdr = core::mem::zeroed();
    let mut retval: c_int = 0;

    elf = elf_begin(fd, PERF_ELF_C_READ_MMAP, ptr::null_mut());
    if elf.is_null() {
        return 0;
    }
    if gelf_getehdr(elf, &mut ehdr).is_null() {
        elf_end(elf);
        return retval;
    }

    retval = (ehdr.e_type == ET_EXEC) as c_int;

    elf_end(elf);
    pr_debug3(b"unwind: elf_is_exec(%s): %d\n\0".as_ptr() as *const c_char, name, retval);
    retval
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind__find_proc_info(
    as_: *mut c_void,
    ip: uint64_t,
    pi: *mut c_void,
    need_unwind_info: c_int,
    arg: *mut c_void,
) -> c_int {
    let ui = arg as *mut unwind_info;
    let map: *mut map;
    let dso: *mut dso;
    let mut table_data: u64 = 0;
    let mut segbase: u64 = 0;
    let mut fde_count: u64 = 0;
    let mut ret: c_int = -EINVAL;

    map = find_map(ip, ui);
    if map.is_null() {
        return -EINVAL;
    }

    dso = map__dso(map);
    if dso.is_null() {
        map__put(map);
        return -EINVAL;
    }

    pr_debug3(b"unwind: find_proc_info dso %s\n\0".as_ptr() as *const c_char, dso__name(dso));

    /* Check the .eh_frame section for unwinding info */
    if read_unwind_spec_eh_frame(dso, ui, &mut table_data, &mut segbase, &mut fde_count) == 0 {
        #[repr(C)]
        struct table_entry {
            start_ip_offset: u32,
            fde_offset: u32,
        }
        let mut di = libarch_unwind__dyn_info {
            start_ip: map__start(map),
            end_ip: map__end(map),
            segbase,
            table_data,
            table_len: fde_count
                .wrapping_mul(size_of::<table_entry>() as u64)
                .wrapping_div((*ui).unw_word_t_size as u64),
        };

        ret = libunwind_arch__dwarf_search_unwind_table(
            (*ui).e_machine,
            as_,
            ip,
            &mut di,
            pi,
            need_unwind_info,
            arg,
        );
    }

    /* Check the .debug_frame section for unwinding info */
    if ret < 0 && read_unwind_spec_debug_frame(dso, (*ui).machine, &mut segbase) == 0 {
        let mut fd: c_int = 0;
        let start: u64 = map__start(map);
        let mut base: u64 = start;
        let symfile: *const c_char;
        let mut di: libarch_unwind__dyn_info = core::mem::zeroed();

        if dso__data_get_fd(dso, (*ui).machine, &mut fd) {
            if elf_is_exec(fd, dso__name(dso)) != 0 {
                base = 0;
            }
            dso__data_put_fd(dso);
        }

        symfile = if !dso__symsrc_filename(dso).is_null() {
            dso__symsrc_filename(dso)
        } else {
            dso__name(dso)
        };

        if libunwind_arch__dwarf_find_debug_frame(
            (*ui).e_machine,
            0,
            &mut di,
            ip,
            base,
            symfile,
            start,
            map__end(map),
        ) {
            ret = libunwind_arch__dwarf_search_unwind_table(
                (*ui).e_machine,
                as_,
                ip,
                &mut di,
                pi,
                need_unwind_info,
                arg,
            );
        }
    }
    map__put(map);
    ret
}

unsafe fn access_dso_mem(ui: *mut unwind_info, addr: uint64_t, data_word: *mut c_void) -> c_int {
    let map: *mut map;
    let dso: *mut dso;
    let size: ssize_t;

    map = find_map(addr, ui);
    if map.is_null() {
        pr_debug(
            b"unwind: no map for %lx\n\0".as_ptr() as *const c_char,
            addr as c_ulong,
        );
        return -1;
    }

    dso = map__dso(map);

    if dso.is_null() {
        map__put(map);
        return -1;
    }

    size = dso__data_read_addr(
        dso,
        map,
        (*ui).machine,
        addr,
        data_word as *mut u8,
        (*ui).unw_word_t_size,
    );
    map__put(map);
    !((size as size_t) == (*ui).unw_word_t_size) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind__access_mem(
    _as: *mut c_void,
    addr: uint64_t,
    valp_word: *mut c_void,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    let ui = arg as *mut unwind_info;
    let stack = &mut (*(*ui).sample).user_stack as *mut stack_dump;
    let mut start: u64 = 0;
    let end: u64;
    let offset: c_int;
    let mut ret: c_int;

    /* Don't support write, probably not needed. */
    if __write != 0
        || stack.is_null()
        || (*(*ui).sample).user_regs.is_null()
        || (*(*(*ui).sample).user_regs).regs.is_null()
    {
        let zero: uint64_t = 0;

        memcpy(valp_word, &zero as *const uint64_t as *const c_void, (*ui).unw_word_t_size);
        return 0;
    }

    ret = perf_reg_value(
        &mut start,
        perf_sample__user_regs((*ui).sample),
        perf_arch_reg_sp((*ui).e_machine),
    );
    if ret != 0 {
        return ret;
    }

    end = start.wrapping_add((*stack).size);

    /* Check overflow. */
    if addr.wrapping_add((*ui).unw_word_t_size as u64) < addr {
        return -EINVAL;
    }

    if addr < start || addr.wrapping_add((*ui).unw_word_t_size as u64) >= end {
        ret = access_dso_mem(ui, addr, valp_word);
        if ret != 0 {
            pr_debug3(
                b"unwind: access_mem %p not inside range 0x%lx-0x%lx\n\0".as_ptr()
                    as *const c_char,
                addr as uintptr_t as *mut c_void,
                start,
                end,
            );
            memset(valp_word, 0, (*ui).unw_word_t_size);
            return ret;
        }
        return 0;
    }

    offset = addr.wrapping_sub(start) as c_int;
    memcpy(
        valp_word,
        (*stack).data.add(offset as usize) as *const c_void,
        (*ui).unw_word_t_size,
    );
    pr_debug3(
        b"unwind: access_mem addr %p val %lx, offset %d\n\0".as_ptr() as *const c_char,
        addr as uintptr_t as *mut c_void,
        *(valp_word as *mut c_ulong),
        offset,
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn __libunwind__access_reg(
    _as: *mut c_void,
    regnum: c_int,
    valp_word: *mut c_void,
    __write: c_int,
    arg: *mut c_void,
) -> c_int {
    let ui = arg as *mut unwind_info;
    let id: c_int;
    let ret: c_int;
    let mut val: u64 = 0;

    /* Don't support write, I suspect we don't need it. */
    if __write != 0 {
        pr_err(b"unwind: access_reg w %d\n\0".as_ptr() as *const c_char, regnum);
        return 0;
    }

    if (*(*ui).sample).user_regs.is_null() || (*(*(*ui).sample).user_regs).regs.is_null() {
        memset(valp_word, 0, (*ui).unw_word_t_size);
        return 0;
    }

    id = get_perf_regnum_for_unw_regnum((*ui).e_machine, regnum);
    if id < 0 {
        return -EINVAL;
    }

    ret = perf_reg_value(&mut val, perf_sample__user_regs((*ui).sample), id);
    if ret != 0 {
        if !(*ui).best_effort {
            pr_err(b"unwind: can't read reg %d\n\0".as_ptr() as *const c_char, regnum);
        }
        return ret;
    }

    if (*ui).unw_word_t_size == 8 {
        *(valp_word as *mut uint64_t) = val;
    } else {
        *(valp_word as *mut u32) = val as u32;
    }
    pr_debug3(
        b"unwind: reg %d, val %lx\n\0".as_ptr() as *const c_char,
        regnum,
        val as c_ulong,
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn unwind__prepare_access(maps: *mut maps, e_machine: uint16_t) -> c_int {
    let addr_space: *mut c_void;

    if !dwarf_callchain_users {
        return 0;
    }

    if !maps__addr_space(maps).is_null() {
        pr_debug3(b"unwind: thread map already set\n\0".as_ptr() as *const c_char);
        return 0;
    }

    if e_machine == EM_NONE {
        return 0;
    }

    maps__set_e_machine(maps, e_machine);
    addr_space = libunwind_arch__create_addr_space(e_machine);

    maps__set_addr_space(maps, addr_space);
    if addr_space.is_null() {
        pr_err(b"unwind: Can't create unwind address space.\n\0".as_ptr() as *const c_char);
        return -ENOMEM;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn unwind__flush_access(maps: *mut maps) {
    libunwind_arch__flush_access(maps);
}

#[no_mangle]
pub unsafe extern "C" fn unwind__finish_access(maps: *mut maps) {
    libunwind_arch__finish_access(maps);
}

unsafe fn entry(
    ip: uint64_t,
    thread: *mut thread,
    cb: unwind_entry_cb_t,
    arg: *mut c_void,
) -> c_int {
    let mut e: unwind_entry = core::mem::zeroed();
    let mut al: addr_location = core::mem::zeroed();
    let ret: c_int;

    addr_location__init(&mut al);
    e.ms.sym = thread__find_symbol(thread, PERF_RECORD_MISC_USER, ip, &mut al);
    e.ip = ip;
    e.ms.map = al.map;
    e.ms.thread = thread__get(al.thread);

    pr_debug(
        b"unwind: %s:ip = 0x%lx (0x%lx)\n\0".as_ptr() as *const c_char,
        if !al.sym.is_null() {
            (*al.sym).name
        } else {
            b"''\0".as_ptr() as *const c_char
        },
        ip,
        if !al.map.is_null() { map__map_ip(al.map, ip) } else { 0 },
    );

    ret = cb.expect("unwind entry callback is NULL")(&mut e, arg);
    addr_location__exit(&mut al);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn libunwind__get_entries(
    cb: unwind_entry_cb_t,
    arg: *mut c_void,
    thread: *mut thread,
    sample: *mut perf_sample,
    max_stack: c_int,
    best_effort: bool,
) -> c_int {
    let ui: *mut unwind_info;
    let mut first_ip: uint64_t = 0;
    let mut ret: c_int;
    let mut entries: c_int = 0;
    let e_machine: uint16_t;

    if (*sample).user_regs.is_null() || (*(*sample).user_regs).regs.is_null() {
        return 0;
    }

    if max_stack <= 0 {
        return 0;
    }

    if thread.is_null() {
        pr_warning_once(b"WARNING: thread is NULL\0".as_ptr() as *const c_char);
        return 0;
    }

    e_machine = thread__e_machine(thread, ptr::null_mut(), ptr::null_mut());
    ret = perf_reg_value(
        &mut first_ip,
        perf_sample__user_regs(sample),
        perf_arch_reg_ip(e_machine),
    );
    if ret != 0 {
        return 0;
    }

    if max_stack == 1 {
        /* Special case for a single entry. */
        ret = entry(first_ip, thread, cb, arg);
        return if ret != 0 {
            if ret == -ENOMEM { -ENOMEM } else { 0 }
        } else {
            1
        };
    }

    ui = libunwind_arch_unwind_info__new(thread, sample, max_stack, best_effort, e_machine, first_ip);
    if ui.is_null() {
        return -ENOMEM;
    }

    loop {
        ret = libunwind_arch__unwind_step(ui);
        if ret < 0 {
            break;
        }
        if ret == 0 {
            break;
        }
    }

    /*
     * Display what we got based on the order setup.
     */
    for i in 0..(*ui).cur_ip {
        let j = if callchain_param.order == ORDER_CALLEE {
            i
        } else {
            (*ui).cur_ip - i - 1
        };

        if *(*ui).ips.add(j as usize) != 0 {
            ret = entry(*(*ui).ips.add(j as usize), thread, cb, arg);
            if ret != 0 {
                break;
            }
            entries += 1;
        }
    }
    libunwind_arch_unwind_info__delete(ui);
    /*
     * Unwinder return contract:
     *  > 0 : unwinding succeeded (stops fallback).
     *    0 : unwinding failed without yielding frames. Ignore non-fatal errors
     *        (e.g. stepping failure) to allow fallback unwinder or kernel callchains.
     *  < 0 : fatal error (e.g. -ENOMEM). Aborts unwinding entirely.
     */
    if ret == -ENOMEM {
        return -ENOMEM;
    }
    if entries > 0 || ret == 0 {
        entries
    } else {
        0
    }
}
