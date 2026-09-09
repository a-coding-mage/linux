// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020 - Google LLC
 * Author: David Brazdil <dbrazdil@google.com>
 */

// Dependencies are supplied by the surrounding kernel Rust environment.

extern "C" {
    fn kvm_hyp_cpu_entry(r0: libc::c_ulong);
    fn kvm_hyp_cpu_resume(r0: libc::c_ulong);
    fn __host_enter(host_ctxt: *mut kvm_cpu_context) -> !;
}

/* Config options set by the host. */
#[no_mangle]
pub static mut kvm_host_psci_config: kvm_host_psci_config = unsafe { core::mem::zeroed() };

const INVALID_CPU_ID: libc::c_uint = libc::UINT_MAX;
const PSCI_BOOT_ARGS_UNLOCKED: i32 = 0;
const PSCI_BOOT_ARGS_LOCKED: i32 = 1;

#[repr(C)]
pub struct psci_boot_args {
    pub lock: atomic_t,
    pub pc: libc::c_ulong,
    pub r0: libc::c_ulong,
}

#[no_mangle]
pub static mut cpu_on_args: psci_boot_args = psci_boot_args {
    lock: atomic_t { counter: PSCI_BOOT_ARGS_UNLOCKED },
    pc: 0,
    r0: 0,
};
#[no_mangle]
pub static mut suspend_args: psci_boot_args = psci_boot_args {
    lock: atomic_t { counter: PSCI_BOOT_ARGS_UNLOCKED },
    pc: 0,
    r0: 0,
};

unsafe fn is_psci_0_1_call(func_id: u64) -> bool {
    (kvm_host_psci_config.psci_0_1_cpu_suspend_implemented && func_id == kvm_host_psci_config.function_ids_0_1.cpu_suspend)
        || (kvm_host_psci_config.psci_0_1_cpu_on_implemented && func_id == kvm_host_psci_config.function_ids_0_1.cpu_on)
        || (kvm_host_psci_config.psci_0_1_cpu_off_implemented && func_id == kvm_host_psci_config.function_ids_0_1.cpu_off)
        || (kvm_host_psci_config.psci_0_1_migrate_implemented && func_id == kvm_host_psci_config.function_ids_0_1.migrate)
}

unsafe fn is_psci_0_2_call(func_id: u64) -> bool {
    /* SMCCC reserves IDs 0x00-1F with the given 32/64-bit base for PSCI. */
    (PSCI_0_2_FN(0) <= func_id && func_id <= PSCI_0_2_FN(31))
        || (PSCI_0_2_FN64(0) <= func_id && func_id <= PSCI_0_2_FN64(31))
}

unsafe fn psci_call(fn_: libc::c_ulong, arg0: libc::c_ulong, arg1: libc::c_ulong, arg2: libc::c_ulong) -> libc::c_ulong {
    let mut res: arm_smccc_res = core::mem::zeroed();
    hyp_smccc_1_1_smc(fn_, arg0, arg1, arg2, &mut res);
    res.a0
}

unsafe fn psci_forward(host_ctxt: *mut kvm_cpu_context) -> libc::c_ulong {
    psci_call(cpu_reg(host_ctxt, 0), cpu_reg(host_ctxt, 1), cpu_reg(host_ctxt, 2), cpu_reg(host_ctxt, 3))
}

unsafe fn find_cpu_id(mpidr: u64) -> libc::c_uint {
    if mpidr & !MPIDR_HWID_BITMASK != 0 { return INVALID_CPU_ID; }
    for i in 0..NR_CPUS {
        if cpu_logical_map(i) == mpidr { return i; }
    }
    INVALID_CPU_ID
}

unsafe fn try_acquire_boot_args(args: *mut psci_boot_args) -> bool {
    atomic_cmpxchg_acquire(&mut (*args).lock, PSCI_BOOT_ARGS_UNLOCKED, PSCI_BOOT_ARGS_LOCKED) == PSCI_BOOT_ARGS_UNLOCKED
}

