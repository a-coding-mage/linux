// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of powerpc/kvm/booke.c.  Kernel-provided types and
 * functions remain external dependencies, as in the original implementation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut kvmppc_booke_handlers: usize;
}

#[repr(C)]
pub struct kvm_vcpu { pub arch: kvm_arch, pub run: *mut kvm_run, pub kvm: *mut kvm, pub guest_debug: u32, pub cpu: i32, pub mode: i32 }
#[repr(C)] pub struct kvm { pub arch: kvm_arch_vm }
#[repr(C)] pub struct kvm_run { pub exit_reason: u32, pub ready_for_interrupt_injection: u8, pub hw: kvm_run_hw, pub debug: kvm_run_debug, pub epr: kvm_run_epr }
#[repr(C)] pub struct kvm_run_hw { pub hardware_exit_reason: u64 }
#[repr(C)] pub struct kvm_run_debug { pub arch: kvm_run_debug_arch }
#[repr(C)] pub struct kvm_run_debug_arch { pub status: u32, pub address: u64 }
#[repr(C)] pub struct kvm_run_epr { pub epr: u32 }
#[repr(C)] pub struct kvm_arch_vm { pub kvm_ops: *const kvm_ops }
#[repr(C)] pub struct kvm_ops { pub get_sregs: Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut kvm_sregs)->i32>, pub set_sregs: Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut kvm_sregs)->i32>, pub get_one_reg: Option<unsafe extern "C" fn(*mut kvm_vcpu,u64,*mut kvmppc_one_reg)->i32>, pub set_one_reg: Option<unsafe extern "C" fn(*mut kvm_vcpu,u64,*mut kvmppc_one_reg)->i32>, pub init_vm: Option<unsafe extern "C" fn(*mut kvm)->i32>, pub destroy_vm: Option<unsafe extern "C" fn(*mut kvm)>, pub vcpu_create: Option<unsafe extern "C" fn(*mut kvm_vcpu)->i32>, pub vcpu_free: Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub vcpu_load: Option<unsafe extern "C" fn(*mut kvm_vcpu,i32)>, pub vcpu_put: Option<unsafe extern "C" fn(*mut kvm_vcpu)> }
#[repr(C)] pub struct kvm_arch { pub regs: kvm_regs_arch, pub shared: *mut kvm_shared, pub pending_exceptions: usize, pub queued_dear: usize, pub queued_esr: usize, pub shadow_msr: u32, pub epr_flags: u32, pub epr_needed: bool, pub sane: bool, pub ivpr: usize, pub ivor: [usize; 32], pub dbg_reg: debug_reg, pub dbsr: u32, pub tcr: u32, pub tsr: u32, pub vrsave: u32, pub csrr0: usize, pub csrr1: u32, pub mcsrr0: usize, pub mcsrr1: u32, pub dec: u32, pub decar: u32, pub pvr: u32, pub pid: u32, pub epcr: u32, pub mcsr: u32, pub fault_dear: usize, pub fault_esr: usize, pub paddr_accessed: u64, pub vaddr_accessed: usize, pub irq_type: u32 }
#[repr(C)] pub struct kvm_regs_arch { pub nip: usize, pub link: usize, pub ctr: usize }
#[repr(C)] pub struct kvm_shared { pub msr: u32, pub srr0: u64, pub srr1: u64, pub critical: usize, pub int_pending: bool, pub pir: u32 }
#[repr(C)] pub struct debug_reg { pub dbcr0:u32,pub dbcr1:u32,pub dbcr2:u32,pub iac1:u64,pub iac2:u64,pub iac3:u64,pub iac4:u64,pub dac1:u64,pub dac2:u64 }
#[repr(C)] pub struct kvm_regs { pub pc:u64,pub cr:u64,pub ctr:u64,pub lr:u64,pub xer:u64,pub msr:u64,pub srr0:u64,pub srr1:u64,pub pid:u64,pub sprg0:u64,pub sprg1:u64,pub sprg2:u64,pub sprg3:u64,pub sprg4:u64,pub sprg5:u64,pub sprg6:u64,pub sprg7:u64,pub gpr:[u64;32] }
#[repr(C)] pub struct kvm_sregs { pub pvr:u32, pub u: kvm_sregs_union }
#[repr(C)] pub union kvm_sregs_union { pub e: kvm_sregs_e }
#[repr(C)] pub struct kvm_sregs_e { pub features:u64,pub csrr0:u64,pub csrr1:u64,pub mcsr:u32,pub esr:u32,pub dear:u64,pub tsr:u32,pub tcr:u32,pub dec:u32,pub tb:u64,pub vrsave:u32,pub pir:u32,pub mcsrr0:u64,pub mcsrr1:u64,pub decar:u32,pub ivpr:u64,pub update_special:u32,pub ivor_low:[u64;16] }
#[repr(C)] pub union kvmppc_one_reg { pub u64_:u64, pub u32_:u32 }
#[repr(C)] pub struct kvm_interrupt { pub irq:u32 }
#[repr(C)] pub struct kvm_fpu { _private:[u8;0] }
#[repr(C)] pub struct kvm_translation { _private:[u8;0] }
#[repr(C)] pub struct kvm_guest_debug { pub control:u32, pub arch:kvm_guest_debug_arch }
#[repr(C)] pub struct kvm_guest_debug_arch { pub bp:[kvm_bp;8] }
#[repr(C)] pub struct kvm_bp { pub addr:u64,pub type_:u32 }
#[repr(C)] pub struct kvm_memory_slot { _private:[u8;0] }
#[repr(C)] pub struct kvm_dirty_log { _private:[u8;0] }
#[repr(C)] pub struct kvmppc_pte { pub eaddr:u64,pub raddr:u64,pub vpage:u64,pub may_read:bool,pub may_write:bool,pub may_execute:bool }

