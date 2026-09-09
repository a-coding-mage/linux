/*
 * File defining checks for different Octeon features.
 *
 * C dependencies from asm/octeon/cvmx-mio-defs.h and cvmx-rnm-defs.h are
 * supplied externally.
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum octeon_feature {
    // CN68XX uses port kinds for packet interface
    OCTEON_FEATURE_PKND,
    // CN68XX has different fields in word0 - word2
    OCTEON_FEATURE_CN68XX_WQE,
    // CN5XXX and higher support atomic add instructions to memory (saa/saad).
    OCTEON_FEATURE_SAAD,
    // ZIP offload engine
    OCTEON_FEATURE_ZIP,
    OCTEON_FEATURE_DORM_CRYPTO,
    // PCI express
    OCTEON_FEATURE_PCIE,
    // SRIOs
    OCTEON_FEATURE_SRIO,
    // Interlaken
    OCTEON_FEATURE_ILK,
    // Internal memory for storing cryptographic keys
    OCTEON_FEATURE_KEY_MEMORY,
    // LED controller for banks of external LEDs
    OCTEON_FEATURE_LED_CONTROLLER,
    // Trace buffer
    OCTEON_FEATURE_TRA,
    // Management port
    OCTEON_FEATURE_MGMT_PORT,
    // RAID unit
    OCTEON_FEATURE_RAID,
    // Builtin USB
    OCTEON_FEATURE_USB,
    // IPD can run without using work queue entries
    OCTEON_FEATURE_NO_WPTR,
    // DFA state machines
    OCTEON_FEATURE_DFA,
    // MDIO clause 45 transactions
    OCTEON_FEATURE_MDIO_CLAUSE_45,
    // CN52XX and CN56XX used NPEI for PCIe access; newer chips use SLI+DPI.
    OCTEON_FEATURE_NPEI,
    OCTEON_FEATURE_HFA,
    OCTEON_FEATURE_DFM,
    OCTEON_FEATURE_CIU2,
    OCTEON_FEATURE_CIU3,
    // FPA first seen on 78XX
    OCTEON_FEATURE_FPA3,
    OCTEON_FEATURE_FAU,
    OCTEON_MAX_FEATURE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum octeon_feature_bits {
    // Crypto acceleration using COP2
    OCTEON_HAS_CRYPTO = 0x0001,
}

extern "C" {
    pub static mut __octeon_feature_bits: octeon_feature_bits;
}

/// Check if this OCTEON has crypto acceleration support.
/// Returns non-zero if the feature exists.
#[inline]
pub unsafe fn octeon_has_crypto() -> i32 {
    __octeon_feature_bits as i32 & octeon_feature_bits::OCTEON_HAS_CRYPTO as i32
}

/// Determine if the current Octeon supports a specific feature.
/// `OCTEON_IS_MODEL`, `cvmx_read_csr`, and the model constants are external
/// dependencies supplied by the translated Octeon headers.
#[inline]
pub unsafe fn octeon_has_feature(feature: octeon_feature) -> bool {
    match feature {
        octeon_feature::OCTEON_FEATURE_SAAD => !OCTEON_IS_MODEL(OCTEON_CN3XXX),
        octeon_feature::OCTEON_FEATURE_DORM_CRYPTO => {
            if OCTEON_IS_MODEL(OCTEON_CN6XXX) {
                let mut fus_2: cvmx_mio_fus_dat2 = core::mem::zeroed();
                fus_2.u64 = cvmx_read_csr(CVMX_MIO_FUS_DAT2);
                !fus_2.s.nocrypto && !fus_2.s.nomul && fus_2.s.dorm_crypto
            } else { false }
        }
        octeon_feature::OCTEON_FEATURE_PCIE => OCTEON_IS_MODEL(OCTEON_CN56XX) || OCTEON_IS_MODEL(OCTEON_CN52XX) || OCTEON_IS_MODEL(OCTEON_CN6XXX) || OCTEON_IS_MODEL(OCTEON_CN7XXX),
        octeon_feature::OCTEON_FEATURE_SRIO => OCTEON_IS_MODEL(OCTEON_CN63XX) || OCTEON_IS_MODEL(OCTEON_CN66XX),
        octeon_feature::OCTEON_FEATURE_ILK => OCTEON_IS_MODEL(OCTEON_CN68XX),
        octeon_feature::OCTEON_FEATURE_KEY_MEMORY => OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) || OCTEON_IS_MODEL(OCTEON_CN56XX) || OCTEON_IS_MODEL(OCTEON_CN6XXX),
        octeon_feature::OCTEON_FEATURE_LED_CONTROLLER => OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) || OCTEON_IS_MODEL(OCTEON_CN56XX),
        octeon_feature::OCTEON_FEATURE_TRA => !OCTEON_IS_MODEL(OCTEON_CN30XX) && !OCTEON_IS_MODEL(OCTEON_CN50XX),
        octeon_feature::OCTEON_FEATURE_MGMT_PORT | octeon_feature::OCTEON_FEATURE_RAID => OCTEON_IS_MODEL(OCTEON_CN56XX) || OCTEON_IS_MODEL(OCTEON_CN52XX) || OCTEON_IS_MODEL(OCTEON_CN6XXX),
        octeon_feature::OCTEON_FEATURE_USB => !OCTEON_IS_MODEL(OCTEON_CN38XX) && !OCTEON_IS_MODEL(OCTEON_CN58XX),
        octeon_feature::OCTEON_FEATURE_NO_WPTR => (OCTEON_IS_MODEL(OCTEON_CN56XX) || OCTEON_IS_MODEL(OCTEON_CN52XX) || OCTEON_IS_MODEL(OCTEON_CN6XXX)) && !OCTEON_IS_MODEL(OCTEON_CN56XX_PASS1_X) && !OCTEON_IS_MODEL(OCTEON_CN52XX_PASS1_X),
        octeon_feature::OCTEON_FEATURE_MDIO_CLAUSE_45 => !OCTEON_IS_MODEL(OCTEON_CN3XXX) && !OCTEON_IS_MODEL(OCTEON_CN58XX) && !OCTEON_IS_MODEL(OCTEON_CN50XX),
        octeon_feature::OCTEON_FEATURE_NPEI => OCTEON_IS_MODEL(OCTEON_CN56XX) || OCTEON_IS_MODEL(OCTEON_CN52XX),
        octeon_feature::OCTEON_FEATURE_PKND | octeon_feature::OCTEON_FEATURE_CN68XX_WQE | octeon_feature::OCTEON_FEATURE_CIU2 => OCTEON_IS_MODEL(OCTEON_CN68XX),
        octeon_feature::OCTEON_FEATURE_CIU3 | octeon_feature::OCTEON_FEATURE_FPA3 => OCTEON_IS_MODEL(OCTEON_CN78XX) || OCTEON_IS_MODEL(OCTEON_CNF75XX) || OCTEON_IS_MODEL(OCTEON_CN73XX),
        octeon_feature::OCTEON_FEATURE_FAU => !OCTEON_IS_MODEL(OCTEON_CN78XX) && !OCTEON_IS_MODEL(OCTEON_CNF75XX) && !OCTEON_IS_MODEL(OCTEON_CN73XX),
        _ => false,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
