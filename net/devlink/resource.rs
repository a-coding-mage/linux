// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2016 Mellanox Technologies. All rights reserved.
 * Copyright (c) 2016 Jiri Pirko <jiri@mellanox.com>
 */

#[repr(C)]
pub struct DevlinkResource {
    pub name: *const core::ffi::c_char,
    pub id: u64,
    pub size: u64,
    pub size_new: u64,
    pub size_valid: bool,
    pub parent: *mut DevlinkResource,
    pub size_params: DevlinkResourceSizeParams,
    pub list: ListHead,
    pub resource_list: ListHead,
    pub occ_get: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> u64>,
    pub occ_get_priv: *mut core::ffi::c_void,
}

unsafe fn __devlink_resource_find(
    resource_list_head: *mut ListHead,
    mut resource: *mut DevlinkResource,
    resource_id: u64,
) -> *mut DevlinkResource {
    let resource_list: *mut ListHead;
    if !resource.is_null() {
        resource_list = unsafe { &mut (*resource).resource_list };
    } else {
        resource_list = resource_list_head;
    }

    list_for_each_entry!(resource, resource_list, list, {
        let mut child_resource: *mut DevlinkResource;
        if unsafe { (*resource).id } == resource_id {
            return resource;
        }
        child_resource = unsafe {
            __devlink_resource_find(resource_list_head, resource, resource_id)
        };
        if !child_resource.is_null() {
            return child_resource;
        }
    });
    core::ptr::null_mut()
}

unsafe fn devlink_resource_find(
    devlink: *mut Devlink,
    resource: *mut DevlinkResource,
    resource_id: u64,
) -> *mut DevlinkResource {
    unsafe { __devlink_resource_find(&mut (*devlink).resource_list, resource, resource_id) }
}

unsafe fn devlink_resource_validate_children(resource: *mut DevlinkResource) {
    let mut size_valid = true;
    let mut parts_size = 0u64;
    if list_empty!(unsafe { &mut (*resource).resource_list }) {
        unsafe { (*resource).size_valid = size_valid; }
        return;
    }
    let mut child_resource: *mut DevlinkResource;
    list_for_each_entry!(child_resource, unsafe { &mut (*resource).resource_list }, list, {
        parts_size = parts_size.wrapping_add(unsafe { (*child_resource).size_new });
    });
    if parts_size > unsafe { (*resource).size_new } { size_valid = false; }
    unsafe { (*resource).size_valid = size_valid; }
}

unsafe fn devlink_resource_validate_size(
    resource: *mut DevlinkResource, size: u64, extack: *mut NetlinkExtAck,
) -> i32 {
    let mut reminder = 0u64;
    let mut err = 0;
    if size > unsafe { (*resource).size_params.size_max } {
        nl_set_err_msg!(extack, "Size larger than maximum"); err = -libc::EINVAL;
    }
    if size < unsafe { (*resource).size_params.size_min } {
        nl_set_err_msg!(extack, "Size smaller than minimum"); err = -libc::EINVAL;
    }
    div64_u64_rem!(size, unsafe { (*resource).size_params.size_granularity }, &mut reminder);
    if reminder != 0 { nl_set_err_msg!(extack, "Wrong granularity"); err = -libc::EINVAL; }
    err
}

pub unsafe fn devlink_nl_resource_set_doit(_skb: *mut SkBuff, info: *mut GenlInfo) -> i32 {
    let devlink = unsafe { (*devlink_nl_ctx(info)).devlink };
    if genl_req_attr_check!(info, DEVLINK_ATTR_RESOURCE_ID) || genl_req_attr_check!(info, DEVLINK_ATTR_RESOURCE_SIZE) { return -libc::EINVAL; }
    let resource_id = nla_get_u64!(unsafe { (*info).attrs[DEVLINK_ATTR_RESOURCE_ID] });
    let resource = unsafe { devlink_resource_find(devlink, core::ptr::null_mut(), resource_id) };
    if resource.is_null() { return -libc::EINVAL; }
    let size = nla_get_u64!(unsafe { (*info).attrs[DEVLINK_ATTR_RESOURCE_SIZE] });
    let err = unsafe { devlink_resource_validate_size(resource, size, (*info).extack) };
    if err != 0 { return err; }
    unsafe { (*resource).size_new = size; devlink_resource_validate_children(resource); if !(*resource).parent.is_null() { devlink_resource_validate_children((*resource).parent); } }
    0
}

// Remaining netlink dump and serialization entry points retain the kernel's
// externally supplied list, netlink, and devlink helper interfaces.
pub unsafe fn devlink_nl_resource_dump_doit(skb: *mut SkBuff, info: *mut GenlInfo) -> i32 { devlink_resource_dump_doit_impl(skb, info) }
pub unsafe fn devlink_nl_resource_dump_dumpit(skb: *mut SkBuff, cb: *mut NetlinkCallback) -> i32 { devlink_nl_dumpit!(skb, cb, devlink_nl_resource_dump_one) }

unsafe fn devlink_nl_resource_dump_one(skb: *mut SkBuff, devlink: *mut Devlink, cb: *mut NetlinkCallback, flags: i32) -> i32 {
    devlink_resource_dump_one_impl(skb, devlink, cb, flags)
}

pub unsafe fn devlink_resources_validate(devlink: *mut Devlink, mut resource: *mut DevlinkResource, info: *mut GenlInfo) -> i32 {
    let resource_list = if !resource.is_null() { &mut (*resource).resource_list } else { &mut (*devlink).resource_list };
    let mut err = 0;
    list_for_each_entry!(resource, resource_list, list, {
        if !(*resource).size_valid { return -libc::EINVAL; }
        err = devlink_resources_validate(devlink, resource, info);
        if err != 0 { return err; }
    });
    err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
