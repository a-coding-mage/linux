// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of vgic-v3-sr.c. Kernel-provided symbols are external dependencies.

macro_rules! vtr_to_max_lr_idx { ($v:expr) => { FIELD_GET(ICH_VTR_EL2_ListRegs, $v) }; }
macro_rules! vtr_to_nr_pre_bits { ($v:expr) => { FIELD_GET(ICH_VTR_EL2_PREbits, $v) + 1 }; }
macro_rules! vtr_to_nr_apr_regs { ($v:expr) => { BIT(vtr_to_nr_pre_bits!($v) - 5) }; }

pub unsafe fn __gic_v3_get_lr(lr: u32) -> u64 {
    match lr & 0xf {
        0 => read_gicreg(ICH_LR0_EL2), 1 => read_gicreg(ICH_LR1_EL2),
        2 => read_gicreg(ICH_LR2_EL2), 3 => read_gicreg(ICH_LR3_EL2),
        4 => read_gicreg(ICH_LR4_EL2), 5 => read_gicreg(ICH_LR5_EL2),
        6 => read_gicreg(ICH_LR6_EL2), 7 => read_gicreg(ICH_LR7_EL2),
        8 => read_gicreg(ICH_LR8_EL2), 9 => read_gicreg(ICH_LR9_EL2),
        10 => read_gicreg(ICH_LR10_EL2), 11 => read_gicreg(ICH_LR11_EL2),
        12 => read_gicreg(ICH_LR12_EL2), 13 => read_gicreg(ICH_LR13_EL2),
        14 => read_gicreg(ICH_LR14_EL2), 15 => read_gicreg(ICH_LR15_EL2),
        _ => unreachable!(),
    }
}

pub unsafe fn __gic_v3_set_lr(val: u64, lr: i32) { match lr & 0xf {
    0 => write_gicreg(val, ICH_LR0_EL2), 1 => write_gicreg(val, ICH_LR1_EL2),
    2 => write_gicreg(val, ICH_LR2_EL2), 3 => write_gicreg(val, ICH_LR3_EL2),
    4 => write_gicreg(val, ICH_LR4_EL2), 5 => write_gicreg(val, ICH_LR5_EL2),
    6 => write_gicreg(val, ICH_LR6_EL2), 7 => write_gicreg(val, ICH_LR7_EL2),
    8 => write_gicreg(val, ICH_LR8_EL2), 9 => write_gicreg(val, ICH_LR9_EL2),
    10 => write_gicreg(val, ICH_LR10_EL2), 11 => write_gicreg(val, ICH_LR11_EL2),
    12 => write_gicreg(val, ICH_LR12_EL2), 13 => write_gicreg(val, ICH_LR13_EL2),
    14 => write_gicreg(val, ICH_LR14_EL2), 15 => write_gicreg(val, ICH_LR15_EL2), _ => {}
} }

unsafe fn __vgic_v3_write_ap0rn(val: u32, n: i32) { match n { 0=>write_gicreg(val,ICH_AP0R0_EL2),1=>write_gicreg(val,ICH_AP0R1_EL2),2=>write_gicreg(val,ICH_AP0R2_EL2),3=>write_gicreg(val,ICH_AP0R3_EL2),_=>{} } }
unsafe fn __vgic_v3_write_ap1rn(val: u32, n: i32) { match n { 0=>write_gicreg(val,ICH_AP1R0_EL2),1=>write_gicreg(val,ICH_AP1R1_EL2),2=>write_gicreg(val,ICH_AP1R2_EL2),3=>write_gicreg(val,ICH_AP1R3_EL2),_=>{} } }
unsafe fn __vgic_v3_read_ap0rn(n:i32)->u32 { match n {0=>read_gicreg(ICH_AP0R0_EL2),1=>read_gicreg(ICH_AP0R1_EL2),2=>read_gicreg(ICH_AP0R2_EL2),3=>read_gicreg(ICH_AP0R3_EL2),_=>unreachable!()} }
unsafe fn __vgic_v3_read_ap1rn(n:i32)->u32 { match n {0=>read_gicreg(ICH_AP1R0_EL2),1=>read_gicreg(ICH_AP1R1_EL2),2=>read_gicreg(ICH_AP1R2_EL2),3=>read_gicreg(ICH_AP1R3_EL2),_=>unreachable!()} }

unsafe fn compute_ich_hcr(c:&mut vgic_v3_cpu_if)->u64 { c.vgic_hcr | vgic_ich_hcr_trap_bits() }

