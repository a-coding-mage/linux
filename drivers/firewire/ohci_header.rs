/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from firewire/ohci.h. */

pub type __le32 = u32;

/* OHCI register map */
macro_rules! ohci_consts { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u32 = $v;)* }; }
ohci_consts! {
 OHCI1394_Version=0x000, OHCI1394_GUID_ROM=0x004, OHCI1394_ATRetries=0x008, OHCI1394_CSRData=0x00C,
 OHCI1394_CSRCompareData=0x010, OHCI1394_CSRControl=0x014, OHCI1394_ConfigROMhdr=0x018, OHCI1394_BusID=0x01C,
 OHCI1394_BusOptions=0x020, OHCI1394_GUIDHi=0x024, OHCI1394_GUIDLo=0x028, OHCI1394_ConfigROMmap=0x034,
 OHCI1394_PostedWriteAddressLo=0x038, OHCI1394_PostedWriteAddressHi=0x03C, OHCI1394_VendorID=0x040,
 OHCI1394_HCControlSet=0x050, OHCI1394_HCControlClear=0x054, OHCI1394_HCControl_BIBimageValid=0x80000000,
 OHCI1394_HCControl_noByteSwapData=0x40000000, OHCI1394_HCControl_programPhyEnable=0x00800000,
 OHCI1394_HCControl_aPhyEnhanceEnable=0x00400000, OHCI1394_HCControl_LPS=0x00080000,
 OHCI1394_HCControl_postedWriteEnable=0x00040000, OHCI1394_HCControl_linkEnable=0x00020000,
 OHCI1394_HCControl_softReset=0x00010000, OHCI1394_SelfIDBuffer=0x064, OHCI1394_SelfIDCount=0x068,
 OHCI1394_IRMultiChanMaskHiSet=0x070, OHCI1394_IRMultiChanMaskHiClear=0x074, OHCI1394_IRMultiChanMaskLoSet=0x078,
 OHCI1394_IRMultiChanMaskLoClear=0x07C, OHCI1394_IntEventSet=0x080, OHCI1394_IntEventClear=0x084,
 OHCI1394_IntMaskSet=0x088, OHCI1394_IntMaskClear=0x08C, OHCI1394_IsoXmitIntEventSet=0x090,
 OHCI1394_IsoXmitIntEventClear=0x094, OHCI1394_IsoXmitIntMaskSet=0x098, OHCI1394_IsoXmitIntMaskClear=0x09C,
 OHCI1394_IsoRecvIntEventSet=0x0A0, OHCI1394_IsoRecvIntEventClear=0x0A4, OHCI1394_IsoRecvIntMaskSet=0x0A8,
 OHCI1394_IsoRecvIntMaskClear=0x0AC, OHCI1394_InitialBandwidthAvailable=0x0B0, OHCI1394_InitialChannelsAvailableHi=0x0B4,
 OHCI1394_InitialChannelsAvailableLo=0x0B8, OHCI1394_FairnessControl=0x0DC, OHCI1394_LinkControlSet=0x0E0,
 OHCI1394_LinkControlClear=0x0E4, OHCI1394_LinkControl_rcvSelfID=1<<9, OHCI1394_LinkControl_rcvPhyPkt=1<<10,
 OHCI1394_LinkControl_cycleTimerEnable=1<<20, OHCI1394_LinkControl_cycleMaster=1<<21, OHCI1394_LinkControl_cycleSource=1<<22,
 OHCI1394_NodeID=0x0E8, OHCI1394_NodeID_idValid=0x80000000, OHCI1394_NodeID_root=0x40000000,
 OHCI1394_NodeID_nodeNumber=0x3f, OHCI1394_NodeID_busNumber=0xffc0, OHCI1394_PhyControl=0x0EC,
 OHCI1394_PhyControl_ReadDone=0x80000000, OHCI1394_PhyControl_WritePending=0x00004000, OHCI1394_IsochronousCycleTimer=0x0F0,
 OHCI1394_AsReqFilterHiSet=0x100, OHCI1394_AsReqFilterHiClear=0x104, OHCI1394_AsReqFilterLoSet=0x108, OHCI1394_AsReqFilterLoClear=0x10C,
 OHCI1394_PhyReqFilterHiSet=0x110, OHCI1394_PhyReqFilterHiClear=0x114, OHCI1394_PhyReqFilterLoSet=0x118, OHCI1394_PhyReqFilterLoClear=0x11C, OHCI1394_PhyUpperBound=0x120,
 OHCI1394_AsReqTrContextBase=0x180, OHCI1394_AsReqTrContextControlSet=0x180, OHCI1394_AsReqTrContextControlClear=0x184, OHCI1394_AsReqTrCommandPtr=0x18C,
 OHCI1394_AsRspTrContextBase=0x1A0, OHCI1394_AsRspTrContextControlSet=0x1A0, OHCI1394_AsRspTrContextControlClear=0x1A4, OHCI1394_AsRspTrCommandPtr=0x1AC,
 OHCI1394_AsReqRcvContextBase=0x1C0, OHCI1394_AsReqRcvContextControlSet=0x1C0, OHCI1394_AsReqRcvContextControlClear=0x1C4, OHCI1394_AsReqRcvCommandPtr=0x1CC,
 OHCI1394_AsRspRcvContextBase=0x1E0, OHCI1394_AsRspRcvContextControlSet=0x1E0, OHCI1394_AsRspRcvContextControlClear=0x1E4, OHCI1394_AsRspRcvCommandPtr=0x1EC
}

