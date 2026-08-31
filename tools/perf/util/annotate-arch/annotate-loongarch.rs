// SPDX-License-Identifier: GPL-2.0
/*
 * Perf annotate functions.
 *
 * Copyright (C) 2020-2023 Loongson Technology Corporation Limited
 */

use core::ffi::{c_char, c_int};

// Dependencies from the original C includes:
// <stdlib.h>, <string.h>, <linux/compiler.h>, <linux/zalloc.h>,
// "../disasm.h", "../map.h", "../maps.h", "../symbol.h", "../thread.h"

type u64 = u64;

extern "C" {
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn zalloc(size: usize) -> *mut arch;

    fn map__get(map: *mut map) -> *mut map;
    fn map__objdump_2mem(map: *mut map, ip: u64) -> u64;
    fn map__rip_2objdump(map: *mut map, ip: u64) -> u64;
    fn map__map_ip(map: *mut map, ip: u64) -> u64;
    fn map__unmap_ip(map: *mut map, ip: u64) -> u64;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn maps__find_ams(maps: *mut maps, target: *mut addr_map_symbol) -> c_int;
    fn addr_map_symbol__exit(target: *mut addr_map_symbol);
    fn arch__associate_ins_ops(arch: *mut arch, name: *const c_char, ops: *const ins_ops);

    fn call__scnprintf();
    fn jump__delete();
    fn jump__scnprintf();

    static ret_ops: ins_ops;
}

#[repr(C)]
pub struct e_machine_and_e_flags {
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
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    pub start: u64,
    pub end: u64,
}

#[repr(C)]
pub struct disasm_line {
    _private: [u8; 0],
}

#[repr(C)]
pub struct objdump {
    pub skip_functions_char: c_char,
    pub comment_char: c_char,
}

#[repr(C)]
pub struct arch {
    pub name: *const c_char,
    pub id: e_machine_and_e_flags,
    pub associate_instruction_ops:
        Option<unsafe extern "C" fn(*mut arch, *const c_char) -> *const ins_ops>,
    pub objdump: objdump,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
    pub thread: *mut thread,
}

#[repr(C)]
pub struct target {
    pub addr: u64,
    pub name: *mut c_char,
    pub sym: *mut symbol,
    pub outside: bool,
    pub offset: u64,
    pub offset_avail: bool,
}

#[repr(C)]
pub struct jump {
    pub raw_comment: *mut c_char,
    pub raw_func_start: *mut c_char,
}

#[repr(C)]
pub struct ins_operands {
    pub raw: *mut c_char,
    pub target: target,
    pub jump: jump,
}

#[repr(C)]
pub struct addr_map_symbol {
    pub ms: map_symbol,
    pub addr: u64,
}

#[repr(C)]
pub struct ins_ops {
    pub free: Option<unsafe extern "C" fn()>,
    pub parse: Option<
        unsafe extern "C" fn(
            *const arch,
            *mut ins_operands,
            *mut map_symbol,
            *mut disasm_line,
        ) -> c_int,
    >,
    pub scnprintf: Option<unsafe extern "C" fn()>,
    pub is_call: bool,
    pub is_jump: bool,
}

unsafe extern "C" fn loongarch_call__parse(
    arch: *const arch,
    ops: *mut ins_operands,
    ms: *mut map_symbol,
    _dl: *mut disasm_line,
) -> c_int {
    let mut endptr: *mut c_char = core::ptr::null_mut();
    let mut map = (*ms).map;
    let mut target: addr_map_symbol;

    let c_hash = strchr((*ops).raw, b'#' as c_int);
    if c_hash.is_null() {
        return -1;
    }
    let c = c_hash.add(1);

    (*ops).target.addr = strtoull(c, &mut endptr, 16);

    let mut name = strchr(endptr, b'<' as c_int);
    name = name.add(1);

    if (*arch).objdump.skip_functions_char != 0
        && !strchr(name, (*arch).objdump.skip_functions_char as c_int).is_null()
    {
        return -1;
    }

    let tok = strchr(name, b'>' as c_int);
    if tok.is_null() {
        return -1;
    }

    *tok = 0;
    (*ops).target.name = strdup(name);
    *tok = b'>' as c_char;

    if (*ops).target.name.is_null() {
        return -1;
    }

    target = addr_map_symbol {
        ms: map_symbol {
            map: map__get(map),
            sym: core::ptr::null_mut(),
            thread: core::ptr::null_mut(),
        },
        addr: map__objdump_2mem(map, (*ops).target.addr),
    };

    if maps__find_ams(thread__maps((*ms).thread), &mut target) == 0
        && map__rip_2objdump(target.ms.map, map__map_ip(target.ms.map, target.addr))
            == (*ops).target.addr
    {
        (*ops).target.sym = target.ms.sym;
    }

    addr_map_symbol__exit(&mut target);
    0
}

