// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the corresponding kernel headers and other translation units.

extern "C" {
    static mut __start_orc_unwind_ip: *mut i32;
    static mut __stop_orc_unwind_ip: *mut i32;
    static mut __start_orc_unwind: *mut orc_entry;
    static mut __stop_orc_unwind: *mut orc_entry;
}

#[repr(C)]
pub struct orc_entry {
    pub sp_reg: i32,
    pub sp_offset: i32,
    pub fp_reg: i32,
    pub fp_offset: i32,
    pub ra_reg: i32,
    pub ra_offset: i32,
    pub type_: i32,
}

static mut orc_init: bool = false;
static mut lookup_num_blocks: u32 = 0;

static mut orc_fp_entry: orc_entry = orc_entry {
    sp_reg: ORC_REG_FP,
    sp_offset: 16,
    fp_reg: ORC_REG_PREV_SP,
    fp_offset: -16,
    ra_reg: ORC_REG_PREV_SP,
    ra_offset: -8,
    type_: ORC_TYPE_CALL,
};

static mut orc_null_entry: orc_entry = orc_entry {
    sp_reg: ORC_REG_SP,
    sp_offset: core::mem::size_of::<c_long>() as i32,
    fp_reg: ORC_REG_UNDEFINED,
    fp_offset: 0,
    ra_reg: 0,
    ra_offset: 0,
    type_: ORC_TYPE_CALL,
};

#[inline]
unsafe fn orc_ip(ip: *const i32) -> usize {
    (ip as usize).wrapping_add((*ip as isize) as usize)
}

unsafe fn __orc_find(ip_table: *mut i32, u_table: *mut orc_entry, num_entries: u32, ip: usize) -> *mut orc_entry {
    if num_entries == 0 { return core::ptr::null_mut(); }
    let mut first = ip_table;
    let mut found = first;
    let mut last = ip_table.add(num_entries as usize - 1);
    while (first as usize) <= (last as usize) {
        let mid = first.add((last.offset_from(first) / 2) as usize);
        if orc_ip(mid) <= ip {
            found = mid;
            first = mid.add(1);
        } else {
            last = mid.sub(1);
        }
    }
    u_table.add(found.offset_from(ip_table) as usize)
}

#[cfg(feature = "CONFIG_MODULES")]
unsafe fn orc_module_find(ip: usize) -> *mut orc_entry {
    let module = __module_address(ip);
    if module.is_null() || (*module).arch.orc_unwind.is_null() || (*module).arch.orc_unwind_ip.is_null() {
        return core::ptr::null_mut();
    }
    __orc_find((*module).arch.orc_unwind_ip, (*module).arch.orc_unwind, (*module).arch.num_orcs, ip)
}
#[cfg(not(feature = "CONFIG_MODULES"))]
unsafe fn orc_module_find(_: usize) -> *mut orc_entry { core::ptr::null_mut() }

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
unsafe fn orc_ftrace_find(ip: usize) -> *mut orc_entry {
    let ops = ftrace_ops_trampoline(ip);
    if ops.is_null() { return core::ptr::null_mut(); }
    let mut tramp_addr = if (*ops).flags & FTRACE_OPS_FL_SAVE_REGS != 0 { ftrace_regs_caller as usize } else { ftrace_caller as usize };
    tramp_addr = tramp_addr.wrapping_add(ip.wrapping_sub((*ops).trampoline));
    if ip == tramp_addr { return core::ptr::null_mut(); }
    orc_find(tramp_addr)
}
#[cfg(not(feature = "CONFIG_DYNAMIC_FTRACE"))]
unsafe fn orc_ftrace_find(_: usize) -> *mut orc_entry { core::ptr::null_mut() }

