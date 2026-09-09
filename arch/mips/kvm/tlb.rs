/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * KVM/MIPS TLB handling, this file is part of the Linux host kernel so that
 * TLB handlers run from KSEG0
 *
 * Copyright (C) 2012  MIPS Technologies, Inc.  All rights reserved.
 * Authors: Sanjay Lal <sanjayl@kymasys.com>
 */

pub static mut GUESTID_MASK: c_ulong = 0;
pub static mut GUESTID_FIRST_VERSION: c_ulong = 0;
pub static mut GUESTID_VERSION_MASK: c_ulong = 0;

unsafe fn kvm_mips_get_root_asid(vcpu: *mut kvm_vcpu) -> u32 {
    let gpa_mm = &(*(*vcpu).kvm).arch.gpa_mm;
    if cpu_has_guestid { 0 } else { cpu_asid(smp_processor_id(), gpa_mm) }
}

unsafe fn _kvm_mips_host_tlb_inv(entryhi: c_ulong) -> c_int {
    let mut idx: c_int;
    write_c0_entryhi(entryhi);
    mtc0_tlbw_hazard();
    tlb_probe();
    tlb_probe_hazard();
    idx = read_c0_index();
    BUG_ON(idx >= current_cpu_data.tlbsize);
    if idx >= 0 {
        write_c0_entryhi(UNIQUE_ENTRYHI(idx));
        write_c0_entrylo0(0);
        write_c0_entrylo1(0);
        mtc0_tlbw_hazard();
        tlb_write_indexed();
        tlbw_use_hazard();
    }
    idx
}

/* GuestID management */

/// clear_root_gid() - Set GuestCtl1.RID for normal root operation.
unsafe fn clear_root_gid() {
    if cpu_has_guestid {
        clear_c0_guestctl1(MIPS_GCTL1_RID);
        mtc0_tlbw_hazard();
    }
}

/// set_root_gid_to_guest_gid() - Set GuestCtl1.RID to match GuestCtl1.ID.
unsafe fn set_root_gid_to_guest_gid() {
    let mut guestctl1: c_uint;
    if cpu_has_guestid {
        back_to_back_c0_hazard();
        guestctl1 = read_c0_guestctl1();
        guestctl1 = (guestctl1 & !MIPS_GCTL1_RID) |
            ((guestctl1 & MIPS_GCTL1_ID) >> MIPS_GCTL1_ID_SHIFT) << MIPS_GCTL1_RID_SHIFT;
        write_c0_guestctl1(guestctl1);
        mtc0_tlbw_hazard();
    }
}

pub unsafe fn kvm_vz_host_tlb_inv(vcpu: *mut kvm_vcpu, va: c_ulong) -> c_int {
    let flags: c_ulong;
    local_irq_save(&mut flags);
    htw_stop();
    set_root_gid_to_guest_gid();
    let old_entryhi = read_c0_entryhi();
    let idx = _kvm_mips_host_tlb_inv((va & VPN2_MASK) | kvm_mips_get_root_asid(vcpu) as c_ulong);
    write_c0_entryhi(old_entryhi);
    clear_root_gid();
    mtc0_tlbw_hazard();
    htw_start();
    local_irq_restore(flags);
    if cpu_has_vtag_icache { flush_icache_all(); }
    if idx > 0 {
        kvm_debug!("%s: Invalidated root entryhi %#lx @ idx %d\n", __func__,
                   (va & VPN2_MASK) | kvm_mips_get_root_asid(vcpu) as c_ulong, idx);
    }
    0
}

