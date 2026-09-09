/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2023 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation.

/* Controlled by 0x5 guest estat */
pub const CPU_SIP0: usize = 1usize << INT_SWI0;
pub const CPU_SIP1: usize = 1usize << INT_SWI1;
pub const CPU_HWI0: usize = 1usize << INT_HWI0;
pub const CPU_HWI1: usize = 1usize << INT_HWI1;
pub const CPU_HWI2: usize = 1usize << INT_HWI2;
pub const CPU_HWI3: usize = 1usize << INT_HWI3;
pub const CPU_HWI4: usize = 1usize << INT_HWI4;
pub const CPU_HWI5: usize = 1usize << INT_HWI5;
pub const CPU_HWI6: usize = 1usize << INT_HWI6;
pub const CPU_HWI7: usize = 1usize << INT_HWI7;
pub const CPU_PMU: usize = 1usize << INT_PCOV;
pub const CPU_TIMER: usize = 1usize << INT_TI;
pub const CPU_IPI: usize = 1usize << INT_IPI;
pub const CPU_AVEC: usize = 1usize << INT_AVEC;
pub const KVM_ESTAT_INTI_MASK: usize = CPU_SIP0 | CPU_SIP1 | CPU_PMU | CPU_TIMER | CPU_IPI | CPU_AVEC;
pub const KVM_ESTAT_EXTI_MASK: usize = CPU_HWI0 | CPU_HWI1 | CPU_HWI2 | CPU_HWI3 | CPU_HWI4 | CPU_HWI5 | CPU_HWI6 | CPU_HWI7;

/* Controlled by 0x52 guest exception VIP aligned to estat bit 5~12 */
pub const VIP_DELTA: usize = INT_HWI0 - CSR_GINTC_VIP_SHIFT;
pub const CPU_IP0: usize = 1usize << (INT_HWI0 - VIP_DELTA);
pub const CPU_IP1: usize = 1usize << (INT_HWI1 - VIP_DELTA);
pub const CPU_IP2: usize = 1usize << (INT_HWI2 - VIP_DELTA);
pub const CPU_IP3: usize = 1usize << (INT_HWI3 - VIP_DELTA);
pub const CPU_IP4: usize = 1usize << (INT_HWI4 - VIP_DELTA);
pub const CPU_IP5: usize = 1usize << (INT_HWI5 - VIP_DELTA);
pub const CPU_IP6: usize = 1usize << (INT_HWI6 - VIP_DELTA);
pub const CPU_IP7: usize = 1usize << (INT_HWI7 - VIP_DELTA);
pub const KVM_GINTC_IRQ_MASK: usize = CPU_IP0 | CPU_IP1 | CPU_IP2 | CPU_IP3 | CPU_IP4 | CPU_IP5 | CPU_IP6 | CPU_IP7;

pub const MNSEC_PER_SEC: usize = NSEC_PER_SEC >> 20;

/* KVM_IRQ_LINE irq field index values */
pub const KVM_LOONGSON_IRQ_TYPE_SHIFT: u32 = 24;
pub const KVM_LOONGSON_IRQ_TYPE_MASK: u32 = 0xff;
pub const KVM_LOONGSON_IRQ_VCPU_SHIFT: u32 = 16;
pub const KVM_LOONGSON_IRQ_VCPU_MASK: u32 = 0xff;
pub const KVM_LOONGSON_IRQ_NUM_SHIFT: u32 = 0;
pub const KVM_LOONGSON_IRQ_NUM_MASK: u32 = 0xffff;

pub type LarchInst = loongarch_instruction;
pub type ExitHandleFn = unsafe extern "C" fn(*mut kvm_vcpu, i32) -> i32;

#[repr(C)] pub union loongarch_instruction { _opaque: u64 }
#[repr(C)] pub struct kvm_vcpu { _opaque: [u8; 0] }
#[repr(C)] pub struct kvm_run { _opaque: [u8; 0] }
#[repr(C)] pub struct loongarch_fpu { _opaque: [u8; 0] }
#[repr(C)] pub struct kvm_interrupt { _opaque: [u8; 0] }
#[repr(C)] pub struct kvm { _opaque: [u8; 0] }

