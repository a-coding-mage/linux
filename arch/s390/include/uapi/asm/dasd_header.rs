/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Rust translation of the DASD userspace interface header. */

pub const DASD_IOCTL_LETTER: u8 = b'D';
pub const DASD_API_VERSION: u32 = 6;

#[repr(C)]
pub struct dasd_information2_t {
    pub devno: u32, pub real_devno: u32, pub schid: u32,
    pub cu_type: u32, pub cu_model: u32, pub dev_type: u32, pub dev_model: u32,
    pub open_count: u32, pub req_queue_len: u32, pub chanq_len: u32,
    pub type_: [u8; 4], pub status: u32, pub label_block: u32, pub FBA_layout: u32,
    pub characteristics_size: u32, pub confdata_size: u32,
    pub characteristics: [u8; 64], pub configuration_data: [u8; 256],
    pub format: u32, pub features: u32,
    pub reserved0: u32, pub reserved1: u32, pub reserved2: u32, pub reserved3: u32,
    pub reserved4: u32, pub reserved5: u32, pub reserved6: u32, pub reserved7: u32,
}

pub const DASD_FORMAT_NONE: u32 = 0;
pub const DASD_FORMAT_LDL: u32 = 1;
pub const DASD_FORMAT_CDL: u32 = 2;
pub const DASD_FEATURE_READONLY: u32 = 0x001;
pub const DASD_FEATURE_USEDIAG: u32 = 0x002;
pub const DASD_FEATURE_INITIAL_ONLINE: u32 = 0x004;
pub const DASD_FEATURE_ERPLOG: u32 = 0x008;
pub const DASD_FEATURE_FAILFAST: u32 = 0x010;
pub const DASD_FEATURE_FAILONSLCK: u32 = 0x020;
pub const DASD_FEATURE_USERAW: u32 = 0x040;
pub const DASD_FEATURE_DISCARD: u32 = 0x080;
pub const DASD_FEATURE_PATH_AUTODISABLE: u32 = 0x100;
pub const DASD_FEATURE_REQUEUEQUIESCE: u32 = 0x200;
pub const DASD_FEATURE_DEFAULT: u32 = DASD_FEATURE_PATH_AUTODISABLE;
pub const DASD_PARTN_BITS: u32 = 2;

#[repr(C)]
pub struct dasd_information_t {
    pub devno: u32, pub real_devno: u32, pub schid: u32,
    pub cu_type: u32, pub cu_model: u32, pub dev_type: u32, pub dev_model: u32,
    pub open_count: u32, pub req_queue_len: u32, pub chanq_len: u32,
    pub type_: [u8; 4], pub status: u32, pub label_block: u32, pub FBA_layout: u32,
    pub characteristics_size: u32, pub confdata_size: u32,
    pub characteristics: [u8; 64], pub configuration_data: [u8; 256],
}

#[repr(C, packed)]
pub struct dasd_rssd_perf_stats_t {
    pub invalid: u8, pub format: u8, pub data_format: u8, pub unit_address: u8,
    pub device_status: u16,
    pub nr_read_normal: u32, pub nr_read_normal_hits: u32, pub nr_write_normal: u32,
    pub nr_write_fast_normal_hits: u32, pub nr_read_seq: u32, pub nr_read_seq_hits: u32,
    pub nr_write_seq: u32, pub nr_write_fast_seq_hits: u32, pub nr_read_cache: u32,
    pub nr_read_cache_hits: u32, pub nr_write_cache: u32, pub nr_write_fast_cache_hits: u32,
    pub nr_inhibit_cache: u32, pub nr_bybass_cache: u32, pub nr_seq_dasd_to_cache: u32,
    pub nr_dasd_to_cache: u32, pub nr_cache_to_dasd: u32, pub nr_delayed_fast_write: u32,
    pub nr_normal_fast_write: u32, pub nr_seq_fast_write: u32, pub nr_cache_miss: u32,
    pub status2: u8, pub nr_quick_write_promotes: u32, pub reserved: u8,
    pub ssid: u16, pub reseved2: [u8; 96],
}

