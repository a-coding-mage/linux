// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Use DWARF Debug information to skip unnecessary callchain entries.
 *
 * Copyright (C) 2014 Sukadev Bhattiprolu, IBM Corporation.
 * Copyright (C) 2014 Ulrich Weigand, IBM Corporation.
 */

use core::ffi::{c_char, c_int, c_ulonglong};
use core::ptr;

// Dependencies from the original C includes:
// <dwarf.h>, <elfutils/libdwfl.h>, and perf util headers.

type size_t = usize;
type bool_ = bool;
type u64 = u64;
type Dwarf_Addr = u64;

const DW_OP_regx: c_int = 0x90;
const DW_OP_bregx: c_int = 0x92;
const PERF_RECORD_MISC_USER: u32 = 2;

#[repr(C)]
pub struct Dwarf_Op {
    pub atom: c_int,
    pub number: c_ulonglong,
    pub number2: c_ulonglong,
    pub offset: i64,
}

#[repr(C)]
pub struct Dwarf_Frame {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwarf_CFI {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwfl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwfl_Module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub name: *const c_char,
}

#[repr(C)]
pub struct addr_location {
    pub map: *mut map,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct ip_callchain {
    pub nr: u64,
    pub ips: [u64; 0],
}

unsafe extern "C" {
    fn dwarf_frame_register(
        frame: *mut Dwarf_Frame,
        regno: c_int,
        ops_mem: *mut Dwarf_Op,
        ops: *mut *mut Dwarf_Op,
        nops: *mut size_t,
    ) -> c_int;
    fn dwarf_frame_cfa(
        frame: *mut Dwarf_Frame,
        ops: *mut *mut Dwarf_Op,
        nops: *mut size_t,
    ) -> c_int;
    fn dwarf_errmsg(error: c_int) -> *const c_char;
    fn dwfl_errmsg(error: c_int) -> *const c_char;
    fn dwfl_module_eh_cfi(mod_: *mut Dwfl_Module, bias: *mut Dwarf_Addr) -> *mut Dwarf_CFI;
    fn dwarf_cfi_addrframe(
        cfi: *mut Dwarf_CFI,
        address: Dwarf_Addr,
        frame: *mut *mut Dwarf_Frame,
    ) -> c_int;
    fn dwfl_module_dwarf_cfi(mod_: *mut Dwfl_Module, bias: *mut Dwarf_Addr) -> *mut Dwarf_CFI;
    fn dso__libdw_dwfl(dso: *mut dso) -> *mut Dwfl;
    fn dwfl_addrmodule(dwfl: *mut Dwfl, address: Dwarf_Addr) -> *mut Dwfl_Module;
    fn dwarf_frame_info(
        frame: *mut Dwarf_Frame,
        start: *mut Dwarf_Addr,
        end: *mut Dwarf_Addr,
        signalp: *mut bool_,
    ) -> c_int;
    fn addr_location__init(al: *mut addr_location);
    fn thread__find_symbol(
        thread: *mut thread,
        cpumode: u32,
        addr: u64,
        al: *mut addr_location,
    );
    fn map__dso(map: *mut map) -> *mut dso;
    fn addr_location__exit(al: *mut addr_location);
    fn map__map_ip(map: *mut map, ip: u64) -> u64;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn pr_debug(fmt: *const c_char, ...);
}

/*
 * Use the DWARF expression for the Call-frame-address and determine
 * if return address is in LR and if a new frame was allocated.
 */
unsafe fn check_return_reg(ra_regno: c_int, frame: *mut Dwarf_Frame) -> c_int {
    let mut ops_mem: [Dwarf_Op; 3] = core::mem::zeroed();
    let mut dummy: Dwarf_Op = core::mem::zeroed();
    let mut ops: *mut Dwarf_Op = &mut dummy;
    let mut nops: size_t = 0;
    let mut result: c_int;

    result = dwarf_frame_register(frame, ra_regno, ops_mem.as_mut_ptr(), &mut ops, &mut nops);
    if result < 0 {
        pr_debug(
            c"dwarf_frame_register() %s\n".as_ptr(),
            dwarf_errmsg(-1),
        );
        return -1;
    }

    /*
     * Check if return address is on the stack. If return address
     * is in a register (typically R0), it is yet to be saved on
     * the stack.
     */
    if (nops != 0 || !ops.is_null())
        && !(nops == 1
            && (*ops.add(0)).atom == DW_OP_regx
            && (*ops.add(0)).number2 == 0
            && (*ops.add(0)).offset == 0)
    {
        return 0;
    }

    /*
     * Return address is in LR. Check if a frame was allocated
     * but not-yet used.
     */
    result = dwarf_frame_cfa(frame, &mut ops, &mut nops);
    if result < 0 {
        pr_debug(
            c"dwarf_frame_cfa() returns %d, %s\n".as_ptr(),
            result,
            dwarf_errmsg(-1),
        );
        return -1;
    }

    /*
     * If call frame address is in r1, no new frame was allocated.
     */
    if nops == 1
        && (*ops.add(0)).atom == DW_OP_bregx
        && (*ops.add(0)).number == 1
        && (*ops.add(0)).number2 == 0
    {
        return 1;
    }

    /*
     * A new frame was allocated but has not yet been used.
     */
    2
}

/*
 * Get the DWARF frame from the .eh_frame section.
 */
unsafe fn get_eh_frame(mod_: *mut Dwfl_Module, pc: Dwarf_Addr) -> *mut Dwarf_Frame {
    let mut result: c_int;
    let mut bias: Dwarf_Addr = 0;
    let mut cfi: *mut Dwarf_CFI;
    let mut frame: *mut Dwarf_Frame = ptr::null_mut();

    cfi = dwfl_module_eh_cfi(mod_, &mut bias);
    if cfi.is_null() {
        pr_debug(
            c"%s(): no CFI - %s\n".as_ptr(),
            c"get_eh_frame".as_ptr(),
            dwfl_errmsg(-1),
        );
        return ptr::null_mut();
    }

    result = dwarf_cfi_addrframe(cfi, pc.wrapping_sub(bias), &mut frame);
    if result != 0 {
        pr_debug(
            c"%s(): %s\n".as_ptr(),
            c"get_eh_frame".as_ptr(),
            dwfl_errmsg(-1),
        );
        return ptr::null_mut();
    }

    frame
}

/*
 * Get the DWARF frame from the .debug_frame section.
 */
unsafe fn get_dwarf_frame(mod_: *mut Dwfl_Module, pc: Dwarf_Addr) -> *mut Dwarf_Frame {
    let mut cfi: *mut Dwarf_CFI;
    let mut bias: Dwarf_Addr = 0;
    let mut frame: *mut Dwarf_Frame = ptr::null_mut();
    let mut result: c_int;

    cfi = dwfl_module_dwarf_cfi(mod_, &mut bias);
    if cfi.is_null() {
        pr_debug(
            c"%s(): no CFI - %s\n".as_ptr(),
            c"get_dwarf_frame".as_ptr(),
            dwfl_errmsg(-1),
        );
        return ptr::null_mut();
    }

    result = dwarf_cfi_addrframe(cfi, pc.wrapping_sub(bias), &mut frame);
    if result != 0 {
        pr_debug(
            c"%s(): %s\n".as_ptr(),
            c"get_dwarf_frame".as_ptr(),
            dwfl_errmsg(-1),
        );
        return ptr::null_mut();
    }

    frame
}

/*
 * Return:
 *	0 if return address for the program counter @pc is on stack
 *	1 if return address is in LR and no new stack frame was allocated
 *	2 if return address is in LR and a new frame was allocated (but not
 *		yet used)
 *	-1 in case of errors
 */
unsafe fn check_return_addr(dso: *mut dso, mapped_pc: Dwarf_Addr) -> c_int {
    let mut rc: c_int = -1;
    let mut dwfl: *mut Dwfl;
    let mut mod_: *mut Dwfl_Module;
    let mut frame: *mut Dwarf_Frame;
    let mut ra_regno: c_int;
    let mut start: Dwarf_Addr = mapped_pc;
    let mut end: Dwarf_Addr = mapped_pc;
    let mut signalp: bool_ = false;

    dwfl = dso__libdw_dwfl(dso);
    if dwfl.is_null() {
        return -1;
    }

    mod_ = dwfl_addrmodule(dwfl, mapped_pc);
    if mod_.is_null() {
        pr_debug(
            c"dwfl_addrmodule() failed, %s\n".as_ptr(),
            dwarf_errmsg(-1),
        );
        return rc;
    }

    /*
     * To work with split debug info files (eg: glibc), check both
     * .eh_frame and .debug_frame sections of the ELF header.
     */
    frame = get_eh_frame(mod_, mapped_pc);
    if frame.is_null() {
        frame = get_dwarf_frame(mod_, mapped_pc);
        if frame.is_null() {
            return rc;
        }
    }

    ra_regno = dwarf_frame_info(frame, &mut start, &mut end, &mut signalp);
    if ra_regno < 0 {
        pr_debug(
            c"Return address register unavailable: %s\n".as_ptr(),
            dwarf_errmsg(-1),
        );
        return rc;
    }

    rc = check_return_reg(ra_regno, frame);

    rc
}

/*
 * The callchain saved by the kernel always includes the link register (LR).
 *
 *	0:	PERF_CONTEXT_USER
 *	1:	Program counter (Next instruction pointer)
 *	2:	LR value
 *	3:	Caller's caller
 *	4:	...
 *
 * The value in LR is only needed when it holds a return address. If the
 * return address is on the stack, we should ignore the LR value.
 *
 * Further, when the return address is in the LR, if a new frame was just
 * allocated but the LR was not saved into it, then the LR contains the
 * caller, slot 4: contains the caller's caller and the contents of slot 3:
 * (chain->ips[3]) is undefined and must be ignored.
 *
 * Use DWARF debug information to determine if any entries need to be skipped.
 *
 * Return:
 *	index:	of callchain entry that needs to be ignored (if any)
 *	-1	if no entry needs to be ignored or in case of errors
 */
#[no_mangle]
pub unsafe extern "C" fn arch_skip_callchain_idx(
    thread: *mut thread,
    chain: *mut ip_callchain,
) -> u64 {
    let mut al: addr_location = core::mem::zeroed();
    let mut dso: *mut dso = ptr::null_mut();
    let mut rc: c_int;
    let mut ip: u64;
    let mut skip_slot: u64 = -1i64 as u64;

    if chain.is_null() || (*chain).nr < 3 {
        return skip_slot;
    }

    addr_location__init(&mut al);
    ip = *(*chain).ips.as_ptr().add(1);

    thread__find_symbol(thread, PERF_RECORD_MISC_USER, ip, &mut al);

    if !al.map.is_null() {
        dso = map__dso(al.map);
    }

    if dso.is_null() {
        pr_debug(c"%llx dso is NULL\n".as_ptr(), ip);
        addr_location__exit(&mut al);
        return skip_slot;
    }

    rc = check_return_addr(dso, map__map_ip(al.map, ip));

    pr_debug(
        c"[DSO %s, sym %s, ip 0x%llx] rc %d\n".as_ptr(),
        dso__long_name(dso),
        (*al.sym).name,
        ip,
        rc,
    );

    if rc == 0 {
        /*
         * Return address on stack. Ignore LR value in callchain
         */
        skip_slot = 2;
    } else if rc == 2 {
        /*
         * New frame allocated but return address still in LR.
         * Ignore the caller's caller entry in callchain.
         */
        skip_slot = 3;
    }

    addr_location__exit(&mut al);
    skip_slot
}
