// SPDX-License-Identifier: GPL-2.0
/* fs/sysfs/group.c - Operations for adding/removing multiple files at once. */

unsafe fn remove_files(parent: *mut kernfs_node, grp: *const attribute_group) {
    if !(*grp).attrs.is_null() {
        let mut attr = (*grp).attrs;
        while !(*attr).is_null() {
            kernfs_remove_by_name(parent, (**attr).name);
            attr = attr.add(1);
        }
    }
    if !(*grp).bin_attrs.is_null() {
        let mut bin_attr = (*grp).bin_attrs;
        while !(*bin_attr).is_null() {
            kernfs_remove_by_name(parent, (**bin_attr).attr.name);
            bin_attr = bin_attr.add(1);
        }
    }
}

unsafe fn __first_visible(grp: *const attribute_group, kobj: *mut kobject) -> umode_t {
    if !(*grp).attrs.is_null() && !(*(*grp).attrs).is_null() && !(*grp).is_visible.is_none() {
        return (*grp).is_visible.unwrap()(kobj, *(*grp).attrs, 0);
    }
    if !(*grp).attrs.is_null() && !(*(*grp).attrs).is_null() && !(*grp).is_visible_const.is_none() {
        return (*grp).is_visible_const.unwrap()(kobj, *(*grp).attrs, 0);
    }
    if !(*grp).bin_attrs.is_null() && !(*(*grp).bin_attrs).is_null() && !(*grp).is_bin_visible.is_none() {
        return (*grp).is_bin_visible.unwrap()(kobj, *(*grp).bin_attrs, 0);
    }
    0
}

unsafe fn create_files(parent: *mut kernfs_node, kobj: *mut kobject, uid: kuid_t, gid: kgid_t, grp: *const attribute_group, update: c_int) -> c_int {
    let mut error = 0;
    if !(*grp).attrs.is_null() {
        let mut attr = (*grp).attrs;
        let mut i = 0;
        while !(*attr).is_null() && error == 0 {
            let mut mode = (**attr).mode;
            if update != 0 { kernfs_remove_by_name(parent, (**attr).name); }
            if !(*grp).is_visible.is_none() || !(*grp).is_visible_const.is_none() {
                mode = if !(*grp).is_visible.is_none() { (*grp).is_visible.unwrap()(kobj, *attr, i) } else { (*grp).is_visible_const.unwrap()(kobj, *attr, i) };
                mode &= !SYSFS_GROUP_INVISIBLE;
                if mode == 0 { attr = attr.add(1); i += 1; continue; }
            }
            WARN(mode & !(SYSFS_PREALLOC | 0o664), "Attribute %s: Invalid permissions 0%o\n", (**attr).name, mode);
            mode &= SYSFS_PREALLOC | 0o664;
            error = sysfs_add_file_mode_ns(parent, *attr, mode, uid, gid, core::ptr::null_mut());
            attr = attr.add(1); i += 1;
        }
        if error != 0 { remove_files(parent, grp); return error; }
    }
    if !(*grp).bin_attrs.is_null() {
        let mut bin_attr = (*grp).bin_attrs;
        let mut i = 0;
        while !(*bin_attr).is_null() && error == 0 {
            let mut mode = (**bin_attr).attr.mode;
            let mut size = (**bin_attr).size;
            if update != 0 { kernfs_remove_by_name(parent, (**bin_attr).attr.name); }
            if !(*grp).is_bin_visible.is_none() {
                mode = (*grp).is_bin_visible.unwrap()(kobj, *bin_attr, i);
                mode &= !SYSFS_GROUP_INVISIBLE;
                if mode == 0 { bin_attr = bin_attr.add(1); i += 1; continue; }
            }
            if !(*grp).bin_size.is_none() { size = (*grp).bin_size.unwrap()(kobj, *bin_attr, i); }
            WARN(mode & !(SYSFS_PREALLOC | 0o664), "Attribute %s: Invalid permissions 0%o\n", (**bin_attr).attr.name, mode);
            mode &= SYSFS_PREALLOC | 0o664;
            error = sysfs_add_bin_file_mode_ns(parent, *bin_attr, mode, size, uid, gid, core::ptr::null_mut());
            bin_attr = bin_attr.add(1); i += 1;
        }
        if error != 0 { remove_files(parent, grp); }
    }
    error
}

