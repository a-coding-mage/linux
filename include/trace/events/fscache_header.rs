/* SPDX-License-Identifier: GPL-2.0-or-later */
/* FS-Cache tracepoints. Rust translation of the C trace header. */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FscacheCacheTrace { Collision, GetAcquire, NewAcquire, PutAllocVolume, PutCache, PutPrepFailed, PutRelinquish, PutVolume }

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FscacheVolumeTrace { Collision, GetCookie, GetCreateWork, GetHashCollision, GetWithdraw, Free, NewAcquire, PutCookie, PutCreateWork, PutHashCollision, PutRelinquish, PutWithdraw, SeeCreateWork, SeeHashWake, WaitCreateWork }

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FscacheCookieTrace { Collision, Discard, Failed, GetAttachObject, GetEndAccess, GetHashCollision, GetInvalWork, GetLru, GetUseWork, NewAcquire, PutHashCollision, PutLru, PutObject, PutOverQueued, PutRelinquish, PutWithdrawn, PutWork, SeeActive, SeeLruDiscard, SeeLruDiscardClear, SeeLruDoOne, SeeRelinquish, SeeWithdraw, SeeWork }

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FscacheActiveTrace { Use, UseModify, Unuse }

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FscacheAccessTrace { AcquireVolume, AcquireVolumeEnd, CachePin, CacheUnpin, InvalidateCookie, InvalidateCookieEnd, IoEnd, IoNotLive, IoRead, IoResize, IoWait, IoWrite, LookupCookie, LookupCookieEnd, LookupCookieEndFailed, RelinquishVolume, RelinquishVolumeEnd, Unlive }

/* The EM/E_ trace tables are represented as the corresponding string maps. */
pub const FSCACHE_CACHE_TRACES: &[(FscacheCacheTrace, &str)] = &[
    (FscacheCacheTrace::Collision, "*COLLIDE*"), (FscacheCacheTrace::GetAcquire, "GET acq  "), (FscacheCacheTrace::NewAcquire, "NEW acq  "), (FscacheCacheTrace::PutAllocVolume, "PUT alvol"), (FscacheCacheTrace::PutCache, "PUT cache"), (FscacheCacheTrace::PutPrepFailed, "PUT pfail"), (FscacheCacheTrace::PutRelinquish, "PUT relnq"), (FscacheCacheTrace::PutVolume, "PUT vol  "),
];
pub const FSCACHE_VOLUME_TRACES: &[(FscacheVolumeTrace, &str)] = &[
    (FscacheVolumeTrace::Collision, "*COLLIDE*"), (FscacheVolumeTrace::GetCookie, "GET cook "), (FscacheVolumeTrace::GetCreateWork, "GET creat"), (FscacheVolumeTrace::GetHashCollision, "GET hcoll"), (FscacheVolumeTrace::GetWithdraw, "GET withd"), (FscacheVolumeTrace::Free, "FREE     "), (FscacheVolumeTrace::NewAcquire, "NEW acq  "), (FscacheVolumeTrace::PutCookie, "PUT cook "), (FscacheVolumeTrace::PutCreateWork, "PUT creat"), (FscacheVolumeTrace::PutHashCollision, "PUT hcoll"), (FscacheVolumeTrace::PutRelinquish, "PUT relnq"), (FscacheVolumeTrace::PutWithdraw, "PUT withd"), (FscacheVolumeTrace::SeeCreateWork, "SEE creat"), (FscacheVolumeTrace::SeeHashWake, "SEE hwake"), (FscacheVolumeTrace::WaitCreateWork, "WAIT crea"),
];
pub const FSCACHE_COOKIE_TRACES: &[(&str)] = &[
    "*COLLIDE*", "DISCARD  ", "FAILED   ", "GET attch", "GQ  endac", "GET hcoll", "GQ  inval", "GET lru  ", "GQ  use  ", "NEW acq  ", "PUT hcoll", "PUT lru  ", "PUT obj  ", "PQ  overq", "PUT relnq", "PUT wthdn", "PQ  work ", "-   activ", "-   x-lru", "-   lrudc", "-   lrudo", "-   x-rlq", "-   x-wth", "-   work ",
];
pub const FSCACHE_ACTIVE_TRACES: &[&str] = &["USE          ", "USE-m        ", "UNUSE        "];
pub const FSCACHE_ACCESS_TRACES: &[&str] = &["BEGIN acq_vol", "END   acq_vol", "PIN   cache  ", "UNPIN cache  ", "BEGIN inval  ", "END   inval  ", "END   io     ", "END   io_notl", "BEGIN io_read", "BEGIN io_resz", "WAIT  io     ", "BEGIN io_writ", "BEGIN lookup ", "END   lookup ", "END   lookupf", "BEGIN rlq_vol", "END   rlq_vol", "END   unlive "];

/* Tracepoint declarations retain the C event interfaces and entry layouts. */
#[repr(C)] pub struct FscacheCacheEntry { pub cache: u32, pub usage: i32, pub where_: FscacheCacheTrace }
#[repr(C)] pub struct FscacheVolumeEntry { pub volume: u32, pub usage: i32, pub where_: FscacheVolumeTrace }
#[repr(C)] pub struct FscacheCookieEntry { pub cookie: u32, pub ref_: i32, pub where_: FscacheCookieTrace }
#[repr(C)] pub struct FscacheActiveEntry { pub cookie: u32, pub ref_: i32, pub n_active: i32, pub n_accesses: i32, pub why: FscacheActiveTrace }
#[repr(C)] pub struct FscacheAccessCacheEntry { pub cache: u32, pub ref_: i32, pub n_accesses: i32, pub why: FscacheAccessTrace }
#[repr(C)] pub struct FscacheAccessVolumeEntry { pub volume: u32, pub cookie: u32, pub ref_: i32, pub n_accesses: i32, pub why: FscacheAccessTrace }
#[repr(C)] pub struct FscacheAccessEntry { pub cookie: u32, pub ref_: i32, pub n_accesses: i32, pub why: FscacheAccessTrace }
#[repr(C)] pub struct FscacheAcquireEntry { pub cookie: u32, pub volume: u32, pub v_ref: i32, pub v_n_cookies: i32 }
#[repr(C)] pub struct FscacheRelinquishEntry { pub cookie: u32, pub volume: u32, pub ref_: i32, pub n_active: i32, pub flags: u8, pub retire: bool }
#[repr(C)] pub struct FscacheInvalidateEntry { pub cookie: u32, pub new_size: i64 }
#[repr(C)] pub struct FscacheResizeEntry { pub cookie: u32, pub old_size: i64, pub new_size: i64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