pub unsafe fn kvm_vz_guest_tlb_lookup(vcpu: *mut kvm_vcpu, gva: c_ulong, gpa: *mut c_ulong) -> c_int {
    let mut o_entrylo = [0 as c_ulong; 2];
    let mut entrylo = [0 as c_ulong; 2];
    let flags: c_ulong;
    local_irq_save(&mut flags);
    htw_stop(); set_root_gid_to_guest_gid();
    let o_entryhi = read_gc0_entryhi();
    let o_index = read_gc0_index();
    write_gc0_entryhi((o_entryhi & 0x3ff) | (gva & !0xfff));
    mtc0_tlbw_hazard(); guest_tlb_probe(); tlb_probe_hazard();
    let index = read_gc0_index();
    if index < 0 {
        write_gc0_entryhi(o_entryhi); write_gc0_index(o_index);
        clear_root_gid(); htw_start(); local_irq_restore(flags); return -EFAULT;
    }
    o_entrylo[0] = read_gc0_entrylo0(); o_entrylo[1] = read_gc0_entrylo1();
    let o_pagemask = read_gc0_pagemask();
    mtc0_tlbr_hazard(); guest_tlb_read(); tlb_read_hazard();
    entrylo[0] = read_gc0_entrylo0(); entrylo[1] = read_gc0_entrylo1();
    let pagemask = !read_gc0_pagemask() & !0x1fff;
    write_gc0_entryhi(o_entryhi); write_gc0_index(o_index);
    write_gc0_entrylo0(entrylo[0]); write_gc0_entrylo1(entrylo[1]); write_gc0_pagemask(o_pagemask);
    clear_root_gid(); htw_start(); local_irq_restore(flags);
    let pagemaskbit = (pagemask ^ (pagemask & pagemask.wrapping_sub(1))) >> 1;
    let mut pa = entrylo[((gva & pagemaskbit) != 0) as usize];
    if pa & ENTRYLO_V == 0 { return -EFAULT; }
    pa = (pa << 6) & !0xfff; pa |= gva & !(pagemask | pagemaskbit);
    *gpa = pa; 0
}

pub unsafe fn kvm_vz_local_flush_roottlb_all_guests() {
    if WARN_ON(!cpu_has_guestid) { return; }
    let flags: c_ulong; local_irq_save(&mut flags); htw_stop();
    let old_entryhi = read_c0_entryhi(); let old_pagemask = read_c0_pagemask();
    let old_guestctl1 = read_c0_guestctl1();
    for entry in 0..current_cpu_data.tlbsize {
        write_c0_index(entry); mtc0_tlbw_hazard(); tlb_read(); tlb_read_hazard();
        if read_c0_guestctl1() & MIPS_GCTL1_RID == 0 { continue; }
        write_c0_entryhi(UNIQUE_ENTRYHI(entry)); write_c0_entrylo0(0); write_c0_entrylo1(0);
        write_c0_guestctl1(0); mtc0_tlbw_hazard(); tlb_write_indexed();
    }
    write_c0_entryhi(old_entryhi); write_c0_pagemask(old_pagemask); write_c0_guestctl1(old_guestctl1);
    tlbw_use_hazard(); htw_start(); local_irq_restore(flags);
}

pub unsafe fn kvm_vz_local_flush_guesttlb_all() {
    let flags: c_ulong; local_irq_save(&mut flags);
    let old_index = read_gc0_index(); let old_entryhi = read_gc0_entryhi();
    let old_entrylo = [read_gc0_entrylo0(), read_gc0_entrylo1()]; let old_pagemask = read_gc0_pagemask();
    let mut cvmmemctl2: u64 = 0;
    match current_cpu_type() { CPU_CAVIUM_OCTEON3 => { cvmmemctl2 = read_c0_cvmmemctl2(); cvmmemctl2 |= CVMMEMCTL2_INHIBITTS; write_c0_cvmmemctl2(cvmmemctl2); }, _ => {} }
    write_gc0_entrylo0(0); write_gc0_entrylo1(0); write_gc0_pagemask(0);
    for entry in 0..current_cpu_data.guest.tlbsize { write_gc0_index(entry); write_gc0_entryhi(UNIQUE_GUEST_ENTRYHI(entry)); mtc0_tlbw_hazard(); guest_tlb_write_indexed(); }
    if cvmmemctl2 != 0 { write_c0_cvmmemctl2(cvmmemctl2 & !CVMMEMCTL2_INHIBITTS); }
    write_gc0_index(old_index); write_gc0_entryhi(old_entryhi); write_gc0_entrylo0(old_entrylo[0]); write_gc0_entrylo1(old_entrylo[1]); write_gc0_pagemask(old_pagemask); tlbw_use_hazard(); local_irq_restore(flags);
}

