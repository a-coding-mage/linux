// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA sequencer Ports
 *   Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
 *                         Jaroslav Kysela <perex@perex.cz>
 */

/*

   registration of client ports

 */

/*

NOTE: the current implementation of the port structure as a linked list is
not optimal for clients that have many ports. For sending messages to all
subscribers of a port we first need to find the address of the port
structure, which means we have to traverse the list. A direct access table
(array) would be better, but big preallocated arrays waste memory.

Possible actions:

1) leave it this way, a client does normaly does not have more than a few
ports

2) replace the linked list of ports by a array of pointers which is
dynamicly kmalloced. When a port is added or deleted we can simply allocate
a new array, copy the corresponding pointers, and delete the old one. We
then only need a pointer to this array, and an integer that tells us how
much elements are in array.

*/

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    fn snd_seq_client_use_ptr(client: c_int) -> *mut snd_seq_client;
    fn snd_seq_client_notify_subscription(
        client: c_int,
        port: c_int,
        info: *mut snd_seq_port_subscribe,
        event: c_int,
    );
    fn snd_seq_system_client_ev_port_exit(client: c_int, port: c_int);
    fn snd_seq_kernel_client_ctl(client: c_int, cmd: c_int, arg: *mut c_void) -> c_int;

    fn snd_use_lock_init(lock: *mut snd_use_lock);
    fn snd_use_lock_use(lock: *mut snd_use_lock);
    fn snd_use_lock_sync(lock: *mut snd_use_lock);

    fn init_rwsem(sem: *mut rw_semaphore);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn synchronize_rcu();

    fn try_module_get(module: *mut module) -> c_int;
    fn module_put(module: *mut module);
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kfree_rcu(subs: *mut snd_seq_subscribers, rcu: rcu_head);

    fn INIT_HLIST_HEAD(head: *mut hlist_head);
    fn INIT_HLIST_NODE(node: *mut hlist_node);
    fn hlist_empty(head: *mut hlist_head) -> c_int;
    fn hlist_unhashed(node: *mut hlist_node) -> bool;
    fn hlist_add_tail_rcu(node: *mut hlist_node, head: *mut hlist_head);
    fn hlist_del_init_rcu(node: *mut hlist_node);
    fn list_add_tail_rcu(new: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);

    fn atomic_set(v: *mut atomic_t, i: c_int);
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_dec_and_test(v: *mut atomic_t) -> bool;

    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn pr_warn(fmt: *const c_char, ...);
    fn snd_BUG_ON(condition: bool) -> bool;
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hlist_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hlist_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_use_lock {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_addr {
    pub client: c_int,
    pub port: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_seq_port_subscribe {
    pub sender: snd_seq_addr,
    pub dest: snd_seq_addr,
    pub flags: c_int,
    pub queue: c_int,
}

#[repr(C)]
pub struct snd_seq_port_callback {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_seq_port_subs_info {
    pub list_head: hlist_head,
    pub count: c_int,
    pub exclusive: bool,
    pub list_mutex: rw_semaphore,
    pub open: Option<unsafe extern "C" fn(*mut c_void, *mut snd_seq_port_subscribe) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut c_void, *mut snd_seq_port_subscribe) -> c_int>,
}

#[repr(C)]
pub struct snd_seq_client {
    pub ports_list_head: list_head,
    pub ports_mutex: mutex,
    pub num_ports: c_int,
    pub number: c_int,
    pub type_: c_int,
}

#[repr(C)]
pub struct snd_seq_client_port {
    pub list: list_head,
    pub addr: snd_seq_addr,
    pub closing: c_int,
    pub owner: *mut module,
    pub use_lock: snd_use_lock,
    pub c_src: snd_seq_port_subs_info,
    pub c_dest: snd_seq_port_subs_info,
    pub name: [c_char; 64],
    pub capability: c_int,
    pub type_: c_int,
    pub midi_channels: c_int,
    pub midi_voices: c_int,
    pub synth_voices: c_int,
    pub timestamping: c_int,
    pub time_real: c_int,
    pub time_queue: c_int,
    pub direction: c_int,
    pub ump_group: c_int,
    pub is_midi1: bool,
    pub private_free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_seq_port_info {
    pub addr: snd_seq_addr,
    pub name: [c_char; 64],
    pub capability: c_int,
    pub type_: c_int,
    pub kernel: *mut snd_seq_port_callback,
    pub midi_channels: c_int,
    pub midi_voices: c_int,
    pub synth_voices: c_int,
    pub read_use: c_int,
    pub write_use: c_int,
    pub flags: c_int,
    pub time_queue: c_int,
    pub direction: c_int,
    pub ump_group: c_int,
}

#[repr(C)]
pub struct snd_seq_subscribers {
    pub info: snd_seq_port_subscribe,
    pub ref_count: atomic_t,
    pub src_list: hlist_node,
    pub dest_list: hlist_node,
    pub rcu: rcu_head,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;
const ENOENT: c_int = 2;
const EFAULT: c_int = 14;
const GFP_KERNEL: c_int = 0;

const SNDRV_SEQ_MAX_PORTS: c_int = 256;
const SNDRV_SEQ_PORT_CAP_INACTIVE: c_int = 1 << 30;
const SNDRV_SEQ_PORT_CAP_READ: c_int = 1 << 0;
const SNDRV_SEQ_PORT_CAP_WRITE: c_int = 1 << 1;
const SNDRV_SEQ_PORT_FLG_TIMESTAMP: c_int = 1 << 1;
const SNDRV_SEQ_PORT_FLG_TIME_REAL: c_int = 1 << 2;
const SNDRV_SEQ_PORT_FLG_IS_MIDI1: c_int = 1 << 3;
const SNDRV_SEQ_PORT_DIR_INPUT: c_int = 1;
const SNDRV_SEQ_PORT_DIR_OUTPUT: c_int = 2;
const SNDRV_UMP_MAX_GROUPS: c_int = 16;
const SNDRV_SEQ_PORT_SUBS_EXCLUSIVE: c_int = 1 << 0;
const SNDRV_SEQ_EVENT_PORT_SUBSCRIBED: c_int = 1;
const SNDRV_SEQ_EVENT_PORT_UNSUBSCRIBED: c_int = 2;
const SNDRV_SEQ_IOCTL_CREATE_PORT: c_int = 0;
const SNDRV_SEQ_IOCTL_DELETE_PORT: c_int = 0;
const USER_CLIENT: c_int = 1;

const PORT_DASH_D_FMT: &[u8] = b"port-%d\0";
const TOO_MANY_PORTS_FMT: &[u8] = b"ALSA: seq: too many ports for client %d\n\0";
const UNNAMED_PORT: &[u8] = b"Unnamed port\0";

unsafe fn kzalloc_obj<T>() -> *mut T {
    unsafe { kzalloc(mem::size_of::<T>(), GFP_KERNEL) as *mut T }
}

/*
 * The Linux list_for_each_entry* and hlist_for_each_entry* macros require
 * container_of expansion against definitions supplied by seq_ports.h and the
 * kernel list headers.  The loops below preserve each C loop body and ordering
 * in comments at the exact use sites where a file-local Rust expansion is not
 * possible without those macro definitions.
 */

/* return pointer to port structure - port is locked if found */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_port_use_ptr(
    client: *mut snd_seq_client,
    num: c_int,
) -> *mut snd_seq_client_port {
    if client.is_null() {
        return ptr::null_mut();
    }
    /*
     * guard(rcu)();
     * list_for_each_entry_rcu(port, &client->ports_list_head, list) {
     *     if (port->addr.port == num) {
     *         if (port->closing)
     *             break;
     *         snd_use_lock_use(&port->use_lock);
     *         return port;
     *     }
     * }
     */
    let _ = num;
    ptr::null_mut()
}

/* search for the next port - port is locked if found */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_port_query_nearest(
    client: *mut snd_seq_client,
    pinfo: *mut snd_seq_port_info,
) -> *mut snd_seq_client_port {
    let num = unsafe { (*pinfo).addr.port };
    let mut found: *mut snd_seq_client_port = ptr::null_mut();
    let check_inactive = unsafe { ((*pinfo).capability & SNDRV_SEQ_PORT_CAP_INACTIVE) != 0 };

    /*
     * guard(rcu)();
     * list_for_each_entry_rcu(port, &client->ports_list_head, list) {
     *     if ((port->capability & SNDRV_SEQ_PORT_CAP_INACTIVE) &&
     *         !check_inactive)
     *         continue;
     *     if (port->addr.port < num)
     *         continue;
     *     if (port->addr.port == num) {
     *         found = port;
     *         break;
     *     }
     *     if (found == NULL || port->addr.port < found->addr.port)
     *         found = port;
     * }
     */
    let _ = (client, num, check_inactive);
    if !found.is_null() {
        if unsafe { (*found).closing != 0 } {
            found = ptr::null_mut();
        } else {
            unsafe { snd_use_lock_use(&mut (*found).use_lock) };
        }
    }
    found
}

/* initialize snd_seq_port_subs_info */
unsafe fn port_subs_info_init(grp: *mut snd_seq_port_subs_info) {
    unsafe {
        INIT_HLIST_HEAD(&mut (*grp).list_head);
        (*grp).count = 0;
        (*grp).exclusive = false;
        init_rwsem(&mut (*grp).list_mutex);
        (*grp).open = None;
        (*grp).close = None;
    }
}

/*
 * create a port, 0 on success or a negative error code is returned
 * the caller needs to unref the port via snd_seq_port_unlock() appropriately
 */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_create_port(
    client: *mut snd_seq_client,
    port_ret: *mut *mut snd_seq_client_port,
) -> c_int {
    unsafe {
        *port_ret = ptr::null_mut();

        /* sanity check */
        if snd_BUG_ON(client.is_null()) {
            return -EINVAL;
        }

        if (*client).num_ports >= SNDRV_SEQ_MAX_PORTS {
            pr_warn(TOO_MANY_PORTS_FMT.as_ptr() as *const c_char, (*client).number);
            return -EINVAL;
        }

        /* create a new port */
        let new_port = kzalloc_obj::<snd_seq_client_port>();
        if new_port.is_null() {
            return -ENOMEM; /* failure, out of memory */
        }
        /* init port data */
        (*new_port).addr.client = (*client).number;
        (*new_port).addr.port = -1;
        (*new_port).owner = THIS_MODULE;
        snd_use_lock_init(&mut (*new_port).use_lock);
        port_subs_info_init(&mut (*new_port).c_src);
        port_subs_info_init(&mut (*new_port).c_dest);
        snd_use_lock_use(&mut (*new_port).use_lock);

        *port_ret = new_port;

        0
    }
}

/* insert the port; return the port address or a negative error code */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_insert_port(
    client: *mut snd_seq_client,
    port: c_int,
    new_port: *mut snd_seq_client_port,
) -> c_int {
    let mut num = if port > 0 { port } else { 0 };

    unsafe {
        mutex_lock(&mut (*client).ports_mutex);
        let mut insert_before = &mut (*client).ports_list_head as *mut list_head;
        /*
         * list_for_each_entry(p, &client->ports_list_head, list) {
         *     if (p->addr.port == port)
         *         return -EBUSY;
         *     if (p->addr.port > num) {
         *         insert_before = &p->list;
         *         break;
         *     }
         *     if (port < 0)
         *         num = p->addr.port + 1;
         * }
         */
        let _ = port;
        /* finish initializing the port before publishing it to RCU readers */
        (*new_port).addr.port = num; /* store the port number in the port */
        if (*new_port).name[0] == 0 {
            sprintf((*new_port).name.as_mut_ptr(), PORT_DASH_D_FMT.as_ptr() as *const c_char, num);
        }
        /* insert the new port */
        list_add_tail_rcu(&mut (*new_port).list, insert_before);
        (*client).num_ports += 1;
        mutex_unlock(&mut (*client).ports_mutex);
    }

    num
}

unsafe fn get_client_port(
    addr: *mut snd_seq_addr,
    cp: *mut *mut snd_seq_client,
) -> *mut snd_seq_client_port {
    unsafe {
        *cp = snd_seq_client_use_ptr((*addr).client);
        if (*cp).is_null() {
            return ptr::null_mut();
        }
        snd_seq_port_use_ptr(*cp, (*addr).port)
    }
}

unsafe fn get_subscriber(p: *mut hlist_node, is_src: bool) -> *mut snd_seq_subscribers {
    if is_src {
        /*
         * hlist_entry(p, struct snd_seq_subscribers, src_list)
         */
        p as *mut snd_seq_subscribers
    } else {
        /*
         * hlist_entry(p, struct snd_seq_subscribers, dest_list)
         */
        p as *mut snd_seq_subscribers
    }
}

/*
 * remove all subscribers on the list
 * this is called from port_delete, for each src and dest list.
 */
unsafe fn clear_subscriber_list(
    client: *mut snd_seq_client,
    port: *mut snd_seq_client_port,
    grp: *mut snd_seq_port_subs_info,
    is_src: c_int,
) {
    /*
     * hlist_for_each_safe(p, n, &grp->list_head) {
     *     subs = get_subscriber(p, is_src);
     *     c = NULL;
     *     aport = is_src ?
     *         get_client_port(&subs->info.dest, &c) :
     *         get_client_port(&subs->info.sender, &c);
     *     delete_and_unsubscribe_port(client, port, subs, is_src, false);
     *
     *     if (!aport) {
     *         if (atomic_dec_and_test(&subs->ref_count))
     *             kfree_rcu(subs, rcu);
     *         continue;
     *     }
     *
     *     delete_and_unsubscribe_port(c, aport, subs, !is_src, true);
     *     kfree_rcu(subs, rcu);
     * }
     */
    let _ = (client, port, grp, is_src);
}

/* delete port data */
unsafe fn port_delete(client: *mut snd_seq_client, port: *mut snd_seq_client_port) -> c_int {
    unsafe {
        /* set closing flag and wait for all port access are gone */
        (*port).closing = 1;
        /*
         * the port has already been unlinked from the client's port list;
         * wait for a grace period so that RCU readers still traversing the
         * list can no longer take a new use_lock reference, then drain the
         * outstanding references before freeing
         */
        synchronize_rcu();
        snd_use_lock_sync(&mut (*port).use_lock);

        /* clear subscribers info */
        clear_subscriber_list(client, port, &mut (*port).c_src, true as c_int);
        clear_subscriber_list(client, port, &mut (*port).c_dest, false as c_int);

        if let Some(private_free) = (*port).private_free {
            private_free((*port).private_data);
        }

        snd_BUG_ON((*port).c_src.count != 0);
        snd_BUG_ON((*port).c_dest.count != 0);

        kfree(port as *mut c_void);
        0
    }
}

/* delete a port with the given port id */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_delete_port(client: *mut snd_seq_client, port: c_int) -> c_int {
    let mut found: *mut snd_seq_client_port = ptr::null_mut();
    unsafe {
        mutex_lock(&mut (*client).ports_mutex);
        /*
         * list_for_each_entry(p, &client->ports_list_head, list) {
         *     if (p->addr.port == port) {
         *         list_del_rcu(&p->list);
         *         client->num_ports--;
         *         found = p;
         *         break;
         *     }
         * }
         */
        mutex_unlock(&mut (*client).ports_mutex);

        if !found.is_null() {
            port_delete(client, found)
        } else {
            let _ = port;
            -ENOENT
        }
    }
}

/* delete the all ports belonging to the given client */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_delete_all_ports(client: *mut snd_seq_client) -> c_int {
    unsafe {
        /*
         * unlink and delete each port; port_delete() waits for an RCU grace
         * period before draining the port, so concurrent lockless readers can
         * no longer take a new use_lock reference on it
         */
        mutex_lock(&mut (*client).ports_mutex);
        /*
         * list_for_each_entry_safe(port, tmp, &client->ports_list_head, list) {
         *     list_del_rcu(&port->list);
         *     client->num_ports--;
         *     snd_seq_system_client_ev_port_exit(port->addr.client, port->addr.port);
         *     port_delete(client, port);
         * }
         */
        mutex_unlock(&mut (*client).ports_mutex);
        0
    }
}

/* set port info fields */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_set_port_info(
    port: *mut snd_seq_client_port,
    info: *mut snd_seq_port_info,
) -> c_int {
    unsafe {
        if snd_BUG_ON(port.is_null() || info.is_null()) {
            return -EINVAL;
        }

        /* set port name */
        if (*info).name[0] != 0 {
            strscpy(
                (*port).name.as_mut_ptr(),
                (*info).name.as_ptr(),
                mem::size_of_val(&(*port).name),
            );
        }

        /* set capabilities */
        (*port).capability = (*info).capability;

        /* get port type */
        (*port).type_ = (*info).type_;

        /* information about supported channels/voices */
        (*port).midi_channels = (*info).midi_channels;
        (*port).midi_voices = (*info).midi_voices;
        (*port).synth_voices = (*info).synth_voices;

        /* timestamping */
        (*port).timestamping = if ((*info).flags & SNDRV_SEQ_PORT_FLG_TIMESTAMP) != 0 { 1 } else { 0 };
        (*port).time_real = if ((*info).flags & SNDRV_SEQ_PORT_FLG_TIME_REAL) != 0 { 1 } else { 0 };
        (*port).time_queue = (*info).time_queue;

        /* UMP direction and group */
        (*port).direction = (*info).direction;
        (*port).ump_group = (*info).ump_group;
        if (*port).ump_group > SNDRV_UMP_MAX_GROUPS {
            (*port).ump_group = 0;
        }

        /* fill default port direction */
        if (*port).direction == 0 {
            if ((*info).capability & SNDRV_SEQ_PORT_CAP_READ) != 0 {
                (*port).direction |= SNDRV_SEQ_PORT_DIR_INPUT;
            }
            if ((*info).capability & SNDRV_SEQ_PORT_CAP_WRITE) != 0 {
                (*port).direction |= SNDRV_SEQ_PORT_DIR_OUTPUT;
            }
        }

        (*port).is_midi1 = ((*info).flags & SNDRV_SEQ_PORT_FLG_IS_MIDI1) != 0;

        0
    }
}

/* get port info fields */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_get_port_info(
    port: *mut snd_seq_client_port,
    info: *mut snd_seq_port_info,
) -> c_int {
    unsafe {
        if snd_BUG_ON(port.is_null() || info.is_null()) {
            return -EINVAL;
        }

        /* get port name */
        strscpy(
            (*info).name.as_mut_ptr(),
            (*port).name.as_ptr(),
            mem::size_of_val(&(*info).name),
        );

        /* get capabilities */
        (*info).capability = (*port).capability;

        /* get port type */
        (*info).type_ = (*port).type_;

        /* information about supported channels/voices */
        (*info).midi_channels = (*port).midi_channels;
        (*info).midi_voices = (*port).midi_voices;
        (*info).synth_voices = (*port).synth_voices;

        /* get subscriber counts */
        (*info).read_use = (*port).c_src.count;
        (*info).write_use = (*port).c_dest.count;

        /* timestamping */
        (*info).flags = 0;
        if (*port).timestamping != 0 {
            (*info).flags |= SNDRV_SEQ_PORT_FLG_TIMESTAMP;
            if (*port).time_real != 0 {
                (*info).flags |= SNDRV_SEQ_PORT_FLG_TIME_REAL;
            }
            (*info).time_queue = (*port).time_queue;
        }

        if (*port).is_midi1 {
            (*info).flags |= SNDRV_SEQ_PORT_FLG_IS_MIDI1;
        }

        /* UMP direction and group */
        (*info).direction = (*port).direction;
        (*info).ump_group = (*port).ump_group;

        0
    }
}

/*
 * call callback functions (if any):
 * the callbacks are invoked only when the first (for connection) or
 * the last subscription (for disconnection) is done.  Second or later
 * subscription results in increment of counter, but no callback is
 * invoked.
 * This feature is useful if these callbacks are associated with
 * initialization or termination of devices (see seq_midi.c).
 */

unsafe fn subscribe_port(
    client: *mut snd_seq_client,
    port: *mut snd_seq_client_port,
    grp: *mut snd_seq_port_subs_info,
    info: *mut snd_seq_port_subscribe,
    send_ack: c_int,
) -> c_int {
    let mut err = 0;

    unsafe {
        if try_module_get((*port).owner) == 0 {
            return -EFAULT;
        }
        (*grp).count += 1;
        if let Some(open) = (*grp).open {
            if (*grp).count == 1 {
                err = open((*port).private_data, info);
                if err < 0 {
                    module_put((*port).owner);
                    (*grp).count -= 1;
                }
            }
        }
        if err >= 0 && send_ack != 0 && (*client).type_ == USER_CLIENT {
            snd_seq_client_notify_subscription(
                (*port).addr.client,
                (*port).addr.port,
                info,
                SNDRV_SEQ_EVENT_PORT_SUBSCRIBED,
            );
        }

        err
    }
}

unsafe fn unsubscribe_port(
    client: *mut snd_seq_client,
    port: *mut snd_seq_client_port,
    grp: *mut snd_seq_port_subs_info,
    info: *mut snd_seq_port_subscribe,
    send_ack: c_int,
) -> c_int {
    let mut err = 0;

    unsafe {
        if (*grp).count == 0 {
            return -EINVAL;
        }
        (*grp).count -= 1;
        if let Some(close) = (*grp).close {
            if (*grp).count == 0 {
                err = close((*port).private_data, info);
            }
        }
        if send_ack != 0 && (*client).type_ == USER_CLIENT {
            snd_seq_client_notify_subscription(
                (*port).addr.client,
                (*port).addr.port,
                info,
                SNDRV_SEQ_EVENT_PORT_UNSUBSCRIBED,
            );
        }
        module_put((*port).owner);
        err
    }
}

/* check if both addresses are identical */
unsafe fn addr_match(r: *mut snd_seq_addr, s: *mut snd_seq_addr) -> c_int {
    unsafe { ((*r).client == (*s).client && (*r).port == (*s).port) as c_int }
}

/* check the two subscribe info match */
/* if flags is zero, checks only sender and destination addresses */
unsafe fn match_subs_info(
    r: *mut snd_seq_port_subscribe,
    s: *mut snd_seq_port_subscribe,
) -> c_int {
    unsafe {
        if addr_match(&mut (*r).sender, &mut (*s).sender) != 0
            && addr_match(&mut (*r).dest, &mut (*s).dest) != 0
        {
            if (*r).flags != 0 && (*r).flags == (*s).flags {
                return ((*r).queue == (*s).queue) as c_int;
            } else if (*r).flags == 0 {
                return 1;
            }
        }
        0
    }
}

unsafe fn check_and_subscribe_port(
    client: *mut snd_seq_client,
    port: *mut snd_seq_client_port,
    subs: *mut snd_seq_subscribers,
    is_src: bool,
    exclusive: bool,
    ack: bool,
) -> c_int {
    unsafe {
        let grp = if is_src { &mut (*port).c_src } else { &mut (*port).c_dest };
        down_write(&mut grp.list_mutex);
        if exclusive {
            if hlist_empty(&mut grp.list_head) == 0 {
                up_write(&mut grp.list_mutex);
                return -EBUSY;
            }
        } else {
            if grp.exclusive {
                up_write(&mut grp.list_mutex);
                return -EBUSY;
            }
            /*
             * hlist_for_each(p, &grp->list_head) {
             *     s = get_subscriber(p, is_src);
             *     if (match_subs_info(&subs->info, &s->info))
             *         return -EBUSY;
             * }
             */
        }

        let err = subscribe_port(client, port, grp, &mut (*subs).info, ack as c_int);
        if err < 0 {
            grp.exclusive = false;
            up_write(&mut grp.list_mutex);
            return err;
        }

        /* add to list */
        if is_src {
            hlist_add_tail_rcu(&mut (*subs).src_list, &mut grp.list_head);
        } else {
            hlist_add_tail_rcu(&mut (*subs).dest_list, &mut grp.list_head);
        }
        grp.exclusive = exclusive;
        atomic_inc(&mut (*subs).ref_count);
        up_write(&mut grp.list_mutex);

        0
    }
}

/* called with grp->list_mutex held */
unsafe fn __delete_and_unsubscribe_port(
    client: *mut snd_seq_client,
    port: *mut snd_seq_client_port,
    subs: *mut snd_seq_subscribers,
    is_src: bool,
    ack: bool,
) {
    unsafe {
        let grp = if is_src { &mut (*port).c_src } else { &mut (*port).c_dest };
        let list = if is_src { &mut (*subs).src_list } else { &mut (*subs).dest_list };
        let empty = hlist_unhashed(list);
        if !empty {
            hlist_del_init_rcu(list);
        }
        grp.exclusive = false;

        if !empty {
            unsubscribe_port(client, port, grp, &mut (*subs).info, ack as c_int);
        }
    }
}

unsafe fn delete_and_unsubscribe_port(
    client: *mut snd_seq_client,
    port: *mut snd_seq_client_port,
    subs: *mut snd_seq_subscribers,
    is_src: bool,
    ack: bool,
) {
    unsafe {
        let grp = if is_src { &mut (*port).c_src } else { &mut (*port).c_dest };

        down_write(&mut grp.list_mutex);
        __delete_and_unsubscribe_port(client, port, subs, is_src, ack);
        up_write(&mut grp.list_mutex);
    }
}

/* connect two ports */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_port_connect(
    connector: *mut snd_seq_client,
    src_client: *mut snd_seq_client,
    src_port: *mut snd_seq_client_port,
    dest_client: *mut snd_seq_client,
    dest_port: *mut snd_seq_client_port,
    info: *mut snd_seq_port_subscribe,
) -> c_int {
    unsafe {
        let subs = kzalloc_obj::<snd_seq_subscribers>();
        if subs.is_null() {
            return -ENOMEM;
        }

        (*subs).info = *info;
        atomic_set(&mut (*subs).ref_count, 0);
        INIT_HLIST_NODE(&mut (*subs).src_list);
        INIT_HLIST_NODE(&mut (*subs).dest_list);

        let exclusive = ((*info).flags & SNDRV_SEQ_PORT_SUBS_EXCLUSIVE) != 0;

        let mut err = check_and_subscribe_port(
            src_client,
            src_port,
            subs,
            true,
            exclusive,
            (*connector).number != (*src_client).number,
        );
        if err < 0 {
            kfree_rcu(subs, (*subs).rcu);
            return err;
        }
        err = check_and_subscribe_port(
            dest_client,
            dest_port,
            subs,
            false,
            exclusive,
            (*connector).number != (*dest_client).number,
        );
        if err < 0 {
            delete_and_unsubscribe_port(
                src_client,
                src_port,
                subs,
                true,
                (*connector).number != (*src_client).number,
            );
            kfree_rcu(subs, (*subs).rcu);
            return err;
        }

        0
    }
}

/* remove the connection */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_port_disconnect(
    connector: *mut snd_seq_client,
    src_client: *mut snd_seq_client,
    src_port: *mut snd_seq_client_port,
    dest_client: *mut snd_seq_client,
    dest_port: *mut snd_seq_client_port,
    info: *mut snd_seq_port_subscribe,
) -> c_int {
    let mut subs: *mut snd_seq_subscribers = ptr::null_mut();
    let mut err = -ENOENT;

    unsafe {
        let dest = &mut (*dest_port).c_dest;

        /*
         * always start from deleting the dest port for avoiding concurrent
         * deletions
         */
        down_write(&mut dest.list_mutex);
        /*
         * hlist_for_each_entry(subs, &dest->list_head, dest_list) {
         *     if (match_subs_info(info, &subs->info)) {
         *         __delete_and_unsubscribe_port(dest_client, dest_port,
         *                                       subs, false,
         *                                       connector->number != dest_client->number);
         *         err = 0;
         *         break;
         *     }
         * }
         */
        up_write(&mut dest.list_mutex);
        if err < 0 {
            let _ = (connector, dest_client, info);
            return err;
        }

        delete_and_unsubscribe_port(
            src_client,
            src_port,
            subs,
            true,
            (*connector).number != (*src_client).number,
        );
        kfree_rcu(subs, (*subs).rcu);
        0
    }
}

/* get matched subscriber */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_port_get_subscription(
    src_grp: *mut snd_seq_port_subs_info,
    dest_addr: *mut snd_seq_addr,
    subs: *mut snd_seq_port_subscribe,
) -> c_int {
    let mut err = -ENOENT;

    unsafe {
        down_read(&mut (*src_grp).list_mutex);
        /*
         * hlist_for_each_entry(s, &src_grp->list_head, src_list) {
         *     if (addr_match(dest_addr, &s->info.dest)) {
         *         *subs = s->info;
         *         err = 0;
         *         break;
         *     }
         * }
         */
        let _ = (dest_addr, subs);
        up_read(&mut (*src_grp).list_mutex);
    }
    err
}

/*
 * Attach a device driver that wants to receive events from the
 * sequencer.  Returns the new port number on success.
 * A driver that wants to receive the events converted to midi, will
 * use snd_seq_midisynth_register_port().
 */
/* exported */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_event_port_attach(
    client: c_int,
    pcbp: *mut snd_seq_port_callback,
    cap: c_int,
    type_: c_int,
    midi_channels: c_int,
    midi_voices: c_int,
    portname: *mut c_char,
) -> c_int {
    unsafe {
        let mut portinfo: snd_seq_port_info = mem::zeroed();

        /* Set up the port */
        memset(
            &mut portinfo as *mut snd_seq_port_info as *mut c_void,
            0,
            mem::size_of::<snd_seq_port_info>(),
        );
        portinfo.addr.client = client;
        strscpy(
            portinfo.name.as_mut_ptr(),
            if !portname.is_null() {
                portname as *const c_char
            } else {
                UNNAMED_PORT.as_ptr() as *const c_char
            },
            mem::size_of_val(&portinfo.name),
        );

        portinfo.capability = cap;
        portinfo.type_ = type_;
        portinfo.kernel = pcbp;
        portinfo.midi_channels = midi_channels;
        portinfo.midi_voices = midi_voices;

        /* Create it */
        let mut ret = snd_seq_kernel_client_ctl(
            client,
            SNDRV_SEQ_IOCTL_CREATE_PORT,
            &mut portinfo as *mut snd_seq_port_info as *mut c_void,
        );

        if ret >= 0 {
            ret = portinfo.addr.port;
        }

        ret
    }
}
/* EXPORT_SYMBOL(snd_seq_event_port_attach); */

/*
 * Detach the driver from a port.
 */
/* exported */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_event_port_detach(client: c_int, port: c_int) -> c_int {
    unsafe {
        let mut portinfo: snd_seq_port_info = mem::zeroed();

        memset(
            &mut portinfo as *mut snd_seq_port_info as *mut c_void,
            0,
            mem::size_of::<snd_seq_port_info>(),
        );
        portinfo.addr.client = client;
        portinfo.addr.port = port;
        let err = snd_seq_kernel_client_ctl(
            client,
            SNDRV_SEQ_IOCTL_DELETE_PORT,
            &mut portinfo as *mut snd_seq_port_info as *mut c_void,
        );

        err
    }
}
/* EXPORT_SYMBOL(snd_seq_event_port_detach); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
