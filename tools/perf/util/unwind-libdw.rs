// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of perf/util/unwind-libdw.c.
// C includes intentionally become external declarations or dependency notes.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type uint16_t = u16;
type uint32_t = u32;
type uint64_t = u64;
type pid_t = c_int;
type ssize_t = isize;
type Dwarf_Addr = u64;
type Dwarf_Word = c_ulong;
type GElf_Word = c_uint;
type unwind_entry_cb_t = Option<unsafe extern "C" fn(*mut unwind_entry, *mut c_void) -> c_int>;

const PERF_RECORD_MISC_USER: c_uint = 2;
const PATH_MAX: usize = 4096;
const DWARF_CB_OK: c_int = 0;
const DWARF_CB_ABORT: c_int = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ORDER_CALLER: c_int = 1;

#[repr(C)]
pub struct Dwfl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwfl_Module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwfl_Thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwfl_Frame {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwarf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
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
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
}

#[repr(C)]
pub struct stack_dump {
    pub data: *mut u8,
    pub size: u64,
}

#[repr(C)]
pub struct regs_dump {
    pub mask: u64,
    pub regs: *mut u64,
}

#[repr(C)]
pub struct perf_sample {
    pub user_regs: *mut regs_dump,
    pub user_stack: stack_dump,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
    pub thread: *mut thread,
}

#[repr(C)]
pub struct unwind_entry {
    pub ip: u64,
    pub ms: map_symbol,
}

