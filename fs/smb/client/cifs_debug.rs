// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of cifs_debug.c. Kernel types, constants, and
 * helpers referenced here are supplied by the surrounding CIFS code. */

use core::ffi::c_void;

pub unsafe fn cifs_dump_mem(label: *mut i8, data: *mut c_void, length: i32) {
    pr_debug!("%s: dump of %d bytes of data at 0x%p\n", label, length, data);
    print_hex_dump!(KERN_DEBUG, "", DUMP_PREFIX_OFFSET, 16, 4, data, length, true);
}

pub unsafe fn cifs_dump_mids(server: *mut TCP_Server_Info) {
    #[cfg(CONFIG_CIFS_DEBUG2)]
    {
        if server.is_null() { return; }
        cifs_dbg!(VFS, "Dump pending requests:\n");
        spin_lock(&mut (*server).mid_queue_lock);
        list_for_each_entry!(mid_entry, (*server).pending_mid_q, qhead, {
            cifs_dbg!(VFS, "State: %d Cmd: %d Pid: %d Cbdata: %p Mid %llu\n",
                mid_entry.mid_state, le16_to_cpu(mid_entry.command), mid_entry.pid,
                mid_entry.callback_data, mid_entry.mid);
            #[cfg(CONFIG_CIFS_STATS2)]
            cifs_dbg!(VFS, "IsLarge: %d buf: %p time rcv: %ld now: %ld\n",
                mid_entry.large_buf, mid_entry.resp_buf, mid_entry.when_received, jiffies);
            cifs_dbg!(VFS, "IsMult: %d IsEnd: %d\n", mid_entry.multiRsp, mid_entry.multiEnd);
            if !mid_entry.resp_buf.is_null() {
                ((*(*server).ops).dump_detail)(mid_entry.resp_buf, mid_entry.response_pdu_len, server);
                cifs_dump_mem(b"existing buf: \0".as_ptr() as *mut i8, mid_entry.resp_buf, 62);
            }
        });
        spin_unlock(&mut (*server).mid_queue_lock);
    }
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn cifs_debug_tcon(m: *mut seq_file, tcon: *mut cifs_tcon) {
    let dev_type = le32_to_cpu((*tcon).fsDevInfo.DeviceType);
    seq_printf!(m, "%s Mounts: %d ", (*tcon).tree_name, (*tcon).tc_count);
    if !(*tcon).nativeFileSystem.is_null() { seq_printf!(m, "Type: %s ", (*tcon).nativeFileSystem); }
    seq_printf!(m, "DevInfo: 0x%x Attributes: 0x%x\n\tPathComponentMax: %d Status: %d",
        le32_to_cpu((*tcon).fsDevInfo.DeviceCharacteristics), le32_to_cpu((*tcon).fsAttrInfo.Attributes),
        le32_to_cpu((*tcon).fsAttrInfo.MaxPathNameComponentLength), (*tcon).status);
    if dev_type == FILE_DEVICE_DISK { seq_puts!(m, " type: DISK "); }
    else if dev_type == FILE_DEVICE_CD_ROM { seq_puts!(m, " type: CDROM "); }
    else { seq_printf!(m, " type: %d ", dev_type); }
    seq_printf!(m, "Serial Number: 0x%x", (*tcon).vol_serial_number);
    if (*tcon).seal || ((*(*tcon).ses).session_flags & SMB2_SESSION_FLAG_ENCRYPT_DATA) != 0 ||
       ((*tcon).share_flags & SHI1005_FLAGS_ENCRYPT_DATA) != 0 { seq_puts!(m, " encrypted"); }
    if (*tcon).nocase { seq_puts!(m, " nocase"); }
    if (*tcon).unix_ext { seq_puts!(m, " POSIX Extensions"); }
    if let Some(f) = (*(*(*tcon).ses).server).ops.dump_share_caps { f(m, tcon); }
    if (*tcon).use_witness { seq_puts!(m, " Witness"); }
    if (*tcon).broken_sparse_sup { seq_puts!(m, " nosparse"); }
    if (*tcon).need_reconnect { seq_puts!(m, "\tDISCONNECTED "); }
    spin_lock(&mut (*tcon).tc_lock);
    if !(*tcon).origin_fullpath.is_null() { seq_printf!(m, "\n\tDFS origin fullpath: %s", (*tcon).origin_fullpath); }
    spin_unlock(&mut (*tcon).tc_lock); seq_putc!(m, b'\n' as i32);
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn smb_speed_to_str(bps: usize) -> &'static str {
    match bps / 1000 / 1000 {
        SPEED_10 => "10Mbps", SPEED_100 => "100Mbps", SPEED_1000 => "1Gbps", SPEED_2500 => "2.5Gbps",
        SPEED_5000 => "5Gbps", SPEED_10000 => "10Gbps", SPEED_14000 => "14Gbps", SPEED_20000 => "20Gbps",
        SPEED_25000 => "25Gbps", SPEED_40000 => "40Gbps", SPEED_50000 => "50Gbps", SPEED_56000 => "56Gbps",
        SPEED_100000 => "100Gbps", SPEED_200000 => "200Gbps", SPEED_400000 => "400Gbps", SPEED_800000 => "800Gbps",
        _ => "Unknown",
    }
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn cifs_dump_channel(m: *mut seq_file, i: i32, chan: *mut cifs_chan) {
    let server = (*chan).server;
    if server.is_null() { seq_printf!(m, "\n\n\t\tChannel: %d DISABLED", i + 1); return; }
    seq_printf!(m, "\n\n\t\tChannel: %d ConnectionId: 0x%llx\n\t\tNumber of credits: %d,%d,%d Dialect 0x%x\n\t\tTCP status: %d Instance: %d\n\t\tLocal Users To Server: %d SecMode: 0x%x Req On Wire: %d\n\t\tIn Send: %d In MaxReq Wait: %d",
        i + 1, (*server).conn_id, (*server).credits, (*server).echo_credits, (*server).oplock_credits,
        (*server).dialect, (*server).tcpStatus, (*server).reconnect_instance, (*server).srv_count,
        (*server).sec_mode, in_flight(server), atomic_read!((*server).in_send), atomic_read!((*server).num_waiters));
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn compression_alg_str(alg: __le16) -> &'static str { match alg {
    SMB3_COMPRESS_NONE => "NONE", SMB3_COMPRESS_LZNT1 => "LZNT1", SMB3_COMPRESS_LZ77 => "LZ77",
    SMB3_COMPRESS_LZ77_HUFF => "LZ77-Huffman", SMB3_COMPRESS_PATTERN => "Pattern_V1", _ => "invalid" } }
#[cfg(CONFIG_PROC_FS)]
unsafe fn cipher_alg_str(cipher: __le16) -> &'static str { match cipher {
    SMB2_ENCRYPTION_AES128_CCM => "AES128-CCM", SMB2_ENCRYPTION_AES128_GCM => "AES128-GCM",
    SMB2_ENCRYPTION_AES256_CCM => "AES256-CCM", SMB2_ENCRYPTION_AES256_GCM => "AES256-GCM", _ => "UNKNOWN" } }

