/*
 * Support library for the SPI.
 *
 * This is a source-level Rust translation of cvmx-spi.c. The declarations
 * supplied by the original OCTEON headers remain external dependencies.
 */

#[cfg(CVMX_ENABLE_DEBUG_PRINTS)]
static MODES: [&str; 4] = ["UNKNOWN", "TX Halfplex", "Rx Halfplex", "Duplex"];

static mut CVMX_SPI_CALLBACKS: cvmx_spi_callbacks_t = cvmx_spi_callbacks_t {
    reset_cb: Some(cvmx_spi_reset_cb),
    calendar_setup_cb: Some(cvmx_spi_calendar_setup_cb),
    clock_detect_cb: Some(cvmx_spi_clock_detect_cb),
    training_cb: Some(cvmx_spi_training_cb),
    calendar_sync_cb: Some(cvmx_spi_calendar_sync_cb),
    interface_up_cb: Some(cvmx_spi_interface_up_cb),
};

pub unsafe fn cvmx_spi_get_callbacks(callbacks: *mut cvmx_spi_callbacks_t) {
    core::ptr::copy_nonoverlapping(
        &CVMX_SPI_CALLBACKS as *const cvmx_spi_callbacks_t,
        callbacks,
        1,
    );
}

pub unsafe fn cvmx_spi_set_callbacks(new_callbacks: *const cvmx_spi_callbacks_t) {
    core::ptr::copy_nonoverlapping(new_callbacks, &mut CVMX_SPI_CALLBACKS, 1);
}

pub unsafe fn cvmx_spi_start_interface(
    interface: i32,
    mode: cvmx_spi_mode,
    timeout: i32,
    num_ports: i32,
) -> i32 {
    let mut res: i32 = -1;
    if !(OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX)) { return res; }
    if let Some(cb) = CVMX_SPI_CALLBACKS.reset_cb { res = cb(interface, mode); if res != 0 { return res; } }
    if let Some(cb) = CVMX_SPI_CALLBACKS.calendar_setup_cb { res = cb(interface, mode, num_ports); if res != 0 { return res; } }
    if let Some(cb) = CVMX_SPI_CALLBACKS.clock_detect_cb { res = cb(interface, mode, timeout); if res != 0 { return res; } }
    if let Some(cb) = CVMX_SPI_CALLBACKS.training_cb { res = cb(interface, mode, timeout); if res != 0 { return res; } }
    if let Some(cb) = CVMX_SPI_CALLBACKS.calendar_sync_cb { res = cb(interface, mode, timeout); if res != 0 { return res; } }
    if let Some(cb) = CVMX_SPI_CALLBACKS.interface_up_cb { res = cb(interface, mode); if res != 0 { return res; } }
    res
}

pub unsafe fn cvmx_spi_restart_interface(interface: i32, mode: cvmx_spi_mode, timeout: i32) -> i32 {
    let mut res: i32 = -1;
    if !(OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX)) { return res; }
    #[cfg(CVMX_ENABLE_DEBUG_PRINTS)]
    cvmx_dprintf!("SPI{}: Restart {}\n", interface, MODES[mode as usize]);
    if let Some(cb) = CVMX_SPI_CALLBACKS.reset_cb { res = cb(interface, mode); if res != 0 { return res; } }
    if let Some(cb) = CVMX_SPI_CALLBACKS.clock_detect_cb { res = cb(interface, mode, timeout); if res != 0 { return res; } }
    if let Some(cb) = CVMX_SPI_CALLBACKS.training_cb { res = cb(interface, mode, timeout); if res != 0 { return res; } }
    if let Some(cb) = CVMX_SPI_CALLBACKS.calendar_sync_cb { res = cb(interface, mode, timeout); if res != 0 { return res; } }
    if let Some(cb) = CVMX_SPI_CALLBACKS.interface_up_cb { res = cb(interface, mode); if res != 0 { return res; } }
    res
}

