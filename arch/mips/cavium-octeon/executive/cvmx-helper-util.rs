/* Small helper utilities.  C header dependencies are supplied by the
 * surrounding OCTEON translation. */

use core::ffi::c_char;

extern "C" {
    fn cvmx_write_csr(address: u64, value: u64);
    fn cvmx_read_csr(address: u64) -> u64;
    fn cvmx_helper_get_first_ipd_port(interface: i32) -> i32;
    fn cvmx_helper_get_last_ipd_port(interface: i32) -> i32;
    fn cvmx_helper_interface_get_mode(interface: i32) -> cvmx_helper_interface_mode;
    fn cvmx_dprintf(format: *const c_char, ...);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cvmx_helper_interface_mode {
    CVMX_HELPER_INTERFACE_MODE_DISABLED,
    CVMX_HELPER_INTERFACE_MODE_RGMII,
    CVMX_HELPER_INTERFACE_MODE_GMII,
    CVMX_HELPER_INTERFACE_MODE_SPI,
    CVMX_HELPER_INTERFACE_MODE_PCIE,
    CVMX_HELPER_INTERFACE_MODE_XAUI,
    CVMX_HELPER_INTERFACE_MODE_SGMII,
    CVMX_HELPER_INTERFACE_MODE_PICMG,
    CVMX_HELPER_INTERFACE_MODE_NPI,
    CVMX_HELPER_INTERFACE_MODE_LOOP,
}

extern "C" {
    static CVMX_CN30XX: u32;
    static CVMX_CN31XX: u32;
    static CVMX_CN50XX: u32;
    fn OCTEON_IS_MODEL(model: u32) -> bool;
}

pub unsafe fn cvmx_helper_interface_mode_to_string(
    mode: cvmx_helper_interface_mode,
) -> *const c_char {
    match mode {
        cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_DISABLED => b"DISABLED\0".as_ptr() as _,
        cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_RGMII => b"RGMII\0".as_ptr() as _,
        cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_GMII => b"GMII\0".as_ptr() as _,
        cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_SPI => b"SPI\0".as_ptr() as _,
        cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_PCIE => b"PCIE\0".as_ptr() as _,
        cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_XAUI => b"XAUI\0".as_ptr() as _,
        cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_SGMII => b"SGMII\0".as_ptr() as _,
        cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_PICMG => b"PICMG\0".as_ptr() as _,
        cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_NPI => b"NPI\0".as_ptr() as _,
        cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_LOOP => b"LOOP\0".as_ptr() as _,
    }
}

/* Register addresses and bitfield helpers are provided by the translated
 * OCTEON register definitions. */
extern "C" {
    fn CVMX_IPD_QOSX_RED_MARKS(queue: i32) -> u64;
    fn CVMX_IPD_RED_QUEX_PARAM(queue: i32) -> u64;
    fn CVMX_IPD_PORTX_BP_PAGE_CNT(port: i32) -> u64;
    fn CVMX_IPD_BP_PRT_RED_END() -> u64;
    fn CVMX_IPD_RED_PORT_ENABLE() -> u64;
    fn CVMX_GMXX_TX_PRTS(interface: i32) -> u64;
    fn CVMX_GMXX_RX_PRTS(interface: i32) -> u64;
    fn CVMX_PKO_REG_GMX_PORT_MODE() -> u64;
    fn CVMX_GMXX_TXX_THRESH(index: i32, interface: i32) -> u64;
}

unsafe fn cvmx_helper_setup_red_queue(queue: i32, pass_thresh: i32, drop_thresh: i32) -> i32 {
    let mut red_marks = 0u64;
    red_marks |= (drop_thresh as u64) & 0xff;
    red_marks |= ((pass_thresh as u64) & 0xff) << 8;
    cvmx_write_csr(CVMX_IPD_QOSX_RED_MARKS(queue), red_marks);
    let mut red_param = 0u64;
    red_param |= (((255u64 << 24) / (pass_thresh - drop_thresh) as u64) & 0xff) << 24;
    red_param |= 1 << 32;
    red_param |= 255 << 40;
    red_param |= 1 << 48;
    cvmx_write_csr(CVMX_IPD_RED_QUEX_PARAM(queue), red_param);
    0
}

pub unsafe fn cvmx_helper_setup_red(pass_thresh: i32, drop_thresh: i32) -> i32 {
    let page_cnt = (100u64 << 8) & !1;
    for interface in 0..2 {
        let mut port = cvmx_helper_get_first_ipd_port(interface);
        while port < cvmx_helper_get_last_ipd_port(interface) {
            cvmx_write_csr(CVMX_IPD_PORTX_BP_PAGE_CNT(port), page_cnt);
            port += 1;
        }
    }
    for queue in 0..8 { cvmx_helper_setup_red_queue(queue, pass_thresh, drop_thresh); }
    cvmx_write_csr(CVMX_IPD_BP_PRT_RED_END(), 0);
    cvmx_write_csr(CVMX_IPD_RED_PORT_ENABLE(), (0xfffffffffu64) | (10000 << 32) | (10000 << 48));
    0
}

pub unsafe fn __cvmx_helper_setup_gmx(interface: i32, mut num_ports: i32) -> i32 {
    let mut tx = cvmx_read_csr(CVMX_GMXX_TX_PRTS(interface));
    tx = (tx & !0xff) | num_ports as u64;
    cvmx_write_csr(CVMX_GMXX_TX_PRTS(interface), tx);
    let mode = cvmx_helper_interface_get_mode(interface);
    if matches!(mode, cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_RGMII | cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_SGMII | cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_GMII | cvmx_helper_interface_mode::CVMX_HELPER_INTERFACE_MODE_XAUI) {
        if num_ports > 4 { return -1; }
        let mut rx = cvmx_read_csr(CVMX_GMXX_RX_PRTS(interface));
        rx = (rx & !0xff) | num_ports as u64;
        cvmx_write_csr(CVMX_GMXX_RX_PRTS(interface), rx);
    }
    if !OCTEON_IS_MODEL(CVMX_CN30XX) && !OCTEON_IS_MODEL(CVMX_CN31XX) && !OCTEON_IS_MODEL(CVMX_CN50XX) {
        let mut pko = cvmx_read_csr(CVMX_PKO_REG_GMX_PORT_MODE());
        let value = if num_ports == 1 { 4 } else if num_ports == 2 { 3 } else if num_ports <= 4 { 2 } else if num_ports <= 8 { 1 } else { 0 };
        let shift = if interface == 0 { 0 } else { 4 };
        pko = (pko & !(7 << shift)) | ((value as u64) << shift);
        cvmx_write_csr(CVMX_PKO_REG_GMX_PORT_MODE(), pko);
    }
    let mut threshold = if OCTEON_IS_MODEL(CVMX_CN30XX) || OCTEON_IS_MODEL(CVMX_CN31XX) || OCTEON_IS_MODEL(CVMX_CN50XX) { 0x40 } else if num_ports <= 1 { 0x100 } else if num_ports == 2 { 0x80 } else { 0x40 };
    if num_ports > 4 { num_ports = 4; }
    for index in 0..num_ports { cvmx_write_csr(CVMX_GMXX_TXX_THRESH(index, interface), threshold); }
    0
}

pub fn cvmx_helper_get_ipd_port(interface: i32, port: i32) -> i32 {
    match interface { 0 => port, 1 => port + 16, 2 => port + 32, 3 => port + 36, 4 => port + 40, 5 => port + 44, _ => -1 }
}

pub unsafe fn cvmx_helper_get_interface_num(ipd_port: i32) -> i32 {
    if ipd_port < 16 { 0 } else if ipd_port < 32 { 1 } else if ipd_port < 36 { 2 } else if ipd_port < 40 { 3 } else if ipd_port < 44 { 4 } else if ipd_port < 48 { 5 } else { -1 }
}

pub fn cvmx_helper_get_interface_index_num(ipd_port: i32) -> i32 {
    if ipd_port < 32 { ipd_port & 15 } else if ipd_port < 48 { ipd_port & 3 } else { -1 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
