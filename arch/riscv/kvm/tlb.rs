// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022 Ventana Micro Systems Inc.
 */

// Linux and RISC-V headers from the original implementation provide the
// types, constants, macros, and external functions referenced below.

#[inline]
unsafe fn has_svinval() -> bool {
    riscv_has_extension_unlikely(RISCV_ISA_EXT_SVINVAL)
}

pub unsafe fn kvm_riscv_local_hfence_gvma_vmid_gpa(
    vmid: c_ulong, gpa: gpa_t, gpsz: gpa_t, order: c_ulong,
) {
    let step = (1 as gpa_t).wrapping_shl(order as u32);
    let end = match gpa.checked_add(gpsz) {
        Some(v) => v,
        None => { kvm_riscv_local_hfence_gvma_vmid_all(vmid); return; }
    };
    if PTRS_PER_PTE < (gpsz >> order) {
        kvm_riscv_local_hfence_gvma_vmid_all(vmid); return;
    }
    let mut pos = gpa;
    if has_svinval() {
        core::arch::asm!("sfence.w.inval", options(nostack, preserves_flags));
        while pos < end {
            core::arch::asm!("hinval.gvma {0}, {1}", in(reg) (pos >> 2), in(reg) vmid,
                options(nostack, preserves_flags));
            if end - pos <= step { break; }
            pos = pos.wrapping_add(step);
        }
        core::arch::asm!("sfence.inval.ir", options(nostack, preserves_flags));
    } else {
        while pos < end {
            core::arch::asm!("hfence.gvma {0}, {1}", in(reg) (pos >> 2), in(reg) vmid,
                options(nostack, preserves_flags));
            if end - pos <= step { break; }
            pos = pos.wrapping_add(step);
        }
    }
}

pub unsafe fn kvm_riscv_local_hfence_gvma_vmid_all(vmid: c_ulong) {
    core::arch::asm!("hfence.gvma zero, {0}", in(reg) vmid, options(nostack, preserves_flags));
}

pub unsafe fn kvm_riscv_local_hfence_gvma_gpa(gpa: gpa_t, gpsz: gpa_t, order: c_ulong) {
    let step = (1 as gpa_t).wrapping_shl(order as u32);
    let end = match gpa.checked_add(gpsz) {
        Some(v) => v,
        None => { kvm_riscv_local_hfence_gvma_all(); return; }
    };
    if PTRS_PER_PTE < (gpsz >> order) { kvm_riscv_local_hfence_gvma_all(); return; }
    let mut pos = gpa;
    if has_svinval() {
        core::arch::asm!("sfence.w.inval", options(nostack, preserves_flags));
        while pos < end {
            core::arch::asm!("hinval.gvma {0}, zero", in(reg) (pos >> 2), options(nostack, preserves_flags));
            if end - pos <= step { break; }
            pos = pos.wrapping_add(step);
        }
        core::arch::asm!("sfence.inval.ir", options(nostack, preserves_flags));
    } else {
        while pos < end {
            core::arch::asm!("hfence.gvma {0}, zero", in(reg) (pos >> 2), options(nostack, preserves_flags));
            if end - pos <= step { break; }
            pos = pos.wrapping_add(step);
        }
    }
}

pub unsafe fn kvm_riscv_local_hfence_gvma_all() {
    core::arch::asm!("hfence.gvma zero, zero", options(nostack, preserves_flags));
}

pub unsafe fn kvm_riscv_local_hfence_vvma_asid_gva(vmid: c_ulong, asid: c_ulong, gva: c_ulong, gvsz: c_ulong, order: c_ulong) {
    let step = (1 as c_ulong).wrapping_shl(order as u32);
    let end = match gva.checked_add(gvsz) { Some(v) => v, None => { kvm_riscv_local_hfence_vvma_asid_all(vmid, asid); return; } };
    if PTRS_PER_PTE < (gvsz >> order) { kvm_riscv_local_hfence_vvma_asid_all(vmid, asid); return; }
    let hgatp = csr_swap(CSR_HGATP, vmid << HGATP_VMID_SHIFT);
    let mut pos = gva;
    if has_svinval() {
        core::arch::asm!("sfence.w.inval", options(nostack, preserves_flags));
        while pos < end { core::arch::asm!("hinval.vvma {0}, {1}", in(reg) pos, in(reg) asid, options(nostack, preserves_flags)); if end-pos <= step { break; } pos=pos.wrapping_add(step); }
        core::arch::asm!("sfence.inval.ir", options(nostack, preserves_flags));
    } else {
        while pos < end { core::arch::asm!("hfence.vvma {0}, {1}", in(reg) pos, in(reg) asid, options(nostack, preserves_flags)); if end-pos <= step { break; } pos=pos.wrapping_add(step); }
    }
    csr_write(CSR_HGATP, hgatp);
}

