// SPDX-License-Identifier: GPL-2.0
/*
 * Init code for a livepatch kernel module
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Dependencies supplied by the kernel and other translation units.
#[repr(C)]
pub struct module;

#[repr(C)]
pub struct klp_callbacks {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct klp_func {
    pub old_name: *const c_char,
    pub new_func: *mut c_void,
    pub old_sympos: c_uint,
}

#[repr(C)]
pub struct klp_object {
    pub name: *const c_char,
    pub funcs: *mut klp_func,
    pub callbacks: klp_callbacks,
}

#[repr(C)]
pub struct klp_patch {
    pub mod_: *mut module,
    pub objs: *mut klp_object,
    pub replace: bool,
}

#[repr(C)]
pub struct klp_func_ext {
    pub old_name: *const c_char,
    pub new_func: *mut c_void,
    pub sympos: c_uint,
}

#[repr(C)]
pub struct klp_object_ext {
    pub funcs: *mut klp_func_ext,
    pub nr_funcs: c_uint,
    pub name: *const c_char,
    pub callbacks: klp_callbacks,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static mut patch: *mut klp_patch;

    fn klp_find_section_by_name(
        module: *mut module,
        name: *const c_char,
        size: *mut usize,
    ) -> *mut klp_object_ext;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn klp_enable_patch(patch: *mut klp_patch) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
}

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

unsafe fn livepatch_mod_init() -> c_int {
    let mut obj_exts: *mut klp_object_ext;
    let mut obj_exts_sec_size: usize = 0;
    let objs: *mut klp_object;
    let mut nr_objs: c_uint;
    let mut ret: c_int;

    obj_exts = klp_find_section_by_name(
        THIS_MODULE,
        b".init.klp_objects\0".as_ptr() as *const c_char,
        &mut obj_exts_sec_size,
    );
    nr_objs = (obj_exts_sec_size / core::mem::size_of::<klp_object_ext>()) as c_uint;
    if nr_objs == 0 {
        pr_err(b"nothing to patch!\n\0".as_ptr() as *const c_char);
        ret = -EINVAL;
        return ret;
    }

    patch = kzalloc(core::mem::size_of::<klp_patch>(), GFP_KERNEL) as *mut klp_patch;
    if patch.is_null() {
        ret = -ENOMEM;
        return ret;
    }

    objs = kzalloc(
        core::mem::size_of::<klp_object>() * (nr_objs as usize + 1),
        GFP_KERNEL,
    ) as *mut klp_object;
    if objs.is_null() {
        ret = -ENOMEM;
        kfree(patch as *mut c_void);
        return ret;
    }

    for i in 0..nr_objs as usize {
        let obj_ext = obj_exts.add(i);
        let funcs_ext = (*obj_ext).funcs;
        let nr_funcs = (*obj_ext).nr_funcs;
        let mut funcs = (*objs.add(i)).funcs;
        let obj = objs.add(i);

        funcs = kzalloc(
            core::mem::size_of::<klp_func>() * (nr_funcs as usize + 1),
            GFP_KERNEL,
        ) as *mut klp_func;
        if funcs.is_null() {
            ret = -ENOMEM;
            for j in 0..i {
                kfree((*objs.add(i)).funcs as *mut c_void);
            }
            kfree(objs as *mut c_void);
            kfree(patch as *mut c_void);
            return ret;
        }

        for j in 0..nr_funcs as usize {
            (*funcs.add(j)).old_name = (*funcs_ext.add(j)).old_name;
            (*funcs.add(j)).new_func = (*funcs_ext.add(j)).new_func;
            (*funcs.add(j)).old_sympos = (*funcs_ext.add(j)).sympos;
        }

        (*obj).name = (*obj_ext).name;
        (*obj).funcs = funcs;
        core::ptr::copy_nonoverlapping(
            &(*obj_ext).callbacks,
            &mut (*obj).callbacks,
            1,
        );
    }

    (*patch).mod_ = THIS_MODULE;
    (*patch).objs = objs;

    /* TODO patch->states */

    #[cfg(not(KLP_NO_REPLACE))]
    {
        (*patch).replace = true;
    }
    #[cfg(KLP_NO_REPLACE)]
    {
        (*patch).replace = false;
    }

    klp_enable_patch(patch)
}

unsafe fn livepatch_mod_exit() {
    // klp_for_each_object_static(patch, obj)
    //     kfree(obj->funcs);
    // The iteration is supplied by the livepatch dependency.
    let mut obj = (*patch).objs;
    while !obj.is_null() && !(*obj).name.is_null() {
        kfree((*obj).funcs as *mut c_void);
        obj = obj.add(1);
    }

    kfree((*patch).objs as *mut c_void);
    kfree(patch as *mut c_void);
}

// module_init(livepatch_mod_init);
// module_exit(livepatch_mod_exit);
// MODULE_LICENSE("GPL");
// MODULE_INFO(livepatch, "Y");
// MODULE_DESCRIPTION("Livepatch module");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