extern "C" {
    pub fn kvm_emu_mmio_read(vcpu: *mut kvm_vcpu, inst: LarchInst) -> i32;
    pub fn kvm_emu_mmio_write(vcpu: *mut kvm_vcpu, inst: LarchInst) -> i32;
    pub fn kvm_complete_mmio_read(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> i32;
    pub fn kvm_complete_iocsr_read(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> i32;
    pub fn kvm_complete_user_service(vcpu: *mut kvm_vcpu, run: *mut kvm_run) -> i32;
    pub fn kvm_emu_idle(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_pending_timer(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_handle_fault(vcpu: *mut kvm_vcpu, fault: i32) -> i32;
    pub fn kvm_deliver_intr(vcpu: *mut kvm_vcpu);
    pub fn kvm_deliver_exception(vcpu: *mut kvm_vcpu);
    pub fn kvm_own_fpu(vcpu: *mut kvm_vcpu);
    pub fn kvm_lose_fpu(vcpu: *mut kvm_vcpu);
    pub fn kvm_save_fpu(fpu: *mut loongarch_fpu);
    pub fn kvm_restore_fpu(fpu: *mut loongarch_fpu);
    pub fn kvm_restore_fcsr(fpu: *mut loongarch_fpu);
    pub fn kvm_init_timer(vcpu: *mut kvm_vcpu, hz: libc::c_ulong);
    pub fn kvm_save_timer(vcpu: *mut kvm_vcpu);
    pub fn kvm_restore_timer(vcpu: *mut kvm_vcpu);
    pub fn kvm_vcpu_ioctl_interrupt(vcpu: *mut kvm_vcpu, irq: *mut kvm_interrupt) -> i32;
    pub fn kvm_get_vcpu_by_cpuid(kvm: *mut kvm, cpuid: i32) -> *mut kvm_vcpu;
}

// CONFIG_CPU_HAS_LSX / CONFIG_CPU_HAS_LASX / CONFIG_CPU_HAS_LBT select the
// declarations below at build time; fallback inline definitions are retained.
#[cfg(CONFIG_CPU_HAS_LSX)] extern "C" { pub fn kvm_own_lsx(vcpu: *mut kvm_vcpu) -> i32; pub fn kvm_save_lsx(fpu: *mut loongarch_fpu); pub fn kvm_restore_lsx(fpu: *mut loongarch_fpu); }
#[cfg(not(CONFIG_CPU_HAS_LSX))] pub unsafe fn kvm_own_lsx(_: *mut kvm_vcpu) -> i32 { -22 }
#[cfg(not(CONFIG_CPU_HAS_LSX))] pub unsafe fn kvm_save_lsx(_: *mut loongarch_fpu) {}
#[cfg(not(CONFIG_CPU_HAS_LSX))] pub unsafe fn kvm_restore_lsx(_: *mut loongarch_fpu) {}
#[cfg(CONFIG_CPU_HAS_LASX)] extern "C" { pub fn kvm_own_lasx(vcpu: *mut kvm_vcpu) -> i32; pub fn kvm_save_lasx(fpu: *mut loongarch_fpu); pub fn kvm_restore_lasx(fpu: *mut loongarch_fpu); }
#[cfg(not(CONFIG_CPU_HAS_LASX))] pub unsafe fn kvm_own_lasx(_: *mut kvm_vcpu) -> i32 { -22 }
#[cfg(not(CONFIG_CPU_HAS_LASX))] pub unsafe fn kvm_save_lasx(_: *mut loongarch_fpu) {}
#[cfg(not(CONFIG_CPU_HAS_LASX))] pub unsafe fn kvm_restore_lasx(_: *mut loongarch_fpu) {}
#[cfg(CONFIG_CPU_HAS_LBT)] extern "C" { pub fn kvm_own_lbt(vcpu: *mut kvm_vcpu) -> i32; }
#[cfg(not(CONFIG_CPU_HAS_LBT))] pub unsafe fn kvm_own_lbt(_: *mut kvm_vcpu) -> i32 { -22 }

// Loongarch KVM guest interrupt handling.
// The following inline operations depend on the externally supplied kvm_vcpu layout.
pub unsafe fn kvm_queue_irq(vcpu: *mut kvm_vcpu, irq: u32) { set_bit(irq, &mut (*vcpu).arch.irq_pending); clear_bit(irq, &mut (*vcpu).arch.irq_clear); }
pub unsafe fn kvm_dequeue_irq(vcpu: *mut kvm_vcpu, irq: u32) { clear_bit(irq, &mut (*vcpu).arch.irq_pending); set_bit(irq, &mut (*vcpu).arch.irq_clear); }
pub unsafe fn kvm_queue_exception(vcpu: *mut kvm_vcpu, code: u32, subcode: u32) -> i32 {
    if !(*vcpu).arch.exception_pending { set_bit(code, &mut (*vcpu).arch.exception_pending); (*vcpu).arch.esubcode = subcode; 0 } else { -1 }
}
pub unsafe fn kvm_read_reg(vcpu: *mut kvm_vcpu, num: i32) -> libc::c_ulong { (*vcpu).arch.gprs[num as usize] }
pub unsafe fn kvm_write_reg(vcpu: *mut kvm_vcpu, num: i32, val: libc::c_ulong) { (*vcpu).arch.gprs[num as usize] = val; }
pub unsafe fn kvm_pvtime_supported() -> bool { sched_info_on() != 0 }
pub unsafe fn kvm_guest_has_pv_feature(vcpu: *mut kvm_vcpu, feature: u32) -> bool { (*vcpu).kvm.arch.pv_features & (1usize << feature) != 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
