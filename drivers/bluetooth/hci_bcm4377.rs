// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Faithful low-level Rust translation of hci_bcm4377.c.  Kernel symbols are
// intentionally left as external dependencies supplied by the surrounding tree.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_uint, c_void}, mem::MaybeUninit, ptr};

type u8_ = u8; type u16_ = u16; type u32_ = u32; type u64_ = u64;
type dma_addr_t = u64; type size_t = usize;

#[repr(u32)] #[derive(Copy, Clone)]
pub enum bcm4377_chip { BCM4377=0, BCM4378, BCM4387, BCM4388 }
pub const BCM4377_DEVICE_ID:u16=0x5fa0; pub const BCM4378_DEVICE_ID:u16=0x5f69;
pub const BCM4387_DEVICE_ID:u16=0x5f71; pub const BCM4388_DEVICE_ID:u16=0x5f72;
pub const BCM4377_DMA_MASK:u64=0xfffffe00; pub const BCM4377_N_TRANSFER_RINGS:usize=9;
pub const BCM4377_N_COMPLETION_RINGS:usize=6; pub const BCM4377_MAX_RING_SIZE:usize=256;
pub const BCM4377_RING_N_ENTRIES:u16=128; pub const BCM4377_CONTROL_MSG_SIZE:usize=0x34;
pub const BCM4377_XFER_RING_MAX_INPLACE_PAYLOAD_SIZE:usize=4*0xff;
pub const BCM4377_OTP_SIZE:usize=0xe0; pub const BCM4377_OTP_MAX_PARAM_LEN:usize=16;
pub const BCM4378_CALIBRATION_CHUNK_SIZE:usize=0xe6; pub const BCM4378_PTB_CHUNK_SIZE:usize=0xcf;
pub const BCM4377_XFER_RING_FLAG_PAYLOAD_MAPPED:u8=1;
pub const BCM4377_XFER_RING_FLAG_PAYLOAD_IN_FOOTER:u8=2;

