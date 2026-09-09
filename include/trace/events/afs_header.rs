/* SPDX-License-Identifier: GPL-2.0-or-later */
/* AFS tracepoints.  This is the Rust representation of the declarations in
 * trace/events/afs.h; Linux tracepoint expansion is supplied by the caller.
 */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum afs_fs_operation {
    afs_FS_FetchData = 130, afs_FS_FetchACL = 131, afs_FS_FetchStatus = 132,
    afs_FS_StoreData = 133, afs_FS_StoreACL = 134, afs_FS_StoreStatus = 135,
    afs_FS_RemoveFile = 136, afs_FS_CreateFile = 137, afs_FS_Rename = 138,
    afs_FS_Symlink = 139, afs_FS_Link = 140, afs_FS_MakeDir = 141,
    afs_FS_RemoveDir = 142, afs_FS_GetVolumeInfo = 148, afs_FS_GetVolumeStatus = 149,
    afs_FS_GetRootVolume = 151, afs_FS_SetLock = 156, afs_FS_ExtendLock = 157,
    afs_FS_ReleaseLock = 158, afs_FS_Lookup = 161, afs_FS_InlineBulkStatus = 65536,
    afs_FS_FetchData64 = 65537, afs_FS_StoreData64 = 65538,
    afs_FS_GiveUpAllCallBacks = 65539, afs_FS_GetCapabilities = 65540,
    yfs_FS_FetchData = 130, yfs_FS_FetchACL = 64131, yfs_FS_FetchStatus = 64132,
    yfs_FS_StoreACL = 64134, yfs_FS_StoreStatus = 64135, yfs_FS_RemoveFile = 64136,
    yfs_FS_CreateFile = 64137, yfs_FS_Rename = 64138, yfs_FS_Symlink = 64139,
    yfs_FS_Link = 64140, yfs_FS_MakeDir = 64141, yfs_FS_RemoveDir = 64142,
    yfs_FS_GetVolumeStatus = 64149, yfs_FS_SetVolumeStatus = 64150,
    yfs_FS_SetLock = 64156, yfs_FS_ExtendLock = 64157, yfs_FS_ReleaseLock = 64158,
    yfs_FS_Lookup = 64161, yfs_FS_FlushCPS = 64165, yfs_FS_FetchOpaqueACL = 64168,
    yfs_FS_WhoAmI = 64170, yfs_FS_RemoveACL = 64171, yfs_FS_RemoveFile2 = 64173,
    yfs_FS_StoreOpaqueACL2 = 64174, yfs_FS_Rename_Replace = 64176,
    yfs_FS_Rename_NoReplace = 64177, yfs_FS_Rename_Exchange = 64187,
    yfs_FS_InlineBulkStatus = 64536, yfs_FS_FetchData64 = 64537,
    yfs_FS_StoreData64 = 64538, yfs_FS_UpdateSymlink = 64540,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum afs_vl_operation {
    afs_VL_GetEntryByNameU = 527, afs_VL_GetAddrsU = 533,
    afs_YFSVL_GetEndpoints = 64002, afs_YFSVL_GetCellName = 64014,
    afs_VL_GetCapabilities = 65537,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum afs_cm_operation {
    afs_CB_CallBack = 204, afs_CB_InitCallBackState = 205, afs_CB_Probe = 206,
    afs_CB_GetLock = 207, afs_CB_GetCE = 208, afs_CB_GetXStatsVersion = 209,
    afs_CB_GetXStats = 210, afs_CB_InitCallBackState3 = 213, afs_CB_ProbeUuid = 214,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum yfs_cm_operation {
    yfs_CB_Probe = 206, yfs_CB_GetLock = 207, yfs_CB_XStatsVersion = 209,
    yfs_CB_GetXStats = 210, yfs_CB_InitCallBackState3 = 213, yfs_CB_ProbeUuid = 214,
    yfs_CB_GetServerPrefs = 215, yfs_CB_GetCellServDV = 216,
    yfs_CB_GetLocalCell = 217, yfs_CB_GetCacheConfig = 218,
    yfs_CB_GetCellByNum = 65537, yfs_CB_TellMeAboutYourself = 65538,
    yfs_CB_CallBack = 64204,
}

/* The following X-macro declarations are represented as Rust name/string
 * tables.  The terminal E_ entry in each C list is included identically. */
macro_rules! afs_trace_table {
    ($($name:ident => $text:literal),* $(,)?) => {
        &[$((stringify!($name), $text)),*]
    };
}

pub const afs_fs_operations: &[(&str, &str)] = afs_trace_table!(
    afs_FS_FetchData => "FS.FetchData", afs_FS_FetchStatus => "FS.FetchStatus",
    afs_FS_StoreData => "FS.StoreData", afs_FS_StoreStatus => "FS.StoreStatus",
    afs_FS_RemoveFile => "FS.RemoveFile", afs_FS_CreateFile => "FS.CreateFile",
    afs_FS_Rename => "FS.Rename", afs_FS_Symlink => "FS.Symlink", afs_FS_Link => "FS.Link",
    afs_FS_MakeDir => "FS.MakeDir", afs_FS_RemoveDir => "FS.RemoveDir",
    afs_FS_GetVolumeInfo => "FS.GetVolumeInfo", afs_FS_GetVolumeStatus => "FS.GetVolumeStatus",
    afs_FS_GetRootVolume => "FS.GetRootVolume", afs_FS_SetLock => "FS.SetLock",
    afs_FS_ExtendLock => "FS.ExtendLock", afs_FS_ReleaseLock => "FS.ReleaseLock",
    afs_FS_Lookup => "FS.Lookup", afs_FS_InlineBulkStatus => "FS.InlineBulkStatus",
    afs_FS_FetchData64 => "FS.FetchData64", afs_FS_StoreData64 => "FS.StoreData64",
    afs_FS_GiveUpAllCallBacks => "FS.GiveUpAllCallBacks", afs_FS_GetCapabilities => "FS.GetCapabilities",
);

/* TRACE_EVENT declarations are intentionally retained as declarative metadata:
 * their TP_PROTO, TP_ARGS, TP_STRUCT__entry, TP_fast_assign and TP_printk
 * clauses are Linux tracing DSL constructs with no standalone Rust ABI. */
pub const AFS_TRACE_SYSTEM: &str = "afs";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