#[cfg(CONFIG_PROC_FS)]
unsafe fn cifs_debug_files_proc_show(m: *mut seq_file, _v: *mut c_void) -> i32 {
    seq_puts!(m, "# Version:1\n# Format:\n# <tree id> <ses id> <persistent fid> <flags> <count> <pid> <uid>");
    #[cfg(CONFIG_CIFS_DEBUG2)] seq_puts!(m, " <filename> <lease> <lease-key> <mid>\n");
    #[cfg(not(CONFIG_CIFS_DEBUG2))] seq_puts!(m, " <filename> <lease> <lease-key>\n");
    spin_lock(&mut cifs_tcp_ses_lock);
    list_for_each_entry!(server, cifs_tcp_ses_list, tcp_ses_list, {
        list_for_each_entry!(ses, server.smb_ses_list, smb_ses_list, {
            if cifs_ses_exiting(ses) { continue; }
            list_for_each_entry!(tcon, ses.tcon_list, tcon_list, {
                spin_lock(&mut tcon.open_file_lock);
                list_for_each_entry!(cfile, tcon.openFileList, tlist, {
                    seq_printf!(m, "0x%x 0x%llx 0x%llx 0x%x %d %d %d %pd\n", tcon.tid, ses.Suid,
                        cfile.fid.persistent_fid, cfile.f_flags, cfile.count, cfile.pid,
                        from_kuid(&init_user_ns, cfile.uid), cfile.dentry);
                });
                spin_unlock(&mut tcon.open_file_lock);
            });
        });
    });
    spin_unlock(&mut cifs_tcp_ses_lock); seq_putc!(m, b'\n' as i32); 0
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn cifs_debug_data_proc_show(m: *mut seq_file, _v: *mut c_void) -> i32 {
    seq_puts!(m, "Display Internal CIFS Data Structures for Debugging\n---------------------------------------------------\n");
    seq_printf!(m, "CIFS Version %s\nFeatures:", CIFS_VERSION);
    seq_puts!(m, ",ACL\n"); seq_printf!(m, "CIFSMaxBufSize: %d\n", CIFSMaxBufSize);
    seq_printf!(m, "Active VFS Requests: %d\n\nServers: ", GlobalTotalActiveXid);
    cifs_swn_dump(m); seq_putc!(m, b'\n' as i32); 0
}

#[cfg(CONFIG_PROC_FS)]
static mut proc_fs_cifs: *mut proc_dir_entry = core::ptr::null_mut();

#[cfg(CONFIG_PROC_FS)]
pub unsafe fn cifs_proc_init() {
    proc_fs_cifs = proc_mkdir!("fs/cifs", core::ptr::null_mut());
    if proc_fs_cifs.is_null() { return; }
    proc_create_single!("DebugData", 0, proc_fs_cifs, cifs_debug_data_proc_show);
    proc_create_single!("open_files", 0o400, proc_fs_cifs, cifs_debug_files_proc_show);
}
#[cfg(CONFIG_PROC_FS)]
pub unsafe fn cifs_proc_clean() { if !proc_fs_cifs.is_null() { remove_proc_entry!("fs/cifs", core::ptr::null_mut()); } }
#[cfg(not(CONFIG_PROC_FS))]
pub unsafe fn cifs_proc_init() {}
#[cfg(not(CONFIG_PROC_FS))]
pub unsafe fn cifs_proc_clean() {}

#[cfg(CONFIG_PROC_FS)]
unsafe fn cifs_security_flags_handle_must_flags(flags: *mut u32) {
    let signflags = *flags & (CIFSSEC_MUST_SIGN | CIFSSEC_MUST_SEAL);
    if (*flags & CIFSSEC_MUST_KRB5) == CIFSSEC_MUST_KRB5 { *flags = CIFSSEC_MUST_KRB5; }
    else if (*flags & CIFSSEC_MUST_NTLMSSP) == CIFSSEC_MUST_NTLMSSP { *flags = CIFSSEC_MUST_NTLMSSP; }
    else if (*flags & CIFSSEC_MUST_NTLMV2) == CIFSSEC_MUST_NTLMV2 { *flags = CIFSSEC_MUST_NTLMV2; }
    *flags |= signflags;
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn cifsFYI_proc_show(m: *mut seq_file, _v: *mut c_void) -> i32 { seq_printf!(m, "%d\n", cifsFYI); 0 }
#[cfg(CONFIG_PROC_FS)]
unsafe fn cifs_linux_ext_proc_show(m: *mut seq_file, _v: *mut c_void) -> i32 { seq_printf!(m, "%d\n", linuxExtEnabled); 0 }
#[cfg(CONFIG_PROC_FS)]
unsafe fn cifs_lookup_cache_proc_show(m: *mut seq_file, _v: *mut c_void) -> i32 { seq_printf!(m, "%d\n", lookupCacheEnabled); 0 }
#[cfg(CONFIG_PROC_FS)]
unsafe fn traceSMB_proc_show(m: *mut seq_file, _v: *mut c_void) -> i32 { seq_printf!(m, "%d\n", traceSMB); 0 }
#[cfg(CONFIG_PROC_FS)]
unsafe fn cifs_security_flags_proc_show(m: *mut seq_file, _v: *mut c_void) -> i32 { seq_printf!(m, "0x%x\n", global_secflags); 0 }

#[cfg(CONFIG_PROC_FS)]
unsafe fn cifs_stats_proc_show(m: *mut seq_file, _v: *mut c_void) -> i32 {
    seq_puts!(m, "Resources in use\n");
    seq_printf!(m, "CIFS Session: %d\nShare (unique mount targets): %d\n", sesInfoAllocCount.counter, tconInfoAllocCount.counter);
    seq_printf!(m, "SMB Request/Response Buffer: %d Pool size: %d\n", buf_alloc_count.counter, cifs_min_rcv + tcpSesAllocCount.counter);
    seq_printf!(m, "SMB Small Req/Resp Buffer: %d Pool size: %d\n", small_buf_alloc_count.counter, cifs_min_small);
    seq_printf!(m, "Operations (MIDs): %d\n", atomic_read!(mid_count));
    seq_printf!(m, "\n%d session %d share reconnects\nTotal vfs operations: %d maximum at one time: %d\n",
        tcpSesReconnectCount.counter, tconInfoReconnectCount.counter, GlobalCurrentXid, GlobalMaxActiveXid);
    seq_putc!(m, b'\n' as i32); 0
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn cifs_mount_params_proc_show(m: *mut seq_file, _v: *mut c_void) -> i32 {
    let mut p = smb3_fs_parameters;
    while !p.name.is_null() {
        let ty = if p.type_.is_null() { if p.flags == fs_param_neg_with_no { "noflag" } else { "flag" } }
            else if p.type_ == fs_param_is_bool { "bool" } else if p.type_ == fs_param_is_u32 { "u32" }
            else if p.type_ == fs_param_is_u64 { "u64" } else if p.type_ == fs_param_is_string { "string" } else { "unknown" };
        seq_printf!(m, "%s:%s\n", p.name, ty); p = p.add(1);
    } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
