/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of ncsi-pkt.h. */

pub type __be16 = u16;
pub type __be32 = u32;
pub type __be64 = u64;
pub const ETH_ALEN: usize = 6;

#[repr(C)] pub struct ncsi_pkt_hdr { pub mc_id:u8, pub revision:u8, pub reserved:u8, pub id:u8, pub r#type:u8, pub channel:u8, pub length:__be16, pub reserved1:[__be32;2] }
#[repr(C)] pub struct ncsi_cmd_pkt_hdr { pub common:ncsi_pkt_hdr }
#[repr(C)] pub struct ncsi_rsp_pkt_hdr { pub common:ncsi_pkt_hdr, pub code:__be16, pub reason:__be16 }
#[repr(C)] pub struct ncsi_aen_pkt_hdr { pub common:ncsi_pkt_hdr, pub reserved2:[u8;3], pub r#type:u8 }

#[repr(C)] pub struct ncsi_cmd_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub checksum:__be32, pub pad:[u8;26] }
#[repr(C)] pub struct ncsi_rsp_pkt { pub rsp:ncsi_rsp_pkt_hdr, pub checksum:__be32, pub pad:[u8;22] }
#[repr(C)] pub struct ncsi_cmd_sp_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub reserved:[u8;3], pub hw_arbitration:u8, pub checksum:__be32, pub pad:[u8;22] }
#[repr(C)] pub struct ncsi_cmd_dc_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub reserved:[u8;3], pub ald:u8, pub checksum:__be32, pub pad:[u8;22] }
#[repr(C)] pub struct ncsi_cmd_rc_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub reserved:__be32, pub checksum:__be32, pub pad:[u8;22] }
#[repr(C)] pub struct ncsi_cmd_ae_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub reserved:[u8;3], pub mc_id:u8, pub mode:__be32, pub checksum:__be32, pub pad:[u8;18] }
#[repr(C)] pub struct ncsi_cmd_sl_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub mode:__be32, pub oem_mode:__be32, pub checksum:__be32, pub pad:[u8;18] }
#[repr(C)] pub struct ncsi_cmd_svf_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub reserved:__be16, pub vlan:__be16, pub reserved1:__be16, pub index:u8, pub enable:u8, pub checksum:__be32, pub pad:[u8;18] }
#[repr(C)] pub struct ncsi_cmd_ev_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub reserved:[u8;3], pub mode:u8, pub checksum:__be32, pub pad:[u8;22] }
#[repr(C)] pub struct ncsi_cmd_sma_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub mac:[u8;6], pub index:u8, pub at_e:u8, pub checksum:__be32, pub pad:[u8;18] }
#[repr(C)] pub struct ncsi_cmd_ebf_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub mode:__be32, pub checksum:__be32, pub pad:[u8;22] }
#[repr(C)] pub struct ncsi_cmd_egmf_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub mode:__be32, pub checksum:__be32, pub pad:[u8;22] }
#[repr(C)] pub struct ncsi_cmd_snfc_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub reserved:[u8;3], pub mode:u8, pub checksum:__be32, pub pad:[u8;22] }
#[repr(C)] pub struct ncsi_cmd_oem_pkt { pub cmd:ncsi_cmd_pkt_hdr, pub mfr_id:__be32, pub data:[u8;0] }
#[repr(C)] pub struct ncsi_rsp_oem_pkt { pub rsp:ncsi_rsp_pkt_hdr, pub mfr_id:__be32, pub data:[u8;0] }
#[repr(C)] pub struct ncsi_rsp_oem_mlx_pkt { pub cmd_rev:u8, pub cmd:u8, pub param:u8, pub optional:u8, pub data:[u8;0] }
#[repr(C)] pub struct ncsi_rsp_oem_bcm_pkt { pub ver:u8, pub r#type:u8, pub len:__be16, pub data:[u8;0] }
#[repr(C)] pub struct ncsi_rsp_oem_intel_pkt { pub cmd:u8, pub data:[u8;0] }

