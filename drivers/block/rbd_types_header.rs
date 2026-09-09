/*
 * Ceph - scalable distributed file system
 *
 * Copyright (C) 2004-2010 Sage Weil <sage@newdream.net>
 *
 * This is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License version 2.1, as published by the Free Software
 * Foundation.  See file COPYING.
 *
 */

/*
 * For format version 2, rbd image 'foo' consists of objects
 *   rbd_id.foo          - id of image
 *   rbd_header.<id>    - image metadata
 *   rbd_object_map.<id> - optional image object map
 *   rbd_data.<id>.0000000000000000
 *   rbd_data.<id>.0000000000000001
 *   ...                 - data
 * Clients do not access header data directly in rbd format 2.
 */

pub const RBD_HEADER_PREFIX: &str = "rbd_header.";
pub const RBD_OBJECT_MAP_PREFIX: &str = "rbd_object_map.";
pub const RBD_ID_PREFIX: &str = "rbd_id.";
pub const RBD_V2_DATA_FORMAT: &str = "%s.%016llx";

pub const RBD_LOCK_NAME: &str = "rbd_lock";
pub const RBD_LOCK_TAG: &str = "internal";
pub const RBD_LOCK_COOKIE_PREFIX: &str = "auto";

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rbd_notify_op {
    RBD_NOTIFY_OP_ACQUIRED_LOCK = 0,
    RBD_NOTIFY_OP_RELEASED_LOCK = 1,
    RBD_NOTIFY_OP_REQUEST_LOCK = 2,
    RBD_NOTIFY_OP_HEADER_UPDATE = 3,
}

pub const OBJECT_NONEXISTENT: u32 = 0;
pub const OBJECT_EXISTS: u32 = 1;
pub const OBJECT_PENDING: u32 = 2;
pub const OBJECT_EXISTS_CLEAN: u32 = 3;

pub const RBD_FLAG_OBJECT_MAP_INVALID: u64 = 1u64 << 0;
pub const RBD_FLAG_FAST_DIFF_INVALID: u64 = 1u64 << 1;

/*
 * For format version 1, rbd image 'foo' consists of objects
 *   foo.rbd          - image metadata
 *   rb.<idhi>.<idlo>.<extra>.000000000000
 *   rb.<idhi>.<idlo>.<extra>.000000000001
 *   ...              - data
 * There is no notion of a persistent image id in rbd format 1.
 */

pub const RBD_SUFFIX: &str = ".rbd";
pub const RBD_V1_DATA_FORMAT: &str = "%s.%012llx";

pub const RBD_DIRECTORY: &str = "rbd_directory";
pub const RBD_INFO: &str = "rbd_info";

pub const RBD_DEFAULT_OBJ_ORDER: u32 = 22; /* 4MB */
pub const RBD_MIN_OBJ_ORDER: u32 = 16;
pub const RBD_MAX_OBJ_ORDER: u32 = 30;

pub const RBD_HEADER_TEXT: &str = "<<< Rados Block Device Image >>>\n";
pub const RBD_HEADER_SIGNATURE: &str = "RBD";
pub const RBD_HEADER_VERSION: &str = "001.005";

#[repr(C, packed)]
pub struct rbd_image_snap_ondisk {
    pub id: u64,
    pub image_size: u64,
}

#[repr(C, packed)]
pub struct rbd_image_header_ondisk_options {
    pub order: u8,
    pub crypt_type: u8,
    pub comp_type: u8,
    pub unused: u8,
}

#[repr(C, packed)]
pub struct rbd_image_header_ondisk {
    pub text: [i8; 40],
    pub object_prefix: [i8; 24],
    pub signature: [i8; 4],
    pub version: [i8; 8],
    pub options: rbd_image_header_ondisk_options,
    pub image_size: u64,
    pub snap_seq: u64,
    pub snap_count: u32,
    pub reserved: u32,
    pub snap_names_len: u64,
    pub snaps: [rbd_image_snap_ondisk; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
