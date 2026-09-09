// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Module sysfs support
 *
 * Copyright (C) 2008 Rusty Russell
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left as external Rust names.

/*
 * /sys/module/foo/sections stuff
 * J. Corbet <corbet@lwn.net>
 */
#[cfg(feature = "CONFIG_KALLSYMS")]
#[repr(C)]
struct module_sect_attrs {
    grp: attribute_group,
    attrs: [bin_attribute; 0],
}

#[cfg(feature = "CONFIG_KALLSYMS")]
const MODULE_SECT_READ_SIZE: usize = 3 + (BITS_PER_LONG / 4);

#[cfg(feature = "CONFIG_KALLSYMS")]
unsafe fn module_sect_read(
    file: *mut file,
    _kobj: *mut kobject,
    battr: *const bin_attribute,
    buf: *mut c_char,
    pos: loff_t,
    mut count: usize,
) -> isize {
    let mut bounce = [0 as c_char; MODULE_SECT_READ_SIZE + 1];
    let wrote: usize;

    if pos != 0 {
        return -EINVAL as isize;
    }

    /*
     * Since we're a binary read handler, we must account for the
     * trailing NUL byte that sprintf will write: if "buf" is
     * too small to hold the NUL, or the NUL is exactly the last
     * byte, the read will look like it got truncated by one byte.
     * Since there is no way to ask sprintf nicely to not write the
     * NUL, we have to use a bounce buffer.
     */
    wrote = scnprintf(
        bounce.as_mut_ptr(),
        bounce.len(),
        c"0x%px\n".as_ptr(),
        if kallsyms_show_value((*file).f_cred) {
            (*battr).private
        } else {
            core::ptr::null_mut()
        },
    ) as usize;
    count = core::cmp::min(count, wrote);
    memcpy(buf, bounce.as_ptr(), count);

    count as isize
}

#[cfg(feature = "CONFIG_KALLSYMS")]
unsafe fn free_sect_attrs(sect_attrs: *mut module_sect_attrs) {
    let mut bin_attr = (*sect_attrs).grp.bin_attrs;

    while !(*bin_attr).is_null() {
        kfree((**bin_attr).attr.name as *mut c_void);
        bin_attr = bin_attr.add(1);
    }
    kfree((*sect_attrs).grp.bin_attrs as *mut c_void);
    kfree(sect_attrs as *mut c_void);
}

#[cfg(feature = "CONFIG_KALLSYMS")]
unsafe fn add_sect_attrs(mod_: *mut module, info: *const load_info) -> c_int {
    let mut nloaded: u32 = 0;
    let mut i: u32;
    let sect_attrs: *mut module_sect_attrs;
    let gattr: *mut *const bin_attribute;
    let mut sattr: *mut bin_attribute;
    let ret: c_int;

    /* Count loaded sections and allocate structures */
    i = 0;
    while i < (*(*info).hdr).e_shnum {
        if !sect_empty((*info).sechdrs.add(i as usize)) {
            nloaded += 1;
        }
        i += 1;
    }
    sect_attrs = kzalloc_flex::<module_sect_attrs>(nloaded);
    if sect_attrs.is_null() {
        return -ENOMEM;
    }

    gattr = kzalloc_objs::<*const bin_attribute>(nloaded + 1);
    if gattr.is_null() {
        kfree(sect_attrs as *mut c_void);
        return -ENOMEM;
    }

    (*sect_attrs).grp.name = c"sections".as_ptr();
    (*sect_attrs).grp.bin_attrs = gattr;

    sattr = (*sect_attrs).attrs.as_mut_ptr();
    i = 0;
    while i < (*(*info).hdr).e_shnum {
        let sec = (*info).sechdrs.add(i as usize);
        if !sect_empty(sec) {
            sysfs_bin_attr_init(sattr);
            (*sattr).attr.name = kstrdup(
                (*info).secstrings.add((*sec).sh_name as usize),
                GFP_KERNEL,
            );
            if (*sattr).attr.name.is_null() {
                ret = -ENOMEM;
                free_sect_attrs(sect_attrs);
                return ret;
            }
            (*sattr).read = Some(module_sect_read);
            (*sattr).private = (*sec).sh_addr as *mut c_void;
            (*sattr).size = MODULE_SECT_READ_SIZE as u64;
            (*sattr).attr.mode = 0o400;
            *gattr = sattr;
            sattr = sattr.add(1);
            gattr = gattr.add(1);
        }
        i += 1;
    }

    ret = sysfs_create_group(&mut (*mod_).mkobj.kobj, &mut (*sect_attrs).grp);
    if ret != 0 {
        free_sect_attrs(sect_attrs);
        return ret;
    }
    (*mod_).sect_attrs = sect_attrs;
    0
}

