// SPDX-License-Identifier: LGPL-2.1
/*
 *
 *   Copyright (C) International Business Machines  Corp., 2002,2008
 *   Author(s): Steve French (sfrench@us.ibm.com)
 *
 */

// Translated from smb1misc.c.  SMB, kernel, and list symbols are supplied by
// the surrounding implementation.

pub unsafe fn header_assemble(
    buffer: *mut smb_hdr,
    smb_command: libc::c_char,
    tree_con: *const cifs_tcon,
    word_count: libc::c_int,
) -> libc::c_uint {
    let temp = buffer as *mut libc::c_char;

    libc::memset(temp as *mut libc::c_void, 0, 256);

    let in_len = (2 * word_count) as libc::c_uint
        + core::mem::size_of::<smb_hdr>() as libc::c_uint
        + 2;

    (*buffer).Protocol[0] = 0xffu8 as libc::c_char;
    (*buffer).Protocol[1] = b'S' as libc::c_char;
    (*buffer).Protocol[2] = b'M' as libc::c_char;
    (*buffer).Protocol[3] = b'B' as libc::c_char;
    (*buffer).Command = smb_command;
    (*buffer).Flags = 0;
    (*buffer).Flags2 = SMBFLG2_KNOWS_LONG_NAMES;
    (*buffer).Pid = cpu_to_le16((*current).tgid as u16);
    (*buffer).PidHigh = cpu_to_le16(((*current).tgid >> 16) as u16);
    if !tree_con.is_null() {
        (*buffer).Tid = (*tree_con).tid;
        if !(*tree_con).ses.is_null() {
            if (*(*tree_con).ses).capabilities & CAP_UNICODE != 0 {
                (*buffer).Flags2 |= SMBFLG2_UNICODE;
            }
            if (*(*tree_con).ses).capabilities & CAP_STATUS32 != 0 {
                (*buffer).Flags2 |= SMBFLG2_ERR_STATUS;
            }
            (*buffer).Uid = (*(*tree_con).ses).Suid;
            if !(*(*tree_con).ses).server.is_null() {
                (*buffer).Mid = get_next_mid((*(*tree_con).ses).server);
            }
        }
        if (*tree_con).Flags & SMB_SHARE_IS_IN_DFS != 0 {
            (*buffer).Flags2 |= SMBFLG2_DFS;
        }
        if (*tree_con).nocase {
            (*buffer).Flags |= SMBFLG_CASELESS;
        }
        if !(*tree_con).ses.is_null()
            && !(*(*tree_con).ses).server.is_null()
            && (*(*(*tree_con).ses).server).sign
        {
            (*buffer).Flags2 |= SMBFLG2_SECURITY_SIGNATURE;
        }
    }

    (*buffer).WordCount = word_count as libc::c_char;
    in_len
}

pub unsafe fn is_valid_oplock_break(
    buffer: *mut libc::c_char,
    srv: *mut TCP_Server_Info,
) -> bool {
    let buf = buffer as *mut smb_hdr;
    let p_smb = buffer as *mut smb_com_lock_req;

    cifs_dbg(FYI, "Checking for oplock break or dnotify response\0".as_ptr() as *const libc::c_char);
    if (*p_smb).hdr.Command == SMB_COM_NT_TRANSACT
        && (*p_smb).hdr.Flags & SMBFLG_RESPONSE != 0
    {
        let p_smbr = buf as *mut smb_com_transaction_change_notify_rsp;
        let mut data_offset: u32 = 0;
        let len = (*srv).total_read;
        if get_bcc(buf) > core::mem::size_of::<file_notify_information>() {
            data_offset = le32_to_cpu((*p_smbr).DataOffset);
            if data_offset > len - core::mem::size_of::<file_notify_information>() {
                cifs_dbg(FYI, "Invalid data_offset %u\n\0".as_ptr() as *const libc::c_char, data_offset);
                return true;
            }
            let pnotify = (&mut (*p_smbr).hdr.Protocol as *mut _ as *mut libc::c_char)
                .add(data_offset as usize) as *mut file_notify_information;
            cifs_dbg(FYI, "dnotify on %s Action: 0x%x\n\0".as_ptr() as *const libc::c_char,
                (*pnotify).FileName, (*pnotify).Action);
            return true;
        }
        if (*p_smbr).hdr.Status.CifsError != 0 {
            cifs_dbg(FYI, "notify err 0x%x\n\0".as_ptr() as *const libc::c_char,
                (*p_smbr).hdr.Status.CifsError);
            return true;
        }
        return false;
    }
    if (*p_smb).hdr.Command != SMB_COM_LOCKING_ANDX { return false; }
    if (*p_smb).hdr.Flags & SMBFLG_RESPONSE != 0 {
        if NT_STATUS_INVALID_HANDLE == le32_to_cpu((*p_smb).hdr.Status.CifsError) {
            cifs_dbg(FYI, "Invalid handle on oplock break\n\0".as_ptr() as *const libc::c_char);
            return true;
        } else if ERRbadfid == le16_to_cpu((*p_smb).hdr.Status.DosError.Error) {
            return true;
        } else { return false; }
    }
    if (*p_smb).hdr.WordCount != 8 { return false; }
    cifs_dbg(FYI, "oplock type 0x%x level 0x%x\n\0".as_ptr() as *const libc::c_char,
        (*p_smb).LockType, (*p_smb).OplockLevel);
    if (*p_smb).LockType & LOCKING_ANDX_OPLOCK_RELEASE == 0 { return false; }

    let pserver = if SERVER_IS_CHAN(srv) { (*srv).primary_server } else { srv };
    spin_lock(&mut cifs_tcp_ses_lock);
    list_for_each_entry!(ses, &(*pserver).smb_ses_list, smb_ses_list, {
        if cifs_ses_exiting(ses) { continue; }
        list_for_each_entry!(tcon, &(*ses).tcon_list, tcon_list, {
            if (*tcon).tid != (*buf).Tid { continue; }
            cifs_stats_inc(&mut (*tcon).stats.cifs_stats.num_oplock_brks);
            spin_lock(&mut (*tcon).open_file_lock);
            list_for_each_entry!(netfile, &(*tcon).openFileList, tlist, {
                if (*p_smb).Fid != (*netfile).fid.netfid { continue; }
                let p_cifs_inode = CIFS_I(d_inode((*netfile).dentry));
                set_bit(CIFS_INODE_PENDING_OPLOCK_BREAK, &mut (*p_cifs_inode).flags);
                (*netfile).oplock_epoch = 0;
                (*netfile).oplock_level = (*p_smb).OplockLevel;
                (*netfile).oplock_break_cancelled = false;
                cifs_queue_oplock_break(netfile);
                spin_unlock(&mut (*tcon).open_file_lock);
                spin_unlock(&mut cifs_tcp_ses_lock);
                return true;
            });
            spin_unlock(&mut (*tcon).open_file_lock);
            spin_unlock(&mut cifs_tcp_ses_lock);
            return true;
        });
    });
    spin_unlock(&mut cifs_tcp_ses_lock);
    true
}

pub unsafe fn smbCalcSize(buf: *mut libc::c_void) -> libc::c_uint {
    let ptr = buf as *mut smb_hdr;
    (core::mem::size_of::<smb_hdr>() + (2 * (*ptr).WordCount as usize) + 2 + get_bcc(ptr) as usize) as libc::c_uint
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
