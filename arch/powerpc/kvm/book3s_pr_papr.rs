// SPDX-License-Identifier: GPL-2.0-only
/* Hypercall handling for running PAPR guests in PR KVM on Book 3S processors. */

const HPTE_SIZE: usize = 16;

unsafe fn get_pteg_addr(vcpu: *mut kvm_vcpu, mut pte_index: libc::c_long) -> libc::c_ulong {
    let vcpu_book3s = to_book3s(vcpu);
    pte_index <<= 4;
    pte_index &= ((((1u64 << ((((*vcpu_book3s).sdr1 & 0x1f) + 11) as u32)) - 1) << 7) | 0x70) as libc::c_long;
    let mut pteg_addr = (*vcpu_book3s).sdr1 & 0xfffffffffffc0000u64;
    pteg_addr |= pte_index as u64;
    pteg_addr as libc::c_ulong
}

unsafe fn kvmppc_h_pr_enter(vcpu: *mut kvm_vcpu) -> libc::c_int {
    let flags = kvmppc_get_gpr(vcpu, 4) as libc::c_long;
    let mut pte_index = kvmppc_get_gpr(vcpu, 5) as libc::c_long;
    let mut pteg = [0u64; 16];
    let mut i = (pte_index & 7) as usize;
    pte_index &= !7;
    let pteg_addr = get_pteg_addr(vcpu, pte_index);
    mutex_lock(&mut (*(*vcpu).kvm).arch.hpt_mutex);
    let mut ret = H_FUNCTION;
    if copy_from_user(pteg.as_mut_ptr() as *mut _, pteg_addr as *const _, core::mem::size_of_val(&pteg)) != 0 { goto_done!(); }
    ret = H_PTEG_FULL;
    if (flags & H_EXACT as libc::c_long) == 0 {
        i = 0;
        loop {
            if i == 8 { goto_done!(); }
            if (u64::from_be(pteg[i * 2]) & HPTE_V_VALID) == 0 { break; }
            i += 1;
        }
    } else if u64::from_be(pteg[i * 2]) & HPTE_V_VALID != 0 { goto_done!(); }
    pteg[i * 2] = (kvmppc_get_gpr(vcpu, 6) as u64).to_be();
    pteg[i * 2 + 1] = (kvmppc_get_gpr(vcpu, 7) as u64).to_be();
    ret = H_FUNCTION;
    if copy_to_user((pteg_addr + i * HPTE_SIZE as libc::c_ulong) as *mut _, pteg.as_ptr().add(i * 2) as *const _, HPTE_SIZE) != 0 { goto_done!(); }
    kvmppc_set_gpr(vcpu, 4, (pte_index as u64) | i as u64);
    ret = H_SUCCESS;
    goto_done!();
    mutex_unlock(&mut (*(*vcpu).kvm).arch.hpt_mutex);
    kvmppc_set_gpr(vcpu, 3, ret as u64);
    EMULATE_DONE
}

unsafe fn kvmppc_h_pr_remove(vcpu: *mut kvm_vcpu) -> libc::c_int {
    let flags = kvmppc_get_gpr(vcpu, 4); let pte_index = kvmppc_get_gpr(vcpu, 5); let avpn = kvmppc_get_gpr(vcpu, 6);
    let pteg = get_pteg_addr(vcpu, pte_index as libc::c_long); let mut pte = [0u64; 2]; let v = 0u64;
    mutex_lock(&mut (*(*vcpu).kvm).arch.hpt_mutex); let mut ret = H_FUNCTION;
    if copy_from_user(pte.as_mut_ptr() as *mut _, pteg as *const _, 16) != 0 { goto_done!(); }
    pte[0] = u64::from_be(pte[0]); pte[1] = u64::from_be(pte[1]); ret = H_NOT_FOUND;
    if pte[0] & HPTE_V_VALID == 0 || (flags & H_AVPN != 0 && pte[0] & !0x7f != avpn) || (flags & H_ANDCOND != 0 && pte[0] & avpn != 0) { goto_done!(); }
    ret = H_FUNCTION; if copy_to_user(pteg as *mut _, &v as *const _ as *const _, 8) != 0 { goto_done!(); }
    let rb = compute_tlbie_rb(pte[0], pte[1], pte_index); (*(*vcpu).arch.mmu.tlbie)(vcpu, rb, rb & 1 != 0);
    ret = H_SUCCESS; kvmppc_set_gpr(vcpu, 4, pte[0]); kvmppc_set_gpr(vcpu, 5, pte[1]);
    goto_done!(); mutex_unlock(&mut (*(*vcpu).kvm).arch.hpt_mutex); kvmppc_set_gpr(vcpu, 3, ret as u64); EMULATE_DONE
}

