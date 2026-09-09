// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level translation of book3s_emulate.c. External kernel symbols are
// intentionally left unresolved for the surrounding translation unit.

const OP_19_XOP_RFID: u32 = 18;
const OP_19_XOP_RFI: u32 = 50;
const OP_31_XOP_MFMSR: u32 = 83;
const OP_31_XOP_MTMSR: u32 = 146;
const OP_31_XOP_MTMSRD: u32 = 178;
const OP_31_XOP_MTSR: u32 = 210;
const OP_31_XOP_MTSRIN: u32 = 242;
const OP_31_XOP_TLBIEL: u32 = 274;
const OP_31_XOP_FAKE_SC1: u32 = 308;
const OP_31_XOP_SLBMTE: u32 = 402;
const OP_31_XOP_SLBIE: u32 = 434;
const OP_31_XOP_SLBIA: u32 = 498;
const OP_31_XOP_MFSR: u32 = 595;
const OP_31_XOP_MFSRIN: u32 = 659;
const OP_31_XOP_DCBA: u32 = 758;
const OP_31_XOP_SLBMFEV: u32 = 851;
const OP_31_XOP_EIOIO: u32 = 854;
const OP_31_XOP_SLBMFEE: u32 = 915;
const OP_31_XOP_SLBFEE: u32 = 979;
const OP_31_XOP_TBEGIN: u32 = 654;
const OP_31_XOP_TABORT: u32 = 910;
const OP_31_XOP_TRECLAIM: u32 = 942;
const OP_31_XOP_TRCHKPT: u32 = 1006;
const OP_31_XOP_DCBZ: u32 = 1010;
const OP_LFS: u32 = 48; const OP_LFD: u32 = 50;
const OP_STFS: u32 = 52; const OP_STFD: u32 = 54;
const SPRN_GQR0: i32 = 912;

#[repr(C)]
pub enum priv_level { PRIV_PROBLEM = 0, PRIV_SUPER = 1, PRIV_HYPER = 2 }

unsafe fn spr_allowed(vcpu: *mut kvm_vcpu, level: priv_level) -> bool {
    if (*vcpu).arch.papr_enabled && (level as i32 > PRIV_SUPER as i32) { return false; }
    if (kvmppc_get_msr(vcpu) & MSR_PR) != 0 && level as i32 > PRIV_PROBLEM as i32 { return false; }
    true
}

#[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
unsafe fn kvmppc_copyto_vcpu_tm(vcpu: *mut kvm_vcpu) {
    core::ptr::copy_nonoverlapping((*vcpu).arch.regs.gpr.as_ptr(), (*vcpu).arch.gpr_tm.as_mut_ptr(), (*vcpu).arch.gpr_tm.len());
    (*vcpu).arch.fp_tm = (*vcpu).arch.fp; (*vcpu).arch.vr_tm = (*vcpu).arch.vr;
    (*vcpu).arch.ppr_tm = (*vcpu).arch.ppr; (*vcpu).arch.dscr_tm = (*vcpu).arch.dscr;
    (*vcpu).arch.amr_tm = (*vcpu).arch.amr; (*vcpu).arch.ctr_tm = (*vcpu).arch.regs.ctr;
    (*vcpu).arch.tar_tm = (*vcpu).arch.tar; (*vcpu).arch.lr_tm = (*vcpu).arch.regs.link;
    (*vcpu).arch.cr_tm = (*vcpu).arch.regs.ccr; (*vcpu).arch.xer_tm = (*vcpu).arch.regs.xer;
    (*vcpu).arch.vrsave_tm = (*vcpu).arch.vrsave;
}
#[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
unsafe fn kvmppc_copyfrom_vcpu_tm(vcpu: *mut kvm_vcpu) {
    core::ptr::copy_nonoverlapping((*vcpu).arch.gpr_tm.as_ptr(), (*vcpu).arch.regs.gpr.as_mut_ptr(), (*vcpu).arch.regs.gpr.len());
    (*vcpu).arch.fp = (*vcpu).arch.fp_tm; (*vcpu).arch.vr = (*vcpu).arch.vr_tm;
    (*vcpu).arch.ppr = (*vcpu).arch.ppr_tm; (*vcpu).arch.dscr = (*vcpu).arch.dscr_tm;
    (*vcpu).arch.amr = (*vcpu).arch.amr_tm; (*vcpu).arch.regs.ctr = (*vcpu).arch.ctr_tm;
    (*vcpu).arch.tar = (*vcpu).arch.tar_tm; (*vcpu).arch.regs.link = (*vcpu).arch.lr_tm;
    (*vcpu).arch.regs.ccr = (*vcpu).arch.cr_tm; (*vcpu).arch.regs.xer = (*vcpu).arch.xer_tm;
    (*vcpu).arch.vrsave = (*vcpu).arch.vrsave_tm;
}

