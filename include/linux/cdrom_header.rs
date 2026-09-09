/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from <linux/cdrom.h>; C includes and build-time bitfield choices are external dependencies. */

#[repr(C)]
pub struct packet_command {
    pub cmd: [u8; CDROM_PACKET_SIZE as usize],
    pub buffer: *mut u8,
    pub buflen: u32,
    pub stat: i32,
    pub sshdr: *mut scsi_sense_hdr,
    pub data_direction: u8,
    pub quiet: i32,
    pub timeout: i32,
    pub reserved: [*mut core::ffi::c_void; 1],
}

pub const CDDA_OLD: i32 = 0;
pub const CDDA_BPC_SINGLE: i32 = 1;
pub const CDDA_BPC_FULL: i32 = 2;

#[repr(C)]
pub struct cdrom_device_info {
    pub ops: *const cdrom_device_ops,
    pub list: list_head,
    pub disk: *mut gendisk,
    pub handle: *mut core::ffi::c_void,
    pub mask: i32,
    pub speed: i32,
    pub capacity: i32,
    pub options: u32,
    pub mc_flags: u32,
    pub vfs_events: u32,
    pub ioctl_events: u32,
    pub use_count: i32,
    pub name: [i8; 20],
    pub per_device_flags: u8,
    pub cdda_method: i32,
    pub last_sense: u8,
    pub media_written: u8,
    pub mmc3_profile: u16,
    pub mrw_mode_page: i32,
    pub opened_for_data: bool,
    pub last_media_change_ms: i64,
}

#[repr(C)]
pub struct cdrom_device_ops {
    pub open: Option<unsafe extern "C" fn(*mut cdrom_device_info, i32) -> i32>,
    pub release: Option<unsafe extern "C" fn(*mut cdrom_device_info)>,
    pub drive_status: Option<unsafe extern "C" fn(*mut cdrom_device_info, i32) -> i32>,
    pub check_events: Option<unsafe extern "C" fn(*mut cdrom_device_info, u32, i32) -> u32>,
    pub tray_move: Option<unsafe extern "C" fn(*mut cdrom_device_info, i32) -> i32>,
    pub lock_door: Option<unsafe extern "C" fn(*mut cdrom_device_info, i32) -> i32>,
    pub select_speed: Option<unsafe extern "C" fn(*mut cdrom_device_info, u64) -> i32>,
    pub get_last_session: Option<unsafe extern "C" fn(*mut cdrom_device_info, *mut cdrom_multisession) -> i32>,
    pub get_mcn: Option<unsafe extern "C" fn(*mut cdrom_device_info, *mut cdrom_mcn) -> i32>,
    pub reset: Option<unsafe extern "C" fn(*mut cdrom_device_info) -> i32>,
    pub audio_ioctl: Option<unsafe extern "C" fn(*mut cdrom_device_info, u32, *mut core::ffi::c_void) -> i32>,
    pub generic_packet: Option<unsafe extern "C" fn(*mut cdrom_device_info, *mut packet_command) -> i32>,
    pub read_cdda_bpc: Option<unsafe extern "C" fn(*mut cdrom_device_info, *mut core::ffi::c_void, u32, u32, *mut u8) -> i32>,
    pub capability: i32,
}

