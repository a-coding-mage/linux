/*
 * Copyright (c) 2006, 2017 Oracle and/or its affiliates. All rights reserved.
 *
 * This software is available under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// Linux kernel and local header dependencies are supplied by other translation units.

static mut rds_trans_modules: [*const ::core::ffi::c_char; RDS_TRANS_COUNT] = [
    /* RDS_TRANS_IB */ c"rds_rdma".as_ptr(),
    /* RDS_TRANS_GAP */ core::ptr::null(),
    /* RDS_TRANS_TCP */ c"rds_tcp".as_ptr(),
];

static mut transports: [*mut rds_transport; RDS_TRANS_COUNT] =
    [core::ptr::null_mut(); RDS_TRANS_COUNT];
static mut rds_trans_sem: rw_semaphore = RWSEM_INITIALIZER;

pub unsafe fn rds_trans_register(trans: *mut rds_transport) {
    BUG_ON(strlen((*trans).t_name) + 1 > TRANSNAMSIZ);

    down_write(&raw mut rds_trans_sem);

    if !transports[(*trans).t_type as usize].is_null() {
        printk(KERN_ERR, c"RDS Transport type %d already registered\n", (*trans).t_type);
    } else {
        transports[(*trans).t_type as usize] = trans;
        printk(KERN_INFO, c"Registered RDS/%s transport\n", (*trans).t_name);
    }

    up_write(&raw mut rds_trans_sem);
}

pub unsafe fn rds_trans_unregister(trans: *mut rds_transport) {
    down_write(&raw mut rds_trans_sem);

    transports[(*trans).t_type as usize] = core::ptr::null_mut();
    printk(KERN_INFO, c"Unregistered RDS/%s transport\n", (*trans).t_name);

    up_write(&raw mut rds_trans_sem);
}

pub unsafe fn rds_trans_put(trans: *mut rds_transport) {
    if !trans.is_null() {
        module_put((*trans).t_owner);
    }
}

pub unsafe fn rds_trans_get_preferred(
    net: *mut net,
    addr: *const in6_addr,
    scope_id: u32,
) -> *mut rds_transport {
    let mut ret: *mut rds_transport = core::ptr::null_mut();
    let mut trans: *mut rds_transport;

    if ipv6_addr_v4mapped(addr) {
        if (*(addr as *const u8).add(12)) == IN_LOOPBACKNET {
            return &raw mut rds_loop_transport;
        }
    } else if ipv6_addr_loopback(addr) {
        return &raw mut rds_loop_transport;
    }

    down_read(&raw mut rds_trans_sem);
    for i in 0..RDS_TRANS_COUNT {
        trans = transports[i];

        if !trans.is_null()
            && ((*trans).laddr_check)(net, addr, scope_id) == 0
            && ((*trans).t_owner.is_null() || try_module_get((*trans).t_owner))
        {
            ret = trans;
            break;
        }
    }
    up_read(&raw mut rds_trans_sem);

    ret
}

pub unsafe fn rds_trans_get(t_type: i32) -> *mut rds_transport {
    let mut ret: *mut rds_transport = core::ptr::null_mut();
    let mut trans: *mut rds_transport;

    down_read(&raw mut rds_trans_sem);
    trans = transports[t_type as usize];
    if trans.is_null() {
        up_read(&raw mut rds_trans_sem);
        if !rds_trans_modules[t_type as usize].is_null() {
            request_module(rds_trans_modules[t_type as usize]);
        }
        down_read(&raw mut rds_trans_sem);
        trans = transports[t_type as usize];
    }
    if !trans.is_null()
        && (*trans).t_type == t_type
        && ((*trans).t_owner.is_null() || try_module_get((*trans).t_owner))
    {
        ret = trans;
    }

    up_read(&raw mut rds_trans_sem);

    ret
}

/*
 * This returns the number of stats entries in the snapshot and only
 * copies them using the iter if there is enough space for them.  The
 * caller passes in the global stats so that we can size and copy while
 * holding the lock.
 */
pub unsafe fn rds_trans_stats_info_copy(
    iter: *mut rds_info_iterator,
    mut avail: u32,
) -> u32 {
    let mut total: u32 = 0;
    let mut part: u32;

    rds_info_iter_unmap(iter);
    down_read(&raw mut rds_trans_sem);

    for i in 0..RDS_TRANS_COUNT {
        let trans = transports[i];
        if trans.is_null() || (*trans).stats_info_copy.is_none() {
            continue;
        }

        part = ((*trans).stats_info_copy.unwrap())(iter, avail);
        avail -= core::cmp::min(avail, part);
        total += part;
    }

    up_read(&raw mut rds_trans_sem);

    total
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
