/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel/Rust translation.

pub const CEPH_OPT_FSID: i32 = 1 << 0;
pub const CEPH_OPT_NOSHARE: i32 = 1 << 1;
pub const CEPH_OPT_MYIP: i32 = 1 << 2;
pub const CEPH_OPT_NOCRC: i32 = 1 << 3;
pub const CEPH_OPT_TCP_NODELAY: i32 = 1 << 4;
pub const CEPH_OPT_NOMSGSIGN: i32 = 1 << 5;
pub const CEPH_OPT_ABORT_ON_FULL: i32 = 1 << 6;
pub const CEPH_OPT_RXBOUNCE: i32 = 1 << 7;
pub const CEPH_OPT_DEFAULT: i32 = CEPH_OPT_TCP_NODELAY;

#[repr(C)]
pub struct ceph_options {
    pub flags: i32,
    pub fsid: ceph_fsid,
    pub my_addr: ceph_entity_addr,
    pub mount_timeout: c_ulong,
    pub osd_idle_ttl: c_ulong,
    pub osd_keepalive_timeout: c_ulong,
    pub osd_request_timeout: c_ulong,
    pub read_from_replica: u32,
    pub con_modes: [i32; 2],
    pub mon_addr: *mut ceph_entity_addr,
    pub num_mon: i32,
    pub name: *mut c_char,
    pub key: *mut ceph_crypto_key,
    pub crush_locs: rb_root,
}

pub const CEPH_MOUNT_TIMEOUT_DEFAULT: c_ulong = msecs_to_jiffies(60 * 1000);
pub const CEPH_OSD_KEEPALIVE_DEFAULT: c_ulong = msecs_to_jiffies(5 * 1000);
pub const CEPH_OSD_IDLE_TTL_DEFAULT: c_ulong = msecs_to_jiffies(60 * 1000);
pub const CEPH_OSD_REQUEST_TIMEOUT_DEFAULT: c_ulong = 0;
pub const CEPH_READ_FROM_REPLICA_DEFAULT: u32 = 0;
pub const CEPH_MONC_HUNT_INTERVAL: c_ulong = msecs_to_jiffies(3 * 1000);
pub const CEPH_MONC_PING_INTERVAL: c_ulong = msecs_to_jiffies(10 * 1000);
pub const CEPH_MONC_PING_TIMEOUT: c_ulong = msecs_to_jiffies(30 * 1000);
pub const CEPH_MONC_HUNT_BACKOFF: i32 = 2;
pub const CEPH_MONC_HUNT_MAX_MULT: i32 = 10;
pub const CEPH_MSG_MAX_CONTROL_LEN: usize = 16 * 1024 * 1024;
pub const CEPH_MSG_MAX_FRONT_LEN: usize = 16 * 1024 * 1024;
pub const CEPH_MSG_MAX_MIDDLE_LEN: usize = 16 * 1024 * 1024;
pub const CEPH_MSG_MAX_DATA_LEN: usize = 64 * 1024 * 1024;
pub const CEPH_AUTH_NAME_DEFAULT: &[u8] = b"guest\0";

#[inline]
pub unsafe fn ceph_timeout_jiffies(timeout: c_ulong) -> c_ulong {
    if timeout != 0 { timeout } else { MAX_SCHEDULE_TIMEOUT }
}

pub struct ceph_mds_client;

#[repr(C)]
pub struct ceph_client {
    pub fsid: ceph_fsid,
    pub have_fsid: bool,
    pub private: *mut c_void,
    pub options: *mut ceph_options,
    pub mount_mutex: mutex,
    pub auth_wq: wait_queue_head_t,
    pub auth_err: i32,
    pub extra_mon_dispatch: Option<unsafe extern "C" fn(*mut ceph_client, *mut ceph_msg) -> i32>,
    pub supported_features: u64,
    pub required_features: u64,
    pub msgr: ceph_messenger,
    pub monc: ceph_mon_client,
    pub osdc: ceph_osd_client,
}

#[inline]
pub unsafe fn ceph_msgr2(client: *mut ceph_client) -> bool {
    (*(*client).options).con_modes[0] != CEPH_CON_MODE_UNKNOWN
}

#[repr(C)]
pub struct ceph_snap_context {
    pub nref: refcount_t,
    pub seq: u64,
    pub num_snaps: u32,
    pub snaps: [u64; 0],
}

extern "C" {
    pub fn ceph_create_snap_context(snap_count: u32, gfp_flags: gfp_t) -> *mut ceph_snap_context;
    pub fn ceph_get_snap_context(sc: *mut ceph_snap_context) -> *mut ceph_snap_context;
    pub fn ceph_put_snap_context(sc: *mut ceph_snap_context);
}

#[inline]
pub fn calc_pages_for(off: u64, len: u64) -> i32 {
    (((off.wrapping_add(len).wrapping_add(PAGE_SIZE - 1)) >> PAGE_SHIFT)
        - (off >> PAGE_SHIFT)) as i32
}

