// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level Rust translation of vgic.c. External kernel definitions are
 * supplied by the surrounding kernel translation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut kvm_vgic_global_state: vgic_global;
    fn vgic_is_v5(kvm: *mut kvm) -> bool; fn irq_is_lpi(kvm: *mut kvm, id: u32) -> bool;
    fn vgic_try_get_irq_ref(i: *mut vgic_irq) -> bool; fn vgic_get_irq_ref(i: *mut vgic_irq);
    fn vgic_put_irq(k: *mut kvm, i: *mut vgic_irq); fn vgic_valid_spi(k: *mut kvm, id: u32) -> bool;
    fn vgic_is_v3(k: *mut kvm) -> bool; fn vgic_initialized(k: *mut kvm) -> bool;
    fn irq_is_private(k: *mut kvm, id: u32) -> bool; fn irq_is_ppi(k: *mut kvm, id: u32) -> bool;
    fn irq_is_pending(i: *mut vgic_irq) -> bool; fn vgic_get_vmcr(v: *mut kvm_vcpu, m: *mut vgic_vmcr);
    fn vgic_v5_get_hwirq_id(id: u32) -> u32; fn __irq_is_sgi(t: kvm_device_type, id: u32) -> bool;
    fn __irq_is_ppi(t: kvm_device_type, id: u32) -> bool;
}

#[repr(C)] pub struct vgic_global { pub gicv3_cpuif: bool, pub nr_lr: i32, pub has_gicv4: bool, pub r#type: i32 }
#[repr(C)] pub struct kvm { pub arch: kvm_arch }
#[repr(C)] pub struct kvm_arch { pub vgic: vgic_dist }
#[repr(C)] pub struct vgic_dist { pub nr_spis: u32, pub vgic_model: kvm_device_type, pub enabled: bool, pub lpi_xa: xarray, pub spis: *mut vgic_irq, pub active_spis: i32 }
#[repr(C)] pub struct kvm_vcpu { pub kvm: *mut kvm, pub arch: vcpu_arch, pub vcpu_id: u32, pub vcpu_idx: u32 }
#[repr(C)] pub struct vcpu_arch { pub vgic_cpu: vgic_cpu }
#[repr(C)] pub struct vgic_cpu { pub private_irqs: *mut vgic_irq, pub ap_list_head: list_head, pub ap_list_lock: raw_spinlock, pub vgic_v2: vgic_v2, pub vgic_v3: vgic_v3 }
#[repr(C)] pub struct vgic_irq { pub intid:u32, pub refcount:i32, pub hw:bool, pub host_irq:u32, pub hwintid:u64, pub active:bool, pub enabled:bool, pub pending_latch:bool, pub line_level:bool, pub priority:u8, pub group:bool, pub vcpu:*mut kvm_vcpu, pub target_vcpu:*mut kvm_vcpu, pub owner:*mut c_void, pub config:i32, pub irq_lock:raw_spinlock, pub ap_list:list_head, pub ops:*const irq_ops }
#[repr(C)] pub struct irq_ops { pub get_input_level: Option<unsafe extern "C" fn(u32)->bool>, pub queue_irq_unlock: Option<unsafe extern "C" fn(*mut kvm,*mut vgic_irq,usize)->bool>, pub set_direct_injection: Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut vgic_irq,bool)> }
#[repr(C)] pub struct vgic_vmcr { pub grpen0:bool, pub grpen1:bool, pub pmr:u8 }
#[repr(C)] pub struct ap_list_summary { pub nr_pend:i32, pub nr_act:i32, pub nr_sgi:i32 }
#[repr(C)] pub struct vgic_v2 { pub used_lrs:i32 } #[repr(C)] pub struct vgic_v3 { pub used_lrs:i32 }
#[repr(C)] pub struct xarray; #[repr(C)] pub struct list_head; #[repr(C)] pub struct raw_spinlock;
pub type kvm_device_type = i32;

#[no_mangle] pub unsafe extern "C" fn vgic_get_vcpu_irq(vcpu:*mut kvm_vcpu, mut intid:u32)->*mut vgic_irq {
    if vcpu.is_null() { return core::ptr::null_mut(); }
    let t=(*vcpu).kvm.as_ref().unwrap().arch.vgic.vgic_model;
    if __irq_is_sgi(t,intid)||__irq_is_ppi(t,intid) { if t==5 { intid=vgic_v5_get_hwirq_id(intid); if intid>=32{return core::ptr::null_mut();} } else if intid>=32{return core::ptr::null_mut();} return (*vcpu).arch.vgic_cpu.private_irqs.add(intid as usize); }
    vgic_get_irq((*vcpu).kvm,intid)
}

#[no_mangle] pub unsafe extern "C" fn vgic_get_irq(k:*mut kvm, intid:u32)->*mut vgic_irq {
    if vgic_is_v5(k) { return core::ptr::null_mut(); }
    let d=&mut (*k).arch.vgic; if intid>=32 && intid<d.nr_spis+32 { return d.spis.add((intid-32) as usize); }
    core::ptr::null_mut()
}

#[no_mangle] pub unsafe extern "C" fn vgic_target_oracle(i:*mut vgic_irq)->*mut kvm_vcpu {
    if (*i).active { return if !(*i).vcpu.is_null(){(*i).vcpu}else{(*i).target_vcpu}; }
    if (*i).enabled && irq_is_pending(i) { return (*i).target_vcpu; } core::ptr::null_mut()
}

#[no_mangle] pub unsafe extern "C" fn kvm_vgic_inject_irq(k:*mut kvm, v:*mut kvm_vcpu, id:u32, level:bool, owner:*mut c_void)->i32 {
    if !vgic_initialized(k) { return 0; } if v.is_null() && irq_is_private(k,id) { return -22; }
    let i=if irq_is_private(k,id){vgic_get_vcpu_irq(v,id)}else{vgic_get_irq(k,id)}; if i.is_null(){return -22;}
    if (*i).owner!=owner { vgic_put_irq(k,i); return 0; } if (*i).config==1 {(*i).line_level=level}else{(*i).pending_latch=true;} vgic_put_irq(k,i); 0
}

// The remaining entry points retain the C kernel's lock/list operations and are
// declared here so their implementations can be provided by dependent units.
extern "C" { fn vgic_flush_pending_lpis(v:*mut kvm_vcpu); fn kvm_vgic_sync_hwstate(v:*mut kvm_vcpu); fn kvm_vgic_flush_hwstate(v:*mut kvm_vcpu); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
