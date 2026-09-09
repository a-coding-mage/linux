/* Functions for XAUI initialization, configuration, and monitoring. */

/* C headers and build-time feature definitions are supplied by the surrounding
 * OCTEON translation unit. */

extern "C" {
    fn cvmx_read_csr(address: u64) -> u64;
    fn cvmx_write_csr(address: u64, value: u64);
    fn __cvmx_helper_setup_gmx(interface: i32, num_ports: i32);
    fn octeon_has_feature(feature: i32) -> i32;
    fn cvmx_helper_get_ipd_port(interface: i32, port: i32) -> i32;
    fn cvmx_helper_get_interface_num(ipd_port: i32) -> i32;
    fn __cvmx_interrupt_pcsx_intx_en_reg_enable(lane: i32, interface: i32);
    fn __cvmx_interrupt_pcsxx_int_en_reg_enable(interface: i32);
    fn __cvmx_interrupt_gmxx_enable(interface: i32);
}

pub unsafe fn __cvmx_helper_xaui_enumerate(interface: i32) -> i32 {
    let mut gmx_hg2_control: cvmx_gmxx_hg2_control = core::mem::zeroed();
    gmx_hg2_control.u64 = cvmx_read_csr(CVMX_GMXX_HG2_CONTROL(interface));
    if gmx_hg2_control.s.hg2tx_en != 0 { 16 } else { 1 }
}

pub unsafe fn __cvmx_helper_xaui_probe(interface: i32) -> i32 {
    let mut mode: cvmx_gmxx_inf_mode = core::mem::zeroed();
    mode.u64 = cvmx_read_csr(CVMX_GMXX_INF_MODE(interface));
    mode.s.en = 1;
    cvmx_write_csr(CVMX_GMXX_INF_MODE(interface), mode.u64);
    __cvmx_helper_setup_gmx(interface, 1);

    for i in 0..16 {
        let mut pko_mem_port_ptrs: cvmx_pko_mem_port_ptrs = core::mem::zeroed();
        pko_mem_port_ptrs.u64 = 0;
        pko_mem_port_ptrs.s.static_p = 0;
        pko_mem_port_ptrs.s.qos_mask = 0xff;
        pko_mem_port_ptrs.s.eid = interface * 4;
        pko_mem_port_ptrs.s.pid = interface * 16 + i;
        cvmx_write_csr(CVMX_PKO_MEM_PORT_PTRS, pko_mem_port_ptrs.u64);
    }
    __cvmx_helper_xaui_enumerate(interface)
}

