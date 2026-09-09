/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2017, Mellanox Technologies inc.  All rights reserved.
 */

// Translated from the C header. External types, functions, constants, and
// macros are supplied by the corresponding RDMA dependencies.

#[macro_export]
macro_rules! _uobj_check_id {
    ($id:expr) => {{ ($id) * typecheck::<u32, _>($id) }};
}

#[macro_export]
macro_rules! uobj_get_type {
    ($attrs:expr, $object:expr) => {{ uapi_get_object(($attrs).ufile.device.uapi, $object) }};
}

#[macro_export]
macro_rules! uobj_get_read {
    ($type:expr, $id:expr, $attrs:expr) => {{
        rdma_lookup_get_uobject(
            uobj_get_type!($attrs, $type),
            ($attrs).ufile,
            _uobj_check_id!($id),
            UVERBS_LOOKUP_READ,
            $attrs,
        )
    }};
}

#[macro_export]
macro_rules! ufd_get_read {
    ($type:expr, $fdnum:expr, $attrs:expr) => {{
        rdma_lookup_get_uobject(
            uobj_get_type!($attrs, $type),
            ($attrs).ufile,
            ($fdnum) * typecheck::<i32, _>($fdnum),
            UVERBS_LOOKUP_READ,
            $attrs,
        )
    }};
}

#[inline]
pub unsafe fn _uobj_get_obj_read(uobj: *mut ib_uobject) -> *mut core::ffi::c_void {
    if IS_ERR(uobj) {
        return ERR_CAST(uobj);
    }
    (*uobj).object
}

#[macro_export]
macro_rules! uobj_get_obj_read {
    ($object:ident, $type:expr, $id:expr, $attrs:expr) => {{
        _uobj_get_obj_read(uobj_get_read!($type, $id, $attrs)) as *mut ib_$object
    }};
}

#[macro_export]
macro_rules! uobj_get_write {
    ($type:expr, $id:expr, $attrs:expr) => {{
        rdma_lookup_get_uobject(
            uobj_get_type!($attrs, $type),
            ($attrs).ufile,
            _uobj_check_id!($id),
            UVERBS_LOOKUP_WRITE,
            $attrs,
        )
    }};
}

pub unsafe extern "C" fn __uobj_perform_destroy(
    obj: *const uverbs_api_object,
    id: u32,
    attrs: *mut uverbs_attr_bundle,
) -> i32;

#[macro_export]
macro_rules! uobj_perform_destroy {
    ($type:expr, $id:expr, $attrs:expr) => {{
        __uobj_perform_destroy(uobj_get_type!($attrs, $type), _uobj_check_id!($id), $attrs)
    }};
}

pub unsafe extern "C" fn __uobj_get_destroy(
    obj: *const uverbs_api_object,
    id: u32,
    attrs: *mut uverbs_attr_bundle,
) -> *mut ib_uobject;

#[macro_export]
macro_rules! uobj_get_destroy {
    ($type:expr, $id:expr, $attrs:expr) => {{
        __uobj_get_destroy(uobj_get_type!($attrs, $type), _uobj_check_id!($id), $attrs)
    }};
}

#[inline]
pub unsafe fn uobj_put_destroy(uobj: *mut ib_uobject) {
    rdma_lookup_put_uobject(uobj, UVERBS_LOOKUP_DESTROY);
}

#[inline]
pub unsafe fn uobj_put_read(uobj: *mut ib_uobject) {
    rdma_lookup_put_uobject(uobj, UVERBS_LOOKUP_READ);
}

#[macro_export]
macro_rules! uobj_put_obj_read {
    ($obj:expr) => {{ uobj_put_read(($obj).uobject) }};
}

#[inline]
pub unsafe fn uobj_put_write(uobj: *mut ib_uobject) {
    rdma_lookup_put_uobject(uobj, UVERBS_LOOKUP_WRITE);
}

#[inline]
pub unsafe fn uobj_alloc_abort(uobj: *mut ib_uobject, attrs: *mut uverbs_attr_bundle) {
    rdma_alloc_abort_uobject(uobj, attrs, false);
}

#[inline]
pub unsafe fn uobj_finalize_uobj_create(
    uobj: *mut ib_uobject,
    attrs: *mut uverbs_attr_bundle,
) {
    /*
     * Tell the core code that the write() handler has completed
     * initializing the object and that the core should commit or
     * abort this object based upon the return code from the write()
     * method. Similar to what uverbs_finalize_uobj_create() does for
     * ioctl()
     */
    WARN_ON((*attrs).uobject);
    (*attrs).uobject = uobj;
}

#[inline]
pub unsafe fn __uobj_alloc(
    obj: *const uverbs_api_object,
    attrs: *mut uverbs_attr_bundle,
    ib_dev: *mut *mut ib_device,
) -> *mut ib_uobject {
    let uobj = rdma_alloc_begin_uobject(obj, attrs);
    if !IS_ERR(uobj) {
        *ib_dev = (*(*attrs).context).device;
    }
    uobj
}

#[macro_export]
macro_rules! uobj_alloc {
    ($type:expr, $attrs:expr, $ib_dev:expr) => {{
        __uobj_alloc(uobj_get_type!($attrs, $type), $attrs, $ib_dev)
    }};
}

#[inline]
pub unsafe fn uverbs_flow_action_fill_action(
    action: *mut ib_flow_action,
    uobj: *mut ib_uobject,
    ib_dev: *mut ib_device,
    type_: ib_flow_action_type,
) {
    atomic_set(&mut (*action).usecnt, 0);
    (*action).device = ib_dev;
    (*action).type_ = type_;
    (*action).uobject = uobj;
    (*uobj).object = action as *mut core::ffi::c_void;
}

#[repr(C)]
pub struct ib_uflow_resources {
    pub max: usize,
    pub num: usize,
    pub collection_num: usize,
    pub counters_num: usize,
    pub counters: *mut *mut ib_counters,
    pub collection: *mut *mut ib_flow_action,
}

#[repr(C)]
pub struct ib_uflow_object {
    pub uobject: ib_uobject,
    pub resources: *mut ib_uflow_resources,
}

pub unsafe extern "C" fn flow_resources_alloc(num_specs: usize) -> *mut ib_uflow_resources;
pub unsafe extern "C" fn flow_resources_add(
    uflow_res: *mut ib_uflow_resources,
    type_: ib_flow_spec_type,
    ibobj: *mut core::ffi::c_void,
);
pub unsafe extern "C" fn ib_uverbs_flow_resources_free(uflow_res: *mut ib_uflow_resources);

#[inline]
pub unsafe fn ib_set_flow(
    uobj: *mut ib_uobject,
    ibflow: *mut ib_flow,
    qp: *mut ib_qp,
    device: *mut ib_device,
    uflow_res: *mut ib_uflow_resources,
) {
    (*uobj).object = ibflow as *mut core::ffi::c_void;
    (*ibflow).uobject = uobj;

    if !qp.is_null() {
        atomic_inc(&mut (*qp).usecnt);
        (*ibflow).qp = qp;
    }

    (*ibflow).device = device;
    let uflow = container_of!(uobj, ib_uflow_object, uobject);
    (*uflow).resources = uflow_res;
}

#[repr(C)]
pub struct uverbs_api_object {
    pub type_attrs: *const uverbs_obj_type,
    pub type_class: *const uverbs_obj_type_class,
    pub disabled: u8,
    pub id: u32,
}

#[inline]
pub unsafe fn uobj_get_object_id(uobj: *mut ib_uobject) -> u32 {
    (*(*uobj).uapi_object).id
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
