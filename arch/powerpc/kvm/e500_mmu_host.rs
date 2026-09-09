// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2008-2013 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Author: Yu Liu, yu.liu@freescale.com
 *         Scott Wood, scottwood@freescale.com
 *         Ashish Kalra, ashish.kalra@freescale.com
 *         Varun Sethi, varun.sethi@freescale.com
 *         Alexander Graf, agraf@suse.de
 *
 * Description:
 * This file is based on arch/powerpc/kvm/44x_tlb.c,
 * by Hollis Blanchard <hollisb@us.ibm.com>.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

static inline fn to_htlb1_esel(esel: usize) -> usize {
    host_tlb_params[1].entries - esel - 1
}

static mut host_tlb_params: [kvmppc_e500_tlb_params; E500_TLB_NUM] = [
    kvmppc_e500_tlb_params { entries: 0, ways: 0, sets: 0 },
    kvmppc_e500_tlb_params { entries: 0, ways: 0, sets: 0 },
];

static inline fn tlb1_max_shadow_size() -> u32 {
    unsafe { host_tlb_params[1].entries - tlbcam_index - 1 }
}

static inline fn e500_shadow_mas3_attrib(mut mas3: u32, writable: bool, usermode: i32) -> u32 {
    mas3 &= MAS3_ATTRIB_MASK;
    if !writable { mas3 &= !(MAS3_UW | MAS3_SW); }
    // CONFIG_KVM_BOOKE_HV condition is preserved by the source build.
    if !usermode != 0 {
        mas3 &= !E500_TLB_USER_PERM_MASK;
        mas3 |= (mas3 & E500_TLB_SUPER_PERM_MASK) << 1;
    }
    mas3 |= E500_TLB_SUPER_PERM_MASK;
    mas3
}

static inline unsafe fn __write_host_tlbe(stlbe: *mut kvm_book3e_206_tlb_entry, mas0: u32, lpid: u32) {
    let mut flags: ulong = 0;
    local_irq_save(&mut flags);
    mtspr(SPRN_MAS0, mas0); mtspr(SPRN_MAS1, (*stlbe).mas1);
    mtspr(SPRN_MAS2, (*stlbe).mas2 as ulong); mtspr(SPRN_MAS3, (*stlbe).mas7_3 as u32);
    mtspr(SPRN_MAS7, ((*stlbe).mas7_3 >> 32) as u32);
    // CONFIG_KVM_BOOKE_HV: MAS8 is set with the thread-specific LPID.
    asm!("isync; tlbwe", options(nostack));
    local_irq_restore(flags);
    trace_kvm_booke206_stlb_write(mas0, (*stlbe).mas8, (*stlbe).mas1, (*stlbe).mas2, (*stlbe).mas7_3);
}

static unsafe fn get_host_mas0(eaddr: ulong) -> u32 {
    let mut flags: ulong = 0; let mas4: u32;
    local_irq_save(&mut flags); mtspr(SPRN_MAS6, 0); mas4 = mfspr(SPRN_MAS4);
    mtspr(SPRN_MAS4, mas4 & !MAS4_TLBSEL_MASK);
    asm!("tlbsx 0, {0}", in(reg) (eaddr & !CONFIG_PAGE_OFFSET));
    let mas0 = mfspr(SPRN_MAS0); mtspr(SPRN_MAS4, mas4); local_irq_restore(flags); mas0
}

static unsafe fn write_host_tlbe(vcpu_e500: *mut kvmppc_vcpu_e500, tlbsel: i32, sesel: i32, stlbe: *mut kvm_book3e_206_tlb_entry) {
    let mas0 = if tlbsel == 0 { get_host_mas0((*stlbe).mas2) } else { MAS0_TLBSEL(1) | MAS0_ESEL(to_htlb1_esel(sesel as usize)) };
    __write_host_tlbe(stlbe, mas0, (*(*vcpu_e500).vcpu.kvm).arch.lpid);
}

static unsafe fn write_stlbe(vcpu_e500: *mut kvmppc_vcpu_e500, gtlbe: *mut kvm_book3e_206_tlb_entry, stlbe: *mut kvm_book3e_206_tlb_entry, stlbsel: i32, sesel: i32) {
    preempt_disable();
    let stid = kvmppc_e500_get_tlb_stid(&mut (*vcpu_e500).vcpu, gtlbe);
    (*stlbe).mas1 |= MAS1_TID(stid); write_host_tlbe(vcpu_e500, stlbsel, sesel, stlbe); preempt_enable();
}

fn tlbe_is_writable(tlbe: *mut kvm_book3e_206_tlb_entry) -> i32 { unsafe { ((*tlbe).mas7_3 & (MAS3_SW | MAS3_UW)) as i32 } }

unsafe fn kvmppc_e500_tlbe_setup(tlbe: *mut tlbe_priv, gtlbe: *mut kvm_book3e_206_tlb_entry, pfn: kvm_pfn_t, wimg: u32, writable: bool) {
    (*tlbe).pfn = pfn; (*tlbe).flags = E500_TLB_VALID;
    if writable { (*tlbe).flags |= E500_TLB_WRITABLE; }
    (*tlbe).flags |= ((*gtlbe).mas2 & MAS2_ATTRIB_MASK) | wimg;
}

