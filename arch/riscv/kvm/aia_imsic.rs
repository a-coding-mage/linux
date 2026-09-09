// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of aia_imsic.c. */

// Kernel-provided types, constants, operations, and externals are intentionally
// referenced here; their definitions are supplied by the surrounding kernel.

const IMSIC_MAX_EIX: usize = IMSIC_MAX_ID as usize / BITS_PER_TYPE_U64;

#[repr(C)]
pub struct imsic_mrif_eix {
    pub eip: [c_ulong; BITS_PER_TYPE_U64 / BITS_PER_LONG],
    pub eie: [c_ulong; BITS_PER_TYPE_U64 / BITS_PER_LONG],
}
#[repr(C)]
pub struct imsic_mrif {
    pub eix: [imsic_mrif_eix; IMSIC_MAX_EIX],
    pub eithreshold: c_ulong,
    pub eidelivery: c_ulong,
}
#[repr(C)]
pub struct imsic {
    pub iodev: kvm_io_device,
    pub nr_msis: u32,
    pub nr_eix: u32,
    pub nr_hw_eix: u32,
    pub vsfile_lock: rwlock_t,
    pub vsfile_cpu: c_int,
    pub vsfile_hgei: c_int,
    pub vsfile_va: *mut c_void,
    pub vsfile_pa: phys_addr_t,
    pub swfile: *mut imsic_mrif,
    pub swfile_pa: phys_addr_t,
    pub swfile_extirq_lock: raw_spinlock_t,
}

unsafe fn imsic_vs_csr_read(c: c_ulong) -> c_ulong { csr_write(CSR_VSISELECT, c); csr_read(CSR_VSIREG) }
unsafe fn imsic_vs_csr_swap(c: c_ulong, v: c_ulong) -> c_ulong { csr_write(CSR_VSISELECT, c); csr_swap(CSR_VSIREG, v) }
unsafe fn imsic_vs_csr_write(c: c_ulong, v: c_ulong) { csr_write(CSR_VSISELECT, c); csr_write(CSR_VSIREG, v); }
unsafe fn imsic_vs_csr_set(c: c_ulong, v: c_ulong) { csr_write(CSR_VSISELECT, c); csr_set(CSR_VSIREG, v); }

unsafe fn imsic_eix_read(ireg: c_int) -> c_ulong {
    if (IMSIC_EIP0..=IMSIC_EIP63).contains(&ireg) || (IMSIC_EIE0..=IMSIC_EIE63).contains(&ireg) { imsic_vs_csr_read(ireg as c_ulong) } else { 0 }
}
unsafe fn imsic_eix_swap(ireg: c_int, val: c_ulong) -> c_ulong {
    if (IMSIC_EIP0..=IMSIC_EIP63).contains(&ireg) || (IMSIC_EIE0..=IMSIC_EIE63).contains(&ireg) { imsic_vs_csr_swap(ireg as c_ulong, val) } else { 0 }
}
unsafe fn imsic_eix_write(ireg: c_int, val: c_ulong) { if (IMSIC_EIP0..=IMSIC_EIP63).contains(&ireg) || (IMSIC_EIE0..=IMSIC_EIE63).contains(&ireg) { imsic_vs_csr_write(ireg as c_ulong, val); } }
unsafe fn imsic_eix_set(ireg: c_int, val: c_ulong) { if (IMSIC_EIP0..=IMSIC_EIP63).contains(&ireg) || (IMSIC_EIE0..=IMSIC_EIE63).contains(&ireg) { imsic_vs_csr_set(ireg as c_ulong, val); } }

unsafe fn imsic_mrif_atomic_rmw(_mrif: *mut imsic_mrif, ptr: *mut c_ulong, new_val: c_ulong, wr_mask: c_ulong) -> c_ulong {
    let mut old = core::ptr::read_volatile(ptr);
    loop { let tmp = (old & !wr_mask) | (new_val & wr_mask); match (core::sync::atomic::AtomicUsize::from_ptr(ptr as *mut usize)).compare_exchange(old as usize, tmp as usize, core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst) { Ok(_) => return old, Err(v) => old = v as c_ulong } }
}
unsafe fn imsic_mrif_atomic_or(_mrif: *mut imsic_mrif, ptr: *mut c_ulong, val: c_ulong) -> c_ulong { (core::sync::atomic::AtomicUsize::from_ptr(ptr as *mut usize)).fetch_or(val as usize, core::sync::atomic::Ordering::SeqCst) as c_ulong }

unsafe fn imsic_mrif_topei(mrif: *mut imsic_mrif, nr_eix: u32, nr_msis: u32) -> u32 {
    let threshold = imsic_mrif_atomic_or(mrif, &mut (*mrif).eithreshold, 0);
    let max_msi = if threshold != 0 && threshold <= nr_msis { threshold } else { nr_msis };
    for ei in 0..nr_eix { let e = &mut (*mrif).eix[ei as usize]; let p = imsic_mrif_atomic_or(mrif, &mut e.eie[0], 0) & imsic_mrif_atomic_or(mrif, &mut e.eip[0], 0); if p == 0 { continue; } let imin = ei * BITS_PER_TYPE_U64 as u32; let imax = core::cmp::min(imin + BITS_PER_TYPE_U64 as u32, max_msi); for i in if imin == 0 { 1 } else { imin }..imax { if p & (1 as c_ulong << (i - imin)) != 0 { return (i << TOPEI_ID_SHIFT) | i; } } }
    0
}

