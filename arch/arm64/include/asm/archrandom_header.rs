/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.
// ARM_SMCCC_TRNG_MIN_VERSION
pub const ARM_SMCCC_TRNG_MIN_VERSION: c_ulong = 0x10000;

extern "C" {
    pub static mut smccc_trng_available: bool;
}

#[inline]
pub unsafe fn smccc_probe_trng() -> bool {
    let mut res: arm_smccc_res = core::mem::zeroed();

    arm_smccc_1_1_invoke(ARM_SMCCC_TRNG_VERSION, &mut res);
    if (res.a0 as i32) < 0 {
        return false;
    }

    res.a0 >= ARM_SMCCC_TRNG_MIN_VERSION
}

#[inline]
pub unsafe fn __arm64_rndr(v: *mut c_ulong) -> bool {
    let mut ok: bool;

    /*
     * Reads of RNDR set PSTATE.NZCV to 0b0000 on success,
     * and set PSTATE.NZCV to 0b0100 otherwise.
     */
    core::arch::asm!(
        "mrs {value}, S3_3_C2_C4_0",
        "cset {ok:w}, ne",
        value = out(reg) *v,
        ok = out(reg) ok,
        options(nostack)
    );

    ok
}

#[inline]
pub unsafe fn __arm64_rndrrs(v: *mut c_ulong) -> bool {
    let mut ok: bool;

    /*
     * Reads of RNDRRS set PSTATE.NZCV to 0b0000 on success,
     * and set PSTATE.NZCV to 0b0100 otherwise.
     */
    core::arch::asm!(
        "mrs {value}, S3_3_C2_C4_1",
        "cset {ok:w}, ne",
        value = out(reg) *v,
        ok = out(reg) ok,
        options(nostack)
    );

    ok
}

#[inline(always)]
pub unsafe fn __cpu_has_rng() -> bool {
    if !system_capabilities_finalized() && !preemptible() {
        return this_cpu_has_cap(ARM64_HAS_RNG);
    }
    alternative_has_cap_unlikely(ARM64_HAS_RNG)
}

#[inline]
pub unsafe fn arch_get_random_longs(v: *mut c_ulong, max_longs: usize) -> usize {
    /*
     * Only support the generic interface after we have detected
     * the system wide capability, avoiding complexity with the
     * cpufeature code and with potential scheduling between CPUs
     * with and without the feature.
     */
    if max_longs != 0 && __cpu_has_rng() && __arm64_rndr(v) {
        return 1;
    }
    0
}

#[inline]
pub unsafe fn arch_get_random_seed_longs(v: *mut c_ulong, mut max_longs: usize) -> usize {
    if max_longs == 0 {
        return 0;
    }

    /*
     * We prefer the SMCCC call, since its semantics (return actual
     * hardware backed entropy) is closer to the idea behind this
     * function here than what even the RNDRSS register provides
     * (the output of a pseudo RNG freshly seeded by a TRNG).
     */
    if smccc_trng_available {
        let mut res: arm_smccc_res = core::mem::zeroed();

        max_longs = core::cmp::min(3, max_longs);
        arm_smccc_1_1_invoke(ARM_SMCCC_TRNG_RND64, max_longs * 64, &mut res);
        if (res.a0 as i32) >= 0 {
            let mut p = v;
            match max_longs {
                3 => {
                    *p = res.a1;
                    p = p.add(1);
                }
                _ => {}
            }
            if max_longs >= 2 {
                *p = res.a2;
                p = p.add(1);
            }
            if max_longs >= 1 {
                *p = res.a3;
            }
            return max_longs;
        }
    }

    /*
     * RNDRRS is not backed by an entropy source but by a DRBG that is
     * reseeded after each invocation. This is not a 100% fit but good
     * enough to implement this API if no other entropy source exists.
     */
    if __cpu_has_rng() && __arm64_rndrrs(v) {
        return 1;
    }

    0
}

#[inline]
pub unsafe fn __early_cpu_has_rndr() -> bool {
    /* Open code as we run prior to the first call to cpufeature. */
    let ftr: c_ulong = read_sysreg_s(SYS_ID_AA64ISAR0_EL1);
    ((ftr >> ID_AA64ISAR0_EL1_RNDR_SHIFT) & 0xf) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
