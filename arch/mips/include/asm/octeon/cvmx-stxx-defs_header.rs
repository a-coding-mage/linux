/* Translated from cvmx-stxx-defs.h. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub type uint64_t = u64;

extern "C" {
    pub fn __cvmx_interrupt_stxx_int_msk_enable(index: i32);
}

// CVMX_ADD_IO_SEG is supplied by the surrounding low-level platform code.
macro_rules! stxx_addr {
    ($name:ident, $base:expr, $block:ident) => {
        #[inline]
        pub const fn $name($block: u64) -> u64 {
            CVMX_ADD_IO_SEG($base) + (($block & 1) * 0x8000000u64)
        }
    };
}

stxx_addr!(CVMX_STXX_ARB_CTL, 0x0001180090000608u64, block_id);
stxx_addr!(CVMX_STXX_BCKPRS_CNT, 0x0001180090000688u64, block_id);
stxx_addr!(CVMX_STXX_COM_CTL, 0x0001180090000600u64, block_id);
stxx_addr!(CVMX_STXX_DIP_CNT, 0x0001180090000690u64, block_id);
stxx_addr!(CVMX_STXX_IGN_CAL, 0x0001180090000610u64, block_id);
stxx_addr!(CVMX_STXX_INT_MSK, 0x00011800900006A0u64, block_id);
stxx_addr!(CVMX_STXX_INT_REG, 0x0001180090000698u64, block_id);
stxx_addr!(CVMX_STXX_INT_SYNC, 0x00011800900006A8u64, block_id);
stxx_addr!(CVMX_STXX_MIN_BST, 0x0001180090000618u64, block_id);
stxx_addr!(CVMX_STXX_SPI4_DAT, 0x0001180090000628u64, block_id);
stxx_addr!(CVMX_STXX_SPI4_STAT, 0x0001180090000630u64, block_id);
stxx_addr!(CVMX_STXX_STAT_BYTES_HI, 0x0001180090000648u64, block_id);
stxx_addr!(CVMX_STXX_STAT_BYTES_LO, 0x0001180090000680u64, block_id);
stxx_addr!(CVMX_STXX_STAT_CTL, 0x0001180090000638u64, block_id);
stxx_addr!(CVMX_STXX_STAT_PKT_XMT, 0x0001180090000640u64, block_id);

#[inline]
pub const fn CVMX_STXX_SPI4_CALX(offset: u64, block_id: u64) -> u64 {
    CVMX_ADD_IO_SEG(0x0001180090000400u64)
        + (((offset & 31) + ((block_id & 1) * 0x1000000u64)) * 8)
}

// C bitfields are represented by their containing word. Masks and shifts retain
// the declared field names and widths without relying on compiler bitfield ABI.
macro_rules! reg_union {
    ($u:ident, $s:ident) => {
        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct $s { pub bits: u64 }
        #[repr(C)]
        pub union $u { pub u64: u64, pub s: $s }
    };
}

reg_union!(cvmx_stxx_arb_ctl, cvmx_stxx_arb_ctl_s);
reg_union!(cvmx_stxx_bckprs_cnt, cvmx_stxx_bckprs_cnt_s);
reg_union!(cvmx_stxx_com_ctl, cvmx_stxx_com_ctl_s);
reg_union!(cvmx_stxx_dip_cnt, cvmx_stxx_dip_cnt_s);
reg_union!(cvmx_stxx_ign_cal, cvmx_stxx_ign_cal_s);
reg_union!(cvmx_stxx_int_msk, cvmx_stxx_int_msk_s);
reg_union!(cvmx_stxx_int_reg, cvmx_stxx_int_reg_s);
reg_union!(cvmx_stxx_int_sync, cvmx_stxx_int_sync_s);
reg_union!(cvmx_stxx_min_bst, cvmx_stxx_min_bst_s);
reg_union!(cvmx_stxx_spi4_calx, cvmx_stxx_spi4_calx_s);
reg_union!(cvmx_stxx_spi4_dat, cvmx_stxx_spi4_dat_s);
reg_union!(cvmx_stxx_spi4_stat, cvmx_stxx_spi4_stat_s);
reg_union!(cvmx_stxx_stat_bytes_hi, cvmx_stxx_stat_bytes_hi_s);
reg_union!(cvmx_stxx_stat_bytes_lo, cvmx_stxx_stat_bytes_lo_s);
reg_union!(cvmx_stxx_stat_ctl, cvmx_stxx_stat_ctl_s);
reg_union!(cvmx_stxx_stat_pkt_xmt, cvmx_stxx_stat_pkt_xmt_s);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
