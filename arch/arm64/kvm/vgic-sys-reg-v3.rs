// SPDX-License-Identifier: GPL-2.0-only
/*
 * VGIC system registers handling functions for AArch64 mode
 */

// External kernel declarations and register definitions are supplied by the surrounding translation unit.

unsafe fn set_gic_ctlr(vcpu: *mut kvm_vcpu, _r: *const sys_reg_desc, val: u64) -> i32 {
    let mut host_pri_bits: u32;
    let mut host_id_bits: u32;
    let mut host_seis: u32;
    let mut host_a3v: u32;
    let mut seis: u32;
    let mut a3v: u32;
    let vgic_v3_cpu = &mut (*vcpu).arch.vgic_cpu;
    let mut vmcr: vgic_vmcr = core::mem::zeroed();

    vgic_get_vmcr(vcpu, &mut vmcr);
    host_pri_bits = FIELD_GET(ICC_CTLR_EL1_PRI_BITS_MASK, val) + 1;
    if host_pri_bits > vgic_v3_cpu.num_pri_bits { return -EINVAL; }
    vgic_v3_cpu.num_pri_bits = host_pri_bits;
    host_id_bits = FIELD_GET(ICC_CTLR_EL1_ID_BITS_MASK, val);
    if host_id_bits > vgic_v3_cpu.num_id_bits { return -EINVAL; }
    vgic_v3_cpu.num_id_bits = host_id_bits;
    host_seis = FIELD_GET(ICH_VTR_EL2_SEIS, vgic_ich_vtr());
    seis = FIELD_GET(ICC_CTLR_EL1_SEIS_MASK, val);
    if host_seis != seis { return -EINVAL; }
    host_a3v = FIELD_GET(ICH_VTR_EL2_A3V, vgic_ich_vtr());
    a3v = FIELD_GET(ICC_CTLR_EL1_A3V_MASK, val);
    if host_a3v != a3v { return -EINVAL; }
    vmcr.cbpr = FIELD_GET(ICC_CTLR_EL1_CBPR_MASK, val);
    vmcr.eoim = FIELD_GET(ICC_CTLR_EL1_EOImode_MASK, val);
    vgic_set_vmcr(vcpu, &vmcr);
    0
}

unsafe fn get_gic_ctlr(vcpu: *mut kvm_vcpu, _r: *const sys_reg_desc, valp: *mut u64) -> i32 {
    let vgic_v3_cpu = &(*vcpu).arch.vgic_cpu;
    let mut vmcr: vgic_vmcr = core::mem::zeroed();
    vgic_get_vmcr(vcpu, &mut vmcr);
    let mut val = 0u64;
    val |= FIELD_PREP(ICC_CTLR_EL1_PRI_BITS_MASK, vgic_v3_cpu.num_pri_bits - 1);
    val |= FIELD_PREP(ICC_CTLR_EL1_ID_BITS_MASK, vgic_v3_cpu.num_id_bits);
    val |= FIELD_PREP(ICC_CTLR_EL1_SEIS_MASK, FIELD_GET(ICH_VTR_EL2_SEIS, vgic_ich_vtr()));
    val |= FIELD_PREP(ICC_CTLR_EL1_A3V_MASK, FIELD_GET(ICH_VTR_EL2_A3V, vgic_ich_vtr()));
    val |= FIELD_PREP(ICC_CTLR_EL1_CBPR_MASK, vmcr.cbpr);
    val |= FIELD_PREP(ICC_CTLR_EL1_EOImode_MASK, vmcr.eoim);
    *valp = val;
    0
}