pub unsafe fn kvm_riscv_local_hfence_vvma_asid_all(vmid: c_ulong, asid: c_ulong) {
    let hgatp = csr_swap(CSR_HGATP, vmid << HGATP_VMID_SHIFT);
    core::arch::asm!("hfence.vvma zero, {0}", in(reg) asid, options(nostack, preserves_flags));
    csr_write(CSR_HGATP, hgatp);
}

pub unsafe fn kvm_riscv_local_hfence_vvma_gva(vmid: c_ulong, gva: c_ulong, gvsz: c_ulong, order: c_ulong) {
    let step=(1 as c_ulong).wrapping_shl(order as u32); let end=match gva.checked_add(gvsz){Some(v)=>v,None=>{kvm_riscv_local_hfence_vvma_all(vmid);return;}};
    if PTRS_PER_PTE < (gvsz >> order) { kvm_riscv_local_hfence_vvma_all(vmid); return; }
    let hgatp=csr_swap(CSR_HGATP,vmid<<HGATP_VMID_SHIFT); let mut pos=gva;
    if has_svinval(){core::arch::asm!("sfence.w.inval",options(nostack,preserves_flags));while pos<end{core::arch::asm!("hinval.vvma {0}, zero",in(reg)pos,options(nostack,preserves_flags));if end-pos<=step{break;}pos=pos.wrapping_add(step);}core::arch::asm!("sfence.inval.ir",options(nostack,preserves_flags));}else{while pos<end{core::arch::asm!("hfence.vvma {0}, zero",in(reg)pos,options(nostack,preserves_flags));if end-pos<=step{break;}pos=pos.wrapping_add(step);}}
    csr_write(CSR_HGATP,hgatp);
}

pub unsafe fn kvm_riscv_local_hfence_vvma_all(vmid:c_ulong){let hgatp=csr_swap(CSR_HGATP,vmid<<HGATP_VMID_SHIFT);core::arch::asm!("hfence.vvma zero, zero",options(nostack,preserves_flags));csr_write(CSR_HGATP,hgatp);}

pub unsafe fn kvm_riscv_local_tlb_sanitize(vcpu:*mut kvm_vcpu){let vmid;if !kvm_riscv_gstage_vmid_bits()||(*(*vcpu).arch.last_exit_cpu==(*vcpu).cpu){return;}vmid=READ_ONCE((*(*vcpu).kvm).arch.vmid.vmid);kvm_riscv_local_hfence_gvma_vmid_all(vmid);if static_branch_unlikely(&kvm_riscv_vsstage_tlb_no_gpa){kvm_riscv_local_hfence_vvma_all(vmid);}}
pub unsafe fn kvm_riscv_fence_i_process(vcpu:*mut kvm_vcpu){kvm_riscv_vcpu_pmu_incr_fw(vcpu,SBI_PMU_FW_FENCE_I_RCVD);local_flush_icache_all();}
pub unsafe fn kvm_riscv_tlb_flush_process(vcpu:*mut kvm_vcpu){let v=&(*(*vcpu).kvm).arch.vmid;let vmid=READ_ONCE(v.vmid);if kvm_riscv_nacl_available(){nacl_hfence_gvma_vmid_all(nacl_shmem(),vmid);}else{kvm_riscv_local_hfence_gvma_vmid_all(vmid);}}
pub unsafe fn kvm_riscv_hfence_vvma_all_process(vcpu:*mut kvm_vcpu){let v=&(*(*vcpu).kvm).arch.vmid;let vmid=READ_ONCE(v.vmid);if kvm_riscv_nacl_available(){nacl_hfence_vvma_all(nacl_shmem(),vmid);}else{kvm_riscv_local_hfence_vvma_all(vmid);}}