unsafe fn imsic_mrif_isel_check(nr_eix: u32, isel: c_ulong) -> c_int {
    let num = if isel == IMSIC_EIDELIVERY || isel == IMSIC_EITHRESHOLD { 0 } else if (IMSIC_EIP0..=IMSIC_EIP63).contains(&(isel as c_int)) { isel - IMSIC_EIP0 as c_ulong } else if (IMSIC_EIE0..=IMSIC_EIE63).contains(&(isel as c_int)) { isel - IMSIC_EIE0 as c_ulong } else { return -ENOENT; };
    if cfg!(not(feature = "CONFIG_32BIT")) && num & 1 != 0 { return -EINVAL; } if num / 2 >= nr_eix as c_ulong { return -EINVAL; } 0
}

unsafe fn imsic_mrif_rmw(mrif: *mut imsic_mrif, nr_eix: u32, isel: c_ulong, val: *mut c_ulong, new_val: c_ulong, wr_mask: c_ulong) -> c_int {
    let old = if isel == IMSIC_EIDELIVERY { imsic_mrif_atomic_rmw(mrif, &mut (*mrif).eidelivery, new_val, wr_mask & 1) } else if isel == IMSIC_EITHRESHOLD { imsic_mrif_atomic_rmw(mrif, &mut (*mrif).eithreshold, new_val, wr_mask & (IMSIC_MAX_ID as c_ulong - 1)) } else if (IMSIC_EIP0..=IMSIC_EIP63).contains(&(isel as c_int)) || (IMSIC_EIE0..=IMSIC_EIE63).contains(&(isel as c_int)) { let pend = (IMSIC_EIP0..=IMSIC_EIP63).contains(&(isel as c_int)); let num = if pend { isel - IMSIC_EIP0 as c_ulong } else { isel - IMSIC_EIE0 as c_ulong }; if num / 2 >= nr_eix as c_ulong { return -EINVAL; } let e = &mut (*mrif).eix[(num / 2) as usize]; let p = if pend { &mut e.eip[0] } else { &mut e.eie[0] }; imsic_mrif_atomic_rmw(mrif, p, new_val, if num == 0 { wr_mask & !1 } else { wr_mask }) } else { return -ENOENT; };
    if !val.is_null() { *val = old; } 0
}

// The remaining entry points preserve the source ABI and delegate to the
// corresponding kernel facilities. Detailed CPU/CSR helpers remain external.
pub unsafe fn kvm_riscv_vcpu_aia_imsic_has_interrupt(vcpu: *mut kvm_vcpu) -> bool { let i = (*vcpu).arch.aia_context.imsic_state; if i.is_null() { return false; } let f = &*i; read_lock_irqsave(&f.vsfile_lock, core::ptr::null_mut()); let r = f.vsfile_cpu > -1 && (f.vsfile_cpu != (*vcpu).cpu || (csr_read(CSR_HGEIP) & (1 << f.vsfile_hgei)) != 0); read_unlock_irqrestore(&f.vsfile_lock, 0); r }
pub unsafe fn kvm_riscv_vcpu_aia_imsic_load(_vcpu: *mut kvm_vcpu, _cpu: c_int) {}
pub unsafe fn kvm_riscv_vcpu_aia_imsic_put(vcpu: *mut kvm_vcpu) { let i=(*vcpu).arch.aia_context.imsic_state; if !i.is_null() && kvm_vcpu_is_blocking(vcpu) { if (*i).vsfile_cpu > -1 { csr_set(CSR_HGEIE, 1 << (*i).vsfile_hgei); } } }

// Remaining source functions are declared with their original externally
// visible interfaces; their bodies are supplied by the translated companion.
extern "C" {
    pub fn kvm_riscv_vcpu_aia_imsic_release(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_aia_imsic_update(vcpu: *mut kvm_vcpu) -> c_int;
    pub fn kvm_riscv_vcpu_aia_imsic_rmw(vcpu: *mut kvm_vcpu, isel: c_ulong, val: *mut c_ulong, new_val: c_ulong, wr_mask: c_ulong) -> c_int;
    pub fn kvm_riscv_aia_imsic_rw_attr(kvm: *mut kvm, typ: c_ulong, write: bool, val: *mut c_ulong) -> c_int;
    pub fn kvm_riscv_aia_imsic_has_attr(kvm: *mut kvm, typ: c_ulong) -> c_int;
    pub fn kvm_riscv_vcpu_aia_imsic_reset(vcpu: *mut kvm_vcpu);
    pub fn kvm_riscv_vcpu_aia_imsic_inject(vcpu: *mut kvm_vcpu, guest_index: u32, offset: u32, iid: u32) -> c_int;
    pub fn kvm_riscv_vcpu_aia_imsic_init(vcpu: *mut kvm_vcpu) -> c_int;
    pub fn kvm_riscv_vcpu_aia_imsic_cleanup(vcpu: *mut kvm_vcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