#[cfg(feature = "CONFIG_KALLSYMS")]
unsafe fn remove_sect_attrs(mod_: *mut module) {
    if !(*mod_).sect_attrs.is_null() {
        sysfs_remove_group(&mut (*mod_).mkobj.kobj, &mut (*(*mod_).sect_attrs).grp);
        free_sect_attrs((*mod_).sect_attrs);
        (*mod_).sect_attrs = core::ptr::null_mut();
    }
}

#[cfg(feature = "CONFIG_KALLSYMS")]
#[repr(C)]
struct module_notes_attrs {
    grp: attribute_group,
    attrs: [bin_attribute; 0],
}

#[cfg(feature = "CONFIG_KALLSYMS")]
unsafe fn free_notes_attrs(notes_attrs: *mut module_notes_attrs) {
    kfree((*notes_attrs).grp.bin_attrs as *mut c_void);
    kfree(notes_attrs as *mut c_void);
}

#[cfg(feature = "CONFIG_KALLSYMS")]
unsafe fn add_notes_attrs(mod_: *mut module, info: *const load_info) -> c_int {
    let mut notes = 0u32;
    let mut i: u32;
    let mut loaded: u32;
    let notes_attrs: *mut module_notes_attrs;
    let mut gattr: *mut *const bin_attribute;
    let mut nattr: *mut bin_attribute;
    let ret: c_int;

    /* Count notes sections and allocate structures.  */
    i = 0;
    while i < (*(*info).hdr).e_shnum {
        let sec = (*info).sechdrs.add(i as usize);
        if !sect_empty(sec) && (*sec).sh_type == SHT_NOTE {
            notes += 1;
        }
        i += 1;
    }
    if notes == 0 { return 0; }

    notes_attrs = kzalloc_flex::<module_notes_attrs>(notes);
    if notes_attrs.is_null() { return -ENOMEM; }
    gattr = kzalloc_objs::<*const bin_attribute>(notes + 1);
    if gattr.is_null() {
        kfree(notes_attrs as *mut c_void);
        return -ENOMEM;
    }
    (*notes_attrs).grp.name = c"notes".as_ptr();
    (*notes_attrs).grp.bin_attrs = gattr;
    nattr = (*notes_attrs).attrs.as_mut_ptr();
    loaded = 0;
    i = 0;
    while i < (*(*info).hdr).e_shnum {
        let sec = (*info).sechdrs.add(i as usize);
        if sect_empty(sec) { i += 1; continue; }
        if (*sec).sh_type == SHT_NOTE {
            sysfs_bin_attr_init(nattr);
            (*nattr).attr.name = (*mod_).sect_attrs.add(loaded as usize).attr.name;
            (*nattr).attr.mode = 0o444;
            (*nattr).size = (*sec).sh_size;
            (*nattr).private = (*sec).sh_addr as *mut c_void;
            (*nattr).read = Some(sysfs_bin_attr_simple_read);
            *gattr = nattr;
            gattr = gattr.add(1);
            nattr = nattr.add(1);
        }
        loaded += 1;
        i += 1;
    }
    ret = sysfs_create_group(&mut (*mod_).mkobj.kobj, &mut (*notes_attrs).grp);
    if ret != 0 { free_notes_attrs(notes_attrs); return ret; }
    (*mod_).notes_attrs = notes_attrs;
    0
}

