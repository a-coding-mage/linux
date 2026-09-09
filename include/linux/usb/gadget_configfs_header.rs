/* SPDX-License-Identifier: GPL-2.0 */

// Original dependency: <linux/configfs.h>

/*
 * Rust translations of the C preprocessor helpers.  Rust has no stable
 * identifier token-pasting facility, so the generated function/item names are
 * supplied explicitly by the invoker.
 */
macro_rules! GS_STRINGS_W {
    ($struct_ty:ty, $name:ident, $store_fn:ident, $to_struct:expr) => {
        unsafe fn $store_fn(
            item: *mut config_item,
            page: *const core::ffi::c_char,
            len: usize,
        ) -> isize {
            let gs: *mut $struct_ty = $to_struct(item);
            let ret: i32 = usb_string_copy(page, unsafe { &mut (*gs).$name });
            if ret != 0 {
                return ret as isize;
            }
            len as isize
        }
    };
}

macro_rules! GS_STRINGS_R {
    ($struct_ty:ty, $name:ident, $show_fn:ident, $to_struct:expr) => {
        unsafe fn $show_fn(
            item: *mut config_item,
            page: *mut core::ffi::c_char,
        ) -> isize {
            let gs: *mut $struct_ty = $to_struct(item);
            // Equivalent to sprintf(page, "%s\n", gs->$name ?: "").
            sprintf_string(page, unsafe { (*gs).$name })
        }
    };
}

macro_rules! GS_STRINGS_RW {
    ($struct_ty:ty, $name:ident, $show_fn:ident, $store_fn:ident, $to_struct:expr, $attr:ident) => {
        GS_STRINGS_R!($struct_ty, $name, $show_fn, $to_struct);
        GS_STRINGS_W!($struct_ty, $name, $store_fn, $to_struct);
        CONFIGFS_ATTR!($attr);
    };
}

macro_rules! USB_CONFIG_STRING_RW_OPS {
    ($struct_in:ident, $item_ops:ident, $item_type:ident, $attr_release:ident, $attrs:ident) => {
        static $item_ops: configfs_item_operations = configfs_item_operations {
            release: Some($attr_release),
        };

        static $item_type: config_item_type = config_item_type {
            ct_item_ops: &$item_ops,
            ct_attrs: $attrs,
            ct_owner: THIS_MODULE,
        };
    };
}

macro_rules! USB_CONFIG_STRINGS_LANG {
    ($struct_in:ident, $struct_member:ty, $make_fn:ident, $drop_fn:ident,
     $ops:ident, $type_name:ident, $langid_type:expr) => {
        unsafe fn $make_fn(
            group: *mut config_group,
            name: *const core::ffi::c_char,
        ) -> *mut config_group {
            let mut langs: i32 = 0;
            let new: *mut $struct_in = kzalloc(
                core::mem::size_of::<$struct_in>(),
                GFP_KERNEL,
            );
            if new.is_null() {
                return ERR_PTR(-ENOMEM);
            }

            let ret = check_user_usb_string(name, unsafe { &mut (*new).stringtab_dev });
            if ret != 0 {
                unsafe { kfree(new as *mut core::ffi::c_void) };
                return ERR_PTR(ret);
            }
            unsafe {
                config_group_init_type_name(&mut (*new).group, name, $langid_type);
            }

            let gi: *mut $struct_member = container_of(group, strings_group);
            let mut ret: i32 = -EEXIST;
            let mut gs = unsafe { (*gi).string_list.next };
            while gs != unsafe { &(*gi).string_list as *const _ as *mut _ } {
                let entry: *mut $struct_in = container_of(gs, list);
                if unsafe { (*entry).stringtab_dev.language == (*new).stringtab_dev.language } {
                    unsafe { kfree(new as *mut core::ffi::c_void) };
                    return ERR_PTR(ret);
                }
                langs += 1;
                gs = unsafe { (*gs).next };
            }
            ret = -EOVERFLOW;
            if langs >= MAX_USB_STRING_LANGS {
                unsafe { kfree(new as *mut core::ffi::c_void) };
                return ERR_PTR(ret);
            }

            unsafe { list_add_tail(&mut (*new).list, &mut (*gi).string_list) };
            unsafe { &mut (*new).group }
        }

        unsafe fn $drop_fn(_group: *mut config_group, item: *mut config_item) {
            config_item_put(item);
        }

        static $ops: configfs_group_operations = configfs_group_operations {
            make_group: Some($make_fn),
            drop_item: Some($drop_fn),
        };

        static $type_name: config_item_type = config_item_type {
            ct_group_ops: &$ops,
            ct_owner: THIS_MODULE,
        };
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