unsafe fn release_boot_args(args: *mut psci_boot_args) {
    atomic_set_release(&mut (*args).lock, PSCI_BOOT_ARGS_UNLOCKED);
}

unsafe fn psci_cpu_on(func_id: u64, host_ctxt: *mut kvm_cpu_context) -> i32 {
    let mpidr = cpu_reg(host_ctxt, 1) as u64;
    let pc = cpu_reg(host_ctxt, 2);
    let r0 = cpu_reg(host_ctxt, 3);
    let cpu_id = find_cpu_id(mpidr);
    if cpu_id == INVALID_CPU_ID { return PSCI_RET_INVALID_PARAMS; }
    let boot_args = per_cpu_ptr(&mut cpu_on_args, cpu_id);
    let init_params = per_cpu_ptr(&mut kvm_init_params, cpu_id);
    if !try_acquire_boot_args(boot_args) { return PSCI_RET_ALREADY_ON; }
    (*boot_args).pc = pc;
    (*boot_args).r0 = r0;
    wmb();
    let ret = psci_call(func_id as libc::c_ulong, mpidr as libc::c_ulong, __hyp_pa(kvm_hyp_cpu_entry as *const ()), __hyp_pa(init_params));
    if ret as i32 != PSCI_RET_SUCCESS { release_boot_args(boot_args); }
    ret as i32
}

unsafe fn psci_cpu_suspend(func_id: u64, host_ctxt: *mut kvm_cpu_context) -> i32 {
    let power_state = cpu_reg(host_ctxt, 1);
    let pc = cpu_reg(host_ctxt, 2);
    let r0 = cpu_reg(host_ctxt, 3);
    let boot_args = this_cpu_ptr(&mut suspend_args);
    let init_params = this_cpu_ptr(&mut kvm_init_params);
    (*boot_args).pc = pc; (*boot_args).r0 = r0;
    psci_call(func_id as libc::c_ulong, power_state, __hyp_pa(kvm_hyp_cpu_resume as *const ()), __hyp_pa(init_params)) as i32
}

unsafe fn psci_system_suspend(func_id: u64, host_ctxt: *mut kvm_cpu_context) -> i32 {
    let pc = cpu_reg(host_ctxt, 1); let r0 = cpu_reg(host_ctxt, 2);
    let boot_args = this_cpu_ptr(&mut suspend_args);
    let init_params = this_cpu_ptr(&mut kvm_init_params);
    (*boot_args).pc = pc; (*boot_args).r0 = r0;
    psci_call(func_id as libc::c_ulong, __hyp_pa(kvm_hyp_cpu_resume as *const ()), __hyp_pa(init_params), 0) as i32
}

unsafe fn __kvm_host_psci_cpu_entry(pc: libc::c_ulong, r0: libc::c_ulong) -> ! {
    let host_ctxt = host_data_ptr(host_ctxt);
    trace_hyp_enter(host_ctxt, HYP_REASON_PSCI);
    cpu_reg(host_ctxt, 0) = r0;
    write_sysreg_el2(pc, SYS_ELR);
    write_sysreg_el1(INIT_SCTLR_EL1_MMU_OFF, SYS_SCTLR);
    write_sysreg(INIT_PSTATE_EL1, SPSR_EL2);
    trace_hyp_exit(host_ctxt, HYP_REASON_PSCI);
    __host_enter(host_ctxt)
}

#[no_mangle]
pub unsafe extern "C" fn __kvm_host_psci_cpu_on_entry() -> ! {
    let boot_args = this_cpu_ptr(&mut cpu_on_args);
    let pc = READ_ONCE((*boot_args).pc); let r0 = READ_ONCE((*boot_args).r0);
    release_boot_args(boot_args); __kvm_host_psci_cpu_entry(pc, r0)
}