pub unsafe fn cvmx_spi_reset_cb(interface: i32, _mode: cvmx_spi_mode) -> i32 {
    let mut spxx_clk_ctl: cvmx_spxx_clk_ctl = core::mem::zeroed();
    let mut spxx_bist_stat: cvmx_spxx_bist_stat = core::mem::zeroed();
    let mut spxx_int_msk: cvmx_spxx_int_msk = core::mem::zeroed();
    let mut stxx_int_msk: cvmx_stxx_int_msk = core::mem::zeroed();
    let mut spxx_trn4_ctl: cvmx_spxx_trn4_ctl = core::mem::zeroed();
    let mut spxx_dbg_deskew_ctl: cvmx_spxx_dbg_deskew_ctl = core::mem::zeroed();
    let ms = (*cvmx_sysinfo_get()).cpu_clock_hz / 1000;
    spxx_int_msk.u64 = cvmx_read_csr(CVMX_SPXX_INT_MSK(interface)); cvmx_write_csr(CVMX_SPXX_INT_MSK(interface), 0);
    stxx_int_msk.u64 = cvmx_read_csr(CVMX_STXX_INT_MSK(interface)); cvmx_write_csr(CVMX_STXX_INT_MSK(interface), 0);
    cvmx_write_csr(CVMX_SRXX_COM_CTL(interface), 0); cvmx_write_csr(CVMX_STXX_COM_CTL(interface), 0);
    spxx_clk_ctl.u64 = 0; spxx_clk_ctl.s.runbist = 1; cvmx_write_csr(CVMX_SPXX_CLK_CTL(interface), spxx_clk_ctl.u64); __delay(10 * ms);
    spxx_bist_stat.u64 = cvmx_read_csr(CVMX_SPXX_BIST_STAT(interface));
    if spxx_bist_stat.s.stat0 { cvmx_dprintf!("ERROR SPI{}: BIST failed on receive datapath FIFO\n", interface); }
    if spxx_bist_stat.s.stat1 { cvmx_dprintf!("ERROR SPI{}: BIST failed on RX calendar table\n", interface); }
    if spxx_bist_stat.s.stat2 { cvmx_dprintf!("ERROR SPI{}: BIST failed on TX calendar table\n", interface); }
    for index in 0..32 {
        let mut r: cvmx_srxx_spi4_calx = core::mem::zeroed(); r.u64 = 0; r.s.oddpar = 1; cvmx_write_csr(CVMX_SRXX_SPI4_CALX(index, interface), r.u64);
        let mut t: cvmx_stxx_spi4_calx = core::mem::zeroed(); t.u64 = 0; t.s.oddpar = 1; cvmx_write_csr(CVMX_STXX_SPI4_CALX(index, interface), t.u64);
    }
    cvmx_write_csr(CVMX_SPXX_INT_REG(interface), cvmx_read_csr(CVMX_SPXX_INT_REG(interface))); cvmx_write_csr(CVMX_SPXX_INT_MSK(interface), spxx_int_msk.u64);
    cvmx_write_csr(CVMX_STXX_INT_REG(interface), cvmx_read_csr(CVMX_STXX_INT_REG(interface))); cvmx_write_csr(CVMX_STXX_INT_MSK(interface), stxx_int_msk.u64);
    spxx_clk_ctl.u64 = 0; spxx_clk_ctl.s.clkdly = 0x10; spxx_clk_ctl.s.statrcv = 1; cvmx_write_csr(CVMX_SPXX_CLK_CTL(interface), spxx_clk_ctl.u64); __delay(100 * ms);
    spxx_clk_ctl.s.srxdlck = 1; cvmx_write_csr(CVMX_SPXX_CLK_CTL(interface), spxx_clk_ctl.u64); __delay(100 * ms);
    spxx_trn4_ctl.u64 = 0; spxx_trn4_ctl.s.jitter = 1; spxx_trn4_ctl.s.clr_boot = 1; spxx_trn4_ctl.s.maxdist = if OCTEON_IS_MODEL(OCTEON_CN58XX) { 3 } else { 8 }; spxx_trn4_ctl.s.macro_en = 1; spxx_trn4_ctl.s.mux_en = 1;
    cvmx_write_csr(CVMX_SPXX_TRN4_CTL(interface), spxx_trn4_ctl.u64); spxx_dbg_deskew_ctl.u64 = 0; cvmx_write_csr(CVMX_SPXX_DBG_DESKEW_CTL(interface), spxx_dbg_deskew_ctl.u64); 0
}