pub unsafe fn __cvmx_helper_xaui_enable(interface: i32) -> i32 {
    let mut gmx_cfg: cvmx_gmxx_prtx_cfg = core::mem::zeroed();
    let mut xaui_ctl: cvmx_pcsxx_control1_reg = core::mem::zeroed();
    let mut xaui_misc_ctl: cvmx_pcsxx_misc_ctl_reg = core::mem::zeroed();
    let mut gmx_xaui_tx_ctl: cvmx_gmxx_tx_xaui_ctl = core::mem::zeroed();
    let mut gmx_rx_int_en: cvmx_gmxx_rxx_int_en = core::mem::zeroed();
    let mut gmx_tx_int_en: cvmx_gmxx_tx_int_en = core::mem::zeroed();
    let mut pcsx_int_en_reg: cvmx_pcsxx_int_en_reg = core::mem::zeroed();

    if octeon_has_feature(OCTEON_FEATURE_PKND) != 0 {
        gmx_cfg.u64 = cvmx_read_csr(CVMX_GMXX_PRTX_CFG(0, interface));
        gmx_cfg.s.pknd = cvmx_helper_get_ipd_port(interface, 0);
        cvmx_write_csr(CVMX_GMXX_PRTX_CFG(0, interface), gmx_cfg.u64);
    }
    xaui_misc_ctl.u64 = cvmx_read_csr(CVMX_PCSXX_MISC_CTL_REG(interface));
    xaui_misc_ctl.s.gmxeno = 1;
    cvmx_write_csr(CVMX_PCSXX_MISC_CTL_REG(interface), xaui_misc_ctl.u64);
    gmx_rx_int_en.u64 = cvmx_read_csr(CVMX_GMXX_RXX_INT_EN(0, interface));
    cvmx_write_csr(CVMX_GMXX_RXX_INT_EN(0, interface), 0);
    gmx_tx_int_en.u64 = cvmx_read_csr(CVMX_GMXX_TX_INT_EN(interface));
    cvmx_write_csr(CVMX_GMXX_TX_INT_EN(interface), 0);
    pcsx_int_en_reg.u64 = cvmx_read_csr(CVMX_PCSXX_INT_EN_REG(interface));
    cvmx_write_csr(CVMX_PCSXX_INT_EN_REG(interface), 0);

    gmx_xaui_tx_ctl.u64 = cvmx_read_csr(CVMX_GMXX_TX_XAUI_CTL(interface));
    gmx_xaui_tx_ctl.s.dic_en = 1;
    gmx_xaui_tx_ctl.s.uni_en = 0;
    cvmx_write_csr(CVMX_GMXX_TX_XAUI_CTL(interface), gmx_xaui_tx_ctl.u64);
    xaui_ctl.u64 = cvmx_read_csr(CVMX_PCSXX_CONTROL1_REG(interface));
    xaui_ctl.s.lo_pwr = 0;
    if !OCTEON_IS_MODEL(OCTEON_CN66XX) && !OCTEON_IS_MODEL(OCTEON_CN68XX_PASS1_X) && !OCTEON_IS_MODEL(OCTEON_CN68XX_PASS2_X) { xaui_ctl.s.reset = 1; }
    cvmx_write_csr(CVMX_PCSXX_CONTROL1_REG(interface), xaui_ctl.u64);
    if CVMX_WAIT_FOR_FIELD64(CVMX_PCSXX_CONTROL1_REG(interface), cvmx_pcsxx_control1_reg, reset, ==, 0, 10000) != 0 { return -1; }
    if CVMX_WAIT_FOR_FIELD64(CVMX_PCSXX_10GBX_STATUS_REG(interface), cvmx_pcsxx_10gbx_status_reg, alignd, ==, 1, 10000) != 0 { return -1; }
    if CVMX_WAIT_FOR_FIELD64(CVMX_GMXX_RX_XAUI_CTL(interface), cvmx_gmxx_rx_xaui_ctl, status, ==, 0, 10000) != 0 { return -1; }

    gmx_cfg.u64 = cvmx_read_csr(CVMX_GMXX_PRTX_CFG(0, interface));
    gmx_cfg.s.en = 0;
    cvmx_write_csr(CVMX_GMXX_PRTX_CFG(0, interface), gmx_cfg.u64);
    if CVMX_WAIT_FOR_FIELD64(CVMX_GMXX_PRTX_CFG(0, interface), cvmx_gmxx_prtx_cfg, rx_idle, ==, 1, 10000) != 0 { return -1; }
    if CVMX_WAIT_FOR_FIELD64(CVMX_GMXX_PRTX_CFG(0, interface), cvmx_gmxx_prtx_cfg, tx_idle, ==, 1, 10000) != 0 { return -1; }
    gmx_cfg.u64 = cvmx_read_csr(CVMX_GMXX_PRTX_CFG(0, interface));
    gmx_cfg.s.speed = 1; gmx_cfg.s.speed_msb = 0; gmx_cfg.s.slottime = 1;
    cvmx_write_csr(CVMX_GMXX_TX_PRTS(interface), 1);
    cvmx_write_csr(CVMX_GMXX_TXX_SLOT(0, interface), 512);
    cvmx_write_csr(CVMX_GMXX_TXX_BURST(0, interface), 8192);
    cvmx_write_csr(CVMX_GMXX_PRTX_CFG(0, interface), gmx_cfg.u64);
    cvmx_write_csr(CVMX_GMXX_RXX_INT_REG(0, interface), cvmx_read_csr(CVMX_GMXX_RXX_INT_REG(0, interface)));
    cvmx_write_csr(CVMX_GMXX_TX_INT_REG(interface), cvmx_read_csr(CVMX_GMXX_TX_INT_REG(interface)));
    cvmx_write_csr(CVMX_PCSXX_INT_REG(interface), cvmx_read_csr(CVMX_PCSXX_INT_REG(interface)));
    if CVMX_WAIT_FOR_FIELD64(CVMX_PCSXX_STATUS1_REG(interface), cvmx_pcsxx_status1_reg, rcv_lnk, ==, 1, 10000) != 0 { return -1; }
    if CVMX_WAIT_FOR_FIELD64(CVMX_PCSXX_STATUS2_REG(interface), cvmx_pcsxx_status2_reg, xmtflt, ==, 0, 10000) != 0 { return -1; }
    if CVMX_WAIT_FOR_FIELD64(CVMX_PCSXX_STATUS2_REG(interface), cvmx_pcsxx_status2_reg, rcvflt, ==, 0, 10000) != 0 { return -1; }
    cvmx_write_csr(CVMX_GMXX_RXX_INT_EN(0, interface), gmx_rx_int_en.u64);
    cvmx_write_csr(CVMX_GMXX_TX_INT_EN(interface), gmx_tx_int_en.u64);
    cvmx_write_csr(CVMX_PCSXX_INT_EN_REG(interface), pcsx_int_en_reg.u64);
    xaui_misc_ctl.s.gmxeno = 0;
    cvmx_write_csr(CVMX_PCSXX_MISC_CTL_REG(interface), xaui_misc_ctl.u64);
    gmx_cfg.u64 = cvmx_read_csr(CVMX_GMXX_PRTX_CFG(0, interface)); gmx_cfg.s.en = 1;
    cvmx_write_csr(CVMX_GMXX_PRTX_CFG(0, interface), gmx_cfg.u64);
    __cvmx_interrupt_pcsx_intx_en_reg_enable(0, interface); __cvmx_interrupt_pcsx_intx_en_reg_enable(1, interface);
    __cvmx_interrupt_pcsx_intx_en_reg_enable(2, interface); __cvmx_interrupt_pcsx_intx_en_reg_enable(3, interface);
    __cvmx_interrupt_pcsxx_int_en_reg_enable(interface); __cvmx_interrupt_gmxx_enable(interface);
    0
}