ohci_consts! {
 OHCI1394_AT_DATA_Q0_srcBusID_MASK=0x00800000, OHCI1394_AT_DATA_Q0_srcBusID_SHIFT=23, OHCI1394_AT_DATA_Q0_spd_MASK=0x00070000, OHCI1394_AT_DATA_Q0_spd_SHIFT=16, OHCI1394_AT_DATA_Q0_tLabel_MASK=0x0000fc00, OHCI1394_AT_DATA_Q0_tLabel_SHIFT=10, OHCI1394_AT_DATA_Q0_rt_MASK=0x00000300, OHCI1394_AT_DATA_Q0_rt_SHIFT=8, OHCI1394_AT_DATA_Q0_tCode_MASK=0xf0, OHCI1394_AT_DATA_Q0_tCode_SHIFT=4, OHCI1394_AT_DATA_Q1_destinationId_MASK=0xffff0000, OHCI1394_AT_DATA_Q1_destinationId_SHIFT=16, OHCI1394_AT_DATA_Q1_destinationOffsetHigh_MASK=0xffff, OHCI1394_AT_DATA_Q1_destinationOffsetHigh_SHIFT=0, OHCI1394_AT_DATA_Q1_rCode_MASK=0xf000, OHCI1394_AT_DATA_Q1_rCode_SHIFT=12,
 OHCI1394_IT_DATA_Q0_spd_MASK=0x00070000, OHCI1394_IT_DATA_Q0_spd_SHIFT=16, OHCI1394_IT_DATA_Q0_tag_MASK=0xc000, OHCI1394_IT_DATA_Q0_tag_SHIFT=14, OHCI1394_IT_DATA_Q0_chanNum_MASK=0x3f00, OHCI1394_IT_DATA_Q0_chanNum_SHIFT=8, OHCI1394_IT_DATA_Q0_tcode_MASK=0xf0, OHCI1394_IT_DATA_Q0_tcode_SHIFT=4, OHCI1394_IT_DATA_Q0_sy_MASK=0xf, OHCI1394_IT_DATA_Q0_sy_SHIFT=0, OHCI1394_IT_DATA_Q1_dataLength_MASK=0xffff0000, OHCI1394_IT_DATA_Q1_dataLength_SHIFT=16,
 OHCI1394_SelfIDCount_selfIDError_MASK=0x80000000, OHCI1394_SelfIDCount_selfIDError_SHIFT=31, OHCI1394_SelfIDCount_selfIDGeneration_MASK=0x00ff0000, OHCI1394_SelfIDCount_selfIDGeneration_SHIFT=16, OHCI1394_SelfIDCount_selfIDSize_MASK=0x7fc, OHCI1394_SelfIDCount_selfIDSize_SHIFT=2, OHCI1394_SELF_ID_RECEIVE_Q0_GENERATION_MASK=0x00ff0000, OHCI1394_SELF_ID_RECEIVE_Q0_GENERATION_SHIFT=16, OHCI1394_SELF_ID_RECEIVE_Q0_TIMESTAMP_MASK=0xffff, OHCI1394_SELF_ID_RECEIVE_Q0_TIMESTAMP_SHIFT=0
}

