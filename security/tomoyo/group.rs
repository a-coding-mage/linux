// SPDX-License-Identifier: GPL-2.0
/*
 * security/tomoyo/group.c
 *
 * Copyright (C) 2005-2011  NTT DATA CORPORATION
 */

/* Dependencies corresponding to linux/slab.h, linux/rculist.h, and common.h
 * are supplied by the surrounding translation unit. */

unsafe fn tomoyo_same_path_group(
    a: *const tomoyo_acl_head,
    b: *const tomoyo_acl_head,
) -> bool {
    container_of!(a, tomoyo_path_group, head).member_name
        == container_of!(b, tomoyo_path_group, head).member_name
}

unsafe fn tomoyo_same_number_group(
    a: *const tomoyo_acl_head,
    b: *const tomoyo_acl_head,
) -> bool {
    let a_number = &container_of!(a, tomoyo_number_group, head).number;
    let b_number = &container_of!(b, tomoyo_number_group, head).number;
    core::slice::from_raw_parts(
        a_number as *const _ as *const u8,
        core::mem::size_of_val(a_number),
    ) == core::slice::from_raw_parts(
        b_number as *const _ as *const u8,
        core::mem::size_of_val(b_number),
    )
}

unsafe fn tomoyo_same_address_group(
    a: *const tomoyo_acl_head,
    b: *const tomoyo_acl_head,
) -> bool {
    let p1 = container_of!(a, tomoyo_address_group, head);
    let p2 = container_of!(b, tomoyo_address_group, head);
    tomoyo_same_ipaddr_union(&p1.address, &p2.address)
}

pub unsafe fn tomoyo_write_group(param: *mut tomoyo_acl_param, type_: u8) -> i32 {
    let group = tomoyo_get_group(param, type_);
    let mut error = -EINVAL;

    if group.is_null() {
        return -ENOMEM;
    }
    (*param).list = &mut (*group).member_list;
    if type_ == TOMOYO_PATH_GROUP {
        let mut e: tomoyo_path_group = core::mem::zeroed();
        e.member_name = tomoyo_get_name(tomoyo_read_token(param));
        if e.member_name.is_null() {
            error = -ENOMEM;
            goto_out!();
        }
        error = tomoyo_update_policy(
            &mut e.head,
            core::mem::size_of_val(&e),
            param,
            tomoyo_same_path_group,
        );
        tomoyo_put_name(e.member_name);
    } else if type_ == TOMOYO_NUMBER_GROUP {
        let mut e: tomoyo_number_group = core::mem::zeroed();
        if (*param).data[0] == b'@' || !tomoyo_parse_number_union(param, &mut e.number) {
            goto_out!();
        }
        error = tomoyo_update_policy(
            &mut e.head,
            core::mem::size_of_val(&e),
            param,
            tomoyo_same_number_group,
        );
        /* tomoyo_put_number_union() is not needed because param->data[0] != '@'. */
    } else {
        let mut e: tomoyo_address_group = core::mem::zeroed();
        if (*param).data[0] == b'@' || !tomoyo_parse_ipaddr_union(param, &mut e.address) {
            goto_out!();
        }
        error = tomoyo_update_policy(
            &mut e.head,
            core::mem::size_of_val(&e),
            param,
            tomoyo_same_address_group,
        );
    }
    goto_out:
    tomoyo_put_group(group);
    error
}

pub unsafe fn tomoyo_path_matches_group(
    pathname: *const tomoyo_path_info,
    group: *const tomoyo_group,
) -> *const tomoyo_path_info {
    let mut member = (*group).member_list.first_entry::<tomoyo_path_group>();
    while !member.is_null() {
        if !(*member).head.is_deleted
            && tomoyo_path_matches_pattern(pathname, (*member).member_name)
        {
            return (*member).member_name;
        }
        member = (*member).head.list.next_entry::<tomoyo_path_group>();
    }
    core::ptr::null()
}

pub unsafe fn tomoyo_number_matches_group(
    min: c_ulong,
    max: c_ulong,
    group: *const tomoyo_group,
) -> bool {
    let mut member = (*group).member_list.first_entry::<tomoyo_number_group>();
    while !member.is_null() {
        if !(*member).head.is_deleted
            && min <= (*member).number.values[1]
            && max >= (*member).number.values[0]
        {
            return true;
        }
        member = (*member).head.list.next_entry::<tomoyo_number_group>();
    }
    false
}

pub unsafe fn tomoyo_address_matches_group(
    is_ipv6: bool,
    address: *const __be32,
    group: *const tomoyo_group,
) -> bool {
    let size = if is_ipv6 { 16 } else { 4 };
    let mut member = (*group).member_list.first_entry::<tomoyo_address_group>();
    while !member.is_null() {
        if !(*member).head.is_deleted && (*member).address.is_ipv6 == is_ipv6 {
            let lower = core::slice::from_raw_parts((*member).address.ip.as_ptr(), size);
            let addr = core::slice::from_raw_parts(address as *const u8, size);
            let upper = core::slice::from_raw_parts((*member).address.ip[1].as_ptr(), size);
            if lower <= addr && addr <= upper {
                return true;
            }
        }
        member = (*member).head.list.next_entry::<tomoyo_address_group>();
    }
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
