/*
 * Rust translation of cvmx-sli-defs.h.
 * The C bit-field records are represented by their 64-bit storage words;
 * accessors preserve the corresponding field positions and widths.
 */

extern "C" {
    fn cvmx_get_octeon_family() -> u32;
}

// These constants and the OCTEON_IS_MODEL operation are supplied by the
// surrounding OCTEON headers.
extern "C" {
    fn OCTEON_IS_MODEL(model: u32) -> bool;
}

pub unsafe fn CVMX_SLI_PCIE_MSI_RCV_FUNC() -> u64 {
    match cvmx_get_octeon_family() {
        x if x == (OCTEON_CNF71XX & OCTEON_FAMILY_MASK)
            || x == (OCTEON_CN61XX & OCTEON_FAMILY_MASK)
            || x == (OCTEON_CN63XX & OCTEON_FAMILY_MASK)
            || x == (OCTEON_CN66XX & OCTEON_FAMILY_MASK)
            || x == (OCTEON_CN68XX & OCTEON_FAMILY_MASK)
            || x == (OCTEON_CN70XX & OCTEON_FAMILY_MASK) => 0x0000_0000_0000_3cb0,
        x if x == (OCTEON_CNF75XX & OCTEON_FAMILY_MASK)
            || x == (OCTEON_CN73XX & OCTEON_FAMILY_MASK)
            || x == (OCTEON_CN78XX & OCTEON_FAMILY_MASK) => {
            if OCTEON_IS_MODEL(OCTEON_CN78XX_PASS1_X) {
                0x0000_0000_0000_3cb0
            } else {
                0x0000_0000_0002_3cb0
            }
        }
        _ => 0x0000_0000_0002_3cb0,
    }
}

#[inline]
pub unsafe fn CVMX_SLI_PCIE_MSI_RCV() -> u64 {
    CVMX_SLI_PCIE_MSI_RCV_FUNC()
}

#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct cvmx_sli_ctl_portx_s(pub u64);

#[repr(C)]
pub union cvmx_sli_ctl_portx {
    pub u64_: u64,
    pub s: cvmx_sli_ctl_portx_s,
}

#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct cvmx_sli_mem_access_ctl_s(pub u64);

#[repr(C)]
pub union cvmx_sli_mem_access_ctl {
    pub u64_: u64,
    pub s: cvmx_sli_mem_access_ctl_s,
}

#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct cvmx_sli_s2m_portx_ctl_s(pub u64);

#[repr(C)]
pub union cvmx_sli_s2m_portx_ctl {
    pub u64_: u64,
    pub s: cvmx_sli_s2m_portx_ctl_s,
}

#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct cvmx_sli_mem_access_subidx_s(pub u64);

#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct cvmx_sli_mem_access_subidx_cn68xx(pub u64);

#[repr(C)]
pub union cvmx_sli_mem_access_subidx {
    pub u64_: u64,
    pub s: cvmx_sli_mem_access_subidx_s,
    pub cn68xx: cvmx_sli_mem_access_subidx_cn68xx,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
