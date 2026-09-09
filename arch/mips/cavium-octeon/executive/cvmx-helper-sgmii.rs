/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (C) 2003-2018 Cavium, Inc.
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, version 2, as
 * published by the Free Software Foundation.
 ***********************license end**************************************/

/* Functions for SGMII initialization, configuration, and monitoring. */

unsafe fn __cvmx_helper_sgmii_hardware_init_one_time(interface: i32, index: i32) -> i32 {
    let clock_mhz = (*cvmx_sysinfo_get()).cpu_clock_hz / 1_000_000;
    let mut pcs_misc_ctl_reg: cvmx_pcsx_miscx_ctl_reg;
    let mut pcsx_linkx_timer_count_reg: cvmx_pcsx_linkx_timer_count_reg;
    let mut gmxx_prtx_cfg: cvmx_gmxx_prtx_cfg;

    gmxx_prtx_cfg.u64 = cvmx_read_csr(CVMX_GMXX_PRTX_CFG(index, interface));
    gmxx_prtx_cfg.s.en = 0;
    cvmx_write_csr(CVMX_GMXX_PRTX_CFG(index, interface), gmxx_prtx_cfg.u64);

    pcs_misc_ctl_reg.u64 = cvmx_read_csr(CVMX_PCSX_MISCX_CTL_REG(index, interface));
    pcsx_linkx_timer_count_reg.u64 = cvmx_read_csr(CVMX_PCSX_LINKX_TIMER_COUNT_REG(index, interface));
    if pcs_misc_ctl_reg.s.mode != 0 {
        pcsx_linkx_timer_count_reg.s.count = (10000u64 * clock_mhz) >> 10;
    } else {
        pcsx_linkx_timer_count_reg.s.count = (1600u64 * clock_mhz) >> 10;
    }
    cvmx_write_csr(CVMX_PCSX_LINKX_TIMER_COUNT_REG(index, interface), pcsx_linkx_timer_count_reg.u64);

    if pcs_misc_ctl_reg.s.mode != 0 {
        let mut r: cvmx_pcsx_anx_adv_reg;
        r.u64 = cvmx_read_csr(CVMX_PCSX_ANX_ADV_REG(index, interface));
        r.s.rem_flt = 0; r.s.pause = 3; r.s.hfd = 1; r.s.fd = 1;
        cvmx_write_csr(CVMX_PCSX_ANX_ADV_REG(index, interface), r.u64);
    } else {
        let mut r: cvmx_pcsx_miscx_ctl_reg;
        r.u64 = cvmx_read_csr(CVMX_PCSX_MISCX_CTL_REG(index, interface));
        if r.s.mac_phy != 0 {
            let mut a: cvmx_pcsx_sgmx_an_adv_reg;
            a.u64 = cvmx_read_csr(CVMX_PCSX_SGMX_AN_ADV_REG(index, interface));
            a.s.link = 1; a.s.dup = 1; a.s.speed = 2;
            cvmx_write_csr(CVMX_PCSX_SGMX_AN_ADV_REG(index, interface), a.u64);
        }
    }
    0
}

unsafe fn __cvmx_helper_sgmii_hardware_init_link(interface: i32, index: i32) -> i32 {
    let mut control_reg: cvmx_pcsx_mrx_control_reg;
    control_reg.u64 = cvmx_read_csr(CVMX_PCSX_MRX_CONTROL_REG(index, interface));
    if (*cvmx_sysinfo_get()).board_type != CVMX_BOARD_TYPE_SIM {
        control_reg.s.reset = 1;
        cvmx_write_csr(CVMX_PCSX_MRX_CONTROL_REG(index, interface), control_reg.u64);
        if CVMX_WAIT_FOR_FIELD64!(CVMX_PCSX_MRX_CONTROL_REG(index, interface), cvmx_pcsx_mrx_control_reg, reset, ==, 0, 10000) {
            cvmx_dprintf!("SGMII%d: Timeout waiting for port %d to finish reset\n", interface, index);
            return -1;
        }
    }
    control_reg.s.rst_an = 1; control_reg.s.an_en = 1; control_reg.s.pwr_dn = 0;
    cvmx_write_csr(CVMX_PCSX_MRX_CONTROL_REG(index, interface), control_reg.u64);
    if (*cvmx_sysinfo_get()).board_type != CVMX_BOARD_TYPE_SIM &&
       CVMX_WAIT_FOR_FIELD64!(CVMX_PCSX_MRX_STATUS_REG(index, interface), cvmx_pcsx_mrx_status_reg, an_cpt, ==, 1, 10000) { return -1; }
    0
}

