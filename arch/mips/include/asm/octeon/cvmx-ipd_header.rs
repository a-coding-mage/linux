/*
 * Interface to the hardware Input Packet Data unit.
 *
 * This file is a source-level Rust translation of cvmx-ipd.h.  Definitions
 * supplied by the OCTEON headers remain external dependencies.
 */

#[repr(i64)]
pub enum cvmx_ipd_mode {
    CVMX_IPD_OPC_MODE_STT = 0,
    CVMX_IPD_OPC_MODE_STF = 1,
    CVMX_IPD_OPC_MODE_STF1_STT = 2,
    CVMX_IPD_OPC_MODE_STF2_STT = 3,
}

pub const CVMX_ENABLE_LEN_M8_FIX: i32 = 0;

pub type cvmx_ipd_mbuff_first_skip_t = cvmx_ipd_1st_mbuff_skip;
pub type cvmx_ipd_first_next_ptr_back_t = cvmx_ipd_1st_next_ptr_back;
pub type cvmx_ipd_mbuff_not_first_skip_t = cvmx_ipd_mbuff_first_skip_t;
pub type cvmx_ipd_second_next_ptr_back_t = cvmx_ipd_first_next_ptr_back_t;

/* External declarations supplied by the OCTEON headers/platform. */
extern "C" {
    fn cvmx_write_csr(address: u64, value: u64);
    fn cvmx_read_csr(address: u64) -> u64;
    fn cvmx_dprintf(format: *const i8, ...);
    fn OCTEON_IS_MODEL(model: u32) -> bool;
    fn octeon_has_feature(feature: u32) -> bool;
    fn cvmx_phys_to_ptr(address: u64) -> *mut core::ffi::c_void;
    fn cvmx_fpa_free(ptr: *mut core::ffi::c_void, pool: u32, aura: u64);
}

pub unsafe fn cvmx_ipd_config(
    mbuff_size: u64, first_mbuff_skip: u64, not_first_mbuff_skip: u64,
    first_back: u64, second_back: u64, wqe_fpa_pool: u64,
    cache_mode: cvmx_ipd_mode, back_pres_enable_flag: u64,
) {
    let mut first_skip: cvmx_ipd_mbuff_first_skip_t = core::mem::zeroed();
    first_skip.u64 = 0; first_skip.s.skip_sz = first_mbuff_skip;
    cvmx_write_csr(CVMX_IPD_1ST_MBUFF_SKIP, first_skip.u64);
    let mut not_first_skip: cvmx_ipd_mbuff_not_first_skip_t = core::mem::zeroed();
    not_first_skip.u64 = 0; not_first_skip.s.skip_sz = not_first_mbuff_skip;
    cvmx_write_csr(CVMX_IPD_NOT_1ST_MBUFF_SKIP, not_first_skip.u64);
    let mut size: cvmx_ipd_packet_mbuff_size = core::mem::zeroed();
    size.u64 = 0; size.s.mb_size = mbuff_size;
    cvmx_write_csr(CVMX_IPD_PACKET_MBUFF_SIZE, size.u64);
    let mut first_back_struct: cvmx_ipd_first_next_ptr_back_t = core::mem::zeroed();
    first_back_struct.u64 = 0; first_back_struct.s.back = first_back;
    cvmx_write_csr(CVMX_IPD_1st_NEXT_PTR_BACK, first_back_struct.u64);
    let mut second_back_struct: cvmx_ipd_second_next_ptr_back_t = core::mem::zeroed();
    second_back_struct.u64 = 0; second_back_struct.s.back = second_back;
    cvmx_write_csr(CVMX_IPD_2nd_NEXT_PTR_BACK, second_back_struct.u64);
    let mut wqe_pool: cvmx_ipd_wqe_fpa_queue = core::mem::zeroed();
    wqe_pool.u64 = 0; wqe_pool.s.wqe_pool = wqe_fpa_pool;
    cvmx_write_csr(CVMX_IPD_WQE_FPA_QUEUE, wqe_pool.u64);
    let mut ipd_ctl_reg: cvmx_ipd_ctl_status = core::mem::zeroed();
    ipd_ctl_reg.u64 = cvmx_read_csr(CVMX_IPD_CTL_STATUS);
    ipd_ctl_reg.s.opc_mode = cache_mode as _;
    ipd_ctl_reg.s.pbp_en = back_pres_enable_flag;
    cvmx_write_csr(CVMX_IPD_CTL_STATUS, ipd_ctl_reg.u64);
}

pub unsafe fn cvmx_ipd_enable() {
    let mut ipd_reg: cvmx_ipd_ctl_status = core::mem::zeroed();
    ipd_reg.u64 = cvmx_read_csr(CVMX_IPD_CTL_STATUS);
    if ipd_reg.s.ipd_en { cvmx_dprintf(b"Warning: Enabling IPD when IPD already enabled.\n\0".as_ptr() as *const i8); }
    ipd_reg.s.ipd_en = 1;
    if CVMX_ENABLE_LEN_M8_FIX != 0 && !OCTEON_IS_MODEL(OCTEON_CN38XX_PASS2) { ipd_reg.s.len_m8 = 1; }
    cvmx_write_csr(CVMX_IPD_CTL_STATUS, ipd_reg.u64);
}

pub unsafe fn cvmx_ipd_disable() {
    let mut ipd_reg: cvmx_ipd_ctl_status = core::mem::zeroed();
    ipd_reg.u64 = cvmx_read_csr(CVMX_IPD_CTL_STATUS); ipd_reg.s.ipd_en = 0;
    cvmx_write_csr(CVMX_IPD_CTL_STATUS, ipd_reg.u64);
}

