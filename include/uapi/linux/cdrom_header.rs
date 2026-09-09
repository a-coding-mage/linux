/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of linux/cdrom.h. */

// External types supplied by the corresponding Linux headers.
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s64 = i64;
pub type __be16 = u16;
pub type __be32 = u32;

pub const EDRIVE_CANT_DO_THIS: i32 = EOPNOTSUPP;

macro_rules! c { ($($n:ident = $v:expr),* $(,)?) => { $(pub const $n: u32 = $v;) * }; }
c!(
 CDROMPAUSE=0x5301, CDROMRESUME=0x5302, CDROMPLAYMSF=0x5303, CDROMPLAYTRKIND=0x5304,
 CDROMREADTOCHDR=0x5305, CDROMREADTOCENTRY=0x5306, CDROMSTOP=0x5307, CDROMSTART=0x5308,
 CDROMEJECT=0x5309, CDROMVOLCTRL=0x530a, CDROMSUBCHNL=0x530b, CDROMREADMODE2=0x530c,
 CDROMREADMODE1=0x530d, CDROMREADAUDIO=0x530e, CDROMEJECT_SW=0x530f, CDROMMULTISESSION=0x5310,
 CDROM_GET_MCN=0x5311, CDROMRESET=0x5312, CDROMVOLREAD=0x5313, CDROMREADRAW=0x5314,
 CDROMREADCOOKED=0x5315, CDROMSEEK=0x5316, CDROMPLAYBLK=0x5317, CDROMREADALL=0x5318,
 CDROMCLOSETRAY=0x5319, CDROMGETSPINDOWN=0x531d, CDROMSETSPINDOWN=0x531e,
 CDROM_SET_OPTIONS=0x5320, CDROM_CLEAR_OPTIONS=0x5321, CDROM_SELECT_SPEED=0x5322,
 CDROM_SELECT_DISC=0x5323, CDROM_MEDIA_CHANGED=0x5325, CDROM_DRIVE_STATUS=0x5326,
 CDROM_DISC_STATUS=0x5327, CDROM_CHANGER_NSLOTS=0x5328, CDROM_LOCKDOOR=0x5329,
 CDROM_DEBUG=0x5330, CDROM_GET_CAPABILITY=0x5331, CDROMAUDIOBUFSIZ=0x5382,
 DVD_READ_STRUCT=0x5390, DVD_WRITE_STRUCT=0x5391, DVD_AUTH=0x5392, CDROM_SEND_PACKET=0x5393,
 CDROM_NEXT_WRITABLE=0x5394, CDROM_LAST_WRITTEN=0x5395, CDROM_TIMED_MEDIA_CHANGE=0x5396
);
pub const CDROM_GET_UPC: u32 = CDROM_GET_MCN;

#[repr(C)] #[derive(Copy, Clone)] pub struct cdrom_msf0 { pub minute: __u8, pub second: __u8, pub frame: __u8 }
#[repr(C)] pub union cdrom_addr { pub msf: cdrom_msf0, pub lba: i32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cdrom_msf { pub cdmsf_min0:__u8,pub cdmsf_sec0:__u8,pub cdmsf_frame0:__u8,pub cdmsf_min1:__u8,pub cdmsf_sec1:__u8,pub cdmsf_frame1:__u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cdrom_ti { pub cdti_trk0:__u8,pub cdti_ind0:__u8,pub cdti_trk1:__u8,pub cdti_ind1:__u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cdrom_tochdr { pub cdth_trk0:__u8,pub cdth_trk1:__u8 }
#[repr(C)] #[derive(Copy, Clone)] pub struct cdrom_volctrl { pub channel0:__u8,pub channel1:__u8,pub channel2:__u8,pub channel3:__u8 }
#[repr(C)] pub struct cdrom_subchnl { pub cdsc_format:__u8,pub cdsc_audiostatus:__u8,pub cdsc_adr_ctrl:__u8,pub cdsc_trk:__u8,pub cdsc_ind:__u8,pub cdsc_absaddr:cdrom_addr,pub cdsc_reladdr:cdrom_addr }
#[repr(C)] pub struct cdrom_tocentry { pub cdte_track:__u8,pub cdte_adr_ctrl:__u8,pub cdte_format:__u8,pub cdte_addr:cdrom_addr,pub cdte_datamode:__u8 }
#[repr(C)] pub struct cdrom_read { pub cdread_lba:i32,pub cdread_bufaddr:*mut i8,pub cdread_buflen:i32 }
#[repr(C)] pub struct cdrom_read_audio { pub addr:cdrom_addr,pub addr_format:__u8,pub nframes:i32,pub buf:*mut __u8 }
#[repr(C)] pub struct cdrom_multisession { pub addr:cdrom_addr,pub xa_flag:__u8,pub addr_format:__u8 }
#[repr(C)] #[derive(Copy,Clone)] pub struct cdrom_mcn { pub medium_catalog_number:[__u8;14] }
#[repr(C)] #[derive(Copy,Clone)] pub struct cdrom_blk { pub from:u32,pub len:u16 }