unsafe fn __cvmx_helper_sgmii_hardware_init_link_speed(interface: i32, index: i32, link_info: cvmx_helper_link_info) -> i32 {
    let mut gmxx_prtx_cfg: cvmx_gmxx_prtx_cfg;
    let mut pcsx_miscx_ctl_reg: cvmx_pcsx_miscx_ctl_reg;
    gmxx_prtx_cfg.u64 = cvmx_read_csr(CVMX_GMXX_PRTX_CFG(index, interface));
    let is_enabled = gmxx_prtx_cfg.s.en;
    gmxx_prtx_cfg.s.en = 0;
    cvmx_write_csr(CVMX_GMXX_PRTX_CFG(index, interface), gmxx_prtx_cfg.u64);
    if CVMX_WAIT_FOR_FIELD64!(CVMX_GMXX_PRTX_CFG(index, interface), cvmx_gmxx_prtx_cfg, rx_idle, ==, 1, 10000) || CVMX_WAIT_FOR_FIELD64!(CVMX_GMXX_PRTX_CFG(index, interface), cvmx_gmxx_prtx_cfg, tx_idle, ==, 1, 10000) { cvmx_dprintf!("SGMII%d: Timeout waiting for port %d to be idle\n", interface, index); return -1; }
    gmxx_prtx_cfg.u64 = cvmx_read_csr(CVMX_GMXX_PRTX_CFG(index, interface));
    pcsx_miscx_ctl_reg.u64 = cvmx_read_csr(CVMX_PCSX_MISCX_CTL_REG(index, interface));
    pcsx_miscx_ctl_reg.s.gmxeno = if link_info.s.link_up != 0 { 0 } else { 1 };
    if link_info.s.link_up != 0 { gmxx_prtx_cfg.s.duplex = link_info.s.full_duplex; }
    match link_info.s.speed {
        10 => { gmxx_prtx_cfg.s.speed = 0; gmxx_prtx_cfg.s.speed_msb = 1; gmxx_prtx_cfg.s.slottime = 0; pcsx_miscx_ctl_reg.s.samp_pt = 25; cvmx_write_csr(CVMX_GMXX_TXX_SLOT(index, interface), 64); cvmx_write_csr(CVMX_GMXX_TXX_BURST(index, interface), 0); }
        100 => { gmxx_prtx_cfg.s.speed = 0; gmxx_prtx_cfg.s.speed_msb = 0; gmxx_prtx_cfg.s.slottime = 0; pcsx_miscx_ctl_reg.s.samp_pt = 0x5; cvmx_write_csr(CVMX_GMXX_TXX_SLOT(index, interface), 64); cvmx_write_csr(CVMX_GMXX_TXX_BURST(index, interface), 0); }
        1000 => { gmxx_prtx_cfg.s.speed = 1; gmxx_prtx_cfg.s.speed_msb = 0; gmxx_prtx_cfg.s.slottime = 1; pcsx_miscx_ctl_reg.s.samp_pt = 1; cvmx_write_csr(CVMX_GMXX_TXX_SLOT(index, interface), 512); cvmx_write_csr(CVMX_GMXX_TXX_BURST(index, interface), 8192); }
        _ => {}
    }
    cvmx_write_csr(CVMX_PCSX_MISCX_CTL_REG(index, interface), pcsx_miscx_ctl_reg.u64);
    cvmx_write_csr(CVMX_GMXX_PRTX_CFG(index, interface), gmxx_prtx_cfg.u64);
    gmxx_prtx_cfg.u64 = cvmx_read_csr(CVMX_GMXX_PRTX_CFG(index, interface));
    gmxx_prtx_cfg.s.en = is_enabled;
    cvmx_write_csr(CVMX_GMXX_PRTX_CFG(index, interface), gmxx_prtx_cfg.u64);
    0
}

