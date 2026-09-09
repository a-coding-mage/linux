// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel translation unit.

unsafe fn is_loongson(c: *mut cpuinfo_mips) -> bool {
    match (*c).processor_id & PRID_COMP_MASK {
        PRID_COMP_LEGACY => ((*c).processor_id & PRID_IMP_MASK) == PRID_IMP_LOONGSON_64C,
        PRID_COMP_LOONGSON => true,
        _ => false,
    }
}

unsafe fn get_loongson_fprev(c: *mut cpuinfo_mips) -> u32 {
    (*c).fpu_id & LOONGSON_FPREV_MASK
}

unsafe fn cpu_has_uca() -> bool {
    let diag = read_c0_diag();
    let mut new_diag: u32;

    if diag & LOONGSON_DIAG_UCAC != 0 {
        /* UCA is already enabled. */
        return true;
    }

    /* See if UCAC bit can be flipped on. This should be safe. */
    new_diag = diag | LOONGSON_DIAG_UCAC;
    write_c0_diag(new_diag);
    new_diag = read_c0_diag();
    write_c0_diag(diag);

    new_diag & LOONGSON_DIAG_UCAC != 0
}

unsafe fn probe_uca(c: *mut cpuinfo_mips) {
    if cpu_has_uca() {
        (*c).loongson3_cpucfg_data[0] |= LOONGSON_CFG1_LSUCA;
    }
}

unsafe fn decode_loongson_config6(c: *mut cpuinfo_mips) {
    let config6 = read_c0_config6();

    if config6 & LOONGSON_CONF6_SFBEN != 0 {
        (*c).loongson3_cpucfg_data[0] |= LOONGSON_CFG1_SFBP;
    }
    if config6 & LOONGSON_CONF6_LLEXC != 0 {
        (*c).loongson3_cpucfg_data[0] |= LOONGSON_CFG1_LLEXC;
    }
    if config6 & LOONGSON_CONF6_SCRAND != 0 {
        (*c).loongson3_cpucfg_data[0] |= LOONGSON_CFG1_SCRAND;
    }
}

unsafe fn patch_cpucfg_sel1(c: *mut cpuinfo_mips) {
    let ases = (*c).ases;
    let options = (*c).options;
    let mut data = (*c).loongson3_cpucfg_data[0];

    if options & MIPS_CPU_FPU != 0 {
        data |= LOONGSON_CFG1_FP;
        data |= get_loongson_fprev(c) << LOONGSON_CFG1_FPREV_OFFSET;
    }
    if ases & MIPS_ASE_LOONGSON_MMI != 0 {
        data |= LOONGSON_CFG1_MMI;
    }
    if ases & MIPS_ASE_MSA != 0 {
        data |= LOONGSON_CFG1_MSA1;
    }

    (*c).loongson3_cpucfg_data[0] = data;
}

unsafe fn patch_cpucfg_sel2(c: *mut cpuinfo_mips) {
    let ases = (*c).ases;
    let options = (*c).options;
    let mut data = (*c).loongson3_cpucfg_data[1];

    if ases & MIPS_ASE_LOONGSON_EXT != 0 { data |= LOONGSON_CFG2_LEXT1; }
    if ases & MIPS_ASE_LOONGSON_EXT2 != 0 { data |= LOONGSON_CFG2_LEXT2; }
    if options & MIPS_CPU_LDPTE != 0 { data |= LOONGSON_CFG2_LSPW; }
    if ases & MIPS_ASE_VZ != 0 { data |= LOONGSON_CFG2_LVZP; }
    else { data &= !LOONGSON_CFG2_LVZREV; }

    (*c).loongson3_cpucfg_data[1] = data;
}

unsafe fn patch_cpucfg_sel3(c: *mut cpuinfo_mips) {
    let ases = (*c).ases;
    let mut data = (*c).loongson3_cpucfg_data[2];

    if ases & MIPS_ASE_LOONGSON_CAM != 0 {
        data |= LOONGSON_CFG3_LCAMP;
    } else {
        data &= !LOONGSON_CFG3_LCAMREV;
        data &= !LOONGSON_CFG3_LCAMNUM;
        data &= !LOONGSON_CFG3_LCAMKW;
        data &= !LOONGSON_CFG3_LCAMVW;
    }

    (*c).loongson3_cpucfg_data[2] = data;
}

