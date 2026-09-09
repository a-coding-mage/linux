// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright SUSE Linux Products GmbH 2009
 *
 * Authors: Alexander Graf <agraf@suse.de>
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

const PTEG_FLAG_ACCESSED: u32 = 0x00000100;
const PTEG_FLAG_DIRTY: u32 = 0x00000080;
const SID_SHIFT: u32 = 28;

#[inline]
unsafe fn check_debug_ip(vcpu: *mut kvm_vcpu) -> bool {
    true
}

#[inline]
fn sr_vsid(sr_raw: u32) -> u32 {
    sr_raw & 0x0fffffff
}

#[inline]
fn sr_valid(sr_raw: u32) -> bool {
    (sr_raw & 0x80000000) == 0
}

#[inline]
fn sr_ks(sr_raw: u32) -> bool {
    (sr_raw & 0x40000000) != 0
}

#[inline]
fn sr_kp(sr_raw: u32) -> bool {
    (sr_raw & 0x20000000) != 0
}

unsafe fn find_sr(vcpu: *mut kvm_vcpu, eaddr: gva_t) -> u32 {
    kvmppc_get_sr(vcpu, (eaddr >> 28) & 0xf)
}

unsafe fn kvmppc_mmu_book3s_32_ea_to_vp(
    vcpu: *mut kvm_vcpu,
    eaddr: gva_t,
    data: bool,
) -> u64 {
    let mut vsid: u64 = 0;
    let mut pte: kvmppc_pte = core::mem::zeroed();
    if kvmppc_mmu_book3s_32_xlate_bat(vcpu, eaddr, &mut pte, data, false) == 0 {
        return pte.vpage;
    }
    kvmppc_mmu_book3s_32_esid_to_vsid(vcpu, eaddr >> SID_SHIFT, &mut vsid);
    (((eaddr as u64 >> 12) & 0xffff) | (vsid << 16))
}

unsafe fn kvmppc_mmu_book3s_32_get_pteg(
    vcpu: *mut kvm_vcpu,
    sre: u32,
    eaddr: gva_t,
    primary: bool,
) -> hva_t {
    let vcpu_book3s = to_book3s(vcpu);
    let page = (eaddr & 0x0FFFFFFF) >> 12;
    let htabmask = ((((*vcpu_book3s).sdr1 & 0x1FF) << 16) | 0xFFC0) as u64;
    let mut hash = ((sr_vsid(sre) ^ page) << 6);
    if !primary { hash = !hash; }
    hash &= htabmask as u32;
    let pteg = ((*vcpu_book3s).sdr1 & 0xffff0000) | hash;
    let r = gfn_to_hva((*vcpu).kvm, pteg >> PAGE_SHIFT);
    if kvm_is_error_hva(r) { return r; }
    r | (pteg as hva_t & !PAGE_MASK)
}

unsafe fn kvmppc_mmu_book3s_32_get_ptem(sre: u32, eaddr: gva_t, primary: bool) -> u32 {
    ((eaddr & 0x0fffffff) >> 22) | (sr_vsid(sre) << 7) |
        (if primary { 0 } else { 0x40 }) | 0x80000000
}

unsafe fn kvmppc_mmu_book3s_32_xlate_bat(
    vcpu: *mut kvm_vcpu, eaddr: gva_t, pte: *mut kvmppc_pte, data: bool, iswrite: bool,
) -> i32 {
    let vcpu_book3s = to_book3s(vcpu);
    for i in 0..8 {
        let bat = if data { &(*vcpu_book3s).dbat[i] } else { &(*vcpu_book3s).ibat[i] };
        if kvmppc_get_msr(vcpu) & MSR_PR != 0 {
            if bat.vp == 0 { continue; }
        } else if bat.vs == 0 { continue; }
        if (eaddr & bat.bepi_mask) == bat.bepi {
            let mut vsid = 0u64;
            kvmppc_mmu_book3s_32_esid_to_vsid(vcpu, eaddr >> SID_SHIFT, &mut vsid);
            (*pte).vpage = (((eaddr as u64 >> 12) & 0xffff) | (vsid << 16));
            (*pte).raddr = bat.brpn | (eaddr & !bat.bepi_mask);
            (*pte).may_read = bat.pp != 0;
            (*pte).may_write = bat.pp > 1;
            (*pte).may_execute = true;
            if !(*pte).may_read { continue; }
            if iswrite && !(*pte).may_write { continue; }
            return 0;
        }
    }
    -ENOENT
}