unsafe fn vcpu_hfence_dequeue(vcpu:*mut kvm_vcpu,out_data:*mut kvm_riscv_hfence)->bool{let a=&mut(*vcpu).arch;spin_lock(&mut a.hfence_lock);let ok=(*a.hfence_queue[a.hfence_head]).type_!=0;if ok{core::ptr::copy_nonoverlapping(&a.hfence_queue[a.hfence_head],out_data,1);(*a.hfence_queue[a.hfence_head]).type_=0;a.hfence_head+=1;if a.hfence_head==KVM_RISCV_VCPU_MAX_HFENCE{a.hfence_head=0;}}spin_unlock(&mut a.hfence_lock);ok}
unsafe fn vcpu_hfence_enqueue(vcpu:*mut kvm_vcpu,data:*const kvm_riscv_hfence)->bool{let a=&mut(*vcpu).arch;spin_lock(&mut a.hfence_lock);let ok=(*a.hfence_queue[a.hfence_tail]).type_==0;if ok{core::ptr::copy_nonoverlapping(data,&mut a.hfence_queue[a.hfence_tail],1);a.hfence_tail+=1;if a.hfence_tail==KVM_RISCV_VCPU_MAX_HFENCE{a.hfence_tail=0;}}spin_unlock(&mut a.hfence_lock);ok}

pub unsafe fn kvm_riscv_hfence_process(vcpu:*mut kvm_vcpu){let mut d:kvm_riscv_hfence=core::mem::zeroed();while vcpu_hfence_dequeue(vcpu,&mut d){match d.type_{KVM_RISCV_HFENCE_UNKNOWN=>{},KVM_RISCV_HFENCE_GVMA_VMID_GPA=>{if kvm_riscv_nacl_available(){nacl_hfence_gvma_vmid(nacl_shmem(),d.vmid,d.addr,d.size,d.order);}else{kvm_riscv_local_hfence_gvma_vmid_gpa(d.vmid,d.addr,d.size,d.order);}},KVM_RISCV_HFENCE_GVMA_VMID_ALL=>{if kvm_riscv_nacl_available(){nacl_hfence_gvma_vmid_all(nacl_shmem(),d.vmid);}else{kvm_riscv_local_hfence_gvma_vmid_all(d.vmid);}},KVM_RISCV_HFENCE_VVMA_ASID_GVA=>{kvm_riscv_vcpu_pmu_incr_fw(vcpu,SBI_PMU_FW_HFENCE_VVMA_ASID_RCVD);if kvm_riscv_nacl_available(){nacl_hfence_vvma_asid(nacl_shmem(),d.vmid,d.asid,d.addr,d.size,d.order);}else{kvm_riscv_local_hfence_vvma_asid_gva(d.vmid,d.asid,d.addr,d.size,d.order);}},KVM_RISCV_HFENCE_VVMA_ASID_ALL=>{kvm_riscv_vcpu_pmu_incr_fw(vcpu,SBI_PMU_FW_HFENCE_VVMA_ASID_RCVD);if kvm_riscv_nacl_available(){nacl_hfence_vvma_asid_all(nacl_shmem(),d.vmid,d.asid);}else{kvm_riscv_local_hfence_vvma_asid_all(d.vmid,d.asid);}},KVM_RISCV_HFENCE_VVMA_GVA=>{kvm_riscv_vcpu_pmu_incr_fw(vcpu,SBI_PMU_FW_HFENCE_VVMA_RCVD);if kvm_riscv_nacl_available(){nacl_hfence_vvma(nacl_shmem(),d.vmid,d.addr,d.size,d.order);}else{kvm_riscv_local_hfence_vvma_gva(d.vmid,d.addr,d.size,d.order);}},KVM_RISCV_HFENCE_VVMA_ALL=>{kvm_riscv_vcpu_pmu_incr_fw(vcpu,SBI_PMU_FW_HFENCE_VVMA_RCVD);if kvm_riscv_nacl_available(){nacl_hfence_vvma_all(nacl_shmem(),d.vmid);}else{kvm_riscv_local_hfence_vvma_all(d.vmid);}},_=>{}}}}

