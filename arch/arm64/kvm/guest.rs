// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012,2013 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 *
 * Derived from arch/arm/kvm/guest.c:
 * Copyright (C) 2012 - Virtual Open Systems and Columbia University
 * Author: Christoffer Dall <c.dall@virtualopensystems.com>
 */

/* Linux headers and the symbols they provide are external dependencies. */

pub static KVM_VM_STATS_DESC: [kvm_stats_desc; 0] = [];
pub static KVM_VM_STATS_HEADER: kvm_stats_header = kvm_stats_header {
    name_size: KVM_STATS_NAME_SIZE,
    num_desc: KVM_VM_STATS_DESC.len(),
    id_offset: core::mem::size_of::<kvm_stats_header>(),
    desc_offset: core::mem::size_of::<kvm_stats_header>() + KVM_STATS_NAME_SIZE,
    data_offset: core::mem::size_of::<kvm_stats_header>() + KVM_STATS_NAME_SIZE +
        core::mem::size_of::<[kvm_stats_desc; 0]>(),
};

pub static KVM_VCPU_STATS_DESC: [kvm_stats_desc; 0] = [];
pub static KVM_VCPU_STATS_HEADER: kvm_stats_header = kvm_stats_header {
    name_size: KVM_STATS_NAME_SIZE,
    num_desc: KVM_VCPU_STATS_DESC.len(),
    id_offset: core::mem::size_of::<kvm_stats_header>(),
    desc_offset: core::mem::size_of::<kvm_stats_header>() + KVM_STATS_NAME_SIZE,
    data_offset: core::mem::size_of::<kvm_stats_header>() + KVM_STATS_NAME_SIZE +
        core::mem::size_of::<[kvm_stats_desc; 0]>(),
};

unsafe fn core_reg_offset_is_vreg(off: u64) -> bool {
    off >= KVM_REG_ARM_CORE_REG(fp_regs.vregs) && off < KVM_REG_ARM_CORE_REG(fp_regs.fpsr)
}
unsafe fn core_reg_offset_from_id(id: u64) -> u64 {
    id & !(KVM_REG_ARCH_MASK | KVM_REG_SIZE_MASK | KVM_REG_ARM_CORE)
}
unsafe fn core_reg_size_from_offset(vcpu: *const kvm_vcpu, off: u64) -> i32 {
    let size: i32;
    match off {
        KVM_REG_ARM_CORE_REG(regs.regs[0]) ..= KVM_REG_ARM_CORE_REG(regs.regs[30]) |
        KVM_REG_ARM_CORE_REG(regs.sp) | KVM_REG_ARM_CORE_REG(regs.pc) |
        KVM_REG_ARM_CORE_REG(regs.pstate) | KVM_REG_ARM_CORE_REG(sp_el1) |
        KVM_REG_ARM_CORE_REG(elr_el1) |
        KVM_REG_ARM_CORE_REG(spsr[0]) ..= KVM_REG_ARM_CORE_REG(spsr[KVM_NR_SPSR - 1]) =>
            size = core::mem::size_of::<u64>() as i32,
        KVM_REG_ARM_CORE_REG(fp_regs.vregs[0]) ..= KVM_REG_ARM_CORE_REG(fp_regs.vregs[31]) =>
            size = core::mem::size_of::<u128>() as i32,
        KVM_REG_ARM_CORE_REG(fp_regs.fpsr) | KVM_REG_ARM_CORE_REG(fp_regs.fpcr) =>
            size = core::mem::size_of::<u32>() as i32,
        _ => return -EINVAL,
    }
    if !off.is_multiple_of((size as u64) / core::mem::size_of::<u32>() as u64) ||
       (vcpu_has_sve(vcpu) && core_reg_offset_is_vreg(off)) { return -EINVAL; }
    size
}

