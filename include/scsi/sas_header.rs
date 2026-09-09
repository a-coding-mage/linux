/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of sas.h. */

pub const SAS_ADDR_SIZE: usize = 8;
pub const HASHED_SAS_ADDR_SIZE: usize = 3;
pub const SMP_REQUEST: u8 = 0x40;
pub const SMP_RESPONSE: u8 = 0x41;
pub const SSP_DATA: u8 = 0x01;
pub const SSP_XFER_RDY: u8 = 0x05;
pub const SSP_COMMAND: u8 = 0x06;
pub const SSP_RESPONSE: u8 = 0x07;
pub const SSP_TASK: u8 = 0x16;

pub const SMP_REPORT_GENERAL: u8 = 0x00;
pub const SMP_REPORT_MANUF_INFO: u8 = 0x01;
pub const SMP_READ_GPIO_REG: u8 = 0x02;
pub const SMP_DISCOVER: u8 = 0x10;
pub const SMP_REPORT_PHY_ERR_LOG: u8 = 0x11;
pub const SMP_REPORT_PHY_SATA: u8 = 0x12;
pub const SMP_REPORT_ROUTE_INFO: u8 = 0x13;
pub const SMP_WRITE_GPIO_REG: u8 = 0x82;
pub const SMP_CONF_ROUTE_INFO: u8 = 0x90;
pub const SMP_PHY_CONTROL: u8 = 0x91;
pub const SMP_PHY_TEST_FUNCTION: u8 = 0x92;

pub const SMP_RESP_FUNC_ACC: u8 = 0x00;
pub const SMP_RESP_FUNC_UNK: u8 = 0x01;
pub const SMP_RESP_FUNC_FAILED: u8 = 0x02;
pub const SMP_RESP_INV_FRM_LEN: u8 = 0x03;
pub const SMP_RESP_NO_PHY: u8 = 0x10;
pub const SMP_RESP_NO_INDEX: u8 = 0x11;
pub const SMP_RESP_PHY_NO_SATA: u8 = 0x12;
pub const SMP_RESP_PHY_UNK_OP: u8 = 0x13;
pub const SMP_RESP_PHY_UNK_TESTF: u8 = 0x14;
pub const SMP_RESP_PHY_TEST_INPROG: u8 = 0x15;
pub const SMP_RESP_PHY_VACANT: u8 = 0x16;

pub const TMF_ABORT_TASK: u8 = 0x01;
pub const TMF_ABORT_TASK_SET: u8 = 0x02;
pub const TMF_CLEAR_TASK_SET: u8 = 0x04;
pub const TMF_LU_RESET: u8 = 0x08;
pub const TMF_CLEAR_ACA: u8 = 0x40;
pub const TMF_QUERY_TASK: u8 = 0x80;
pub const TMF_RESP_FUNC_COMPLETE: u8 = 0x00;
pub const TMF_RESP_INVALID_FRAME: u8 = 0x02;
pub const TMF_RESP_FUNC_ESUPP: u8 = 0x04;
pub const TMF_RESP_FUNC_FAILED: u8 = 0x05;
pub const TMF_RESP_FUNC_SUCC: u8 = 0x08;
pub const TMF_RESP_NO_LUN: u8 = 0x09;
pub const TMF_RESP_OVERLAPPED_TAG: u8 = 0x0a;

#[repr(u32)] pub enum SasOobMode { OobNotConnected, SataOobMode, SasOobMode }
#[repr(u32)] pub enum SasDeviceType { SasPhyUnused=0, SasEndDevice=1, SasEdgeExpanderDevice=2, SasFanoutExpanderDevice=3, SasHa=4, SasSataDev=5, SasSataPm=7, SasSataPmPort=8, SasSataPending=9 }
#[repr(u32)] pub enum SasProtocol { SasProtocolNone=0, SasProtocolSata=1, SasProtocolSmp=2, SasProtocolStp=4, SasProtocolSsp=8, SasProtocolAll=0x0e, SasProtocolStpAll=5, SasProtocolInternalAbort=0x10 }
#[repr(u32)] pub enum PhyFunc { PhyFuncNop, PhyFuncLinkReset, PhyFuncHardReset, PhyFuncDisable, PhyFuncClearErrorLog=5, PhyFuncClearAffil, PhyFuncTxSataPsSignal, PhyFuncReleaseSpinupHold=0x10, PhyFuncSetLinkRate, PhyFuncGetEvents }
#[repr(u32)] pub enum SasGpioRegType { SasGpioRegCfg=0, SasGpioRegRx=1, SasGpioRegRxGp=2, SasGpioRegTx=3, SasGpioRegTxGp=4 }
pub const SAS_DATAPRES_NO_DATA: u8=0; pub const SAS_DATAPRES_RESPONSE_DATA: u8=1; pub const SAS_DATAPRES_SENSE_DATA: u8=2;
#[repr(u32)] pub enum SasPrim { SasPrimAipNormal=1,SasPrimAipR0,SasPrimAipR1,SasPrimAipR2,SasPrimAipWc,SasPrimAipWd,SasPrimAipWp,SasPrimAipRwp,SasPrimBcCh,SasPrimBcRch0,SasPrimBcRch1,SasPrimBcR0,SasPrimBcR1,SasPrimBcR2,SasPrimBcR3,SasPrimBcR4,SasPrimNotifyEnsp,SasPrimNotifyR0,SasPrimNotifyR1,SasPrimNotifyR2,SasPrimCloseClaf,SasPrimCloseNorm,SasPrimCloseR0,SasPrimCloseR1,SasPrimOpenRtry,SasPrimOpenRjct,SasPrimOpenAcpt,SasPrimDone,SasPrimBreak,SataPrimDmat=33,SataPrimPmnak,SataPrimPmack,SataPrimPmreqS,SataPrimPmreqP,SataSataRErr }
#[repr(u32)] pub enum SasOpenRejReason { SasOrejUnknown=0,SasOrejBadDest,SasOrejConnRate,SasOrejEproto,SasOrejResvAb0,SasOrejResvAb1,SasOrejResvAb2,SasOrejResvAb3,SasOrejWrongDest,SasOrejStpNores,SasOrejNoDest,SasOrejPathBlocked,SasOrejRsvdCont0,SasOrejRsvdCont1,SasOrejRsvdInit0,SasOrejRsvdInit1,SasOrejRsvdStop0,SasOrejRsvdStop1,SasOrejRsvdRetry }

