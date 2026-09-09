// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of e500_mmu.c; kernel dependencies are external. */

unsafe fn gtlb0_get_next_victim(v: *mut kvmppc_vcpu_e500) -> u32 {
    let victim = (*v).gtlb_nv[0];
    (*v).gtlb_nv[0] = (*v).gtlb_nv[0].wrapping_add(1);
    if (*v).gtlb_nv[0] >= (*v).gtlb_params[0].ways { (*v).gtlb_nv[0] = 0; }
    victim
}

unsafe fn tlb0_set_base(addr: gva_t, sets: i32, ways: i32) -> i32 {
    (((addr >> PAGE_SHIFT) as i32) & (sets - 1)) * ways
}
unsafe fn gtlb0_set_base(v: *mut kvmppc_vcpu_e500, addr: gva_t) -> i32 {
    tlb0_set_base(addr, (*v).gtlb_params[0].sets, (*v).gtlb_params[0].ways)
}
unsafe fn get_tlb_esel(vcpu: *mut kvm_vcpu, tlbsel: i32) -> u32 {
    let v = to_e500(vcpu); let mut esel = get_tlb_esel_bit(vcpu) as i32;
    if tlbsel == 0 { esel &= (*v).gtlb_params[0].ways - 1; esel += gtlb0_set_base(v, (*vcpu).arch.shared.mas2) }
    else { esel &= (*v).gtlb_params[tlbsel as usize].entries as i32 - 1; }
    esel as u32
}

/* Search the guest TLB for a matching entry. */
unsafe fn kvmppc_e500_tlb_index(v: *mut kvmppc_vcpu_e500, eaddr: gva_t, tlbsel: i32, pid: u32, as_: i32) -> i32 {
    let mut size = (*v).gtlb_params[tlbsel as usize].entries as i32; let set_base: u32;
    if tlbsel == 0 { set_base = gtlb0_set_base(v, eaddr) as u32; size = (*v).gtlb_params[0].ways; }
    else { if eaddr < (*v).tlb1_min_eaddr || eaddr > (*v).tlb1_max_eaddr { return -1; } set_base = 0; }
    let offset = (*v).gtlb_offset[tlbsel as usize];
    for i in 0..size { let t = (*v).gtlb_arch.add((offset + set_base + i as u32) as usize); let tid = get_tlb_tid(t);
        if eaddr < get_tlb_eaddr(t) || eaddr > get_tlb_end(t) || (tid != 0 && tid != pid) || !get_tlb_v(t) || (get_tlb_ts(t) != as_ && as_ != -1) { continue; }
        return set_base as i32 + i;
    } -1
}

unsafe fn kvmppc_e500_deliver_tlb_miss(vcpu: *mut kvm_vcpu, eaddr: gva_t, as_: i32) {
    let v = to_e500(vcpu); let tlbsel = (((*vcpu).arch.shared.mas4 >> 28) & 1) as i32;
    let victim = if tlbsel == 0 { gtlb0_get_next_victim(v) } else { 0 }; let tsized = ((*vcpu).arch.shared.mas4 >> 7) & 0x1f;
    (*vcpu).arch.shared.mas0 = MAS0_TLBSEL(tlbsel) | MAS0_ESEL(victim) | MAS0_NV((*v).gtlb_nv[tlbsel as usize]);
    (*vcpu).arch.shared.mas1 = MAS1_VALID | if as_ != 0 { MAS1_TS } else { 0 } | MAS1_TID(get_tlbmiss_tid(vcpu)) | MAS1_TSIZE(tsized);
    (*vcpu).arch.shared.mas2 = (eaddr & MAS2_EPN) | ((*vcpu).arch.shared.mas4 & MAS2_ATTRIB_MASK);
    (*vcpu).arch.shared.mas7_3 &= MAS3_U0 | MAS3_U1 | MAS3_U2 | MAS3_U3;
    (*vcpu).arch.shared.mas6 = ((*vcpu).arch.shared.mas6 & MAS6_SPID1) | (get_cur_pid(vcpu) << 16) | if as_ != 0 { MAS6_SAS } else { 0 };
}

