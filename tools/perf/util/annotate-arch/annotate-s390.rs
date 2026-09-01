// SPDX-License-Identifier: GPL-2.0
// C dependencies originally included:
// string.h, linux/compiler.h, ../debug.h, ../disasm.h, ../map.h, ../maps.h,
// ../symbol.h, ../thread.h, ../annotate.h, ../annotate-data.h

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct arch {
    pub name: *const c_char,
    pub id: e_machine_and_e_flags,
    pub associate_instruction_ops:
        Option<unsafe extern "C" fn(*mut arch, *const c_char) -> *const ins_ops>,
    pub family: c_uint,
    pub model: c_uint,
    pub objdump: arch_objdump,
}

#[repr(C)]
pub struct arch_objdump {
    pub skip_functions_char: c_char,
    pub comment_char: c_char,
}

#[repr(C)]
pub struct e_machine_and_e_flags {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ins_operands {
    pub raw: *mut c_char,
    pub source: ins_operand,
    pub target: ins_operand,
}

#[repr(C)]
pub struct ins_operand {
    pub raw: *mut c_char,
    pub addr: u64,
    pub name: *mut c_char,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
    pub thread: *mut thread,
}

#[repr(C)]
pub struct addr_map_symbol {
    pub ms: map_symbol,
    pub addr: u64,
}

#[repr(C)]
pub struct disasm_line {
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
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ins_ops {
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
}

unsafe extern "C" {
    static jump_ops: ins_ops;
    static ret_ops: ins_ops;

    static mut errno: c_int;

    fn call__scnprintf();
    fn mov__scnprintf();

    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;

    fn zalloc(size: usize) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_char);

