// SPDX-License-Identifier: GPL-2.0
// C includes are supplied by the surrounding kernel translation unit.

#[cfg(CONFIG_X86_32)]
const PERF_REG_X86_MAX: usize = PERF_REG_X86_32_MAX as usize;
#[cfg(not(CONFIG_X86_32))]
const PERF_REG_X86_MAX: usize = PERF_REG_X86_64_MAX as usize;

#[cfg(CONFIG_X86_32)]
static mut PT_REGS_OFFSET: [u32; PERF_REG_X86_MAX] = [
    core::mem::offset_of!(pt_regs, ax) as u32,
    core::mem::offset_of!(pt_regs, bx) as u32,
    core::mem::offset_of!(pt_regs, cx) as u32,
    core::mem::offset_of!(pt_regs, dx) as u32,
    core::mem::offset_of!(pt_regs, si) as u32,
    core::mem::offset_of!(pt_regs, di) as u32,
    core::mem::offset_of!(pt_regs, bp) as u32,
    core::mem::offset_of!(pt_regs, sp) as u32,
    core::mem::offset_of!(pt_regs, ip) as u32,
    core::mem::offset_of!(pt_regs, flags) as u32,
    core::mem::offset_of!(pt_regs, cs) as u32,
    core::mem::offset_of!(pt_regs, ss) as u32,
    core::mem::offset_of!(pt_regs, ds) as u32,
    core::mem::offset_of!(pt_regs, es) as u32,
    core::mem::offset_of!(pt_regs, fs) as u32,
    core::mem::offset_of!(pt_regs, gs) as u32,
];

#[cfg(not(CONFIG_X86_32))]
static mut PT_REGS_OFFSET: [u32; PERF_REG_X86_MAX] = [
    core::mem::offset_of!(pt_regs, ax) as u32,
    core::mem::offset_of!(pt_regs, bx) as u32,
    core::mem::offset_of!(pt_regs, cx) as u32,
    core::mem::offset_of!(pt_regs, dx) as u32,
    core::mem::offset_of!(pt_regs, si) as u32,
    core::mem::offset_of!(pt_regs, di) as u32,
    core::mem::offset_of!(pt_regs, bp) as u32,
    core::mem::offset_of!(pt_regs, sp) as u32,
    core::mem::offset_of!(pt_regs, ip) as u32,
    core::mem::offset_of!(pt_regs, flags) as u32,
    core::mem::offset_of!(pt_regs, cs) as u32,
    core::mem::offset_of!(pt_regs, ss) as u32,
    u32::MAX,
    u32::MAX,
    u32::MAX,
    u32::MAX,
    #[cfg(CONFIG_X86_64)] core::mem::offset_of!(pt_regs, r8) as u32,
    #[cfg(CONFIG_X86_64)] core::mem::offset_of!(pt_regs, r9) as u32,
    #[cfg(CONFIG_X86_64)] core::mem::offset_of!(pt_regs, r10) as u32,
    #[cfg(CONFIG_X86_64)] core::mem::offset_of!(pt_regs, r11) as u32,
    #[cfg(CONFIG_X86_64)] core::mem::offset_of!(pt_regs, r12) as u32,
    #[cfg(CONFIG_X86_64)] core::mem::offset_of!(pt_regs, r13) as u32,
    #[cfg(CONFIG_X86_64)] core::mem::offset_of!(pt_regs, r14) as u32,
    #[cfg(CONFIG_X86_64)] core::mem::offset_of!(pt_regs, r15) as u32,
];

pub unsafe fn perf_reg_value(regs: *mut pt_regs, idx: i32) -> u64 {
    if idx >= PERF_REG_X86_XMM0 as i32 && idx < PERF_REG_X86_XMM_MAX as i32 {
        let perf_regs = (regs as *mut u8).sub(core::mem::offset_of!(x86_perf_regs, regs)) as *mut x86_perf_regs;
        if (*perf_regs).xmm_regs.is_null() { return 0; }
        return *(*perf_regs).xmm_regs.add((idx - PERF_REG_X86_XMM0 as i32) as usize);
    }
    if idx < 0 || (idx as usize) >= PT_REGS_OFFSET.len() { return 0; }
    regs_get_register(regs, PT_REGS_OFFSET[idx as usize])
}

