/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from the LoongArch KVM CSR header. */

// Dependencies supplied by the surrounding kernel translation:
// linux/uaccess.h, linux/kvm_host.h, asm/loongarch.h, asm/kvm_vcpu.h

#[inline(always)]
pub unsafe fn gcsr_read(csr: usize) -> usize {
    let value: usize;
    core::arch::asm!("gcsrrd {value}, {reg}", value = out(reg) value,
        reg = const csr, options(nostack));
    value
}

#[inline(always)]
pub unsafe fn gcsr_write(value: usize, csr: usize) -> usize {
    let mut value = value;
    core::arch::asm!("gcsrwr {value}, {reg}", value = inout(reg) value,
        reg = const csr, options(nostack));
    value
}

#[inline(always)]
pub unsafe fn gcsr_xchg(value: usize, mask: usize, csr: usize) -> usize {
    let mut value = value;
    core::arch::asm!("gcsrxchg {value}, {mask}, {reg}", value = inout(reg) value,
        mask = in(reg) mask, reg = const csr, options(nostack));
    value
}

macro_rules! gcsr_accessors {
    ($(($read:ident, $write:ident, $csr:ident)),* $(,)?) => {
        $(
            #[inline(always)] pub unsafe fn $read() -> usize { gcsr_read($csr) }
            #[inline(always)] pub unsafe fn $write(value: usize) -> usize { gcsr_write(value, $csr) }
        )*
    };
}

gcsr_accessors!(
    (read_gcsr_crmd, write_gcsr_crmd, LOONGARCH_CSR_CRMD),
    (read_gcsr_prmd, write_gcsr_prmd, LOONGARCH_CSR_PRMD),
    (read_gcsr_euen, write_gcsr_euen, LOONGARCH_CSR_EUEN),
    (read_gcsr_misc, write_gcsr_misc, LOONGARCH_CSR_MISC),
    (read_gcsr_ecfg, write_gcsr_ecfg, LOONGARCH_CSR_ECFG),
    (read_gcsr_estat, write_gcsr_estat, LOONGARCH_CSR_ESTAT),
    (read_gcsr_era, write_gcsr_era, LOONGARCH_CSR_ERA),
    (read_gcsr_badv, write_gcsr_badv, LOONGARCH_CSR_BADV),
    (read_gcsr_badi, write_gcsr_badi, LOONGARCH_CSR_BADI),
    (read_gcsr_eentry, write_gcsr_eentry, LOONGARCH_CSR_EENTRY),
    (read_gcsr_asid, write_gcsr_asid, LOONGARCH_CSR_ASID),
    (read_gcsr_pgdl, write_gcsr_pgdl, LOONGARCH_CSR_PGDL),
    (read_gcsr_pgdh, write_gcsr_pgdh, LOONGARCH_CSR_PGDH),
    (read_gcsr_pgd, write_gcsr_pgd, LOONGARCH_CSR_PGD),
    (read_gcsr_pwctl0, write_gcsr_pwctl0, LOONGARCH_CSR_PWCTL0),
    (read_gcsr_pwctl1, write_gcsr_pwctl1, LOONGARCH_CSR_PWCTL1),
    (read_gcsr_stlbpgsize, write_gcsr_stlbpgsize, LOONGARCH_CSR_STLBPGSIZE),
    (read_gcsr_rvacfg, write_gcsr_rvacfg, LOONGARCH_CSR_RVACFG),
    (read_gcsr_cpuid, write_gcsr_cpuid, LOONGARCH_CSR_CPUID),
    (read_gcsr_prcfg1, write_gcsr_prcfg1, LOONGARCH_CSR_PRCFG1),
    (read_gcsr_prcfg2, write_gcsr_prcfg2, LOONGARCH_CSR_PRCFG2),
    (read_gcsr_prcfg3, write_gcsr_prcfg3, LOONGARCH_CSR_PRCFG3),
    (read_gcsr_kscratch0, write_gcsr_kscratch0, LOONGARCH_CSR_KS0),
    (read_gcsr_kscratch1, write_gcsr_kscratch1, LOONGARCH_CSR_KS1),
    (read_gcsr_kscratch2, write_gcsr_kscratch2, LOONGARCH_CSR_KS2),
    (read_gcsr_kscratch3, write_gcsr_kscratch3, LOONGARCH_CSR_KS3),
    (read_gcsr_kscratch4, write_gcsr_kscratch4, LOONGARCH_CSR_KS4),
    (read_gcsr_kscratch5, write_gcsr_kscratch5, LOONGARCH_CSR_KS5),
    (read_gcsr_kscratch6, write_gcsr_kscratch6, LOONGARCH_CSR_KS6),
    (read_gcsr_kscratch7, write_gcsr_kscratch7, LOONGARCH_CSR_KS7),
    (read_gcsr_timerid, write_gcsr_timerid, LOONGARCH_CSR_TMID),
    (read_gcsr_timercfg, write_gcsr_timercfg, LOONGARCH_CSR_TCFG),
    (read_gcsr_timertick, write_gcsr_timertick, LOONGARCH_CSR_TVAL),
    (read_gcsr_timeroffset, write_gcsr_timeroffset, LOONGARCH_CSR_CNTC),
    (read_gcsr_llbctl, write_gcsr_llbctl, LOONGARCH_CSR_LLBCTL),
    (read_gcsr_tlbidx, write_gcsr_tlbidx, LOONGARCH_CSR_TLBIDX),
    (read_gcsr_tlbrentry, write_gcsr_tlbrentry, LOONGARCH_CSR_TLBRENTRY),
    (read_gcsr_tlbrbadv, write_gcsr_tlbrbadv, LOONGARCH_CSR_TLBRBADV),
    (read_gcsr_tlbrera, write_gcsr_tlbrera, LOONGARCH_CSR_TLBRERA),
    (read_gcsr_tlbrsave, write_gcsr_tlbrsave, LOONGARCH_CSR_TLBRSAVE),
    (read_gcsr_tlbrelo0, write_gcsr_tlbrelo0, LOONGARCH_CSR_TLBRELO0),
    (read_gcsr_tlbrelo1, write_gcsr_tlbrelo1, LOONGARCH_CSR_TLBRELO1),
    (read_gcsr_tlbrehi, write_gcsr_tlbrehi, LOONGARCH_CSR_TLBREHI),
    (read_gcsr_tlbrprmd, write_gcsr_tlbrprmd, LOONGARCH_CSR_TLBRPRMD),
    (read_gcsr_directwin0, write_gcsr_directwin0, LOONGARCH_CSR_DMWIN0),
    (read_gcsr_directwin1, write_gcsr_directwin1, LOONGARCH_CSR_DMWIN1),
    (read_gcsr_directwin2, write_gcsr_directwin2, LOONGARCH_CSR_DMWIN2),
    (read_gcsr_directwin3, write_gcsr_directwin3, LOONGARCH_CSR_DMWIN3),
);

