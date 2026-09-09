/* Translated from cvmx-helper-rgmii.c. */

pub unsafe fn __cvmx_helper_rgmii_probe(interface: i32) -> i32 {
    let mut num_ports = 0;
    let mut mode: cvmx_gmxx_inf_mode = core::mem::zeroed();
    mode.u64 = cvmx_read_csr(CVMX_GMXX_INF_MODE(interface));
    if mode.s.type_ {
        if OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) {
            cvmx_dprintf!("ERROR: RGMII initialize called in SPI interface\n");
        } else if OCTEON_IS_MODEL(OCTEON_CN31XX) || OCTEON_IS_MODEL(OCTEON_CN30XX) || OCTEON_IS_MODEL(OCTEON_CN50XX) {
            num_ports = 2;
        } else {
            cvmx_dprintf!("ERROR: Unsupported Octeon model in __cvmx_helper_rgmii_probe\n");
        }
    } else if OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) {
        num_ports = 4;
    } else if OCTEON_IS_MODEL(OCTEON_CN31XX) || OCTEON_IS_MODEL(OCTEON_CN30XX) || OCTEON_IS_MODEL(OCTEON_CN50XX) {
        num_ports = 3;
    } else {
        cvmx_dprintf!("ERROR: Unsupported Octeon model in __cvmx_helper_rgmii_probe\n");
    }
    num_ports
}

pub unsafe fn cvmx_helper_rgmii_internal_loopback(port: i32) {
    let interface = (port >> 4) & 1;
    let index = port & 0xf;
    let mut gmx_cfg: cvmx_gmxx_prtx_cfg = core::mem::zeroed();
    gmx_cfg.u64 = 0;
    gmx_cfg.s.duplex = 1; gmx_cfg.s.slottime = 1; gmx_cfg.s.speed = 1;
    cvmx_write_csr(CVMX_GMXX_TXX_CLK(index, interface), 1);
    cvmx_write_csr(CVMX_GMXX_TXX_SLOT(index, interface), 0x200);
    cvmx_write_csr(CVMX_GMXX_TXX_BURST(index, interface), 0x2000);
    cvmx_write_csr(CVMX_GMXX_PRTX_CFG(index, interface), gmx_cfg.u64);
    let mut tmp = cvmx_read_csr(CVMX_ASXX_PRT_LOOP(interface));
    cvmx_write_csr(CVMX_ASXX_PRT_LOOP(interface), (1u64 << index) | tmp);
    tmp = cvmx_read_csr(CVMX_ASXX_TX_PRT_EN(interface));
    cvmx_write_csr(CVMX_ASXX_TX_PRT_EN(interface), (1u64 << index) | tmp);
    tmp = cvmx_read_csr(CVMX_ASXX_RX_PRT_EN(interface));
    cvmx_write_csr(CVMX_ASXX_RX_PRT_EN(interface), (1u64 << index) | tmp);
    gmx_cfg.s.en = 1;
    cvmx_write_csr(CVMX_GMXX_PRTX_CFG(index, interface), gmx_cfg.u64);
}

unsafe fn __cvmx_helper_errata_asx_pass1(interface: i32, port: i32, cpu_clock_hz: i32) -> i32 {
    if cpu_clock_hz >= 325000000 && cpu_clock_hz < 375000000 { cvmx_write_csr(CVMX_ASXX_TX_HI_WATERX(port, interface), 12); }
    else if cpu_clock_hz >= 375000000 && cpu_clock_hz < 437000000 { cvmx_write_csr(CVMX_ASXX_TX_HI_WATERX(port, interface), 11); }
    else if cpu_clock_hz >= 437000000 && cpu_clock_hz < 550000000 { cvmx_write_csr(CVMX_ASXX_TX_HI_WATERX(port, interface), 10); }
    else if cpu_clock_hz >= 550000000 && cpu_clock_hz < 687000000 { cvmx_write_csr(CVMX_ASXX_TX_HI_WATERX(port, interface), 9); }
    else { cvmx_dprintf!("Illegal clock frequency ({}). CVMX_ASXX_TX_HI_WATERX not set\n", cpu_clock_hz); }
    0
}