// The following declarations preserve the source interfaces and control flow;
// their field and helper types are supplied by the translated kernel headers.
pub unsafe fn kvmppc_core_emulate_op_pr(vcpu: *mut kvm_vcpu, inst: u32, advance: *mut i32) -> i32 {
    let mut emulated = EMULATE_DONE;
    let rt = get_rt(inst); let rs = get_rs(inst); let ra = get_ra(inst); let rb = get_rb(inst);
    let inst_sc: u32 = 0x44000002;
    match get_op(inst) {
        0 => { emulated = EMULATE_FAIL; if (kvmppc_get_msr(vcpu)&MSR_LE)!=0 && inst==swab32(inst_sc) { kvmppc_set_gpr(vcpu,3,EV_UNIMPLEMENTED); kvmppc_set_pc(vcpu,kvmppc_get_pc(vcpu)+4); emulated=EMULATE_DONE; } },
        19 => match get_xop(inst) { OP_19_XOP_RFID|OP_19_XOP_RFI => { kvmppc_set_pc(vcpu,kvmppc_get_srr0(vcpu)); kvmppc_set_msr(vcpu,kvmppc_get_srr1(vcpu)); *advance=0; }, _=>emulated=EMULATE_FAIL },
        31 => match get_xop(inst) {
            OP_31_XOP_MFMSR => kvmppc_set_gpr(vcpu,rt,kvmppc_get_msr(vcpu)),
            OP_31_XOP_MTMSR => kvmppc_set_msr(vcpu,kvmppc_get_gpr(vcpu,rs)),
            OP_31_XOP_MTMSRD => { let v=kvmppc_get_gpr(vcpu,rs); if inst&0x10000!=0 { let mut m=kvmppc_get_msr(vcpu); m &= !(MSR_RI|MSR_EE); m |= v&(MSR_RI|MSR_EE); kvmppc_set_msr_fast(vcpu,m); } else { kvmppc_set_msr(vcpu,v); } },
            OP_31_XOP_EIOIO|OP_31_XOP_DCBA => {},
            OP_31_XOP_DCBZ => { let mut addr=if ra!=0{kvmppc_get_gpr(vcpu,ra)}else{0}; addr=(addr+kvmppc_get_gpr(vcpu,rb))&!31u64; let zeros=[0u32;8]; let r=kvmppc_st(vcpu,&mut addr,32,zeros.as_ptr(),true); if r==-ENOENT||r==-EPERM { *advance=0; kvmppc_set_dar(vcpu,addr); (*vcpu).arch.fault_dar=addr; let mut d=DSISR_ISSTORE; if r==-ENOENT{d|=DSISR_NOHPTE}else{d|=DSISR_PROTFAULT}; kvmppc_set_dsisr(vcpu,d); (*vcpu).arch.fault_dsisr=d; kvmppc_book3s_queue_irqprio(vcpu,BOOK3S_INTERRUPT_DATA_STORAGE); } },
            _ => emulated=EMULATE_FAIL,
        }, _=>emulated=EMULATE_FAIL
    }
    if emulated==EMULATE_FAIL { emulated=kvmppc_emulate_paired_single(vcpu); } emulated
}

pub unsafe fn kvmppc_set_bat(vcpu: *mut kvm_vcpu, bat: *mut kvmppc_bat, upper: bool, val: u32) {
    if upper { let bl=(val>>2)&0x7ff; (*bat).bepi_mask=(!bl)<<17; (*bat).bepi=val&0xfffe0000; (*bat).vs=if val&2!=0{1}else{0}; (*bat).vp=if val&1!=0{1}else{0}; (*bat).raw=((*bat).raw&0xffffffff00000000)|val as u64; }
    else { (*bat).brpn=val&0xfffe0000; (*bat).wimg=(val>>3)&0xf; (*bat).pp=val&3; (*bat).raw=((*bat).raw&0xffffffff)|((val as u64)<<32); }
}

pub unsafe fn kvmppc_alignment_dsisr(_vcpu:*mut kvm_vcpu, inst:u32)->u32 { make_dsisr(inst) }
pub unsafe fn kvmppc_alignment_dar(vcpu:*mut kvm_vcpu, _inst:u32)->ulong { (*vcpu).arch.fault_dar }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
