// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of the ARC unaligned access fix-up handler. */

#[cfg(target_endian = "big")]
const BE: bool = true;
#[cfg(not(target_endian = "big"))]
const BE: bool = false;

#[repr(C)]
pub struct disasm_state {
    pub aa: i32, pub wb_reg: i32, pub src1: u32, pub src2: u32, pub src3: u32,
    pub zz: i32, pub x: i32, pub pref: i32, pub dest: i32, pub fault: i32,
    pub di: i32, pub write: i32, pub instr_len: u32, pub words: [u32; 1],
}
#[repr(C)]
pub struct pt_regs {
    pub ret: u32, pub bta: u32, pub status32: u32, pub lp_end: u32,
    pub lp_count: u32, pub lp_start: u32,
}
#[repr(C)] pub struct callee_regs { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { pub comm: [u8; 16] }

extern "C" {
    fn set_reg(reg: i32, value: u32, regs: *mut pt_regs, cregs: *mut callee_regs);
    fn user_mode(regs: *const pt_regs) -> i32;
    fn disasm_instr(pc: u32, state: *mut disasm_state, flag: i32,
                    regs: *mut pt_regs, cregs: *mut callee_regs);
    fn delay_mode(regs: *const pt_regs) -> i32;
    fn perf_sw_event(event: u32, count: u64, regs: *mut pt_regs, address: u32);
    static mut current: *mut task_struct;
    fn task_pid_nr(task: *mut task_struct) -> i32;
    fn pr_warn_once(fmt: *const u8, ...);
    fn pr_warn(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
}

pub static mut unaligned_enabled: i32 = 1;
pub static mut no_unaligned_warning: i32 = 1;
const STATUS_DE_MASK: u32 = 1 << 6;
const PERF_COUNT_SW_ALIGNMENT_FAULTS: u32 = 6;

unsafe fn get_bytes(mut address: u32, count: usize, value: &mut u32) -> bool {
    let mut v = 0u32;
    for i in 0..count {
        let b = core::ptr::read_volatile(address as *const u8) as u32;
        address = address.wrapping_add(1);
        if BE { v |= b << (8 * (count - 1 - i)); }
        else { v |= b << (8 * i); }
    }
    *value = v;
    true
}

unsafe fn put_bytes(mut value: u32, mut address: u32, count: usize) -> bool {
    if BE { value = value.rotate_left((32 - 8 * count) as u32); }
    for i in 0..count {
        core::ptr::write_volatile(address as *mut u8, (value >> (8 * i)) as u8);
        address = address.wrapping_add(1);
    }
    true
}

unsafe fn fixup_load(state: *mut disasm_state, regs: *mut pt_regs, cregs: *mut callee_regs) {
    let s = &mut *state;
    if s.aa == 1 || s.aa == 2 { set_reg(s.wb_reg, s.src1.wrapping_add(s.src2), regs, cregs); if s.aa == 2 { s.src2 = 0; } }
    let mut val = 0u32;
    if !get_bytes(s.src1.wrapping_add(s.src2), if s.zz == 0 { 4 } else { 2 }, &mut val) { s.fault = 1; return; }
    if s.zz != 0 && s.x != 0 { val = ((val as i16) as i32) as u32; }
    if s.pref == 0 { set_reg(s.dest, val, regs, cregs); }
}

unsafe fn fixup_store(state: *mut disasm_state, regs: *mut pt_regs, cregs: *mut callee_regs) {
    let s = &mut *state;
    if s.aa == 1 || s.aa == 2 { set_reg(s.wb_reg, s.src2.wrapping_add(s.src3), regs, cregs); if s.aa == 3 { s.src3 = 0; } }
    else if s.aa == 3 { s.src2 = s.src2.wrapping_add(if s.zz == 2 { s.src3 << 1 } else if s.zz == 0 { s.src3 << 2 } else { s.fault = 1; return; }); }
    if !put_bytes(s.src1, s.src2.wrapping_add(s.src3), if s.zz == 0 { 4 } else { 2 }) { s.fault = 1; }
}

pub unsafe fn misaligned_fixup(address: u32, regs: *mut pt_regs, cregs: *mut callee_regs) -> i32 {
    let mut state = core::mem::zeroed::<disasm_state>();
    if user_mode(regs) == 0 || unaligned_enabled == 0 { return 1; }
    disasm_instr((*regs).ret, &mut state, 1, regs, cregs);
    if state.fault != 0 || state.zz == 1 || state.di != 0 { return 1; }
    if state.write == 0 { fixup_load(&mut state, regs, cregs); } else { fixup_store(&mut state, regs, cregs); }
    if state.fault != 0 { return 1; }
    if delay_mode(regs) != 0 { (*regs).ret = (*regs).bta & !1; (*regs).status32 &= !STATUS_DE_MASK; }
    else { (*regs).ret = (*regs).ret.wrapping_add(state.instr_len); if (*regs).ret == (*regs).lp_end && (*regs).lp_count != 0 { (*regs).ret = (*regs).lp_start; (*regs).lp_count -= 1; } }
    perf_sw_event(PERF_COUNT_SW_ALIGNMENT_FAULTS, 1, regs, address); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