const H_BULK_REMOVE_TYPE:u64=0xc000000000000000; const H_BULK_REMOVE_REQUEST:u64=0x4000000000000000; const H_BULK_REMOVE_RESPONSE:u64=0x8000000000000000; const H_BULK_REMOVE_END:u64=0xc000000000000000;
const H_BULK_REMOVE_CODE:u64=0x3000000000000000; const H_BULK_REMOVE_SUCCESS:u64=0; const H_BULK_REMOVE_NOT_FOUND:u64=0x1000000000000000; const H_BULK_REMOVE_PARM:u64=0x2000000000000000; const H_BULK_REMOVE_RC:u64=0x0c00000000000000; const H_BULK_REMOVE_FLAGS:u64=0x0300000000000000; const H_BULK_REMOVE_PTEX:u64=0x00ffffffffffffff; const H_BULK_REMOVE_MAX_BATCH:usize=4;
const H_BULK_REMOVE_ANDCOND:u64=0x0100000000000000; const H_BULK_REMOVE_AVPN:u64=0x0200000000000000;

unsafe fn kvmppc_h_pr_bulk_remove(vcpu:*mut kvm_vcpu)->libc::c_int { let mut ret=H_SUCCESS; mutex_lock(&mut (*(*vcpu).kvm).arch.hpt_mutex); for i in 0..H_BULK_REMOVE_MAX_BATCH { let mut tsh=kvmppc_get_gpr(vcpu,4+2*i); let tsl=kvmppc_get_gpr(vcpu,5+2*i); if tsh&H_BULK_REMOVE_TYPE==H_BULK_REMOVE_END{break;} if tsh&H_BULK_REMOVE_TYPE!=H_BULK_REMOVE_REQUEST{ret=H_PARAMETER;break;} tsh=(tsh&(H_BULK_REMOVE_PTEX|H_BULK_REMOVE_FLAGS))|H_BULK_REMOVE_RESPONSE; if tsh&H_BULK_REMOVE_ANDCOND!=0&&tsh&H_BULK_REMOVE_AVPN!=0{tsh|=H_BULK_REMOVE_PARM;kvmppc_set_gpr(vcpu,4+2*i,tsh);ret=H_PARAMETER;break;} let pteg=get_pteg_addr(vcpu,(tsh&H_BULK_REMOVE_PTEX) as libc::c_long); let mut pte=[0u64;2]; if copy_from_user(pte.as_mut_ptr() as *mut _,pteg as *const _,16)!=0{ret=H_FUNCTION;break;} pte[0]=u64::from_be(pte[0]);pte[1]=u64::from_be(pte[1]);let flags=(tsh&H_BULK_REMOVE_FLAGS)>>26;if pte[0]&HPTE_V_VALID==0||(flags&H_AVPN!=0&&pte[0]&!0x7f!=tsl)||(flags&H_ANDCOND!=0&&pte[0]&tsl!=0){tsh|=H_BULK_REMOVE_NOT_FOUND;}else{let v=0u64;if copy_to_user(pteg as *mut_,&v as *const _ as *const _,8)!=0{ret=H_FUNCTION;break;}let rb=compute_tlbie_rb(pte[0],pte[1],tsh&H_BULK_REMOVE_PTEX);(*(*vcpu).arch.mmu.tlbie)(vcpu,rb,rb&1!=0);tsh|=H_BULK_REMOVE_SUCCESS;tsh|=(pte[1]&(HPTE_R_C|HPTE_R_R))<<43;}kvmppc_set_gpr(vcpu,4+2*i,tsh);}mutex_unlock(&mut (*(*vcpu).kvm).arch.hpt_mutex);kvmppc_set_gpr(vcpu,3,ret as u64);EMULATE_DONE }