#[inline(always)] pub unsafe fn read_csr_gtlbc() -> usize { csr_read64(LOONGARCH_CSR_GTLBC) }
#[inline(always)] pub unsafe fn write_csr_gtlbc(v: usize) -> usize { csr_write64(v, LOONGARCH_CSR_GTLBC) }
#[inline(always)] pub unsafe fn read_csr_trgp() -> usize { csr_read64(LOONGARCH_CSR_TRGP) }
#[inline(always)] pub unsafe fn read_csr_gcfg() -> usize { csr_read64(LOONGARCH_CSR_GCFG) }
#[inline(always)] pub unsafe fn write_csr_gcfg(v: usize) -> usize { csr_write64(v, LOONGARCH_CSR_GCFG) }
#[inline(always)] pub unsafe fn read_csr_gstat() -> usize { csr_read64(LOONGARCH_CSR_GSTAT) }
#[inline(always)] pub unsafe fn write_csr_gstat(v: usize) -> usize { csr_write64(v, LOONGARCH_CSR_GSTAT) }
#[inline(always)] pub unsafe fn read_csr_gintc() -> usize { csr_read64(LOONGARCH_CSR_GINTC) }
#[inline(always)] pub unsafe fn write_csr_gintc(v: usize) -> usize { csr_write64(v, LOONGARCH_CSR_GINTC) }
#[inline(always)] pub unsafe fn read_csr_gcntc() -> usize { csr_read64(LOONGARCH_CSR_GCNTC) }
#[inline(always)] pub unsafe fn write_csr_gcntc(v: usize) -> usize { csr_write64(v, LOONGARCH_CSR_GCNTC) }

// __BUILD_GCSR_OP expands through the external __BUILD_CSR_COMMON macro.
// __BUILD_GCSR_OP(gcfg), __BUILD_GCSR_OP(gstat), __BUILD_GCSR_OP(gtlbc),
// __BUILD_GCSR_OP(gintc), __BUILD_GCSR_OP(llbctl), __BUILD_GCSR_OP(tlbidx)

#[inline(always)] pub unsafe fn set_gcsr_estat(value: usize) -> usize {
    gcsr_xchg(value, value, LOONGARCH_CSR_ESTAT)
}
#[inline(always)] pub unsafe fn clear_gcsr_estat(value: usize) -> usize {
    gcsr_xchg(!value, value, LOONGARCH_CSR_ESTAT)
}
#[inline(always)] pub unsafe fn kvm_read_hw_gcsr(id: usize) -> usize { gcsr_read(id) }
#[inline(always)] pub unsafe fn kvm_write_hw_gcsr(id: usize, value: usize) -> usize { gcsr_write(value, id) }

macro_rules! kvm_save_hw_gcsr {
    ($csr:expr, $gid:expr) => { (*$csr).csrs[$gid] = gcsr_read($gid); };
}
macro_rules! kvm_restore_hw_gcsr {
    ($csr:expr, $gid:expr) => { gcsr_write((*$csr).csrs[$gid], $gid); };
}
macro_rules! kvm_read_clear_hw_gcsr {
    ($csr:expr, $gid:expr) => { (*$csr).csrs[$gid] = gcsr_write(0, $gid); };
}

pub unsafe fn kvm_emu_iocsr(inst: larch_inst, run: *mut kvm_run, vcpu: *mut kvm_vcpu) -> i32;

#[inline(always)] pub unsafe fn kvm_read_sw_gcsr(csr: *mut loongarch_csrs, gid: usize) -> usize { (*csr).csrs[gid] }
#[inline(always)] pub unsafe fn kvm_write_sw_gcsr(csr: *mut loongarch_csrs, gid: usize, value: usize) { (*csr).csrs[gid] = value; }
#[inline(always)] pub unsafe fn kvm_set_sw_gcsr(csr: *mut loongarch_csrs, gid: usize, value: usize) { (*csr).csrs[gid] |= value; }
#[inline(always)] pub unsafe fn kvm_change_sw_gcsr(csr: *mut loongarch_csrs, gid: usize, mask: usize, value: usize) {
    (*csr).csrs[gid] &= !mask;
    (*csr).csrs[gid] |= value & mask;
}

pub const KVM_PMU_EVENT_ENABLED: usize = CSR_PERFCTRL_PLV0 | CSR_PERFCTRL_PLV1 |
    CSR_PERFCTRL_PLV2 | CSR_PERFCTRL_PLV3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
