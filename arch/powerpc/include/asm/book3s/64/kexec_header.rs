/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: symbols from <asm/plpar_wrappers.h> are supplied by
// other translated files.

#[inline]
pub unsafe fn reset_sprs() {
    if cpu_has_feature(CPU_FTR_ARCH_206) {
        mtspr(SPRN_AMR, 0);
        mtspr(SPRN_UAMOR, 0);
    }

    if cpu_has_feature(CPU_FTR_ARCH_207S) {
        mtspr(SPRN_IAMR, 0);
        if cpu_has_feature(CPU_FTR_HVMODE) {
            mtspr(SPRN_CIABR, 0);
        } else {
            plpar_set_ciabr(0);
        }
    }

    if cpu_has_feature(CPU_FTR_ARCH_31) {
        mtspr(SPRN_DEXCR, 0);
        mtspr(SPRN_HASHKEYR, 0);
    }

    /*  Do we need isync()? We are going via a kexec reset */
    isync();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
