/* SPDX-License-Identifier: LGPL-2.1 */
/*
 *
 *   Structure definitions for io control for cifs/smb3
 *
 *   Copyright (c) 2015 Steve French <steve.french@primarydata.com>
 *
 */

#[repr(C, packed)]
pub struct smb_mnt_fs_info {
    pub version: __u32, /* 0001 */
    pub protocol_id: __u16,
    pub tcon_flags: __u16,
    pub vol_serial_number: __u32,
    pub vol_create_time: __u32,
    pub share_caps: __u32,
    pub share_flags: __u32,
    pub sector_flags: __u32,
    pub optimal_sector_size: __u32,
    pub max_bytes_chunk: __u32,
    pub fs_attributes: __u32,
    pub max_path_component: __u32,
    pub device_type: __u32,
    pub device_characteristics: __u32,
    pub maximal_access: __u32,
    pub cifs_posix_caps: __u64,
}

#[repr(C, packed)]
pub struct smb_mnt_tcon_info {
    pub tid: __u32,
    pub session_id: __u64,
}

#[repr(C, packed)]
pub struct smb_snapshot_array {
    pub number_of_snapshots: __u32,
    pub number_of_snapshots_returned: __u32,
    pub snapshot_array_size: __u32,
    /* snapshots[]; */
}

/* query_info flags */
pub const PASSTHRU_QUERY_INFO: u32 = 0x00000000;
pub const PASSTHRU_FSCTL: u32 = 0x00000001;
pub const PASSTHRU_SET_INFO: u32 = 0x00000002;

#[repr(C, packed)]
pub struct smb_query_info {
    pub info_type: __u32,
    pub file_info_class: __u32,
    pub additional_information: __u32,
    pub flags: __u32,
    pub input_buffer_length: __u32,
    pub output_buffer_length: __u32,
    /* char buffer[]; */
}

/*
 * Dumping the commonly used 16 byte (e.g. CCM and GCM128) keys still supported
 * for backlevel compatibility, but is not sufficient for dumping the less
 * frequently used GCM256 (32 byte) keys (see the newer "CIFS_DUMP_FULL_KEY"
 * ioctl for dumping decryption info for GCM256 mounts)
 */
#[repr(C, packed)]
pub struct smb3_key_debug_info {
    pub Suid: __u64,
    pub cipher_type: __u16,
    pub auth_key: [__u8; SMB2_NTLMV2_SESSKEY_SIZE as usize],
    pub smb3encryptionkey: [__u8; SMB3_SIGN_KEY_SIZE as usize],
    pub smb3decryptionkey: [__u8; SMB3_SIGN_KEY_SIZE as usize],
}

/* Dump variable-sized keys */
#[repr(C, packed)]
pub struct smb3_full_key_debug_info {
    /* INPUT: size of userspace buffer */
    pub in_size: __u32,
    /*
     * INPUT: 0 for current user, otherwise session to dump
     * OUTPUT: session id that was dumped
     */
    pub session_id: __u64,
    pub cipher_type: __u16,
    pub session_key_length: __u8,
    pub server_in_key_length: __u8,
    pub server_out_key_length: __u8,
    pub data: [__u8; 0],
    /*
     * return this struct with the keys appended at the end:
     * __u8 session_key[session_key_length];
     * __u8 server_in_key[server_in_key_length];
     * __u8 server_out_key[server_out_key_length];
     */
}

#[repr(C, packed)]
pub struct smb3_notify {
    pub completion_filter: __u32,
    pub watch_tree: bool,
}

#[repr(C, packed)]
pub struct smb3_notify_info {
    pub completion_filter: __u32,
    pub watch_tree: bool,
    pub data_len: __u32, /* size of notify data below */
    pub notify_data: [__u8; 0],
}

pub const CIFS_IOCTL_MAGIC: u32 = 0xCF;
pub const CIFS_IOC_COPYCHUNK_FILE: _ = _IOW!(CIFS_IOCTL_MAGIC, 3, i32);
pub const CIFS_IOC_SET_INTEGRITY: _ = _IO!(CIFS_IOCTL_MAGIC, 4);
pub const CIFS_IOC_GET_MNT_INFO: _ = _IOR!(CIFS_IOCTL_MAGIC, 5, smb_mnt_fs_info);
pub const CIFS_ENUMERATE_SNAPSHOTS: _ = _IOR!(CIFS_IOCTL_MAGIC, 6, smb_snapshot_array);
pub const CIFS_QUERY_INFO: _ = _IOWR!(CIFS_IOCTL_MAGIC, 7, smb_query_info);
pub const CIFS_DUMP_KEY: _ = _IOWR!(CIFS_IOCTL_MAGIC, 8, smb3_key_debug_info);
pub const CIFS_IOC_NOTIFY: _ = _IOW!(CIFS_IOCTL_MAGIC, 9, smb3_notify);
pub const CIFS_DUMP_FULL_KEY: _ = _IOWR!(CIFS_IOCTL_MAGIC, 10, smb3_full_key_debug_info);
pub const CIFS_IOC_NOTIFY_INFO: _ = _IOWR!(CIFS_IOCTL_MAGIC, 11, smb3_notify_info);
pub const CIFS_IOC_GET_TCON_INFO: _ = _IOR!(CIFS_IOCTL_MAGIC, 12, smb_mnt_tcon_info);
pub const CIFS_IOC_SHUTDOWN: _ = _IOR!('X', 125, __u32);

/*
 * Flags for going down operation
 */
pub const CIFS_GOING_FLAGS_DEFAULT: u32 = 0x0; /* going down */
pub const CIFS_GOING_FLAGS_LOGFLUSH: u32 = 0x1; /* flush log but not data */
pub const CIFS_GOING_FLAGS_NOLOGFLUSH: u32 = 0x2; /* don't flush log nor data */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
