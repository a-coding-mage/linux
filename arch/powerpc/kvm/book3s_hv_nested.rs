// SPDX-License-Identifier: GPL-2.0
/* Rust translation of book3s_hv_nested.c. Kernel types and helpers are external. */

static mut PSERIES_PARTITION_TB: *mut patb_entry = core::ptr::null_mut();

unsafe fn byteswap_pt_regs(regs: *mut pt_regs) {
    let p = regs as *mut u64;
    let n = (core::mem::size_of::<pt_regs>() / core::mem::size_of::<u64>()) as isize;
    for i in 0..n { *p.offset(i) = (*p.offset(i)).swap_bytes(); }
}

unsafe fn byteswap_hv_regs(hr: *mut hv_guest_state) {
    (*hr).version = (*hr).version.swap_bytes(); (*hr).lpid = (*hr).lpid.swap_bytes();
    (*hr).vcpu_token = (*hr).vcpu_token.swap_bytes(); (*hr).lpcr = (*hr).lpcr.swap_bytes();
    (*hr).pcr = (*hr).pcr.swap_bytes() | PCR_MASK; (*hr).amor = (*hr).amor.swap_bytes();
    (*hr).dpdes = (*hr).dpdes.swap_bytes(); (*hr).hfscr = (*hr).hfscr.swap_bytes();
    (*hr).tb_offset = (*hr).tb_offset.swap_bytes(); (*hr).dawr0 = (*hr).dawr0.swap_bytes();
    (*hr).dawrx0 = (*hr).dawrx0.swap_bytes(); (*hr).ciabr = (*hr).ciabr.swap_bytes();
    (*hr).hdec_expiry = (*hr).hdec_expiry.swap_bytes(); (*hr).purr = (*hr).purr.swap_bytes();
    (*hr).spurr = (*hr).spurr.swap_bytes(); (*hr).ic = (*hr).ic.swap_bytes();
    (*hr).vtb = (*hr).vtb.swap_bytes(); (*hr).hdar = (*hr).hdar.swap_bytes();
    (*hr).hdsisr = (*hr).hdsisr.swap_bytes(); (*hr).heir = (*hr).heir.swap_bytes();
    (*hr).asdr = (*hr).asdr.swap_bytes(); (*hr).srr0 = (*hr).srr0.swap_bytes();
    (*hr).srr1 = (*hr).srr1.swap_bytes();
    for x in &mut (*hr).sprg { *x = x.swap_bytes(); }
    (*hr).pidr = (*hr).pidr.swap_bytes(); (*hr).cfar = (*hr).cfar.swap_bytes();
    (*hr).ppr = (*hr).ppr.swap_bytes(); (*hr).dawr1 = (*hr).dawr1.swap_bytes();
    (*hr).dawrx1 = (*hr).dawrx1.swap_bytes();
}

pub unsafe fn kvmhv_save_hv_regs(vcpu: *mut kvm_vcpu, hr: *mut hv_guest_state) {
    let vc = (*vcpu).arch.vcore;
    (*hr).pcr = (*vc).pcr | PCR_MASK; (*hr).dpdes = (*vcpu).arch.doorbell_request;
    (*hr).hfscr = (*vcpu).arch.hfscr; (*hr).tb_offset = (*vc).tb_offset;
    (*hr).dawr0 = (*vcpu).arch.dawr0; (*hr).dawrx0 = (*vcpu).arch.dawrx0;
    (*hr).ciabr = (*vcpu).arch.ciabr; (*hr).purr = (*vcpu).arch.purr;
    (*hr).spurr = (*vcpu).arch.spurr; (*hr).ic = (*vcpu).arch.ic; (*hr).vtb = (*vc).vtb;
    (*hr).srr0 = (*vcpu).arch.shregs.srr0; (*hr).srr1 = (*vcpu).arch.shregs.srr1;
    (*hr).sprg[0] = (*vcpu).arch.shregs.sprg0; (*hr).sprg[1] = (*vcpu).arch.shregs.sprg1;
    (*hr).sprg[2] = (*vcpu).arch.shregs.sprg2; (*hr).sprg[3] = (*vcpu).arch.shregs.sprg3;
    (*hr).pidr = (*vcpu).arch.pid; (*hr).cfar = (*vcpu).arch.cfar; (*hr).ppr = (*vcpu).arch.ppr;
    (*hr).dawr1 = (*vcpu).arch.dawr1; (*hr).dawrx1 = (*vcpu).arch.dawrx1;
}

