/*
 * Automatically generated functions useful for enabling and decoding
 * RSL_INT_BLOCKS interrupts.
 *
 * C headers and hardware definitions are supplied by external dependencies.
 */

pub unsafe fn __cvmx_interrupt_gmxx_rxx_int_en_enable(index: i32, block: i32) {
    let mut gmx_rx_int_en: cvmx_gmxx_rxx_int_en = core::mem::zeroed();
    cvmx_write_csr(CVMX_GMXX_RXX_INT_REG(index, block),
                   cvmx_read_csr(CVMX_GMXX_RXX_INT_REG(index, block)));
    gmx_rx_int_en.u64 = 0;
    if OCTEON_IS_MODEL(OCTEON_CN56XX) {
        gmx_rx_int_en.s.hg2cc = 1; gmx_rx_int_en.s.hg2fld = 1;
        gmx_rx_int_en.s.undat = 1; gmx_rx_int_en.s.uneop = 1;
        gmx_rx_int_en.s.unsop = 1; gmx_rx_int_en.s.bad_term = 1;
        gmx_rx_int_en.s.bad_seq = 1; gmx_rx_int_en.s.rem_fault = 1;
        gmx_rx_int_en.s.loc_fault = 1; gmx_rx_int_en.s.pause_drp = 1;
        gmx_rx_int_en.s.ovrerr = 1; gmx_rx_int_en.s.skperr = 1;
        gmx_rx_int_en.s.rcverr = 1; gmx_rx_int_en.s.jabber = 1;
        gmx_rx_int_en.s.carext = 1;
    }
    if OCTEON_IS_MODEL(OCTEON_CN30XX) {
        gmx_rx_int_en.s.ovrerr = 1; gmx_rx_int_en.s.niberr = 1;
        gmx_rx_int_en.s.skperr = 1; gmx_rx_int_en.s.rcverr = 1;
        gmx_rx_int_en.s.alnerr = 1; gmx_rx_int_en.s.jabber = 1;
        gmx_rx_int_en.s.maxerr = 1; gmx_rx_int_en.s.carext = 1;
        gmx_rx_int_en.s.minerr = 1;
    }
    if OCTEON_IS_MODEL(OCTEON_CN50XX) {
        gmx_rx_int_en.s.pause_drp = 1; gmx_rx_int_en.s.ovrerr = 1;
        gmx_rx_int_en.s.niberr = 1; gmx_rx_int_en.s.skperr = 1;
        gmx_rx_int_en.s.rcverr = 1; gmx_rx_int_en.s.alnerr = 1;
        gmx_rx_int_en.s.jabber = 1; gmx_rx_int_en.s.carext = 1;
    }
    if OCTEON_IS_MODEL(OCTEON_CN58XX) {
        gmx_rx_int_en.s.pause_drp = 1; gmx_rx_int_en.s.ovrerr = 1;
        gmx_rx_int_en.s.niberr = 1; gmx_rx_int_en.s.skperr = 1;
        gmx_rx_int_en.s.rcverr = 1; gmx_rx_int_en.s.alnerr = 1;
        gmx_rx_int_en.s.jabber = 1; gmx_rx_int_en.s.maxerr = 1;
        gmx_rx_int_en.s.carext = 1; gmx_rx_int_en.s.minerr = 1;
    }
    if OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN31XX) {
        gmx_rx_int_en.s.ovrerr = 1; gmx_rx_int_en.s.niberr = 1;
        gmx_rx_int_en.s.skperr = 1; gmx_rx_int_en.s.rcverr = 1;
        gmx_rx_int_en.s.alnerr = 1; gmx_rx_int_en.s.jabber = 1;
        gmx_rx_int_en.s.maxerr = 1; gmx_rx_int_en.s.carext = 1;
        gmx_rx_int_en.s.minerr = 1;
    }
    if OCTEON_IS_MODEL(OCTEON_CN52XX) {
        gmx_rx_int_en.s.hg2cc = 1; gmx_rx_int_en.s.hg2fld = 1;
        gmx_rx_int_en.s.undat = 1; gmx_rx_int_en.s.uneop = 1;
        gmx_rx_int_en.s.unsop = 1; gmx_rx_int_en.s.bad_term = 1;
        gmx_rx_int_en.s.bad_seq = 0; gmx_rx_int_en.s.rem_fault = 1;
        gmx_rx_int_en.s.loc_fault = 0; gmx_rx_int_en.s.pause_drp = 1;
        gmx_rx_int_en.s.ovrerr = 1; gmx_rx_int_en.s.skperr = 1;
        gmx_rx_int_en.s.rcverr = 1; gmx_rx_int_en.s.jabber = 1;
        gmx_rx_int_en.s.carext = 1;
    }
    cvmx_write_csr(CVMX_GMXX_RXX_INT_EN(index, block), gmx_rx_int_en.u64);
}