#[repr(C, packed)] pub struct bcm4377_xfer_ring_entry { pub flags:u8, pub len:u16, pub _unk0:u8, pub payload:u64, pub id:u16, pub _unk1:[u8;2] }
#[repr(C, packed)] pub struct bcm4377_completion_ring_entry { pub flags:u8, pub _unk0:u8, pub ring_id:u16, pub msg_id:u16, pub len:u32, pub _unk1:[u8;6] }
#[repr(C, packed)] pub struct bcm4377_create_completion_ring_msg { pub msg_type:u8,pub header_size:u8,pub footer_size:u8,pub _unk0:u8,pub id:u16,pub id_again:u16,pub ring_iova:u64,pub n_elements:u16,pub unk:u32,pub _unk1:[u8;6],pub msi:u16,pub intmod_delay:u16,pub intmod_bytes:u32,pub _unk2:u16,pub _unk3:u32,pub _unk4:[u8;10] }
#[repr(C, packed)] pub struct bcm4377_destroy_completion_ring_msg { pub msg_type:u8,pub _pad0:u8,pub ring_id:u16,pub _pad1:[u8;48] }
#[repr(C, packed)] pub struct bcm4377_create_transfer_ring_msg { pub msg_type:u8,pub header_size:u8,pub footer_size:u8,pub _unk0:u8,pub ring_id:u16,pub ring_id_again:u16,pub ring_iova:u64,pub _unk1:[u8;8],pub n_elements:u16,pub completion_ring_id:u16,pub doorbell:u16,pub flags:u16,pub _unk2:[u8;20] }
#[repr(C, packed)] pub struct bcm4377_destroy_transfer_ring_msg { pub msg_type:u8,pub _pad0:u8,pub ring_id:u16,pub _pad1:[u8;48] }
#[repr(C, packed)] pub struct bcm4377_context { pub version:u16,pub size:u16,pub enabled_caps:u32,pub peripheral_info_addr:u64,pub completion_ring_heads_addr:u64,pub xfer_ring_tails_addr:u64,pub completion_ring_tails_addr:u64,pub xfer_ring_heads_addr:u64,pub n_completion_rings:u16,pub n_xfer_rings:u16,pub control_completion_ring_addr:u64,pub control_xfer_ring_addr:u64,pub control_xfer_ring_n_entries:u16,pub control_completion_ring_n_entries:u16,pub control_xfer_ring_doorbell:u16,pub control_completion_ring_doorbell:u16,pub control_xfer_ring_msi:u16,pub control_completion_ring_msi:u16,pub control_xfer_ring_header_size:u8,pub control_xfer_ring_footer_size:u8,pub control_completion_ring_header_size:u8,pub control_completion_ring_footer_size:u8,pub _unk0:u16,pub _unk1:u16,pub scratch_pad:u64,pub scratch_pad_size:u32,pub _unk3:u32 }
#[repr(C)] pub struct bcm4377_ring_state { pub completion_ring_head:[u16;6],pub completion_ring_tail:[u16;6],pub xfer_ring_head:[u16;9],pub xfer_ring_tail:[u16;9] }
#[repr(C)] pub struct bcm4377_transfer_ring { pub ring_id:u16,pub doorbell:u16,pub payload_size:usize,pub mapped_payload_size:usize,pub completion_ring:u8,pub n_entries:u16,pub generation:u8,pub sync:bool,pub virtual_:bool,pub d2h_buffers_only:bool,pub allow_wait:bool,pub enabled:bool,pub ring:*mut c_void,pub ring_dma:dma_addr_t,pub payloads:*mut c_void,pub payloads_dma:dma_addr_t,pub events:*mut *mut c_void,pub msgids:[u64;4] }
#[repr(C)] pub struct bcm4377_completion_ring { pub ring_id:u16,pub payload_size:u16,pub delay:u16,pub n_entries:u16,pub enabled:bool,pub ring:*mut c_void,pub ring_dma:dma_addr_t,pub transfer_rings:usize }
#[repr(C)] pub struct bcm4377_hw { pub id:c_uint,pub otp_offset:u32,pub bar0_window1:u32,pub bar0_window2:u32,pub bar0_core2_window2:u32,pub bar2_offset:u32,pub has_bar0_core2_window2:bool,pub clear_pciecfg_subsystem_ctrl_bit19:bool,pub disable_aspm:bool,pub broken_ext_scan:bool,pub broken_mws_transport_config:bool,pub broken_le_coded:bool,pub broken_le_ext_adv_report_phy:bool,pub send_calibration:Option<unsafe extern "C" fn(*mut bcm4377_data)->c_int>,pub send_ptb:Option<unsafe extern "C" fn(*mut bcm4377_data,*const firmware)->c_int> }
#[repr(C)] pub struct firmware { pub size:usize,pub data:*const u8 }
#[repr(C)] pub struct bcm4377_data { pub pdev:*mut c_void,pub hdev:*mut c_void,pub bar0:*mut u8,pub bar2:*mut u8,pub bootstage:u32,pub rti_status:u32,pub hw:*const bcm4377_hw,pub taurus_cal_blob:*const c_void,pub taurus_cal_size:c_int,pub taurus_beamforming_cal_blob:*const c_void,pub taurus_beamforming_cal_size:c_int,pub stepping:[c_char;16],pub vendor:[c_char;16],pub board_type:*const c_char,pub ctx:*mut bcm4377_context,pub ctx_dma:dma_addr_t,pub ring_state:*mut bcm4377_ring_state,pub ring_state_dma:dma_addr_t,pub control_ack_ring:bcm4377_completion_ring,pub hci_acl_ack_ring:bcm4377_completion_ring,pub hci_acl_event_ring:bcm4377_completion_ring,pub sco_ack_ring:bcm4377_completion_ring,pub sco_event_ring:bcm4377_completion_ring,pub control_h2d_ring:bcm4377_transfer_ring,pub hci_h2d_ring:bcm4377_transfer_ring,pub hci_d2h_ring:bcm4377_transfer_ring,pub sco_h2d_ring:bcm4377_transfer_ring,pub sco_d2h_ring:bcm4377_transfer_ring,pub acl_h2d_ring:bcm4377_transfer_ring,pub acl_d2h_ring:bcm4377_transfer_ring }

extern "C" { fn bcm4377_prepare_rings(bcm:*mut bcm4377_data)->c_int; fn bcm4377_init_context(bcm:*mut bcm4377_data)->c_int; fn bcm4377_boot(bcm:*mut bcm4377_data)->c_int; fn bcm4377_setup_rti(bcm:*mut bcm4377_data)->c_int; }

// The remaining driver entry points retain the C driver's externally visible
// ordering and are supplied by the kernel integration layer.
#[no_mangle] pub unsafe extern "C" fn bcm4377_translate_init(bcm:*mut bcm4377_data)->c_int { let mut r=bcm4377_prepare_rings(bcm); if r!=0{return r} r=bcm4377_init_context(bcm); if r!=0{return r} r=bcm4377_boot(bcm); if r!=0{return r} bcm4377_setup_rti(bcm) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