#[cfg(feature = "CONFIG_KALLSYMS")]
unsafe fn remove_notes_attrs(mod_: *mut module) {
    if !(*mod_).notes_attrs.is_null() {
        sysfs_remove_group(&mut (*mod_).mkobj.kobj, &mut (*(*mod_).notes_attrs).grp);
        free_notes_attrs((*mod_).notes_attrs);
        (*mod_).notes_attrs = core::ptr::null_mut();
    }
}

#[cfg(not(feature = "CONFIG_KALLSYMS"))]
unsafe fn add_sect_attrs(_mod_: *mut module, _info: *const load_info) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_KALLSYMS"))]
unsafe fn remove_sect_attrs(_mod_: *mut module) {}
#[cfg(not(feature = "CONFIG_KALLSYMS"))]
unsafe fn add_notes_attrs(_mod_: *mut module, _info: *const load_info) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_KALLSYMS"))]
unsafe fn remove_notes_attrs(_mod_: *mut module) {}

unsafe fn del_usage_links(mod_: *mut module) {
    #[cfg(feature = "CONFIG_MODULE_UNLOAD")]
    {
        let mut use_: *mut module_use;
        mutex_lock(&mut module_mutex);
        list_for_each_entry!(use_, &mut (*mod_).target_list, target_list, {
            sysfs_remove_link((*use_).target.holders_dir, (*mod_).name);
        });
        mutex_unlock(&mut module_mutex);
    }
}

unsafe fn add_usage_links(mod_: *mut module) -> c_int {
    let mut ret = 0;
    #[cfg(feature = "CONFIG_MODULE_UNLOAD")]
    {
        let mut use_: *mut module_use;
        mutex_lock(&mut module_mutex);
        list_for_each_entry!(use_, &mut (*mod_).target_list, target_list, {
            ret = sysfs_create_link((*use_).target.holders_dir, &mut (*mod_).mkobj.kobj, (*mod_).name);
            if ret != 0 { break; }
        });
        mutex_unlock(&mut module_mutex);
        if ret != 0 { del_usage_links(mod_); }
    }
    ret
}

unsafe fn module_remove_modinfo_attrs(mod_: *mut module, end: c_int) {
    let mut i = 0;
    loop {
        let attr = (*mod_).modinfo_attrs.add(i);
        if end >= 0 && i as c_int > end { break; }
        if (*attr).attr.name.is_null() { break; }
        sysfs_remove_file(&mut (*mod_).mkobj.kobj, &(*attr).attr);
        if let Some(free) = (*attr).free { free(mod_); }
        i += 1;
    }
    kfree((*mod_).modinfo_attrs as *mut c_void);
}

unsafe fn module_add_modinfo_attrs(mod_: *mut module) -> c_int {
    let mut error = 0;
    let mut i = 0;
    (*mod_).modinfo_attrs = kzalloc_array::<module_attribute>(modinfo_attrs_count + 1, GFP_KERNEL);
    if (*mod_).modinfo_attrs.is_null() { return -ENOMEM; }
    let mut temp_attr = (*mod_).modinfo_attrs;
    while !(*modinfo_attrs.add(i)).is_null() {
        let attr = *modinfo_attrs.add(i);
        if (*attr).test.is_none() || (*attr).test.unwrap()(mod_) {
            memcpy(temp_attr as *mut c_void, attr as *const c_void, core::mem::size_of::<module_attribute>());
            sysfs_attr_init(&mut (*temp_attr).attr);
            error = sysfs_create_file(&mut (*mod_).mkobj.kobj, &(*temp_attr).attr);
            if error != 0 { break; }
            temp_attr = temp_attr.add(1);
        }
        i += 1;
    }
    if error == 0 { return 0; }
    if i > 0 { module_remove_modinfo_attrs(mod_, (i - 1) as c_int); }
    else { kfree((*mod_).modinfo_attrs as *mut c_void); }
    error
}