unsafe fn kvmppc_h_pr_protect(vcpu:*mut kvm_vcpu)->libc::c_int { let flags=kvmppc_get_gpr(vcpu,4);let pte_index=kvmppc_get_gpr(vcpu,5);let avpn=kvmppc_get_gpr(vcpu,6);let pteg=get_pteg_addr(vcpu,pte_index as libc::c_long);let mut pte=[0u64;2];mutex_lock(&mut (*(*vcpu).kvm).arch.hpt_mutex);let mut ret=H_FUNCTION;if copy_from_user(pte.as_mut_ptr() as *mut _,pteg as *const _,16)!=0{goto_done!();}pte[0]=u64::from_be(pte[0]);pte[1]=u64::from_be(pte[1]);ret=H_NOT_FOUND;if pte[0]&HPTE_V_VALID==0||(flags&H_AVPN!=0&&pte[0]&!0x7f!=avpn){goto_done!();}let v=pte[0];let mut r=pte[1];r&=!(HPTE_R_PP0|HPTE_R_PP|HPTE_R_N|HPTE_R_KEY_HI|HPTE_R_KEY_LO);r|=(flags<<55)&HPTE_R_PP0;r|=(flags<<48)&HPTE_R_KEY_HI;r|=flags&(HPTE_R_PP|HPTE_R_N|HPTE_R_KEY_LO);let rb=compute_tlbie_rb(v,r,pte_index);(*(*vcpu).arch.mmu.tlbie)(vcpu,rb,rb&1!=0);pte[0]=pte[0].to_be();pte[1]=r.to_be();ret=H_FUNCTION;if copy_to_user(pteg as *mut _,pte.as_ptr() as *const _,16)!=0{goto_done!();}ret=H_SUCCESS;goto_done!();mutex_unlock(&mut (*(*vcpu).kvm).arch.hpt_mutex);kvmppc_set_gpr(vcpu,3,ret as u64);EMULATE_DONE }

unsafe fn kvmppc_h_pr_logical_ci_load(v:*mut kvm_vcpu)->libc::c_int{let rc=kvmppc_h_logical_ci_load(v);if rc==H_TOO_HARD{return EMULATE_FAIL;}kvmppc_set_gpr(v,3,rc as u64);EMULATE_DONE} unsafe fn kvmppc_h_pr_logical_ci_store(v:*mut kvm_vcpu)->libc::c_int{let rc=kvmppc_h_logical_ci_store(v);if rc==H_TOO_HARD{return EMULATE_FAIL;}kvmppc_set_gpr(v,3,rc as u64);EMULATE_DONE}
unsafe fn kvmppc_h_pr_set_mode(v:*mut kvm_vcpu)->libc::c_int{let m=kvmppc_get_gpr(v,4);if kvmppc_get_gpr(v,5)==H_SET_MODE_RESOURCE_ADDR_TRANS_MODE{if m==0{kvmppc_set_gpr(v,3,H_SUCCESS as u64)}else{kvmppc_set_gpr(v,3,(H_UNSUPPORTED_FLAG_START-63) as u64)}return EMULATE_DONE;}EMULATE_FAIL}