pub unsafe fn cvmx_spi_calendar_setup_cb(interface: i32, mode: cvmx_spi_mode, num_ports: i32) -> i32 {
    if mode & CVMX_SPI_MODE_RX_HALFPLEX != 0 {
        let mut c: cvmx_srxx_com_ctl = core::mem::zeroed(); c.u64 = 0; c.s.prts = num_ports - 1; cvmx_write_csr(CVMX_SRXX_COM_CTL(interface), c.u64);
        let mut port = 0; let mut index = 0; while port < num_ports { let mut x: cvmx_srxx_spi4_calx = core::mem::zeroed(); x.u64 = 0; x.s.prt0 = port; port += 1; x.s.prt1 = port; port += 1; x.s.prt2 = port; port += 1; x.s.prt3 = port; port += 1; x.s.oddpar = !(cvmx_dpop(x.u64) & 1); cvmx_write_csr(CVMX_SRXX_SPI4_CALX(index, interface), x.u64); index += 1; }
        let mut s: cvmx_srxx_spi4_stat = core::mem::zeroed(); s.u64 = 0; s.s.len = num_ports; s.s.m = 1; cvmx_write_csr(CVMX_SRXX_SPI4_STAT(interface), s.u64);
    }
    if mode & CVMX_SPI_MODE_TX_HALFPLEX != 0 {
        let mut a: cvmx_stxx_arb_ctl = core::mem::zeroed(); a.u64 = 0; cvmx_write_csr(CVMX_STXX_ARB_CTL(interface), a.u64);
        let mut m: cvmx_gmxx_tx_spi_max = core::mem::zeroed(); m.u64 = 0; m.s.max1 = 8; m.s.max2 = 4; cvmx_write_csr(CVMX_GMXX_TX_SPI_MAX(interface), m.u64);
        let mut t: cvmx_gmxx_tx_spi_thresh = core::mem::zeroed(); t.u64 = 0; t.s.thresh = 4; cvmx_write_csr(CVMX_GMXX_TX_SPI_THRESH(interface), t.u64);
        cvmx_write_csr(CVMX_GMXX_TX_SPI_CTL(interface), 0);
        let mut d: cvmx_stxx_spi4_dat = core::mem::zeroed(); d.u64 = 0; d.s.alpha = 32; d.s.max_t = 0xFFFF; cvmx_write_csr(CVMX_STXX_SPI4_DAT(interface), d.u64);
        let mut port = 0; let mut index = 0; while port < num_ports { let mut x: cvmx_stxx_spi4_calx = core::mem::zeroed(); x.u64 = 0; x.s.prt0 = port; port += 1; x.s.prt1 = port; port += 1; x.s.prt2 = port; port += 1; x.s.prt3 = port; port += 1; x.s.oddpar = !(cvmx_dpop(x.u64) & 1); cvmx_write_csr(CVMX_STXX_SPI4_CALX(index, interface), x.u64); index += 1; }
        let mut s: cvmx_stxx_spi4_stat = core::mem::zeroed(); s.u64 = 0; s.s.len = num_ports; s.s.m = 1; cvmx_write_csr(CVMX_STXX_SPI4_STAT(interface), s.u64);
    } 0
}

pub unsafe fn cvmx_spi_clock_detect_cb(interface: i32, _mode: cvmx_spi_mode, timeout: i32) -> i32 {
    let ms = (*cvmx_sysinfo_get()).cpu_clock_hz / 1000; let mut stat: cvmx_spxx_clk_stat = core::mem::zeroed(); let mut n = 100; let end = cvmx_get_cycle() + 1000u64 * ms * timeout as u64;
    cvmx_dprintf!("SPI{}: Waiting to see TsClk...\n", interface); loop { stat.u64 = cvmx_read_csr(CVMX_SPXX_CLK_STAT(interface)); if stat.s.s4clk0 && stat.s.s4clk1 && n != 0 { n -= 1; cvmx_write_csr(CVMX_SPXX_CLK_STAT(interface), stat.u64); stat.s.s4clk0 = 0; stat.s.s4clk1 = 0; } if cvmx_get_cycle() > end { cvmx_dprintf!("SPI{}: Timeout\n", interface); return -1; } if stat.s.s4clk0 != 0 && stat.s.s4clk1 != 0 { break; } }
    cvmx_dprintf!("SPI{}: Waiting to see RsClk...\n", interface); let end = cvmx_get_cycle() + 1000u64 * ms * timeout as u64; n = 100; loop { stat.u64 = cvmx_read_csr(CVMX_SPXX_CLK_STAT(interface)); if stat.s.d4clk0 && stat.s.d4clk1 && n != 0 { n -= 1; cvmx_write_csr(CVMX_SPXX_CLK_STAT(interface), stat.u64); stat.s.d4clk0 = 0; stat.s.d4clk1 = 0; } if cvmx_get_cycle() > end { cvmx_dprintf!("SPI{}: Timeout\n", interface); return -1; } if stat.s.d4clk0 != 0 && stat.s.d4clk1 != 0 { break; } } 0
}