unsafe fn mod_kobject_put(mod_: *mut module) {
    let mut c = completion::default();
    (*mod_).mkobj.kobj_completion = &mut c;
    kobject_put(&mut (*mod_).mkobj.kobj);
    wait_for_completion(&mut c);
}

unsafe fn mod_sysfs_init(mod_: *mut module) -> c_int {
    let err: c_int;
    let kobj: *mut kobject;
    if module_kset.is_null() {
        pr_err(c"%s: module sysfs not initialized\n".as_ptr(), (*mod_).name);
        return -EINVAL;
    }
    kobj = kset_find_obj(module_kset, (*mod_).name);
    if !kobj.is_null() {
        pr_err(c"%s: module is already loaded\n".as_ptr(), (*mod_).name);
        kobject_put(kobj);
        return -EINVAL;
    }
    (*mod_).mkobj.mod_ = mod_;
    core::ptr::write_bytes(&mut (*mod_).mkobj.kobj as *mut kobject as *mut u8, 0, core::mem::size_of::<kobject>());
    (*mod_).mkobj.kobj.kset = module_kset;
    err = kobject_init_and_add(&mut (*mod_).mkobj.kobj, &module_ktype, core::ptr::null_mut(), c"%s".as_ptr(), (*mod_).name);
    if err != 0 { mod_kobject_put(mod_); }
    err
}

pub unsafe fn mod_sysfs_setup(mod_: *mut module, info: *const load_info, kparam: *mut kernel_param, num_params: u32) -> c_int {
    let mut err = mod_sysfs_init(mod_);
    if err != 0 { return err; }
    (*mod_).holders_dir = kobject_create_and_add(c"holders".as_ptr(), &mut (*mod_).mkobj.kobj);
    if (*mod_).holders_dir.is_null() { err = -ENOMEM; mod_kobject_put(mod_); return err; }
    err = module_param_sysfs_setup(mod_, kparam, num_params);
    if err != 0 { kobject_put((*mod_).holders_dir); mod_kobject_put(mod_); return err; }
    err = module_add_modinfo_attrs(mod_);
    if err != 0 { module_param_sysfs_remove(mod_); kobject_put((*mod_).holders_dir); mod_kobject_put(mod_); return err; }
    err = add_usage_links(mod_);
    if err != 0 { module_remove_modinfo_attrs(mod_, -1); module_param_sysfs_remove(mod_); kobject_put((*mod_).holders_dir); mod_kobject_put(mod_); return err; }
    err = add_sect_attrs(mod_, info);
    if err != 0 { del_usage_links(mod_); module_remove_modinfo_attrs(mod_, -1); module_param_sysfs_remove(mod_); kobject_put((*mod_).holders_dir); mod_kobject_put(mod_); return err; }
    err = add_notes_attrs(mod_, info);
    if err != 0 { remove_sect_attrs(mod_); del_usage_links(mod_); module_remove_modinfo_attrs(mod_, -1); module_param_sysfs_remove(mod_); kobject_put((*mod_).holders_dir); mod_kobject_put(mod_); return err; }
    0
}

unsafe fn mod_sysfs_fini(mod_: *mut module) {
    remove_notes_attrs(mod_);
    remove_sect_attrs(mod_);
    mod_kobject_put(mod_);
}

pub unsafe fn mod_sysfs_teardown(mod_: *mut module) {
    del_usage_links(mod_);
    module_remove_modinfo_attrs(mod_, -1);
    module_param_sysfs_remove(mod_);
    kobject_put((*mod_).mkobj.drivers_dir);
    kobject_put((*mod_).holders_dir);
    mod_sysfs_fini(mod_);
}

pub unsafe fn init_param_lock(mod_: *mut module) {
    mutex_init(&mut (*mod_).param_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
