// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * SPU file system
 *
 * (C) Copyright IBM Deutschland Entwicklung GmbH 2005
 *
 * Author: Arnd Bergmann <arndb@de.ibm.com>
 */

// Translated from the C implementation.  The Linux list, slab, and spufs
// definitions are supplied by the surrounding kernel translation.
use crate::spufs::{
    container_of, kfree, kref_get, kref_init, kref_put,
    kzalloc_obj, list_add, list_del_init, list_empty, mutex_init, mutex_lock,
    mutex_unlock, warn_on, AffList, Kref, ListHead, Mutex, SpuContext, SpuGang,
    AFF_OFFSETS_SET,
};

pub unsafe fn alloc_spu_gang() -> *mut SpuGang {
    let gang: *mut SpuGang = kzalloc_obj::<SpuGang>();
    if gang.is_null() {
        return gang;
    }

    kref_init(&mut (*gang).kref);
    mutex_init(&mut (*gang).mutex);
    mutex_init(&mut (*gang).aff_mutex);
    crate::spufs::init_list_head(&mut (*gang).list);
    crate::spufs::init_list_head(&mut (*gang).aff_list_head);
    (*gang).alive = 1;

    gang
}

unsafe fn destroy_spu_gang(kref: *mut Kref) {
    let gang: *mut SpuGang = container_of!(kref, SpuGang, kref);
    warn_on((*gang).contexts != 0 || !list_empty(&(*gang).list));
    kfree(gang);
}

pub unsafe fn get_spu_gang(gang: *mut SpuGang) -> *mut SpuGang {
    kref_get(&mut (*gang).kref);
    gang
}

pub unsafe fn put_spu_gang(gang: *mut SpuGang) -> i32 {
    kref_put(&mut (*gang).kref, destroy_spu_gang)
}

pub unsafe fn spu_gang_add_ctx(gang: *mut SpuGang, ctx: *mut SpuContext) {
    mutex_lock(&mut (*gang).mutex);
    (*ctx).gang = get_spu_gang(gang);
    list_add(&mut (*ctx).gang_list, &mut (*gang).list);
    (*gang).contexts = (*gang).contexts.wrapping_add(1);
    mutex_unlock(&mut (*gang).mutex);
}

pub unsafe fn spu_gang_remove_ctx(gang: *mut SpuGang, ctx: *mut SpuContext) {
    mutex_lock(&mut (*gang).mutex);
    warn_on((*ctx).gang != gang);
    if !list_empty(&(*ctx).aff_list) {
        list_del_init(&mut (*ctx).aff_list);
        (*gang).aff_flags &= !AFF_OFFSETS_SET;
    }
    list_del_init(&mut (*ctx).gang_list);
    (*gang).contexts = (*gang).contexts.wrapping_sub(1);
    mutex_unlock(&mut (*gang).mutex);

    put_spu_gang(gang);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