unsafe fn internal_create_group(kobj: *mut kobject, update: c_int, grp: *const attribute_group) -> c_int {
    let mut kn: *mut kernfs_node;
    let mut uid = core::mem::zeroed(); let mut gid = core::mem::zeroed();
    if WARN_ON(kobj.is_null() || (update == 0 && (*kobj).sd.is_null())) { return -EINVAL; }
    if update != 0 && (*kobj).sd.is_null() { return -EINVAL; }
    if (*grp).attrs.is_null() && (*grp).bin_attrs.is_null() { pr_debug!("sysfs: (bin_)attrs not set by subsystem for group: %s/%s, skipping\n", (*kobj).name, if (*grp).name.is_null() { "" } else { (*grp).name }); return 0; }
    kobject_get_ownership(kobj, &mut uid, &mut gid);
    if !(*grp).name.is_null() {
        let mut mode = __first_visible(grp, kobj);
        mode = if mode & SYSFS_GROUP_INVISIBLE != 0 { 0 } else { S_IRWXU | S_IRUGO | S_IXUGO };
        if update != 0 { kn = kernfs_find_and_get((*kobj).sd, (*grp).name); if kn.is_null() { pr_debug!("attr grp %s/%s not created yet\n", (*kobj).name, (*grp).name); update = 0; } else if mode == 0 { sysfs_remove_group(kobj, grp); kernfs_put(kn); return 0; } }
        if update == 0 { if mode == 0 { return 0; } kn = kernfs_create_dir_ns((*kobj).sd, (*grp).name, mode, uid, gid, kobj, core::ptr::null_mut()); if IS_ERR(kn) { if PTR_ERR(kn) == -EEXIST { sysfs_warn_dup((*kobj).sd, (*grp).name); } return PTR_ERR(kn); } }
    } else { kn = (*kobj).sd; }
    kernfs_get(kn); let error = create_files(kn, kobj, uid, gid, grp, update);
    if error != 0 && !(*grp).name.is_null() && update == 0 { kernfs_remove(kn); }
    kernfs_put(kn); if !(*grp).name.is_null() && update != 0 { kernfs_put(kn); } error
}

pub unsafe fn sysfs_create_group(kobj: *mut kobject, grp: *const attribute_group) -> c_int { internal_create_group(kobj, 0, grp) }
pub unsafe fn sysfs_create_groups(kobj: *mut kobject, groups: *const *const attribute_group) -> c_int { internal_create_groups(kobj, 0, groups) }
pub unsafe fn sysfs_update_groups(kobj: *mut kobject, groups: *const *const attribute_group) -> c_int { internal_create_groups(kobj, 1, groups) }
pub unsafe fn sysfs_update_group(kobj: *mut kobject, grp: *const attribute_group) -> c_int { internal_create_group(kobj, 1, grp) }

unsafe fn internal_create_groups(kobj: *mut kobject, update: c_int, groups: *const *const attribute_group) -> c_int {
    if groups.is_null() { return 0; }
    let mut error = 0; let mut i = 0;
    while !(*groups.add(i)).is_null() { error = internal_create_group(kobj, update, *groups.add(i)); if error != 0 { while i > 0 { i -= 1; sysfs_remove_group(kobj, *groups.add(i)); } break; } i += 1; }
    error
}

pub unsafe fn sysfs_remove_group(kobj: *mut kobject, grp: *const attribute_group) { let parent = (*kobj).sd; let kn = if !(*grp).name.is_null() { let p = kernfs_find_and_get(parent, (*grp).name); if p.is_null() { pr_debug!("sysfs group '%s' not found for kobject '%s'\n", (*grp).name, kobject_name(kobj)); return; } p } else { kernfs_get(parent); parent }; remove_files(kn, grp); if !(*grp).name.is_null() { kernfs_remove(kn); } kernfs_put(kn); }
pub unsafe fn sysfs_remove_groups(kobj: *mut kobject, groups: *const *const attribute_group) { if groups.is_null() { return; } let mut i = 0; while !(*groups.add(i)).is_null() { sysfs_remove_group(kobj, *groups.add(i)); i += 1; } }

pub unsafe fn sysfs_merge_group(kobj: *mut kobject, grp: *const attribute_group) -> c_int { let parent = kernfs_find_and_get((*kobj).sd, (*grp).name); if parent.is_null() { return -ENOENT; } let mut uid = core::mem::zeroed(); let mut gid = core::mem::zeroed(); kobject_get_ownership(kobj, &mut uid, &mut gid); let mut error = 0; let mut i = 0; let mut attr = (*grp).attrs; while !(*attr).is_null() && error == 0 { error = sysfs_add_file_mode_ns(parent, *attr, (**attr).mode, uid, gid, core::ptr::null_mut()); attr = attr.add(1); i += 1; } if error != 0 { while i > 0 { i -= 1; attr = attr.sub(1); kernfs_remove_by_name(parent, (**attr).name); } } kernfs_put(parent); error }
pub unsafe fn sysfs_unmerge_group(kobj: *mut kobject, grp: *const attribute_group) { let parent = kernfs_find_and_get((*kobj).sd, (*grp).name); if !parent.is_null() { let mut attr = (*grp).attrs; while !(*attr).is_null() { kernfs_remove_by_name(parent, (**attr).name); attr = attr.add(1); } kernfs_put(parent); } }
pub unsafe fn sysfs_add_link_to_group(kobj: *mut kobject, group_name: *const c_char, target: *mut kobject, link_name: *const c_char) -> c_int { let parent = kernfs_find_and_get((*kobj).sd, group_name); if parent.is_null() { return -ENOENT; } let error = sysfs_create_link_sd(parent, target, link_name); kernfs_put(parent); error }
pub unsafe fn sysfs_remove_link_from_group(kobj: *mut kobject, group_name: *const c_char, link_name: *const c_char) { let parent = kernfs_find_and_get((*kobj).sd, group_name); if !parent.is_null() { kernfs_remove_by_name(parent, link_name); kernfs_put(parent); } }

