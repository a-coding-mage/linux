// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hpfs/dentry.c
 *
 *  Mikulas Patocka (mikulas@artax.karlin.mff.cuni.cz), 1998-1999
 *
 *  dcache operations
 */

// Dependency declarations formerly supplied by "hpfs_fn.h" are provided by
// the surrounding translation unit.

unsafe extern "C" {
    fn hpfs_adjust_length(name: *const core::ffi::c_char, length: *mut u32);
    fn init_name_hash(dentry: *const dentry) -> usize;
    fn partial_name_hash(value: u8, hash: usize) -> usize;
    fn end_name_hash(hash: usize) -> u32;
    fn hpfs_upcase(cp_table: *const u8, character: u8) -> u8;
    fn hpfs_sb(sb: *mut super_block) -> *mut hpfs_sb_info;
    fn hpfs_chk_name(name: *const core::ffi::c_char, length: *mut u32) -> i32;
    fn hpfs_compare_names(
        sb: *mut super_block,
        a: *const core::ffi::c_char,
        al: u32,
        b: *const core::ffi::c_char,
        bl: u32,
        case_sensitive: i32,
    ) -> i32;
}

/*
 * Note: the dentry argument is the parent dentry.
 */

unsafe extern "C" fn hpfs_hash_dentry(dentry: *const dentry, qstr: *mut qstr) -> i32 {
    let mut hash: usize;
    let mut i: i32;
    let mut l: u32 = (*qstr).len;

    if l == 1 {
        if (*qstr).name[0] == b'.' as core::ffi::c_char {
            // goto x
        } else {
            hpfs_adjust_length((*qstr).name, &mut l);
        }
    } else if l == 2 {
        if (*qstr).name[0] == b'.' as core::ffi::c_char
            || (*qstr).name[1] == b'.' as core::ffi::c_char
        {
            // goto x
        } else {
            hpfs_adjust_length((*qstr).name, &mut l);
        }
    } else {
        hpfs_adjust_length((*qstr).name, &mut l);
    }
    /*if (hpfs_chk_name(qstr->name,&l))*/
        /*return -ENAMETOOLONG;*/
        /*return -ENOENT;*/

    hash = init_name_hash(dentry);
    i = 0;
    while i < l as i32 {
        hash = partial_name_hash(
            hpfs_upcase(
                (*hpfs_sb((*dentry).d_sb)).sb_cp_table,
                (*qstr).name[i as usize] as u8,
            ),
            hash,
        );
        i += 1;
    }
    (*qstr).hash = end_name_hash(hash);

    0
}

unsafe extern "C" fn hpfs_compare_dentry(
    dentry: *const dentry,
    len: u32,
    str_: *const core::ffi::c_char,
    name: *const qstr,
) -> i32 {
    let mut al: u32 = len;
    let mut bl: u32 = (*name).len;

    hpfs_adjust_length(str_, &mut al);
    /*hpfs_adjust_length(b->name, &bl);*/

    /*
     * 'str' is the nane of an already existing dentry, so the name
     * must be valid. 'name' must be validated first.
     */

    if hpfs_chk_name((*name).name, &mut bl) != 0 {
        return 1;
    }
    if hpfs_compare_names((*dentry).d_sb, str_, al, (*name).name, bl, 0) != 0 {
        return 1;
    }
    0
}

pub static hpfs_dentry_operations: dentry_operations = dentry_operations {
    d_hash: Some(hpfs_hash_dentry),
    d_compare: Some(hpfs_compare_dentry),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