#[repr(C)] pub struct ncsi_rsp_gls_pkt { pub rsp:ncsi_rsp_pkt_hdr, pub status:__be32, pub other:__be32, pub oem_status:__be32, pub checksum:__be32, pub pad:[u8;10] }
#[repr(C)] pub struct ncsi_rsp_gvi_pkt { pub rsp:ncsi_rsp_pkt_hdr, pub major:u8, pub minor:u8, pub update:u8, pub alpha1:u8, pub reserved:[u8;3], pub alpha2:u8, pub fw_name:[u8;12], pub fw_version:__be32, pub pci_ids:[__be16;4], pub mf_id:__be32, pub checksum:__be32 }
#[repr(C)] pub struct ncsi_rsp_gc_pkt { pub rsp:ncsi_rsp_pkt_hdr, pub cap:__be32, pub bc_cap:__be32, pub mc_cap:__be32, pub buf_cap:__be32, pub aen_cap:__be32, pub vlan_cnt:u8, pub mixed_cnt:u8, pub mc_cnt:u8, pub uc_cnt:u8, pub reserved:[u8;2], pub vlan_mode:u8, pub channel_cnt:u8, pub checksum:__be32 }
#[repr(C)] pub struct ncsi_rsp_gp_pkt { pub rsp:ncsi_rsp_pkt_hdr, pub mac_cnt:u8, pub reserved:[u8;2], pub mac_enable:u8, pub vlan_cnt:u8, pub reserved1:u8, pub vlan_enable:__be16, pub link_mode:__be32, pub bc_mode:__be32, pub valid_modes:__be32, pub vlan_mode:u8, pub fc_mode:u8, pub reserved2:[u8;2], pub aen_mode:__be32, pub mac:[u8;6], pub vlan:__be16, pub checksum:__be32 }

#[repr(C, packed(4))] pub struct ncsi_rsp_gcps_pkt {
 pub rsp:ncsi_rsp_pkt_hdr, pub cnt:__be64, pub rx_bytes:__be64, pub tx_bytes:__be64, pub rx_uc_pkts:__be64, pub rx_mc_pkts:__be64, pub rx_bc_pkts:__be64, pub tx_uc_pkts:__be64, pub tx_mc_pkts:__be64, pub tx_bc_pkts:__be64,
 pub fcs_err:__be32, pub align_err:__be32, pub false_carrier:__be32, pub runt_pkts:__be32, pub jabber_pkts:__be32, pub rx_pause_xon:__be32, pub rx_pause_xoff:__be32, pub tx_pause_xon:__be32, pub tx_pause_xoff:__be32, pub tx_s_collision:__be32, pub tx_m_collision:__be32, pub l_collision:__be32, pub e_collision:__be32, pub rx_ctl_frames:__be32, pub rx_64_frames:__be32, pub rx_127_frames:__be32, pub rx_255_frames:__be32, pub rx_511_frames:__be32, pub rx_1023_frames:__be32, pub rx_1522_frames:__be32, pub rx_9022_frames:__be32, pub tx_64_frames:__be32, pub tx_127_frames:__be32, pub tx_255_frames:__be32, pub tx_511_frames:__be32, pub tx_1023_frames:__be32, pub tx_1522_frames:__be32, pub tx_9022_frames:__be32, pub rx_valid_bytes:__be64, pub rx_runt_pkts:__be32, pub rx_jabber_pkts:__be32, pub checksum:__be32
}
#[repr(C)] pub struct ncsi_rsp_gns_pkt { pub rsp:ncsi_rsp_pkt_hdr, pub rx_cmds:__be32, pub dropped_cmds:__be32, pub cmd_type_errs:__be32, pub cmd_csum_errs:__be32, pub rx_pkts:__be32, pub tx_pkts:__be32, pub tx_aen_pkts:__be32, pub checksum:__be32 }
#[repr(C)] pub struct ncsi_rsp_gnpts_pkt { pub rsp:ncsi_rsp_pkt_hdr, pub tx_pkts:__be32, pub tx_dropped:__be32, pub tx_channel_err:__be32, pub tx_us_err:__be32, pub rx_pkts:__be32, pub rx_dropped:__be32, pub rx_channel_err:__be32, pub rx_us_err:__be32, pub rx_os_err:__be32, pub checksum:__be32 }
#[repr(C)] pub struct ncsi_rsp_gps_pkt { pub rsp:ncsi_rsp_pkt_hdr, pub status:__be32, pub checksum:__be32 }
#[repr(C)] pub struct ncsi_rsp_gpuuid_pkt { pub rsp:ncsi_rsp_pkt_hdr, pub uuid:[u8;16], pub checksum:__be32 }
#[repr(C)] pub struct ncsi_rsp_gmcma_pkt { pub rsp:ncsi_rsp_pkt_hdr, pub address_count:u8, pub reserved:[u8;3], pub addresses:[[u8;ETH_ALEN];0] }
#[repr(C)] pub struct ncsi_aen_lsc_pkt { pub aen:ncsi_aen_pkt_hdr, pub status:__be32, pub oem_status:__be32, pub checksum:__be32, pub pad:[u8;14] }
#[repr(C)] pub struct ncsi_aen_cr_pkt { pub aen:ncsi_aen_pkt_hdr, pub checksum:__be32, pub pad:[u8;22] }
#[repr(C)] pub struct ncsi_aen_hncdsc_pkt { pub aen:ncsi_aen_pkt_hdr, pub status:__be32, pub checksum:__be32, pub pad:[u8;18] }

