// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * hvcserver.c
 * Copyright (C) 2004 Ryan S Arnold, IBM Corporation
 *
 * PPC64 virtual I/O console server support.
 */

// Linux kernel, PowerPC hypervisor, hvcserver, and I/O declarations are
// supplied by the surrounding kernel translation unit.

const HVCS_ARCH_VERSION: &str = "1.0.0";

/* Module metadata: MODULE_AUTHOR, MODULE_DESCRIPTION, MODULE_LICENSE, and
 * MODULE_VERSION(HVCS_ARCH_VERSION) are supplied by the kernel module layer. */

/*
 * Convert arch specific return codes into relevant errnos.  The hvcs
 * functions aren't performance sensitive, so this conversion isn't an
 * issue.
 */
unsafe fn hvcs_convert(to_convert: libc::c_long) -> libc::c_int {
    match to_convert {
        H_SUCCESS => 0,
        H_PARAMETER => -EINVAL,
        H_HARDWARE => -EIO,
        H_BUSY
        | H_LONG_BUSY_ORDER_1_MSEC
        | H_LONG_BUSY_ORDER_10_MSEC
        | H_LONG_BUSY_ORDER_100_MSEC
        | H_LONG_BUSY_ORDER_1_SEC
        | H_LONG_BUSY_ORDER_10_SEC
        | H_LONG_BUSY_ORDER_100_SEC => -EBUSY,
        H_FUNCTION => -EPERM,
        _ => -EPERM,
    }
}

/**
 * hvcs_free_partner_info - free pi allocated by hvcs_get_partner_info
 * @head: list_head pointer for an allocated list of partner info structs to
 *\tfree.
 *
 * This function is used to free the partner info list that was returned by
 * calling hvcs_get_partner_info().
 */
#[no_mangle]
pub unsafe extern "C" fn hvcs_free_partner_info(head: *mut list_head) -> libc::c_int {
    let mut element: *mut list_head;

    if head.is_null() {
        return -EINVAL;
    }

    while !list_empty(head) {
        element = (*head).next;
        let pi = list_entry(element, hvcs_partner_info, node);
        list_del(element);
        kfree(pi as *mut libc::c_void);
    }

    0
}

/* Helper function for hvcs_get_partner_info */
unsafe fn hvcs_next_partner(
    unit_address: u32,
    last_p_partition_ID: libc::c_ulong,
    last_p_unit_address: libc::c_ulong,
    pi_buff: *mut libc::c_ulong,
) -> libc::c_int {
    let retval = plpar_hcall_norets(
        H_VTERM_PARTNER_INFO,
        unit_address as libc::c_ulong,
        last_p_partition_ID,
        last_p_unit_address,
        virt_to_phys(pi_buff),
    );
    hvcs_convert(retval)
}

/**
 * hvcs_get_partner_info - Get all of the partner info for a vty-server adapter
 * @unit_address: The unit_address of the vty-server adapter for which this
 *\tfunction is fetching partner info.
 * @head: An initialized list_head pointer to an empty list to use to return the
 *\tlist of partner info fetched from the hypervisor to the caller.
 * @pi_buff: A page sized buffer pre-allocated prior to calling this function
 *\tthat is to be used to be used by firmware as an iterator to keep track
 *\tof the partner info retrieval.
 *
 * This function returns non-zero on success, or if there is no partner info.
 *
 * The pi_buff is pre-allocated prior to calling this function because this
 * function may be called with a spin_lock held and kmalloc of a page is not
 * recommended as GFP_ATOMIC.
 *
 * The first long of this buffer is used to store a partner unit address.  The
 * second long is used to store a partner partition ID and starting at
 * pi_buff[2] is the 79 character Converged Location Code (diff size than the
 * unsigned longs, hence the casting mumbo jumbo you see later).
 *
 * Invocation of this function should always be followed by an invocation of
 * hvcs_free_partner_info() using a pointer to the SAME list head instance
 * that was passed as a parameter to this function.
 */
#[no_mangle]
pub unsafe extern "C" fn hvcs_get_partner_info(
    unit_address: u32,
    head: *mut list_head,
    pi_buff: *mut libc::c_ulong,
) -> libc::c_int {
    let mut last_p_partition_ID: libc::c_ulong;
    let mut last_p_unit_address: libc::c_ulong;
    let mut next_partner_info: *mut hvcs_partner_info = core::ptr::null_mut();
    let more = 1;
    let mut retval: libc::c_int;

    /* invalid parameters */
    if head.is_null() || pi_buff.is_null() {
        return -EINVAL;
    }

    memset(pi_buff as *mut libc::c_void, 0x00, PAGE_SIZE);
    last_p_partition_ID = !0;
    last_p_unit_address = !0;
    INIT_LIST_HEAD(head);

    while more != 0 {
        retval = hvcs_next_partner(unit_address, last_p_partition_ID,
            last_p_unit_address, pi_buff);
        if retval != 0 {
            if !list_empty(head) {
                return 0;
            }
            return retval;
        }

        last_p_partition_ID = be64_to_cpu(*pi_buff.add(0));
        last_p_unit_address = be64_to_cpu(*pi_buff.add(1));

        /* This indicates that there are no further partners */
        if last_p_partition_ID == !0 && last_p_unit_address == !0 {
            break;
        }

        /* This is a very small struct and will be freed soon in
         * hvcs_free_partner_info(). */
        next_partner_info = kmalloc_obj::<hvcs_partner_info>(GFP_ATOMIC);

        if next_partner_info.is_null() {
            printk(KERN_WARNING, "HVCONSOLE: kmalloc() failed to allocate partner info struct.\n");
            hvcs_free_partner_info(head);
            return -ENOMEM;
        }

        (*next_partner_info).unit_address = last_p_unit_address as u32;
        (*next_partner_info).partition_ID = last_p_partition_ID as u32;

        /* copy the Null-term char too */
        strscpy(
            (*next_partner_info).location_code.as_mut_ptr(),
            (pi_buff.add(2)) as *const libc::c_char,
            core::mem::size_of_val(&(*next_partner_info).location_code),
        );

        list_add_tail(&mut (*next_partner_info).node, head);
        next_partner_info = core::ptr::null_mut();
    }

    0
}

/**
 * hvcs_register_connection - establish a connection between this vty-server and
 *\ta vty.
 */
#[no_mangle]
pub unsafe extern "C" fn hvcs_register_connection(
    unit_address: u32,
    p_partition_ID: u32,
    p_unit_address: u32,
) -> libc::c_int {
    let retval = plpar_hcall_norets(
        H_REGISTER_VTERM,
        unit_address as libc::c_ulong,
        p_partition_ID as libc::c_ulong,
        p_unit_address as libc::c_ulong,
    );
    hvcs_convert(retval)
}

/**
 * hvcs_free_connection - free the connection between a vty-server and vty
 */
#[no_mangle]
pub unsafe extern "C" fn hvcs_free_connection(unit_address: u32) -> libc::c_int {
    let retval = plpar_hcall_norets(H_FREE_VTERM, unit_address as libc::c_ulong);
    hvcs_convert(retval)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
