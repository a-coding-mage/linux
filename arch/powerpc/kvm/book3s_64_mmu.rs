// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of book3s_64_mmu.c. Kernel types, constants, and
// external functions are supplied by the surrounding translation unit.

unsafe fn kvmppc_mmu_book3s_64_find_slbe(vcpu: *mut kvm_vcpu, eaddr: gva_t) -> *mut kvmppc_slb {
    let esid = GET_ESID(eaddr);
    let esid_1t = GET_ESID_1T(eaddr);
    for i in 0..(*vcpu).arch.slb_nr {
        let slb = &mut (*vcpu).arch.slb[i as usize];
        if !slb.valid { continue; }
        let cmp = if slb.tb { esid_1t } else { esid };
        if slb.esid == cmp { return slb as *mut _; }
    }
    core::ptr::null_mut()
}

unsafe fn kvmppc_slb_sid_shift(slbe: *mut kvmppc_slb) -> i32 { if (*slbe).tb { SID_SHIFT_1T } else { SID_SHIFT } }
unsafe fn kvmppc_slb_offset_mask(slbe: *mut kvmppc_slb) -> u64 { (1u64 << kvmppc_slb_sid_shift(slbe)) - 1 }
unsafe fn kvmppc_slb_calc_vpn(slb: *mut kvmppc_slb, mut eaddr: gva_t) -> u64 {
    eaddr &= kvmppc_slb_offset_mask(slb);
    (eaddr >> VPN_SHIFT) | ((*slb).vsid << (kvmppc_slb_sid_shift(slb) - VPN_SHIFT))
}
unsafe fn kvmppc_mmu_book3s_64_ea_to_vp(vcpu: *mut kvm_vcpu, eaddr: gva_t, _data: bool) -> u64 {
    let slb = kvmppc_mmu_book3s_64_find_slbe(vcpu, eaddr); if slb.is_null() { 0 } else { kvmppc_slb_calc_vpn(slb, eaddr) }
}
unsafe fn mmu_pagesize(mmu_pg: i32) -> i32 { match mmu_pg { MMU_PAGE_64K => 16, MMU_PAGE_16M => 24, _ => 12 } }
unsafe fn kvmppc_mmu_book3s_64_get_pagesize(slbe: *mut kvmppc_slb) -> i32 { mmu_pagesize((*slbe).base_page_size) }
unsafe fn kvmppc_mmu_book3s_64_get_page(slbe: *mut kvmppc_slb, eaddr: gva_t) -> u32 { ((eaddr & kvmppc_slb_offset_mask(slbe)) >> kvmppc_mmu_book3s_64_get_pagesize(slbe)) as u32 }

unsafe fn kvmppc_mmu_book3s_64_get_pteg(vcpu: *mut kvm_vcpu, slbe: *mut kvmppc_slb, eaddr: gva_t, second: bool) -> hva_t {
    let b = to_book3s(vcpu); let htabsize = (1u64 << ((((*b).sdr1 & 0x1f) + 11) as u32)) - 1;
    let vpn = kvmppc_slb_calc_vpn(slbe, eaddr); let ssize = if (*slbe).tb { MMU_SEGSIZE_1T } else { MMU_SEGSIZE_256M };
    let mut hash = hpt_hash(vpn, kvmppc_mmu_book3s_64_get_pagesize(slbe), ssize); if second { hash = !hash; }
    hash = (hash & ((1u64 << 39) - 1)) & htabsize; hash <<= 7;
    let pteg = ((*b).sdr1 & 0xfffffffffffc0000) | hash;
    let r = if (*vcpu).arch.papr_enabled { pteg } else { gfn_to_hva((*vcpu).kvm, pteg >> PAGE_SHIFT) };
    if kvm_is_error_hva(r) { r } else { r | (pteg & !PAGE_MASK) }
}
unsafe fn kvmppc_mmu_book3s_64_get_avpn(slbe: *mut kvmppc_slb, eaddr: gva_t) -> u64 {
    let p = kvmppc_mmu_book3s_64_get_pagesize(slbe); let mut avpn = kvmppc_mmu_book3s_64_get_page(slbe,eaddr) as u64;
    avpn |= (*slbe).vsid << (kvmppc_slb_sid_shift(slbe)-p); if p < 16 { avpn >>= (16-p) - 8 } else { avpn <<= p-16 }; avpn
}
unsafe fn decode_pagesize(slbe: *mut kvmppc_slb, r: u64) -> i32 { match (*slbe).base_page_size { MMU_PAGE_64K if r&0xf000==0x1000 => MMU_PAGE_64K, MMU_PAGE_16M if r&0xff000==0 => MMU_PAGE_16M, _ => -1 } }