pub const CDROM_PACKET_SIZE:u32=12; pub const CGC_DATA_UNKNOWN:u32=0; pub const CGC_DATA_WRITE:u32=1; pub const CGC_DATA_READ:u32=2; pub const CGC_DATA_NONE:u32=3;
#[repr(C)] pub union cdrom_generic_reserved { pub reserved:[*mut core::ffi::c_void;1], pub unused:*mut core::ffi::c_void }
#[repr(C)] pub struct cdrom_generic_command { pub cmd:[u8;12],pub buffer:*mut u8,pub buflen:u32,pub stat:i32,pub sense:*mut request_sense,pub data_direction:u8,pub quiet:i32,pub timeout:i32,pub reserved:cdrom_generic_reserved }
#[repr(C)] #[derive(Copy,Clone)] pub struct cdrom_timed_media_change_info { pub last_media_change:__s64,pub media_flags:__u64 }
pub const MEDIA_CHANGED_FLAG:u64=0x1;

pub const CD_MINS:u32=74; pub const CD_SECS:u32=60; pub const CD_FRAMES:u32=75; pub const CD_SYNC_SIZE:u32=12; pub const CD_MSF_OFFSET:u32=150; pub const CD_CHUNK_SIZE:u32=24; pub const CD_NUM_OF_CHUNKS:u32=98; pub const CD_FRAMESIZE_SUB:u32=96; pub const CD_HEAD_SIZE:u32=4; pub const CD_SUBHEAD_SIZE:u32=8; pub const CD_EDC_SIZE:u32=4; pub const CD_ZERO_SIZE:u32=8; pub const CD_ECC_SIZE:u32=276; pub const CD_FRAMESIZE:u32=2048; pub const CD_FRAMESIZE_RAW:u32=2352; pub const CD_FRAMESIZE_RAWER:u32=2646; pub const CD_FRAMESIZE_RAW1:u32=CD_FRAMESIZE_RAW-CD_SYNC_SIZE; pub const CD_FRAMESIZE_RAW0:u32=CD_FRAMESIZE_RAW-CD_SYNC_SIZE-CD_HEAD_SIZE; pub const CD_XA_HEAD:u32=CD_HEAD_SIZE+CD_SUBHEAD_SIZE; pub const CD_XA_TAIL:u32=CD_EDC_SIZE+CD_ECC_SIZE; pub const CD_XA_SYNC_HEAD:u32=CD_SYNC_SIZE+CD_XA_HEAD;
pub const CDROM_LBA:u32=1; pub const CDROM_MSF:u32=2; pub const CDROM_DATA_TRACK:u32=4; pub const CDROM_LEADOUT:u32=0xaa;
pub const CDROM_AUDIO_INVALID:u32=0; pub const CDROM_AUDIO_PLAY:u32=0x11; pub const CDROM_AUDIO_PAUSED:u32=0x12; pub const CDROM_AUDIO_COMPLETED:u32=0x13; pub const CDROM_AUDIO_ERROR:u32=0x14; pub const CDROM_AUDIO_NO_STATUS:u32=0x15;
pub const CDC_CLOSE_TRAY:u32=1; pub const CDC_OPEN_TRAY:u32=2; pub const CDC_LOCK:u32=4; pub const CDC_SELECT_SPEED:u32=8; pub const CDC_SELECT_DISC:u32=0x10; pub const CDC_MULTI_SESSION:u32=0x20; pub const CDC_MCN:u32=0x40; pub const CDC_MEDIA_CHANGED:u32=0x80; pub const CDC_PLAY_AUDIO:u32=0x100; pub const CDC_RESET:u32=0x200; pub const CDC_DRIVE_STATUS:u32=0x800; pub const CDC_GENERIC_PACKET:u32=0x1000; pub const CDC_CD_R:u32=0x2000; pub const CDC_CD_RW:u32=0x4000; pub const CDC_DVD:u32=0x8000; pub const CDC_DVD_R:u32=0x10000; pub const CDC_DVD_RAM:u32=0x20000; pub const CDC_MO_DRIVE:u32=0x40000; pub const CDC_MRW:u32=0x80000; pub const CDC_MRW_W:u32=0x100000; pub const CDC_RAM:u32=0x200000;
pub const CDS_NO_INFO:u32=0; pub const CDS_NO_DISC:u32=1; pub const CDS_TRAY_OPEN:u32=2; pub const CDS_DRIVE_NOT_READY:u32=3; pub const CDS_DISC_OK:u32=4; pub const CDS_AUDIO:u32=100; pub const CDS_DATA_1:u32=101; pub const CDS_DATA_2:u32=102; pub const CDS_XA_2_1:u32=103; pub const CDS_XA_2_2:u32=104; pub const CDS_MIXED:u32=105;
pub const CDO_AUTO_CLOSE:u32=1; pub const CDO_AUTO_EJECT:u32=2; pub const CDO_USE_FFLAGS:u32=4; pub const CDO_LOCK:u32=8; pub const CDO_CHECK_TYPE:u32=0x10; pub const CDSL_NONE:i32=i32::MAX-1; pub const CDSL_CURRENT:i32=i32::MAX; pub const CD_PART_MAX:u32=64; pub const CD_PART_MASK:u32=CD_PART_MAX-1;

