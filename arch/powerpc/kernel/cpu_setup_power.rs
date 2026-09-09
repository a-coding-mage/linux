// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2020, Jordan Niethe, IBM Corporation.
 *
 * This file contains low level CPU setup functions.
 * Originally written in assembly by Benjamin Herrenschmidt & various other
 * authors.
 */

// Dependencies are supplied by the surrounding kernel translation.

unsafe fn init_hvmode_206(t: *mut cpu_spec) -> bool {
    let msr: u64 = mfmsr();

    if msr & MSR_HV != 0 {
        return true;
    }

    (*t).cpu_features &= !(CPU_FTR_HVMODE | CPU_FTR_P9_TM_HV_ASSIST);
    false
}

unsafe fn init_LPCR_ISA300(mut lpcr: u64, lpes: u64) {
    // POWER9 has no VRMASD
    lpcr |= (lpes << LPCR_LPES_SH) & LPCR_LPES;
    lpcr |= LPCR_PECE0 | LPCR_PECE1 | LPCR_PECE2;
    lpcr |= (4u64 << LPCR_DPFD_SH) & LPCR_DPFD;
    lpcr &= !LPCR_HDICE; // clear HDICE
    lpcr |= 4u64 << LPCR_VC_SH;
    mtspr(SPRN_LPCR, lpcr);
    isync();
}

/*
 * Setup a sane LPCR:
 *   Called with initial LPCR and desired LPES 2-bit value
 *
 *   LPES = 0b01 (HSRR0/1 used for 0x500)
 *   PECE = 0b111
 *   DPFD = 4
 *   HDICE = 0
 *   VC = 0b100 (VPM0=1, VPM1=0, ISL=0)
 *   VRMASD = 0b10000 (L=1, LP=00)
 *
 * Other bits untouched for now
 */
unsafe fn init_LPCR_ISA206(mut lpcr: u64, lpes: u64) {
    lpcr |= (0x10u64 << LPCR_VRMASD_SH) & LPCR_VRMASD;
    init_LPCR_ISA300(lpcr, lpes);
}

unsafe fn init_FSCR() {
    let mut fscr: u64 = mfspr(SPRN_FSCR);
    fscr |= FSCR_TAR | FSCR_EBB;
    mtspr(SPRN_FSCR, fscr);
}

unsafe fn init_FSCR_power9() {
    let mut fscr: u64 = mfspr(SPRN_FSCR);
    fscr |= FSCR_SCV;
    mtspr(SPRN_FSCR, fscr);
    init_FSCR();
}

unsafe fn init_FSCR_power10() {
    let mut fscr: u64 = mfspr(SPRN_FSCR);
    fscr |= FSCR_PREFIX;
    mtspr(SPRN_FSCR, fscr);
    init_FSCR_power9();
}

unsafe fn init_HFSCR() {
    let mut hfscr: u64 = mfspr(SPRN_HFSCR);
    hfscr |= HFSCR_TAR | HFSCR_TM | HFSCR_BHRB | HFSCR_PM | HFSCR_DSCR |
        HFSCR_VECVSX | HFSCR_FP | HFSCR_EBB | HFSCR_MSGP;
    mtspr(SPRN_HFSCR, hfscr);
}

unsafe fn init_PMU_HV() { mtspr(SPRN_MMCRC, 0); }
unsafe fn init_PMU_HV_ISA207() { mtspr(SPRN_MMCRH, 0); }

unsafe fn init_PMU() {
    mtspr(SPRN_MMCRA, 0);
    mtspr(SPRN_MMCR0, MMCR0_FC);
    mtspr(SPRN_MMCR1, 0);
    mtspr(SPRN_MMCR2, 0);
}

unsafe fn init_PMU_ISA207() { mtspr(SPRN_MMCRS, 0); }

unsafe fn init_PMU_ISA31() {
    mtspr(SPRN_MMCR3, 0);
    mtspr(SPRN_MMCRA, MMCRA_BHRB_DISABLE);
    mtspr(SPRN_MMCR0, MMCR0_FC | MMCR0_PMCCEXT);
}