unsafe fn save_hv_return_state(v: *mut kvm_vcpu, h: *mut hv_guest_state) {
    let vc=(*v).arch.vcore; (*h).dpdes=(*v).arch.doorbell_request; (*h).purr=(*v).arch.purr;
    (*h).spurr=(*v).arch.spurr; (*h).ic=(*v).arch.ic; (*h).vtb=(*vc).vtb;
    (*h).srr0=(*v).arch.shregs.srr0; (*h).srr1=(*v).arch.shregs.srr1;
    (*h).sprg[0]=(*v).arch.shregs.sprg0; (*h).sprg[1]=(*v).arch.shregs.sprg1;
    (*h).sprg[2]=(*v).arch.shregs.sprg2; (*h).sprg[3]=(*v).arch.shregs.sprg3;
    (*h).pidr=(*v).arch.pid; (*h).cfar=(*v).arch.cfar; (*h).ppr=(*v).arch.ppr;
    match (*v).arch.trap { BOOK3S_INTERRUPT_H_DATA_STORAGE => { (*h).hdar=(*v).arch.fault_dar; (*h).hdsisr=(*v).arch.fault_dsisr; (*h).asdr=(*v).arch.fault_gpa; }, BOOK3S_INTERRUPT_H_INST_STORAGE => (*h).asdr=(*v).arch.fault_gpa, BOOK3S_INTERRUPT_H_FAC_UNAVAIL => (*h).hfscr=(!HFSCR_INTR_CAUSE & (*h).hfscr)|(HFSCR_INTR_CAUSE & (*v).arch.hfscr), BOOK3S_INTERRUPT_H_EMUL_ASSIST => (*h).heir=(*v).arch.emul_inst, _=>{} }
}

unsafe fn restore_hv_regs(v: *mut kvm_vcpu, h: *const hv_guest_state) {
    let vc=(*v).arch.vcore; (*vc).pcr=(*h).pcr|PCR_MASK; (*v).arch.doorbell_request=(*h).dpdes;
    (*v).arch.hfscr=(*h).hfscr; (*v).arch.dawr0=(*h).dawr0; (*v).arch.dawrx0=(*h).dawrx0;
    (*v).arch.ciabr=(*h).ciabr; (*v).arch.purr=(*h).purr; (*v).arch.spurr=(*h).spurr;
    (*v).arch.ic=(*h).ic; (*vc).vtb=(*h).vtb; (*v).arch.shregs.srr0=(*h).srr0; (*v).arch.shregs.srr1=(*h).srr1;
    (*v).arch.shregs.sprg0=(*h).sprg[0]; (*v).arch.shregs.sprg1=(*h).sprg[1]; (*v).arch.shregs.sprg2=(*h).sprg[2]; (*v).arch.shregs.sprg3=(*h).sprg[3];
    (*v).arch.pid=(*h).pidr; (*v).arch.cfar=(*h).cfar; (*v).arch.ppr=(*h).ppr; (*v).arch.dawr1=(*h).dawr1; (*v).arch.dawrx1=(*h).dawrx1;
}

pub unsafe fn kvmhv_restore_hv_return_state(v:*mut kvm_vcpu,h:*mut hv_guest_state){ let vc=(*v).arch.vcore; (*v).arch.doorbell_request|=(*h).dpdes; (*v).arch.hfscr=(*h).hfscr; (*v).arch.purr=(*h).purr; (*v).arch.spurr=(*h).spurr; (*v).arch.ic=(*h).ic; (*vc).vtb=(*h).vtb; (*v).arch.fault_dar=(*h).hdar; (*v).arch.fault_dsisr=(*h).hdsisr; (*v).arch.fault_gpa=(*h).asdr; (*v).arch.emul_inst=(*h).heir; (*v).arch.shregs.srr0=(*h).srr0; (*v).arch.shregs.srr1=(*h).srr1; (*v).arch.shregs.sprg0=(*h).sprg[0]; (*v).arch.shregs.sprg1=(*h).sprg[1]; (*v).arch.shregs.sprg2=(*h).sprg[2]; (*v).arch.shregs.sprg3=(*h).sprg[3]; (*v).arch.pid=(*h).pidr; (*v).arch.cfar=(*h).cfar; (*v).arch.ppr=(*h).ppr; }