unsafe fn make_xfence_request_nodata(kvm:*mut kvm,hbase:c_ulong,hmask:c_ulong,req:c_uint){let mut mask=core::mem::zeroed();bitmap_zero(&mut mask,KVM_MAX_VCPUS);let mut i=0;let mut v=core::ptr::null_mut();kvm_for_each_vcpu(kvm,&mut i,&mut v){if hbase!=!0{if (*v).vcpu_id<hbase||(*v).vcpu_id>=hbase+BITS_PER_LONG{continue;}if hmask&(1usize<<((*v).vcpu_id-hbase))==0{continue;}}bitmap_set(&mut mask,i,1);}kvm_make_vcpus_request_mask(kvm,req,&mask);}
unsafe fn make_xfence_request(kvm:*mut kvm,hbase:c_ulong,hmask:c_ulong,req:c_uint,fallback_req:c_uint,data:*const kvm_riscv_hfence){if data.is_null()||(*data).type_==0{return;}let mut rm=core::mem::zeroed();let mut fm=core::mem::zeroed();bitmap_zero(&mut rm,KVM_MAX_VCPUS);bitmap_zero(&mut fm,KVM_MAX_VCPUS);let mut i=0;let mut v=core::ptr::null_mut();kvm_for_each_vcpu(kvm,&mut i,&mut v){if hbase!=!0{if (*v).vcpu_id<hbase||(*v).vcpu_id>=hbase+BITS_PER_LONG{continue;}if hmask&(1usize<<((*v).vcpu_id-hbase))==0{continue;}}if !vcpu_hfence_enqueue(v,data){bitmap_set(&mut fm,i,1);}else{bitmap_set(&mut rm,i,1);}}kvm_make_vcpus_request_mask(kvm,req,&rm);kvm_make_vcpus_request_mask(kvm,fallback_req,&fm);}

pub unsafe fn kvm_riscv_fence_i(k:*mut kvm,b:c_ulong,m:c_ulong){make_xfence_request_nodata(k,b,m,KVM_REQ_FENCE_I);}
pub unsafe fn kvm_riscv_hfence_gvma_vmid_gpa(k:*mut kvm,b:c_ulong,m:c_ulong,g:gpa_t,s:gpa_t,o:c_ulong,v:c_ulong){let d=kvm_riscv_hfence{type_:KVM_RISCV_HFENCE_GVMA_VMID_GPA,asid:0,vmid:v,addr:g,size:s,order:o};make_xfence_request(k,b,m,KVM_REQ_HFENCE,KVM_REQ_TLB_FLUSH,&d);}
pub unsafe fn kvm_riscv_hfence_gvma_vmid_all(k:*mut kvm,b:c_ulong,m:c_ulong,v:c_ulong){let mut d:kvm_riscv_hfence=core::mem::zeroed();d.type_=KVM_RISCV_HFENCE_GVMA_VMID_ALL;d.vmid=v;make_xfence_request(k,b,m,KVM_REQ_HFENCE,KVM_REQ_TLB_FLUSH,&d);}
pub unsafe fn kvm_riscv_hfence_vvma_asid_gva(k:*mut kvm,b:c_ulong,m:c_ulong,g:c_ulong,s:c_ulong,o:c_ulong,a:c_ulong,v:c_ulong){let d=kvm_riscv_hfence{type_:KVM_RISCV_HFENCE_VVMA_ASID_GVA,asid:a,vmid:v,addr:g,size:s,order:o};make_xfence_request(k,b,m,KVM_REQ_HFENCE,KVM_REQ_HFENCE_VVMA_ALL,&d);}
pub unsafe fn kvm_riscv_hfence_vvma_asid_all(k:*mut kvm,b:c_ulong,m:c_ulong,a:c_ulong,v:c_ulong){let mut d:kvm_riscv_hfence=core::mem::zeroed();d.type_=KVM_RISCV_HFENCE_VVMA_ASID_ALL;d.asid=a;d.vmid=v;make_xfence_request(k,b,m,KVM_REQ_HFENCE,KVM_REQ_HFENCE_VVMA_ALL,&d);}
pub unsafe fn kvm_riscv_hfence_vvma_gva(k:*mut kvm,b:c_ulong,m:c_ulong,g:c_ulong,s:c_ulong,o:c_ulong,v:c_ulong){let d=kvm_riscv_hfence{type_:KVM_RISCV_HFENCE_VVMA_GVA,asid:0,vmid:v,addr:g,size:s,order:o};make_xfence_request(k,b,m,KVM_REQ_HFENCE,KVM_REQ_HFENCE_VVMA_ALL,&d);}
pub unsafe fn kvm_riscv_hfence_vvma_all(k:*mut kvm,b:c_ulong,m:c_ulong,v:c_ulong){let mut d:kvm_riscv_hfence=core::mem::zeroed();d.type_=KVM_RISCV_HFENCE_VVMA_ALL;d.vmid=v;make_xfence_request(k,b,m,KVM_REQ_HFENCE,KVM_REQ_HFENCE_VVMA_ALL,&d);}
pub unsafe fn kvm_arch_flush_remote_tlbs_range(k:*mut kvm,g:gfn_t,n:u64)->c_int{kvm_riscv_hfence_gvma_vmid_gpa(k,!0,0,g<<PAGE_SHIFT,n<<PAGE_SHIFT,PAGE_SHIFT,READ_ONCE((*k).arch.vmid.vmid));0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
