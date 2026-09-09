/*
 * Translated from cvmx-led-defs.h.  The C bit-field ordering is selected by
 * __BIG_ENDIAN_BITFIELD; the accessors below preserve the corresponding
 * masks and positions while keeping each register one u64 wide.
 */

#[inline]
pub const fn cvmx_led_prt_statusx(offset: u64) -> u64 {
    CVMX_ADD_IO_SEG(0x0001_1800_0000_1A80u64) + (offset & 7) * 8
}

#[inline]
pub const fn cvmx_led_udd_cntx(offset: u64) -> u64 {
    CVMX_ADD_IO_SEG(0x0001_1800_0000_1A20u64) + (offset & 1) * 8
}

#[inline]
pub const fn cvmx_led_udd_datx(offset: u64) -> u64 {
    CVMX_ADD_IO_SEG(0x0001_1800_0000_1A38u64) + (offset & 1) * 8
}

#[inline]
pub const fn cvmx_led_udd_dat_clrx(offset: u64) -> u64 {
    CVMX_ADD_IO_SEG(0x0001_1800_0000_1AC8u64) + (offset & 1) * 16
}

#[inline]
pub const fn cvmx_led_udd_dat_setx(offset: u64) -> u64 {
    CVMX_ADD_IO_SEG(0x0001_1800_0000_1AC0u64) + (offset & 1) * 16
}

pub const CVMX_LED_BLINK: u64 = CVMX_ADD_IO_SEG(0x0001_1800_0000_1A48u64);
pub const CVMX_LED_CLK_PHASE: u64 = CVMX_ADD_IO_SEG(0x0001_1800_0000_1A08u64);
pub const CVMX_LED_CYLON: u64 = CVMX_ADD_IO_SEG(0x0001_1800_0000_1AF8u64);
pub const CVMX_LED_DBG: u64 = CVMX_ADD_IO_SEG(0x0001_1800_0000_1A18u64);
pub const CVMX_LED_EN: u64 = CVMX_ADD_IO_SEG(0x0001_1800_0000_1A00u64);
pub const CVMX_LED_POLARITY: u64 = CVMX_ADD_IO_SEG(0x0001_1800_0000_1A50u64);
pub const CVMX_LED_PRT: u64 = CVMX_ADD_IO_SEG(0x0001_1800_0000_1A10u64);
pub const CVMX_LED_PRT_FMT: u64 = CVMX_ADD_IO_SEG(0x0001_1800_0000_1A30u64);

macro_rules! led_reg {
    ($union:ident, $bits:ident, $field:ident, $mask:expr, $shift:expr) => {
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct $bits { pub bits: u64 }
        impl $bits {
            #[inline] pub const fn $field(&self) -> u64 { (self.bits >> $shift) & $mask }
            #[inline] pub fn set_bits(&mut self, value: u64) {
                self.bits = (self.bits & !($mask << $shift)) | ((value & $mask) << $shift);
            }
        }
        #[repr(C)]
        pub union $union { pub u64: u64, pub s: $bits }
    };
}

led_reg!(cvmx_led_blink, cvmx_led_blink_s, rate, 0xff, 0);
led_reg!(cvmx_led_clk_phase, cvmx_led_clk_phase_s, phase, 0x7f, 0);
led_reg!(cvmx_led_cylon, cvmx_led_cylon_s, rate, 0xffff, 0);
led_reg!(cvmx_led_dbg, cvmx_led_dbg_s, dbg_en, 1, 0);
led_reg!(cvmx_led_en, cvmx_led_en_s, en, 1, 0);
led_reg!(cvmx_led_polarity, cvmx_led_polarity_s, polarity, 1, 0);
led_reg!(cvmx_led_prt, cvmx_led_prt_s, prt_en, 0xff, 0);
led_reg!(cvmx_led_prt_fmt, cvmx_led_prt_fmt_s, format, 0xf, 0);
led_reg!(cvmx_led_prt_statusx, cvmx_led_prt_statusx_s, status, 0x3f, 0);
led_reg!(cvmx_led_udd_cntx, cvmx_led_udd_cntx_s, cnt, 0x3f, 0);
led_reg!(cvmx_led_udd_datx, cvmx_led_udd_datx_s, dat, 0xffff_ffff, 0);
led_reg!(cvmx_led_udd_dat_clrx, cvmx_led_udd_dat_clrx_s, clr, 0xffff_ffff, 0);
led_reg!(cvmx_led_udd_dat_setx, cvmx_led_udd_dat_setx_s, set, 0xffff_ffff, 0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