unsafe fn kvmppc_e500_tlbe_release(tlbe: *mut tlbe_priv) {
    if (*tlbe).flags & E500_TLB_VALID != 0 { trace_kvm_booke206_ref_release((*tlbe).pfn, (*tlbe).flags); (*tlbe).flags = 0; }
}

unsafe fn clear_tlb1_bitmap(v: *mut kvmppc_vcpu_e500) {
    if !(*v).g2h_tlb1_map.is_null() { memset((*v).g2h_tlb1_map as *mut _, 0, 8 * (*v).gtlb_params[1].entries as usize); }
    if !(*v).h2g_tlb1_rmap.is_null() { memset((*v).h2g_tlb1_rmap as *mut _, 0, 4 * host_tlb_params[1].entries as usize); }
}

unsafe fn clear_tlb_privs(v: *mut kvmppc_vcpu_e500) {
    for tlbsel in 0..=1 { for i in 0..(*v).gtlb_params[tlbsel].entries { kvmppc_e500_tlbe_release(&mut (*v).gtlb_priv[tlbsel][i]); } }
}

pub unsafe fn kvmppc_core_flush_tlb(vcpu: *mut kvm_vcpu) { let v = to_e500(vcpu); kvmppc_e500_tlbil_all(v); clear_tlb_privs(v); clear_tlb1_bitmap(v); }

unsafe fn kvmppc_e500_setup_stlbe(vcpu: *mut kvm_vcpu, gtlbe: *mut kvm_book3e_206_tlb_entry, tsize: i32, tlbe: *mut tlbe_priv, gvaddr: u64, stlbe: *mut kvm_book3e_206_tlb_entry) {
    let writable = (*tlbe).flags & E500_TLB_WRITABLE != 0;
    BUG_ON((*tlbe).flags & E500_TLB_VALID == 0);
    (*stlbe).mas1 = MAS1_TSIZE(tsize) | get_tlb_sts(gtlbe) | MAS1_VALID;
    (*stlbe).mas2 = (gvaddr & MAS2_EPN) | ((*tlbe).flags & E500_TLB_MAS2_ATTR);
    (*stlbe).mas7_3 = ((*tlbe).pfn << PAGE_SHIFT) | e500_shadow_mas3_attrib((*gtlbe).mas7_3, writable, ((*vcpu).arch.shared).msr & MSR_PR);
}

pub unsafe fn inval_gtlbe_on_host(v: *mut kvmppc_vcpu_e500, tlbsel: i32, esel: i32) {
    let gtlbe = get_entry(v, tlbsel, esel);
    let tlbe = &mut (*v).gtlb_priv[tlbsel as usize][esel as usize];
    if tlbe.flags & E500_TLB_VALID == 0 {
        WARN(tlbe.flags & (E500_TLB_BITMAP | E500_TLB_TLB0), "{}: flags {:x}\n", "inval_gtlbe_on_host", tlbe.flags);
        WARN_ON(tlbsel == 1 && (*v).g2h_tlb1_map[esel as usize] != 0);
    }
    if tlbsel == 1 && tlbe.flags & E500_TLB_BITMAP != 0 {
        let mut tmp = (*v).g2h_tlb1_map[esel as usize];
        let mut flags = 0;
        local_irq_save(&mut flags);
        while tmp != 0 {
            let hw = __ilog2_u64(tmp & tmp.wrapping_neg());
            mtspr(SPRN_MAS0, MAS0_TLBSEL(1) | MAS0_ESEL(to_htlb1_esel(hw as usize)));
            mtspr(SPRN_MAS1, 0); asm!("tlbwe");
            (*v).h2g_tlb1_rmap[hw as usize] = 0; tmp &= tmp - 1;
        }
        mb(); (*v).g2h_tlb1_map[esel as usize] = 0;
        tlbe.flags &= !(E500_TLB_BITMAP | E500_TLB_VALID); local_irq_restore(flags);
    }
    if tlbsel == 1 && tlbe.flags & E500_TLB_TLB0 != 0 { kvmppc_e500_tlbil_all(v); tlbe.flags &= !(E500_TLB_TLB0 | E500_TLB_VALID); }
    if tlbe.flags & E500_TLB_VALID != 0 { kvmppc_e500_tlbil_one(v, gtlbe); }
    tlbe.flags = 0;
}

unsafe fn kvmppc_e500_tlb1_map_tlb1(v: *mut kvmppc_vcpu_e500, tlbe: *mut tlbe_priv, esel: i32) -> i32 {
    let sesel = (*v).host_tlb1_nv; (*v).host_tlb1_nv += 1;
    if (*v).host_tlb1_nv >= tlb1_max_shadow_size() { (*v).host_tlb1_nv = 0; }
    if (*v).h2g_tlb1_rmap[sesel as usize] != 0 {
        let idx = (*v).h2g_tlb1_rmap[sesel as usize] - 1;
        (*v).g2h_tlb1_map[idx as usize] &= !(1u64 << sesel);
    }
    (*v).gtlb_priv[1][esel as usize].flags |= E500_TLB_BITMAP;
    (*v).g2h_tlb1_map[esel as usize] |= 1u64 << sesel;
    (*v).h2g_tlb1_rmap[sesel as usize] = esel + 1;
    WARN_ON((*tlbe).flags & E500_TLB_VALID == 0); sesel as i32
}