unsafe fn __cvmx_helper_sgmii_hardware_init(interface: i32, num_ports: i32) -> i32 {
    __cvmx_helper_setup_gmx(interface, num_ports);
    for index in 0..num_ports { let ipd_port = cvmx_helper_get_ipd_port(interface, index); __cvmx_helper_sgmii_hardware_init_one_time(interface, index); if (*cvmx_sysinfo_get()).board_type == CVMX_BOARD_TYPE_SIM { __cvmx_helper_sgmii_link_set(ipd_port, __cvmx_helper_sgmii_link_get(ipd_port)); } }
    0
}

pub unsafe fn __cvmx_helper_sgmii_enumerate(_interface: i32) -> i32 { 4 }

pub unsafe fn __cvmx_helper_sgmii_probe(interface: i32) -> i32 {
    let mut mode: cvmx_gmxx_inf_mode; mode.u64 = cvmx_read_csr(CVMX_GMXX_INF_MODE(interface)); mode.s.en = 1; cvmx_write_csr(CVMX_GMXX_INF_MODE(interface), mode.u64); __cvmx_helper_sgmii_enumerate(interface)
}

pub unsafe fn __cvmx_helper_sgmii_enable(interface: i32) -> i32 {
    let num_ports = cvmx_helper_ports_on_interface(interface);
    __cvmx_helper_sgmii_hardware_init(interface, num_ports);
    for index in 0..num_ports { let mut r: cvmx_gmxx_prtx_cfg; r.u64 = cvmx_read_csr(CVMX_GMXX_PRTX_CFG(index, interface)); r.s.en = 1; cvmx_write_csr(CVMX_GMXX_PRTX_CFG(index, interface), r.u64); __cvmx_interrupt_pcsx_intx_en_reg_enable(index, interface); }
    __cvmx_interrupt_pcsxx_int_en_reg_enable(interface); __cvmx_interrupt_gmxx_enable(interface); 0
}

pub unsafe fn __cvmx_helper_sgmii_link_get(ipd_port: i32) -> cvmx_helper_link_info {
    let mut result: cvmx_helper_link_info; result.u64 = 0;
    let interface = cvmx_helper_get_interface_num(ipd_port); let index = cvmx_helper_get_interface_index_num(ipd_port);
    if (*cvmx_sysinfo_get()).board_type == CVMX_BOARD_TYPE_SIM { result.s.link_up = 1; result.s.full_duplex = 1; result.s.speed = 1000; return result; }
    let mut control: cvmx_pcsx_mrx_control_reg; control.u64 = cvmx_read_csr(CVMX_PCSX_MRX_CONTROL_REG(index, interface));
    if control.s.loopbck1 != 0 { result.s.link_up = 1; result.s.full_duplex = 1; result.s.speed = 1000; return result; }
    let mut misc: cvmx_pcsx_miscx_ctl_reg; misc.u64 = cvmx_read_csr(CVMX_PCSX_MISCX_CTL_REG(index, interface));
    if misc.s.mode == 0 { let mut m: cvmx_pcsx_miscx_ctl_reg; m.u64 = cvmx_read_csr(CVMX_PCSX_MISCX_CTL_REG(index, interface)); if m.s.mac_phy != 0 { let mut status: cvmx_pcsx_mrx_status_reg; status.u64 = cvmx_read_csr(CVMX_PCSX_MRX_STATUS_REG(index, interface)); if status.s.lnk_st == 0 && __cvmx_helper_sgmii_hardware_init_link(interface, index) != 0 { return result; } let mut a: cvmx_pcsx_anx_results_reg; a.u64 = cvmx_read_csr(CVMX_PCSX_ANX_RESULTS_REG(index, interface)); if a.s.an_cpt != 0 { result.s.full_duplex = a.s.dup; result.s.link_up = a.s.link_ok; result.s.speed = match a.s.spd { 0 => 10, 1 => 100, 2 => 1000, _ => { result.s.link_up = 0; 0 } }; } } else { result = __cvmx_helper_board_link_get(ipd_port); } }
    result
}

pub unsafe fn __cvmx_helper_sgmii_link_set(ipd_port: i32, link_info: cvmx_helper_link_info) -> i32 {
    let interface = cvmx_helper_get_interface_num(ipd_port); let index = cvmx_helper_get_interface_index_num(ipd_port);
    __cvmx_helper_sgmii_hardware_init_link(interface, index); __cvmx_helper_sgmii_hardware_init_link_speed(interface, index, link_info)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