unsafe fn kvmppc_recalc_tlb1map_range(v: *mut kvmppc_vcpu_e500) {
    (*v).tlb1_min_eaddr = !0; (*v).tlb1_max_eaddr = 0; let off = (*v).gtlb_offset[1];
    for i in 0..(*v).gtlb_params[1].entries { let t = (*v).gtlb_arch.add((off + i) as usize); if get_tlb_v(t) { (*v).tlb1_min_eaddr = core::cmp::min((*v).tlb1_min_eaddr, get_tlb_eaddr(t)); (*v).tlb1_max_eaddr = core::cmp::max((*v).tlb1_max_eaddr, get_tlb_end(t)); } }
}
unsafe fn kvmppc_need_recalc_tlb1map_range(v: *mut kvmppc_vcpu_e500, t: *mut kvm_book3e_206_tlb_entry) -> bool { let s=get_tlb_bytes(t); let st=get_tlb_eaddr(t)&!(s-1); let en=st+s-1; (*v).tlb1_min_eaddr==st || (*v).tlb1_max_eaddr==en }
unsafe fn kvmppc_set_tlb1map_range(vcpu: *mut kvm_vcpu, t: *mut kvm_book3e_206_tlb_entry) { let v=to_e500(vcpu); if !get_tlb_v(t){return;} let s=get_tlb_bytes(t); let st=get_tlb_eaddr(t)&!(s-1); let en=st+s-1; (*v).tlb1_min_eaddr=core::cmp::min((*v).tlb1_min_eaddr,st); (*v).tlb1_max_eaddr=core::cmp::max((*v).tlb1_max_eaddr,en); }
unsafe fn kvmppc_e500_gtlbe_invalidate(v:*mut kvmppc_vcpu_e500, tlbsel:i32, esel:i32)->i32 { let t=get_entry(v,tlbsel,esel); if get_tlb_iprot(t){return -1;} if tlbsel==1 && kvmppc_need_recalc_tlb1map_range(v,t){kvmppc_recalc_tlb1map_range(v);} (*t).mas1=0; 0 }

pub unsafe fn kvmppc_e500_emul_mt_mmucsr0(v:*mut kvmppc_vcpu_e500,value:ulong)->i32 { if value&MMUCSR0_TLB0FI!=0 {for e in 0..(*v).gtlb_params[0].entries as i32 {kvmppc_e500_gtlbe_invalidate(v,0,e);}} if value&MMUCSR0_TLB1FI!=0 {for e in 0..(*v).gtlb_params[1].entries as i32 {kvmppc_e500_gtlbe_invalidate(v,1,e);}} kvmppc_core_flush_tlb(&mut (*v).vcpu); EMULATE_DONE }
pub unsafe fn kvmppc_e500_emul_tlbivax(vcpu:*mut kvm_vcpu,mut ea:gva_t)->i32 {let v=to_e500(vcpu);let ia=(ea>>2)&1;let tlbsel=((ea>>3)&1) as i32;if ia!=0 {for e in 0..(*v).gtlb_params[tlbsel as usize].entries as i32{kvmppc_e500_gtlbe_invalidate(v,tlbsel,e);}}else{ea&=0xfffff000;let e=kvmppc_e500_tlb_index(v,ea,tlbsel,get_cur_pid(vcpu),-1);if e>=0{kvmppc_e500_gtlbe_invalidate(v,tlbsel,e);}}kvmppc_core_flush_tlb(&mut (*v).vcpu);EMULATE_DONE}
unsafe fn tlbilx_all(v:*mut kvmppc_vcpu_e500,tlbsel:i32,pid:i32,typ:i32){for e in 0..(*v).gtlb_params[tlbsel as usize].entries as i32{let t=get_entry(v,tlbsel,e);if typ==0||get_tlb_tid(t) as i32==pid{inval_gtlbe_on_host(v,tlbsel,e);kvmppc_e500_gtlbe_invalidate(v,tlbsel,e);}}}
unsafe fn tlbilx_one(v:*mut kvmppc_vcpu_e500,pid:i32,ea:gva_t){for s in 0..2{let e=kvmppc_e500_tlb_index(v,ea,s,pid as u32,-1);if e>=0{inval_gtlbe_on_host(v,s,e);kvmppc_e500_gtlbe_invalidate(v,s,e);break;}}}
pub unsafe fn kvmppc_e500_emul_tlbilx(vcpu:*mut kvm_vcpu,typ:i32,ea:gva_t)->i32{let v=to_e500(vcpu);let pid=get_cur_spid(vcpu) as i32;if typ==0||typ==1{tlbilx_all(v,0,pid,typ);tlbilx_all(v,1,pid,typ)}else if typ==3{tlbilx_one(v,pid,ea)}EMULATE_DONE}