pub type Be16=u16; pub type Be32=u32; pub type Be64=u64;
#[inline] pub unsafe fn sas_addr(sa: *const u8) -> u64 { u64::from_be((*(sa as *const Be64))) }
#[repr(C, packed)] pub struct DevToHostFis { pub fis_type:u8,pub flags:u8,pub status:u8,pub error:u8,pub lbal:u8,pub lbam:u8,pub lbah:u8,pub device:u8,pub lbal_exp:u8,pub lbam_exp:u8,pub lbah_exp:u8,pub _r_a:u8,pub sector_count:u8,pub sector_count_exp:u8,pub _r_b:u8,pub _r_c:u8,pub _r_d:u32 }
#[repr(C, packed)] pub struct HostToDevFis { pub fis_type:u8,pub flags:u8,pub command:u8,pub features:u8,pub lbal:u8,pub lbam:u8,pub lbah:u8,pub device:u8,pub lbal_exp:u8,pub lbam_exp:u8,pub lbah_exp:u8,pub features_exp:u8,pub sector_count:u8,pub sector_count_exp:u8,pub _r_a:u8,pub control:u8,pub _r_b:u32 }

#[repr(C, packed)] pub struct SasIdentifyFrame { pub frame_type:u8,pub _un1:u8,pub initiator_bits:u8,pub target_bits:u8,pub _un4_11:[u8;8],pub sas_addr:[u8;SAS_ADDR_SIZE],pub phy_id:u8,pub _un21_27:[u8;7],pub crc:Be32 }
#[repr(C, packed)] pub struct SspFrameHdr { pub frame_type:u8,pub hashed_dest_addr:[u8;3],pub _r_a:u8,pub hashed_src_addr:[u8;3],pub _r_b:Be16,pub flags:u8,pub fill:u8,pub _r_e:u32,pub tag:Be16,pub tptt:Be16,pub data_offs:Be32 }
#[repr(C, packed)] pub struct SspResponseIe { pub _r_a:[u8;10],pub datapres:u8,pub status:u8,pub _r_c:u32,pub sense_data_len:Be32,pub response_data_len:Be32,pub resp_data:[u8;0] }
#[repr(C, packed)] pub struct SspCommandIu { pub lun:[u8;8],pub _r_a:u8,pub efb_prio_attr:u8,pub _r_b:u8,pub add_cdb_len:u8,pub cdb:[u8;16],pub add_cdb:[u8;0] }
#[repr(C, packed)] pub struct XferRdyIu { pub requested_offset:Be32,pub write_data_len:Be32,pub _r_a:Be32 }
#[repr(C, packed)] pub struct SspTmfIu { pub lun:[u8;8],pub _r_a:u16,pub tmf:u8,pub _r_b:u8,pub tag:Be16,pub _r_c:[u8;14] }

#[repr(C, packed)] pub struct ReportGeneralResp { pub change_count:Be16,pub route_indexes:Be16,pub _r_a:u8,pub num_phys:u8,pub flags:u8,pub _r_c:u8,pub enclosure_logical_id:[u8;8],pub _r_d:[u8;12] }
#[repr(C, packed)] pub struct DiscoverResp { pub _r_a:[u8;5],pub phy_id:u8,pub _r_b:Be16,pub dev_type:u8,pub linkrate:u8,pub iproto:u8,pub tproto:u8,pub sas_addr:[u8;8],pub attached_sas_addr:[u8;8],pub attached_phy_id:u8,pub _r_h:[u8;7],pub min_linkrate:u8,pub max_linkrate:u8,pub change_count:u8,pub pptv_virtual:u8,pub routing_attr:u8,pub conn_type:u8,pub conn_el_index:u8,pub conn_phy_link:u8,pub _r_k:[u8;8] }
#[repr(C, packed)] pub struct ReportPhySataResp { pub _r_a:[u8;5],pub phy_id:u8,pub _r_b:u8,pub affil:u8,pub _r_d:u32,pub stp_sas_addr:[u8;8],pub fis:DevToHostFis,pub _r_e:u32,pub affil_stp_ini_addr:[u8;8],pub crc:Be32 }
#[repr(C, packed)] pub struct SmpRgResp { pub frame_type:u8,pub function:u8,pub result:u8,pub reserved:u8,pub rg:ReportGeneralResp }
#[repr(C, packed)] pub struct SmpDiscResp { pub frame_type:u8,pub function:u8,pub result:u8,pub reserved:u8,pub disc:DiscoverResp }
#[repr(C, packed)] pub struct SmpRpsResp { pub frame_type:u8,pub function:u8,pub result:u8,pub reserved:u8,pub rps:ReportPhySataResp }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