pub unsafe fn __cvmx_helper_xaui_link_get(ipd_port: i32) -> cvmx_helper_link_info {
    let interface = cvmx_helper_get_interface_num(ipd_port);
    let mut tx: cvmx_gmxx_tx_xaui_ctl = core::mem::zeroed(); let mut rx: cvmx_gmxx_rx_xaui_ctl = core::mem::zeroed();
    let mut status: cvmx_pcsxx_status1_reg = core::mem::zeroed(); let mut result: cvmx_helper_link_info = core::mem::zeroed();
    tx.u64 = cvmx_read_csr(CVMX_GMXX_TX_XAUI_CTL(interface)); rx.u64 = cvmx_read_csr(CVMX_GMXX_RX_XAUI_CTL(interface));
    status.u64 = cvmx_read_csr(CVMX_PCSXX_STATUS1_REG(interface)); result.u64 = 0;
    if tx.s.ls == 0 && rx.s.status == 0 && status.s.rcv_lnk == 1 { result.s.link_up = 1; result.s.full_duplex = 1; result.s.speed = 10000; }
    else { cvmx_write_csr(CVMX_GMXX_RXX_INT_EN(0, interface), 0); cvmx_write_csr(CVMX_GMXX_TX_INT_EN(interface), 0); cvmx_write_csr(CVMX_PCSXX_INT_EN_REG(interface), 0); }
    result
}

pub unsafe fn __cvmx_helper_xaui_link_set(ipd_port: i32, link_info: cvmx_helper_link_info) -> i32 {
    let interface = cvmx_helper_get_interface_num(ipd_port);
    let mut tx: cvmx_gmxx_tx_xaui_ctl = core::mem::zeroed(); let mut rx: cvmx_gmxx_rx_xaui_ctl = core::mem::zeroed();
    tx.u64 = cvmx_read_csr(CVMX_GMXX_TX_XAUI_CTL(interface)); rx.u64 = cvmx_read_csr(CVMX_GMXX_RX_XAUI_CTL(interface));
    if link_info.s.link_up == 0 || (tx.s.ls == 0 && rx.s.status == 0) { return 0; }
    __cvmx_helper_xaui_enable(interface)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