// Generic packet command and mode-page constants.
macro_rules! g { ($($n:ident=$v:expr),* $(,)?) => { $(pub const $n:u32=$v;)* }; }
g!(GPCMD_BLANK=0xa1,GPCMD_CLOSE_TRACK=0x5b,GPCMD_FLUSH_CACHE=0x35,GPCMD_FORMAT_UNIT=4,GPCMD_GET_CONFIGURATION=0x46,GPCMD_GET_EVENT_STATUS_NOTIFICATION=0x4a,GPCMD_GET_PERFORMANCE=0xac,GPCMD_INQUIRY=0x12,GPCMD_LOAD_UNLOAD=0xa6,GPCMD_MECHANISM_STATUS=0xbd,GPCMD_MODE_SELECT_10=0x55,GPCMD_MODE_SENSE_10=0x5a,GPCMD_PAUSE_RESUME=0x4b,GPCMD_PLAY_AUDIO_10=0x45,GPCMD_PLAY_AUDIO_MSF=0x47,GPCMD_PLAY_AUDIO_TI=0x48,GPCMD_PLAY_CD=0xbc,GPCMD_PREVENT_ALLOW_MEDIUM_REMOVAL=0x1e,GPCMD_READ_10=0x28,GPCMD_READ_12=0xa8,GPCMD_READ_BUFFER=0x3c,GPCMD_READ_BUFFER_CAPACITY=0x5c,GPCMD_READ_CDVD_CAPACITY=0x25,GPCMD_READ_CD=0xbe,GPCMD_READ_CD_MSF=0xb9,GPCMD_READ_DISC_INFO=0x51,GPCMD_READ_DVD_STRUCTURE=0xad,GPCMD_READ_FORMAT_CAPACITIES=0x23,GPCMD_READ_HEADER=0x44,GPCMD_READ_TRACK_RZONE_INFO=0x52,GPCMD_READ_SUBCHANNEL=0x42,GPCMD_READ_TOC_PMA_ATIP=0x43,GPCMD_REPAIR_RZONE_TRACK=0x58,GPCMD_REPORT_KEY=0xa4,GPCMD_REQUEST_SENSE=3,GPCMD_RESERVE_RZONE_TRACK=0x53,GPCMD_SEND_CUE_SHEET=0x5d,GPCMD_SCAN=0xba,GPCMD_SEEK=0x2b,GPCMD_SEND_DVD_STRUCTURE=0xbf,GPCMD_SEND_EVENT=0xa2,GPCMD_SEND_KEY=0xa3,GPCMD_SEND_OPC=0x54,GPCMD_SET_READ_AHEAD=0xa7,GPCMD_SET_STREAMING=0xb6,GPCMD_START_STOP_UNIT=0x1b,GPCMD_STOP_PLAY_SCAN=0x4e,GPCMD_TEST_UNIT_READY=0,GPCMD_VERIFY_10=0x2f,GPCMD_WRITE_10=0x2a,GPCMD_WRITE_12=0xaa,GPCMD_WRITE_AND_VERIFY_10=0x2e,GPCMD_WRITE_BUFFER=0x3b,GPCMD_SET_SPEED=0xbb,GPCMD_PLAYAUDIO_TI=0x48,GPCMD_GET_MEDIA_STATUS=0xda,GPMODE_VENDOR_PAGE=0,GPMODE_R_W_ERROR_PAGE=1,GPMODE_WRITE_PARMS_PAGE=5,GPMODE_WCACHING_PAGE=8,GPMODE_AUDIO_CTL_PAGE=0x0e,GPMODE_POWER_PAGE=0x1a,GPMODE_FAULT_FAIL_PAGE=0x1c,GPMODE_TO_PROTECT_PAGE=0x1d,GPMODE_CAPABILITIES_PAGE=0x2a,GPMODE_ALL_PAGES=0x3f,GPMODE_CDROM_PAGE=0x0d);