#[repr(C)]
pub struct addr_location {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct unwind_info {
    pub sample: *mut perf_sample,
    pub thread: *mut thread,
    pub machine: *mut machine,
    pub cb: unwind_entry_cb_t,
    pub arg: *mut c_void,
    pub max_stack: c_int,
    pub idx: c_int,
    pub dwfl: *mut Dwfl,
    pub e_machine: uint16_t,
    pub e_flags: uint32_t,
    pub best_effort: bool,
    pub entries: [unwind_entry; 0],
}

#[repr(C)]
pub struct dwfl_ui_thread_info {
    /* Back link to the dwfl. */
    pub dwfl: *mut Dwfl,
    /* The current unwind info, only 1 is supported. */
    pub ui: *mut unwind_info,
}

#[repr(C)]
pub struct Dwfl_Callbacks {
    pub find_debuginfo: Option<
        unsafe extern "C" fn(
            *mut Dwfl_Module,
            *mut *mut c_void,
            *const c_char,
            Dwarf_Addr,
            *const c_char,
            *const c_char,
            GElf_Word,
            *mut *mut c_char,
        ) -> c_int,
    >,
    pub debuginfo_path: *mut *mut c_char,
    pub section_address:
        Option<unsafe extern "C" fn(*mut Dwfl_Module, *mut c_void, *const c_char, Dwarf_Addr) -> c_int>,
}

#[repr(C)]
pub struct Dwfl_Thread_Callbacks {
    pub next_thread: Option<unsafe extern "C" fn(*mut Dwfl, *mut c_void, *mut *mut c_void) -> pid_t>,
    pub get_thread:
        Option<unsafe extern "C" fn(*mut Dwfl, pid_t, *mut c_void, *mut *mut c_void) -> bool>,
    pub memory_read:
        Option<unsafe extern "C" fn(*mut Dwfl, Dwarf_Addr, *mut Dwarf_Word, *mut c_void) -> bool>,
    pub set_initial_registers: Option<unsafe extern "C" fn(*mut Dwfl_Thread, *mut c_void) -> bool>,
}

#[repr(C)]
pub struct callchain_param_t {
    pub order: c_int,
}

static mut debuginfo_path: *mut c_char = ptr::null_mut();

extern "C" {
    static mut callchain_param: callchain_param_t;

    fn assert_fail(assertion: *const c_char);
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;

    fn dso__symsrc_filename(dso: *const dso) -> *const c_char;
    fn dso__long_name(dso: *const dso) -> *const c_char;
    fn dso__short_name(dso: *const dso) -> *const c_char;
    fn dso__build_id_filename(
        dso: *const dso,
        filename: *mut c_char,
        size: usize,
        is_debug: bool,
    ) -> bool;
    fn dso__data_read_addr(
        dso: *mut dso,
        map: *mut map,
        machine: *mut machine,
        addr: u64,
        data: *mut u8,
        size: usize,
    ) -> ssize_t;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__start(map: *mut map) -> u64;
    fn map__pgoff(map: *mut map) -> u64;
    fn map__map_ip(map: *mut map, ip: u64) -> u64;
    fn map__get(map: *mut map) -> *mut map;
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn maps__libdw_addr_space_dwfl(maps: *mut maps) -> *mut dwfl_ui_thread_info;
    fn maps__set_libdw_addr_space_dwfl(maps: *mut maps, dwfl: *mut dwfl_ui_thread_info);
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn thread__e_machine(thread: *mut thread, machine: *mut machine, e_flags: *mut uint32_t) -> uint16_t;
    fn thread__find_symbol(
        thread: *mut thread,
        cpumode: c_uint,
        addr: u64,
        al: *mut addr_location,
    );
    fn thread__find_map(
        thread: *mut thread,
        cpumode: c_uint,
        addr: u64,
        al: *mut addr_location,
    ) -> bool;
    fn thread__get(thread: *mut thread) -> *mut thread;
    fn thread__tid(thread: *mut thread) -> pid_t;
    fn thread__pid(thread: *mut thread) -> pid_t;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn map_symbol__exit(ms: *mut map_symbol);
    fn __symbol__join_symfs(buf: *mut c_char, size: usize, name: *const c_char);
    fn is_regular_file(filename: *const c_char) -> bool;
    fn perf_arch_reg_sp(e_machine: uint16_t) -> uint64_t;
    fn perf_arch_reg_ip(e_machine: uint16_t) -> uint64_t;
    fn perf_reg_value(val: *mut Dwarf_Word, regs: *mut regs_dump, id: uint64_t) -> c_int;
    fn perf_sample__user_regs(sample: *mut perf_sample) -> *mut regs_dump;
    fn get_dwarf_regnum_for_perf_regnum(
        perf_reg: c_int,
        e_machine: uint16_t,
        e_flags: c_int,
        only_libdw_supported: bool,
    ) -> c_int;
    fn zalloc(size: usize) -> *mut c_void;

    fn dwfl_begin(callbacks: *const Dwfl_Callbacks) -> *mut Dwfl;
    fn dwfl_end(dwfl: *mut Dwfl);
    fn dwfl_offline_section_address(
        mod_: *mut Dwfl_Module,
        userdata: *mut c_void,
        name: *const c_char,
        addr: Dwarf_Addr,
    ) -> c_int;
    fn dwfl_addrmodule(dwfl: *mut Dwfl, addr: Dwarf_Addr) -> *mut Dwfl_Module;
    fn dwfl_module_info(
        mod_: *mut Dwfl_Module,
        userdata: *mut *mut c_void,
        start: *mut Dwarf_Addr,
        end: *mut Dwarf_Addr,
        dwarf: *mut *mut Dwarf,
        bias: *mut Dwarf_Addr,
        mainfile: *mut *const c_char,
        debugfile: *mut *const c_char,
    ) -> *const c_char;
    fn dwfl_report_elf(
        dwfl: *mut Dwfl,
        name: *const c_char,
        file_name: *const c_char,
        fd: c_int,
        base: Dwarf_Addr,
        add_p_vaddr: bool,
    ) -> *mut Dwfl_Module;
    fn dwfl_attach_state(
        dwfl: *mut Dwfl,
        elf: *mut c_void,
        pid: pid_t,
        callbacks: *const Dwfl_Thread_Callbacks,
        arg: *mut c_void,
    ) -> c_int;
    fn dwfl_getthread_frames(
        dwfl: *mut Dwfl,
        tid: pid_t,
        callback: Option<unsafe extern "C" fn(*mut Dwfl_Frame, *mut c_void) -> c_int>,
        arg: *mut c_void,
    ) -> c_int;
    fn dwfl_frame_pc(frame: *mut Dwfl_Frame, pc: *mut Dwarf_Addr, isactivation: *mut bool) -> bool;
    fn dwfl_thread_state_register_pc(thread: *mut Dwfl_Thread, pc: Dwarf_Word);
    fn dwfl_thread_state_registers(
        thread: *mut Dwfl_Thread,
        firstreg: c_int,
        nregs: c_uint,
        regs: *mut Dwarf_Word,
    ) -> bool;
    fn dwfl_errmsg(err: c_int) -> *const c_char;

    fn pr_debug(format: *const c_char, ...);
    fn pr_err(format: *const c_char, ...);
}

unsafe fn c_assert(cond: bool, msg: *const c_char) {
    if !cond {
        assert_fail(msg);
    }
}

unsafe fn ui_entry(ui: *mut unwind_info, idx: c_int) -> *mut unwind_entry {
    (*ui).entries.as_mut_ptr().add(idx as usize)
}

unsafe extern "C" fn __find_debuginfo(
    _mod: *mut Dwfl_Module,
    userdata: *mut *mut c_void,
    _modname: *const c_char,
    _base: Dwarf_Addr,
    file_name: *const c_char,
    _debuglink_file: *const c_char,
    _debuglink_crc: GElf_Word,
    debuginfo_file_name: *mut *mut c_char,
) -> c_int {
    let dso = *userdata as *const dso;

    c_assert(!dso.is_null(), b"dso\0".as_ptr() as *const c_char);
    if !dso__symsrc_filename(dso).is_null()
        && strcmp(file_name, dso__symsrc_filename(dso)) != 0
    {
        *debuginfo_file_name = strdup(dso__symsrc_filename(dso));
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn libdw__invalidate_dwfl(maps: *mut maps, arg: *mut c_void) {
    let dwfl_ui_ti = arg as *mut dwfl_ui_thread_info;

    if dwfl_ui_ti.is_null() {
        return;
    }

    c_assert((*dwfl_ui_ti).ui.is_null(), b"dwfl_ui_ti->ui == NULL\0".as_ptr() as *const c_char);
    maps__set_libdw_addr_space_dwfl(maps, ptr::null_mut());
    dwfl_end((*dwfl_ui_ti).dwfl);
    free(dwfl_ui_ti as *mut c_void);
}

static offline_callbacks: Dwfl_Callbacks = Dwfl_Callbacks {
    find_debuginfo: Some(__find_debuginfo),
    debuginfo_path: unsafe { &mut debuginfo_path as *mut *mut c_char },
    section_address: Some(dwfl_offline_section_address),
    // .find_elf is not set as we use dwfl_report_elf() instead.
};

unsafe fn __report_module(al: *mut addr_location, ip: u64, ui: *mut unwind_info) -> c_int {
    let mut mod_: *mut Dwfl_Module;
    let mut dso: *mut dso = ptr::null_mut();
    let base: Dwarf_Addr;
    /*
     * Some callers will use al->sym, so we can't just use the
     * cheaper thread__find_map() here.
     */
    thread__find_symbol((*ui).thread, PERF_RECORD_MISC_USER, ip, al);

    if !(*al).map.is_null() {
        dso = map__dso((*al).map);
    }

    if dso.is_null() {
        return 0;
    }

    /*
     * The generated JIT DSO files only map the code segment without
     * ELF headers.  Since JIT codes used to be packed in a memory
     * segment, calculating the base address using pgoff falls into
     * a different code in another DSO.  So just use the map->start
     * directly to pick the correct one.
     */
    if strncmp(dso__long_name(dso), b"/tmp/jitted-\0".as_ptr() as *const c_char, 12) == 0 {
        base = map__start((*al).map);
    } else {
        base = map__start((*al).map).wrapping_sub(map__pgoff((*al).map));
    }

    mod_ = dwfl_addrmodule((*ui).dwfl, ip);
    if !mod_.is_null() {
        let mut s: Dwarf_Addr = 0;

        dwfl_module_info(
            mod_,
            ptr::null_mut(),
            &mut s,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        if s != base {
            mod_ = ptr::null_mut();
        }
    }

    if mod_.is_null() {
        let mut filename = [0 as c_char; PATH_MAX];

        __symbol__join_symfs(filename.as_mut_ptr(), filename.len(), dso__long_name(dso));
        /* Don't hang up on device files like /dev/dri/renderD128. */
        if is_regular_file(filename.as_ptr()) {
            mod_ = dwfl_report_elf(
                (*ui).dwfl,
                dso__short_name(dso),
                filename.as_ptr(),
                -1,
                base,
                false,
            );
        }
    }
    if mod_.is_null() {
        let mut filename = [0 as c_char; PATH_MAX];

        if dso__build_id_filename(dso, filename.as_mut_ptr(), filename.len(), false) {
            mod_ = dwfl_report_elf(
                (*ui).dwfl,
                dso__short_name(dso),
                filename.as_ptr(),
                -1,
                base,
                false,
            );
        }
    }

    if !mod_.is_null() {
        let mut userdatap: *mut c_void = ptr::null_mut();

        dwfl_module_info(
            mod_,
            &mut userdatap,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        *(userdatap as *mut *mut c_void) = dso as *mut c_void;
    }

    if !mod_.is_null() && dwfl_addrmodule((*ui).dwfl, ip) == mod_ {
        0
    } else {
        -1
    }
}

unsafe fn report_module(ip: u64, ui: *mut unwind_info) -> c_int {
    let mut al: addr_location = zeroed();
    let res: c_int;

    addr_location__init(&mut al);
    res = __report_module(&mut al, ip, ui);
    addr_location__exit(&mut al);
    res
}

/*
 * Store all entries within entries array,
 * we will process it after we finish unwind.
 */
unsafe fn entry(ip: u64, ui: *mut unwind_info) -> c_int {
    let e = ui_entry(ui, (*ui).idx);
    (*ui).idx += 1;
    let mut al: addr_location = zeroed();

    addr_location__init(&mut al);
    if __report_module(&mut al, ip, ui) != 0 {
        addr_location__exit(&mut al);
        return -1;
    }

    (*e).ip = ip;
    (*e).ms.thread = thread__get(al.thread);
    (*e).ms.map = map__get(al.map);
    (*e).ms.sym = al.sym;

    pr_debug(
        b"unwind: %s:ip = 0x%lx (0x%lx)\n\0".as_ptr() as *const c_char,
        if !al.sym.is_null() {
            (*al.sym).name
        } else {
            b"''\0".as_ptr() as *const c_char
        },
        ip as c_ulong,
        if !al.map.is_null() {
            map__map_ip(al.map, ip)
        } else {
            0
        } as c_ulong,
    );
    addr_location__exit(&mut al);
    0
}

unsafe extern "C" fn next_thread(
    _dwfl: *mut Dwfl,
    arg: *mut c_void,
    thread_argp: *mut *mut c_void,
) -> pid_t {
    let dwfl_ui_ti = arg as *mut dwfl_ui_thread_info;

    /* We want only single thread to be processed. */
    if !(*thread_argp).is_null() {
        return 0;
    }

    c_assert(!(*dwfl_ui_ti).ui.is_null(), b"dwfl_ui_ti->ui != NULL\0".as_ptr() as *const c_char);
    *thread_argp = arg;
    thread__tid((*(*dwfl_ui_ti).ui).thread)
}

unsafe extern "C" fn get_thread(
    _dwfl: *mut Dwfl,
    tid: pid_t,
    arg: *mut c_void,
    thread_argp: *mut *mut c_void,
) -> bool {
    let dwfl_ui_ti = arg as *mut dwfl_ui_thread_info;

    c_assert(!(*dwfl_ui_ti).ui.is_null(), b"dwfl_ui_ti->ui != NULL\0".as_ptr() as *const c_char);
    if tid != thread__tid((*(*dwfl_ui_ti).ui).thread) {
        return false;
    }

    *thread_argp = arg;
    true
}

unsafe fn access_dso_mem(ui: *mut unwind_info, addr: Dwarf_Addr, data: *mut Dwarf_Word) -> c_int {
    let mut al: addr_location = zeroed();
    let size: ssize_t;
    let dso: *mut dso;

    addr_location__init(&mut al);
    if !thread__find_map((*ui).thread, PERF_RECORD_MISC_USER, addr, &mut al) {
        pr_debug(b"unwind: no map for %lx\n\0".as_ptr() as *const c_char, addr as c_ulong);
        goto_out_fail(&mut al);
        return -1;
    }
    dso = map__dso(al.map);
    if dso.is_null() {
        goto_out_fail(&mut al);
        return -1;
    }

    size = dso__data_read_addr(
        dso,
        al.map,
        (*ui).machine,
        addr,
        data as *mut u8,
        size_of::<Dwarf_Word>(),
    );

    addr_location__exit(&mut al);
    (size != size_of::<Dwarf_Word>() as ssize_t) as c_int
}

unsafe fn goto_out_fail(al: *mut addr_location) {
    addr_location__exit(al);
}

unsafe extern "C" fn memory_read(
    _dwfl: *mut Dwfl,
    addr: Dwarf_Addr,
    result: *mut Dwarf_Word,
    arg: *mut c_void,
) -> bool {
    let dwfl_ui_ti = arg as *mut dwfl_ui_thread_info;
    let ui = (*dwfl_ui_ti).ui;
    let stack = &mut (*(*ui).sample).user_stack as *mut stack_dump;
    let mut start: u64 = 0;
    let end: u64;
    let offset: c_int;
    let mut ret: c_int;

    if (*(*ui).sample).user_regs.is_null() {
        return false;
    }

    ret = perf_reg_value(
        &mut start as *mut u64 as *mut Dwarf_Word,
        (*(*ui).sample).user_regs,
        perf_arch_reg_sp((*ui).e_machine),
    );
    if ret != 0 {
        return false;
    }

    end = start.wrapping_add((*stack).size);

    /* Check overflow. */
    if addr.wrapping_add(size_of::<Dwarf_Word>() as u64) < addr {
        return false;
    }

    if addr < start || addr.wrapping_add(size_of::<Dwarf_Word>() as u64) > end {
        ret = access_dso_mem(ui, addr, result);
        if ret != 0 {
            pr_debug(
                b"unwind: access_mem 0x%lx not inside range 0x%lx-0x%lx\n\0".as_ptr()
                    as *const c_char,
                addr as c_ulong,
                start as c_ulong,
                end as c_ulong,
            );
            return false;
        }
        return true;
    }

    offset = addr.wrapping_sub(start) as c_int;
    *result = *((*stack).data.add(offset as usize) as *mut Dwarf_Word);
    pr_debug(
        b"unwind: access_mem addr 0x%lx, val %lx, offset %d\n\0".as_ptr() as *const c_char,
        addr as c_ulong,
        *result as c_ulong,
        offset,
    );
    true
}

unsafe extern "C" fn libdw_set_initial_registers(thread: *mut Dwfl_Thread, arg: *mut c_void) -> bool {
    let dwfl_ui_ti = arg as *mut dwfl_ui_thread_info;
    let ui = (*dwfl_ui_ti).ui;
    let user_regs = perf_sample__user_regs((*ui).sample);
    let dwarf_regs: *mut Dwarf_Word;
    let mut max_dwarf_reg: c_int = 0;
    let ret: bool;
    let e_machine: uint16_t = (*ui).e_machine;
    let e_flags: c_int = (*ui).e_flags as c_int;
    let ip_perf_reg: uint64_t = perf_arch_reg_ip(e_machine);
    let mut val: Dwarf_Word = 0;

    /*
     * For every possible perf register in the bitmap determine the dwarf
     * register and use to compute the max.
     */
    for perf_reg in 0..64 {
        if (*user_regs).mask & (1_u64 << perf_reg) != 0 {
            let dwarf_reg = get_dwarf_regnum_for_perf_regnum(
                perf_reg,
                e_machine,
                e_flags,
                true, /*only_libdw_supported=*/
            );
            if dwarf_reg > max_dwarf_reg {
                max_dwarf_reg = dwarf_reg;
            }
        }
    }

    dwarf_regs = calloc((max_dwarf_reg + 1) as usize, size_of::<Dwarf_Word>()) as *mut Dwarf_Word;
    if dwarf_regs.is_null() {
        return false;
    }

    for perf_reg in 0..64 {
        if (*user_regs).mask & (1_u64 << perf_reg) != 0 {
            let dwarf_reg = get_dwarf_regnum_for_perf_regnum(
                perf_reg,
                e_machine,
                e_flags,
                true, /*only_libdw_supported=*/
            );
            if dwarf_reg >= 0 {
                val = 0;
                if perf_reg_value(&mut val, user_regs, perf_reg as uint64_t) == 0 {
                    *dwarf_regs.add(dwarf_reg as usize) = val;
                }
            }
        }
    }
    if perf_reg_value(&mut val, user_regs, ip_perf_reg) == 0 {
        dwfl_thread_state_register_pc(thread, val);
    }

    ret = dwfl_thread_state_registers(thread, 0, (max_dwarf_reg + 1) as c_uint, dwarf_regs);
    free(dwarf_regs as *mut c_void);
    ret
}

static callbacks: Dwfl_Thread_Callbacks = Dwfl_Thread_Callbacks {
    next_thread: Some(next_thread),
    get_thread: Some(get_thread),
    memory_read: Some(memory_read),
    set_initial_registers: Some(libdw_set_initial_registers),
};

unsafe extern "C" fn frame_callback(state: *mut Dwfl_Frame, arg: *mut c_void) -> c_int {
    let ui = arg as *mut unwind_info;
    let mut pc: Dwarf_Addr = 0;
    let mut isactivation: bool = false;

    if !dwfl_frame_pc(state, &mut pc, ptr::null_mut()) {
        if !(*ui).best_effort {
            pr_err(b"%s\0".as_ptr() as *const c_char, dwfl_errmsg(-1));
        }
        return DWARF_CB_ABORT;
    }

    // report the module before we query for isactivation
    report_module(pc, ui);

    if !dwfl_frame_pc(state, &mut pc, &mut isactivation) {
        if !(*ui).best_effort {
            pr_err(b"%s\0".as_ptr() as *const c_char, dwfl_errmsg(-1));
        }
        return DWARF_CB_ABORT;
    }

    if !isactivation {
        pc = pc.wrapping_sub(1);
    }

    (*ui).max_stack -= 1;
    if entry(pc, ui) != 0 || !((*ui).max_stack != 0) {
        DWARF_CB_ABORT
    } else {
        DWARF_CB_OK
    }
}

#[no_mangle]
pub unsafe extern "C" fn libdw__get_entries(
    cb: unwind_entry_cb_t,
    arg: *mut c_void,
    thread: *mut thread,
    data: *mut perf_sample,
    max_stack: c_int,
    best_effort: bool,
) -> c_int {
    let maps = thread__maps(thread);
    let machine = maps__machine(maps);
    let mut e_flags: uint32_t = 0;
    let e_machine: uint16_t = thread__e_machine(thread, machine, &mut e_flags);
    let mut dwfl_ui_ti: *mut dwfl_ui_thread_info;
    static mut ui: *mut unwind_info = ptr::null_mut();
    let dwfl: *mut Dwfl;
    let mut ip: Dwarf_Word = 0;
    let mut err: c_int = -EINVAL;
    let mut i: c_int;
    let entries: c_int;

    if (*data).user_regs.is_null() || (*(*data).user_regs).regs.is_null() {
        return 0;
    }

    ui = zalloc(size_of::<unwind_info>() + size_of::<unwind_entry>() * max_stack as usize)
        as *mut unwind_info;
    if ui.is_null() {
        return -ENOMEM;
    }

    *ui = unwind_info {
        sample: data,
        thread,
        machine,
        cb,
        arg,
        max_stack,
        idx: 0,
        dwfl: ptr::null_mut(),
        e_machine,
        e_flags,
        best_effort,
        entries: [],
    };

    dwfl_ui_ti = maps__libdw_addr_space_dwfl(maps);
    if !dwfl_ui_ti.is_null() {
        dwfl = (*dwfl_ui_ti).dwfl;
    } else {
        dwfl_ui_ti = zalloc(size_of::<dwfl_ui_thread_info>()) as *mut dwfl_ui_thread_info;
        dwfl = dwfl_begin(&offline_callbacks);
        if dwfl.is_null() {
            goto_out(ui, dwfl_ui_ti, &mut err, &mut i, &mut ip, 0);
            return finish_unwind(ui, dwfl_ui_ti, err);
        }

        (*dwfl_ui_ti).dwfl = dwfl;
        maps__set_libdw_addr_space_dwfl(maps, dwfl_ui_ti);
    }
    c_assert((*dwfl_ui_ti).ui.is_null(), b"dwfl_ui_ti->ui == NULL\0".as_ptr() as *const c_char);
    c_assert((*dwfl_ui_ti).dwfl == dwfl, b"dwfl_ui_ti->dwfl == dwfl\0".as_ptr() as *const c_char);
    c_assert(
        dwfl_ui_ti == maps__libdw_addr_space_dwfl(maps),
        b"dwfl_ui_ti == maps__libdw_addr_space_dwfl(maps)\0".as_ptr() as *const c_char,
    );
    (*dwfl_ui_ti).ui = ui;
    (*ui).dwfl = dwfl;

    err = perf_reg_value(&mut ip, (*data).user_regs, perf_arch_reg_ip(e_machine));
    if err != 0 {
        return out(ui, dwfl_ui_ti, err);
    }

    err = report_module(ip as u64, ui);
    if err != 0 {
        return out(ui, dwfl_ui_ti, err);
    }

    dwfl_attach_state(
        dwfl,
        ptr::null_mut(), /*elf=*/
        thread__pid(thread),
        &callbacks,
        dwfl_ui_ti as *mut c_void, /* Dwfl thread function argument*/
    );
    // Ignore thread already attached error.

    err = dwfl_getthread_frames(
        dwfl,
        thread__tid(thread),
        Some(frame_callback),
        ui as *mut c_void, /* Dwfl frame function argument*/
    );

    if err != 0 && (*ui).max_stack != max_stack {
        err = 0;
    }

    /*
     * Display what we got based on the order setup.
     */
    i = 0;
    while i < (*ui).idx && err == 0 {
        let mut j = i;

        if callchain_param.order == ORDER_CALLER {
            j = (*ui).idx - i - 1;
        }

        let e = ui_entry(ui, j);
        err = if (*e).ip != 0 {
            match (*ui).cb {
                Some(func) => func(e, (*ui).arg),
                None => 0,
            }
        } else {
            0
        };
        i += 1;
    }

    out(ui, dwfl_ui_ti, err)
}

unsafe fn out(ui: *mut unwind_info, dwfl_ui_ti: *mut dwfl_ui_thread_info, err: c_int) -> c_int {
    let entries: c_int;
    let mut i: c_int;

    if err != 0 {
        pr_debug(
            b"unwind: failed with '%s'\n\0".as_ptr() as *const c_char,
            dwfl_errmsg(-1),
        );
    }

    i = 0;
    while i < (*ui).idx {
        map_symbol__exit(&mut (*ui_entry(ui, i)).ms);
        i += 1;
    }

    (*dwfl_ui_ti).ui = ptr::null_mut();
    entries = (*ui).idx as c_int;
    free(ui as *mut c_void);
    /*
     * Unwinder return contract:
     *  > 0 : unwinding succeeded (stops fallback). If we found frames but hit an error
     *        (e.g. truncated stack), report success to preserve existing frames.
     *    0 : unwinding failed without yielding frames. Ignore non-fatal errors
     *        (e.g. missing debug info, DWARF corruption) to allow fallback unwinder or
     *        kernel callchain resolution to proceed.
     *  < 0 : fatal error (e.g. -ENOMEM). Aborts unwinding entirely.
     */
    if err != 0 {
        if err == -ENOMEM {
            -ENOMEM
        } else if entries > 0 {
            1
        } else {
            0
        }
    } else {
        entries
    }
}

unsafe fn goto_out(
    _ui: *mut unwind_info,
    _dwfl_ui_ti: *mut dwfl_ui_thread_info,
    _err: &mut c_int,
    _i: &mut c_int,
    _ip: &mut Dwarf_Word,
    _entries: c_int,
) {
}

unsafe fn finish_unwind(ui: *mut unwind_info, dwfl_ui_ti: *mut dwfl_ui_thread_info, err: c_int) -> c_int {
    out(ui, dwfl_ui_ti, err)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
