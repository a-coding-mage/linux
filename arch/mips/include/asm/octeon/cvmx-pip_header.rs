/*
 * Interface to the hardware Packet Input Processing unit.
 * Translated from cvmx-pip.h; external headers and CSR definitions remain dependencies.
 */

pub const CVMX_PIP_NUM_INPUT_PORTS: u64 = 48;
pub const CVMX_PIP_NUM_WATCHERS: u64 = 4;

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cvmx_pip_l4_err_t {
    CVMX_PIP_L4_NO_ERR = 0,
    CVMX_PIP_L4_MAL_ERR = 1,
    CVMX_PIP_CHK_ERR = 2,
    CVMX_PIP_L4_LENGTH_ERR = 3,
    CVMX_PIP_BAD_PRT_ERR = 4,
    CVMX_PIP_TCP_FLG8_ERR = 8,
    CVMX_PIP_TCP_FLG9_ERR = 9,
    CVMX_PIP_TCP_FLG10_ERR = 10,
    CVMX_PIP_TCP_FLG11_ERR = 11,
    CVMX_PIP_TCP_FLG12_ERR = 12,
    CVMX_PIP_TCP_FLG13_ERR = 13,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cvmx_pip_ip_exc_t {
    CVMX_PIP_IP_NO_ERR = 0,
    CVMX_PIP_NOT_IP = 1,
    CVMX_PIP_IPV4_HDR_CHK = 2,
    CVMX_PIP_IP_MAL_HDR = 3,
    CVMX_PIP_IP_MAL_PKT = 4,
    CVMX_PIP_TTL_HOP = 5,
    CVMX_PIP_OPTS = 6,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cvmx_pip_rcv_err_t {
    CVMX_PIP_RX_NO_ERR = 0,
    CVMX_PIP_PARTIAL_ERR = 1,
    CVMX_PIP_JABBER_ERR = 2,
    CVMX_PIP_OVER_FCS_ERR = 3,
    CVMX_PIP_OVER_ERR = 4,
    CVMX_PIP_ALIGN_ERR = 5,
    CVMX_PIP_UNDER_FCS_ERR = 6,
    CVMX_PIP_GMX_FCS_ERR = 7,
    CVMX_PIP_UNDER_ERR = 8,
    CVMX_PIP_EXTEND_ERR = 9,
    CVMX_PIP_LENGTH_ERR = 10,
    CVMX_PIP_DAT_ERR = 11,
    CVMX_PIP_DIP_ERR = 11,
    CVMX_PIP_SKIP_ERR = 12,
    CVMX_PIP_NIBBLE_ERR = 13,
    CVMX_PIP_PIP_FCS = 16,
    CVMX_PIP_PIP_SKIP_ERR = 17,
    CVMX_PIP_PIP_L2_MAL_HDR = 18,
}

#[repr(C)]
pub union cvmx_pip_err_t {
    pub l4_err: cvmx_pip_l4_err_t,
    pub ip_exc: cvmx_pip_ip_exc_t,
    pub rcv_err: cvmx_pip_rcv_err_t,
}

#[repr(C)]
pub struct cvmx_pip_port_status {
    pub dropped_octets: u32,
    pub dropped_packets: u32,
    pub pci_raw_packets: u32,
    pub octets: u32,
    pub packets: u32,
    pub multicast_packets: u32,
    pub broadcast_packets: u32,
    pub len_64_packets: u32,
    pub len_65_127_packets: u32,
    pub len_128_255_packets: u32,
    pub len_256_511_packets: u32,
    pub len_512_1023_packets: u32,
    pub len_1024_1518_packets: u32,
    pub len_1519_max_packets: u32,
    pub fcs_align_err_packets: u32,
    pub runt_packets: u32,
    pub runt_crc_packets: u32,
    pub oversize_packets: u32,
    pub oversize_crc_packets: u32,
    pub inb_packets: u32,
    pub inb_octets: u64,
    pub inb_errors: u16,
}

/* C bitfields are represented by their containing hardware word. */
#[repr(C)]
pub union cvmx_pip_pkt_inst_hdr_t {
    pub u64: u64,
    pub s: cvmx_pip_pkt_inst_hdr_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_pip_pkt_inst_hdr_s {
    pub rawfull: u64,
    pub reserved0: u64,
    pub parse_mode: u64,
    pub reserved1: u64,
    pub skip_len: u64,
    pub reserved2: u64,
    pub qos: u64,
    pub grp: u64,
    pub rs: u64,
    pub tag_type: u64,
    pub tag: u64,
}

pub unsafe fn cvmx_pip_config_port(
    port_num: u64,
    port_cfg: cvmx_pip_prt_cfgx,
    port_tag_cfg: cvmx_pip_prt_tagx,
) {
    cvmx_write_csr(CVMX_PIP_PRT_CFGX(port_num), port_cfg.u64);
    cvmx_write_csr(CVMX_PIP_PRT_TAGX(port_num), port_tag_cfg.u64);
}

pub unsafe fn cvmx_pip_config_vlan_qos(vlan_priority: u64, qos: u64) {
    let mut pip_qos_vlanx: cvmx_pip_qos_vlanx = core::mem::zeroed();
    pip_qos_vlanx.u64 = 0;
    pip_qos_vlanx.s.qos = qos;
    cvmx_write_csr(CVMX_PIP_QOS_VLANX(vlan_priority), pip_qos_vlanx.u64);
}

pub unsafe fn cvmx_pip_config_diffserv_qos(diffserv: u64, qos: u64) {
    let mut pip_qos_diffx: cvmx_pip_qos_diffx = core::mem::zeroed();
    pip_qos_diffx.u64 = 0;
    pip_qos_diffx.s.qos = qos;
    cvmx_write_csr(CVMX_PIP_QOS_DIFFX(diffserv), pip_qos_diffx.u64);
}

pub unsafe fn cvmx_pip_get_port_status(
    port_num: u64,
    clear: u64,
    status: *mut cvmx_pip_port_status,
) {
    let mut pip_stat_ctl: cvmx_pip_stat_ctl = core::mem::zeroed();
    pip_stat_ctl.u64 = 0;
    pip_stat_ctl.s.rdclr = clear;
    cvmx_write_csr(CVMX_PIP_STAT_CTL, pip_stat_ctl.u64);
    let stat0: cvmx_pip_stat0_prtx = cvmx_pip_stat0_prtx { u64: cvmx_read_csr(CVMX_PIP_STAT0_PRTX(port_num)) };
    let stat1: cvmx_pip_stat1_prtx = cvmx_pip_stat1_prtx { u64: cvmx_read_csr(CVMX_PIP_STAT1_PRTX(port_num)) };
    let stat2: cvmx_pip_stat2_prtx = cvmx_pip_stat2_prtx { u64: cvmx_read_csr(CVMX_PIP_STAT2_PRTX(port_num)) };
    let stat3: cvmx_pip_stat3_prtx = cvmx_pip_stat3_prtx { u64: cvmx_read_csr(CVMX_PIP_STAT3_PRTX(port_num)) };
    let stat4: cvmx_pip_stat4_prtx = cvmx_pip_stat4_prtx { u64: cvmx_read_csr(CVMX_PIP_STAT4_PRTX(port_num)) };
    let stat5: cvmx_pip_stat5_prtx = cvmx_pip_stat5_prtx { u64: cvmx_read_csr(CVMX_PIP_STAT5_PRTX(port_num)) };
    let stat6: cvmx_pip_stat6_prtx = cvmx_pip_stat6_prtx { u64: cvmx_read_csr(CVMX_PIP_STAT6_PRTX(port_num)) };
    let stat7: cvmx_pip_stat7_prtx = cvmx_pip_stat7_prtx { u64: cvmx_read_csr(CVMX_PIP_STAT7_PRTX(port_num)) };
    let stat8: cvmx_pip_stat8_prtx = cvmx_pip_stat8_prtx { u64: cvmx_read_csr(CVMX_PIP_STAT8_PRTX(port_num)) };
    let stat9: cvmx_pip_stat9_prtx = cvmx_pip_stat9_prtx { u64: cvmx_read_csr(CVMX_PIP_STAT9_PRTX(port_num)) };
    let inb_pkts: cvmx_pip_stat_inb_pktsx = cvmx_pip_stat_inb_pktsx { u64: cvmx_read_csr(CVMX_PIP_STAT_INB_PKTSX(port_num)) };
    let inb_octs: cvmx_pip_stat_inb_octsx = cvmx_pip_stat_inb_octsx { u64: cvmx_read_csr(CVMX_PIP_STAT_INB_OCTSX(port_num)) };
    let inb_errs: cvmx_pip_stat_inb_errsx = cvmx_pip_stat_inb_errsx { u64: cvmx_read_csr(CVMX_PIP_STAT_INB_ERRSX(port_num)) };
    (*status).dropped_octets = stat0.s.drp_octs;
    (*status).dropped_packets = stat0.s.drp_pkts;
    (*status).octets = stat1.s.octs;
    (*status).pci_raw_packets = stat2.s.raw;
    (*status).packets = stat2.s.pkts;
    (*status).multicast_packets = stat3.s.mcst;
    (*status).broadcast_packets = stat3.s.bcst;
    (*status).len_64_packets = stat4.s.h64;
    (*status).len_65_127_packets = stat4.s.h65to127;
    (*status).len_128_255_packets = stat5.s.h128to255;
    (*status).len_256_511_packets = stat5.s.h256to511;
    (*status).len_512_1023_packets = stat6.s.h512to1023;
    (*status).len_1024_1518_packets = stat6.s.h1024to1518;
    (*status).len_1519_max_packets = stat7.s.h1519;
    (*status).fcs_align_err_packets = stat7.s.fcs;
    (*status).runt_packets = stat8.s.undersz;
    (*status).runt_crc_packets = stat8.s.frag;
    (*status).oversize_packets = stat9.s.oversz;
    (*status).oversize_crc_packets = stat9.s.jabber;
    (*status).inb_packets = inb_pkts.s.pkts;
    (*status).inb_octets = inb_octs.s.octs;
    (*status).inb_errors = inb_errs.s.errs;
    if cvmx_octeon_is_pass1() {
        if (*status).inb_packets > (*status).packets { (*status).dropped_packets = (*status).inb_packets - (*status).packets; } else { (*status).dropped_packets = 0; }
        if (*status).inb_octets - (*status).inb_packets as u64 * 4 > (*status).octets as u64 { (*status).dropped_octets = ((*status).inb_octets - (*status).inb_packets as u64 * 4 - (*status).octets as u64) as u32; } else { (*status).dropped_octets = 0; }
    }
}

pub unsafe fn cvmx_pip_config_crc(interface: u64, invert_result: u64, reflect: u64, initialization_vector: u32) {
    if OCTEON_IS_MODEL(OCTEON_CN38XX) || OCTEON_IS_MODEL(OCTEON_CN58XX) {
        let mut config: cvmx_pip_crc_ctlx = core::mem::zeroed();
        let mut iv: cvmx_pip_crc_ivx = core::mem::zeroed();
        config.u64 = 0; config.s.invres = invert_result; config.s.reflect = reflect;
        cvmx_write_csr(CVMX_PIP_CRC_CTLX(interface), config.u64);
        iv.u64 = 0; iv.s.iv = initialization_vector;
        cvmx_write_csr(CVMX_PIP_CRC_IVX(interface), iv.u64);
    }
}

pub unsafe fn cvmx_pip_tag_mask_clear(mask_index: u64) {
    let mut pip_tag_incx: cvmx_pip_tag_incx = core::mem::zeroed();
    pip_tag_incx.u64 = 0; pip_tag_incx.s.en = 0;
    let mut index = mask_index * 16;
    while index < (mask_index + 1) * 16 { cvmx_write_csr(CVMX_PIP_TAG_INCX(index), pip_tag_incx.u64); index += 1; }
}

pub unsafe fn cvmx_pip_tag_mask_set(mut mask_index: u64, mut offset: u64, mut len: u64) {
    while len != 0 {
        let index = mask_index * 16 + offset / 8;
        let mut pip_tag_incx: cvmx_pip_tag_incx = core::mem::zeroed();
        pip_tag_incx.u64 = cvmx_read_csr(CVMX_PIP_TAG_INCX(index));
        pip_tag_incx.s.en |= 0x80 >> (offset & 0x7);
        cvmx_write_csr(CVMX_PIP_TAG_INCX(index), pip_tag_incx.u64);
        offset += 1; len -= 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