extern "C" {
    pub fn cdrom_multisession(cdi: *mut cdrom_device_info, info: *mut cdrom_multisession) -> i32;
    pub fn cdrom_read_tocentry(cdi: *mut cdrom_device_info, entry: *mut cdrom_tocentry) -> i32;
    pub fn cdrom_open(cdi: *mut cdrom_device_info, mode: blk_mode_t) -> i32;
    pub fn cdrom_release(cdi: *mut cdrom_device_info);
    pub fn cdrom_ioctl(cdi: *mut cdrom_device_info, bdev: *mut block_device, cmd: u32, arg: u64) -> i32;
    pub fn cdrom_check_events(cdi: *mut cdrom_device_info, clearing: u32) -> u32;
    pub fn cdrom_probe_write_features(cdi: *mut cdrom_device_info);
    pub fn register_cdrom(disk: *mut gendisk, cdi: *mut cdrom_device_info) -> i32;
    pub fn unregister_cdrom(cdi: *mut cdrom_device_info);
    pub fn cdrom_get_last_written(cdi: *mut cdrom_device_info, last_written: *mut i64) -> i32;
    pub fn cdrom_number_of_slots(cdi: *mut cdrom_device_info) -> i32;
    pub fn cdrom_mode_select(cdi: *mut cdrom_device_info, cgc: *mut packet_command) -> i32;
    pub fn cdrom_mode_sense(cdi: *mut cdrom_device_info, cgc: *mut packet_command, page_code: i32, page_control: i32) -> i32;
    pub fn init_cdrom_command(cgc: *mut packet_command, buffer: *mut core::ffi::c_void, len: i32, type_: i32);
    pub fn cdrom_dummy_generic_packet(cdi: *mut cdrom_device_info, cgc: *mut packet_command) -> i32;
}

#[repr(C)]
pub struct tracktype { pub data: i32, pub audio: i32, pub cdi: i32, pub xa: i32, pub error: i64 }
pub const CDROM_MAX_SLOTS: usize = 256;

#[repr(C, packed)]
pub struct cdrom_mechstat_header { pub bitfields: u8, pub curlba: [u8; 3], pub nslots: u8, pub slot_tablelen: u16 }
#[repr(C, packed)]
pub struct cdrom_slot { pub bitfields: u8, pub reserved2: [u8; 3] }
#[repr(C)]
pub struct cdrom_changer_info { pub hdr: cdrom_mechstat_header, pub slots: [cdrom_slot; CDROM_MAX_SLOTS] }

#[repr(i32)]
pub enum mechtype_t { mechtype_caddy = 0, mechtype_tray = 1, mechtype_popup = 2, mechtype_individual_changer = 4, mechtype_cartridge_changer = 5 }

#[repr(C, packed)]
pub struct write_param_page { pub bitfields1: u8, pub page_length: u8, pub bitfields2: u8, pub link_size: u8, pub reserved4: u8, pub bitfields3: u8, pub session_format: u8, pub reserved6: u8, pub packet_size: u32, pub audio_pause: u16, pub mcn: [u8; 16], pub isrc: [u8; 16], pub subhdr0: u8, pub subhdr1: u8, pub subhdr2: u8, pub subhdr3: u8 }

#[repr(C)]
pub struct modesel_head { pub reserved1: u8, pub medium: u8, pub reserved2: u8, pub block_desc_length: u8, pub density: u8, pub number_of_blocks_hi: u8, pub number_of_blocks_med: u8, pub number_of_blocks_lo: u8, pub reserved3: u8, pub block_length_hi: u8, pub block_length_med: u8, pub block_length_lo: u8 }
#[repr(C)]
pub struct rpc_state_t { pub report_key_length: u16, pub reserved1: u8, pub reserved2: u8, pub bitfields: u8, pub region_mask: u8, pub rpc_scheme: u8, pub reserved3: u8 }
#[repr(C)]
pub struct event_header { pub data_len: u16, pub bitfields: u8, pub supp_event_class: u8 }
#[repr(C)]
pub struct media_event_desc { pub bitfields: u8, pub start_slot: u8, pub end_slot: u8 }

extern "C" { pub fn cdrom_get_media_event(cdi: *mut cdrom_device_info, med: *mut media_event_desc) -> i32; }

#[inline]
pub unsafe fn lba_to_msf(mut lba: i32, m: *mut u8, s: *mut u8, f: *mut u8) {
    lba = lba.wrapping_add(CD_MSF_OFFSET);
    lba &= 0xffffff;
    *m = (lba / (CD_SECS * CD_FRAMES)) as u8;
    lba %= CD_SECS * CD_FRAMES;
    *s = (lba / CD_FRAMES) as u8;
    *f = (lba % CD_FRAMES) as u8;
}

#[inline]
pub fn msf_to_lba(m: u8, s: u8, f: u8) -> i32 { (((m as i32 * CD_SECS) + s as i32) * CD_FRAMES + f as i32) - CD_MSF_OFFSET }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