pub const DVD_STRUCT_PHYSICAL:u32=0; pub const DVD_STRUCT_COPYRIGHT:u32=1; pub const DVD_STRUCT_DISCKEY:u32=2; pub const DVD_STRUCT_BCA:u32=3; pub const DVD_STRUCT_MANUFACT:u32=4;
#[repr(C)] #[derive(Copy,Clone)] pub struct dvd_layer { pub book_version:u8,pub min_rate:u8,pub layer_type:u8,pub track_path:u8,pub track_density:u8,pub linear_density:u8,pub bca:u8,pub start_sector:__u32,pub end_sector:__u32,pub end_sector_l0:__u32 }
pub const DVD_LAYERS:usize=4;
#[repr(C)] pub struct dvd_physical { pub type_:u8,pub layer_num:u8,pub layer:[dvd_layer;4] }
#[repr(C)] pub struct dvd_copyright { pub type_:u8,pub layer_num:u8,pub cpst:u8,pub rmi:u8 }
#[repr(C)] pub struct dvd_disckey { pub type_:u8,pub agid:u8,pub value:[u8;2048] }
#[repr(C)] pub struct dvd_bca { pub type_:u8,pub len:i32,pub value:[u8;188] }
#[repr(C)] pub struct dvd_manufact { pub type_:u8,pub layer_num:u8,pub len:i32,pub value:[u8;2048] }
#[repr(C)] pub union dvd_struct { pub type_:u8,pub physical:dvd_physical,pub copyright:dvd_copyright,pub disckey:dvd_disckey,pub bca:dvd_bca,pub manufact:dvd_manufact }

pub const DVD_LU_SEND_AGID:u32=0; pub const DVD_HOST_SEND_CHALLENGE:u32=1; pub const DVD_LU_SEND_KEY1:u32=2; pub const DVD_LU_SEND_CHALLENGE:u32=3; pub const DVD_HOST_SEND_KEY2:u32=4; pub const DVD_AUTH_ESTABLISHED:u32=5; pub const DVD_AUTH_FAILURE:u32=6; pub const DVD_LU_SEND_TITLE_KEY:u32=7; pub const DVD_LU_SEND_ASF:u32=8; pub const DVD_INVALIDATE_AGID:u32=9; pub const DVD_LU_SEND_RPC_STATE:u32=10; pub const DVD_HOST_SEND_RPC_STATE:u32=11;
pub type dvd_key=[u8;5]; pub type dvd_challenge=[u8;10];
#[repr(C)] pub struct dvd_lu_send_agid { pub type_:u8,pub agid:u8 } #[repr(C)] pub struct dvd_host_send_challenge { pub type_:u8,pub agid:u8,pub chal:dvd_challenge } #[repr(C)] pub struct dvd_send_key { pub type_:u8,pub agid:u8,pub key:dvd_key } #[repr(C)] pub struct dvd_lu_send_challenge { pub type_:u8,pub agid:u8,pub chal:dvd_challenge }
pub const DVD_CPM_NO_COPYRIGHT:u32=0; pub const DVD_CPM_COPYRIGHTED:u32=1; pub const DVD_CP_SEC_NONE:u32=0; pub const DVD_CP_SEC_EXIST:u32=1; pub const DVD_CGMS_UNRESTRICTED:u32=0; pub const DVD_CGMS_SINGLE:u32=2; pub const DVD_CGMS_RESTRICTED:u32=3;
#[repr(C)] pub struct dvd_lu_send_title_key { pub type_:u8,pub agid:u8,pub title_key:dvd_key,pub lba:i32,pub cpm:u8,pub cp_sec:u8,pub cgms:u8 }
#[repr(C)] pub struct dvd_lu_send_asf { pub type_:u8,pub agid:u8,pub asf:u8 } #[repr(C)] pub struct dvd_host_send_rpcstate { pub type_:u8,pub pdrc:u8 } #[repr(C)] pub struct dvd_lu_send_rpcstate { pub type_:u8,pub region_mask:u8,pub rpc_scheme:u8 }
#[repr(C)] pub union dvd_authinfo { pub type_:u8,pub lsa:dvd_lu_send_agid,pub hsc:dvd_host_send_challenge,pub lsk:dvd_send_key,pub lsc:dvd_lu_send_challenge,pub hsk:dvd_send_key,pub lstk:dvd_lu_send_title_key,pub lsasf:dvd_lu_send_asf,pub hrpcs:dvd_host_send_rpcstate,pub lrpcs:dvd_lu_send_rpcstate }

