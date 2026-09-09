/*
 * demux.h
 *
 * Rust translation of the Kernel Digital TV Demux kABI declarations.
 * External kernel/DVB types are referenced as supplied dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* C headers: linux/types.h, linux/list.h, linux/time.h, linux/dvb/dmx.h */

#[cfg(not(target_pointer_width = "16"))]
pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type size_t = usize;

/* External kernel/DVB declarations. */
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

pub type ktime_t = i64;

#[repr(C)]
pub struct dmx_ts_pes {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dmx_ts_pes_opaque {
    _private: [u8; 0],
}

/* DMX_MAX_FILTER_SIZE: Maximum length (in bytes) of a section/PES filter. */
pub const DMX_MAX_FILTER_SIZE: usize = 18;
pub const DMX_MAX_SECTION_SIZE: usize = 4096;
pub const DMX_MAX_SECFEED_SIZE: usize = DMX_MAX_SECTION_SIZE + 188;

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum ts_filter_type {
    TS_PACKET = 1,
    TS_PAYLOAD_ONLY = 2,
    TS_DECODER = 4,
    TS_DEMUX = 8,
}

#[repr(C)]
pub struct dmx_ts_feed {
    pub is_filtering: i32,
    pub parent: *mut dmx_demux,
    pub priv_: *mut ::core::ffi::c_void,
    pub set: Option<unsafe extern "C" fn(*mut dmx_ts_feed, u16, i32, dmx_ts_pes, ktime_t) -> i32>,
    pub start_filtering: Option<unsafe extern "C" fn(*mut dmx_ts_feed) -> i32>,
    pub stop_filtering: Option<unsafe extern "C" fn(*mut dmx_ts_feed) -> i32>,
}

#[repr(C)]
pub struct dmx_section_filter {
    pub filter_value: [u8; DMX_MAX_FILTER_SIZE],
    pub filter_mask: [u8; DMX_MAX_FILTER_SIZE],
    pub filter_mode: [u8; DMX_MAX_FILTER_SIZE],
    pub parent: *mut dmx_section_feed,
    pub priv_: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct dmx_section_feed {
    pub is_filtering: i32,
    pub parent: *mut dmx_demux,
    pub priv_: *mut ::core::ffi::c_void,
    pub check_crc: i32,
    pub crc_val: u32,
    pub secbuf: *mut u8,
    pub secbuf_base: [u8; DMX_MAX_SECFEED_SIZE],
    pub secbufp: u16,
    pub seclen: u16,
    pub tsfeedp: u16,
    pub set: Option<unsafe extern "C" fn(*mut dmx_section_feed, u16, i32) -> i32>,
    pub allocate_filter: Option<unsafe extern "C" fn(*mut dmx_section_feed, *mut *mut dmx_section_filter) -> i32>,
    pub release_filter: Option<unsafe extern "C" fn(*mut dmx_section_feed, *mut dmx_section_filter) -> i32>,
    pub start_filtering: Option<unsafe extern "C" fn(*mut dmx_section_feed) -> i32>,
    pub stop_filtering: Option<unsafe extern "C" fn(*mut dmx_section_feed) -> i32>,
}

pub type dmx_ts_cb = unsafe extern "C" fn(*const u8, size_t, *const u8, size_t, *mut dmx_ts_feed, *mut u32) -> i32;
pub type dmx_section_cb = unsafe extern "C" fn(*const u8, size_t, *const u8, size_t, *mut dmx_section_filter, *mut u32) -> i32;

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum dmx_frontend_source {
    DMX_MEMORY_FE = 0,
    DMX_FRONTEND_0 = 1,
}

#[repr(C)]
pub struct dmx_frontend {
    pub connectivity_list: list_head,
    pub source: dmx_frontend_source,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum dmx_demux_caps {
    DMX_TS_FILTERING = 1,
    DMX_SECTION_FILTERING = 4,
    DMX_MEMORY_BASED_FILTERING = 8,
}

/* DMX_FE_ENTRY(list) casts a list element to struct dmx_frontend. */
#[inline]
pub unsafe fn DMX_FE_ENTRY(list: *mut list_head) -> *mut dmx_frontend {
    (list as *mut u8).sub(core::mem::offset_of!(dmx_frontend, connectivity_list)) as *mut dmx_frontend
}

#[repr(C)]
pub struct dmx_demux {
    pub capabilities: dmx_demux_caps,
    pub frontend: *mut dmx_frontend,
    pub priv_: *mut ::core::ffi::c_void,
    pub open: Option<unsafe extern "C" fn(*mut dmx_demux) -> i32>,
    pub close: Option<unsafe extern "C" fn(*mut dmx_demux) -> i32>,
    pub write: Option<unsafe extern "C" fn(*mut dmx_demux, *const u8, size_t) -> i32>,
    pub allocate_ts_feed: Option<unsafe extern "C" fn(*mut dmx_demux, *mut *mut dmx_ts_feed, dmx_ts_cb) -> i32>,
    pub release_ts_feed: Option<unsafe extern "C" fn(*mut dmx_demux, *mut dmx_ts_feed) -> i32>,
    pub allocate_section_feed: Option<unsafe extern "C" fn(*mut dmx_demux, *mut *mut dmx_section_feed, dmx_section_cb) -> i32>,
    pub release_section_feed: Option<unsafe extern "C" fn(*mut dmx_demux, *mut dmx_section_feed) -> i32>,
    pub add_frontend: Option<unsafe extern "C" fn(*mut dmx_demux, *mut dmx_frontend) -> i32>,
    pub remove_frontend: Option<unsafe extern "C" fn(*mut dmx_demux, *mut dmx_frontend) -> i32>,
    pub get_frontends: Option<unsafe extern "C" fn(*mut dmx_demux) -> *mut list_head>,
    pub connect_frontend: Option<unsafe extern "C" fn(*mut dmx_demux, *mut dmx_frontend) -> i32>,
    pub disconnect_frontend: Option<unsafe extern "C" fn(*mut dmx_demux) -> i32>,
    pub get_pes_pids: Option<unsafe extern "C" fn(*mut dmx_demux, *mut u16) -> i32>,
    pub get_stc: Option<unsafe extern "C" fn(*mut dmx_demux, ::core::ffi::c_uint, *mut u64, *mut ::core::ffi::c_uint) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
