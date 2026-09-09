/* Translated from cvmx-mixx-defs.h. */

/* CVMX_ADD_IO_SEG is supplied by the surrounding translation unit. */
extern "C" {
    fn CVMX_ADD_IO_SEG(address: u64) -> u64;
}

#[inline]
pub unsafe fn CVMX_MIXX_BIST(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100078) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_CTL(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100020) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_INTENA(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100050) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_IRCNT(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100030) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_IRHWM(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100028) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_IRING1(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100010) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_IRING2(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100018) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_ISR(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100048) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_ORCNT(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100040) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_ORHWM(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100038) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_ORING1(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100000) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_ORING2(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100008) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_REMCNT(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100058) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_TSCTL(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100068) + (offset & 1) * 2048 }
#[inline]
pub unsafe fn CVMX_MIXX_TSTAMP(offset: u64) -> u64 { CVMX_ADD_IO_SEG(0x0001070000100060) + (offset & 1) * 2048 }

/* C bit-fields are represented by their containing 64-bit word.  The named
 * fields and widths below are retained in comments to preserve the register
 * layout intent; access is through the raw word, as in the union's u64 arm. */
macro_rules! mixx_reg {
    ($name:ident, $s:ident) => {
        #[repr(C)]
        pub union $name { pub u64: u64, pub s: $s, pub cn52xx: $s }
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct $s { pub bits: u64 }
    };
}

mixx_reg!(cvmx_mixx_bist, cvmx_mixx_bist_s);
mixx_reg!(cvmx_mixx_ctl, cvmx_mixx_ctl_s);
mixx_reg!(cvmx_mixx_intena, cvmx_mixx_intena_s);
mixx_reg!(cvmx_mixx_ircnt, cvmx_mixx_ircnt_s);
mixx_reg!(cvmx_mixx_irhwm, cvmx_mixx_irhwm_s);
mixx_reg!(cvmx_mixx_iring1, cvmx_mixx_iring1_s);
mixx_reg!(cvmx_mixx_iring2, cvmx_mixx_iring2_s);
mixx_reg!(cvmx_mixx_isr, cvmx_mixx_isr_s);
mixx_reg!(cvmx_mixx_orcnt, cvmx_mixx_orcnt_s);
mixx_reg!(cvmx_mixx_orhwm, cvmx_mixx_orhwm_s);
mixx_reg!(cvmx_mixx_oring1, cvmx_mixx_oring1_s);
mixx_reg!(cvmx_mixx_oring2, cvmx_mixx_oring2_s);
mixx_reg!(cvmx_mixx_remcnt, cvmx_mixx_remcnt_s);
mixx_reg!(cvmx_mixx_tsctl, cvmx_mixx_tsctl_s);
mixx_reg!(cvmx_mixx_tstamp, cvmx_mixx_tstamp_s);

#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_mixx_bist_cn52xx { pub bits: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_mixx_ctl_cn52xx { pub bits: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_mixx_intena_cn52xx { pub bits: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_mixx_iring1_cn52xx { pub bits: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cvmx_mixx_isr_cn52xx { pub bits: u64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