unsafe fn kvmppc_mmu_book3s_64_xlate(vcpu: *mut kvm_vcpu, eaddr: gva_t, gpte: *mut kvmppc_pte, data: bool, iswrite: bool) -> i32 {
    let slbe = kvmppc_mmu_book3s_64_find_slbe(vcpu,eaddr); if slbe.is_null() { return -EINVAL; }
    let avpn=kvmppc_mmu_book3s_64_get_avpn(slbe,eaddr); let mut v_val=avpn&HPTE_V_AVPN;
    if (*slbe).tb {v_val|=SLB_VSID_B_1T} if (*slbe).large {v_val|=HPTE_V_LARGE} v_val|=HPTE_V_VALID;
    let v_mask=SLB_VSID_B|HPTE_V_AVPN|HPTE_V_LARGE|HPTE_V_VALID|HPTE_V_SECONDARY; let mut pgsize=if (*slbe).large{MMU_PAGE_16M}else{MMU_PAGE_4K};
    let mut second=false; let mut pteg=[0u64;16]; let ptegp;
    mutex_lock(&mut (*(*vcpu).kvm).arch.hpt_mutex);
    'lookup: loop { ptegp=kvmppc_mmu_book3s_64_get_pteg(vcpu,slbe,eaddr,second); if kvm_is_error_hva(ptegp){mutex_unlock(&mut (*(*vcpu).kvm).arch.hpt_mutex);return -ENOENT;}
        if copy_from_user(pteg.as_mut_ptr(),ptegp as *const _,core::mem::size_of_val(&pteg))!=0 {mutex_unlock(&mut (*(*vcpu).kvm).arch.hpt_mutex);return -ENOENT;}
        for i in (0..16).step_by(2) { let p0=be64_to_cpu(pteg[i]); let p1=be64_to_cpu(pteg[i+1]); if p0&v_mask==v_val { if (*slbe).large&&((*vcpu).arch.hflags&BOOK3S_HFLAG_MULTI_PGSIZE)!=0 {pgsize=decode_pagesize(slbe,p1);if pgsize<0{continue;}} break 'lookup; } }
        if second {mutex_unlock(&mut (*(*vcpu).kvm).arch.hpt_mutex);return -ENOENT;} v_val|=HPTE_V_SECONDARY;second=true;
    }
    let i=(0..16).step_by(2).find(|&j| (be64_to_cpu(pteg[j])&v_mask)==v_val).unwrap_or(0); let r=be64_to_cpu(pteg[i+1]); let mut pp=(r&HPTE_R_PP)|if ((kvmppc_get_msr(vcpu)&MSR_PR)!=0&&(*slbe).Kp)||((kvmppc_get_msr(vcpu)&MSR_PR)==0&&(*slbe).Ks){4}else{0};if r&HPTE_R_PP0!=0{pp|=8;}
    (*gpte).eaddr=eaddr;(*gpte).vpage=kvmppc_mmu_book3s_64_ea_to_vp(vcpu,eaddr,data);let mask=(1u64<<mmu_pagesize(pgsize))-1;(*gpte).raddr=(r&HPTE_R_RPN&!mask)|(eaddr&mask);(*gpte).page_size=pgsize;(*gpte).may_execute=r&HPTE_R_N==0;(*gpte).may_read=false;(*gpte).may_write=false;(*gpte).wimg=r&HPTE_R_WIMG;
    match pp {0|1|2|6=>{(*gpte).may_write=true;(*gpte).may_read=true},3|5|7|10=>{(*gpte).may_read=true},_=>{}} mutex_unlock(&mut (*(*vcpu).kvm).arch.hpt_mutex);if !(*gpte).may_read||iswrite&&!(*gpte).may_write{-EPERM}else{0}
}

unsafe fn kvmppc_mmu_book3s_64_slbmte(vcpu:*mut kvm_vcpu,rs:u64,rb:u64){let esid=GET_ESID(rb);let e1=GET_ESID_1T(rb);let n=(rb&0xfff) as usize;if n>=(*vcpu).arch.slb_nr as usize{return}let s=&mut (*vcpu).arch.slb[n];s.large=rs&SLB_VSID_L!=0;s.tb=rs&SLB_VSID_B_1T!=0;s.esid=if s.tb{e1}else{esid};s.vsid=(rs&!SLB_VSID_B)>>(kvmppc_slb_sid_shift(s)-16);s.valid=rb&SLB_ESID_V!=0;s.Ks=rs&SLB_VSID_KS!=0;s.Kp=rs&SLB_VSID_KP!=0;s.nx=rs&SLB_VSID_N!=0;s.class=rs&SLB_VSID_C!=0;s.base_page_size=if s.large{MMU_PAGE_16M}else{MMU_PAGE_4K};s.orige=rb&(ESID_MASK|SLB_ESID_V);s.origv=rs;kvmppc_mmu_map_segment(vcpu,esid<<SID_SHIFT);}
unsafe fn kvmppc_mmu_book3s_64_slbfee(v:*mut kvm_vcpu,e:gva_t,r:*mut ulong)->i32{let s=kvmppc_mmu_book3s_64_find_slbe(v,e);if s.is_null(){*r=0;-ENOENT}else{*r=(*s).origv;0}}
unsafe fn kvmppc_mmu_book3s_64_slbmfee(v:*mut kvm_vcpu,n:u64)->u64{if n>=(*v).arch.slb_nr as u64{0}else{(*v).arch.slb[n as usize].orige}}
unsafe fn kvmppc_mmu_book3s_64_slbmfev(v:*mut kvm_vcpu,n:u64)->u64{if n>=(*v).arch.slb_nr as u64{0}else{(*v).arch.slb[n as usize].origv}}
unsafe fn kvmppc_mmu_book3s_64_slbie(v:*mut kvm_vcpu,e:u64){let s=kvmppc_mmu_book3s_64_find_slbe(v,e);if s.is_null(){return}(*s).valid=false;(*s).orige=0;(*s).origv=0;let z=1u64<<kvmppc_slb_sid_shift(s);kvmppc_mmu_flush_segment(v,e&!(z-1),z);}
unsafe fn kvmppc_mmu_book3s_64_slbia(v:*mut kvm_vcpu){for i in 1..(*v).arch.slb_nr as usize{(*v).arch.slb[i].valid=false;(*v).arch.slb[i].orige=0;(*v).arch.slb[i].origv=0;}if kvmppc_get_msr(v)&MSR_IR!=0{kvmppc_mmu_flush_segments(v);kvmppc_mmu_map_segment(v,kvmppc_get_pc(v));}}
unsafe fn kvmppc_mmu_book3s_64_mtsrin(v:*mut kvm_vcpu,n:u32,value:ulong){let rb=((n&0xf)<<28)|(1<<27)|n;let rs=((value&0xfffffff)<<12)|(((value>>28)&7)<<9);kvmppc_mmu_book3s_64_slbmte(v,rs as u64,rb as u64);}
unsafe fn kvmppc_mmu_book3s_64_tlbie(v:*mut kvm_vcpu,va:ulong,large:bool){let mut mask=0xFFFFFFFFFu64;if (*v).arch.hflags&BOOK3S_HFLAG_NEW_TLBIE!=0{if va&1!=0{mask=if va&0xf000==0x1000{0xFFFFFFFF0}else{0xFFFFFF000}}}else if large{mask=0xFFFFFF000}let mut i=0;let mut x=core::ptr::null_mut();kvm_for_each_vcpu(i,x,(*v).kvm){kvmppc_mmu_pte_vflush(x,va>>12,mask);}}
unsafe fn kvmppc_mmu_book3s_64_esid_to_vsid(v:*mut kvm_vcpu,esid:ulong,out:*mut u64)->i32{let mut g=esid;let mut s=kvmppc_mmu_book3s_64_find_slbe(v,esid<<SID_SHIFT);let msr=kvmppc_get_msr(v);if msr&(MSR_DR|MSR_IR)!=0&&!s.is_null(){g=(*s).vsid;if (*s).tb{g=(g<<(SID_SHIFT_1T-SID_SHIFT))|(esid&((1u64<<(SID_SHIFT_1T-SID_SHIFT))-1))|VSID_1T;}}match msr&(MSR_DR|MSR_IR){0=>g=VSID_REAL|esid,MSR_IR=>g|=VSID_REAL_IR,MSR_DR=>g|=VSID_REAL_DR,_=>if s.is_null(){return -EINVAL}}if msr&MSR_PR!=0{g|=VSID_PR}*out=g;0}
unsafe fn kvmppc_mmu_book3s_64_is_dcbz32(v:*mut kvm_vcpu)->bool{to_book3s(v).hid[5]&0x80!=0}
pub unsafe fn kvmppc_mmu_book3s_64_init(v:*mut kvm_vcpu){let m=&mut (*v).arch.mmu;m.mfsrin=None;m.mtsrin=Some(kvmppc_mmu_book3s_64_mtsrin);m.slbmte=Some(kvmppc_mmu_book3s_64_slbmte);m.slbmfee=Some(kvmppc_mmu_book3s_64_slbmfee);m.slbmfev=Some(kvmppc_mmu_book3s_64_slbmfev);m.slbfee=Some(kvmppc_mmu_book3s_64_slbfee);m.slbie=Some(kvmppc_mmu_book3s_64_slbie);m.slbia=Some(kvmppc_mmu_book3s_64_slbia);m.xlate=Some(kvmppc_mmu_book3s_64_xlate);m.tlbie=Some(kvmppc_mmu_book3s_64_tlbie);m.esid_to_vsid=Some(kvmppc_mmu_book3s_64_esid_to_vsid);m.ea_to_vp=Some(kvmppc_mmu_book3s_64_ea_to_vp);m.is_dcbz32=Some(kvmppc_mmu_book3s_64_is_dcbz32);(*v).arch.hflags|=BOOK3S_HFLAG_SLB;}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