#[macro_export]
macro_rules! ceph_set_opt {
    ($client:expr, FSID) => {{ (*(*$client).options).flags |= $crate::CEPH_OPT_FSID; }};
    ($client:expr, NOSHARE) => {{ (*(*$client).options).flags |= $crate::CEPH_OPT_NOSHARE; }};
    ($client:expr, MYIP) => {{ (*(*$client).options).flags |= $crate::CEPH_OPT_MYIP; }};
    ($client:expr, NOCRC) => {{ (*(*$client).options).flags |= $crate::CEPH_OPT_NOCRC; }};
    ($client:expr, TCP_NODELAY) => {{ (*(*$client).options).flags |= $crate::CEPH_OPT_TCP_NODELAY; }};
    ($client:expr, NOMSGSIGN) => {{ (*(*$client).options).flags |= $crate::CEPH_OPT_NOMSGSIGN; }};
    ($client:expr, ABORT_ON_FULL) => {{ (*(*$client).options).flags |= $crate::CEPH_OPT_ABORT_ON_FULL; }};
    ($client:expr, RXBOUNCE) => {{ (*(*$client).options).flags |= $crate::CEPH_OPT_RXBOUNCE; }};
}
#[macro_export]
macro_rules! ceph_test_opt {
    ($client:expr, FSID) => {{ ((*(*$client).options).flags & $crate::CEPH_OPT_FSID) != 0 }};
    ($client:expr, NOSHARE) => {{ ((*(*$client).options).flags & $crate::CEPH_OPT_NOSHARE) != 0 }};
    ($client:expr, MYIP) => {{ ((*(*$client).options).flags & $crate::CEPH_OPT_MYIP) != 0 }};
    ($client:expr, NOCRC) => {{ ((*(*$client).options).flags & $crate::CEPH_OPT_NOCRC) != 0 }};
    ($client:expr, TCP_NODELAY) => {{ ((*(*$client).options).flags & $crate::CEPH_OPT_TCP_NODELAY) != 0 }};
    ($client:expr, NOMSGSIGN) => {{ ((*(*$client).options).flags & $crate::CEPH_OPT_NOMSGSIGN) != 0 }};
    ($client:expr, ABORT_ON_FULL) => {{ ((*(*$client).options).flags & $crate::CEPH_OPT_ABORT_ON_FULL) != 0 }};
    ($client:expr, RXBOUNCE) => {{ ((*(*$client).options).flags & $crate::CEPH_OPT_RXBOUNCE) != 0 }};
}

// The following red-black-tree helpers are retained as source-level macros;
// their rb_root/rb_node operations and container_of implementation are supplied externally.
#[macro_export]
macro_rules! RB_BYVAL { ($a:expr) => { $a }; }
#[macro_export]
macro_rules! RB_BYPTR { ($a:expr) => { &$a }; }
#[macro_export]
macro_rules! RB_CMP3WAY { ($a:expr, $b:expr) => { if $a < $b { -1 } else if $a > $b { 1 } else { 0 } }; }

extern "C" {
    pub static mut ceph_inode_cachep: *mut kmem_cache;
    pub static mut ceph_cap_cachep: *mut kmem_cache;
    pub static mut ceph_cap_snap_cachep: *mut kmem_cache;
    pub static mut ceph_cap_flush_cachep: *mut kmem_cache;
    pub static mut ceph_dentry_cachep: *mut kmem_cache;
    pub static mut ceph_file_cachep: *mut kmem_cache;
    pub static mut ceph_dir_file_cachep: *mut kmem_cache;
    pub static mut ceph_mds_request_cachep: *mut kmem_cache;
    pub static mut ceph_wb_pagevec_pool: *mut mempool_t;
    pub fn libceph_compatible(data: *mut c_void) -> bool;
    pub fn ceph_msg_type_name(ty: i32) -> *const c_char;
    pub fn ceph_check_fsid(client: *mut ceph_client, fsid: *mut ceph_fsid) -> i32;
    pub fn ceph_parse_fsid(s: *const c_char, fsid: *mut ceph_fsid) -> i32;
    pub fn ceph_alloc_options() -> *mut ceph_options;
    pub fn ceph_destroy_options(opt: *mut ceph_options);
    pub fn ceph_compare_options(new_opt: *mut ceph_options, client: *mut ceph_client) -> i32;
    pub fn ceph_create_client(opt: *mut ceph_options, private: *mut c_void) -> *mut ceph_client;
    pub fn ceph_client_addr(client: *mut ceph_client) -> *mut ceph_entity_addr;
    pub fn ceph_client_gid(client: *mut ceph_client) -> u64;
    pub fn ceph_destroy_client(client: *mut ceph_client);
    pub fn ceph_reset_client_addr(client: *mut ceph_client);
    pub fn __ceph_open_session(client: *mut ceph_client) -> i32;
    pub fn ceph_open_session(client: *mut ceph_client) -> i32;
    pub fn ceph_wait_for_latest_osdmap(client: *mut ceph_client, timeout: c_ulong) -> i32;
    pub fn ceph_release_page_vector(pages: *mut *mut page, num_pages: i32);
    pub fn ceph_alloc_page_vector(num_pages: i32, flags: gfp_t) -> *mut *mut page;
    pub fn ceph_copy_from_page_vector(pages: *mut *mut page, data: *mut c_void, off: loff_t, len: size_t);
    pub fn ceph_zero_page_vector_range(off: i32, len: i32, pages: *mut *mut page);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