pub unsafe fn kvm_vz_save_guesttlb(buf: *mut kvm_mips_tlb, index: c_uint, count: c_uint) {
    let end = index + count; let old_index = read_gc0_index(); let old_entryhi = read_gc0_entryhi(); let old_entrylo0 = read_gc0_entrylo0(); let old_entrylo1 = read_gc0_entrylo1(); let old_pagemask = read_gc0_pagemask(); let mut guestctl1 = 0;
    htw_stop(); set_root_gid_to_guest_gid(); if cpu_has_guestid { guestctl1 = read_c0_guestctl1(); }
    for i in index..end { write_gc0_index(i); mtc0_tlbr_hazard(); guest_tlb_read(); tlb_read_hazard(); let b = &mut *buf.add((i-index) as usize); if cpu_has_guestid && ((read_c0_guestctl1() ^ guestctl1) & MIPS_GCTL1_RID) != 0 { b.tlb_hi=UNIQUE_GUEST_ENTRYHI(i); b.tlb_lo=[0,0]; b.tlb_mask=0; } else { b.tlb_hi=read_gc0_entryhi(); b.tlb_lo=[read_gc0_entrylo0(),read_gc0_entrylo1()]; b.tlb_mask=read_gc0_pagemask(); } }
    clear_root_gid(); htw_start(); write_gc0_index(old_index); write_gc0_entryhi(old_entryhi); write_gc0_entrylo0(old_entrylo0); write_gc0_entrylo1(old_entrylo1); write_gc0_pagemask(old_pagemask); tlbw_use_hazard();
}

pub unsafe fn kvm_vz_load_guesttlb(buf: *const kvm_mips_tlb, index: c_uint, count: c_uint) {
    let end=index+count; let old_index=read_gc0_index(); let old_entryhi=read_gc0_entryhi(); let old_entrylo0=read_gc0_entrylo0(); let old_entrylo1=read_gc0_entrylo1(); let old_pagemask=read_gc0_pagemask(); htw_stop(); set_root_gid_to_guest_gid();
    for i in index..end { let b=&*buf.add((i-index) as usize); write_gc0_index(i); write_gc0_entryhi(b.tlb_hi); write_gc0_entrylo0(b.tlb_lo[0]); write_gc0_entrylo1(b.tlb_lo[1]); write_gc0_pagemask(b.tlb_mask); mtc0_tlbw_hazard(); guest_tlb_write_indexed(); }
    clear_root_gid(); htw_start(); write_gc0_index(old_index); write_gc0_entryhi(old_entryhi); write_gc0_entrylo0(old_entrylo0); write_gc0_entrylo1(old_entrylo1); write_gc0_pagemask(old_pagemask); tlbw_use_hazard();
}

#[cfg(CONFIG_CPU_LOONGSON64)]
pub unsafe fn kvm_loongson_clear_guest_vtlb() { let idx=read_gc0_index(); set_root_gid_to_guest_gid(); write_gc0_index(0); guest_tlbinvf(); write_gc0_index(idx); clear_root_gid(); set_c0_diag(LOONGSON_DIAG_ITLB|LOONGSON_DIAG_DTLB); }

#[cfg(CONFIG_CPU_LOONGSON64)]
pub unsafe fn kvm_loongson_clear_guest_ftlb() { let idx=read_gc0_index(); set_root_gid_to_guest_gid(); for i in current_cpu_data.tlbsizevtlb..(current_cpu_data.tlbsizevtlb+current_cpu_data.tlbsizeftlbsets) { write_gc0_index(i); guest_tlbinvf(); } write_gc0_index(idx); clear_root_gid(); set_c0_diag(LOONGSON_DIAG_ITLB|LOONGSON_DIAG_DTLB); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