pub unsafe fn __cvmx_helper_rgmii_enable(interface: i32) -> i32 {
    let num_ports = cvmx_helper_ports_on_interface(interface);
    let sys_info_ptr = cvmx_sysinfo_get();
    let mut mode: cvmx_gmxx_inf_mode = core::mem::zeroed();
    let mut asx_tx: cvmx_asxx_tx_prt_en = core::mem::zeroed();
    let mut asx_rx: cvmx_asxx_rx_prt_en = core::mem::zeroed();
    mode.u64 = cvmx_read_csr(CVMX_GMXX_INF_MODE(interface));
    if mode.s.en == 0 { return -1; }
    if (OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX)) && mode.s.type_ == 1 { return -1; }
    asx_tx.u64 = 0; asx_tx.s.prt_en = cvmx_build_mask(num_ports); cvmx_write_csr(CVMX_ASXX_TX_PRT_EN(interface), asx_tx.u64);
    asx_rx.u64 = 0; asx_rx.s.prt_en = cvmx_build_mask(num_ports); cvmx_write_csr(CVMX_ASXX_RX_PRT_EN(interface), asx_rx.u64);
    for port in 0..num_ports {
        if cvmx_octeon_is_pass1() { __cvmx_helper_errata_asx_pass1(interface, port, (*sys_info_ptr).cpu_clock_hz); }
        else { let mut frm_ctl: cvmx_gmxx_rxx_frm_ctl = core::mem::zeroed(); frm_ctl.u64 = cvmx_read_csr(CVMX_GMXX_RXX_FRM_CTL(port, interface)); frm_ctl.s.pre_free = 1; cvmx_write_csr(CVMX_GMXX_RXX_FRM_CTL(port, interface), frm_ctl.u64); }
        cvmx_write_csr(CVMX_GMXX_TXX_PAUSE_PKT_TIME(port, interface), 20000);
        cvmx_write_csr(CVMX_GMXX_TXX_PAUSE_PKT_INTERVAL(port, interface), 19000);
        let clk = if OCTEON_IS_MODEL(OCTEON_CN50XX) { 16 } else { 24 };
        cvmx_write_csr(CVMX_ASXX_TX_CLK_SETX(port, interface), clk); cvmx_write_csr(CVMX_ASXX_RX_CLK_SETX(port, interface), clk);
    }
    __cvmx_helper_setup_gmx(interface, num_ports);
    for port in 0..num_ports { let mut cfg: cvmx_gmxx_prtx_cfg = core::mem::zeroed(); cfg.u64 = cvmx_read_csr(CVMX_GMXX_PRTX_CFG(port, interface)); cfg.s.en = 1; cvmx_write_csr(CVMX_GMXX_PRTX_CFG(port, interface), cfg.u64); }
    __cvmx_interrupt_asxx_enable(interface); __cvmx_interrupt_gmxx_enable(interface); 0
}

pub unsafe fn __cvmx_helper_rgmii_link_get(ipd_port: i32) -> cvmx_helper_link_info {
    let interface = cvmx_helper_get_interface_num(ipd_port); let index = cvmx_helper_get_interface_index_num(ipd_port);
    let mut loop_cfg: cvmx_asxx_prt_loop = core::mem::zeroed(); loop_cfg.u64 = cvmx_read_csr(CVMX_ASXX_PRT_LOOP(interface));
    if loop_cfg.s.int_loop & (1 << index) != 0 { let mut result: cvmx_helper_link_info = core::mem::zeroed(); result.u64 = 0; result.s.full_duplex = 1; result.s.link_up = 1; result.s.speed = 1000; result } else { __cvmx_helper_board_link_get(ipd_port) }
}