pub unsafe fn cvmx_ipd_free_ptr() {
    if !OCTEON_IS_MODEL(OCTEON_CN38XX_PASS1) && !OCTEON_IS_MODEL(OCTEON_CN38XX_PASS2) {
        let mut count: cvmx_ipd_ptr_count = core::mem::zeroed(); count.u64 = cvmx_read_csr(CVMX_IPD_PTR_COUNT);
        let mut no_wptr = 0;
        if octeon_has_feature(OCTEON_FEATURE_NO_WPTR) { let mut c: cvmx_ipd_ctl_status = core::mem::zeroed(); c.u64 = cvmx_read_csr(CVMX_IPD_CTL_STATUS); if c.s.no_wptr { no_wptr = 1; } }
        if count.s.wqev_cnt { let mut p: cvmx_ipd_wqe_ptr_valid = core::mem::zeroed(); p.u64 = cvmx_read_csr(CVMX_IPD_WQE_PTR_VALID); cvmx_fpa_free(cvmx_phys_to_ptr((p.s.ptr as u64) << 7), if no_wptr != 0 { CVMX_FPA_PACKET_POOL } else { CVMX_FPA_WQE_POOL }, 0); }
        if count.s.wqe_pcnt { let mut c: cvmx_ipd_pwp_ptr_fifo_ctl = core::mem::zeroed(); c.u64 = cvmx_read_csr(CVMX_IPD_PWP_PTR_FIFO_CTL); for i in 0..count.s.wqe_pcnt { c.s.cena = 0; c.s.raddr = c.s.max_cnts + (c.s.wraddr + i) % c.s.max_cnts; cvmx_write_csr(CVMX_IPD_PWP_PTR_FIFO_CTL, c.u64); c.u64 = cvmx_read_csr(CVMX_IPD_PWP_PTR_FIFO_CTL); cvmx_fpa_free(cvmx_phys_to_ptr((c.s.ptr as u64) << 7), if no_wptr != 0 { CVMX_FPA_PACKET_POOL } else { CVMX_FPA_WQE_POOL }, 0); } c.s.cena = 1; cvmx_write_csr(CVMX_IPD_PWP_PTR_FIFO_CTL, c.u64); }
        /* The remaining FIFO draining and IPD/PIP reset operations are kept in
         * the same order as the C header. */
        if count.s.pktv_cnt { let mut p: cvmx_ipd_pkt_ptr_valid = core::mem::zeroed(); p.u64 = cvmx_read_csr(CVMX_IPD_PKT_PTR_VALID); cvmx_fpa_free(cvmx_phys_to_ptr((p.s.ptr as u64) << 7), CVMX_FPA_PACKET_POOL, 0); }
        let mut c: cvmx_ipd_prc_port_ptr_fifo_ctl = core::mem::zeroed(); c.u64 = cvmx_read_csr(CVMX_IPD_PRC_PORT_PTR_FIFO_CTL); for i in 0..c.s.max_pkt { c.s.cena=0; c.s.raddr=i%c.s.max_pkt; cvmx_write_csr(CVMX_IPD_PRC_PORT_PTR_FIFO_CTL,c.u64); c.u64=cvmx_read_csr(CVMX_IPD_PRC_PORT_PTR_FIFO_CTL); cvmx_fpa_free(cvmx_phys_to_ptr((c.s.ptr as u64)<<7),CVMX_FPA_PACKET_POOL,0); } c.s.cena=1; cvmx_write_csr(CVMX_IPD_PRC_PORT_PTR_FIFO_CTL,c.u64);
        if count.s.pfif_cnt { let mut h: cvmx_ipd_prc_hold_ptr_fifo_ctl=core::mem::zeroed(); h.u64=cvmx_read_csr(CVMX_IPD_PRC_HOLD_PTR_FIFO_CTL); for i in 0..count.s.pfif_cnt { h.s.cena=0; h.s.raddr=(h.s.praddr+i)%h.s.max_pkt; cvmx_write_csr(CVMX_IPD_PRC_HOLD_PTR_FIFO_CTL,h.u64); h.u64=cvmx_read_csr(CVMX_IPD_PRC_HOLD_PTR_FIFO_CTL); cvmx_fpa_free(cvmx_phys_to_ptr((h.s.ptr as u64)<<7),CVMX_FPA_PACKET_POOL,0); } h.s.cena=1; cvmx_write_csr(CVMX_IPD_PRC_HOLD_PTR_FIFO_CTL,h.u64); }
        if count.s.pkt_pcnt { let mut p: cvmx_ipd_pwp_ptr_fifo_ctl=core::mem::zeroed(); p.u64=cvmx_read_csr(CVMX_IPD_PWP_PTR_FIFO_CTL); for i in 0..count.s.pkt_pcnt { p.s.cena=0; p.s.raddr=(p.s.praddr+i)%p.s.max_cnts; cvmx_write_csr(CVMX_IPD_PWP_PTR_FIFO_CTL,p.u64); p.u64=cvmx_read_csr(CVMX_IPD_PWP_PTR_FIFO_CTL); cvmx_fpa_free(cvmx_phys_to_ptr((p.s.ptr as u64)<<7),CVMX_FPA_PACKET_POOL,0); } p.s.cena=1; cvmx_write_csr(CVMX_IPD_PWP_PTR_FIFO_CTL,p.u64); }
        let mut ctl: cvmx_ipd_ctl_status=core::mem::zeroed(); ctl.u64=cvmx_read_csr(CVMX_IPD_CTL_STATUS); ctl.s.reset=1; cvmx_write_csr(CVMX_IPD_CTL_STATUS,ctl.u64);
        let mut pip: cvmx_pip_sft_rst=core::mem::zeroed(); pip.u64=cvmx_read_csr(CVMX_PIP_SFT_RST); pip.s.rst=1; cvmx_write_csr(CVMX_PIP_SFT_RST,pip.u64);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