const PERF_REG_X86_RESERVED: u64 = ((1u64 << PERF_REG_X86_XMM0) - 1) & !((1u64 << PERF_REG_X86_MAX) - 1);

#[cfg(CONFIG_X86_32)]
const REG_NOSUPPORT: u64 = (1u64 << PERF_REG_X86_R8) | (1u64 << PERF_REG_X86_R9) | (1u64 << PERF_REG_X86_R10) | (1u64 << PERF_REG_X86_R11) | (1u64 << PERF_REG_X86_R12) | (1u64 << PERF_REG_X86_R13) | (1u64 << PERF_REG_X86_R14) | (1u64 << PERF_REG_X86_R15);
#[cfg(not(CONFIG_X86_32))]
const REG_NOSUPPORT: u64 = (1u64 << PERF_REG_X86_DS) | (1u64 << PERF_REG_X86_ES) | (1u64 << PERF_REG_X86_FS) | (1u64 << PERF_REG_X86_GS);

pub fn perf_reg_validate(mask: u64) -> i32 {
    if mask == 0 || (mask & (REG_NOSUPPORT | PERF_REG_X86_RESERVED)) != 0 { return -EINVAL; }
    0
}

#[cfg(CONFIG_X86_32)]
pub unsafe fn perf_reg_abi(_task: *mut task_struct) -> u64 { PERF_SAMPLE_REGS_ABI_32 }

#[cfg(not(CONFIG_X86_32))]
pub unsafe fn perf_reg_abi(task: *mut task_struct) -> u64 {
    if !user_64bit_mode(task_pt_regs(task)) { PERF_SAMPLE_REGS_ABI_32 } else { PERF_SAMPLE_REGS_ABI_64 }
}

#[cfg(CONFIG_X86_32)]
pub unsafe fn perf_get_regs_user(regs_user: *mut perf_regs, _regs: *mut pt_regs) {
    (*regs_user).regs = task_pt_regs(current);
    (*regs_user).abi = perf_reg_abi(current);
}

#[cfg(not(CONFIG_X86_32))]
pub unsafe fn perf_get_regs_user(regs_user: *mut perf_regs, regs: *mut pt_regs) {
    let regs_user_copy = this_cpu_ptr(&mut nmi_user_regs);
    let user_regs = task_pt_regs(current);
    if !in_nmi() { (*regs_user).regs = user_regs; (*regs_user).abi = perf_reg_abi(current); return; }
    if (*regs).sp > (&(*user_regs).r11 as *const _ as usize) && (*regs).sp <= (user_regs.add(1) as usize) {
        (*regs_user).abi = PERF_SAMPLE_REGS_ABI_NONE; (*regs_user).regs = core::ptr::null_mut(); return;
    }
    (*regs_user_copy).ip = (*user_regs).ip; (*regs_user_copy).ax = (*user_regs).ax; (*regs_user_copy).cx = (*user_regs).cx; (*regs_user_copy).dx = (*user_regs).dx;
    (*regs_user_copy).si = (*user_regs).si; (*regs_user_copy).di = (*user_regs).di; (*regs_user_copy).r8 = (*user_regs).r8; (*regs_user_copy).r9 = (*user_regs).r9;
    (*regs_user_copy).r10 = (*user_regs).r10; (*regs_user_copy).r11 = (*user_regs).r11; (*regs_user_copy).orig_ax = (*user_regs).orig_ax;
    (*regs_user_copy).flags = (*user_regs).flags; (*regs_user_copy).sp = (*user_regs).sp; (*regs_user_copy).cs = (*user_regs).cs; (*regs_user_copy).ss = (*user_regs).ss;
    (*regs_user_copy).bp = (*user_regs).bp;
    (*regs_user_copy).bx = -1; (*regs_user_copy).r12 = -1; (*regs_user_copy).r13 = -1; (*regs_user_copy).r14 = -1; (*regs_user_copy).r15 = -1;
    (*regs_user).abi = if user_64bit_mode(user_regs) { PERF_SAMPLE_REGS_ABI_64 } else { PERF_SAMPLE_REGS_ABI_32 };
    (*regs_user).regs = regs_user_copy;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