unsafe fn kvmppc_h_pr_put_tce(v:*mut kvm_vcpu)->libc::c_int{let r=kvmppc_h_put_tce(v,kvmppc_get_gpr(v,4),kvmppc_get_gpr(v,5),kvmppc_get_gpr(v,6));if r==H_TOO_HARD{return EMULATE_FAIL;}kvmppc_set_gpr(v,3,r as u64);EMULATE_DONE} unsafe fn kvmppc_h_pr_put_tce_indirect(v:*mut kvm_vcpu)->libc::c_int{let r=kvmppc_h_put_tce_indirect(v,kvmppc_get_gpr(v,4),kvmppc_get_gpr(v,5),kvmppc_get_gpr(v,6),kvmppc_get_gpr(v,7));if r==H_TOO_HARD{return EMULATE_FAIL;}kvmppc_set_gpr(v,3,r as u64);EMULATE_DONE} unsafe fn kvmppc_h_pr_stuff_tce(v:*mut kvm_vcpu)->libc::c_int{let r=kvmppc_h_stuff_tce(v,kvmppc_get_gpr(v,4),kvmppc_get_gpr(v,5),kvmppc_get_gpr(v,6),kvmppc_get_gpr(v,7));if r==H_TOO_HARD{return EMULATE_FAIL;}kvmppc_set_gpr(v,3,r as u64);EMULATE_DONE}

unsafe fn kvmppc_h_pr_xics_hcall(v:*mut kvm_vcpu,cmd:u32)->libc::c_int{let r=kvmppc_xics_hcall(v,cmd);kvmppc_set_gpr(v,3,r as u64);EMULATE_DONE}
pub unsafe fn kvmppc_h_pr(v:*mut kvm_vcpu,cmd:libc::c_ulong)->libc::c_int{if cmd<=MAX_HCALL_OPCODE as libc::c_ulong&&!test_bit(cmd/4,(*(*v).kvm).arch.enabled_hcalls){return EMULATE_FAIL;}match cmd as u32{H_ENTER=>kvmppc_h_pr_enter(v),H_REMOVE=>kvmppc_h_pr_remove(v),H_PROTECT=>kvmppc_h_pr_protect(v),H_BULK_REMOVE=>kvmppc_h_pr_bulk_remove(v),H_PUT_TCE=>kvmppc_h_pr_put_tce(v),H_PUT_TCE_INDIRECT=>kvmppc_h_pr_put_tce_indirect(v),H_STUFF_TCE=>kvmppc_h_pr_stuff_tce(v),H_CEDE=>{kvmppc_set_msr_fast(v,kvmppc_get_msr(v)|MSR_EE);kvm_vcpu_halt(v);(*v).stat.generic.halt_wakeup+=1;EMULATE_DONE},H_LOGICAL_CI_LOAD=>kvmppc_h_pr_logical_ci_load(v),H_LOGICAL_CI_STORE=>kvmppc_h_pr_logical_ci_store(v),H_SET_MODE=>kvmppc_h_pr_set_mode(v),H_XIRR|H_CPPR|H_EOI|H_IPI|H_IPOLL|H_XIRR_X=>if kvmppc_xics_enabled(v){kvmppc_h_pr_xics_hcall(v,cmd as u32)}else{EMULATE_FAIL},_=>EMULATE_FAIL}}

pub fn kvmppc_hcall_impl_pr(cmd: libc::c_ulong)->libc::c_int{match cmd as u32{H_ENTER|H_REMOVE|H_PROTECT|H_BULK_REMOVE|H_GET_TCE|H_PUT_TCE|H_PUT_TCE_INDIRECT|H_STUFF_TCE|H_CEDE|H_LOGICAL_CI_LOAD|H_LOGICAL_CI_STORE|H_SET_MODE|H_XIRR|H_CPPR|H_EOI|H_IPI|H_IPOLL|H_XIRR_X=>1,_=>0}}
static DEFAULT_HCALL_LIST:&[u32]=&[H_ENTER,H_REMOVE,H_PROTECT,H_BULK_REMOVE,H_GET_TCE,H_PUT_TCE,H_CEDE,H_SET_MODE,H_XIRR,H_CPPR,H_EOI,H_IPI,H_IPOLL,H_XIRR_X,0];
pub unsafe fn kvmppc_pr_init_default_hcalls(kvm:*mut kvm){for &h in DEFAULT_HCALL_LIST{if h==0{break;}WARN_ON(kvmppc_hcall_impl_pr(h as libc::c_ulong)==0);__set_bit((h/4) as usize,(*kvm).arch.enabled_hcalls);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
