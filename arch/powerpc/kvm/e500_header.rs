/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2008-2011 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Author: Yu Liu <yu.liu@freescale.com>
 *         Scott Wood <scottwood@freescale.com>
 *         Ashish Kalra <ashish.kalra@freescale.com>
 *         Varun Sethi <varun.sethi@freescale.com>
 *
 * Description:
 * This file is based on arch/powerpc/kvm/44x_tlb.h and
 * arch/powerpc/include/asm/kvm_44x.h by Hollis Blanchard <hollisb@us.ibm.com>,
 * Copyright IBM Corp. 2007-2008
 */

// Dependencies: linux/kvm_host.h, asm/nohash/mmu-e500.h, asm/tlb.h,
// asm/cputhreads.h

#[repr(C)]
pub enum vcpu_ftr {
    VCPU_FTR_MMU_V2,
}

pub const E500_PID_NUM: usize = 3;
pub const E500_TLB_NUM: usize = 2;

pub const E500_TLB_VALID: u32 = 1 << 31;
pub const E500_TLB_BITMAP: u32 = 1 << 30;
pub const E500_TLB_TLB0: u32 = 1 << 29;
pub const E500_TLB_WRITABLE: u32 = 1 << 28;
pub const E500_TLB_MAS2_ATTR: u32 = 0x7f;

#[repr(C)]
pub struct tlbe_priv {
    pub pfn: kvm_pfn_t,
    pub flags: u32,
}

#[repr(C)]
pub struct kvmppc_e500_tlb_params {
    pub entries: i32,
    pub ways: i32,
    pub sets: i32,
}

#[repr(C)]
pub struct kvmppc_vcpu_e500 {
    pub vcpu: kvm_vcpu,
    pub gtlb_arch: *mut kvm_book3e_206_tlb_entry,
    pub gtlb_offset: [i32; E500_TLB_NUM],
    pub gtlb_priv: [*mut tlbe_priv; E500_TLB_NUM],
    pub gtlb_params: [kvmppc_e500_tlb_params; E500_TLB_NUM],
    pub gtlb_nv: [u32; E500_TLB_NUM],
    pub host_tlb1_nv: u32,
    pub svr: u32,
    pub l1csr0: u32,
    pub l1csr1: u32,
    pub hid0: u32,
    pub hid1: u32,
    pub mcar: u64,
    pub shared_tlb_pages: *mut *mut page,
    pub num_shared_tlb_pages: i32,
    pub g2h_tlb1_map: *mut u64,
    pub h2g_tlb1_rmap: *mut u32,
    pub tlb1_min_eaddr: c_ulong,
    pub tlb1_max_eaddr: c_ulong,
    #[cfg(CONFIG_KVM_E500V2)]
    pub pid: [u32; E500_PID_NUM],
    #[cfg(CONFIG_KVM_E500V2)]
    pub idt: *mut vcpu_id_table,
}

pub const KVM_E500_TLB0_WAY_SIZE: i32 = 128;
pub const KVM_E500_TLB0_WAY_NUM: i32 = 2;
pub const KVM_E500_TLB0_SIZE: i32 = KVM_E500_TLB0_WAY_SIZE * KVM_E500_TLB0_WAY_NUM;
pub const KVM_E500_TLB1_SIZE: i32 = 16;

#[inline]
pub const fn index_of(tlbsel: i32, esel: i32) -> i32 { (tlbsel << 16) | (esel & 0xFFFF) }
#[inline]
pub const fn tlbsel_of(index: i32) -> i32 { index >> 16 }
#[inline]
pub const fn esel_of(index: i32) -> i32 { index & 0xFFFF }

pub const E500_TLB_USER_PERM_MASK: u32 = MAS3_UX | MAS3_UR | MAS3_UW;
pub const E500_TLB_SUPER_PERM_MASK: u32 = MAS3_SX | MAS3_SR | MAS3_SW;
pub const MAS2_ATTRIB_MASK: u32 = MAS2_X0 | MAS2_X1 | MAS2_E | MAS2_G;
pub const MAS3_ATTRIB_MASK: u32 = MAS3_U0 | MAS3_U1 | MAS3_U2 | MAS3_U3 |
    E500_TLB_USER_PERM_MASK | E500_TLB_SUPER_PERM_MASK;