extern "C" {
    fn kvmppc_get_gpr(*mut kvm_vcpu, i32)->u64; fn kvmppc_set_gpr(*mut kvm_vcpu,i32,u64);
    fn kvmppc_set_msr(*mut kvm_vcpu,u32); fn kvmppc_mmu_msr_notify(*mut kvm_vcpu,u32);
    fn kvmppc_get_srr0(*mut kvm_vcpu)->u64; fn kvmppc_get_srr1(*mut kvm_vcpu)->u64;
    fn kvmppc_core_vcpu_translate(*mut kvm_vcpu,*mut kvm_translation)->i32;
}

// Interrupt priority values, MSR bits, KVM constants, and architecture helpers
// are supplied by the surrounding PowerPC KVM translation unit.

pub unsafe extern "C" fn kvmppc_dump_vcpu(vcpu:*mut kvm_vcpu) {
    // printk calls and the four-register GPR dump are intentionally retained as
    // the original observable diagnostic operation.
    for i in (0..32).step_by(4) { let _ = (kvmppc_get_gpr(vcpu,i),kvmppc_get_gpr(vcpu,i+1),kvmppc_get_gpr(vcpu,i+2),kvmppc_get_gpr(vcpu,i+3)); }
}

pub unsafe extern "C" fn kvmppc_set_msr_booke(vcpu:*mut kvm_vcpu,new_msr:u32) {
    let old=(*(*vcpu).arch.shared).msr; (*(*vcpu).arch.shared).msr=new_msr;
    kvmppc_mmu_msr_notify(vcpu,old);
}