#[repr(C)] pub struct request_sense { pub valid_error_code:u8,pub segment_number:u8,pub sense_flags:u8,pub information:[u8;4],pub add_sense_len:u8,pub command_info:[u8;4],pub asc:u8,pub ascq:u8,pub fruc:u8,pub sks:[u8;3],pub asb:[u8;46] }
pub const CDF_RWRT:u32=0x20; pub const CDF_HWDM:u32=0x24; pub const CDF_MRW:u32=0x28; pub const CDM_MRW_NOTMRW:u32=0; pub const CDM_MRW_BGFORMAT_INACTIVE:u32=1; pub const CDM_MRW_BGFORMAT_ACTIVE:u32=2; pub const CDM_MRW_BGFORMAT_COMPLETE:u32=3; pub const MRW_LBA_DMA:u32=0; pub const MRW_LBA_GAA:u32=1; pub const MRW_MODE_PC_PRE1:u32=0x2c; pub const MRW_MODE_PC:u32=3;
#[repr(C)] pub struct mrw_feature_desc { pub feature_code:__be16,pub feature_flags:u8,pub add_len:u8,pub write_flags:u8,pub reserved3:u8,pub reserved4:u8,pub reserved5:u8 }
#[repr(C)] pub struct rwrt_feature_desc { pub feature_code:__be16,pub feature_flags:u8,pub add_len:u8,pub last_lba:__u32,pub block_size:__u32,pub blocking:__u16,pub page_flags:u8,pub reserved3:u8 }
#[repr(C)] pub struct disc_information { pub disc_information_length:__be16,pub status_flags:u8,pub n_first_track:u8,pub n_sessions_lsb:u8,pub first_track_lsb:u8,pub last_track_lsb:u8,pub mrw_flags:u8,pub disc_type:u8,pub n_sessions_msb:u8,pub first_track_msb:u8,pub last_track_msb:u8,pub disc_id:__u32,pub lead_in:__u32,pub lead_out:__u32,pub disc_bar_code:[u8;8],pub reserved3:u8,pub n_opc:u8 }
#[repr(C)] pub struct track_information { pub track_information_length:__be16,pub track_lsb:u8,pub session_lsb:u8,pub reserved1:u8,pub track_flags:u8,pub data_flags:u8,pub address_flags:u8,pub track_start:__be32,pub next_writable:__be32,pub free_blocks:__be32,pub fixed_packet_size:__be32,pub track_size:__be32,pub last_rec_address:__be32 }
#[repr(C)] pub struct feature_header { pub data_len:__u32,pub reserved1:u8,pub reserved2:u8,pub curr_profile:__u16 }
#[repr(C)] pub struct mode_page_header { pub mode_data_length:__be16,pub medium_type:u8,pub reserved1:u8,pub reserved2:u8,pub reserved3:u8,pub desc_length:__be16 }
#[repr(C)] pub struct rm_feature_desc { pub feature_code:__be16,pub feature_flags:u8,pub add_len:u8,pub mechanism_flags:u8,pub reserved2:u8,pub reserved3:u8,pub reserved4:u8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