unsafe fn orc_find(ip: usize) -> *mut orc_entry {
    static mut orc: *mut orc_entry = core::ptr::null_mut();
    if ip == 0 { return &raw mut orc_null_entry; }
    if ip >= LOOKUP_START_IP && ip < LOOKUP_STOP_IP {
        let idx = (ip - LOOKUP_START_IP) / LOOKUP_BLOCK_SIZE;
        if idx >= lookup_num_blocks.wrapping_sub(1) { orc_warn(); return core::ptr::null_mut(); }
        let start = orc_lookup[idx];
        let stop = orc_lookup[idx + 1] + 1;
        if __start_orc_unwind.add(start as usize) >= __stop_orc_unwind || __start_orc_unwind.add(stop as usize) > __stop_orc_unwind { orc_warn(); return core::ptr::null_mut(); }
        return __orc_find(__start_orc_unwind_ip.add(start as usize), __start_orc_unwind.add(start as usize), stop - start, ip);
    }
    if is_kernel_inittext(ip) { return __orc_find(__start_orc_unwind_ip, __start_orc_unwind, __stop_orc_unwind_ip.offset_from(__start_orc_unwind_ip) as u32, ip); }
    orc = orc_module_find(ip);
    if !orc.is_null() { return orc; }
    orc_ftrace_find(ip)
}

#[cfg(feature = "CONFIG_MODULES")]
unsafe fn orc_sort_swap(a: *mut i32, b: *mut i32, _: i32) {
    let delta = b.offset_from(a);
    let tmp = *a; *a = (*b).wrapping_add(delta as i32); *b = tmp.wrapping_sub(delta as i32);
}

#[cfg(feature = "CONFIG_MODULES")]
unsafe fn orc_sort_cmp(a: *const i32, b: *const i32) -> i32 {
    let av = orc_ip(a); let bv = orc_ip(b);
    if av > bv { return 1; } if av < bv { return -1; }
    if (*cur_orc_table.add(a.offset_from(cur_orc_ip_table) as usize)).type_ == ORC_TYPE_UNDEFINED { -1 } else { 1 }
}

#[cfg(feature = "CONFIG_MODULES")]
static mut cur_orc_ip_table: *mut i32 = core::ptr::null_mut();
#[cfg(feature = "CONFIG_MODULES")]
static mut cur_orc_table: *mut orc_entry = core::ptr::null_mut();

#[cfg(feature = "CONFIG_MODULES")]
pub unsafe fn unwind_module_init(mod_: *mut module, orc_ip: *mut i32, orc_ip_size: usize, orc: *mut orc_entry, orc_size: usize) {
    let num_entries = orc_ip_size / core::mem::size_of::<i32>();
    cur_orc_ip_table = orc_ip; cur_orc_table = orc;
    sort(orc_ip, num_entries, core::mem::size_of::<i32>(), orc_sort_cmp, orc_sort_swap);
    (*mod_).arch.orc_unwind_ip = orc_ip; (*mod_).arch.orc_unwind = orc; (*mod_).arch.num_orcs = num_entries as u32;
}

pub unsafe fn unwind_init() {
    let orc_size = (__stop_orc_unwind as usize).wrapping_sub(__start_orc_unwind as usize);
    let orc_ip_size = (__stop_orc_unwind_ip as usize).wrapping_sub(__start_orc_unwind_ip as usize);
    let num_entries = orc_ip_size / core::mem::size_of::<i32>();
    if num_entries == 0 || orc_ip_size % core::mem::size_of::<i32>() != 0 || orc_size % core::mem::size_of::<orc_entry>() != 0 || num_entries != orc_size / core::mem::size_of::<orc_entry>() { orc_warn(); return; }
    lookup_num_blocks = (orc_lookup_end as usize - orc_lookup as usize) / core::mem::size_of::<u32>();
    for i in 0..lookup_num_blocks.saturating_sub(1) { let o = __orc_find(__start_orc_unwind_ip, __start_orc_unwind, num_entries as u32, LOOKUP_START_IP + LOOKUP_BLOCK_SIZE * i as usize); if o.is_null() { orc_warn(); return; } orc_lookup[i] = o.offset_from(__start_orc_unwind) as u32; }
    let o = __orc_find(__start_orc_unwind_ip, __start_orc_unwind, num_entries as u32, LOOKUP_STOP_IP); if o.is_null() { orc_warn(); return; }
    orc_lookup[lookup_num_blocks as usize - 1] = o.offset_from(__start_orc_unwind) as u32; orc_init = true;
}