unsafe fn set_gic_pmr(vcpu: *mut kvm_vcpu, _r: *const sys_reg_desc, val: u64) -> i32 { let mut v=core::mem::zeroed(); vgic_get_vmcr(vcpu,&mut v); v.pmr=FIELD_GET(ICC_PMR_EL1_MASK,val); vgic_set_vmcr(vcpu,&v); 0 }
unsafe fn get_gic_pmr(vcpu: *mut kvm_vcpu, _r: *const sys_reg_desc, val: *mut u64) -> i32 { let mut v=core::mem::zeroed(); vgic_get_vmcr(vcpu,&mut v); *val=FIELD_PREP(ICC_PMR_EL1_MASK,v.pmr); 0 }
unsafe fn set_gic_bpr0(vcpu: *mut kvm_vcpu, _r: *const sys_reg_desc, val: u64) -> i32 { let mut v=core::mem::zeroed(); vgic_get_vmcr(vcpu,&mut v); v.bpr=FIELD_GET(ICC_BPR0_EL1_MASK,val); vgic_set_vmcr(vcpu,&v); 0 }
unsafe fn get_gic_bpr0(vcpu: *mut kvm_vcpu, _r: *const sys_reg_desc, val: *mut u64) -> i32 { let mut v=core::mem::zeroed(); vgic_get_vmcr(vcpu,&mut v); *val=FIELD_PREP(ICC_BPR0_EL1_MASK,v.bpr); 0 }
unsafe fn set_gic_bpr1(vcpu: *mut kvm_vcpu, _r: *const sys_reg_desc, val: u64) -> i32 { let mut v=core::mem::zeroed(); vgic_get_vmcr(vcpu,&mut v); if v.cbpr==0 { v.abpr=FIELD_GET(ICC_BPR1_EL1_MASK,val); vgic_set_vmcr(vcpu,&v); } 0 }
unsafe fn get_gic_bpr1(vcpu: *mut kvm_vcpu, _r: *const sys_reg_desc, val: *mut u64) -> i32 { let mut v=core::mem::zeroed(); vgic_get_vmcr(vcpu,&mut v); if v.cbpr==0 { *val=FIELD_PREP(ICC_BPR1_EL1_MASK,v.abpr); } else { *val=core::cmp::min(v.bpr+1,7u32); } 0 }
unsafe fn set_gic_grpen0(vcpu:*mut kvm_vcpu,_r:*const sys_reg_desc,val:u64)->i32 { let mut v=core::mem::zeroed();vgic_get_vmcr(vcpu,&mut v);v.grpen0=FIELD_GET(ICC_IGRPEN0_EL1_MASK,val);vgic_set_vmcr(vcpu,&v);0 }
unsafe fn get_gic_grpen0(vcpu:*mut kvm_vcpu,_r:*const sys_reg_desc,val:*mut u64)->i32 { let mut v=core::mem::zeroed();vgic_get_vmcr(vcpu,&mut v);*val=FIELD_PREP(ICC_IGRPEN0_EL1_MASK,v.grpen0);0 }
unsafe fn set_gic_grpen1(vcpu:*mut kvm_vcpu,_r:*const sys_reg_desc,val:u64)->i32 { let mut v=core::mem::zeroed();vgic_get_vmcr(vcpu,&mut v);v.grpen1=FIELD_GET(ICC_IGRPEN1_EL1_MASK,val);vgic_set_vmcr(vcpu,&v);0 }
unsafe fn get_gic_grpen1(vcpu:*mut kvm_vcpu,_r:*const sys_reg_desc,val:*mut u64)->i32 { let mut v=core::mem::zeroed();vgic_get_vmcr(vcpu,&mut v);*val=FIELD_GET(ICC_IGRPEN1_EL1_MASK,v.grpen1);0 }

unsafe fn set_apr_reg(vcpu:*mut kvm_vcpu,val:u64,apr:u8,idx:u8) { let v=&mut (*vcpu).arch.vgic_cpu.vgic_v3; if apr!=0 {v.vgic_ap1r[idx as usize]=val;} else {v.vgic_ap0r[idx as usize]=val;} }
unsafe fn get_apr_reg(vcpu:*mut kvm_vcpu,apr:u8,idx:u8)->u64 { let v=&(*vcpu).arch.vgic_cpu.vgic_v3; if apr!=0 {v.vgic_ap1r[idx as usize]} else {v.vgic_ap0r[idx as usize]} }
unsafe fn set_gic_ap0r(vcpu:*mut kvm_vcpu,r:*const sys_reg_desc,val:u64)->i32 {let idx=(*r).Op2&3;if idx>vgic_v3_max_apr_idx(vcpu){return -EINVAL;}set_apr_reg(vcpu,val,0,idx);0}
unsafe fn get_gic_ap0r(vcpu:*mut kvm_vcpu,r:*const sys_reg_desc,val:*mut u64)->i32 {let idx=(*r).Op2&3;if idx>vgic_v3_max_apr_idx(vcpu){return -EINVAL;}*val=get_apr_reg(vcpu,0,idx);0}
unsafe fn set_gic_ap1r(vcpu:*mut kvm_vcpu,r:*const sys_reg_desc,val:u64)->i32 {let idx=(*r).Op2&3;if idx>vgic_v3_max_apr_idx(vcpu){return -EINVAL;}set_apr_reg(vcpu,val,1,idx);0}
unsafe fn get_gic_ap1r(vcpu:*mut kvm_vcpu,r:*const sys_reg_desc,val:*mut u64)->i32 {let idx=(*r).Op2&3;if idx>vgic_v3_max_apr_idx(vcpu){return -EINVAL;}*val=get_apr_reg(vcpu,1,idx);0}