pub unsafe fn kvmppc_e500_emul_tlbre(vcpu:*mut kvm_vcpu)->i32{let v=to_e500(vcpu);let s=get_tlb_tlbsel(vcpu);let e=get_tlb_esel(vcpu,s) as i32;let t=get_entry(v,s,e);(*vcpu).arch.shared.mas0=((*vcpu).arch.shared.mas0)&!MAS0_NV(!0);(*vcpu).arch.shared.mas0|=MAS0_NV((*v).gtlb_nv[s as usize]);(*vcpu).arch.shared.mas1=(*t).mas1;(*vcpu).arch.shared.mas2=(*t).mas2;(*vcpu).arch.shared.mas7_3=(*t).mas7_3;EMULATE_DONE}
pub unsafe fn kvmppc_e500_emul_tlbsx(vcpu:*mut kvm_vcpu,ea:gva_t)->i32{let v=to_e500(vcpu);let as_=get_cur_sas(vcpu) as i32;let pid=get_cur_spid(vcpu);let(mut e,mut s)=(-1,0);let mut t=core::ptr::null_mut();for x in 0..2{e=kvmppc_e500_tlb_index(v,ea,x,pid,as_);if e>=0{s=x;t=get_entry(v,x,e);break;}}if !t.is_null(){e&=(*v).gtlb_params[s as usize].ways-1;(*vcpu).arch.shared.mas0=MAS0_TLBSEL(s)|MAS0_ESEL(e)|MAS0_NV((*v).gtlb_nv[s as usize]);(*vcpu).arch.shared.mas1=(*t).mas1;(*vcpu).arch.shared.mas2=(*t).mas2;(*vcpu).arch.shared.mas7_3=(*t).mas7_3;}else{ s=(((*vcpu).arch.shared.mas4>>28)&1) as i32;let victim=if s==0{gtlb0_get_next_victim(v)}else{0};(*vcpu).arch.shared.mas0=MAS0_TLBSEL(s)|MAS0_ESEL(victim)|MAS0_NV((*v).gtlb_nv[s as usize]);(*vcpu).arch.shared.mas1=((*vcpu).arch.shared.mas6&MAS6_SPID0)|if (*vcpu).arch.shared.mas6&MAS6_SAS!=0{MAS1_TS}else{0}|((*vcpu).arch.shared.mas4&MAS4_TSIZED(!0));(*vcpu).arch.shared.mas2=((*vcpu).arch.shared.mas2&MAS2_EPN)|((*vcpu).arch.shared.mas4&MAS2_ATTRIB_MASK);(*vcpu).arch.shared.mas7_3&=MAS3_U0|MAS3_U1|MAS3_U2|MAS3_U3;}kvmppc_set_exit_type(vcpu,EMULATED_TLBSX_EXITS);EMULATE_DONE}

pub unsafe fn kvmppc_e500_emul_tlbwe(vcpu:*mut kvm_vcpu)->i32{let v=to_e500(vcpu);let s=get_tlb_tlbsel(vcpu);let e=get_tlb_esel(vcpu,s) as i32;let t=get_entry(v,s,e);let mut recal=false;if get_tlb_v(t){inval_gtlbe_on_host(v,s,e);if s==1&&kvmppc_need_recalc_tlb1map_range(v,t){recal=true;}}(*t).mas1=(*vcpu).arch.shared.mas1;(*t).mas2=(*vcpu).arch.shared.mas2;if (*vcpu).arch.shared.msr&MSR_CM==0{(*t).mas2&=0xffffffff;}(*t).mas7_3=(*vcpu).arch.shared.mas7_3;if s==1{if recal{kvmppc_recalc_tlb1map_range(v)}else{kvmppc_set_tlb1map_range(vcpu,t)}}let idx=srcu_read_lock((*vcpu).kvm.srcu);if tlbe_is_host_safe(vcpu,t){let ea=get_tlb_eaddr(t);let ra=get_tlb_raddr(t);if s==0{(*t).mas1&=!MAS1_TSIZE(!0);(*t).mas1|=MAS1_TSIZE(BOOK3E_PAGESZ_4K);}kvmppc_mmu_map(vcpu,ea,ra,index_of(s,e));}srcu_read_unlock((*vcpu).kvm.srcu,idx);kvmppc_set_exit_type(vcpu,EMULATED_TLBWE_EXITS);EMULATE_DONE}
unsafe fn kvmppc_e500_tlb_search(vcpu:*mut kvm_vcpu,ea:gva_t,pid:u32,as_:i32)->i32{let v=to_e500(vcpu);for s in 0..2{let e=kvmppc_e500_tlb_index(v,ea,s,pid,as_);if e>=0{return index_of(s,e)}}-1}
pub unsafe fn kvmppc_core_vcpu_translate(vcpu:*mut kvm_vcpu,tr:*mut kvm_translation)->i32{let ea=(*tr).linear_address;let pid=((ea>>32)&0xff) as u8;let as_=((ea>>40)&1) as u8;let i=kvmppc_e500_tlb_search(vcpu,ea,pid as u32,as_ as i32);if i<0{(*tr).valid=0;return 0;}(*tr).physical_address=kvmppc_mmu_xlate(vcpu,i as u32,ea);(*tr).valid=1;0}
pub unsafe fn kvmppc_mmu_itlb_index(vcpu:*mut kvm_vcpu,ea:gva_t)->i32{kvmppc_e500_tlb_search(vcpu,ea,get_cur_pid(vcpu),(((*vcpu).arch.shared.msr&MSR_IS)!=0) as i32)}
pub unsafe fn kvmppc_mmu_dtlb_index(vcpu:*mut kvm_vcpu,ea:gva_t)->i32{kvmppc_e500_tlb_search(vcpu,ea,get_cur_pid(vcpu),(((*vcpu).arch.shared.msr&MSR_DS)!=0) as i32)}
pub unsafe fn kvmppc_mmu_itlb_miss(vcpu:*mut kvm_vcpu){kvmppc_e500_deliver_tlb_miss(vcpu,(*vcpu).arch.regs.nip,(((*vcpu).arch.shared.msr&MSR_IS)!=0) as i32)}
pub unsafe fn kvmppc_mmu_dtlb_miss(vcpu:*mut kvm_vcpu){kvmppc_e500_deliver_tlb_miss(vcpu,(*vcpu).arch.fault_dear,(((*vcpu).arch.shared.msr&MSR_DS)!=0) as i32)}
pub unsafe fn kvmppc_mmu_xlate(vcpu:*mut kvm_vcpu,index:u32,ea:gva_t)->gpa_t{let v=to_e500(vcpu);let t=get_entry(v,tlbsel_of(index),esel_of(index));let mask=get_tlb_bytes(t)-1;get_tlb_raddr(t)|(ea&mask)}