#[inline] pub unsafe fn ohci1394_at_data_get_src_bus_id(data: *const __le32)->bool { ((*data & OHCI1394_AT_DATA_Q0_srcBusID_MASK)>>23) != 0 }
#[inline] pub unsafe fn ohci1394_at_data_set_src_bus_id(data:*mut __le32, v:bool) { *data = (*data & !OHCI1394_AT_DATA_Q0_srcBusID_MASK) | ((v as u32)<<23 & OHCI1394_AT_DATA_Q0_srcBusID_MASK); }
macro_rules! field { ($get:ident,$set:ident,$mask:ident,$shift:ident,$ty:ty) => { #[inline] pub unsafe fn $get(d:*const __le32)->$ty { ((*d as u32 & $mask)>>$shift) as $ty } #[inline] pub unsafe fn $set(d:*mut __le32,v:$ty) { *d=(*d & !$mask)|(((v as u32)<<$shift)&$mask); } }; }
field!(ohci1394_at_data_get_speed,ohci1394_at_data_set_speed,OHCI1394_AT_DATA_Q0_spd_MASK,OHCI1394_AT_DATA_Q0_spd_SHIFT,u32);
field!(ohci1394_at_data_get_tlabel,ohci1394_at_data_set_tlabel,OHCI1394_AT_DATA_Q0_tLabel_MASK,OHCI1394_AT_DATA_Q0_tLabel_SHIFT,u32);
field!(ohci1394_at_data_get_retry,ohci1394_at_data_set_retry,OHCI1394_AT_DATA_Q0_rt_MASK,OHCI1394_AT_DATA_Q0_rt_SHIFT,u32);
field!(ohci1394_at_data_get_tcode,ohci1394_at_data_set_tcode,OHCI1394_AT_DATA_Q0_tCode_MASK,OHCI1394_AT_DATA_Q0_tCode_SHIFT,u32);
#[inline] pub unsafe fn ohci1394_at_data_get_destination_id(d:*const __le32)->u32 { ((*d.add(1)&0xffff0000)>>16) }
#[inline] pub unsafe fn ohci1394_at_data_set_destination_id(d:*mut __le32,v:u32) { *d.add(1)=(*d.add(1)&!0xffff0000)|(v<<16&0xffff0000); }
#[inline] pub unsafe fn ohci1394_at_data_get_destination_offset(d:*const __le32)->u64 { ((*d.add(1)&0xffff) as u64)<<32 | *d.add(2) as u64 }
#[inline] pub unsafe fn ohci1394_at_data_set_destination_offset(d:*mut __le32,v:u64) { *d.add(1)=(*d.add(1)&!0xffff)|((v>>32) as u32&0xffff); *d.add(2)=v as u32; }
#[inline] pub unsafe fn ohci1394_at_data_get_rcode(d:*const __le32)->u32 { ((*d.add(1)&0xf000)>>12) }
#[inline] pub unsafe fn ohci1394_at_data_set_rcode(d:*mut __le32,v:u32) { *d.add(1)=(*d.add(1)&!0xf000)|(v<<12&0xf000); }

macro_rules! itfield { ($g:ident,$s:ident,$m:ident,$sh:ident) => { field!($g,$s,$m,$sh,u32); }; }
itfield!(ohci1394_it_data_get_speed,ohci1394_it_data_set_speed,OHCI1394_IT_DATA_Q0_spd_MASK,OHCI1394_IT_DATA_Q0_spd_SHIFT);
itfield!(ohci1394_it_data_get_tag,ohci1394_it_data_set_tag,OHCI1394_IT_DATA_Q0_tag_MASK,OHCI1394_IT_DATA_Q0_tag_SHIFT);
itfield!(ohci1394_it_data_get_channel,ohci1394_it_data_set_channel,OHCI1394_IT_DATA_Q0_chanNum_MASK,OHCI1394_IT_DATA_Q0_chanNum_SHIFT);
itfield!(ohci1394_it_data_get_tcode,ohci1394_it_data_set_tcode,OHCI1394_IT_DATA_Q0_tcode_MASK,OHCI1394_IT_DATA_Q0_tcode_SHIFT);
itfield!(ohci1394_it_data_get_sync,ohci1394_it_data_set_sync,OHCI1394_IT_DATA_Q0_sy_MASK,OHCI1394_IT_DATA_Q0_sy_SHIFT);
#[inline] pub unsafe fn ohci1394_it_data_get_data_length(d:*const __le32)->u32 { (*d.add(1)&0xffff0000)>>16 }
#[inline] pub unsafe fn ohci1394_it_data_set_data_length(d:*mut __le32,v:u32) { *d.add(1)=(*d.add(1)&!0xffff0000)|(v<<16&0xffff0000); }

#[inline] pub fn ohci1394_self_id_count_is_error(v:u32)->bool { v&OHCI1394_SelfIDCount_selfIDError_MASK != 0 }
#[inline] pub fn ohci1394_self_id_count_get_generation(v:u32)->u8 { ((v&0x00ff0000)>>16) as u8 }
#[inline] pub fn ohci1394_self_id_count_get_size(v:u32)->u32 { (v&0x7fc)>>2 }
#[inline] pub fn ohci1394_self_id_receive_q0_get_generation(v:u32)->u8 { ((v&0x00ff0000)>>16) as u8 }
#[inline] pub fn ohci1394_self_id_receive_q0_get_timestamp(v:u32)->u16 { v as u16 }

#[inline] pub const fn OHCI1394_PhyControl_Read(addr: u32) -> u32 { (addr << 8) | 0x8000 }
#[inline] pub const fn OHCI1394_PhyControl_ReadData(r: u32) -> u32 { (r & 0x00ff0000) >> 16 }
#[inline] pub const fn OHCI1394_PhyControl_Write(addr: u32, data: u32) -> u32 { (addr << 8) | data | 0x4000 }

macro_rules! ohci_reg_fns { ($($n:ident($b:expr, $s:expr)),* $(,)?) => { $(#[inline] pub const fn $n(n: u32)->u32 { $b + $s*n })* }; }
ohci_reg_fns! { OHCI1394_IsoXmitContextBase(0x200,16), OHCI1394_IsoXmitContextControlSet(0x200,16), OHCI1394_IsoXmitContextControlClear(0x204,16), OHCI1394_IsoXmitCommandPtr(0x20C,16), OHCI1394_IsoRcvContextBase(0x400,32), OHCI1394_IsoRcvContextControlSet(0x400,32), OHCI1394_IsoRcvContextControlClear(0x404,32), OHCI1394_IsoRcvCommandPtr(0x40C,32), OHCI1394_IsoRcvContextMatch(0x410,32) }

ohci_consts! {
 OHCI1394_reqTxComplete=1, OHCI1394_respTxComplete=2, OHCI1394_ARRQ=4, OHCI1394_ARRS=8, OHCI1394_RQPkt=0x10, OHCI1394_RSPkt=0x20, OHCI1394_isochTx=0x40, OHCI1394_isochRx=0x80, OHCI1394_postedWriteErr=0x100, OHCI1394_lockRespErr=0x200, OHCI1394_selfIDComplete=0x10000, OHCI1394_busReset=0x20000, OHCI1394_regAccessFail=0x40000, OHCI1394_phy=0x80000, OHCI1394_cycleSynch=0x100000, OHCI1394_cycle64Seconds=0x200000, OHCI1394_cycleLost=0x400000, OHCI1394_cycleInconsistent=0x800000, OHCI1394_unrecoverableError=0x1000000, OHCI1394_cycleTooLong=0x2000000, OHCI1394_phyRegRcvd=0x4000000, OHCI1394_masterIntEnable=0x80000000,
 OHCI1394_evt_no_status=0, OHCI1394_evt_long_packet=2, OHCI1394_evt_missing_ack=3, OHCI1394_evt_underrun=4, OHCI1394_evt_overrun=5, OHCI1394_evt_descriptor_read=6, OHCI1394_evt_data_read=7, OHCI1394_evt_data_write=8, OHCI1394_evt_bus_reset=9, OHCI1394_evt_timeout=10, OHCI1394_evt_tcode_err=11, OHCI1394_evt_reserved_b=12, OHCI1394_evt_reserved_c=13, OHCI1394_evt_unknown=14, OHCI1394_evt_flushed=15
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