unsafe fn set_gic_sre(_vcpu:*mut kvm_vcpu,_r:*const sys_reg_desc,val:u64)->i32 {if val&ICC_SRE_EL1_SRE==0{-EINVAL}else{0}}
unsafe fn get_gic_sre(vcpu:*mut kvm_vcpu,_r:*const sys_reg_desc,val:*mut u64)->i32 {*val=(*vcpu).arch.vgic_cpu.vgic_v3.vgic_sre;0}
unsafe fn set_gic_ich_reg(vcpu:*mut kvm_vcpu,r:*const sys_reg_desc,val:u64)->i32 {__vcpu_assign_sys_reg(vcpu,(*r).reg,val);0}
unsafe fn get_gic_ich_reg(vcpu:*mut kvm_vcpu,r:*const sys_reg_desc,val:*mut u64)->i32 {*val=__vcpu_sys_reg(vcpu,(*r).reg);0}
unsafe fn set_gic_ich_apr(vcpu:*mut kvm_vcpu,r:*const sys_reg_desc,val:u64)->i32 {let idx=(*r).Op2&3;if idx>vgic_v3_max_apr_idx(vcpu){return -EINVAL;}set_gic_ich_reg(vcpu,r,val)}
unsafe fn get_gic_ich_apr(vcpu:*mut kvm_vcpu,r:*const sys_reg_desc,val:*mut u64)->i32 {let idx=(*r).Op2&3;if idx>vgic_v3_max_apr_idx(vcpu){return -EINVAL;}get_gic_ich_reg(vcpu,r,val)}
unsafe fn set_gic_icc_sre(_vcpu:*mut kvm_vcpu,_r:*const sys_reg_desc,val:u64)->i32 {if val!=KVM_ICC_SRE_EL2{-EINVAL}else{0}}
unsafe fn get_gic_icc_sre(_vcpu:*mut kvm_vcpu,_r:*const sys_reg_desc,val:*mut u64)->i32 {*val=KVM_ICC_SRE_EL2;0}
unsafe fn set_gic_ich_vtr(_vcpu:*mut kvm_vcpu,_r:*const sys_reg_desc,val:u64)->i32 {if val!=kvm_get_guest_vtr_el2(){-EINVAL}else{0}}
unsafe fn get_gic_ich_vtr(_vcpu:*mut kvm_vcpu,_r:*const sys_reg_desc,val:*mut u64)->i32 {*val=kvm_get_guest_vtr_el2();0}

unsafe fn el2_visibility(vcpu:*const kvm_vcpu,_rd:*const sys_reg_desc)->u32 {if vcpu_has_nv(vcpu){0}else{REG_HIDDEN}}

// C descriptor macros are retained as Rust construction macros; register constants and callbacks are external.
macro_rules! __EL2_REG { ($r:ident,$acc:ident,$i:expr) => { sys_reg_desc { desc: SYS_DESC!(SYS_$r), get_user: Some(get_gic_$acc), set_user: Some(set_gic_$acc), reg: $i, visibility: Some(el2_visibility) } }; }
macro_rules! EL2_REG { ($r:ident,$acc:ident) => { __EL2_REG!($r,$acc,SYS_$r) }; }
macro_rules! EL2_REG_RO { ($r:ident,$acc:ident) => { __EL2_REG!($r,$acc,0) }; }