#[repr(C)]
pub struct dasd_profile_info_t {
    pub dasd_io_reqs: u32, pub dasd_io_sects: u32, pub dasd_io_secs: [u32; 32],
    pub dasd_io_times: [u32; 32], pub dasd_io_timps: [u32; 32], pub dasd_io_time1: [u32; 32],
    pub dasd_io_time2: [u32; 32], pub dasd_io_time2ps: [u32; 32], pub dasd_io_time3: [u32; 32],
    pub dasd_io_nr_req: [u32; 32],
}
#[repr(C)] pub struct format_data_t { pub start_unit: u32, pub stop_unit: u32, pub blksize: u32, pub intensity: u32 }
#[repr(C)] pub struct dasd_copypair_swap_data_t { pub primary: [u8; 20], pub secondary: [u8; 20], pub reserved: [u8; 64] }
pub const DASD_FMT_INT_FMT_R0: u32 = 1; pub const DASD_FMT_INT_FMT_HA: u32 = 2;
pub const DASD_FMT_INT_INVAL: u32 = 4; pub const DASD_FMT_INT_COMPAT: u32 = 8;
pub const DASD_FMT_INT_FMT_NOR0: u32 = 16; pub const DASD_FMT_INT_ESE_FULL: u32 = 32;
#[repr(C)] pub struct format_check_t { pub expect: format_data_t, pub result: u32, pub unit: u32, pub rec: u32, pub num_records: u32, pub blksize: u32, pub key_length: u32 }
pub const DASD_FMT_ERR_TOO_FEW_RECORDS: u32 = 1; pub const DASD_FMT_ERR_TOO_MANY_RECORDS: u32 = 2;
pub const DASD_FMT_ERR_BLKSIZE: u32 = 3; pub const DASD_FMT_ERR_RECORD_ID: u32 = 4; pub const DASD_FMT_ERR_KEY_LENGTH: u32 = 5;
#[repr(C, packed)] pub struct attrib_data_t { pub operation: u8, pub reserved: u8, pub nr_cyl: u16, pub reserved2: [u8; 29] }
pub const DASD_NORMAL_CACHE: u32 = 0; pub const DASD_BYPASS_CACHE: u32 = 1; pub const DASD_INHIBIT_LOAD: u32 = 2; pub const DASD_SEQ_ACCESS: u32 = 3; pub const DASD_SEQ_PRESTAGE: u32 = 4; pub const DASD_REC_ACCESS: u32 = 5;
#[repr(C, packed)] pub struct dasd_symmio_parms_t { pub reserved: [u8; 8], pub psf_data: u64, pub rssd_result: u64, pub psf_data_len: i32, pub rssd_result_len: i32 }
#[repr(C, packed)] pub struct dasd_snid_data_path_state { pub group: u8, pub reserve: u8, pub mode: u8, pub res: u8 }
#[repr(C, packed)] pub struct dasd_snid_data { pub path_state: dasd_snid_data_path_state, pub pgid: [u8; 11] }
#[repr(C, packed)] pub struct dasd_snid_ioctl_data { pub data: dasd_snid_data, pub path_mask: u8 }

// Linux _IO/_IOR/_IOW/_IOWR encodings; dependent kernel type sizes are represented locally.
const fn ioctl(dir: u32, nr: u32, size: u32) -> u32 { (dir << 30) | (size << 16) | ((DASD_IOCTL_LETTER as u32) << 8) | nr }
const fn io(nr: u32) -> u32 { ioctl(0, nr, 0) }
const fn ior<T>(nr: u32) -> u32 { ioctl(2, nr, core::mem::size_of::<T>() as u32) }
const fn iow<T>(nr: u32) -> u32 { ioctl(1, nr, core::mem::size_of::<T>() as u32) }
const fn iowr<T>(nr: u32) -> u32 { ioctl(3, nr, core::mem::size_of::<T>() as u32) }
pub const BIODASDDISABLE: u32 = io(0); pub const BIODASDENABLE: u32 = io(1); pub const BIODASDRSRV: u32 = io(2); pub const BIODASDRLSE: u32 = io(3); pub const BIODASDSLCK: u32 = io(4); pub const BIODASDPRRST: u32 = io(5); pub const BIODASDQUIESCE: u32 = io(6); pub const BIODASDRESUME: u32 = io(7); pub const BIODASDABORTIO: u32 = io(240); pub const BIODASDALLOWIO: u32 = io(241);
pub const DASDAPIVER: u32 = ior::<i32>(0); pub const BIODASDINFO: u32 = ior::<dasd_information_t>(1); pub const BIODASDPRRD: u32 = ior::<dasd_profile_info_t>(2); pub const BIODASDINFO2: u32 = ior::<dasd_information2_t>(3); pub const BIODASDPSRD: u32 = ior::<dasd_rssd_perf_stats_t>(4); pub const BIODASDGATTR: u32 = ior::<attrib_data_t>(5);
pub const BIODASDFMT: u32 = iow::<format_data_t>(1); pub const BIODASDSATTR: u32 = iow::<attrib_data_t>(2); pub const BIODASDRAS: u32 = iow::<format_data_t>(3); pub const BIODASDCOPYPAIRSWAP: u32 = iow::<dasd_copypair_swap_data_t>(4);
pub const BIODASDSNID: u32 = iowr::<dasd_snid_ioctl_data>(1); pub const BIODASDCHECKFMT: u32 = iowr::<format_check_t>(2); pub const BIODASDSYMMIO: u32 = iowr::<dasd_symmio_parms_t>(240);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
