/* Translated from cvmx-pcsxx-defs.h. */

/* The following symbols are supplied by the surrounding OCTEON headers. */
extern "C" {
    fn cvmx_get_octeon_family() -> u32;
    fn __cvmx_interrupt_pcsxx_int_en_reg_enable(index: i32);
}

/* C bit-field layout is retained as raw register words; the listed masks and
 * positions document the corresponding source fields. */
#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_pcsxx_register {
    pub u64: u64,
}

macro_rules! pcsxx_addr_fn {
    ($name:ident, $addr:expr) => {
        #[inline]
        pub unsafe fn $name(block_id: libc::c_ulong) -> u64 {
            match cvmx_get_octeon_family() {
                x if x == (OCTEON_CN56XX & OCTEON_FAMILY_MASK)
                    || x == (OCTEON_CN66XX & OCTEON_FAMILY_MASK)
                    || x == (OCTEON_CN61XX & OCTEON_FAMILY_MASK)
                    || x == (OCTEON_CN52XX & OCTEON_FAMILY_MASK)
                    || x == (OCTEON_CN63XX & OCTEON_FAMILY_MASK) =>
                    CVMX_ADD_IO_SEG($addr) + (block_id as u64) * 0x8000000u64,
                x if x == (OCTEON_CN68XX & OCTEON_FAMILY_MASK) =>
                    CVMX_ADD_IO_SEG($addr) + (block_id as u64) * 0x1000000u64,
                _ => CVMX_ADD_IO_SEG($addr) + (block_id as u64) * 0x1000000u64,
            }
        }
    };
}

pcsxx_addr_fn!(CVMX_PCSXX_10GBX_STATUS_REG, 0x00011800B0000828u64);
pcsxx_addr_fn!(CVMX_PCSXX_BIST_STATUS_REG, 0x00011800B0000870u64);
pcsxx_addr_fn!(CVMX_PCSXX_BIT_LOCK_STATUS_REG, 0x00011800B0000850u64);
pcsxx_addr_fn!(CVMX_PCSXX_CONTROL1_REG, 0x00011800B0000800u64);
pcsxx_addr_fn!(CVMX_PCSXX_CONTROL2_REG, 0x00011800B0000818u64);
pcsxx_addr_fn!(CVMX_PCSXX_INT_EN_REG, 0x00011800B0000860u64);
pcsxx_addr_fn!(CVMX_PCSXX_INT_REG, 0x00011800B0000858u64);
pcsxx_addr_fn!(CVMX_PCSXX_LOG_ANL_REG, 0x00011800B0000868u64);
pcsxx_addr_fn!(CVMX_PCSXX_MISC_CTL_REG, 0x00011800B0000848u64);
pcsxx_addr_fn!(CVMX_PCSXX_RX_SYNC_STATES_REG, 0x00011800B0000838u64);
pcsxx_addr_fn!(CVMX_PCSXX_SPD_ABIL_REG, 0x00011800B0000810u64);
pcsxx_addr_fn!(CVMX_PCSXX_STATUS1_REG, 0x00011800B0000808u64);
pcsxx_addr_fn!(CVMX_PCSXX_STATUS2_REG, 0x00011800B0000820u64);
pcsxx_addr_fn!(CVMX_PCSXX_TX_RX_POLARITY_REG, 0x00011800B0000840u64);
pcsxx_addr_fn!(CVMX_PCSXX_TX_RX_STATES_REG, 0x00011800B0000830u64);

/* External constants and CVMX_ADD_IO_SEG are intentionally unresolved here. */
extern "C" {
    static OCTEON_CN56XX: u32;
    static OCTEON_CN66XX: u32;
    static OCTEON_CN61XX: u32;
    static OCTEON_CN52XX: u32;
    static OCTEON_CN63XX: u32;
    static OCTEON_CN68XX: u32;
    static OCTEON_FAMILY_MASK: u32;
}

/* Register unions below preserve the C union names and raw 64-bit storage.
 * Field positions are the source bitfields, with low-order fields first in
 * little-endian builds and reversed in big-endian builds. */
pub type cvmx_pcsxx_10gbx_status_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_bist_status_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_bit_lock_status_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_control1_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_control2_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_int_en_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_int_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_log_anl_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_misc_ctl_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_rx_sync_states_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_spd_abil_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_status1_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_status2_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_tx_rx_polarity_reg = cvmx_pcsxx_register;
pub type cvmx_pcsxx_tx_rx_states_reg = cvmx_pcsxx_register;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