pub unsafe fn __cvmx_helper_rgmii_link_set(ipd_port: i32, link_info: cvmx_helper_link_info) -> i32 {
    let interface = cvmx_helper_get_interface_num(ipd_port); let index = cvmx_helper_get_interface_index_num(ipd_port);
    if (*cvmx_sysinfo_get()).board_type == CVMX_BOARD_TYPE_SIM { return 0; }
    let mut original: cvmx_gmxx_prtx_cfg = core::mem::zeroed(); original.u64 = cvmx_read_csr(CVMX_GMXX_PRTX_CFG(index, interface)); let mut new_cfg = original;
    cvmx_write_csr(CVMX_ASXX_RX_PRT_EN(interface), cvmx_read_csr(CVMX_ASXX_RX_PRT_EN(interface)) & !(1u64 << index));
    let mut qos_save: [cvmx_pko_mem_queue_qos; 16] = core::mem::zeroed();
    for i in 0..cvmx_pko_get_num_queues(ipd_port) { let queue = cvmx_pko_get_base_queue(ipd_port) + i; cvmx_write_csr(CVMX_PKO_REG_READ_IDX, queue); let mut qos: cvmx_pko_mem_queue_qos = core::mem::zeroed(); qos.u64 = cvmx_read_csr(CVMX_PKO_MEM_QUEUE_QOS); qos.s.pid = ipd_port; qos.s.qid = queue; qos_save[i as usize] = qos; qos.s.qos_mask = 0; cvmx_write_csr(CVMX_PKO_MEM_QUEUE_QOS, qos.u64); }
    let mut bp: cvmx_gmxx_tx_ovr_bp = core::mem::zeroed(); bp.u64 = cvmx_read_csr(CVMX_GMXX_TX_OVR_BP(interface)); let bp_save = bp; bp.s.bp &= !(1 << index); bp.s.en |= 1 << index; cvmx_write_csr(CVMX_GMXX_TX_OVR_BP(interface), bp.u64); cvmx_read_csr(CVMX_GMXX_TX_OVR_BP(interface));
    cvmx_write_csr(CVMX_NPI_DBG_SELECT, interface * 0x800 + index * 0x100 + 0x880);
    CVMX_WAIT_FOR_FIELD64!(CVMX_DBG_DATA, cvmx_dbg_data, data & 7, ==, 0, 10000); CVMX_WAIT_FOR_FIELD64!(CVMX_DBG_DATA, cvmx_dbg_data, data & 0xf, ==, 0, 10000);
    new_cfg.s.en = 0; cvmx_write_csr(CVMX_GMXX_PRTX_CFG(index, interface), new_cfg.u64); cvmx_read_csr(CVMX_GMXX_PRTX_CFG(index, interface));
    if cvmx_octeon_is_pass1() || !link_info.s.link_up { new_cfg.s.duplex = 1; } else { new_cfg.s.duplex = link_info.s.full_duplex; }
    if link_info.s.speed == 10 || link_info.s.speed == 100 { new_cfg.s.slottime = 0; new_cfg.s.speed = 0; } else { new_cfg.s.slottime = 1; new_cfg.s.speed = 1; }
    if link_info.s.speed == 10 { cvmx_write_csr(CVMX_GMXX_TXX_CLK(index, interface), 50); cvmx_write_csr(CVMX_GMXX_TXX_SLOT(index, interface), 0x40); cvmx_write_csr(CVMX_GMXX_TXX_BURST(index, interface), 0); } else if link_info.s.speed == 100 { cvmx_write_csr(CVMX_GMXX_TXX_CLK(index, interface), 5); cvmx_write_csr(CVMX_GMXX_TXX_SLOT(index, interface), 0x40); cvmx_write_csr(CVMX_GMXX_TXX_BURST(index, interface), 0); } else { cvmx_write_csr(CVMX_GMXX_TXX_CLK(index, interface), 1); cvmx_write_csr(CVMX_GMXX_TXX_SLOT(index, interface), 0x200); cvmx_write_csr(CVMX_GMXX_TXX_BURST(index, interface), 0x2000); }
    if (OCTEON_IS_MODEL(OCTEON_CN30XX) || OCTEON_IS_MODEL(OCTEON_CN50XX)) && (link_info.s.speed == 10 || link_info.s.speed == 100) { let mut mode: cvmx_gmxx_inf_mode = core::mem::zeroed(); mode.u64 = cvmx_read_csr(CVMX_GMXX_INF_MODE(interface)); if (index == 0 && mode.s.p0mii == 1) || (index != 0 && mode.s.type_ == 1) { cvmx_write_csr(CVMX_GMXX_TXX_CLK(index, interface), 1); } }
    cvmx_read_csr(CVMX_GMXX_PRTX_CFG(index, interface)); cvmx_write_csr(CVMX_GMXX_PRTX_CFG(index, interface), new_cfg.u64);
    cvmx_write_csr(CVMX_ASXX_RX_PRT_EN(interface), cvmx_read_csr(CVMX_ASXX_RX_PRT_EN(interface)) | (1u64 << index));
    for i in 0..cvmx_pko_get_num_queues(ipd_port) { let queue = cvmx_pko_get_base_queue(ipd_port) + i; cvmx_write_csr(CVMX_PKO_REG_READ_IDX, queue); cvmx_write_csr(CVMX_PKO_MEM_QUEUE_QOS, qos_save[i as usize].u64); }
    cvmx_write_csr(CVMX_GMXX_TX_OVR_BP(interface), bp_save.u64); new_cfg.s.en = original.s.en; cvmx_write_csr(CVMX_GMXX_PRTX_CFG(index, interface), new_cfg.u64); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