pub const NCSI_PKT_REVISION:u32=0x01;
pub const NCSI_PKT_CMD_CIS:u32=0x00; pub const NCSI_PKT_CMD_SP:u32=0x01; pub const NCSI_PKT_CMD_DP:u32=0x02; pub const NCSI_PKT_CMD_EC:u32=0x03; pub const NCSI_PKT_CMD_DC:u32=0x04; pub const NCSI_PKT_CMD_RC:u32=0x05; pub const NCSI_PKT_CMD_ECNT:u32=0x06; pub const NCSI_PKT_CMD_DCNT:u32=0x07; pub const NCSI_PKT_CMD_AE:u32=0x08; pub const NCSI_PKT_CMD_SL:u32=0x09; pub const NCSI_PKT_CMD_GLS:u32=0x0a; pub const NCSI_PKT_CMD_SVF:u32=0x0b; pub const NCSI_PKT_CMD_EV:u32=0x0c; pub const NCSI_PKT_CMD_DV:u32=0x0d; pub const NCSI_PKT_CMD_SMA:u32=0x0e; pub const NCSI_PKT_CMD_EBF:u32=0x10; pub const NCSI_PKT_CMD_DBF:u32=0x11; pub const NCSI_PKT_CMD_EGMF:u32=0x12; pub const NCSI_PKT_CMD_DGMF:u32=0x13; pub const NCSI_PKT_CMD_SNFC:u32=0x14; pub const NCSI_PKT_CMD_GVI:u32=0x15; pub const NCSI_PKT_CMD_GC:u32=0x16; pub const NCSI_PKT_CMD_GP:u32=0x17; pub const NCSI_PKT_CMD_GCPS:u32=0x18; pub const NCSI_PKT_CMD_GNS:u32=0x19; pub const NCSI_PKT_CMD_GNPTS:u32=0x1a; pub const NCSI_PKT_CMD_GPS:u32=0x1b; pub const NCSI_PKT_CMD_OEM:u32=0x50; pub const NCSI_PKT_CMD_PLDM:u32=0x51; pub const NCSI_PKT_CMD_GPUUID:u32=0x52; pub const NCSI_PKT_CMD_QPNPR:u32=0x56; pub const NCSI_PKT_CMD_SNPR:u32=0x57; pub const NCSI_PKT_CMD_GMCMA:u32=0x58;