unsafe fn on_stack(info: *const stack_info, addr: usize, len: usize) -> bool {
    let begin = (*info).begin; let end = (*info).end;
    (*info).type_ != STACK_TYPE_UNKNOWN && addr >= begin && addr < end && addr.wrapping_add(len) > begin && addr.wrapping_add(len) <= end
}
unsafe fn stack_access_ok(state: *mut unwind_state, addr: usize, len: usize) -> bool {
    let info = &mut (*state).stack_info;
    if on_stack(info, addr, len) { return true; }
    !get_stack_info(addr, (*state).task, info) && on_stack(info, addr, len)
}

pub unsafe fn unwind_get_return_address(state: *mut unwind_state) -> usize { __unwind_get_return_address(state) }
pub unsafe fn unwind_start(state: *mut unwind_state, task: *mut task_struct, regs: *mut pt_regs) {
    __unwind_start(state, task, regs); (*state).type_ = UNWINDER_ORC;
    if !unwind_done(state) && !__kernel_text_address((*state).pc) { unwind_next_frame(state); }
}

unsafe fn bt_address(mut ra: usize) -> usize {
    if ra >= eentry && ra < eentry + EXCCODE_INT_END * VECSIZE {
        let ty = (ra - eentry) / VECSIZE; let offset = (ra - eentry) % VECSIZE;
        let func = if ty < EXCCODE_INT_START { exception_table[ty] as usize } else if ty <= EXCCODE_INT_END { handle_vint as usize } else { handle_reserved as usize };
        ra = func + offset;
    }
    if __kernel_text_address(ra) { ra } else { 0 }
}

pub unsafe fn unwind_next_frame(state: *mut unwind_state) -> bool {
    if unwind_done(state) { return false; }
    let info = &mut (*state).stack_info;
    let mut orc = orc_find((*state).pc);
    if orc.is_null() { orc = &raw mut orc_fp_entry; (*state).error = true; }
    else if (*orc).type_ == ORC_TYPE_UNDEFINED { (*state).error = true; info.type_ = STACK_TYPE_UNKNOWN; return false; }
    else if (*orc).type_ == ORC_TYPE_END_OF_STACK { info.type_ = STACK_TYPE_UNKNOWN; return false; }
    match (*orc).sp_reg { ORC_REG_SP => (*state).sp = (*state).sp.wrapping_add((*orc).sp_offset as usize), ORC_REG_FP => (*state).sp = (*state).fp, _ => { (*state).error = true; info.type_ = STACK_TYPE_UNKNOWN; return false; } }
    if (*orc).fp_reg == ORC_REG_PREV_SP { let p = ((*state).sp as isize + (*orc).fp_offset as isize) as *mut usize; if !stack_access_ok(state, p as usize, core::mem::size_of::<usize>()) { (*state).error = true; info.type_ = STACK_TYPE_UNKNOWN; return false; } (*state).fp = *p; }
    let pc = if (*orc).type_ == ORC_TYPE_CALL { let p = ((*state).sp as isize + (*orc).ra_offset as isize) as *mut usize; if (*orc).ra_reg == ORC_REG_PREV_SP { if !stack_access_ok(state, p as usize, core::mem::size_of::<usize>()) { (*state).error = true; info.type_ = STACK_TYPE_UNKNOWN; return false; } unwind_graph_addr(state, *p, (*state).sp).wrapping_sub(LOONGARCH_INSN_SIZE) } else { if (*state).ra == 0 || (*state).ra == (*state).pc { (*state).error = true; info.type_ = STACK_TYPE_UNKNOWN; return false; } let v = unwind_graph_addr(state, (*state).ra, (*state).sp).wrapping_sub(LOONGARCH_INSN_SIZE); (*state).ra = 0; v } } else { (*state).error = true; info.type_ = STACK_TYPE_UNKNOWN; return false; };
    (*state).pc = bt_address(pc); if (*state).pc == 0 { (*state).error = true; info.type_ = STACK_TYPE_UNKNOWN; return false; } true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