#[no_mangle]
pub unsafe extern "C" fn __kvm_host_psci_cpu_resume_entry() -> ! {
    let boot_args = this_cpu_ptr(&mut suspend_args);
    __kvm_host_psci_cpu_entry((*boot_args).pc, (*boot_args).r0)
}

unsafe fn psci_0_1_handler(func_id: u64, host_ctxt: *mut kvm_cpu_context) -> libc::c_ulong {
    if (kvm_host_psci_config.psci_0_1_cpu_off_implemented && func_id == kvm_host_psci_config.function_ids_0_1.cpu_off) || (kvm_host_psci_config.psci_0_1_migrate_implemented && func_id == kvm_host_psci_config.function_ids_0_1.migrate) { return psci_forward(host_ctxt); }
    if kvm_host_psci_config.psci_0_1_cpu_on_implemented && func_id == kvm_host_psci_config.function_ids_0_1.cpu_on { return psci_cpu_on(func_id, host_ctxt) as libc::c_ulong; }
    if kvm_host_psci_config.psci_0_1_cpu_suspend_implemented && func_id == kvm_host_psci_config.function_ids_0_1.cpu_suspend { return psci_cpu_suspend(func_id, host_ctxt) as libc::c_ulong; }
    PSCI_RET_NOT_SUPPORTED as libc::c_ulong
}

unsafe fn psci_0_2_handler(func_id: u64, host_ctxt: *mut kvm_cpu_context) -> libc::c_ulong {
    match func_id {
        PSCI_0_2_FN_PSCI_VERSION | PSCI_0_2_FN_CPU_OFF | PSCI_0_2_FN64_AFFINITY_INFO | PSCI_0_2_FN64_MIGRATE | PSCI_0_2_FN_MIGRATE_INFO_TYPE | PSCI_0_2_FN64_MIGRATE_INFO_UP_CPU | PSCI_0_2_FN_SYSTEM_OFF | PSCI_0_2_FN_SYSTEM_RESET => psci_forward(host_ctxt),
        PSCI_0_2_FN64_CPU_SUSPEND => psci_cpu_suspend(func_id, host_ctxt) as libc::c_ulong,
        PSCI_0_2_FN64_CPU_ON => psci_cpu_on(func_id, host_ctxt) as libc::c_ulong,
        _ => PSCI_RET_NOT_SUPPORTED as libc::c_ulong,
    }
}

unsafe fn psci_1_0_handler(func_id: u64, host_ctxt: *mut kvm_cpu_context) -> libc::c_ulong {
    match func_id {
        PSCI_1_0_FN_PSCI_FEATURES | PSCI_1_0_FN_SET_SUSPEND_MODE | PSCI_1_1_FN64_SYSTEM_RESET2 | PSCI_1_3_FN_SYSTEM_OFF2 | PSCI_1_3_FN64_SYSTEM_OFF2 => psci_forward(host_ctxt),
        PSCI_1_0_FN64_SYSTEM_SUSPEND => psci_system_suspend(func_id, host_ctxt) as libc::c_ulong,
        _ => psci_0_2_handler(func_id, host_ctxt),
    }
}

#[no_mangle]
pub unsafe extern "C" fn kvm_host_psci_handler(host_ctxt: *mut kvm_cpu_context, func_id: u32) -> bool {
    let ret = match kvm_host_psci_config.version {
        x if x == PSCI_VERSION(0, 1) => { if !is_psci_0_1_call(func_id as u64) { return false; } psci_0_1_handler(func_id as u64, host_ctxt) },
        x if x == PSCI_VERSION(0, 2) => { if !is_psci_0_2_call(func_id as u64) { return false; } psci_0_2_handler(func_id as u64, host_ctxt) },
        _ => { if !is_psci_0_2_call(func_id as u64) { return false; } psci_1_0_handler(func_id as u64, host_ctxt) },
    };
    cpu_reg(host_ctxt, 0) = ret; cpu_reg(host_ctxt, 1) = 0; cpu_reg(host_ctxt, 2) = 0; cpu_reg(host_ctxt, 3) = 0;
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