unsafe fn kvmppc_mmu_book3s_32_xlate_pte(
    vcpu: *mut kvm_vcpu, eaddr: gva_t, pte: *mut kvmppc_pte, data: bool,
    iswrite: bool, primary: bool,
) -> i32 {
    let sre = find_sr(vcpu, eaddr);
    (*pte).vpage = kvmppc_mmu_book3s_32_ea_to_vp(vcpu, eaddr, data);
    let ptegp = kvmppc_mmu_book3s_32_get_pteg(vcpu, sre, eaddr, primary);
    if kvm_is_error_hva(ptegp) { return -ENOENT; }
    let ptem = kvmppc_mmu_book3s_32_get_ptem(sre, eaddr, primary);
    let mut pteg: [u32; 16] = [0; 16];
    if copy_from_user(pteg.as_mut_ptr(), ptegp as *const core::ffi::c_void, core::mem::size_of_val(&pteg)) != 0 { return -ENOENT; }
    let mut found = false;
    let mut pte1 = 0u32;
    let mut index = 0usize;
    for i in (0..16).step_by(2) {
        let pte0 = be32_to_cpu(pteg[i]); pte1 = be32_to_cpu(pteg[i + 1]);
        if ptem == pte0 {
            index = i; let mut pp = (pte1 & 3) as u8;
            (*pte).raddr = (pte1 & !0xFFF) as u64 | (eaddr & 0xFFF) as u64;
            if (sr_kp(sre) && kvmppc_get_msr(vcpu) & MSR_PR != 0) || (sr_ks(sre) && kvmppc_get_msr(vcpu) & MSR_PR == 0) { pp |= 4; }
            (*pte).may_write = false; (*pte).may_read = false; (*pte).may_execute = true;
            match pp { 0 | 1 | 2 | 6 => { (*pte).may_write = true; (*pte).may_read = true; }, 3 | 5 | 7 => (*pte).may_read = true, _ => {} }
            found = true; break;
        }
    }
    if found {
        let mut pte_r = pte1;
        let addr = (ptegp + ((index + 1) * core::mem::size_of::<u32>()) as hva_t) as *mut u8;
        if (*pte).may_read && pte_r & PTEG_FLAG_ACCESSED == 0 { pte_r |= PTEG_FLAG_ACCESSED; put_user((pte_r >> 8) as u8, addr.add(2)); }
        if iswrite && (*pte).may_write && pte_r & PTEG_FLAG_DIRTY == 0 { pte_r |= PTEG_FLAG_DIRTY; put_user(pte_r as u8, addr.add(3)); }
        if !(*pte).may_read || (iswrite && !(*pte).may_write) { return -EPERM; }
        return 0;
    }
    -ENOENT
}

