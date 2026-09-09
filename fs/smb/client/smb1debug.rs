// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 *   Copyright (C) International Business Machines  Corp., 2000,2005
 *
 *   Modified by Steve French (sfrench@us.ibm.com)
 */

// Declarations supplied by cifsproto.h, smb1proto.h, and cifs_debug.h are
// intentionally left to the surrounding translation unit.

#[cfg(CONFIG_CIFS_DEBUG2)]
#[repr(C)]
pub struct TCP_Server_Info {
    pub ops: *mut TCP_Server_Operations,
    pub total_read: usize,
}

#[cfg(CONFIG_CIFS_DEBUG2)]
#[repr(C)]
pub struct TCP_Server_Operations {
    pub check_message:
        unsafe extern "C" fn(*mut core::ffi::c_void, usize, usize, *mut TCP_Server_Info) -> bool,
    pub calc_smb_size: unsafe extern "C" fn(*mut smb_hdr) -> usize,
}

#[cfg(CONFIG_CIFS_DEBUG2)]
#[repr(C)]
pub struct CifsStatus {
    pub CifsError: u32,
}

#[cfg(CONFIG_CIFS_DEBUG2)]
#[repr(C)]
pub struct smb_hdr {
    pub Command: u8,
    pub Status: CifsStatus,
    pub Flags: u8,
    pub Flags2: u16,
    pub Mid: u16,
    pub Pid: u16,
    pub WordCount: u8,
}

#[cfg(CONFIG_CIFS_DEBUG2)]
unsafe extern "C" {
    fn cifs_dbg(level: i32, format: *const core::ffi::c_char, ...);
}

#[cfg(CONFIG_CIFS_DEBUG2)]
pub const VFS: i32 = 0;

pub unsafe fn cifs_dump_detail(
    buf: *mut core::ffi::c_void,
    buf_len: usize,
    server: *mut TCP_Server_Info,
) {
    #[cfg(CONFIG_CIFS_DEBUG2)]
    {
        let smb = buf as *mut smb_hdr;

        cifs_dbg(
            VFS,
            c"Cmd: %d Err: 0x%x Flags: 0x%x Flgs2: 0x%x Mid: %d Pid: %d Wct: %d\n".as_ptr(),
            (*smb).Command as i32,
            (*smb).Status.CifsError,
            (*smb).Flags as i32,
            (*smb).Flags2 as i32,
            (*smb).Mid as i32,
            (*smb).Pid as i32,
            (*smb).WordCount as i32,
        );
        if !((*(*server).ops).check_message)(
            buf,
            buf_len,
            (*server).total_read,
            server,
        ) {
            cifs_dbg(
                VFS,
                c"smb buf %p len %u\n".as_ptr(),
                smb,
                ((*(*server).ops).calc_smb_size)(smb),
            );
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
