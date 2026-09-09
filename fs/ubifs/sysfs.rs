// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file is part of UBIFS.
 *
 * Copyright (C) 2021 Cisco Systems
 *
 * Author: Stefan Schaeckeler
 */

// Dependencies supplied by the surrounding kernel/UBIFS translation.

#[repr(C)]
#[derive(Copy, Clone)]
enum AttrIdT {
    AttrErrorsMagic,
    AttrErrorsNode,
    AttrErrorsCrc,
}

#[repr(C)]
struct UbifsAttr {
    attr: Attribute,
    attr_id: AttrIdT,
}

// Direct translation of UBIFS_ATTR / UBIFS_ATTR_FUNC declarations.
static mut UBIFS_ATTR_ERRORS_MAGIC: UbifsAttr = UbifsAttr {
    attr: Attribute { name: b"errors_magic\0".as_ptr() as *const i8, mode: 0o444 },
    attr_id: AttrIdT::AttrErrorsMagic,
};
static mut UBIFS_ATTR_ERRORS_CRC: UbifsAttr = UbifsAttr {
    attr: Attribute { name: b"errors_crc\0".as_ptr() as *const i8, mode: 0o444 },
    attr_id: AttrIdT::AttrErrorsCrc,
};
static mut UBIFS_ATTR_ERRORS_NODE: UbifsAttr = UbifsAttr {
    attr: Attribute { name: b"errors_node\0".as_ptr() as *const i8, mode: 0o444 },
    attr_id: AttrIdT::AttrErrorsNode,
};

static mut UBIFS_ATTRS: [*mut Attribute; 4] = [
    unsafe { &raw mut UBIFS_ATTR_ERRORS_MAGIC.attr },
    unsafe { &raw mut UBIFS_ATTR_ERRORS_NODE.attr },
    unsafe { &raw mut UBIFS_ATTR_ERRORS_CRC.attr },
    core::ptr::null_mut(),
];

// ATTRIBUTE_GROUPS(ubifs)
static mut UBIFS_GROUPS: [*mut AttributeGroup; 2] = [
    unsafe { &raw mut UBIFS_GROUP },
    core::ptr::null_mut(),
];

unsafe fn ubifs_attr_show(
    kobj: *mut Kobject,
    attr: *mut Attribute,
    buf: *mut core::ffi::c_char,
) -> Isize {
    let sbi = container_of!(kobj, UbifsInfo, kobj);
    let a = container_of!(attr, UbifsAttr, attr);

    match (*a).attr_id {
        AttrIdT::AttrErrorsMagic => sysfs_emit!(buf, "%u\n", (*(*sbi).stats).magic_errors),
        AttrIdT::AttrErrorsNode => sysfs_emit!(buf, "%u\n", (*(*sbi).stats).node_errors),
        AttrIdT::AttrErrorsCrc => sysfs_emit!(buf, "%u\n", (*(*sbi).stats).crc_errors),
    }
}

unsafe fn ubifs_sb_release(kobj: *mut Kobject) {
    let c = container_of!(kobj, UbifsInfo, kobj);
    complete!(&mut (*c).kobj_unregister);
}

static UBIFS_ATTR_OPS: SysfsOps = SysfsOps {
    show: Some(ubifs_attr_show),
};

static UBIFS_SB_KTYPE: KobjType = KobjType {
    default_groups: unsafe { &raw const UBIFS_GROUPS[0] },
    sysfs_ops: &UBIFS_ATTR_OPS,
    release: Some(ubifs_sb_release),
};

static UBIFS_KTYPE: KobjType = KobjType {
    sysfs_ops: &UBIFS_ATTR_OPS,
};

static mut UBIFS_KSET: Kset = Kset {
    kobj: Kobject { ktype: &UBIFS_KTYPE },
};

pub unsafe fn ubifs_sysfs_register(c: *mut UbifsInfo) -> i32 {
    let mut ret: i32;
    let mut n: i32;
    let mut dfs_dir_name: [u8; UBIFS_DFS_DIR_LEN] = [0; UBIFS_DFS_DIR_LEN];

    (*c).stats = kzalloc_obj::<UbifsStatsInfo>();
    if (*c).stats.is_null() {
        ret = -ENOMEM;
        goto!(out_last);
    }
    n = snprintf!(
        dfs_dir_name.as_mut_ptr(),
        UBIFS_DFS_DIR_LEN,
        UBIFS_DFS_DIR_NAME,
        (*c).vi.ubi_num,
        (*c).vi.vol_id
    );

    if n >= UBIFS_DFS_DIR_LEN as i32 {
        // The array size is too small
        ret = -EINVAL;
        goto!(out_free);
    }

    (*c).kobj.kset = &mut UBIFS_KSET;
    init_completion!(&mut (*c).kobj_unregister);

    ret = kobject_init_and_add(
        &mut (*c).kobj,
        &UBIFS_SB_KTYPE,
        core::ptr::null_mut(),
        b"%s\0".as_ptr() as *const i8,
        dfs_dir_name.as_ptr(),
    );
    if ret != 0 {
        goto!(out_put);
    }

    return 0;

out_put:
    kobject_put(&mut (*c).kobj);
    wait_for_completion(&mut (*c).kobj_unregister);
out_free:
    kfree((*c).stats);
out_last:
    ubifs_err!(c, "cannot create sysfs entry for ubifs%d_%d, error %d\n", (*c).vi.ubi_num, (*c).vi.vol_id, ret);
    ret
}

pub unsafe fn ubifs_sysfs_unregister(c: *mut UbifsInfo) {
    kobject_del(&mut (*c).kobj);
    kobject_put(&mut (*c).kobj);
    wait_for_completion(&mut (*c).kobj_unregister);
    kfree((*c).stats);
}

pub unsafe fn ubifs_sysfs_init() -> i32 {
    let mut ret: i32;

    kobject_set_name(&mut UBIFS_KSET.kobj, b"ubifs\0".as_ptr() as *const i8);
    UBIFS_KSET.kobj.parent = fs_kobj;
    ret = kset_register(&mut UBIFS_KSET);
    if ret != 0 {
        kset_put(&mut UBIFS_KSET);
    }

    ret
}

pub unsafe fn ubifs_sysfs_exit() {
    kset_unregister(&mut UBIFS_KSET);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