pub const NCSI_PKT_RSP_CIS:u32=NCSI_PKT_CMD_CIS+0x80; pub const NCSI_PKT_RSP_SP:u32=NCSI_PKT_CMD_SP+0x80; pub const NCSI_PKT_RSP_DP:u32=NCSI_PKT_CMD_DP+0x80; pub const NCSI_PKT_RSP_EC:u32=NCSI_PKT_CMD_EC+0x80; pub const NCSI_PKT_RSP_DC:u32=NCSI_PKT_CMD_DC+0x80; pub const NCSI_PKT_RSP_RC:u32=NCSI_PKT_CMD_RC+0x80; pub const NCSI_PKT_RSP_ECNT:u32=NCSI_PKT_CMD_ECNT+0x80; pub const NCSI_PKT_RSP_DCNT:u32=NCSI_PKT_CMD_DCNT+0x80; pub const NCSI_PKT_RSP_AE:u32=NCSI_PKT_CMD_AE+0x80; pub const NCSI_PKT_RSP_SL:u32=NCSI_PKT_CMD_SL+0x80; pub const NCSI_PKT_RSP_GLS:u32=NCSI_PKT_CMD_GLS+0x80; pub const NCSI_PKT_RSP_SVF:u32=NCSI_PKT_CMD_SVF+0x80; pub const NCSI_PKT_RSP_EV:u32=NCSI_PKT_CMD_EV+0x80; pub const NCSI_PKT_RSP_DV:u32=NCSI_PKT_CMD_DV+0x80; pub const NCSI_PKT_RSP_SMA:u32=NCSI_PKT_CMD_SMA+0x80; pub const NCSI_PKT_RSP_EBF:u32=NCSI_PKT_CMD_EBF+0x80; pub const NCSI_PKT_RSP_DBF:u32=NCSI_PKT_CMD_DBF+0x80; pub const NCSI_PKT_RSP_EGMF:u32=NCSI_PKT_CMD_EGMF+0x80; pub const NCSI_PKT_RSP_DGMF:u32=NCSI_PKT_CMD_DGMF+0x80; pub const NCSI_PKT_RSP_SNFC:u32=NCSI_PKT_CMD_SNFC+0x80; pub const NCSI_PKT_RSP_GVI:u32=NCSI_PKT_CMD_GVI+0x80; pub const NCSI_PKT_RSP_GC:u32=NCSI_PKT_CMD_GC+0x80; pub const NCSI_PKT_RSP_GP:u32=NCSI_PKT_CMD_GP+0x80; pub const NCSI_PKT_RSP_GCPS:u32=NCSI_PKT_CMD_GCPS+0x80; pub const NCSI_PKT_RSP_GNS:u32=NCSI_PKT_CMD_GNS+0x80; pub const NCSI_PKT_RSP_GNPTS:u32=NCSI_PKT_CMD_GNPTS+0x80; pub const NCSI_PKT_RSP_GPS:u32=NCSI_PKT_CMD_GPS+0x80; pub const NCSI_PKT_RSP_OEM:u32=NCSI_PKT_CMD_OEM+0x80; pub const NCSI_PKT_RSP_PLDM:u32=NCSI_PKT_CMD_PLDM+0x80; pub const NCSI_PKT_RSP_GPUUID:u32=NCSI_PKT_CMD_GPUUID+0x80; pub const NCSI_PKT_RSP_QPNPR:u32=NCSI_PKT_CMD_QPNPR+0x80; pub const NCSI_PKT_RSP_SNPR:u32=NCSI_PKT_CMD_SNPR+0x80; pub const NCSI_PKT_RSP_GMCMA:u32=NCSI_PKT_CMD_GMCMA+0x80;

pub const NCSI_PKT_RSP_C_COMPLETED:u32=0x0000; pub const NCSI_PKT_RSP_C_FAILED:u32=0x0001; pub const NCSI_PKT_RSP_C_UNAVAILABLE:u32=0x0002; pub const NCSI_PKT_RSP_C_UNSUPPORTED:u32=0x0003; pub const NCSI_PKT_RSP_R_NO_ERROR:u32=0x0000; pub const NCSI_PKT_RSP_R_INTERFACE:u32=0x0001; pub const NCSI_PKT_RSP_R_PARAM:u32=0x0002; pub const NCSI_PKT_RSP_R_CHANNEL:u32=0x0003; pub const NCSI_PKT_RSP_R_PACKAGE:u32=0x0004; pub const NCSI_PKT_RSP_R_LENGTH:u32=0x0005; pub const NCSI_PKT_RSP_R_UNKNOWN:u32=0x7fff;
pub const NCSI_PKT_AEN:u32=0xff; pub const NCSI_PKT_AEN_LSC:u32=0x00; pub const NCSI_PKT_AEN_CR:u32=0x01; pub const NCSI_PKT_AEN_HNCDSC:u32=0x02;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