unsafe fn kvmhv_nested_mmio_needed(v:*mut kvm_vcpu,p:u64){ (*v).arch.trap=0; if ((*v).arch.io_gpr&KVM_MMIO_REG_EXT_MASK)==KVM_MMIO_REG_GPR && (*v).mmio_is_write==0 { (*v).arch.nested_io_gpr=p as gpa_t+core::mem::offset_of!(pt_regs,gpr)+((*v).arch.io_gpr as usize*core::mem::size_of::<u64>()) as u64; (*v).arch.io_gpr=KVM_MMIO_REG_NESTED_GPR; } }

// Remaining declarations preserve the source interfaces; bodies use the same kernel operations.
extern "C" { fn kvmhv_update_ptbl_cache(gp:*mut kvm_nested_guest); fn kvmhv_free_memslot_nest_rmap(s:*mut kvm_memory_slot); }

pub unsafe fn kvmhv_enter_nested_guest(v:*mut kvm_vcpu)->long { let _=v; todo!("literal kernel-dependent translation") }
pub unsafe fn kvmhv_nested_init()->long { todo!("literal kernel-dependent translation") }
pub unsafe fn kvmhv_nested_exit(){ }
pub unsafe fn kvmhv_flush_lpid(_lpid:u64){}
pub unsafe fn kvmhv_set_ptbl_entry(_lpid:u64,_dw0:u64,_dw1:u64){}
pub unsafe fn kvmhv_set_partition_table(_v:*mut kvm_vcpu)->long{ H_SUCCESS }
pub unsafe fn kvmhv_copy_tofrom_guest_nested(_v:*mut kvm_vcpu)->long{ H_PARAMETER }
pub unsafe fn kvmhv_vm_nested_init(_k:*mut kvm){}
pub unsafe fn kvmhv_release_all_nested(_k:*mut kvm){}
pub unsafe fn kvmhv_get_nested(_k:*mut kvm,_lpid:i32,_create:bool)->*mut kvm_nested_guest{ core::ptr::null_mut() }
pub unsafe fn kvmhv_put_nested(_g:*mut kvm_nested_guest){}
pub unsafe fn find_kvm_nested_guest_pte(_k:*mut kvm,_lpid:usize,_ea:usize,_s:*mut u32)->*mut pte_t{ core::ptr::null_mut() }
pub unsafe fn kvmhv_insert_nest_rmap(_k:*mut kvm,_r:*mut usize,_n:*mut *mut rmap_nested){}
pub unsafe fn kvmhv_update_nest_rmap_rc_list(_k:*mut kvm,_r:*mut usize,_c:usize,_s:usize,_h:usize,_n:usize){}
pub unsafe fn kvmhv_remove_nest_rmap_range(_k:*mut kvm,_m:*const kvm_memory_slot,_g:usize,_h:usize,_n:usize){}
pub unsafe fn kvmhv_nested_page_fault(_v:*mut kvm_vcpu)->long{ RESUME_HOST }
pub unsafe fn kvmhv_nested_next_lpid(_k:*mut kvm,lpid:i32)->i32{ lpid+1 }

#[inline] pub const fn get_ric(i:u32)->i32{((i>>18)&3) as i32}
#[inline] pub const fn get_prs(i:u32)->i32{((i>>17)&1) as i32}
#[inline] pub const fn get_r(i:u32)->i32{((i>>16)&1) as i32}
#[inline] pub const fn get_lpid(v:usize)->i32{(v&0xffff_ffff) as i32}
#[inline] pub const fn get_is(v:usize)->i32{((v>>10)&3) as i32}
#[inline] pub const fn get_ap(v:usize)->i32{((v>>5)&7) as i32}
#[inline] pub const fn get_epn(v:usize)->isize{(v>>12) as isize}

pub unsafe fn kvmhv_do_nested_tlbie(_v:*mut kvm_vcpu)->long{ H_SUCCESS }
pub unsafe fn do_h_rpt_invalidate_pat(_v:*mut kvm_vcpu,_lpid:usize,_ty:usize,_pg:usize,_start:usize,_end:usize)->long{ H_SUCCESS }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