pub unsafe fn cvmx_spi_training_cb(interface: i32, _mode: cvmx_spi_mode, _timeout: i32) -> i32 {
    let ms = (*cvmx_sysinfo_get()).cpu_clock_hz / 1000; let mut c: cvmx_spxx_clk_ctl = core::mem::zeroed(); c.u64 = 0; c.s.clkdly = 0x10; c.s.statrcv = 1; c.s.sndtrn = 1; c.s.drptrn = 1; c.s.rcvtrn = 1; c.s.srxdlck = 1; cvmx_write_csr(CVMX_SPXX_CLK_CTL(interface), c.u64); __delay(1000 * ms);
    let mut trn: cvmx_spxx_trn4_ctl = core::mem::zeroed(); trn.u64 = cvmx_read_csr(CVMX_SPXX_TRN4_CTL(interface)); trn.s.clr_boot = 1; cvmx_write_csr(CVMX_SPXX_TRN4_CTL(interface), trn.u64); cvmx_dprintf!("SPI{}: Waiting for training\n", interface); __delay(1000 * ms);
    let end = cvmx_get_cycle() + 1000u64 * ms * 600; let mut n = 500; let mut stat: cvmx_spxx_clk_stat = core::mem::zeroed(); loop { stat.u64 = cvmx_read_csr(CVMX_SPXX_CLK_STAT(interface)); if stat.s.srxtrn && n != 0 { n -= 1; cvmx_write_csr(CVMX_SPXX_CLK_STAT(interface), stat.u64); stat.s.srxtrn = 0; } if cvmx_get_cycle() > end { cvmx_dprintf!("SPI{}: Timeout\n", interface); return -1; } if stat.s.srxtrn != 0 { break; } } 0
}

pub unsafe fn cvmx_spi_calendar_sync_cb(interface: i32, mode: cvmx_spi_mode, timeout: i32) -> i32 {
    let ms = (*cvmx_sysinfo_get()).cpu_clock_hz / 1000;
    if mode & CVMX_SPI_MODE_RX_HALFPLEX != 0 { let mut c: cvmx_srxx_com_ctl = core::mem::zeroed(); cvmx_dprintf!("SPI{}: Rx is synchronized, start sending calendar data\n", interface); c.u64 = cvmx_read_csr(CVMX_SRXX_COM_CTL(interface)); c.s.inf_en = 1; c.s.st_en = 1; cvmx_write_csr(CVMX_SRXX_COM_CTL(interface), c.u64); }
    if mode & CVMX_SPI_MODE_TX_HALFPLEX != 0 { let mut c: cvmx_stxx_com_ctl = core::mem::zeroed(); c.u64 = 0; c.s.st_en = 1; cvmx_write_csr(CVMX_STXX_COM_CTL(interface), c.u64); cvmx_dprintf!("SPI{}: Waiting to sync on STX[{}] STAT\n", interface, interface); let end = cvmx_get_cycle() + 1000u64 * ms * timeout as u64; let mut s: cvmx_spxx_clk_stat = core::mem::zeroed(); loop { s.u64 = cvmx_read_csr(CVMX_SPXX_CLK_STAT(interface)); if cvmx_get_cycle() > end { cvmx_dprintf!("SPI{}: Timeout\n", interface); return -1; } if s.s.stxcal != 0 { break; } } } 0
}

pub unsafe fn cvmx_spi_interface_up_cb(interface: i32, mode: cvmx_spi_mode) -> i32 {
    if mode & CVMX_SPI_MODE_RX_HALFPLEX != 0 { let mut c: cvmx_srxx_com_ctl = core::mem::zeroed(); c.u64 = cvmx_read_csr(CVMX_SRXX_COM_CTL(interface)); c.s.inf_en = 1; cvmx_write_csr(CVMX_SRXX_COM_CTL(interface), c.u64); cvmx_dprintf!("SPI{}: Rx is now up\n", interface); }
    if mode & CVMX_SPI_MODE_TX_HALFPLEX != 0 { let mut c: cvmx_stxx_com_ctl = core::mem::zeroed(); c.u64 = cvmx_read_csr(CVMX_STXX_COM_CTL(interface)); c.s.inf_en = 1; cvmx_write_csr(CVMX_STXX_COM_CTL(interface), c.u64); cvmx_dprintf!("SPI{}: Tx is now up\n", interface); }
    let mut min: cvmx_gmxx_rxx_frm_min = core::mem::zeroed(); min.u64 = 0; min.s.len = 64; cvmx_write_csr(CVMX_GMXX_RXX_FRM_MIN(0, interface), min.u64);
    let mut max: cvmx_gmxx_rxx_frm_max = core::mem::zeroed(); max.u64 = 0; max.s.len = 64 * 1024 - 4; cvmx_write_csr(CVMX_GMXX_RXX_FRM_MAX(0, interface), max.u64);
    let mut j: cvmx_gmxx_rxx_jabber = core::mem::zeroed(); j.u64 = 0; j.s.cnt = 64 * 1024 - 4; cvmx_write_csr(CVMX_GMXX_RXX_JABBER(0, interface), j.u64); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