extern "C" {
    pub fn kvmppc_e500_emul_mt_mmucsr0(vcpu_e500: *mut kvmppc_vcpu_e500, value: c_ulong) -> i32;
    pub fn kvmppc_e500_emul_tlbwe(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvmppc_e500_emul_tlbre(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvmppc_e500_emul_tlbivax(vcpu: *mut kvm_vcpu, ea: gva_t) -> i32;
    pub fn kvmppc_e500_emul_tlbilx(vcpu: *mut kvm_vcpu, type_: i32, ea: gva_t) -> i32;
    pub fn kvmppc_e500_emul_tlbsx(vcpu: *mut kvm_vcpu, ea: gva_t) -> i32;
    pub fn kvmppc_e500_tlb_init(vcpu_e500: *mut kvmppc_vcpu_e500) -> i32;
    pub fn kvmppc_e500_tlb_uninit(vcpu_e500: *mut kvmppc_vcpu_e500);
    pub fn kvmppc_get_sregs_e500_tlb(vcpu: *mut kvm_vcpu, sregs: *mut kvm_sregs);
    pub fn kvmppc_set_sregs_e500_tlb(vcpu: *mut kvm_vcpu, sregs: *mut kvm_sregs) -> i32;
    pub fn kvmppc_get_one_reg_e500_tlb(vcpu: *mut kvm_vcpu, id: u64, val: *mut kvmppc_one_reg) -> i32;
    pub fn kvmppc_set_one_reg_e500_tlb(vcpu: *mut kvm_vcpu, id: u64, val: *mut kvmppc_one_reg) -> i32;
    pub fn kvmppc_e500_tlbil_one(vcpu_e500: *mut kvmppc_vcpu_e500, gtlbe: *mut kvm_book3e_206_tlb_entry);
    pub fn kvmppc_e500_tlbil_all(vcpu_e500: *mut kvmppc_vcpu_e500);
}

#[inline]
pub unsafe fn get_tlb_size(tlbe: *const kvm_book3e_206_tlb_entry) -> u32 { ((*tlbe).mas1 >> 7) & 0x1f }
#[inline]
pub unsafe fn get_tlb_eaddr(tlbe: *const kvm_book3e_206_tlb_entry) -> gva_t { (*tlbe).mas2 & MAS2_EPN }
#[inline]
pub unsafe fn get_tlb_bytes(tlbe: *const kvm_book3e_206_tlb_entry) -> u64 { 1u64 << 10 << get_tlb_size(tlbe) }
#[inline]
pub unsafe fn get_tlb_end(tlbe: *const kvm_book3e_206_tlb_entry) -> gva_t { get_tlb_eaddr(tlbe) + get_tlb_bytes(tlbe) - 1 }
#[inline]
pub unsafe fn get_tlb_raddr(tlbe: *const kvm_book3e_206_tlb_entry) -> u64 { (*tlbe).mas7_3 & !0xfff_u64 }
#[inline]
pub unsafe fn get_tlb_tid(tlbe: *const kvm_book3e_206_tlb_entry) -> u32 { ((*tlbe).mas1 >> 16) & 0xff }
#[inline]
pub unsafe fn get_tlb_ts(tlbe: *const kvm_book3e_206_tlb_entry) -> u32 { ((*tlbe).mas1 >> 12) & 1 }
#[inline]
pub unsafe fn get_tlb_v(tlbe: *const kvm_book3e_206_tlb_entry) -> u32 { ((*tlbe).mas1 >> 31) & 1 }
#[inline]
pub unsafe fn get_tlb_iprot(tlbe: *const kvm_book3e_206_tlb_entry) -> u32 { ((*tlbe).mas1 >> 30) & 1 }
#[inline]
pub unsafe fn get_tlb_tsize(tlbe: *const kvm_book3e_206_tlb_entry) -> u32 { ((*tlbe).mas1 & MAS1_TSIZE_MASK) >> MAS1_TSIZE_SHIFT }

#[inline]
pub unsafe fn get_cur_pid(vcpu: *const kvm_vcpu) -> u32 { (*vcpu).arch.pid & 0xff }
#[inline]
pub unsafe fn get_cur_as(vcpu: *const kvm_vcpu) -> u32 { ((*vcpu).arch.shared.as_ref().unwrap().msr & (MSR_IS | MSR_DS) != 0) as u32 }
#[inline]
pub unsafe fn get_cur_pr(vcpu: *const kvm_vcpu) -> u32 { ((*vcpu).arch.shared.as_ref().unwrap().msr & MSR_PR != 0) as u32 }
#[inline]
pub unsafe fn get_cur_spid(vcpu: *const kvm_vcpu) -> u32 { ((*vcpu).arch.shared.as_ref().unwrap().mas6 >> 16) & 0xff }
#[inline]
pub unsafe fn get_cur_sas(vcpu: *const kvm_vcpu) -> u32 { (*vcpu).arch.shared.as_ref().unwrap().mas6 & 1 }
#[inline]
pub unsafe fn get_tlb_tlbsel(vcpu: *const kvm_vcpu) -> u32 {
    /* Manual says that tlbsel has 2 bits wide. Since we only have two TLBs, only lower bit is used. */
    ((*vcpu).arch.shared.as_ref().unwrap().mas0 >> 28) & 1
}
#[inline]
pub unsafe fn get_tlb_nv_bit(vcpu: *const kvm_vcpu) -> u32 { (*vcpu).arch.shared.as_ref().unwrap().mas0 & 0xfff }
#[inline]
pub unsafe fn get_tlb_esel_bit(vcpu: *const kvm_vcpu) -> u32 { ((*vcpu).arch.shared.as_ref().unwrap().mas0 >> 16) & 0xfff }

#[inline]
pub unsafe fn tlbe_is_host_safe(vcpu: *const kvm_vcpu, tlbe: *const kvm_book3e_206_tlb_entry) -> i32 {
    if get_tlb_v(tlbe) == 0 { return 0; }
    // Without CONFIG_KVM_BOOKE_HV, match the current guest address space.
    #[cfg(not(CONFIG_KVM_BOOKE_HV))]
    if get_tlb_ts(tlbe) != ((*vcpu).arch.shared.as_ref().unwrap().msr & MSR_IS != 0) as u32 { return 0; }
    let gpa = get_tlb_raddr(tlbe);
    if gfn_to_memslot((*vcpu).kvm, gpa >> PAGE_SHIFT).is_null() { return 0; }
    1
}

#[cfg(CONFIG_KVM_E500V2)]
extern "C" {
    pub fn kvmppc_e500_get_sid(vcpu_e500: *mut kvmppc_vcpu_e500, as_: u32, gid: u32, pr: u32, avoid_recursion: i32) -> u32;
}

#[cfg(not(CONFIG_KVM_BOOKE_HV))]
extern "C" {
    pub fn kvmppc_e500_get_tlb_stid(vcpu: *mut kvm_vcpu, gtlbe: *mut kvm_book3e_206_tlb_entry) -> u32;
}

#[cfg(CONFIG_KVM_BOOKE_HV)]
#[inline]
pub unsafe fn get_tlb_stid(_vcpu: *mut kvm_vcpu, gtlbe: *const kvm_book3e_206_tlb_entry) -> u32 { get_tlb_tid(gtlbe) }

#[cfg(CONFIG_KVM_BOOKE_HV)]
#[inline]
pub unsafe fn get_tlbmiss_tid(vcpu: *mut kvm_vcpu) -> u32 { get_cur_pid(vcpu) }

#[cfg(not(CONFIG_KVM_BOOKE_HV))]
#[inline]
pub unsafe fn get_tlbmiss_tid(vcpu: *mut kvm_vcpu) -> u32 {
    let vcpu_e500 = to_e500(vcpu);
    let tidseld = ((*vcpu).arch.shared.as_ref().unwrap().mas4 >> 16) & 0xf;
    (*vcpu_e500).pid[tidseld as usize]
}

#[cfg(CONFIG_KVM_BOOKE_HV)]
#[inline]
pub unsafe fn get_tlb_sts(gtlbe: *const kvm_book3e_206_tlb_entry) -> u32 { (*gtlbe).mas1 & MAS1_TS }

#[cfg(not(CONFIG_KVM_BOOKE_HV))]
#[inline]
pub const fn get_tlb_sts(_gtlbe: *const kvm_book3e_206_tlb_entry) -> u32 { MAS1_TS }

#[cfg(CONFIG_KVM_BOOKE_HV)]
#[inline]
pub unsafe fn get_thread_specific_lpid(vm_lpid: i32) -> i32 {
    let mut vcpu_lpid = vm_lpid;
    if threads_per_core == 2 { vcpu_lpid |= smp_processor_id() & 1; }
    vcpu_lpid
}

#[cfg(CONFIG_KVM_BOOKE_HV)]
#[inline]
pub unsafe fn get_lpid(vcpu: *mut kvm_vcpu) -> i32 {
    get_thread_specific_lpid((*vcpu).kvm.as_ref().unwrap().arch.lpid)
}

#[inline]
pub unsafe fn to_e500(vcpu: *mut kvm_vcpu) -> *mut kvmppc_vcpu_e500 {
    container_of!(vcpu, kvmppc_vcpu_e500, vcpu)
}

// Conditional configuration-specific declarations and helper semantics are preserved below.
#[inline]
pub unsafe fn get_entry(vcpu_e500: *mut kvmppc_vcpu_e500, tlbsel: i32, entry: i32) -> *mut kvm_book3e_206_tlb_entry {
    let offset = (*vcpu_e500).gtlb_offset[tlbsel as usize];
    (*vcpu_e500).gtlb_arch.offset((offset + entry) as isize)
}

// CONFIG_KVM_BOOKE_HV: get_tlb_stid(vcpu, gtlbe) = get_tlb_tid(gtlbe);
// CONFIG_KVM_BOOKE_HV: get_tlbmiss_tid(vcpu) = get_cur_pid(vcpu);
// Non-HV builds provide kvmppc_e500_get_tlb_stid externally and force get_tlb_sts to MAS1_TS.
#[inline]
pub unsafe fn has_feature(vcpu: *const kvm_vcpu, ftr: vcpu_ftr) -> bool {
    match ftr {
        vcpu_ftr::VCPU_FTR_MMU_V2 => ((*vcpu).arch.mmucfg & MMUCFG_MAVN) == MMUCFG_MAVN_V2,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