unsafe fn kvmppc_mmu_book3s_32_xlate(vcpu: *mut kvm_vcpu, eaddr: gva_t, pte: *mut kvmppc_pte, data: bool, iswrite: bool) -> i32 {
    (*pte).eaddr = eaddr; (*pte).page_size = MMU_PAGE_4K;
    let mp_ea = (*vcpu).arch.magic_page_ea;
    if mp_ea != 0 && (eaddr & !0xfff) == (mp_ea & !0xfff) && kvmppc_get_msr(vcpu) & MSR_PR == 0 {
        (*pte).vpage = kvmppc_mmu_book3s_32_ea_to_vp(vcpu, eaddr, data);
        (*pte).raddr = (*vcpu).arch.magic_page_pa | ((*pte).raddr & 0xfff); (*pte).raddr &= KVM_PAM;
        (*pte).may_execute = true; (*pte).may_read = true; (*pte).may_write = true; return 0;
    }
    let mut r = kvmppc_mmu_book3s_32_xlate_bat(vcpu, eaddr, pte, data, iswrite);
    if r < 0 { r = kvmppc_mmu_book3s_32_xlate_pte(vcpu, eaddr, pte, data, iswrite, true); }
    if r == -ENOENT { r = kvmppc_mmu_book3s_32_xlate_pte(vcpu, eaddr, pte, data, iswrite, false); }
    r
}

unsafe fn kvmppc_mmu_book3s_32_mfsrin(vcpu: *mut kvm_vcpu, srnum: u32) -> u32 { kvmppc_get_sr(vcpu, srnum) }
unsafe fn kvmppc_mmu_book3s_32_mtsrin(vcpu: *mut kvm_vcpu, srnum: u32, value: ulong) { kvmppc_set_sr(vcpu, srnum, value); kvmppc_mmu_map_segment(vcpu, srnum as ulong << SID_SHIFT); }
unsafe fn kvmppc_mmu_book3s_32_tlbie(vcpu: *mut kvm_vcpu, ea: ulong, _large: bool) { let mut i = 0; let mut v: *mut kvm_vcpu = core::ptr::null_mut(); kvm_for_each_vcpu(i, v, (*vcpu).kvm) { kvmppc_mmu_pte_flush(v, ea, 0x0FFFF000); } }

unsafe fn kvmppc_mmu_book3s_32_esid_to_vsid(vcpu: *mut kvm_vcpu, esid: ulong, vsid: *mut u64) -> i32 {
    let ea = esid << SID_SHIFT; let mut sr = 0; let mut gvsid = esid as u64; let msr = kvmppc_get_msr(vcpu);
    if msr & (MSR_DR | MSR_IR) != 0 { sr = find_sr(vcpu, ea); if sr_valid(sr) { gvsid = sr_vsid(sr) as u64; } }
    match msr & (MSR_DR | MSR_IR) { 0 => *vsid = VSID_REAL | esid as u64, MSR_IR => *vsid = VSID_REAL_IR | gvsid, MSR_DR => *vsid = VSID_REAL_DR | gvsid, MSR_DR | MSR_IR => *vsid = if sr_valid(sr) { sr_vsid(sr) as u64 } else { VSID_BAT | gvsid }, _ => BUG() }
    if msr & MSR_PR != 0 { *vsid |= VSID_PR; } 0
}

unsafe fn kvmppc_mmu_book3s_32_is_dcbz32(_vcpu: *mut kvm_vcpu) -> bool { true }

pub unsafe fn kvmppc_mmu_book3s_32_init(vcpu: *mut kvm_vcpu) {
    let mmu = &mut (*vcpu).arch.mmu;
    mmu.mtsrin = Some(kvmppc_mmu_book3s_32_mtsrin); mmu.mfsrin = Some(kvmppc_mmu_book3s_32_mfsrin);
    mmu.xlate = Some(kvmppc_mmu_book3s_32_xlate); mmu.tlbie = Some(kvmppc_mmu_book3s_32_tlbie);
    mmu.esid_to_vsid = Some(kvmppc_mmu_book3s_32_esid_to_vsid); mmu.ea_to_vp = Some(kvmppc_mmu_book3s_32_ea_to_vp);
    mmu.is_dcbz32 = Some(kvmppc_mmu_book3s_32_is_dcbz32);
    mmu.slbmte = None; mmu.slbmfee = None; mmu.slbmfev = None; mmu.slbfee = None; mmu.slbie = None; mmu.slbia = None;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