unsafe fn free_gtlb(v:*mut kvmppc_vcpu_e500){kvmppc_core_flush_tlb(&mut (*v).vcpu);kfree((*v).g2h_tlb1_map);kfree((*v).gtlb_priv[0]);kfree((*v).gtlb_priv[1]);if !(*v).shared_tlb_pages.is_null(){vfree(round_down((*v).gtlb_arch as usize,PAGE_SIZE) as *mut _);for i in 0..(*v).num_shared_tlb_pages{set_page_dirty_lock(*(*v).shared_tlb_pages.add(i));put_page(*(*v).shared_tlb_pages.add(i));}(*v).num_shared_tlb_pages=0;kfree((*v).shared_tlb_pages);(*v).shared_tlb_pages=core::ptr::null_mut();}else{kfree((*v).gtlb_arch);}(*v).gtlb_arch=core::ptr::null_mut();}
pub unsafe fn kvmppc_get_sregs_e500_tlb(vcpu:*mut kvm_vcpu,s:*mut kvm_sregs){(*s).u.e.mas0=(*vcpu).arch.shared.mas0;(*s).u.e.mas1=(*vcpu).arch.shared.mas1;(*s).u.e.mas2=(*vcpu).arch.shared.mas2;(*s).u.e.mas7_3=(*vcpu).arch.shared.mas7_3;(*s).u.e.mas4=(*vcpu).arch.shared.mas4;(*s).u.e.mas6=(*vcpu).arch.shared.mas6;(*s).u.e.mmucfg=(*vcpu).arch.mmucfg;(*s).u.e.tlbcfg[0]=(*vcpu).arch.tlbcfg[0];(*s).u.e.tlbcfg[1]=(*vcpu).arch.tlbcfg[1];(*s).u.e.tlbcfg[2]=0;(*s).u.e.tlbcfg[3]=0;}
pub unsafe fn kvmppc_set_sregs_e500_tlb(vcpu:*mut kvm_vcpu,s:*mut kvm_sregs)->i32{if (*s).u.e.features&KVM_SREGS_E_ARCH206_MMU!=0{(*vcpu).arch.shared.mas0=(*s).u.e.mas0;(*vcpu).arch.shared.mas1=(*s).u.e.mas1;(*vcpu).arch.shared.mas2=(*s).u.e.mas2;(*vcpu).arch.shared.mas7_3=(*s).u.e.mas7_3;(*vcpu).arch.shared.mas4=(*s).u.e.mas4;(*vcpu).arch.shared.mas6=(*s).u.e.mas6;}0}