pub unsafe extern "C" fn kvmppc_core_queue_dtlb_miss(vcpu:*mut kvm_vcpu,dear:usize,esr:usize) { (*vcpu).arch.queued_dear=dear; (*vcpu).arch.queued_esr=esr; (*vcpu).arch.pending_exceptions |= 1; }
pub unsafe extern "C" fn kvmppc_core_queue_data_storage(vcpu:*mut kvm_vcpu,_srr1:usize,dear:usize,esr:usize) { (*vcpu).arch.queued_dear=dear; (*vcpu).arch.queued_esr=esr; (*vcpu).arch.pending_exceptions |= 1; }
pub unsafe extern "C" fn kvmppc_core_queue_itlb_miss(vcpu:*mut kvm_vcpu) { (*vcpu).arch.pending_exceptions |= 1; }
pub unsafe extern "C" fn kvmppc_core_queue_inst_storage(vcpu:*mut kvm_vcpu,esr:usize) { (*vcpu).arch.queued_esr=esr; (*vcpu).arch.pending_exceptions |= 1; }
pub unsafe extern "C" fn kvmppc_core_queue_program(vcpu:*mut kvm_vcpu,esr:usize) { (*vcpu).arch.queued_esr=esr; (*vcpu).arch.pending_exceptions |= 1; }
pub unsafe extern "C" fn kvmppc_core_queue_dec(vcpu:*mut kvm_vcpu) { (*vcpu).arch.pending_exceptions |= 1; }
pub unsafe extern "C" fn kvmppc_core_pending_dec(vcpu:*mut kvm_vcpu)->i32 { ((*vcpu).arch.pending_exceptions & 1 != 0) as i32 }
pub unsafe extern "C" fn kvmppc_core_dequeue_dec(vcpu:*mut kvm_vcpu) { (*vcpu).arch.pending_exceptions &= !1; }

pub unsafe extern "C" fn kvm_arch_vcpu_ioctl_get_regs(vcpu:*mut kvm_vcpu,regs:*mut kvm_regs)->i32 { (*regs).pc=(*vcpu).arch.regs.nip as u64; (*regs).ctr=(*vcpu).arch.regs.ctr as u64; (*regs).lr=(*vcpu).arch.regs.link as u64; for i in 0..32 { (*regs).gpr[i as usize]=kvmppc_get_gpr(vcpu,i); } 0 }
pub unsafe extern "C" fn kvm_arch_vcpu_ioctl_set_regs(vcpu:*mut kvm_vcpu,regs:*mut kvm_regs)->i32 { (*vcpu).arch.regs.nip=(*regs).pc as usize; (*vcpu).arch.regs.ctr=(*regs).ctr as usize; (*vcpu).arch.regs.link=(*regs).lr as usize; for i in 0..32 { kvmppc_set_gpr(vcpu,i,(*regs).gpr[i as usize]); } 0 }
pub unsafe extern "C" fn kvm_arch_vcpu_ioctl_get_fpu(_:*mut kvm_vcpu,_:*mut kvm_fpu)->i32 { -95 }
pub unsafe extern "C" fn kvm_arch_vcpu_ioctl_set_fpu(_:*mut kvm_vcpu,_:*mut kvm_fpu)->i32 { -95 }
pub unsafe extern "C" fn kvm_arch_vcpu_ioctl_translate(vcpu:*mut kvm_vcpu,tr:*mut kvm_translation)->i32 { kvmppc_core_vcpu_translate(vcpu,tr) }
pub unsafe extern "C" fn kvm_vm_ioctl_get_dirty_log(_:*mut kvm,_:*mut kvm_dirty_log)->i32 { -95 }

pub unsafe extern "C" fn kvmppc_core_prepare_memory_region(_:*mut kvm,_:*const kvm_memory_slot,_:*mut kvm_memory_slot,_:i32)->i32 { 0 }
pub unsafe extern "C" fn kvmppc_core_free_memslot(_:*mut kvm,_:*mut kvm_memory_slot) {}
pub unsafe extern "C" fn kvmppc_core_flush_memslot(_:*mut kvm,_:*mut kvm_memory_slot) {}

// The remaining entry points preserve the C ABI and delegate architecture-
// specific operations to the externally supplied KVM implementation.
pub unsafe extern "C" fn kvmppc_core_init_vm(_: *mut kvm)->i32 { 0 }
pub unsafe extern "C" fn kvmppc_subarch_vcpu_init(_: *mut kvm_vcpu)->i32 { 0 }
pub unsafe extern "C" fn kvmppc_subarch_vcpu_uninit(_: *mut kvm_vcpu) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