static mut gic_v3_icc_reg_descs: [sys_reg_desc; 43] = [
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_PMR_EL1), set_user:Some(set_gic_pmr), get_user:Some(get_gic_pmr) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_BPR0_EL1), set_user:Some(set_gic_bpr0), get_user:Some(get_gic_bpr0) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_AP0R0_EL1), set_user:Some(set_gic_ap0r), get_user:Some(get_gic_ap0r) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_AP0R1_EL1), set_user:Some(set_gic_ap0r), get_user:Some(get_gic_ap0r) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_AP0R2_EL1), set_user:Some(set_gic_ap0r), get_user:Some(get_gic_ap0r) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_AP0R3_EL1), set_user:Some(set_gic_ap0r), get_user:Some(get_gic_ap0r) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_AP1R0_EL1), set_user:Some(set_gic_ap1r), get_user:Some(get_gic_ap1r) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_AP1R1_EL1), set_user:Some(set_gic_ap1r), get_user:Some(get_gic_ap1r) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_AP1R2_EL1), set_user:Some(set_gic_ap1r), get_user:Some(get_gic_ap1r) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_AP1R3_EL1), set_user:Some(set_gic_ap1r), get_user:Some(get_gic_ap1r) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_BPR1_EL1), set_user:Some(set_gic_bpr1), get_user:Some(get_gic_bpr1) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_CTLR_EL1), set_user:Some(set_gic_ctlr), get_user:Some(get_gic_ctlr) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_SRE_EL1), set_user:Some(set_gic_sre), get_user:Some(get_gic_sre) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_IGRPEN0_EL1), set_user:Some(set_gic_grpen0), get_user:Some(get_gic_grpen0) },
    sys_reg_desc { desc: SYS_DESC!(SYS_ICC_IGRPEN1_EL1), set_user:Some(set_gic_grpen1), get_user:Some(get_gic_grpen1) },
    EL2_REG!(ICH_AP0R0_EL2, ich_apr), EL2_REG!(ICH_AP0R1_EL2, ich_apr), EL2_REG!(ICH_AP0R2_EL2, ich_apr), EL2_REG!(ICH_AP0R3_EL2, ich_apr),
    EL2_REG!(ICH_AP1R0_EL2, ich_apr), EL2_REG!(ICH_AP1R1_EL2, ich_apr), EL2_REG!(ICH_AP1R2_EL2, ich_apr), EL2_REG!(ICH_AP1R3_EL2, ich_apr),
    EL2_REG_RO!(ICC_SRE_EL2, icc_sre), EL2_REG!(ICH_HCR_EL2, ich_reg), EL2_REG_RO!(ICH_VTR_EL2, ich_vtr), EL2_REG!(ICH_VMCR_EL2, ich_reg),
    EL2_REG!(ICH_LR0_EL2, ich_reg), EL2_REG!(ICH_LR1_EL2, ich_reg), EL2_REG!(ICH_LR2_EL2, ich_reg), EL2_REG!(ICH_LR3_EL2, ich_reg),
    EL2_REG!(ICH_LR4_EL2, ich_reg), EL2_REG!(ICH_LR5_EL2, ich_reg), EL2_REG!(ICH_LR6_EL2, ich_reg), EL2_REG!(ICH_LR7_EL2, ich_reg),
    EL2_REG!(ICH_LR8_EL2, ich_reg), EL2_REG!(ICH_LR9_EL2, ich_reg), EL2_REG!(ICH_LR10_EL2, ich_reg), EL2_REG!(ICH_LR11_EL2, ich_reg),
    EL2_REG!(ICH_LR12_EL2, ich_reg), EL2_REG!(ICH_LR13_EL2, ich_reg), EL2_REG!(ICH_LR14_EL2, ich_reg), EL2_REG!(ICH_LR15_EL2, ich_reg),
];

unsafe fn vgic_v3_get_sysreg_table(sz:*mut u32)->*const sys_reg_desc {*sz=core::mem::size_of_val(&gic_v3_icc_reg_descs) as u32/core::mem::size_of::<sys_reg_desc>() as u32;gic_v3_icc_reg_descs.as_ptr()}
unsafe fn attr_to_id(attr:u64)->u64 { ARM64_SYS_REG(FIELD_GET(KVM_REG_ARM_VGIC_SYSREG_OP0_MASK,attr),FIELD_GET(KVM_REG_ARM_VGIC_SYSREG_OP1_MASK,attr),FIELD_GET(KVM_REG_ARM_VGIC_SYSREG_CRN_MASK,attr),FIELD_GET(KVM_REG_ARM_VGIC_SYSREG_CRM_MASK,attr),FIELD_GET(KVM_REG_ARM_VGIC_SYSREG_OP2_MASK,attr)) }
unsafe fn vgic_v3_has_cpu_sysregs_attr(vcpu:*mut kvm_vcpu,attr:*mut kvm_device_attr)->i32 {let r=get_reg_by_id(attr_to_id((*attr).attr),gic_v3_icc_reg_descs.as_ptr(),gic_v3_icc_reg_descs.len());if !r.is_null()&&!sysreg_hidden(vcpu,r){0}else{-ENXIO}}
unsafe fn vgic_v3_cpu_sysregs_uaccess(vcpu:*mut kvm_vcpu,attr:*mut kvm_device_attr,is_write:bool)->i32 {let reg=kvm_one_reg{id:attr_to_id((*attr).attr),addr:(*attr).addr};if is_write{kvm_sys_reg_set_user(vcpu,&reg,gic_v3_icc_reg_descs.as_ptr(),gic_v3_icc_reg_descs.len())}else{kvm_sys_reg_get_user(vcpu,&reg,gic_v3_icc_reg_descs.as_ptr(),gic_v3_icc_reg_descs.len())}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