static loongarch_call_ops: ins_ops = ins_ops {
    free: None,
    parse: Some(loongarch_call__parse),
    scnprintf: Some(call__scnprintf),
    is_call: true,
    is_jump: false,
};

unsafe extern "C" fn loongarch_jump__parse(
    arch: *const arch,
    ops: *mut ins_operands,
    ms: *mut map_symbol,
    _dl: *mut disasm_line,
) -> c_int {
    let map = (*ms).map;
    let sym = (*ms).sym;
    let mut target = addr_map_symbol {
        ms: map_symbol {
            map: map__get(map),
            sym: core::ptr::null_mut(),
            thread: core::ptr::null_mut(),
        },
        addr: 0,
    };
    let mut c = strchr((*ops).raw, b'#' as c_int) as *const c_char;
    let start: u64;
    let end: u64;

    (*ops).jump.raw_comment = strchr((*ops).raw, (*arch).objdump.comment_char as c_int);
    (*ops).jump.raw_func_start = strchr((*ops).raw, b'<' as c_int);

    if !(*ops).jump.raw_func_start.is_null()
        && !c.is_null()
        && c > (*ops).jump.raw_func_start as *const c_char
    {
        c = core::ptr::null();
    }

    if !c.is_null() {
        c = c.add(1);
        (*ops).target.addr = strtoull(c, core::ptr::null_mut(), 16);
    } else {
        (*ops).target.addr = strtoull((*ops).raw, core::ptr::null_mut(), 16);
    }

    target.addr = map__objdump_2mem(map, (*ops).target.addr);
    start = map__unmap_ip(map, (*sym).start);
    end = map__unmap_ip(map, (*sym).end);

    (*ops).target.outside = target.addr < start || target.addr >= end;

    if maps__find_ams(thread__maps((*ms).thread), &mut target) == 0
        && map__rip_2objdump(target.ms.map, map__map_ip(target.ms.map, target.addr))
            == (*ops).target.addr
    {
        (*ops).target.sym = target.ms.sym;
    }

    if !(*ops).target.outside {
        (*ops).target.offset = target.addr.wrapping_sub(start);
        (*ops).target.offset_avail = true;
    } else {
        (*ops).target.offset_avail = false;
    }
    addr_map_symbol__exit(&mut target);
    0
}

static loongarch_jump_ops: ins_ops = ins_ops {
    free: Some(jump__delete),
    parse: Some(loongarch_jump__parse),
    scnprintf: Some(jump__scnprintf),
    is_call: false,
    is_jump: true,
};

unsafe extern "C" fn loongarch__associate_ins_ops(
    arch: *mut arch,
    name: *const c_char,
) -> *const ins_ops {
    let mut ops: *const ins_ops = core::ptr::null();

    if strcmp(name, b"bl\0".as_ptr() as *const c_char) == 0 {
        ops = &loongarch_call_ops;
    } else if strcmp(name, b"jirl\0".as_ptr() as *const c_char) == 0 {
        ops = &ret_ops;
    } else if strcmp(name, b"b\0".as_ptr() as *const c_char) == 0
        || strncmp(name, b"beq\0".as_ptr() as *const c_char, 3) == 0
        || strncmp(name, b"bne\0".as_ptr() as *const c_char, 3) == 0
        || strncmp(name, b"blt\0".as_ptr() as *const c_char, 3) == 0
        || strncmp(name, b"bge\0".as_ptr() as *const c_char, 3) == 0
        || strncmp(name, b"bltu\0".as_ptr() as *const c_char, 4) == 0
        || strncmp(name, b"bgeu\0".as_ptr() as *const c_char, 4) == 0
    {
        ops = &loongarch_jump_ops;
    } else {
        return core::ptr::null();
    }

    arch__associate_ins_ops(arch, name, ops);

    ops
}

#[no_mangle]
pub unsafe extern "C" fn arch__new_loongarch(
    id: *const e_machine_and_e_flags,
    _cpuid: *const c_char,
) -> *const arch {
    let arch = zalloc(core::mem::size_of::<arch>());

    if arch.is_null() {
        return core::ptr::null();
    }

    (*arch).name = b"loongarch\0".as_ptr() as *const c_char;
    (*arch).id = core::ptr::read(id);
    (*arch).associate_instruction_ops = Some(loongarch__associate_ins_ops);
    (*arch).objdump.comment_char = b'#' as c_char;
    arch
}
