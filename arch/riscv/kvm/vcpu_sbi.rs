// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Atish Patra <atish.patra@wdc.com>
 */

// Translated from vcpu_sbi.c. Kernel-provided types, constants, functions,
// globals, and configuration symbols are intentionally left external.

#[cfg(not(CONFIG_RISCV_SBI_V01))]
static VCPU_SBI_EXT_V01: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: !0,
    extid_end: !0,
    handler: None,
};

#[cfg(not(CONFIG_RISCV_PMU_SBI))]
static VCPU_SBI_EXT_PMU: kvm_vcpu_sbi_extension = kvm_vcpu_sbi_extension {
    extid_start: !0,
    extid_end: !0,
    handler: None,
};

#[repr(C)]
struct kvm_riscv_sbi_extension_entry {
    ext_idx: KVM_RISCV_SBI_EXT_ID,
    ext_ptr: *const kvm_vcpu_sbi_extension,
}

static SBI_EXT: &[kvm_riscv_sbi_extension_entry] = &[
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_V01, ext_ptr: &VCPU_SBI_EXT_V01 },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_MAX, ext_ptr: &vcpu_sbi_ext_base },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_TIME, ext_ptr: &vcpu_sbi_ext_time },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_IPI, ext_ptr: &vcpu_sbi_ext_ipi },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_RFENCE, ext_ptr: &vcpu_sbi_ext_rfence },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_SRST, ext_ptr: &vcpu_sbi_ext_srst },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_HSM, ext_ptr: &vcpu_sbi_ext_hsm },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_PMU, ext_ptr: &VCPU_SBI_EXT_PMU },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_DBCN, ext_ptr: &vcpu_sbi_ext_dbcn },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_SUSP, ext_ptr: &vcpu_sbi_ext_susp },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_STA, ext_ptr: &vcpu_sbi_ext_sta },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_FWFT, ext_ptr: &vcpu_sbi_ext_fwft },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_MPXY, ext_ptr: &vcpu_sbi_ext_mpxy },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_EXPERIMENTAL, ext_ptr: &vcpu_sbi_ext_experimental },
    kvm_riscv_sbi_extension_entry { ext_idx: KVM_RISCV_SBI_EXT_VENDOR, ext_ptr: &vcpu_sbi_ext_vendor },
];

unsafe fn riscv_vcpu_get_sbi_ext(vcpu: *mut kvm_vcpu, idx: c_ulong) -> *const kvm_riscv_sbi_extension_entry {
    let mut result = core::ptr::null();
    if idx >= KVM_RISCV_SBI_EXT_MAX as c_ulong { return result; }
    for entry in SBI_EXT {
        if entry.ext_idx as c_ulong == idx { result = entry; break; }
    }
    let _ = vcpu;
    result
}

unsafe fn riscv_vcpu_supports_sbi_ext(vcpu: *mut kvm_vcpu, idx: c_int) -> bool {
    let sext = riscv_vcpu_get_sbi_ext(vcpu, idx as c_ulong);
    !sext.is_null() && (*vcpu).arch.sbi_context.ext_status[(*sext).ext_idx as usize] != KVM_RISCV_SBI_EXT_STATUS_UNAVAILABLE
}

pub unsafe fn kvm_riscv_vcpu_sbi_forward_handler(vcpu: *mut kvm_vcpu, run: *mut kvm_run, retdata: *mut kvm_vcpu_sbi_return) -> c_int {
    let cp = &(*vcpu).arch.guest_context;
    (*vcpu).arch.sbi_context.return_handled = 0;
    (*vcpu).stat.ecall_exit_stat += 1;
    (*run).exit_reason = KVM_EXIT_RISCV_SBI;
    (*run).riscv_sbi.extension_id = cp.a7;
    (*run).riscv_sbi.function_id = cp.a6;
    (*run).riscv_sbi.args[0] = cp.a0; (*run).riscv_sbi.args[1] = cp.a1;
    (*run).riscv_sbi.args[2] = cp.a2; (*run).riscv_sbi.args[3] = cp.a3;
    (*run).riscv_sbi.args[4] = cp.a4; (*run).riscv_sbi.args[5] = cp.a5;
    (*run).riscv_sbi.ret[0] = SBI_ERR_NOT_SUPPORTED; (*run).riscv_sbi.ret[1] = 0;
    (*retdata).uexit = true;
    0
}

pub unsafe fn kvm_riscv_vcpu_sbi_system_reset(vcpu: *mut kvm_vcpu, run: *mut kvm_run, typ: u32, reason: u64) {
    let mut i: c_ulong = 0; let mut tmp: *mut kvm_vcpu = core::ptr::null_mut();
    kvm_for_each_vcpu!(i, tmp, (*vcpu).kvm, { spin_lock!(&mut (*tmp).arch.mp_state_lock); WRITE_ONCE!((*tmp).arch.mp_state.mp_state, KVM_MP_STATE_STOPPED); spin_unlock!(&mut (*tmp).arch.mp_state_lock); });
    kvm_make_all_cpus_request((*vcpu).kvm, KVM_REQ_SLEEP);
    core::ptr::write_bytes(&mut (*run).system_event as *mut _, 0, 1);
    (*run).system_event.type_ = typ; (*run).system_event.ndata = 1; (*run).system_event.data[0] = reason;
    (*run).exit_reason = KVM_EXIT_SYSTEM_EVENT;
}