    fn map__get(map: *mut map) -> *mut map;
    fn map__objdump_2mem(map: *mut map, addr: u64) -> u64;
    fn map__rip_2objdump(map: *mut map, addr: u64) -> u64;
    fn map__map_ip(map: *mut map, addr: u64) -> u64;
    fn maps__find_ams(maps: *mut maps, target: *mut addr_map_symbol) -> c_int;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn addr_map_symbol__exit(target: *mut addr_map_symbol);
    fn arch__associate_ins_ops(arch: *mut arch, name: *const c_char, ops: *const ins_ops);
}

const SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_CPUID_PARSING: c_int = 1;

unsafe extern "C" fn s390_call__parse(
    arch: *const arch,
    ops: *mut ins_operands,
    ms: *mut map_symbol,
    _dl: *mut disasm_line,
) -> c_int {
    let mut endptr: *mut c_char = core::ptr::null_mut();
    let mut tok: *mut c_char;
    let mut name: *mut c_char;
    let map = (*ms).map;
    let mut target: addr_map_symbol;

    tok = strchr((*ops).raw, b',' as c_int);
    if tok.is_null() {
        return -1;
    }

    (*ops).target.addr = strtoull(tok.add(1), &mut endptr, 16);

    name = strchr(endptr, b'<' as c_int);
    if name.is_null() {
        return -1;
    }

    name = name.add(1);

    if (*arch).objdump.skip_functions_char != 0
        && !strchr(name, (*arch).objdump.skip_functions_char as c_int).is_null()
    {
        return -1;
    }

    tok = strchr(name, b'>' as c_int);
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

static s390_call_ops: ins_ops = ins_ops {
    parse: Some(s390_call__parse),
    scnprintf: Some(call__scnprintf),
    is_call: true,
};

unsafe extern "C" fn s390_mov__parse(
    _arch: *const arch,
    ops: *mut ins_operands,
    _ms: *mut map_symbol,
    _dl: *mut disasm_line,
) -> c_int {
    let mut s = strchr((*ops).raw, b',' as c_int);
    let target: *mut c_char;
    let mut endptr: *mut c_char = core::ptr::null_mut();

    if s.is_null() {
        return -1;
    }

    *s = 0;
    (*ops).source.raw = strdup((*ops).raw);
    *s = b',' as c_char;

    if (*ops).source.raw.is_null() {
        return -1;
    }

    s = s.add(1);
    target = s;
    (*ops).target.raw = strdup(target);
    if (*ops).target.raw.is_null() {
        zfree(&mut (*ops).source.raw);
        return -1;
    }

    (*ops).target.addr = strtoull(target, &mut endptr, 16);
    if endptr == target {
        zfree(&mut (*ops).target.raw);
        zfree(&mut (*ops).source.raw);
        return -1;
    }

    s = strchr(endptr, b'<' as c_int);
    if s.is_null() {
        zfree(&mut (*ops).target.raw);
        zfree(&mut (*ops).source.raw);
        return -1;
    }
    endptr = strchr(s.add(1), b'>' as c_int);
    if endptr.is_null() {
        zfree(&mut (*ops).target.raw);
        zfree(&mut (*ops).source.raw);
        return -1;
    }

    *endptr = 0;
    (*ops).target.name = strdup(s.add(1));
    *endptr = b'>' as c_char;
    if (*ops).target.name.is_null() {
        zfree(&mut (*ops).target.raw);
        zfree(&mut (*ops).source.raw);
        return -1;
    }

    0
}

static s390_mov_ops: ins_ops = ins_ops {
    parse: Some(s390_mov__parse),
    scnprintf: Some(mov__scnprintf),
    is_call: false,
};

unsafe extern "C" fn s390__associate_ins_ops(
    arch: *mut arch,
    name: *const c_char,
) -> *const ins_ops {
    let mut ops: *const ins_ops = core::ptr::null();

    /* catch all kind of jumps */
    if !strchr(name, b'j' as c_int).is_null()
        || strncmp(name, c"bct".as_ptr(), 3) == 0
        || strncmp(name, c"br".as_ptr(), 2) == 0
    {
        ops = &jump_ops;
    }
    /* override call/returns */
    if strcmp(name, c"bras".as_ptr()) == 0
        || strcmp(name, c"brasl".as_ptr()) == 0
        || strcmp(name, c"basr".as_ptr()) == 0
    {
        ops = &s390_call_ops;
    }
    if strcmp(name, c"br".as_ptr()) == 0 {
        ops = &ret_ops;
    }
    /* override load/store relative to PC */
    if strcmp(name, c"lrl".as_ptr()) == 0
        || strcmp(name, c"lgrl".as_ptr()) == 0
        || strcmp(name, c"lgfrl".as_ptr()) == 0
        || strcmp(name, c"llgfrl".as_ptr()) == 0
        || strcmp(name, c"strl".as_ptr()) == 0
        || strcmp(name, c"stgrl".as_ptr()) == 0
    {
        ops = &s390_mov_ops;
    }

    if !ops.is_null() {
        arch__associate_ins_ops(arch, name, ops);
    }
    ops
}

unsafe extern "C" fn s390__cpuid_parse(arch: *mut arch, cpuid: *const c_char) -> c_int {
    let mut family: c_uint = 0;
    let mut model: [c_char; 16] = [0; 16];
    let mut model_c: [c_char; 16] = [0; 16];
    let mut cpumf_v: [c_char; 16] = [0; 16];
    let mut cpumf_a: [c_char; 16] = [0; 16];
    let ret: c_int;

    /*
     * cpuid string format:
     * "IBM,family,model-capacity,model[,cpum_cf-version,cpum_cf-authorization]"
     */
    ret = sscanf(
        cpuid,
        c"%*[^,],%u,%[^,],%[^,],%[^,],%s".as_ptr(),
        &mut family as *mut c_uint,
        model_c.as_mut_ptr(),
        model.as_mut_ptr(),
        cpumf_v.as_mut_ptr(),
        cpumf_a.as_mut_ptr(),
    );
    if ret >= 2 {
        (*arch).family = family;
        (*arch).model = 0;
        return 0;
    }

    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__new_s390(
    id: *const e_machine_and_e_flags,
    cpuid: *const c_char,
) -> *const arch {
    let arch = zalloc(core::mem::size_of::<arch>()) as *mut arch;

    if arch.is_null() {
        return core::ptr::null();
    }

    (*arch).name = c"s390".as_ptr();
    (*arch).id = *id;
    (*arch).associate_instruction_ops = Some(s390__associate_ins_ops);
    if !cpuid.is_null() {
        if s390__cpuid_parse(arch, cpuid) != 0 {
            errno = SYMBOL_ANNOTATE_ERRNO__ARCH_INIT_CPUID_PARSING;
            return core::ptr::null();
        }
    }
    (*arch).objdump.comment_char = b'#' as c_char;
    arch
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