pub unsafe fn loongson3_cpucfg_synthesize_data(c: *mut cpuinfo_mips) {
    /* Only engage the logic on Loongson processors. */
    if !is_loongson(c) { return; }

    /* CPUs with CPUCFG support don't need to synthesize anything. */
    if cpu_has_cfg() {
        elf_hwcap |= HWCAP_LOONGSON_CPUCFG;
        return;
    }

    (*c).loongson3_cpucfg_data[0] = 0;
    (*c).loongson3_cpucfg_data[1] = 0;
    (*c).loongson3_cpucfg_data[2] = 0;

    /* Add CPUCFG features non-discoverable otherwise. */
    match (*c).processor_id & (PRID_IMP_MASK | PRID_REV_MASK) {
        x if x == (PRID_IMP_LOONGSON_64R | PRID_REV_LOONGSON2K_R1_0) ||
             x == (PRID_IMP_LOONGSON_64R | PRID_REV_LOONGSON2K_R1_1) ||
             x == (PRID_IMP_LOONGSON_64R | PRID_REV_LOONGSON2K_R1_2) ||
             x == (PRID_IMP_LOONGSON_64R | PRID_REV_LOONGSON2K_R1_3) => {
            decode_loongson_config6(c); probe_uca(c);
            (*c).loongson3_cpucfg_data[0] |= LOONGSON_CFG1_LSLDR0 | LOONGSON_CFG1_LSSYNCI | LOONGSON_CFG1_LLSYNC | LOONGSON_CFG1_TGTSYNC;
            (*c).loongson3_cpucfg_data[1] |= LOONGSON_CFG2_LBT1 | LOONGSON_CFG2_LBT2 | LOONGSON_CFG2_LPMP | LOONGSON_CFG2_LPM_REV2;
            (*c).loongson3_cpucfg_data[2] = 0;
        }
        x if x == (PRID_IMP_LOONGSON_64C | PRID_REV_LOONGSON3A_R1) ||
             x == (PRID_IMP_LOONGSON_64C | PRID_REV_LOONGSON3B_R1) ||
             x == (PRID_IMP_LOONGSON_64C | PRID_REV_LOONGSON3B_R2) => {
            (*c).loongson3_cpucfg_data[0] |= LOONGSON_CFG1_LSLDR0 | LOONGSON_CFG1_LSSYNCI | LOONGSON_CFG1_LSUCA | LOONGSON_CFG1_LLSYNC | LOONGSON_CFG1_TGTSYNC;
            (*c).loongson3_cpucfg_data[1] |= LOONGSON_CFG2_LBT1 | LOONGSON_CFG2_LPMP | LOONGSON_CFG2_LPM_REV1;
            (*c).loongson3_cpucfg_data[2] |= LOONGSON_CFG3_LCAM_REV1 | LOONGSON_CFG3_LCAMNUM_REV1 | LOONGSON_CFG3_LCAMKW_REV1 | LOONGSON_CFG3_LCAMVW_REV1;
        }
        x if x == (PRID_IMP_LOONGSON_64C | PRID_REV_LOONGSON3A_R2_0) ||
             x == (PRID_IMP_LOONGSON_64C | PRID_REV_LOONGSON3A_R2_1) ||
             x == (PRID_IMP_LOONGSON_64C | PRID_REV_LOONGSON3A_R3_0) ||
             x == (PRID_IMP_LOONGSON_64C | PRID_REV_LOONGSON3A_R3_1) => {
            decode_loongson_config6(c); probe_uca(c);
            (*c).loongson3_cpucfg_data[0] |= LOONGSON_CFG1_CNT64 | LOONGSON_CFG1_LSLDR0 | LOONGSON_CFG1_LSPREF | LOONGSON_CFG1_LSPREFX | LOONGSON_CFG1_LSSYNCI | LOONGSON_CFG1_LLSYNC | LOONGSON_CFG1_TGTSYNC;
            (*c).loongson3_cpucfg_data[1] |= LOONGSON_CFG2_LBT1 | LOONGSON_CFG2_LBT2 | LOONGSON_CFG2_LBTMMU | LOONGSON_CFG2_LPMP | LOONGSON_CFG2_LPM_REV1 | LOONGSON_CFG2_LVZ_REV1;
            (*c).loongson3_cpucfg_data[2] |= LOONGSON_CFG3_LCAM_REV1 | LOONGSON_CFG3_LCAMNUM_REV1 | LOONGSON_CFG3_LCAMKW_REV1 | LOONGSON_CFG3_LCAMVW_REV1;
        }
        _ => {
            /* It is possible that some future Loongson cores still do
             * not have CPUCFG, so do not emulate anything for these
             * cores.
             */
            return;
        }
    }

    /* This feature is set by firmware, but all known Loongson-64 systems
     * are configured this way.
     */
    (*c).loongson3_cpucfg_data[0] |= LOONGSON_CFG1_CDMAP;

    /* Patch in dynamically probed bits. */
    patch_cpucfg_sel1(c);
    patch_cpucfg_sel2(c);
    patch_cpucfg_sel3(c);

    /* We have usable CPUCFG now, emulated or not.
     * Announce CPUCFG availability to userspace via hwcap.
     */
    elf_hwcap |= HWCAP_LOONGSON_CPUCFG;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