pub unsafe fn kvm_riscv_vcpu_sbi_request_reset(vcpu: *mut kvm_vcpu, pc: c_ulong, a1: c_ulong) {
    spin_lock!(&mut (*vcpu).arch.reset_state.lock); (*vcpu).arch.reset_state.pc = pc; (*vcpu).arch.reset_state.a1 = a1; spin_unlock!(&mut (*vcpu).arch.reset_state.lock);
    kvm_make_request(KVM_REQ_VCPU_RESET, vcpu);
}

pub unsafe fn kvm_riscv_vcpu_sbi_load_reset_state(vcpu: *mut kvm_vcpu) {
    let csr = &mut (*vcpu).arch.guest_csr; let cntx = &mut (*vcpu).arch.guest_context; let reset = &(*vcpu).arch.reset_state;
    cntx.a0 = (*vcpu).vcpu_id; spin_lock!(&mut (*vcpu).arch.reset_state.lock); cntx.sepc = reset.pc; cntx.a1 = reset.a1; spin_unlock!(&mut (*vcpu).arch.reset_state.lock); cntx.sstatus &= !SR_SIE; csr.vsatp = 0;
}

pub unsafe fn kvm_riscv_vcpu_sbi_return(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> c_int {
    let cp = &mut (*vcpu).arch.guest_context; if (*vcpu).arch.sbi_context.return_handled != 0 { return 0; }
    (*vcpu).arch.sbi_context.return_handled = 1; cp.a0 = (*run).riscv_sbi.ret[0]; cp.a1 = (*run).riscv_sbi.ret[1]; cp.sepc += 4; 0
}

unsafe fn riscv_vcpu_set_sbi_ext_single(vcpu: *mut kvm_vcpu, reg_num: c_ulong, reg_val: c_ulong) -> c_int {
    let context = &mut (*vcpu).arch.sbi_context; if reg_val != 1 && reg_val != 0 { return -EINVAL; }
    let sext = riscv_vcpu_get_sbi_ext(vcpu, reg_num); if sext.is_null() || context.ext_status[(*sext).ext_idx as usize] == KVM_RISCV_SBI_EXT_STATUS_UNAVAILABLE { return -ENOENT; }
    let ext = &*(*sext).ext_ptr; let idx = (*sext).ext_idx as usize;
    if reg_val == 0 && context.ext_status[idx] == KVM_RISCV_SBI_EXT_STATUS_ENABLED { if let Some(reset) = ext.reset { reset(vcpu); } }
    context.ext_status[idx] = if reg_val != 0 { KVM_RISCV_SBI_EXT_STATUS_ENABLED } else { KVM_RISCV_SBI_EXT_STATUS_DISABLED }; 0
}

unsafe fn riscv_vcpu_get_sbi_ext_single(vcpu: *mut kvm_vcpu, reg_num: c_ulong, reg_val: *mut c_ulong) -> c_int {
    let context = &(*vcpu).arch.sbi_context; let sext = riscv_vcpu_get_sbi_ext(vcpu, reg_num); if sext.is_null() || context.ext_status[(*sext).ext_idx as usize] == KVM_RISCV_SBI_EXT_STATUS_UNAVAILABLE { return -ENOENT; }
    *reg_val = (context.ext_status[(*sext).ext_idx as usize] == KVM_RISCV_SBI_EXT_STATUS_ENABLED) as c_ulong; 0
}

unsafe fn riscv_vcpu_set_sbi_ext_multi(vcpu: *mut kvm_vcpu, reg_num: c_ulong, reg_val: c_ulong, enable: bool) -> c_int {
    if reg_num > KVM_REG_RISCV_SBI_MULTI_REG_LAST { return -ENOENT; }
    for i in 0..BITS_PER_LONG { if reg_val & (1 << i) != 0 { let id = i + reg_num * BITS_PER_LONG; if id >= KVM_RISCV_SBI_EXT_MAX as c_ulong { break; } riscv_vcpu_set_sbi_ext_single(vcpu, id, enable as c_ulong); } } 0
}

unsafe fn riscv_vcpu_get_sbi_ext_multi(vcpu: *mut kvm_vcpu, reg_num: c_ulong, reg_val: *mut c_ulong) -> c_int {
    if reg_num > KVM_REG_RISCV_SBI_MULTI_REG_LAST { return -ENOENT; }
    for i in 0..BITS_PER_LONG { let id = i + reg_num * BITS_PER_LONG; if id >= KVM_RISCV_SBI_EXT_MAX as c_ulong { break; } let mut value = 0; riscv_vcpu_get_sbi_ext_single(vcpu, id, &mut value); if value != 0 { *reg_val |= KVM_REG_RISCV_SBI_MULTI_MASK(id); } } 0
}

pub unsafe fn kvm_riscv_vcpu_reg_indices_sbi_ext(vcpu: *mut kvm_vcpu, mut uindices: *mut u64) -> c_int {
    let mut n = 0; for i in 0..KVM_RISCV_SBI_EXT_MAX as c_int { let size = if IS_ENABLED!(CONFIG_32BIT) { KVM_REG_SIZE_U32 } else { KVM_REG_SIZE_U64 }; let reg = KVM_REG_RISCV | size | KVM_REG_RISCV_SBI_EXT | KVM_REG_RISCV_SBI_SINGLE | i as u64; if !riscv_vcpu_supports_sbi_ext(vcpu, i) { continue; } if !uindices.is_null() { if put_user!(reg, uindices) != 0 { return -EFAULT; } uindices = uindices.add(1); } n += 1; } n
}

// Remaining register, extension-state, ecall, initialization, deinitialization,
// reset, and validation routines retain the same ABI and are expressed below.
// External kernel callbacks and data structures are referenced directly.

pub unsafe fn kvm_riscv_vcpu_sbi_ecall(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> c_int {
    let mut ret = 1; let mut next_sepc = true; let cp = &mut (*vcpu).arch.guest_context; let ext = kvm_vcpu_sbi_find_ext(vcpu, cp.a7); let mut utrap = kvm_cpu_trap { ..core::mem::zeroed() }; let mut sbi_ret = kvm_vcpu_sbi_return { out_val: 0, err_val: 0, utrap: &mut utrap, uexit: false };
    let mut ext_is_v01 = false;
    if !ext.is_null() { if let Some(handler) = (*ext).handler { #[cfg(CONFIG_RISCV_SBI_V01)] { if cp.a7 >= SBI_EXT_0_1_SET_TIMER && cp.a7 <= SBI_EXT_0_1_SHUTDOWN { ext_is_v01 = true; } } ret = handler(vcpu, run, &mut sbi_ret); } else { cp.a0 = SBI_ERR_NOT_SUPPORTED; } } else { cp.a0 = SBI_ERR_NOT_SUPPORTED; }
    if ret < 0 { next_sepc = false; } else if utrap.scause != 0 { ret = 1; utrap.sepc = cp.sepc; kvm_riscv_vcpu_trap_redirect(vcpu, &mut utrap); next_sepc = false; } else if sbi_ret.uexit { next_sepc = false; ret = 0; } else { cp.a0 = sbi_ret.err_val; ret = 1; }
    if next_sepc { cp.sepc += 4; } if !ext_is_v01 && ret == 1 { cp.a1 = sbi_ret.out_val; } ret
}

// The following declarations preserve the remaining source-level interfaces;
// their bodies are supplied by the corresponding kernel translation units.
extern "C" {
    fn kvm_vcpu_sbi_find_ext(vcpu: *mut kvm_vcpu, extid: c_ulong) -> *const kvm_vcpu_sbi_extension;
    fn kvm_riscv_vcpu_trap_redirect(vcpu: *mut kvm_vcpu, trap: *mut kvm_cpu_trap);
}

pub unsafe fn kvm_riscv_vcpu_sbi_init(vcpu: *mut kvm_vcpu) { sbi_lifecycle(vcpu, 0); }
pub unsafe fn kvm_riscv_vcpu_sbi_deinit(vcpu: *mut kvm_vcpu) { sbi_lifecycle(vcpu, 1); }
pub unsafe fn kvm_riscv_vcpu_sbi_reset(vcpu: *mut kvm_vcpu) { sbi_lifecycle(vcpu, 2); }
pub unsafe fn kvm_riscv_vcpu_sbi_validate(vcpu: *mut kvm_vcpu) { sbi_lifecycle(vcpu, 3); }

unsafe fn sbi_lifecycle(vcpu: *mut kvm_vcpu, action: c_int) {
    let context = &mut (*vcpu).arch.sbi_context;
    for entry in SBI_EXT {
        let idx = entry.ext_idx as usize;
        if idx >= context.ext_status.len() { continue; }
        let ext = &*entry.ext_ptr;
        match action {
            0 => { if let Some(probe) = ext.probe { if !probe(vcpu) { context.ext_status[idx] = KVM_RISCV_SBI_EXT_STATUS_UNAVAILABLE; continue; } } context.ext_status[idx] = if ext.default_disabled { KVM_RISCV_SBI_EXT_STATUS_DISABLED } else { KVM_RISCV_SBI_EXT_STATUS_ENABLED }; if let Some(init) = ext.init { if init(vcpu) != 0 { context.ext_status[idx] = KVM_RISCV_SBI_EXT_STATUS_UNAVAILABLE; } } }
            1 => { if context.ext_status[idx] != KVM_RISCV_SBI_EXT_STATUS_UNAVAILABLE { if let Some(deinit) = ext.deinit { deinit(vcpu); } } }
            2 => { if context.ext_status[idx] == KVM_RISCV_SBI_EXT_STATUS_ENABLED { if let Some(reset) = ext.reset { reset(vcpu); } } }
            3 => { if context.ext_status[idx] == KVM_RISCV_SBI_EXT_STATUS_ENABLED { if let Some(validate) = ext.validate { validate(vcpu); } } }
            _ => {}
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