unsafe fn init_DEXCR() {
    mtspr(SPRN_DEXCR, DEXCR_INIT);
    mtspr(SPRN_HASHKEYR, 0);
}

/* Note that we can be called twice of pseudo-PVRs. The parameter offset is not used. */
pub unsafe fn __setup_cpu_power7(_offset: usize, t: *mut cpu_spec) {
    if !init_hvmode_206(t) { return; }
    mtspr(SPRN_LPID, 0); mtspr(SPRN_AMOR, !0u64); mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA206(mfspr(SPRN_LPCR), LPCR_LPES1 >> LPCR_LPES_SH);
}

pub unsafe fn __restore_cpu_power7() {
    if mfmsr() & MSR_HV == 0 { return; }
    mtspr(SPRN_LPID, 0); mtspr(SPRN_AMOR, !0u64); mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA206(mfspr(SPRN_LPCR), LPCR_LPES1 >> LPCR_LPES_SH);
}

pub unsafe fn __setup_cpu_power8(_offset: usize, t: *mut cpu_spec) {
    init_FSCR(); init_PMU(); init_PMU_ISA207();
    if !init_hvmode_206(t) { return; }
    mtspr(SPRN_LPID, 0); mtspr(SPRN_AMOR, !0u64); mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA206(mfspr(SPRN_LPCR) | LPCR_PECEDH, 0);
    init_HFSCR(); init_PMU_HV(); init_PMU_HV_ISA207();
}

pub unsafe fn __restore_cpu_power8() {
    init_FSCR(); init_PMU(); init_PMU_ISA207();
    if mfmsr() & MSR_HV == 0 { return; }
    mtspr(SPRN_LPID, 0); mtspr(SPRN_AMOR, !0u64); mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA206(mfspr(SPRN_LPCR) | LPCR_PECEDH, 0);
    init_HFSCR(); init_PMU_HV(); init_PMU_HV_ISA207();
}

unsafe fn setup_power9_common() {
    mtspr(SPRN_PSSCR, 0); mtspr(SPRN_LPID, 0); mtspr(SPRN_PID, 0);
    mtspr(SPRN_AMOR, !0u64); mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA300((mfspr(SPRN_LPCR) | LPCR_PECEDH | LPCR_PECE_HVEE |
        LPCR_HVICE | LPCR_HEIC) & !(LPCR_UPRT | LPCR_HR), 0);
    init_HFSCR(); init_PMU_HV();
}

pub unsafe fn __setup_cpu_power9(_offset: usize, t: *mut cpu_spec) {
    init_FSCR_power9(); init_PMU();
    if !init_hvmode_206(t) { return; }
    setup_power9_common();
}

pub unsafe fn __restore_cpu_power9() {
    init_FSCR_power9(); init_PMU();
    if mfmsr() & MSR_HV == 0 { return; }
    setup_power9_common();
}

pub unsafe fn __setup_cpu_power10(_offset: usize, t: *mut cpu_spec) {
    init_FSCR_power10(); init_PMU(); init_PMU_ISA31(); init_DEXCR();
    if !init_hvmode_206(t) { return; }
    setup_power9_common();
}

pub unsafe fn __restore_cpu_power10() {
    init_FSCR_power10(); init_PMU(); init_PMU_ISA31(); init_DEXCR();
    if mfmsr() & MSR_HV == 0 { return; }
    setup_power9_common();
}

pub unsafe fn __setup_cpu_power12(_offset: usize, t: *mut cpu_spec) {
    init_FSCR_power10(); init_PMU(); init_PMU_ISA31();
    if !init_hvmode_206(t) { return; }
    setup_power9_common();
}

pub unsafe fn __restore_cpu_power12() {
    init_FSCR_power10(); init_PMU(); init_PMU_ISA31();
    if mfmsr() & MSR_HV == 0 { return; }
    setup_power9_common();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