unsafe fn core_reg_addr(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> *mut core::ffi::c_void {
    let off = core_reg_offset_from_id((*reg).id);
    let size = core_reg_size_from_offset(vcpu, off);
    if size < 0 || KVM_REG_SIZE((*reg).id) != size as u64 { return core::ptr::null_mut(); }
    match off {
        KVM_REG_ARM_CORE_REG(regs.regs[0]) ..= KVM_REG_ARM_CORE_REG(regs.regs[30]) => {
            let n = (off - KVM_REG_ARM_CORE_REG(regs.regs[0])) / 2;
            (&mut (*vcpu).arch.ctxt.regs.regs[n as usize]) as *mut _ as *mut _
        }
        KVM_REG_ARM_CORE_REG(regs.sp) => &mut (*vcpu).arch.ctxt.regs.sp as *mut _ as *mut _,
        KVM_REG_ARM_CORE_REG(regs.pc) => &mut (*vcpu).arch.ctxt.regs.pc as *mut _ as *mut _,
        KVM_REG_ARM_CORE_REG(regs.pstate) => &mut (*vcpu).arch.ctxt.regs.pstate as *mut _ as *mut _,
        KVM_REG_ARM_CORE_REG(sp_el1) => __ctxt_sys_reg(&mut (*vcpu).arch.ctxt, SP_EL1),
        KVM_REG_ARM_CORE_REG(elr_el1) => __ctxt_sys_reg(&mut (*vcpu).arch.ctxt, ELR_EL1),
        KVM_REG_ARM_CORE_REG(spsr[KVM_SPSR_EL1]) => __ctxt_sys_reg(&mut (*vcpu).arch.ctxt, SPSR_EL1),
        KVM_REG_ARM_CORE_REG(spsr[KVM_SPSR_ABT]) => &mut (*vcpu).arch.ctxt.spsr_abt as *mut _ as *mut _,
        KVM_REG_ARM_CORE_REG(spsr[KVM_SPSR_UND]) => &mut (*vcpu).arch.ctxt.spsr_und as *mut _ as *mut _,
        KVM_REG_ARM_CORE_REG(spsr[KVM_SPSR_IRQ]) => &mut (*vcpu).arch.ctxt.spsr_irq as *mut _ as *mut _,
        KVM_REG_ARM_CORE_REG(spsr[KVM_SPSR_FIQ]) => &mut (*vcpu).arch.ctxt.spsr_fiq as *mut _ as *mut _,
        KVM_REG_ARM_CORE_REG(fp_regs.vregs[0]) ..= KVM_REG_ARM_CORE_REG(fp_regs.vregs[31]) => {
            let n = (off - KVM_REG_ARM_CORE_REG(fp_regs.vregs[0])) / 4;
            (&mut (*vcpu).arch.ctxt.fp_regs.vregs[n as usize]) as *mut _ as *mut _
        }
        KVM_REG_ARM_CORE_REG(fp_regs.fpsr) => &mut (*vcpu).arch.ctxt.fp_regs.fpsr as *mut _ as *mut _,
        KVM_REG_ARM_CORE_REG(fp_regs.fpcr) => &mut (*vcpu).arch.ctxt.fp_regs.fpcr as *mut _ as *mut _,
        _ => core::ptr::null_mut(),
    }
}

unsafe fn get_core_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32 {
    let off = core_reg_offset_from_id((*reg).id);
    let nr = (core::mem::size_of::<kvm_regs>() / core::mem::size_of::<u32>()) as u64;
    if off >= nr || off + KVM_REG_SIZE((*reg).id) / 4 >= nr { return -ENOENT; }
    let addr = core_reg_addr(vcpu, reg); if addr.is_null() { return -EINVAL; }
    if copy_to_user((*reg).addr as *mut u32, addr, KVM_REG_SIZE((*reg).id) as usize) != 0 { -EFAULT } else { 0 }
}

unsafe fn set_core_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32 {
    let off = core_reg_offset_from_id((*reg).id);
    let nr = (core::mem::size_of::<kvm_regs>() / 4) as u64;
    if off >= nr || off + KVM_REG_SIZE((*reg).id) / 4 >= nr { return -ENOENT; }
    let addr = core_reg_addr(vcpu, reg); if addr.is_null() || KVM_REG_SIZE((*reg).id) > 16 { return -EINVAL; }
    let mut tmp = [0u8; 16];
    if copy_from_user(tmp.as_mut_ptr() as *mut _, (*reg).addr as *const _, KVM_REG_SIZE((*reg).id) as usize) != 0 { return -EFAULT; }
    if off == KVM_REG_ARM_CORE_REG(regs.pstate) {
        let mode = *(tmp.as_ptr() as *const u64) & PSR_AA32_MODE_MASK;
        match mode {
            PSR_AA32_MODE_USR if !kvm_supports_32bit_el0() => return -EINVAL,
            PSR_AA32_MODE_FIQ | PSR_AA32_MODE_IRQ | PSR_AA32_MODE_SVC | PSR_AA32_MODE_ABT |
            PSR_AA32_MODE_UND | PSR_AA32_MODE_SYS if !vcpu_el1_is_32bit(vcpu) => return -EINVAL,
            PSR_MODE_EL2h | PSR_MODE_EL2t if !vcpu_has_nv(vcpu) => return -EINVAL,
            PSR_MODE_EL0t | PSR_MODE_EL1t | PSR_MODE_EL1h if vcpu_el1_is_32bit(vcpu) => return -EINVAL,
            _ => {}
        }
    }
    core::ptr::copy_nonoverlapping(tmp.as_ptr(), addr as *mut u8, KVM_REG_SIZE((*reg).id) as usize);
    0
}

unsafe fn get_sve_vls(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32 {
    if !vcpu_has_sve(vcpu) { return -ENOENT; }
    let mut vqs = [0u64; KVM_ARM64_SVE_VLS_WORDS];
    let max = vcpu_sve_max_vq(vcpu);
    for vq in SVE_VQ_MIN..=max { if sve_vq_available(vq) { vqs[vq_word(vq)] |= vq_mask(vq); } }
    if copy_to_user((*reg).addr as *mut _, vqs.as_ptr() as *const _, core::mem::size_of_val(&vqs)) != 0 { -EFAULT } else { 0 }
}

const fn vq_word(vq: usize) -> usize { (vq - SVE_VQ_MIN) / 64 }
const fn vq_mask(vq: usize) -> u64 { 1u64 << ((vq - SVE_VQ_MIN) % 64) }

pub unsafe fn kvm_arch_vcpu_ioctl_get_regs(_: *mut kvm_vcpu, _: *mut kvm_regs) -> i32 { -EINVAL }
pub unsafe fn kvm_arch_vcpu_ioctl_set_regs(_: *mut kvm_vcpu, _: *mut kvm_regs) -> i32 { -EINVAL }
pub unsafe fn kvm_arch_vcpu_ioctl_get_sregs(_: *mut kvm_vcpu, _: *mut kvm_sregs) -> i32 { -EINVAL }
pub unsafe fn kvm_arch_vcpu_ioctl_set_sregs(_: *mut kvm_vcpu, _: *mut kvm_sregs) -> i32 { -EINVAL }
pub unsafe fn kvm_arch_vcpu_ioctl_get_fpu(_: *mut kvm_vcpu, _: *mut kvm_fpu) -> i32 { -EINVAL }
pub unsafe fn kvm_arch_vcpu_ioctl_set_fpu(_: *mut kvm_vcpu, _: *mut kvm_fpu) -> i32 { -EINVAL }
pub unsafe fn kvm_arch_vcpu_ioctl_translate(_: *mut kvm_vcpu, _: *mut kvm_translation) -> i32 { -EINVAL }

pub unsafe fn kvm_arm_get_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32 {
    if (((*reg).id & !KVM_REG_SIZE_MASK) >> 32) != (KVM_REG_ARM64 >> 32) { return -EINVAL; }
    match (*reg).id & KVM_REG_ARM_COPROC_MASK {
        KVM_REG_ARM_CORE => get_core_reg(vcpu, reg),
        KVM_REG_ARM_FW | KVM_REG_ARM_FW_FEAT_BMAP => kvm_arm_get_fw_reg(vcpu, reg),
        KVM_REG_ARM64_SVE => get_sve_reg(vcpu, reg),
        _ => kvm_arm_sys_reg_get_reg(vcpu, reg),
    }
}
pub unsafe fn kvm_arm_set_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32 {
    if (((*reg).id & !KVM_REG_SIZE_MASK) >> 32) != (KVM_REG_ARM64 >> 32) { return -EINVAL; }
    match (*reg).id & KVM_REG_ARM_COPROC_MASK {
        KVM_REG_ARM_CORE => set_core_reg(vcpu, reg),
        KVM_REG_ARM_FW | KVM_REG_ARM_FW_FEAT_BMAP => kvm_arm_set_fw_reg(vcpu, reg),
        KVM_REG_ARM64_SVE => set_sve_reg(vcpu, reg),
        _ => kvm_arm_sys_reg_set_reg(vcpu, reg),
    }
}

pub unsafe extern "C" fn kvm_target_cpu() -> u32 {
    match (read_cpuid_implementor(), read_cpuid_part_number()) {
        (ARM_CPU_IMP_ARM, ARM_CPU_PART_AEM_V8) => KVM_ARM_TARGET_AEM_V8,
        (ARM_CPU_IMP_ARM, ARM_CPU_PART_FOUNDATION) => KVM_ARM_TARGET_FOUNDATION_V8,
        (ARM_CPU_IMP_ARM, ARM_CPU_PART_CORTEX_A53) => KVM_ARM_TARGET_CORTEX_A53,
        (ARM_CPU_IMP_ARM, ARM_CPU_PART_CORTEX_A57) => KVM_ARM_TARGET_CORTEX_A57,
        (ARM_CPU_IMP_APM, APM_CPU_PART_XGENE) => KVM_ARM_TARGET_XGENE_POTENZA,
        _ => KVM_ARM_TARGET_GENERIC_V8,
    }
}

// The remaining architecture entry points retain the source interfaces; their
// implementations are supplied by the surrounding KVM translation units.
extern "C" {
    fn get_sve_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32;
    fn set_sve_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32;
    fn kvm_arm_get_fw_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32;
    fn kvm_arm_set_fw_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32;
    fn kvm_arm_sys_reg_get_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32;
    fn kvm_arm_sys_reg_set_reg(vcpu: *mut kvm_vcpu, reg: *const kvm_one_reg) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