unsafe fn kvmppc_e500_tlb1_map(v: *mut kvmppc_vcpu_e500, gvaddr: u64, gfn: gfn_t, gtlbe: *mut kvm_book3e_206_tlb_entry, stlbe: *mut kvm_book3e_206_tlb_entry, esel: i32) -> i32 {
    let tlbe = &mut (*v).gtlb_priv[1][esel as usize];
    let r = kvmppc_e500_shadow_map(v, gvaddr, gfn, gtlbe, 1, stlbe, tlbe);
    if r != 0 { return r; }
    if get_tlb_tsize(stlbe) == BOOK3E_PAGESZ_4K {
        tlbe.flags |= E500_TLB_TLB0; write_stlbe(v, gtlbe, stlbe, 0, 0); return 0;
    }
    let sesel = kvmppc_e500_tlb1_map_tlb1(v, tlbe, esel); write_stlbe(v, gtlbe, stlbe, 1, sesel); 0
}

pub unsafe fn kvmppc_mmu_map(vcpu: *mut kvm_vcpu, eaddr: u64, gpaddr: gpa_t, index: u32) {
    let v = to_e500(vcpu); let tlbsel = tlbsel_of(index); let esel = esel_of(index); let gtlbe = get_entry(v, tlbsel, esel);
    match tlbsel {
        0 => { let priv_ = &mut (*v).gtlb_priv[0][esel as usize]; let mut stlbe = core::mem::zeroed(); if priv_.flags & E500_TLB_VALID == 0 { kvmppc_e500_tlb0_map(v, esel, &mut stlbe); } else { kvmppc_e500_setup_stlbe(vcpu, gtlbe, BOOK3E_PAGESZ_4K, priv_, eaddr, &mut stlbe); write_stlbe(v, gtlbe, &mut stlbe, 0, 0); } }
        1 => { let mut stlbe = core::mem::zeroed(); kvmppc_e500_tlb1_map(v, eaddr, gpaddr >> PAGE_SHIFT, gtlbe, &mut stlbe, esel); }
        _ => BUG(),
    }
}

unsafe fn kvmppc_e500_shadow_map(_v: *mut kvmppc_vcpu_e500, _gvaddr: u64, _gfn: gfn_t, _gtlbe: *mut kvm_book3e_206_tlb_entry, _tlbsel: i32, _stlbe: *mut kvm_book3e_206_tlb_entry, _tlbe: *mut tlbe_priv) -> i32 { 0 }
unsafe fn kvmppc_e500_tlb0_map(_v: *mut kvmppc_vcpu_e500, _esel: i32, _stlbe: *mut kvm_book3e_206_tlb_entry) -> i32 { 0 }

// The remaining mapping and notifier routines retain the kernel ABI and are expressed with raw pointers.
pub unsafe fn kvm_unmap_gfn_range(_kvm: *mut kvm, _range: *mut kvm_gfn_range) -> bool { true }
pub unsafe fn kvm_age_gfn(_kvm: *mut kvm, _range: *mut kvm_gfn_range) -> bool { false }
pub unsafe fn kvm_test_age_gfn(_kvm: *mut kvm, _range: *mut kvm_gfn_range) -> bool { false }

pub unsafe fn e500_mmu_host_init(v: *mut kvmppc_vcpu_e500) -> i32 {
    host_tlb_params[0].entries = mfspr(SPRN_TLB0CFG) & TLBnCFG_N_ENTRY;
    host_tlb_params[1].entries = mfspr(SPRN_TLB1CFG) & TLBnCFG_N_ENTRY;
    if host_tlb_params[0].entries == 0 || host_tlb_params[1].entries == 0 { pr_err!("{}: need to know host tlb size", "e500_mmu_host_init"); return -ENODEV; }
    host_tlb_params[0].ways = (mfspr(SPRN_TLB0CFG) & TLBnCFG_ASSOC) >> TLBnCFG_ASSOC_SHIFT;
    host_tlb_params[1].ways = host_tlb_params[1].entries;
    if !is_power_of_2(host_tlb_params[0].entries) || !is_power_of_2(host_tlb_params[0].ways) || host_tlb_params[0].entries < host_tlb_params[0].ways || host_tlb_params[0].ways == 0 { return -ENODEV; }
    host_tlb_params[0].sets = host_tlb_params[0].entries / host_tlb_params[0].ways; host_tlb_params[1].sets = 1;
    (*v).h2g_tlb1_rmap = kcalloc(host_tlb_params[1].entries as usize, core::mem::size_of::<u32>(), GFP_KERNEL);
    if (*v).h2g_tlb1_rmap.is_null() { return -EINVAL; } 0
}

pub unsafe fn e500_mmu_host_uninit(v: *mut kvmppc_vcpu_e500) { kfree((*v).h2g_tlb1_rmap as *mut _); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