pub unsafe fn __cvmx_interrupt_pcsx_intx_en_reg_enable(index: i32, block: i32) {
    let mut v: cvmx_pcsx_intx_en_reg = core::mem::zeroed();
    cvmx_write_csr(CVMX_PCSX_INTX_REG(index, block), cvmx_read_csr(CVMX_PCSX_INTX_REG(index, block)));
    v.u64 = 0;
    if OCTEON_IS_MODEL(OCTEON_CN56XX) || OCTEON_IS_MODEL(OCTEON_CN52XX) {
        v.s.sync_bad_en = 1; v.s.an_bad_en = 1; v.s.rxlock_en = 1;
        v.s.rxbad_en = 1; v.s.txbad_en = 1; v.s.txfifo_en = 1;
        v.s.txfifu_en = 1; v.s.an_err_en = 1;
    }
    cvmx_write_csr(CVMX_PCSX_INTX_EN_REG(index, block), v.u64);
}

pub unsafe fn __cvmx_interrupt_pcsxx_int_en_reg_enable(index: i32) {
    let mut v: cvmx_pcsxx_int_en_reg = core::mem::zeroed();
    cvmx_write_csr(CVMX_PCSXX_INT_REG(index), cvmx_read_csr(CVMX_PCSXX_INT_REG(index)));
    v.u64 = 0;
    if OCTEON_IS_MODEL(OCTEON_CN56XX) || OCTEON_IS_MODEL(OCTEON_CN52XX) {
        v.s.algnlos_en = 1; v.s.synlos_en = 1; v.s.rxsynbad_en = 1;
        v.s.rxbad_en = 1; v.s.txflt_en = 1;
    }
    if OCTEON_IS_MODEL(OCTEON_CN52XX) { v.s.bitlckls_en = 0; }
    if OCTEON_IS_MODEL(OCTEON_CN56XX) { v.s.bitlckls_en = 1; }
    cvmx_write_csr(CVMX_PCSXX_INT_EN_REG(index), v.u64);
}

pub unsafe fn __cvmx_interrupt_spxx_int_msk_enable(index: i32) {
    let mut v: cvmx_spxx_int_msk = core::mem::zeroed();
    cvmx_write_csr(CVMX_SPXX_INT_REG(index), cvmx_read_csr(CVMX_SPXX_INT_REG(index)));
    v.u64 = 0;
    if OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) {
        v.s.calerr = 1; v.s.syncerr = 1; v.s.diperr = 1; v.s.tpaovr = 1;
        v.s.rsverr = 1; v.s.drwnng = 1; v.s.clserr = 1; v.s.spiovr = 1;
        v.s.abnorm = 1; v.s.prtnxa = 1;
    }
    cvmx_write_csr(CVMX_SPXX_INT_MSK(index), v.u64);
}

pub unsafe fn __cvmx_interrupt_stxx_int_msk_enable(index: i32) {
    let mut v: cvmx_stxx_int_msk = core::mem::zeroed();
    cvmx_write_csr(CVMX_STXX_INT_REG(index), cvmx_read_csr(CVMX_STXX_INT_REG(index)));
    v.u64 = 0;
    if OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) {
        v.s.frmerr = 1; v.s.unxfrm = 1; v.s.nosync = 1; v.s.diperr = 1;
        v.s.datovr = 1; v.s.ovrbst = 1; v.s.calpar1 = 1; v.s.calpar0 = 1;
    }
    cvmx_write_csr(CVMX_STXX_INT_MSK(index), v.u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