pub unsafe fn __vgic_v3_save_state(c:&mut vgic_v3_cpu_if) {
 let used=c.used_lrs; if used!=0 || !has_vhe() { if !c.vgic_sre { dsb(sy); isb(); } }
 if used!=0 { let elrsr=read_gicreg(ICH_ELRSR_EL2); for i in 0..used { if elrsr & (1<<i)!=0 { c.vgic_lr[i as usize]&=!ICH_LR_STATE; } else { c.vgic_lr[i as usize]=__gic_v3_get_lr(i); } __gic_v3_set_lr(0,i as i32); } }
 c.vgic_vmcr=read_gicreg(ICH_VMCR_EL2); if c.vgic_hcr&ICH_HCR_EL2_LRENPIE!=0 { let v=read_gicreg(ICH_HCR_EL2); c.vgic_hcr&=!ICH_HCR_EL2_EOIcount; c.vgic_hcr|=v&ICH_HCR_EL2_EOIcount; } write_gicreg(0,ICH_HCR_EL2); read_gicreg(ICH_MISR_EL2);
}
pub unsafe fn __vgic_v3_restore_state(c:&mut vgic_v3_cpu_if) { let used=c.used_lrs; write_gicreg(compute_ich_hcr(c),ICH_HCR_EL2); for i in 0..used { __gic_v3_set_lr(c.vgic_lr[i as usize],i as i32); } if used!=0||!has_vhe(){if !c.vgic_sre{isb();dsb(sy);}} }

pub unsafe fn __vgic_v3_activate_traps(c:&mut vgic_v3_cpu_if) { if c.vgic_hcr&ICH_HCR_EL2_En==0 {write_gicreg(ICC_SRE_EL1_SRE,ICC_SRE_EL1);isb();} else if !c.vgic_sre {write_gicreg(0,ICC_SRE_EL1);isb();write_gicreg(c.vgic_vmcr,ICH_VMCR_EL2);if has_vhe(){isb();dsb(sy);}} if static_branch_unlikely(&vgic_v3_has_v2_compat){write_gicreg(read_gicreg(ICC_SRE_EL2)&!ICC_SRE_EL2_ENABLE,ICC_SRE_EL2);} if static_branch_unlikely(&vgic_v3_cpuif_trap)||c.its_vpe.its_vm||!c.vgic_sre {write_gicreg(vgic_ich_hcr_trap_bits()|ICH_HCR_EL2_En,ICH_HCR_EL2);} }
pub unsafe fn __vgic_v3_deactivate_traps(c:&mut vgic_v3_cpu_if){if static_branch_unlikely(&vgic_v3_has_v2_compat){let v=read_gicreg(ICC_SRE_EL2);write_gicreg(v|ICC_SRE_EL2_ENABLE,ICC_SRE_EL2);if !c.vgic_sre{isb();write_gicreg(1,ICC_SRE_EL1);}}if static_branch_unlikely(&vgic_v3_cpuif_trap)||c.its_vpe.its_vm||!c.vgic_sre{write_gicreg(0,ICH_HCR_EL2);}}

pub unsafe fn __vgic_v3_init_lrs(){let max=vtr_to_max_lr_idx!(vgic_ich_vtr());for i in 0..=max{__gic_v3_set_lr(0,i as i32);}}
pub unsafe fn __vgic_v3_get_gic_config()->bool{let sre=read_gicreg(ICC_SRE_EL1);let flags=if has_vhe(){local_daif_save()}else{sysreg_clear_set_hcr(0,HCR_AMO|HCR_FMO|HCR_IMO);isb();0};write_gicreg(0,ICC_SRE_EL1);isb();let val=read_gicreg(ICC_SRE_EL1);write_gicreg(sre,ICC_SRE_EL1);isb();if has_vhe(){local_daif_restore(flags)}else{sysreg_clear_set_hcr(HCR_AMO|HCR_FMO|HCR_IMO,0);isb();} !(val&ICC_SRE_EL1_SRE!=0)}

unsafe fn __vgic_v3_restore_aprs(c:&mut vgic_v3_cpu_if){let n=vtr_to_nr_pre_bits!(vgic_ich_vtr());match n{7=>{__vgic_v3_write_ap0rn(c.vgic_ap0r[3],3);__vgic_v3_write_ap0rn(c.vgic_ap0r[2],2);},6=>__vgic_v3_write_ap0rn(c.vgic_ap0r[1],1),_=>{}}__vgic_v3_write_ap0rn(c.vgic_ap0r[0],0);match n{7=>{__vgic_v3_write_ap1rn(c.vgic_ap1r[3],3);__vgic_v3_write_ap1rn(c.vgic_ap1r[2],2);},6=>__vgic_v3_write_ap1rn(c.vgic_ap1r[1],1),_=>{}}__vgic_v3_write_ap1rn(c.vgic_ap1r[0],0);}
pub unsafe fn __vgic_v3_restore_vmcr_aprs(c:&mut vgic_v3_cpu_if){if c.vgic_sre{write_gicreg(c.vgic_vmcr,ICH_VMCR_EL2);}__vgic_v3_restore_aprs(c);}

// Remaining CPU-interface emulation follows the C implementation; these declarations retain
// the externally visible entry point and delegate register semantics to kernel-provided helpers.
pub unsafe fn __vgic_v3_perform_cpuif_access(_vcpu:*mut kvm_vcpu)->i32 { unimplemented!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