pub unsafe fn kvmppc_get_one_reg_e500_tlb(v:*mut kvm_vcpu,id:u64,val:*mut kvmppc_one_reg)->i32{match id{KVM_REG_PPC_MAS0=>*val=get_reg_val(id,(*v).arch.shared.mas0),KVM_REG_PPC_MAS1=>*val=get_reg_val(id,(*v).arch.shared.mas1),KVM_REG_PPC_MAS2=>*val=get_reg_val(id,(*v).arch.shared.mas2),KVM_REG_PPC_MAS7_3=>*val=get_reg_val(id,(*v).arch.shared.mas7_3),KVM_REG_PPC_MAS4=>*val=get_reg_val(id,(*v).arch.shared.mas4),KVM_REG_PPC_MAS6=>*val=get_reg_val(id,(*v).arch.shared.mas6),KVM_REG_PPC_MMUCFG=>*val=get_reg_val(id,(*v).arch.mmucfg),KVM_REG_PPC_EPTCFG=>*val=get_reg_val(id,(*v).arch.eptcfg),KVM_REG_PPC_TLB0CFG..=KVM_REG_PPC_TLB3CFG=>{let i=(id-KVM_REG_PPC_TLB0CFG)as usize;*val=get_reg_val(id,(*v).arch.tlbcfg[i])},KVM_REG_PPC_TLB0PS..=KVM_REG_PPC_TLB3PS=>{let i=(id-KVM_REG_PPC_TLB0PS)as usize;*val=get_reg_val(id,(*v).arch.tlbps[i])},_=>return -EINVAL}0}
pub unsafe fn kvmppc_set_one_reg_e500_tlb(v:*mut kvm_vcpu,id:u64,val:*mut kvmppc_one_reg)->i32{match id{KVM_REG_PPC_MAS0=>(*v).arch.shared.mas0=set_reg_val(id,*val),KVM_REG_PPC_MAS1=>(*v).arch.shared.mas1=set_reg_val(id,*val),KVM_REG_PPC_MAS2=>(*v).arch.shared.mas2=set_reg_val(id,*val),KVM_REG_PPC_MAS7_3=>(*v).arch.shared.mas7_3=set_reg_val(id,*val),KVM_REG_PPC_MAS4=>(*v).arch.shared.mas4=set_reg_val(id,*val),KVM_REG_PPC_MAS6=>(*v).arch.shared.mas6=set_reg_val(id,*val),KVM_REG_PPC_MMUCFG=>if set_reg_val(id,*val)!=(*v).arch.mmucfg{return -EINVAL},KVM_REG_PPC_EPTCFG=>if set_reg_val(id,*val)!=(*v).arch.eptcfg{return -EINVAL},KVM_REG_PPC_TLB0CFG..=KVM_REG_PPC_TLB3CFG=>{let i=(id-KVM_REG_PPC_TLB0CFG)as usize;if set_reg_val(id,*val)!=(*v).arch.tlbcfg[i]{return -EINVAL}},KVM_REG_PPC_TLB0PS..=KVM_REG_PPC_TLB3PS=>{let i=(id-KVM_REG_PPC_TLB0PS)as usize;if set_reg_val(id,*val)!=(*v).arch.tlbps[i]{return -EINVAL}},_=>return -EINVAL}0}
unsafe fn vcpu_mmu_geometry_update(v:*mut kvm_vcpu,p:*mut kvm_book3e_206_tlb_params)->i32{(*v).arch.tlbcfg[0]&=!(TLBnCFG_N_ENTRY|TLBnCFG_ASSOC);if (*p).tlb_sizes[0]<=2048{(*v).arch.tlbcfg[0]|=(*p).tlb_sizes[0];}(*v).arch.tlbcfg[0]|=(*p).tlb_ways[0]<<TLBnCFG_ASSOC_SHIFT;(*v).arch.tlbcfg[1]&=!(TLBnCFG_N_ENTRY|TLBnCFG_ASSOC);(*v).arch.tlbcfg[1]|=(*p).tlb_sizes[1];(*v).arch.tlbcfg[1]|=(*p).tlb_ways[1]<<TLBnCFG_ASSOC_SHIFT;0}
pub unsafe fn kvm_vcpu_ioctl_dirty_tlb(v:*mut kvm_vcpu,_d:*mut kvm_dirty_tlb)->i32{kvmppc_recalc_tlb1map_range(to_e500(v));kvmppc_core_flush_tlb(v);0}
pub unsafe fn kvmppc_e500_tlb_uninit(v:*mut kvmppc_vcpu_e500){free_gtlb(v);e500_mmu_host_uninit(v)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