pub unsafe fn compat_only_sysfs_link_entry_to_kobj(kobj: *mut kobject, target_kobj: *mut kobject, target_name: *const c_char, mut symlink_name: *const c_char) -> c_int { spin_lock(&mut sysfs_symlink_target_lock); let target = (*target_kobj).sd; if !target.is_null() { kernfs_get(target); } spin_unlock(&mut sysfs_symlink_target_lock); if target.is_null() { return -ENOENT; } let entry = kernfs_find_and_get(target, target_name); if entry.is_null() { kernfs_put(target); return -ENOENT; } if symlink_name.is_null() { symlink_name = target_name; } let link = kernfs_create_link((*kobj).sd, symlink_name, entry); if PTR_ERR(link) == -EEXIST { sysfs_warn_dup((*kobj).sd, symlink_name); } kernfs_put(entry); kernfs_put(target); PTR_ERR_OR_ZERO(link) }

unsafe fn sysfs_group_attrs_change_owner(kobj: *mut kobject, grp_kn: *mut kernfs_node, grp: *const attribute_group, newattrs: *mut iattr) -> c_int { let mut mode; if !(*grp).attrs.is_null() { let mut a = (*grp).attrs; let mut i = 0; while !(*a).is_null() { if !(*grp).is_visible.is_none() || !(*grp).is_visible_const.is_none() { mode = if !(*grp).is_visible.is_none() { (*grp).is_visible.unwrap()(kobj, *a, i) } else { (*grp).is_visible_const.unwrap()(kobj, *a, i) }; if mode & SYSFS_GROUP_INVISIBLE != 0 { break; } if mode == 0 { a = a.add(1); i += 1; continue; } } let kn = kernfs_find_and_get(grp_kn, (**a).name); if kn.is_null() { return -ENOENT; } let error = kernfs_setattr(kn, newattrs); kernfs_put(kn); if error != 0 { return error; } a = a.add(1); i += 1; } } if !(*grp).bin_attrs.is_null() { let mut a = (*grp).bin_attrs; let mut i = 0; while !(*a).is_null() { if !(*grp).is_bin_visible.is_none() { mode = (*grp).is_bin_visible.unwrap()(kobj, *a, i); if mode & SYSFS_GROUP_INVISIBLE != 0 { break; } if mode == 0 { a = a.add(1); i += 1; continue; } } let kn = kernfs_find_and_get(grp_kn, (**a).attr.name); if kn.is_null() { return -ENOENT; } let error = kernfs_setattr(kn, newattrs); kernfs_put(kn); if error != 0 { return error; } a = a.add(1); i += 1; } } 0 }

pub unsafe fn sysfs_group_change_owner(kobj: *mut kobject, grp: *const attribute_group, kuid: kuid_t, kgid: kgid_t) -> c_int { if !(*kobj).state_in_sysfs { return -EINVAL; } let grp_kn = if !(*grp).name.is_null() { kernfs_find_and_get((*kobj).sd, (*grp).name) } else { kernfs_get((*kobj).sd); (*kobj).sd }; if grp_kn.is_null() { return -ENOENT; } let mut newattrs: iattr = core::mem::zeroed(); newattrs.ia_valid = ATTR_UID | ATTR_GID; newattrs.ia_uid = kuid; newattrs.ia_gid = kgid; let mut error = kernfs_setattr(grp_kn, &mut newattrs); if error == 0 { error = sysfs_group_attrs_change_owner(kobj, grp_kn, grp, &mut newattrs); } kernfs_put(grp_kn); error }
pub unsafe fn sysfs_groups_change_owner(kobj: *mut kobject, groups: *const *const attribute_group, kuid: kuid_t, kgid: kgid_t) -> c_int { if !(*kobj).state_in_sysfs { return -EINVAL; } if groups.is_null() { return 0; } let mut error = 0; let mut i = 0; while !(*groups.add(i)).is_null() { error = sysfs_group_change_owner(kobj, *groups.add(i), kuid, kgid); if error != 0 { break; } i += 1; } error }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
