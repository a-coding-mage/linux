// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ALSA sequencer Client Manager
 * Copyright (c) 1998-2001 by Frank van de Pol <fvdpol@coil.demon.nl>
 *                            Jaroslav Kysela <perex@perex.cz>
 *                            Takashi Iwai <tiwai@suse.de>
 *
 * Source-level Rust translation of core/seq/seq_clientmgr.c.
 * C include dependencies are intentionally left as external kernel symbols.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{self, MaybeUninit};
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type bool_t = bool;
type __poll_t = c_uint;

#[repr(C)] pub struct file { pub f_mode: c_uint, pub f_flags: c_uint, pub private_data: *mut c_void }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct poll_table { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct snd_card { pub number: c_int }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct snd_info_entry { _private: [u8; 0] }
#[repr(C)] pub struct snd_info_buffer { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct atomic_t { pub counter: c_int }

#[repr(C)] #[derive(Copy, Clone)] pub struct snd_seq_addr { pub client: c_uchar, pub port: c_uchar }
type c_uchar = u8;

#[repr(C)] #[derive(Copy, Clone)] pub struct snd_seq_real_time { pub tv_sec: c_uint, pub tv_nsec: c_uint }
#[repr(C)] #[derive(Copy, Clone)] pub union snd_seq_timestamp { pub tick: c_uint, pub time: snd_seq_real_time }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_seq_ev_note { pub channel: c_uchar, pub note: c_uchar, pub velocity: c_uchar, pub off_velocity: c_uchar, pub duration: c_uint }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_seq_ev_ext { pub len: c_uint, pub ptr: *mut c_void }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_seq_ev_quote { pub origin: snd_seq_addr, pub event: *mut snd_seq_event, pub value: c_int }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_seq_ev_connect { pub sender: snd_seq_addr, pub dest: snd_seq_addr }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_seq_ev_raw32 { pub d: [u32; 12] }
#[repr(C)] #[derive(Copy, Clone)] pub union snd_seq_event_data {
    pub note: snd_seq_ev_note,
    pub ext: snd_seq_ev_ext,
    pub quote: snd_seq_ev_quote,
    pub connect: snd_seq_ev_connect,
    pub raw32: snd_seq_ev_raw32,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_seq_event {
    pub type_: c_uchar,
    pub flags: c_uchar,
    pub tag: c_uchar,
    pub queue: c_uchar,
    pub time: snd_seq_timestamp,
    pub source: snd_seq_addr,
    pub dest: snd_seq_addr,
    pub data: snd_seq_event_data,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_seq_ump_event {
    pub core: snd_seq_event,
    pub ump: [u32; 4],
}
#[repr(C)] pub union __snd_seq_event { pub legacy: snd_seq_event, pub ump: snd_seq_ump_event }

#[repr(C)] pub struct snd_seq_pool { pub size: c_int, pub room: c_int, pub counter: atomic_t }
#[repr(C)] pub struct snd_seq_fifo { pub overflow: atomic_t, pub pool: *mut snd_seq_pool }
#[repr(C)] pub struct snd_seq_event_cell { pub event: snd_seq_event }
#[repr(C)] pub struct snd_seq_usage { pub cur: c_int, pub peak: c_int }
#[repr(C)] pub struct snd_seq_user_client { pub fifo: *mut snd_seq_fifo, pub fifo_pool_size: c_int, pub file: *mut file, pub owner: *mut pid }
#[repr(C)] pub struct snd_seq_kernel_client { pub card: *mut snd_card }
#[repr(C)] pub union snd_seq_client_data { pub user: snd_seq_user_client, pub kernel: snd_seq_kernel_client }
#[repr(C)] pub struct snd_seq_client {
    pub number: c_int,
    pub type_: c_int,
    pub pool: *mut snd_seq_pool,
    pub accept_input: c_int,
    pub accept_output: c_int,
    pub data: snd_seq_client_data,
    pub use_lock: c_void,
    pub ports_mutex: mutex,
    pub ports_list_head: list_head,
    pub ioctl_mutex: mutex,
    pub ump_endpoint_port: c_int,
    pub midi_version: c_uint,
    pub filter: c_uint,
    pub event_filter: [c_ulong; 4],
    pub group_filter: c_uint,
    pub event_lost: c_int,
    pub name: [c_char; 64],
    pub user_pversion: c_uint,
    pub convert32: c_int,
    pub num_ports: c_int,
    pub ump_info: *mut *mut c_void,
}
#[repr(C)] pub struct snd_seq_port_subs_info { pub list_mutex: rw_semaphore, pub list_head: hlist_head, pub count: c_int, pub exclusive: c_int, pub open: *mut c_void, pub close: *mut c_void }
#[repr(C)] pub struct snd_seq_port_subscribe { pub sender: snd_seq_addr, pub dest: snd_seq_addr, pub voices: c_uint, pub flags: c_uint, pub queue: c_uchar }
#[repr(C)] pub struct snd_seq_subscribers { pub src_list: hlist_node, pub dest_list: hlist_node, pub info: snd_seq_port_subscribe, pub ref_count: atomic_t }
#[repr(C)] pub struct snd_seq_port_callback {
    pub owner: *mut module,
    pub private_data: *mut c_void,
    pub private_free: *mut c_void,
    pub event_input: Option<unsafe extern "C" fn(*mut snd_seq_event, c_int, *mut c_void, c_int, c_int) -> c_int>,
    pub subscribe: *mut c_void,
    pub unsubscribe: *mut c_void,
    pub use_: *mut c_void,
    pub unuse: *mut c_void,
}
#[repr(C)] pub struct snd_seq_client_port {
    pub addr: snd_seq_addr,
    pub name: [c_char; 64],
    pub capability: c_uint,
    pub direction: c_uchar,
    pub owner: *mut module,
    pub private_data: *mut c_void,
    pub private_free: *mut c_void,
    pub event_input: Option<unsafe extern "C" fn(*mut snd_seq_event, c_int, *mut c_void, c_int, c_int) -> c_int>,
    pub c_src: snd_seq_port_subs_info,
    pub c_dest: snd_seq_port_subs_info,
    pub timestamping: c_int,
    pub time_queue: c_int,
    pub time_real: c_int,
    pub is_midi1: c_int,
    pub list: list_head,
}

#[repr(C)] pub struct snd_seq_system_info { pub queues: c_int, pub clients: c_int, pub ports: c_int, pub channels: c_int, pub cur_clients: c_int, pub cur_queues: c_int }
#[repr(C)] pub struct snd_seq_running_info { pub client: c_int, pub big_endian: c_int, pub cpu_mode: c_int }
#[repr(C)] pub struct snd_seq_client_info { pub client: c_int, pub type_: c_int, pub name: [c_char; 64], pub filter: c_uint, pub event_lost: c_int, pub event_filter: [c_ulong; 4], pub group_filter: c_uint, pub num_ports: c_int, pub pid: c_int, pub card: c_int, pub midi_version: c_uint, pub reserved: [c_char; 64] }
#[repr(C)] pub struct snd_seq_port_info { pub addr: snd_seq_addr, pub flags: c_uint, pub capability: c_uint, pub kernel: *mut snd_seq_port_callback }
#[repr(C)] pub struct snd_seq_queue_info { pub queue: c_int, pub owner: c_int, pub locked: c_int, pub flags: c_uint, pub name: [c_char; 64] }
#[repr(C)] pub struct snd_seq_timer { pub running: c_int, pub tempo: c_uint, pub ppq: c_int, pub skew: c_uint, pub skew_base: c_uint, pub tempo_base: c_uint, pub type_: c_int, pub alsa_id: c_uint, pub preferred_resolution: c_uint }
#[repr(C)] pub struct queue_cells { pub cells: c_int }
#[repr(C)] pub struct snd_seq_queue { pub queue: c_int, pub owner: c_int, pub locked: c_int, pub flags: c_uint, pub name: [c_char; 64], pub timer: *mut snd_seq_timer, pub tickq: *mut queue_cells, pub timeq: *mut queue_cells, pub timer_mutex: mutex }
#[repr(C)] pub struct snd_seq_queue_status { pub queue: c_int, pub events: c_int, pub time: snd_seq_real_time, pub tick: c_uint, pub running: c_int, pub flags: c_uint }
#[repr(C)] pub struct snd_seq_queue_tempo { pub queue: c_int, pub tempo: c_uint, pub ppq: c_int, pub skew_value: c_uint, pub skew_base: c_uint, pub tempo_base: c_uint }
#[repr(C)] pub struct snd_seq_queue_timer_alsa { pub id: c_uint, pub resolution: c_uint }
#[repr(C)] pub union snd_seq_queue_timer_u { pub alsa: snd_seq_queue_timer_alsa }
#[repr(C)] pub struct snd_seq_queue_timer { pub queue: c_int, pub type_: c_int, pub u: snd_seq_queue_timer_u }
#[repr(C)] pub struct snd_seq_queue_client { pub queue: c_int, pub client: c_int, pub used: c_int }
#[repr(C)] pub struct snd_seq_client_pool { pub client: c_int, pub output_pool: c_int, pub output_room: c_int, pub output_free: c_int, pub input_pool: c_int, pub input_free: c_int }
#[repr(C)] pub struct snd_seq_remove_events { pub remove_mode: c_uint }
#[repr(C)] pub struct snd_seq_query_subs { pub root: snd_seq_addr, pub type_: c_int, pub index: c_int, pub num_subs: c_int, pub addr: snd_seq_addr, pub flags: c_uint, pub queue: c_uchar }
#[repr(C)] pub struct snd_ump_endpoint_info { pub name: [c_char; 64] }
#[repr(C)] pub struct snd_ump_block_info { pub name: [c_char; 64], pub active: c_int, pub first_group: c_int, pub num_groups: c_int }
#[repr(C)] pub struct snd_seq_client_ump_info { pub client: c_int, pub type_: c_int, pub info: *mut c_void }

pub const SNDRV_SEQ_GLOBAL_CLIENTS: c_int = 16;
pub const SNDRV_SEQ_CLIENTS_PER_CARD: c_int = 4;
pub const SNDRV_SEQ_DYNAMIC_CLIENTS_BEGIN: c_int = 128;
pub const SNDRV_SEQ_LFLG_INPUT: u16 = 0x0001;
pub const SNDRV_SEQ_LFLG_OUTPUT: u16 = 0x0002;
pub const SNDRV_SEQ_LFLG_OPEN: u16 = SNDRV_SEQ_LFLG_INPUT | SNDRV_SEQ_LFLG_OUTPUT;
pub const PERM_RD: c_uint = SNDRV_SEQ_PORT_CAP_READ | SNDRV_SEQ_PORT_CAP_SUBS_READ;
pub const PERM_WR: c_uint = SNDRV_SEQ_PORT_CAP_WRITE | SNDRV_SEQ_PORT_CAP_SUBS_WRITE;
pub const NUM_UMP_INFOS: c_int = SNDRV_UMP_MAX_BLOCKS + 1;

extern "C" {
    static mut current: *mut task_struct;
    static mut snd_ecards_limit: c_int;
    static mut THIS_MODULE: module;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(d: *mut c_void, s: *const c_void, n: size_t) -> *mut c_void;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kcalloc(n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn memdup_user(p: *const c_void, size: size_t) -> *mut c_void;
    fn IS_ERR(p: *const c_void) -> bool_t;
    fn PTR_ERR(p: *const c_void) -> c_int;
    fn stream_open(inode: *mut inode, file: *mut file);
    fn access_ok(buf: *const c_void, size: size_t) -> bool_t;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: size_t) -> c_int;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: size_t) -> c_int;
    fn get_user_int(dst: *mut c_int, src: *const c_int) -> c_int;
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn mutex_init(m: *mut mutex);
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn synchronize_rcu();
    fn down_read_nested(s: *mut rw_semaphore, subclass: c_int);
    fn up_read(s: *mut rw_semaphore);
    fn snd_BUG_ON(cond: bool_t) -> bool_t;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn snd_seq_total_cells(pool: *mut snd_seq_pool) -> c_int;
    fn snd_seq_pool_new(size: c_int) -> *mut snd_seq_pool;
    fn snd_seq_pool_delete(pool: *mut *mut snd_seq_pool);
    fn snd_seq_pool_init(pool: *mut snd_seq_pool) -> c_int;
    fn snd_seq_pool_done(pool: *mut snd_seq_pool);
    fn snd_seq_pool_mark_closing(pool: *mut snd_seq_pool);
    fn snd_seq_unused_cells(pool: *mut snd_seq_pool) -> c_int;
    fn snd_seq_pool_poll_wait(pool: *mut snd_seq_pool, file: *mut file, wait: *mut poll_table) -> bool_t;
    fn snd_seq_fifo_new(size: c_int) -> *mut snd_seq_fifo;
    fn snd_seq_fifo_delete(fifo: *mut *mut snd_seq_fifo);
    fn snd_seq_fifo_clear(fifo: *mut snd_seq_fifo);
    fn snd_seq_fifo_cell_out(fifo: *mut snd_seq_fifo, cell: *mut *mut snd_seq_event_cell, nonblock: c_int) -> c_int;
    fn snd_seq_fifo_cell_putback(fifo: *mut snd_seq_fifo, cell: *mut snd_seq_event_cell);
    fn snd_seq_fifo_event_in(fifo: *mut snd_seq_fifo, ev: *mut snd_seq_event) -> c_int;
    fn snd_seq_fifo_poll_wait(fifo: *mut snd_seq_fifo, file: *mut file, wait: *mut poll_table) -> bool_t;
    fn snd_seq_fifo_unused_cells(fifo: *mut snd_seq_fifo) -> c_int;
    fn snd_seq_fifo_resize(fifo: *mut snd_seq_fifo, size: c_int) -> c_int;
    fn snd_seq_cell_free(cell: *mut snd_seq_event_cell);
    fn snd_seq_event_dup(pool: *mut snd_seq_pool, ev: *mut snd_seq_event, cell: *mut *mut snd_seq_event_cell, nonblock: c_int, file: *mut file, mutexp: *mut mutex) -> c_int;
    fn snd_seq_enqueue_event(cell: *mut snd_seq_event_cell, atomic: c_int, hop: c_int) -> c_int;
    fn snd_seq_expand_var_event(ev: *mut snd_seq_event, count: size_t, buf: *mut c_char, in_kernel: c_int, size_aligned: size_t) -> c_int;
    fn snd_seq_event_packet_size(ev: *mut snd_seq_event) -> size_t;
    fn snd_seq_ev_is_ump(ev: *const snd_seq_event) -> bool_t;
    fn snd_seq_ev_is_variable(ev: *const snd_seq_event) -> bool_t;
    fn snd_seq_ev_is_variable_type(ev: *const snd_seq_event) -> bool_t;
    fn snd_seq_ev_is_varusr(ev: *const snd_seq_event) -> bool_t;
    fn snd_seq_ev_is_direct(ev: *const snd_seq_event) -> bool_t;
    fn snd_seq_ev_is_reserved(ev: *const snd_seq_event) -> bool_t;
    fn snd_seq_ev_length_type(ev: *const snd_seq_event) -> c_int;
    fn snd_seq_delete_all_ports(client: *mut snd_seq_client);
    fn snd_seq_queue_client_leave(client: c_int);
    fn snd_seq_queue_is_used(queue: c_int, client: c_int) -> c_int;
    fn snd_seq_queue_get_cur_queues() -> c_int;
    fn snd_seq_queue_alloc(owner: c_int, locked: c_int, flags: c_uint) -> *mut snd_seq_queue;
    fn snd_seq_queue_delete(client: c_int, queue: c_int) -> c_int;
    fn queueptr(queue: c_int) -> *mut snd_seq_queue;
    fn snd_seq_queue_check_access(queue: c_int, client: c_int) -> bool_t;
    fn snd_seq_queue_set_owner(queue: c_int, client: c_int, locked: c_int) -> c_int;
    fn snd_seq_queue_use(queue: c_int, client: c_int, use_: c_int) -> c_int;
    fn snd_seq_queue_find_name(name: *const c_char) -> *mut snd_seq_queue;
    fn snd_seq_queue_timer_set_tempo(queue: c_int, client: c_int, tempo: *mut snd_seq_queue_tempo) -> c_int;
    fn snd_seq_queue_timer_close(queue: c_int);
    fn snd_seq_queue_timer_open(queue: c_int) -> c_int;
    fn snd_seq_queue_remove_cells(client: c_int, info: *mut snd_seq_remove_events);
    fn snd_seq_timer_get_cur_time(tmr: *mut snd_seq_timer, adjust: bool_t) -> snd_seq_real_time;
    fn snd_seq_timer_get_cur_tick(tmr: *mut snd_seq_timer) -> c_uint;
    fn snd_seq_create_port(client: *mut snd_seq_client, port: *mut *mut snd_seq_client_port) -> c_int;
    fn snd_seq_insert_port(client: *mut snd_seq_client, port_idx: c_int, port: *mut snd_seq_client_port) -> c_int;
    fn snd_seq_delete_port(client: *mut snd_seq_client, port: c_int) -> c_int;
    fn snd_seq_port_unlock(port: *mut snd_seq_client_port);
    fn snd_seq_port_use_ptr(client: *mut snd_seq_client, port: c_int) -> *mut snd_seq_client_port;
    fn snd_seq_port_query_nearest(client: *mut snd_seq_client, info: *mut snd_seq_port_info) -> *mut snd_seq_client_port;
    fn snd_seq_set_port_info(port: *mut snd_seq_client_port, info: *mut snd_seq_port_info);
    fn snd_seq_get_port_info(port: *mut snd_seq_client_port, info: *mut snd_seq_port_info);
    fn snd_seq_port_connect(client: *mut snd_seq_client, sender: *mut snd_seq_client, sport: *mut snd_seq_client_port, receiver: *mut snd_seq_client, dport: *mut snd_seq_client_port, subs: *mut snd_seq_port_subscribe) -> c_int;
    fn snd_seq_port_disconnect(client: *mut snd_seq_client, sender: *mut snd_seq_client, sport: *mut snd_seq_client_port, receiver: *mut snd_seq_client, dport: *mut snd_seq_client_port, subs: *mut snd_seq_port_subscribe) -> c_int;
    fn snd_seq_port_get_subscription(group: *mut snd_seq_port_subs_info, dest: *mut snd_seq_addr, subs: *mut snd_seq_port_subscribe) -> c_int;
    fn snd_seq_client_ref(client: *mut snd_seq_client) -> *mut snd_seq_client;
    fn snd_seq_client_unref(client: *mut snd_seq_client);
    fn snd_use_lock_init(lock: *mut c_void);
    fn snd_use_lock_sync(lock: *mut c_void);
    fn task_pid(task: *mut task_struct) -> *mut pid;
    fn get_pid(pid: *mut pid) -> *mut pid;
    fn put_pid(pid: *mut pid);
    fn pid_vnr(pid: *mut pid) -> c_int;
    fn snd_seq_system_client_ev_client_start(client: c_int);
    fn snd_seq_system_client_ev_client_exit(client: c_int);
    fn snd_seq_system_client_ev_client_change(client: c_int);
    fn snd_seq_system_client_ev_port_start(client: c_int, port: c_int);
    fn snd_seq_system_client_ev_port_exit(client: c_int, port: c_int);
    fn snd_seq_system_client_ev_port_change(client: c_int, port: c_int);
    fn snd_seq_system_notify(client: c_int, port: c_int, event: *mut snd_seq_event, atomic: bool_t) -> c_int;
    fn snd_seq_system_ump_notify(client: c_int, blk: c_int, evtype: c_int, atomic: bool_t);
    fn snd_seq_client_is_ump(client: *mut snd_seq_client) -> bool_t;
    fn snd_seq_client_is_midi2(client: *mut snd_seq_client) -> bool_t;
    fn snd_seq_ump_group_port(event: *mut snd_seq_event) -> c_int;
    fn snd_seq_deliver_from_ump(client: *mut snd_seq_client, dest: *mut snd_seq_client, port: *mut snd_seq_client_port, event: *mut snd_seq_event, atomic: c_int, hop: c_int) -> c_int;
    fn snd_seq_deliver_to_ump(client: *mut snd_seq_client, dest: *mut snd_seq_client, port: *mut snd_seq_client_port, event: *mut snd_seq_event, atomic: c_int, hop: c_int) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_seq_info_pool(buffer: *mut snd_info_buffer, pool: *mut snd_seq_pool, prefix: *const c_char);
    fn snd_device_alloc(dev: *mut *mut device, parent: *mut device) -> c_int;
    fn dev_set_name(dev: *mut device, fmt: *const c_char, ...);
    fn snd_register_device(t: c_int, card: *mut snd_card, dev: c_int, fops: *const file_operations, private_data: *mut c_void, device: *mut device) -> c_int;
    fn snd_unregister_device(dev: *mut device);
    fn put_device(dev: *mut device);
    fn vsnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, args: VaList) -> c_int;
    fn snprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: size_t) -> c_long;
    fn in_interrupt() -> bool_t;
    fn request_module(fmt: *const c_char, ...);
    fn snd_request_card(card: c_int);
    fn snd_seq_device_load_drivers();
    fn test_and_set_bit(bit: c_int, addr: *mut c_ulong) -> c_int;
    fn test_bit(bit: c_int, addr: *const c_ulong) -> c_int;
}

#[repr(C)] pub struct VaList { _private: [u8; 0] }
#[repr(C)] pub struct file_operations {
    pub owner: *mut module,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub poll: Option<unsafe extern "C" fn(*mut file, *mut poll_table) -> __poll_t>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub compat_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
}

extern "C" {
    static SNDRV_SEQ_MAX_CLIENTS: c_int;
    static SNDRV_SEQ_MAX_QUEUES: c_int;
    static SNDRV_SEQ_MAX_PORTS: c_int;
    static SNDRV_SEQ_DEFAULT_EVENTS: c_int;
    static SNDRV_SEQ_DEFAULT_CLIENT_EVENTS: c_int;
    static SNDRV_SEQ_MAX_EVENTS: c_int;
    static SNDRV_SEQ_MAX_CLIENT_EVENTS: c_int;
    static SNDRV_SEQ_MAX_EVENT_LEN: c_uint;
    static SNDRV_SEQ_EXT_MASK: c_uint;
    static SNDRV_SEQ_EXT_USRPTR: c_uint;
    static SNDRV_SEQ_MAX_HOPS: c_int;
    static SNDRV_SEQ_VERSION: c_uint;
    static SNDRV_SEQ_CLIENT_UMP_MIDI_2_0: c_uint;
    static SNDRV_SEQ_CLIENT_LEGACY_MIDI: c_uint;
    static SNDRV_SEQ_CLIENT_UMP_MIDI_1_0: c_uint;
    static SNDRV_SEQ_FILTER_USE_EVENT: c_uint;
    static SNDRV_SEQ_FILTER_BOUNCE: c_uint;
    static SNDRV_SEQ_FILTER_NO_CONVERT: c_uint;
    static SND_SEQ_GROUP_FILTER_MASK: c_uint;
    static SNDRV_SEQ_PORT_CAP_READ: c_uint;
    static SNDRV_SEQ_PORT_CAP_WRITE: c_uint;
    static SNDRV_SEQ_PORT_CAP_SUBS_READ: c_uint;
    static SNDRV_SEQ_PORT_CAP_SUBS_WRITE: c_uint;
    static SNDRV_SEQ_PORT_CAP_NO_EXPORT: c_uint;
    static SNDRV_SEQ_PORT_CAP_DUPLEX: c_uint;
    static SNDRV_SEQ_PORT_CAP_INACTIVE: c_uint;
    static SNDRV_SEQ_PORT_CAP_UMP_ENDPOINT: c_uint;
    static SNDRV_SEQ_PORT_FLG_GIVEN_PORT: c_uint;
    static SNDRV_SEQ_PORT_DIR_BIDIRECTION: c_uchar;
    static SNDRV_SEQ_PORT_SUBS_TIMESTAMP: c_uint;
    static SNDRV_SEQ_PORT_SUBS_TIME_REAL: c_uint;
    static SNDRV_SEQ_ADDRESS_SUBSCRIBERS: c_uchar;
    static SNDRV_SEQ_ADDRESS_UNKNOWN: c_int;
    static SNDRV_SEQ_QUEUE_DIRECT: c_uchar;
    static SNDRV_SEQ_CLIENT_SYSTEM: c_uchar;
    static SNDRV_SEQ_PORT_SYSTEM_ANNOUNCE: c_uchar;
    static SNDRV_SEQ_EVENT_BOUNCE: c_uchar;
    static SNDRV_SEQ_EVENT_KERNEL_ERROR: c_uchar;
    static SNDRV_SEQ_EVENT_LENGTH_FIXED: c_int;
    static SNDRV_SEQ_EVENT_LENGTH_VARIABLE: c_int;
    static SNDRV_SEQ_EVENT_LENGTH_VARUSR: c_int;
    static SNDRV_SEQ_EVENT_NOTE: c_uchar;
    static SNDRV_SEQ_EVENT_NOTEON: c_uchar;
    static SNDRV_SEQ_EVENT_NOTEOFF: c_uchar;
    static SNDRV_SEQ_EVENT_NONE: c_uchar;
    static SNDRV_SEQ_EVENT_PORT_SUBSCRIBED: c_int;
    static SNDRV_SEQ_EVENT_PORT_UNSUBSCRIBED: c_int;
    static SNDRV_SEQ_EVENT_UMP_EP_CHANGE: c_int;
    static SNDRV_SEQ_EVENT_UMP_BLOCK_CHANGE: c_int;
    static SNDRV_SEQ_PRIORITY_HIGH: c_uchar;
    static SNDRV_SEQ_TIME_STAMP_MASK: c_uchar;
    static SNDRV_SEQ_TIME_STAMP_TICK: c_uchar;
    static SNDRV_SEQ_TIME_STAMP_REAL: c_uchar;
    static SNDRV_SEQ_REMOVE_INPUT: c_uint;
    static SNDRV_SEQ_REMOVE_OUTPUT: c_uint;
    static SNDRV_SEQ_QUERY_SUBS_READ: c_int;
    static SNDRV_SEQ_QUERY_SUBS_WRITE: c_int;
    static SNDRV_SEQ_TIMER_ALSA: c_int;
    static SNDRV_UMP_MAX_BLOCKS: c_int;
    static SNDRV_SEQ_CLIENT_UMP_INFO_ENDPOINT: c_int;
    static SNDRV_SEQ_IOCTL_SET_CLIENT_UMP_INFO: c_uint;
    static SNDRV_SEQ_IOCTL_GET_CLIENT_UMP_INFO: c_uint;
    static SNDRV_DEVICE_TYPE_SEQUENCER: c_int;
    static FMODE_READ: c_uint;
    static FMODE_WRITE: c_uint;
    static O_NONBLOCK: c_uint;
    static EPOLLERR: __poll_t;
    static EPOLLIN: __poll_t;
    static EPOLLRDNORM: __poll_t;
    static EPOLLOUT: __poll_t;
    static EPOLLWRNORM: __poll_t;
    static IOC_IN: c_uint;
    static IOC_OUT: c_uint;
}

const NO_CLIENT: c_int = 0;
const USER_CLIENT: c_int = 1;
const KERNEL_CLIENT: c_int = 2;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const ENOSPC: c_int = 28;
const EINVAL: c_int = 22;
const EFAULT: c_int = 14;
const EAGAIN: c_int = 11;
const ENOENT: c_int = 2;
const EPERM: c_int = 1;
const EMLINK: c_int = 31;
const EBUSY: c_int = 16;
const ENOTTY: c_int = 25;
const ENODEV: c_int = 19;
const EBADFD: c_int = 77;
const EINTR: c_int = 4;
const INT_MAX: c_int = c_int::MAX;

static mut clients_lock: spinlock_t = spinlock_t { _private: [] };
static mut register_mutex: mutex = mutex { _private: [] };
static mut clienttablock: [c_char; 256] = [0; 256];
static mut clienttab: [*mut snd_seq_client; 256] = [ptr::null_mut(); 256];
static mut client_usage: snd_seq_usage = snd_seq_usage { cur: 0, peak: 0 };
static mut seq_dev: *mut device = ptr::null_mut();

unsafe fn snd_seq_file_flags(file: *mut file) -> u16 {
    match (*file).f_mode & (FMODE_READ | FMODE_WRITE) {
        x if x == FMODE_WRITE => SNDRV_SEQ_LFLG_OUTPUT,
        x if x == FMODE_READ => SNDRV_SEQ_LFLG_INPUT,
        _ => SNDRV_SEQ_LFLG_OPEN,
    }
}

unsafe fn snd_seq_write_pool_allocated(client: *mut snd_seq_client) -> c_int {
    (snd_seq_total_cells((*client).pool) > 0) as c_int
}

unsafe fn __clientptr(clientid: c_int) -> *mut snd_seq_client {
    if clientid < 0 || clientid >= SNDRV_SEQ_MAX_CLIENTS {
        pr_debug(c"ALSA: seq: oops. Trying to get pointer to client %d\n".as_ptr(), clientid);
        return ptr::null_mut();
    }
    clienttab[clientid as usize]
}

unsafe fn clientptr(clientid: c_int) -> *mut snd_seq_client {
    rcu_read_lock();
    let p = __clientptr(clientid);
    rcu_read_unlock();
    p
}

unsafe fn client_use_ptr(clientid: c_int, load_module: bool_t) -> *mut snd_seq_client {
    if clientid < 0 || clientid >= SNDRV_SEQ_MAX_CLIENTS {
        pr_debug(c"ALSA: seq: oops. Trying to get pointer to client %d\n".as_ptr(), clientid);
        return ptr::null_mut();
    }
    rcu_read_lock();
    let mut client = __clientptr(clientid);
    if !client.is_null() {
        let r = snd_seq_client_ref(client);
        rcu_read_unlock();
        return r;
    }
    if clienttablock[clientid as usize] != 0 {
        rcu_read_unlock();
        return ptr::null_mut();
    }
    rcu_read_unlock();

    /* CONFIG_MODULES: request snd-seq-client-N or card drivers, then retry. */
    if load_module {
        snd_seq_device_load_drivers();
        rcu_read_lock();
        client = __clientptr(clientid);
        if !client.is_null() {
            let r = snd_seq_client_ref(client);
            rcu_read_unlock();
            return r;
        }
        rcu_read_unlock();
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_client_use_ptr(clientid: c_int) -> *mut snd_seq_client {
    client_use_ptr(clientid, false)
}

unsafe fn client_load_and_use_ptr(clientid: c_int) -> *mut snd_seq_client {
    client_use_ptr(clientid, true)
}

unsafe fn usage_alloc(res: *mut snd_seq_usage, num: c_int) {
    (*res).cur += num;
    if (*res).cur > (*res).peak { (*res).peak = (*res).cur; }
}

unsafe fn usage_free(res: *mut snd_seq_usage, num: c_int) {
    (*res).cur -= num;
}

#[no_mangle]
pub unsafe extern "C" fn client_init_data() -> c_int {
    memset(clienttablock.as_mut_ptr() as *mut c_void, 0, mem::size_of_val(&clienttablock));
    memset(clienttab.as_mut_ptr() as *mut c_void, 0, mem::size_of_val(&clienttab));
    0
}

unsafe fn seq_create_client1(client_index: c_int, poolsize: c_int) -> *mut snd_seq_client {
    let client = kzalloc(mem::size_of::<snd_seq_client>(), GFP_KERNEL) as *mut snd_seq_client;
    if client.is_null() { return ptr::null_mut(); }
    (*client).pool = snd_seq_pool_new(poolsize);
    if (*client).pool.is_null() {
        kfree(client as *mut c_void);
        return ptr::null_mut();
    }
    (*client).type_ = NO_CLIENT;
    snd_use_lock_init(&mut (*client).use_lock);
    mutex_init(&mut (*client).ports_mutex);
    (*client).ports_list_head.next = &mut (*client).ports_list_head;
    (*client).ports_list_head.prev = &mut (*client).ports_list_head;
    mutex_init(&mut (*client).ioctl_mutex);
    (*client).ump_endpoint_port = -1;

    if client_index < 0 {
        let mut c = SNDRV_SEQ_DYNAMIC_CLIENTS_BEGIN;
        while c < SNDRV_SEQ_MAX_CLIENTS {
            if clienttab[c as usize].is_null() && clienttablock[c as usize] == 0 {
                (*client).number = c;
                clienttab[c as usize] = client;
                return client;
            }
            c += 1;
        }
    } else if clienttab[client_index as usize].is_null() && clienttablock[client_index as usize] == 0 {
        (*client).number = client_index;
        clienttab[client_index as usize] = client;
        return client;
    }
    snd_seq_pool_delete(&mut (*client).pool);
    kfree(client as *mut c_void);
    ptr::null_mut()
}

unsafe fn seq_free_client1(client: *mut snd_seq_client) -> c_int {
    if client.is_null() { return 0; }
    clienttablock[(*client).number as usize] = 1;
    clienttab[(*client).number as usize] = ptr::null_mut();
    snd_seq_delete_all_ports(client);
    snd_seq_queue_client_leave((*client).number);
    synchronize_rcu();
    snd_use_lock_sync(&mut (*client).use_lock);
    if !(*client).pool.is_null() { snd_seq_pool_delete(&mut (*client).pool); }
    clienttablock[(*client).number as usize] = 0;
    0
}

unsafe fn seq_free_client(client: *mut snd_seq_client) {
    mutex_lock(&mut register_mutex);
    match (*client).type_ {
        NO_CLIENT => pr_warn(c"ALSA: seq: Trying to free unused client %d\n".as_ptr(), (*client).number),
        USER_CLIENT | KERNEL_CLIENT => { seq_free_client1(client); usage_free(&mut client_usage, 1); }
        _ => pr_err(c"ALSA: seq: Trying to free client %d with undefined type = %d\n".as_ptr(), (*client).number, (*client).type_),
    }
    mutex_unlock(&mut register_mutex);
    snd_seq_system_client_ev_client_exit((*client).number);
}

unsafe extern "C" fn snd_seq_open(inode: *mut inode, file: *mut file) -> c_int {
    stream_open(inode, file);
    mutex_lock(&mut register_mutex);
    let client = seq_create_client1(-1, SNDRV_SEQ_DEFAULT_EVENTS);
    if client.is_null() { mutex_unlock(&mut register_mutex); return -ENOMEM; }
    let mode = snd_seq_file_flags(file);
    if mode & SNDRV_SEQ_LFLG_INPUT != 0 { (*client).accept_input = 1; }
    if mode & SNDRV_SEQ_LFLG_OUTPUT != 0 { (*client).accept_output = 1; }
    (*client).data.user.fifo = ptr::null_mut();
    (*client).data.user.fifo_pool_size = 0;
    if mode & SNDRV_SEQ_LFLG_INPUT != 0 {
        (*client).data.user.fifo_pool_size = SNDRV_SEQ_DEFAULT_CLIENT_EVENTS;
        (*client).data.user.fifo = snd_seq_fifo_new((*client).data.user.fifo_pool_size);
        if (*client).data.user.fifo.is_null() {
            seq_free_client1(client);
            kfree(client as *mut c_void);
            mutex_unlock(&mut register_mutex);
            return -ENOMEM;
        }
    }
    usage_alloc(&mut client_usage, 1);
    (*client).type_ = USER_CLIENT;
    mutex_unlock(&mut register_mutex);

    let c = (*client).number;
    (*file).private_data = client as *mut c_void;
    (*client).data.user.file = file;
    sprintf((*client).name.as_mut_ptr(), c"Client-%d".as_ptr(), c);
    (*client).data.user.owner = get_pid(task_pid(current));
    snd_seq_system_client_ev_client_start(c);
    0
}

unsafe extern "C" fn snd_seq_release(_inode: *mut inode, file: *mut file) -> c_int {
    let client = (*file).private_data as *mut snd_seq_client;
    if !client.is_null() {
        seq_free_client(client);
        if !(*client).data.user.fifo.is_null() { snd_seq_fifo_delete(&mut (*client).data.user.fifo); }
        free_ump_info(client);
        put_pid((*client).data.user.owner);
        kfree(client as *mut c_void);
    }
    0
}

unsafe fn event_is_compatible(client: *const snd_seq_client, ev: *const snd_seq_event) -> bool_t {
    if snd_seq_ev_is_ump(ev) && (*client).midi_version == 0 { return false; }
    if snd_seq_ev_is_ump(ev) && snd_seq_ev_is_variable(ev) { return false; }
    true
}

unsafe extern "C" fn snd_seq_read(file: *mut file, mut buf: *mut c_char, mut count: size_t, _offset: *mut loff_t) -> ssize_t {
    let client = (*file).private_data as *mut snd_seq_client;
    if snd_seq_file_flags(file) & SNDRV_SEQ_LFLG_INPUT == 0 { return -ENXIO as ssize_t; }
    if !access_ok(buf as *const c_void, count) { return -EFAULT as ssize_t; }
    if snd_BUG_ON(client.is_null()) { return -ENXIO as ssize_t; }
    if (*client).accept_input == 0 { return -ENXIO as ssize_t; }
    let fifo = (*client).data.user.fifo;
    if fifo.is_null() { return -ENXIO as ssize_t; }
    if atomic_read(&(*fifo).overflow) > 0 { snd_seq_fifo_clear(fifo); return -ENOSPC as ssize_t; }
    let mut cell: *mut snd_seq_event_cell = ptr::null_mut();
    let mut err = 0;
    let mut result: c_long = 0;
    let aligned_size = if (*client).midi_version > 0 { mem::size_of::<snd_seq_ump_event>() } else { mem::size_of::<snd_seq_event>() };
    while count >= aligned_size {
        let nonblock = (((*file).f_flags & O_NONBLOCK) != 0 || result > 0) as c_int;
        err = snd_seq_fifo_cell_out(fifo, &mut cell, nonblock);
        if err < 0 { break; }
        if !event_is_compatible(client, &(*cell).event) {
            snd_seq_cell_free(cell);
            cell = ptr::null_mut();
            continue;
        }
        if snd_seq_ev_is_variable(&(*cell).event) {
            let mut tmpev = MaybeUninit::<snd_seq_ump_event>::zeroed().assume_init();
            memcpy(&mut tmpev as *mut _ as *mut c_void, &(*cell).event as *const _ as *const c_void, aligned_size);
            tmpev.core.data.ext.len &= !SNDRV_SEQ_EXT_MASK;
            tmpev.core.data.ext.ptr = ptr::null_mut();
            if copy_to_user(buf as *mut c_void, &tmpev as *const _ as *const c_void, aligned_size) != 0 { err = -EFAULT; break; }
            count -= aligned_size; buf = buf.add(aligned_size);
            err = snd_seq_expand_var_event(&mut (*cell).event, count, buf, 0, aligned_size);
            if err < 0 { break; }
            result += err as c_long; count -= err as usize; buf = buf.add(err as usize);
        } else {
            if copy_to_user(buf as *mut c_void, &(*cell).event as *const _ as *const c_void, aligned_size) != 0 { err = -EFAULT; break; }
            count -= aligned_size; buf = buf.add(aligned_size);
        }
        snd_seq_cell_free(cell);
        cell = ptr::null_mut();
        result += aligned_size as c_long;
    }
    if err < 0 {
        if !cell.is_null() { snd_seq_fifo_cell_putback(fifo, cell); }
        if err == -EAGAIN && result > 0 { err = 0; }
    }
    if err < 0 { err as ssize_t } else { result as ssize_t }
}

unsafe fn check_port_perm(port: *mut snd_seq_client_port, flags: c_uint) -> c_int {
    if ((*port).capability & flags) != flags { return 0; }
    flags as c_int
}

unsafe fn get_event_dest_client(event: *mut snd_seq_event) -> *mut snd_seq_client {
    let dest = snd_seq_client_use_ptr((*event).dest.client as c_int);
    if dest.is_null() { return ptr::null_mut(); }
    if (*dest).accept_input == 0 { snd_seq_client_unref(dest); return ptr::null_mut(); }
    if snd_seq_ev_is_ump(event) { return dest; }
    if ((*dest).filter & SNDRV_SEQ_FILTER_USE_EVENT) != 0 && test_bit((*event).type_ as c_int, (*dest).event_filter.as_ptr()) == 0 {
        snd_seq_client_unref(dest);
        return ptr::null_mut();
    }
    dest
}

unsafe fn bounce_error_event(client: *mut snd_seq_client, event: *mut snd_seq_event, err: c_int, atomic: c_int, hop: c_int) -> c_int {
    let mut bounce_ev: snd_seq_event = mem::zeroed();
    let mut quoted: snd_seq_event;
    if client.is_null() || ((*client).filter & SNDRV_SEQ_FILTER_BOUNCE) == 0 || (*client).accept_input == 0 { return 0; }
    if (*event).type_ == SNDRV_SEQ_EVENT_BOUNCE || (*event).type_ == SNDRV_SEQ_EVENT_KERNEL_ERROR { return err; }
    if (*client).type_ == USER_CLIENT {
        quoted = *event;
        if snd_seq_ev_is_variable(&quoted) {
            quoted.data.ext.len &= !SNDRV_SEQ_EXT_MASK;
            quoted.data.ext.ptr = ptr::null_mut();
        }
        bounce_ev.type_ = SNDRV_SEQ_EVENT_BOUNCE;
        bounce_ev.flags = SNDRV_SEQ_EVENT_LENGTH_VARIABLE as c_uchar;
        bounce_ev.data.ext.len = mem::size_of::<snd_seq_event>() as c_uint;
        bounce_ev.data.ext.ptr = &mut quoted as *mut _ as *mut c_void;
    } else {
        bounce_ev.type_ = SNDRV_SEQ_EVENT_KERNEL_ERROR;
        bounce_ev.flags = SNDRV_SEQ_EVENT_LENGTH_FIXED as c_uchar;
        bounce_ev.data.quote.origin = (*event).dest;
        bounce_ev.data.quote.event = event;
        bounce_ev.data.quote.value = -err;
    }
    bounce_ev.queue = SNDRV_SEQ_QUEUE_DIRECT;
    bounce_ev.source.client = SNDRV_SEQ_CLIENT_SYSTEM;
    bounce_ev.source.port = SNDRV_SEQ_PORT_SYSTEM_ANNOUNCE;
    bounce_ev.dest.client = (*client).number as c_uchar;
    bounce_ev.dest.port = (*event).source.port;
    let result = snd_seq_deliver_single_event(ptr::null_mut(), &mut bounce_ev, atomic, hop + 1);
    if result < 0 { (*client).event_lost += 1; }
    result
}

unsafe fn update_timestamp_of_queue(event: *mut snd_seq_event, queue: c_int, real_time: c_int) -> c_int {
    let q = queueptr(queue);
    if q.is_null() { return 0; }
    (*event).queue = queue as c_uchar;
    (*event).flags &= !SNDRV_SEQ_TIME_STAMP_MASK;
    if real_time != 0 {
        (*event).time.time = snd_seq_timer_get_cur_time((*q).timer, true);
        (*event).flags |= SNDRV_SEQ_TIME_STAMP_REAL;
    } else {
        (*event).time.tick = snd_seq_timer_get_cur_tick((*q).timer);
        (*event).flags |= SNDRV_SEQ_TIME_STAMP_TICK;
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn __snd_seq_deliver_single_event(dest: *mut snd_seq_client, dest_port: *mut snd_seq_client_port, event: *mut snd_seq_event, atomic: c_int, hop: c_int) -> c_int {
    match (*dest).type_ {
        USER_CLIENT => if (*dest).data.user.fifo.is_null() { 0 } else { snd_seq_fifo_event_in((*dest).data.user.fifo, event) },
        KERNEL_CLIENT => match (*dest_port).event_input {
            None => 0,
            Some(f) => f(event, snd_seq_ev_is_direct(event) as c_int, (*dest_port).private_data, atomic, hop),
        },
        _ => 0,
    }
}

unsafe fn _snd_seq_deliver_single_event(client: *mut snd_seq_client, event: *mut snd_seq_event, atomic: c_int, hop: c_int) -> c_int {
    let dest = get_event_dest_client(event);
    if dest.is_null() { return -ENOENT; }
    let dest_port = snd_seq_port_use_ptr(dest, (*event).dest.port as c_int);
    if dest_port.is_null() { snd_seq_client_unref(dest); return -ENOENT; }
    if check_port_perm(dest_port, SNDRV_SEQ_PORT_CAP_WRITE) == 0 { snd_seq_client_unref(dest); return -EPERM; }
    if (*dest_port).timestamping != 0 { update_timestamp_of_queue(event, (*dest_port).time_queue, (*dest_port).time_real); }
    if snd_seq_ev_is_ump(event) {
        if ((*dest).filter & SNDRV_SEQ_FILTER_NO_CONVERT) == 0 {
            let r = snd_seq_deliver_from_ump(client, dest, dest_port, event, atomic, hop);
            snd_seq_client_unref(dest);
            return r;
        } else if (*dest).type_ == USER_CLIENT && !snd_seq_client_is_ump(dest) {
            snd_seq_client_unref(dest);
            return 0;
        }
    } else if snd_seq_client_is_ump(dest) && ((*dest).filter & SNDRV_SEQ_FILTER_NO_CONVERT) == 0 {
        let r = snd_seq_deliver_to_ump(client, dest, dest_port, event, atomic, hop);
        snd_seq_client_unref(dest);
        return r;
    }
    let r = __snd_seq_deliver_single_event(dest, dest_port, event, atomic, hop);
    snd_seq_client_unref(dest);
    r
}

unsafe fn snd_seq_deliver_single_event(client: *mut snd_seq_client, event: *mut snd_seq_event, atomic: c_int, hop: c_int) -> c_int {
    let result = _snd_seq_deliver_single_event(client, event, atomic, hop);
    if result < 0 && !snd_seq_ev_is_direct(event) { return bounce_error_event(client, event, result, atomic, hop); }
    result
}

unsafe fn __deliver_to_subscribers(client: *mut snd_seq_client, event: *mut snd_seq_event, port: c_int, atomic: c_int, hop: c_int) -> c_int {
    if port < 0 { return 0; }
    let src_port = snd_seq_port_use_ptr(client, port);
    if src_port.is_null() { return 0; }
    let saved_size = snd_seq_event_packet_size(event);
    let mut event_saved: __snd_seq_event = mem::zeroed();
    memcpy(&mut event_saved as *mut _ as *mut c_void, event as *const c_void, saved_size);
    let grp = &mut (*src_port).c_src;
    let mut result = 0;
    let mut num_ev = 0;
    if atomic != 0 { rcu_read_lock(); } else { down_read_nested(&mut grp.list_mutex, hop); }
    let mut node = grp.list_head.first;
    while !node.is_null() {
        let subs = node as *mut snd_seq_subscribers;
        if atomic_read(&(*subs).ref_count) == 2 {
            (*event).dest = (*subs).info.dest;
            if ((*subs).info.flags & SNDRV_SEQ_PORT_SUBS_TIMESTAMP) != 0 {
                update_timestamp_of_queue(event, (*subs).info.queue as c_int, ((*subs).info.flags & SNDRV_SEQ_PORT_SUBS_TIME_REAL) as c_int);
            }
            let err = snd_seq_deliver_single_event(client, event, atomic, hop);
            if err < 0 {
                if result == 0 { result = err; }
            } else {
                num_ev += 1;
                memcpy(event as *mut c_void, &event_saved as *const _ as *const c_void, saved_size);
            }
        }
        node = (*node).next;
    }
    if atomic != 0 { rcu_read_unlock(); } else { up_read(&mut grp.list_mutex); }
    memcpy(event as *mut c_void, &event_saved as *const _ as *const c_void, saved_size);
    if result < 0 { result } else { num_ev }
}

unsafe fn deliver_to_subscribers(client: *mut snd_seq_client, event: *mut snd_seq_event, atomic: c_int, hop: c_int) -> c_int {
    let ret = __deliver_to_subscribers(client, event, (*event).source.port as c_int, atomic, hop);
    if !snd_seq_client_is_ump(client) || (*client).ump_endpoint_port < 0 { return ret; }
    let ret2 = if (*event).source.port as c_int == (*client).ump_endpoint_port {
        __deliver_to_subscribers(client, event, snd_seq_ump_group_port(event), atomic, hop)
    } else {
        __deliver_to_subscribers(client, event, (*client).ump_endpoint_port, atomic, hop)
    };
    if ret2 < 0 { ret2 } else { ret }
}

unsafe fn snd_seq_deliver_event(client: *mut snd_seq_client, event: *mut snd_seq_event, atomic: c_int, mut hop: c_int) -> c_int {
    hop += 1;
    if hop >= SNDRV_SEQ_MAX_HOPS {
        pr_debug(c"ALSA: seq: too long delivery path (%d:%d->%d:%d)\n".as_ptr(), (*event).source.client as c_int, (*event).source.port as c_int, (*event).dest.client as c_int, (*event).dest.port as c_int);
        return -EMLINK;
    }
    if snd_seq_ev_is_variable(event) && snd_BUG_ON(atomic != 0 && ((*event).data.ext.len & SNDRV_SEQ_EXT_USRPTR) != 0) { return -EINVAL; }
    if (*event).queue == SNDRV_SEQ_ADDRESS_SUBSCRIBERS || (*event).dest.client == SNDRV_SEQ_ADDRESS_SUBSCRIBERS {
        deliver_to_subscribers(client, event, atomic, hop)
    } else {
        snd_seq_deliver_single_event(client, event, atomic, hop)
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_dispatch_event(cell: *mut snd_seq_event_cell, atomic: c_int, hop: c_int) -> c_int {
    if snd_BUG_ON(cell.is_null()) { return -EINVAL; }
    let client = snd_seq_client_use_ptr((*cell).event.source.client as c_int);
    if client.is_null() { snd_seq_cell_free(cell); return -EINVAL; }
    let result;
    if !snd_seq_ev_is_ump(&(*cell).event) && (*cell).event.type_ == SNDRV_SEQ_EVENT_NOTE {
        let mut tmpev = (*cell).event;
        tmpev.type_ = SNDRV_SEQ_EVENT_NOTEON;
        result = snd_seq_deliver_event(client, &mut tmpev, atomic, hop);
        let ev = &mut (*cell).event;
        ev.type_ = SNDRV_SEQ_EVENT_NOTEOFF;
        ev.flags |= SNDRV_SEQ_PRIORITY_HIGH;
        match ev.flags & SNDRV_SEQ_TIME_STAMP_MASK {
            x if x == SNDRV_SEQ_TIME_STAMP_TICK => { ev.time.tick = ev.time.tick.wrapping_add(ev.data.note.duration); }
            x if x == SNDRV_SEQ_TIME_STAMP_REAL => {
                ev.time.time.tv_nsec = ev.time.time.tv_nsec.wrapping_add(1000000u32.wrapping_mul(ev.data.note.duration % 1000));
                ev.time.time.tv_sec = ev.time.time.tv_sec.wrapping_add(ev.data.note.duration / 1000 + ev.time.time.tv_nsec / 1000000000);
                ev.time.time.tv_nsec %= 1000000000;
            }
            _ => {}
        }
        ev.data.note.velocity = ev.data.note.off_velocity;
        if snd_seq_enqueue_event(cell, atomic, hop) < 0 { snd_seq_cell_free(cell); }
    } else {
        result = snd_seq_deliver_event(client, &mut (*cell).event, atomic, hop);
        snd_seq_cell_free(cell);
    }
    snd_seq_client_unref(client);
    result
}

unsafe fn snd_seq_client_enqueue_event(client: *mut snd_seq_client, event: *mut snd_seq_event, file: *mut file, blocking: c_int, atomic: c_int, hop: c_int, mutexp: *mut mutex) -> c_int {
    if (*event).queue == SNDRV_SEQ_ADDRESS_SUBSCRIBERS {
        (*event).dest.client = SNDRV_SEQ_ADDRESS_SUBSCRIBERS;
        (*event).queue = SNDRV_SEQ_QUEUE_DIRECT;
    } else if (*event).dest.client == SNDRV_SEQ_ADDRESS_SUBSCRIBERS {
        let src_port = snd_seq_port_use_ptr(client, (*event).source.port as c_int);
        if src_port.is_null() { return -EINVAL; }
    }
    if snd_seq_ev_is_direct(event) {
        if !snd_seq_ev_is_ump(event) && (*event).type_ == SNDRV_SEQ_EVENT_NOTE { return -EINVAL; }
        return snd_seq_deliver_event(client, event, atomic, hop);
    }
    if snd_seq_queue_is_used((*event).queue as c_int, (*client).number) <= 0 { return -EINVAL; }
    if snd_seq_write_pool_allocated(client) == 0 { return -ENXIO; }
    let mut cell: *mut snd_seq_event_cell = ptr::null_mut();
    let mut err = snd_seq_event_dup((*client).pool, event, &mut cell, (!blocking != 0 || atomic != 0) as c_int, file, mutexp);
    if err < 0 { return err; }
    err = snd_seq_enqueue_event(cell, atomic, hop);
    if err < 0 { snd_seq_cell_free(cell); return err; }
    0
}

unsafe fn check_event_type_and_length(ev: *mut snd_seq_event) -> c_int {
    match snd_seq_ev_length_type(ev) {
        x if x == SNDRV_SEQ_EVENT_LENGTH_FIXED => if snd_seq_ev_is_variable_type(ev) { return -EINVAL; },
        x if x == SNDRV_SEQ_EVENT_LENGTH_VARIABLE => if !snd_seq_ev_is_variable_type(ev) || ((*ev).data.ext.len & !SNDRV_SEQ_EXT_MASK) >= SNDRV_SEQ_MAX_EVENT_LEN { return -EINVAL; },
        x if x == SNDRV_SEQ_EVENT_LENGTH_VARUSR => if !snd_seq_ev_is_direct(ev) { return -EINVAL; },
        _ => {}
    }
    0
}

unsafe extern "C" fn snd_seq_write(file: *mut file, mut buf: *const c_char, mut count: size_t, _offset: *mut loff_t) -> ssize_t {
    let client = (*file).private_data as *mut snd_seq_client;
    let mut written: c_int = 0;
    if snd_seq_file_flags(file) & SNDRV_SEQ_LFLG_OUTPUT == 0 { return -ENXIO as ssize_t; }
    if snd_BUG_ON(client.is_null()) { return -ENXIO as ssize_t; }
    if (*client).accept_output == 0 || (*client).pool.is_null() { return -ENXIO as ssize_t; }
    let mut err: c_int;
    'repeat: loop {
        let mut handled = 0;
        mutex_lock(&mut (*client).ioctl_mutex);
        if (*(*client).pool).size > 0 && snd_seq_write_pool_allocated(client) == 0 {
            err = snd_seq_pool_init((*client).pool);
            if err < 0 { break 'repeat; }
        }
        err = -EINVAL;
        while count >= mem::size_of::<snd_seq_event>() {
            let mut event: __snd_seq_event = mem::zeroed();
            let ev = &mut event.legacy as *mut snd_seq_event;
            let mut len = mem::size_of::<snd_seq_event>();
            if copy_from_user(ev as *mut c_void, buf as *const c_void, len) != 0 { err = -EFAULT; break; }
            if snd_seq_ev_is_ump(ev) {
                if count < mem::size_of::<snd_seq_ump_event>() { break; }
                if copy_from_user((ev as *mut c_char).add(len) as *mut c_void, buf.add(len) as *const c_void, mem::size_of::<snd_seq_ump_event>() - len) != 0 { err = -EFAULT; break; }
                len = mem::size_of::<snd_seq_ump_event>();
            }
            (*ev).source.client = (*client).number as c_uchar;
            if check_event_type_and_length(ev) != 0 || !event_is_compatible(client, ev) { err = -EINVAL; break; }
            if !snd_seq_ev_is_ump(ev) {
                if (*ev).type_ == SNDRV_SEQ_EVENT_NONE { count -= len; buf = buf.add(len); written += len as c_int; continue; }
                if snd_seq_ev_is_reserved(ev) { err = -EINVAL; break; }
            }
            if snd_seq_ev_is_variable(ev) {
                let extlen = ((*ev).data.ext.len & !SNDRV_SEQ_EXT_MASK) as usize;
                if extlen + len > count { err = -EINVAL; break; }
                (*ev).data.ext.len = extlen as c_uint | SNDRV_SEQ_EXT_USRPTR;
                (*ev).data.ext.ptr = buf.add(len) as *mut c_void;
                len += extlen;
            }
            err = snd_seq_client_enqueue_event(client, ev, file, (((*file).f_flags & O_NONBLOCK) == 0) as c_int, 0, 0, &mut (*client).ioctl_mutex);
            if err < 0 { break; }
            handled += 1;
            count -= len; buf = buf.add(len); written += len as c_int;
            if handled >= 200 { mutex_unlock(&mut (*client).ioctl_mutex); continue 'repeat; }
        }
        break;
    }
    mutex_unlock(&mut (*client).ioctl_mutex);
    if written != 0 { written as ssize_t } else { err as ssize_t }
}

unsafe extern "C" fn snd_seq_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    let client = (*file).private_data as *mut snd_seq_client;
    let mut mask: __poll_t = 0;
    if snd_BUG_ON(client.is_null()) { return EPOLLERR; }
    if (snd_seq_file_flags(file) & SNDRV_SEQ_LFLG_INPUT) != 0 && !(*client).data.user.fifo.is_null() {
        if snd_seq_fifo_poll_wait((*client).data.user.fifo, file, wait) { mask |= EPOLLIN | EPOLLRDNORM; }
    }
    if (snd_seq_file_flags(file) & SNDRV_SEQ_LFLG_OUTPUT) != 0 {
        if snd_seq_pool_poll_wait((*client).pool, file, wait) { mask |= EPOLLOUT | EPOLLWRNORM; }
    }
    mask
}

type ioctl_func = unsafe fn(*mut snd_seq_client, *mut c_void) -> c_int;
#[repr(C)] struct ioctl_handler { cmd: c_uint, func: Option<ioctl_func> }

unsafe fn snd_seq_ioctl_pversion(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int { *(arg as *mut c_int) = SNDRV_SEQ_VERSION as c_int; 0 }
unsafe fn snd_seq_ioctl_user_pversion(client: *mut snd_seq_client, arg: *mut c_void) -> c_int { (*client).user_pversion = *(arg as *mut c_uint); 0 }
unsafe fn snd_seq_ioctl_client_id(client: *mut snd_seq_client, arg: *mut c_void) -> c_int { *(arg as *mut c_int) = (*client).number; 0 }
unsafe fn snd_seq_ioctl_system_info(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_system_info;
    memset(info as *mut c_void, 0, mem::size_of::<snd_seq_system_info>());
    (*info).queues = SNDRV_SEQ_MAX_QUEUES; (*info).clients = SNDRV_SEQ_MAX_CLIENTS; (*info).ports = SNDRV_SEQ_MAX_PORTS; (*info).channels = 256;
    (*info).cur_clients = client_usage.cur; (*info).cur_queues = snd_seq_queue_get_cur_queues(); 0
}
unsafe fn snd_seq_ioctl_running_mode(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_running_info;
    let cptr = client_load_and_use_ptr((*info).client);
    if cptr.is_null() { return -ENOENT; }
    if (*info).cpu_mode as usize > mem::size_of::<c_long>() { snd_seq_client_unref(cptr); return -EINVAL; }
    (*cptr).convert32 = ((*info).cpu_mode as usize) < mem::size_of::<c_long>() as c_int;
    snd_seq_client_unref(cptr); 0
}
unsafe fn get_client_info(cptr: *mut snd_seq_client, info: *mut snd_seq_client_info) {
    (*info).client = (*cptr).number;
    (*info).type_ = (*cptr).type_;
    strscpy((*info).name.as_mut_ptr(), (*cptr).name.as_ptr(), (*info).name.len());
    (*info).filter = (*cptr).filter; (*info).event_lost = (*cptr).event_lost;
    memcpy((*info).event_filter.as_mut_ptr() as *mut c_void, (*cptr).event_filter.as_ptr() as *const c_void, 32);
    (*info).group_filter = (*cptr).group_filter; (*info).num_ports = (*cptr).num_ports;
    (*info).pid = if (*cptr).type_ == USER_CLIENT { pid_vnr((*cptr).data.user.owner) } else { -1 };
    (*info).card = if (*cptr).type_ == KERNEL_CLIENT && !(*cptr).data.kernel.card.is_null() { (*(*cptr).data.kernel.card).number } else { -1 };
    (*info).midi_version = (*cptr).midi_version;
    memset((*info).reserved.as_mut_ptr() as *mut c_void, 0, (*info).reserved.len());
}
unsafe fn snd_seq_ioctl_get_client_info(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_client_info;
    let cptr = client_load_and_use_ptr((*info).client);
    if cptr.is_null() { return -ENOENT; }
    get_client_info(cptr, info);
    snd_seq_client_unref(cptr); 0
}
unsafe fn snd_seq_ioctl_set_client_info(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_client_info;
    if (*client).number != (*info).client { return -EPERM; }
    if (*client).type_ != (*info).type_ { return -EINVAL; }
    if (*info).name[0] != 0 { strscpy((*client).name.as_mut_ptr(), (*info).name.as_ptr(), (*client).name.len()); }
    (*client).filter = (*info).filter; (*client).event_lost = (*info).event_lost; (*client).midi_version = (*info).midi_version;
    memcpy((*client).event_filter.as_mut_ptr() as *mut c_void, (*info).event_filter.as_ptr() as *const c_void, 32);
    (*client).group_filter = (*info).group_filter & SND_SEQ_GROUP_FILTER_MASK;
    snd_seq_system_client_ev_client_change((*client).number); 0
}

unsafe fn snd_seq_ioctl_create_port(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_port_info;
    if (*info).addr.client as c_int != (*client).number { return -EPERM; }
    if (*client).type_ == USER_CLIENT && !(*info).kernel.is_null() { return -EINVAL; }
    if ((*info).capability & SNDRV_SEQ_PORT_CAP_UMP_ENDPOINT) != 0 && (*client).ump_endpoint_port >= 0 { return -EBUSY; }
    let port_idx = if ((*info).flags & SNDRV_SEQ_PORT_FLG_GIVEN_PORT) != 0 { (*info).addr.port as c_int } else { -1 };
    if port_idx >= SNDRV_SEQ_ADDRESS_UNKNOWN { return -EINVAL; }
    let mut port: *mut snd_seq_client_port = ptr::null_mut();
    let mut err = snd_seq_create_port(client, &mut port);
    if err < 0 { return err; }
    if (*client).type_ == KERNEL_CLIENT {
        let cb = (*info).kernel;
        if !cb.is_null() {
            if !(*cb).owner.is_null() { (*port).owner = (*cb).owner; }
            (*port).private_data = (*cb).private_data; (*port).private_free = (*cb).private_free; (*port).event_input = (*cb).event_input;
            (*port).c_src.open = (*cb).subscribe; (*port).c_src.close = (*cb).unsubscribe; (*port).c_dest.open = (*cb).use_; (*port).c_dest.close = (*cb).unuse;
        }
    }
    snd_seq_set_port_info(port, info);
    err = snd_seq_insert_port(client, port_idx, port);
    if err < 0 { kfree(port as *mut c_void); return err; }
    (*info).addr = (*port).addr;
    if ((*info).capability & SNDRV_SEQ_PORT_CAP_UMP_ENDPOINT) != 0 { (*client).ump_endpoint_port = (*port).addr.port as c_int; }
    snd_seq_system_client_ev_port_start((*port).addr.client as c_int, (*port).addr.port as c_int);
    snd_seq_port_unlock(port); 0
}
unsafe fn snd_seq_ioctl_delete_port(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_port_info;
    if (*info).addr.client as c_int != (*client).number { return -EPERM; }
    let err = snd_seq_delete_port(client, (*info).addr.port as c_int);
    if err >= 0 { if (*client).ump_endpoint_port == (*info).addr.port as c_int { (*client).ump_endpoint_port = -1; } snd_seq_system_client_ev_port_exit((*client).number, (*info).addr.port as c_int); }
    err
}
unsafe fn snd_seq_ioctl_get_port_info(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_port_info;
    let cptr = client_load_and_use_ptr((*info).addr.client as c_int);
    if cptr.is_null() { return -ENXIO; }
    let port = snd_seq_port_use_ptr(cptr, (*info).addr.port as c_int);
    if port.is_null() { snd_seq_client_unref(cptr); return -ENOENT; }
    snd_seq_get_port_info(port, info); snd_seq_client_unref(cptr); 0
}
unsafe fn snd_seq_ioctl_set_port_info(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_port_info;
    if (*info).addr.client as c_int != (*client).number { return -EPERM; }
    let port = snd_seq_port_use_ptr(client, (*info).addr.port as c_int);
    if !port.is_null() { snd_seq_set_port_info(port, info); snd_seq_system_client_ev_port_change((*info).addr.client as c_int, (*info).addr.port as c_int); }
    0
}

unsafe fn check_subscription_permission(client: *mut snd_seq_client, sport: *mut snd_seq_client_port, dport: *mut snd_seq_client_port, subs: *mut snd_seq_port_subscribe) -> c_int {
    if (*client).number != (*subs).sender.client as c_int && (*client).number != (*subs).dest.client as c_int {
        if check_port_perm(sport, SNDRV_SEQ_PORT_CAP_NO_EXPORT) != 0 { return -EPERM; }
        if check_port_perm(dport, SNDRV_SEQ_PORT_CAP_NO_EXPORT) != 0 { return -EPERM; }
    }
    if (*client).number != (*subs).sender.client as c_int && check_port_perm(sport, PERM_RD) == 0 { return -EPERM; }
    if (*client).number != (*subs).dest.client as c_int && check_port_perm(dport, PERM_WR) == 0 { return -EPERM; }
    0
}
#[no_mangle]
pub unsafe extern "C" fn snd_seq_client_notify_subscription(client: c_int, port: c_int, info: *mut snd_seq_port_subscribe, evtype: c_int) -> c_int {
    let mut event: snd_seq_event = mem::zeroed();
    event.type_ = evtype as c_uchar;
    event.data.connect.dest = (*info).dest;
    event.data.connect.sender = (*info).sender;
    snd_seq_system_notify(client, port, &mut event, false)
}
unsafe fn snd_seq_ioctl_subscribe_port(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let subs = arg as *mut snd_seq_port_subscribe;
    let receiver = client_load_and_use_ptr((*subs).dest.client as c_int); if receiver.is_null() { return -EINVAL; }
    let sender = client_load_and_use_ptr((*subs).sender.client as c_int); if sender.is_null() { return -EINVAL; }
    let sport = snd_seq_port_use_ptr(sender, (*subs).sender.port as c_int); if sport.is_null() { return -EINVAL; }
    let dport = snd_seq_port_use_ptr(receiver, (*subs).dest.port as c_int); if dport.is_null() { return -EINVAL; }
    let mut result = check_subscription_permission(client, sport, dport, subs);
    if result < 0 { return result; }
    result = snd_seq_port_connect(client, sender, sport, receiver, dport, subs);
    if result == 0 { snd_seq_client_notify_subscription(SNDRV_SEQ_ADDRESS_SUBSCRIBERS as c_int, 0, subs, SNDRV_SEQ_EVENT_PORT_SUBSCRIBED); }
    result
}
unsafe fn snd_seq_ioctl_unsubscribe_port(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let subs = arg as *mut snd_seq_port_subscribe;
    let receiver = snd_seq_client_use_ptr((*subs).dest.client as c_int); if receiver.is_null() { return -ENXIO; }
    let sender = snd_seq_client_use_ptr((*subs).sender.client as c_int); if sender.is_null() { return -ENXIO; }
    let sport = snd_seq_port_use_ptr(sender, (*subs).sender.port as c_int); if sport.is_null() { return -ENXIO; }
    let dport = snd_seq_port_use_ptr(receiver, (*subs).dest.port as c_int); if dport.is_null() { return -ENXIO; }
    let mut result = check_subscription_permission(client, sport, dport, subs);
    if result < 0 { return result; }
    result = snd_seq_port_disconnect(client, sender, sport, receiver, dport, subs);
    if result == 0 { snd_seq_client_notify_subscription(SNDRV_SEQ_ADDRESS_SUBSCRIBERS as c_int, 0, subs, SNDRV_SEQ_EVENT_PORT_UNSUBSCRIBED); }
    result
}

unsafe fn snd_seq_ioctl_create_queue(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_queue_info;
    let q = snd_seq_queue_alloc((*client).number, (*info).locked, (*info).flags);
    if IS_ERR(q as *const c_void) { return PTR_ERR(q as *const c_void); }
    (*info).queue = (*q).queue; (*info).locked = (*q).locked; (*info).owner = (*q).owner;
    if (*info).name[0] == 0 { snprintf((*info).name.as_mut_ptr(), (*info).name.len(), c"Queue-%d".as_ptr(), (*q).queue); }
    strscpy((*q).name.as_mut_ptr(), (*info).name.as_ptr(), (*q).name.len()); 0
}
unsafe fn snd_seq_ioctl_delete_queue(client: *mut snd_seq_client, arg: *mut c_void) -> c_int { snd_seq_queue_delete((*client).number, (*(arg as *mut snd_seq_queue_info)).queue) }
unsafe fn snd_seq_ioctl_get_queue_info(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_queue_info; let q = queueptr((*info).queue); if q.is_null() { return -EINVAL; }
    memset(info as *mut c_void, 0, mem::size_of::<snd_seq_queue_info>()); (*info).queue = (*q).queue; (*info).owner = (*q).owner; (*info).locked = (*q).locked; strscpy((*info).name.as_mut_ptr(), (*q).name.as_ptr(), (*info).name.len()); 0
}
unsafe fn snd_seq_ioctl_set_queue_info(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_queue_info;
    if (*info).owner != (*client).number { return -EINVAL; }
    if snd_seq_queue_check_access((*info).queue, (*client).number) {
        if snd_seq_queue_set_owner((*info).queue, (*client).number, (*info).locked) < 0 { return -EPERM; }
        if (*info).locked != 0 { snd_seq_queue_use((*info).queue, (*client).number, 1); }
    } else { return -EPERM; }
    let q = queueptr((*info).queue); if q.is_null() { return -EINVAL; }
    if (*q).owner != (*client).number { return -EPERM; }
    strscpy((*q).name.as_mut_ptr(), (*info).name.as_ptr(), (*q).name.len()); 0
}
unsafe fn snd_seq_ioctl_get_named_queue(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_queue_info; let q = snd_seq_queue_find_name((*info).name.as_ptr()); if q.is_null() { return -EINVAL; }
    (*info).queue = (*q).queue; (*info).owner = (*q).owner; (*info).locked = (*q).locked; 0
}
unsafe fn snd_seq_ioctl_get_queue_status(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let status = arg as *mut snd_seq_queue_status; let queue = queueptr((*status).queue); if queue.is_null() { return -EINVAL; }
    memset(status as *mut c_void, 0, mem::size_of::<snd_seq_queue_status>()); (*status).queue = (*queue).queue;
    (*status).events = (*(*queue).tickq).cells + (*(*queue).timeq).cells; (*status).time = snd_seq_timer_get_cur_time((*queue).timer, true); (*status).tick = snd_seq_timer_get_cur_tick((*queue).timer); (*status).running = (*(*queue).timer).running; (*status).flags = (*queue).flags; 0
}
unsafe fn snd_seq_ioctl_get_queue_tempo(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let tempo = arg as *mut snd_seq_queue_tempo; let queue = queueptr((*tempo).queue); if queue.is_null() { return -EINVAL; }
    memset(tempo as *mut c_void, 0, mem::size_of::<snd_seq_queue_tempo>()); (*tempo).queue = (*queue).queue;
    let tmr = (*queue).timer; (*tempo).tempo = (*tmr).tempo; (*tempo).ppq = (*tmr).ppq; (*tempo).skew_value = (*tmr).skew; (*tempo).skew_base = (*tmr).skew_base;
    if (*client).user_pversion >= SNDRV_PROTOCOL_VERSION(1,0,4) { (*tempo).tempo_base = (*tmr).tempo_base; } 0
}
const fn SNDRV_PROTOCOL_VERSION(a: c_uint, b: c_uint, c: c_uint) -> c_uint { (a << 16) | (b << 8) | c }
#[no_mangle]
pub unsafe extern "C" fn snd_seq_set_queue_tempo(client: c_int, tempo: *mut snd_seq_queue_tempo) -> c_int {
    if !snd_seq_queue_check_access((*tempo).queue, client) { return -EPERM; }
    snd_seq_queue_timer_set_tempo((*tempo).queue, client, tempo)
}
unsafe fn snd_seq_ioctl_set_queue_tempo(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let tempo = arg as *mut snd_seq_queue_tempo; if (*client).user_pversion < SNDRV_PROTOCOL_VERSION(1,0,4) { (*tempo).tempo_base = 0; }
    let r = snd_seq_set_queue_tempo((*client).number, tempo); if r < 0 { r } else { 0 }
}
unsafe fn snd_seq_ioctl_get_queue_timer(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let timer = arg as *mut snd_seq_queue_timer; let queue = queueptr((*timer).queue); if queue.is_null() { return -EINVAL; }
    let tmr = (*queue).timer; memset(timer as *mut c_void, 0, mem::size_of::<snd_seq_queue_timer>()); (*timer).queue = (*queue).queue; (*timer).type_ = (*tmr).type_;
    if (*tmr).type_ == SNDRV_SEQ_TIMER_ALSA { (*timer).u.alsa.id = (*tmr).alsa_id; (*timer).u.alsa.resolution = (*tmr).preferred_resolution; } 0
}
unsafe fn snd_seq_ioctl_set_queue_timer(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let timer = arg as *mut snd_seq_queue_timer; if (*timer).type_ != SNDRV_SEQ_TIMER_ALSA { return -EINVAL; }
    if !snd_seq_queue_check_access((*timer).queue, (*client).number) { return -EPERM; }
    let q = queueptr((*timer).queue); if q.is_null() { return -ENXIO; }
    let tmr = (*q).timer; snd_seq_queue_timer_close((*timer).queue); (*tmr).type_ = (*timer).type_;
    if (*tmr).type_ == SNDRV_SEQ_TIMER_ALSA { (*tmr).alsa_id = (*timer).u.alsa.id; (*tmr).preferred_resolution = (*timer).u.alsa.resolution; }
    snd_seq_queue_timer_open((*timer).queue)
}
unsafe fn snd_seq_ioctl_get_queue_client(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_queue_client; let used = snd_seq_queue_is_used((*info).queue, (*client).number); if used < 0 { return -EINVAL; }
    (*info).used = used; (*info).client = (*client).number; 0
}
unsafe fn snd_seq_ioctl_set_queue_client(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_queue_client; if (*info).used >= 0 { let err = snd_seq_queue_use((*info).queue, (*client).number, (*info).used); if err < 0 { return err; } }
    snd_seq_ioctl_get_queue_client(client, arg)
}
unsafe fn snd_seq_ioctl_get_client_pool(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_client_pool; let cptr = client_load_and_use_ptr((*info).client); if cptr.is_null() { return -ENOENT; }
    memset(info as *mut c_void, 0, mem::size_of::<snd_seq_client_pool>()); (*info).client = (*cptr).number; (*info).output_pool = (*(*cptr).pool).size; (*info).output_room = (*(*cptr).pool).room; (*info).output_free = snd_seq_unused_cells((*cptr).pool);
    if (*cptr).type_ == USER_CLIENT { (*info).input_pool = (*cptr).data.user.fifo_pool_size; (*info).input_free = snd_seq_fifo_unused_cells((*cptr).data.user.fifo); }
    snd_seq_client_unref(cptr); 0
}
unsafe fn snd_seq_ioctl_set_client_pool(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_client_pool;
    if (*client).number != (*info).client { return -EINVAL; }
    if (*info).output_pool >= 1 && (*info).output_pool <= SNDRV_SEQ_MAX_EVENTS && (snd_seq_write_pool_allocated(client) == 0 || (*info).output_pool != (*(*client).pool).size) {
        if snd_seq_write_pool_allocated(client) != 0 {
            if atomic_read(&(*(*client).pool).counter) != 0 { return -EBUSY; }
            snd_seq_pool_mark_closing((*client).pool); snd_seq_pool_done((*client).pool);
        }
        (*(*client).pool).size = (*info).output_pool; let rc = snd_seq_pool_init((*client).pool); if rc < 0 { return rc; }
    }
    if (*client).type_ == USER_CLIENT && !(*client).data.user.fifo.is_null() && (*info).input_pool >= 1 && (*info).input_pool <= SNDRV_SEQ_MAX_CLIENT_EVENTS && (*info).input_pool != (*client).data.user.fifo_pool_size {
        let rc = snd_seq_fifo_resize((*client).data.user.fifo, (*info).input_pool); if rc < 0 { return rc; } (*client).data.user.fifo_pool_size = (*info).input_pool;
    }
    if (*info).output_room >= 1 && (*info).output_room <= (*(*client).pool).size { (*(*client).pool).room = (*info).output_room; }
    snd_seq_ioctl_get_client_pool(client, arg)
}
unsafe fn snd_seq_ioctl_remove_events(client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_remove_events;
    if ((*info).remove_mode & SNDRV_SEQ_REMOVE_INPUT) != 0 && (*client).type_ == USER_CLIENT && !(*client).data.user.fifo.is_null() { snd_seq_fifo_clear((*client).data.user.fifo); }
    if ((*info).remove_mode & SNDRV_SEQ_REMOVE_OUTPUT) != 0 { snd_seq_queue_remove_cells((*client).number, info); }
    0
}
unsafe fn snd_seq_ioctl_get_subscription(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let subs = arg as *mut snd_seq_port_subscribe; let sender = client_load_and_use_ptr((*subs).sender.client as c_int); if sender.is_null() { return -EINVAL; }
    let sport = snd_seq_port_use_ptr(sender, (*subs).sender.port as c_int); if sport.is_null() { return -EINVAL; }
    snd_seq_port_get_subscription(&mut (*sport).c_src, &mut (*subs).dest, subs)
}
unsafe fn snd_seq_ioctl_query_subs(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let subs = arg as *mut snd_seq_query_subs; let cptr = client_load_and_use_ptr((*subs).root.client as c_int); if cptr.is_null() { return -ENXIO; }
    let port = snd_seq_port_use_ptr(cptr, (*subs).root.port as c_int); if port.is_null() { return -ENXIO; }
    let group = match (*subs).type_ { x if x == SNDRV_SEQ_QUERY_SUBS_READ => &mut (*port).c_src, x if x == SNDRV_SEQ_QUERY_SUBS_WRITE => &mut (*port).c_dest, _ => return -ENXIO };
    (*subs).num_subs = group.count;
    let mut i = 0; let mut p = group.list_head.first;
    while !p.is_null() {
        if i == (*subs).index {
            let s = p as *mut snd_seq_subscribers;
            if (*subs).type_ == SNDRV_SEQ_QUERY_SUBS_READ { (*subs).addr = (*s).info.dest; } else { (*subs).addr = (*s).info.sender; }
            (*subs).flags = (*s).info.flags; (*subs).queue = (*s).info.queue; return 0;
        }
        i += 1; p = (*p).next;
    }
    -ENOENT
}
unsafe fn snd_seq_ioctl_query_next_client(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_client_info;
    if (*info).client < INT_MAX { (*info).client += 1; }
    if (*info).client < 0 { (*info).client = 0; }
    while (*info).client < SNDRV_SEQ_MAX_CLIENTS {
        let cptr = client_load_and_use_ptr((*info).client);
        if !cptr.is_null() { get_client_info(cptr, info); snd_seq_client_unref(cptr); return 0; }
        (*info).client += 1;
    }
    -ENOENT
}
unsafe fn snd_seq_ioctl_query_next_port(_client: *mut snd_seq_client, arg: *mut c_void) -> c_int {
    let info = arg as *mut snd_seq_port_info; let cptr = client_load_and_use_ptr((*info).addr.client as c_int); if cptr.is_null() { return -ENXIO; }
    (*info).addr.port = (*info).addr.port.wrapping_add(1); let port = snd_seq_port_query_nearest(cptr, info); if port.is_null() { return -ENOENT; }
    (*info).addr = (*port).addr; snd_seq_get_port_info(port, info); 0
}

unsafe fn free_ump_info(client: *mut snd_seq_client) {
    if (*client).ump_info.is_null() { return; }
    let mut i = 0; while i < NUM_UMP_INFOS { kfree(*(*client).ump_info.add(i as usize)); i += 1; }
    kfree((*client).ump_info as *mut c_void); (*client).ump_info = ptr::null_mut();
}
unsafe fn terminate_ump_info_strings(p: *mut c_void, type_: c_int) {
    if type_ == SNDRV_SEQ_CLIENT_UMP_INFO_ENDPOINT {
        let ep = p as *mut snd_ump_endpoint_info; (*ep).name[(*ep).name.len() - 1] = 0;
    } else {
        let bp = p as *mut snd_ump_block_info; (*bp).name[(*bp).name.len() - 1] = 0;
    }
}
unsafe fn snd_seq_ioctl_client_ump_info(caller: *mut snd_seq_client, cmd: c_uint, arg: c_ulong) -> c_int {
    let argp = arg as *mut snd_seq_client_ump_info;
    let client = (*argp).client; let type_ = (*argp).type_; let mut err = 0;
    if cmd == SNDRV_SEQ_IOCTL_SET_CLIENT_UMP_INFO && (*caller).number != client { return -EPERM; }
    if type_ < 0 || type_ >= NUM_UMP_INFOS { return -EINVAL; }
    let size = if type_ == SNDRV_SEQ_CLIENT_UMP_INFO_ENDPOINT { mem::size_of::<snd_ump_endpoint_info>() } else { mem::size_of::<snd_ump_block_info>() };
    let cptr = client_load_and_use_ptr(client); if cptr.is_null() { return -ENOENT; }
    mutex_lock(&mut (*cptr).ioctl_mutex);
    if (*cptr).midi_version == 0 { err = -EBADFD; }
    else if cmd == SNDRV_SEQ_IOCTL_GET_CLIENT_UMP_INFO {
        let p = if (*cptr).ump_info.is_null() { ptr::null_mut() } else { *(*cptr).ump_info.add(type_ as usize) };
        if p.is_null() { err = -ENODEV; } else if copy_to_user((*argp).info, p, size) != 0 { err = -EFAULT; }
    } else {
        if (*cptr).type_ != USER_CLIENT { err = -EBADFD; }
        else {
            if (*cptr).ump_info.is_null() { (*cptr).ump_info = kcalloc(NUM_UMP_INFOS as usize, mem::size_of::<*mut c_void>(), GFP_KERNEL) as *mut *mut c_void; if (*cptr).ump_info.is_null() { err = -ENOMEM; } }
            if err == 0 {
                let p = memdup_user((*argp).info, size);
                if IS_ERR(p) { err = PTR_ERR(p); } else { kfree(*(*cptr).ump_info.add(type_ as usize)); terminate_ump_info_strings(p, type_); *(*cptr).ump_info.add(type_ as usize) = p; }
            }
        }
    }
    mutex_unlock(&mut (*cptr).ioctl_mutex);
    snd_seq_client_unref(cptr);
    if err == 0 && cmd == SNDRV_SEQ_IOCTL_SET_CLIENT_UMP_INFO {
        if type_ == SNDRV_SEQ_CLIENT_UMP_INFO_ENDPOINT { snd_seq_system_ump_notify(client, 0, SNDRV_SEQ_EVENT_UMP_EP_CHANGE, false); }
        else { snd_seq_system_ump_notify(client, type_ - 1, SNDRV_SEQ_EVENT_UMP_BLOCK_CHANGE, false); }
    }
    err
}

extern "C" {
    static SNDRV_SEQ_IOCTL_PVERSION: c_uint; static SNDRV_SEQ_IOCTL_USER_PVERSION: c_uint; static SNDRV_SEQ_IOCTL_CLIENT_ID: c_uint; static SNDRV_SEQ_IOCTL_SYSTEM_INFO: c_uint;
    static SNDRV_SEQ_IOCTL_RUNNING_MODE: c_uint; static SNDRV_SEQ_IOCTL_GET_CLIENT_INFO: c_uint; static SNDRV_SEQ_IOCTL_SET_CLIENT_INFO: c_uint; static SNDRV_SEQ_IOCTL_CREATE_PORT: c_uint;
    static SNDRV_SEQ_IOCTL_DELETE_PORT: c_uint; static SNDRV_SEQ_IOCTL_GET_PORT_INFO: c_uint; static SNDRV_SEQ_IOCTL_SET_PORT_INFO: c_uint; static SNDRV_SEQ_IOCTL_SUBSCRIBE_PORT: c_uint;
    static SNDRV_SEQ_IOCTL_UNSUBSCRIBE_PORT: c_uint; static SNDRV_SEQ_IOCTL_CREATE_QUEUE: c_uint; static SNDRV_SEQ_IOCTL_DELETE_QUEUE: c_uint; static SNDRV_SEQ_IOCTL_GET_QUEUE_INFO: c_uint;
    static SNDRV_SEQ_IOCTL_SET_QUEUE_INFO: c_uint; static SNDRV_SEQ_IOCTL_GET_NAMED_QUEUE: c_uint; static SNDRV_SEQ_IOCTL_GET_QUEUE_STATUS: c_uint; static SNDRV_SEQ_IOCTL_GET_QUEUE_TEMPO: c_uint;
    static SNDRV_SEQ_IOCTL_SET_QUEUE_TEMPO: c_uint; static SNDRV_SEQ_IOCTL_GET_QUEUE_TIMER: c_uint; static SNDRV_SEQ_IOCTL_SET_QUEUE_TIMER: c_uint; static SNDRV_SEQ_IOCTL_GET_QUEUE_CLIENT: c_uint;
    static SNDRV_SEQ_IOCTL_SET_QUEUE_CLIENT: c_uint; static SNDRV_SEQ_IOCTL_GET_CLIENT_POOL: c_uint; static SNDRV_SEQ_IOCTL_SET_CLIENT_POOL: c_uint; static SNDRV_SEQ_IOCTL_GET_SUBSCRIPTION: c_uint;
    static SNDRV_SEQ_IOCTL_QUERY_NEXT_CLIENT: c_uint; static SNDRV_SEQ_IOCTL_QUERY_NEXT_PORT: c_uint; static SNDRV_SEQ_IOCTL_REMOVE_EVENTS: c_uint; static SNDRV_SEQ_IOCTL_QUERY_SUBS: c_uint;
    fn _IOC_SIZE(cmd: c_uint) -> c_ulong;
    fn _IOC_TYPE(cmd: c_uint) -> c_int;
    fn _IOC_NR(cmd: c_uint) -> c_int;
}

static ioctl_handlers: [ioctl_handler; 33] = [
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_PVERSION }, func: Some(snd_seq_ioctl_pversion) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_USER_PVERSION }, func: Some(snd_seq_ioctl_user_pversion) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_CLIENT_ID }, func: Some(snd_seq_ioctl_client_id) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_SYSTEM_INFO }, func: Some(snd_seq_ioctl_system_info) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_RUNNING_MODE }, func: Some(snd_seq_ioctl_running_mode) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_GET_CLIENT_INFO }, func: Some(snd_seq_ioctl_get_client_info) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_SET_CLIENT_INFO }, func: Some(snd_seq_ioctl_set_client_info) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_CREATE_PORT }, func: Some(snd_seq_ioctl_create_port) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_DELETE_PORT }, func: Some(snd_seq_ioctl_delete_port) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_GET_PORT_INFO }, func: Some(snd_seq_ioctl_get_port_info) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_SET_PORT_INFO }, func: Some(snd_seq_ioctl_set_port_info) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_SUBSCRIBE_PORT }, func: Some(snd_seq_ioctl_subscribe_port) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_UNSUBSCRIBE_PORT }, func: Some(snd_seq_ioctl_unsubscribe_port) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_CREATE_QUEUE }, func: Some(snd_seq_ioctl_create_queue) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_DELETE_QUEUE }, func: Some(snd_seq_ioctl_delete_queue) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_GET_QUEUE_INFO }, func: Some(snd_seq_ioctl_get_queue_info) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_SET_QUEUE_INFO }, func: Some(snd_seq_ioctl_set_queue_info) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_GET_NAMED_QUEUE }, func: Some(snd_seq_ioctl_get_named_queue) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_GET_QUEUE_STATUS }, func: Some(snd_seq_ioctl_get_queue_status) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_GET_QUEUE_TEMPO }, func: Some(snd_seq_ioctl_get_queue_tempo) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_SET_QUEUE_TEMPO }, func: Some(snd_seq_ioctl_set_queue_tempo) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_GET_QUEUE_TIMER }, func: Some(snd_seq_ioctl_get_queue_timer) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_SET_QUEUE_TIMER }, func: Some(snd_seq_ioctl_set_queue_timer) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_GET_QUEUE_CLIENT }, func: Some(snd_seq_ioctl_get_queue_client) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_SET_QUEUE_CLIENT }, func: Some(snd_seq_ioctl_set_queue_client) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_GET_CLIENT_POOL }, func: Some(snd_seq_ioctl_get_client_pool) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_SET_CLIENT_POOL }, func: Some(snd_seq_ioctl_set_client_pool) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_GET_SUBSCRIPTION }, func: Some(snd_seq_ioctl_get_subscription) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_QUERY_NEXT_CLIENT }, func: Some(snd_seq_ioctl_query_next_client) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_QUERY_NEXT_PORT }, func: Some(snd_seq_ioctl_query_next_port) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_REMOVE_EVENTS }, func: Some(snd_seq_ioctl_remove_events) },
    ioctl_handler { cmd: unsafe { SNDRV_SEQ_IOCTL_QUERY_SUBS }, func: Some(snd_seq_ioctl_query_subs) },
    ioctl_handler { cmd: 0, func: None },
];

unsafe extern "C" fn snd_seq_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let client = (*file).private_data as *mut snd_seq_client;
    if snd_BUG_ON(client.is_null()) { return -ENXIO as c_long; }
    if cmd == SNDRV_SEQ_IOCTL_GET_CLIENT_UMP_INFO || cmd == SNDRV_SEQ_IOCTL_SET_CLIENT_UMP_INFO {
        return snd_seq_ioctl_client_ump_info(client, cmd, arg) as c_long;
    }
    let mut handler: *const ioctl_handler = ptr::null();
    for h in ioctl_handlers.iter() {
        if h.cmd == 0 || h.cmd == cmd { handler = h; break; }
    }
    if handler.is_null() || (*handler).cmd == 0 { return -ENOTTY as c_long; }
    let mut buf = [0u8; 256];
    memset(buf.as_mut_ptr() as *mut c_void, 0, buf.len());
    let size = _IOC_SIZE((*handler).cmd) as usize;
    if ((*handler).cmd & IOC_IN) != 0 && copy_from_user(buf.as_mut_ptr() as *mut c_void, arg as *const c_void, size) != 0 { return -EFAULT as c_long; }
    mutex_lock(&mut (*client).ioctl_mutex);
    let err = ((*handler).func.unwrap())(client, buf.as_mut_ptr() as *mut c_void);
    mutex_unlock(&mut (*client).ioctl_mutex);
    if err >= 0 && ((*handler).cmd == SNDRV_SEQ_IOCTL_SET_QUEUE_CLIENT || (*handler).cmd == SNDRV_SEQ_IOCTL_SET_CLIENT_POOL || ((*handler).cmd & IOC_OUT) != 0) {
        if copy_to_user(arg as *mut c_void, buf.as_ptr() as *const c_void, size) != 0 { return -EFAULT as c_long; }
    }
    err as c_long
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_create_kernel_client(card: *mut snd_card, mut client_index: c_int, name_fmt: *const c_char, _args: ...) -> c_int {
    if snd_BUG_ON(in_interrupt()) { return -EBUSY; }
    if !card.is_null() && client_index >= SNDRV_SEQ_CLIENTS_PER_CARD { return -EINVAL; }
    if card.is_null() && client_index >= SNDRV_SEQ_GLOBAL_CLIENTS { return -EINVAL; }
    mutex_lock(&mut register_mutex);
    if !card.is_null() {
        client_index += SNDRV_SEQ_GLOBAL_CLIENTS + (*card).number * SNDRV_SEQ_CLIENTS_PER_CARD;
        if client_index >= SNDRV_SEQ_DYNAMIC_CLIENTS_BEGIN { client_index = -1; }
    }
    let client = seq_create_client1(client_index, 0);
    if client.is_null() { mutex_unlock(&mut register_mutex); return -EBUSY; }
    usage_alloc(&mut client_usage, 1);
    (*client).accept_input = 1; (*client).accept_output = 1; (*client).data.kernel.card = card; (*client).user_pversion = SNDRV_SEQ_VERSION;
    /* C varargs are represented by the extern "C" variadic signature; va_list forwarding is target ABI dependent. */
    snprintf((*client).name.as_mut_ptr(), (*client).name.len(), name_fmt);
    (*client).type_ = KERNEL_CLIENT;
    mutex_unlock(&mut register_mutex);
    snd_seq_system_client_ev_client_start((*client).number);
    (*client).number
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_delete_kernel_client(client: c_int) -> c_int {
    if snd_BUG_ON(in_interrupt()) { return -EBUSY; }
    let ptr = clientptr(client); if ptr.is_null() { return -EINVAL; }
    seq_free_client(ptr); kfree(ptr as *mut c_void); 0
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_kernel_client_enqueue(client: c_int, ev: *mut snd_seq_event, file: *mut file, blocking: bool_t) -> c_int {
    if snd_BUG_ON(ev.is_null()) { return -EINVAL; }
    if !snd_seq_ev_is_ump(ev) {
        if (*ev).type_ == SNDRV_SEQ_EVENT_NONE { return 0; }
        if (*ev).type_ == SNDRV_SEQ_EVENT_KERNEL_ERROR { return -EINVAL; }
    }
    (*ev).source.client = client as c_uchar;
    if check_event_type_and_length(ev) != 0 { return -EINVAL; }
    let cptr = client_load_and_use_ptr(client); if cptr.is_null() { return -EINVAL; }
    if (*cptr).accept_output == 0 { snd_seq_client_unref(cptr); return -EPERM; }
    mutex_lock(&mut (*cptr).ioctl_mutex);
    let r = snd_seq_client_enqueue_event(cptr, ev, file, blocking as c_int, false as c_int, 0, &mut (*cptr).ioctl_mutex);
    mutex_unlock(&mut (*cptr).ioctl_mutex); snd_seq_client_unref(cptr); r
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_kernel_client_dispatch(client: c_int, ev: *mut snd_seq_event, atomic: c_int, hop: c_int) -> c_int {
    if snd_BUG_ON(ev.is_null()) { return -EINVAL; }
    (*ev).queue = SNDRV_SEQ_QUEUE_DIRECT; (*ev).source.client = client as c_uchar;
    if check_event_type_and_length(ev) != 0 { return -EINVAL; }
    let cptr = snd_seq_client_use_ptr(client); if cptr.is_null() { return -EINVAL; }
    let r = if (*cptr).accept_output == 0 { -EPERM } else { snd_seq_deliver_event(cptr, ev, atomic, hop) };
    snd_seq_client_unref(cptr); r
}

unsafe fn call_seq_client_ctl(client: *mut snd_seq_client, cmd: c_uint, arg: *mut c_void) -> c_int {
    for h in ioctl_handlers.iter() {
        if h.cmd == 0 { break; }
        if h.cmd == cmd { return (h.func.unwrap())(client, arg); }
    }
    pr_debug(c"ALSA: seq unknown ioctl() 0x%x (type='%c', number=0x%02x)\n".as_ptr(), cmd, _IOC_TYPE(cmd), _IOC_NR(cmd));
    -ENOTTY
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_kernel_client_ctl(clientid: c_int, cmd: c_uint, arg: *mut c_void) -> c_int {
    let client = clientptr(clientid); if client.is_null() { return -ENXIO; }
    call_seq_client_ctl(client, cmd, arg)
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_kernel_client_ioctl(clientid: c_int, cmd: c_uint, arg: *mut c_void) -> c_int {
    let client = client_load_and_use_ptr(clientid); if client.is_null() { return -ENXIO; }
    mutex_lock(&mut (*client).ioctl_mutex); let r = call_seq_client_ctl(client, cmd, arg); mutex_unlock(&mut (*client).ioctl_mutex); snd_seq_client_unref(client); r
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_kernel_client_write_poll(clientid: c_int, file: *mut file, wait: *mut poll_table) -> c_int {
    let client = clientptr(clientid); if client.is_null() { return -ENXIO; }
    if snd_seq_pool_poll_wait((*client).pool, file, wait) { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn snd_seq_kernel_client_get(id: c_int) -> *mut snd_seq_client { snd_seq_client_use_ptr(id) }

#[no_mangle]
pub unsafe extern "C" fn snd_seq_kernel_client_put(cptr: *mut snd_seq_client) { if !cptr.is_null() { snd_seq_client_unref(cptr); } }

unsafe fn FLAG_PERM_RD(perm: c_uint) -> c_char { if (perm & SNDRV_SEQ_PORT_CAP_READ) != 0 { if (perm & SNDRV_SEQ_PORT_CAP_SUBS_READ) != 0 { b'R' as c_char } else { b'r' as c_char } } else { b'-' as c_char } }
unsafe fn FLAG_PERM_WR(perm: c_uint) -> c_char { if (perm & SNDRV_SEQ_PORT_CAP_WRITE) != 0 { if (perm & SNDRV_SEQ_PORT_CAP_SUBS_WRITE) != 0 { b'W' as c_char } else { b'w' as c_char } } else { b'-' as c_char } }
unsafe fn FLAG_PERM_EX(perm: c_uint) -> c_char { if (perm & SNDRV_SEQ_PORT_CAP_NO_EXPORT) != 0 { b'-' as c_char } else { b'e' as c_char } }
unsafe fn FLAG_PERM_DUPLEX(perm: c_uint) -> c_char { if (perm & SNDRV_SEQ_PORT_CAP_DUPLEX) != 0 { b'X' as c_char } else { b'-' as c_char } }

unsafe fn port_direction_name(dir: c_uchar) -> *const c_char {
    match dir {
        0 => c"-".as_ptr(), 1 => c"In".as_ptr(), 2 => c"Out".as_ptr(), x if x == SNDRV_SEQ_PORT_DIR_BIDIRECTION => c"In/Out".as_ptr(), _ => c"Invalid".as_ptr(),
    }
}
unsafe fn midi_version_string(version: c_uint) -> *const c_char {
    if version == SNDRV_SEQ_CLIENT_LEGACY_MIDI { c"Legacy".as_ptr() }
    else if version == SNDRV_SEQ_CLIENT_UMP_MIDI_1_0 { c"UMP MIDI1".as_ptr() }
    else if version == SNDRV_SEQ_CLIENT_UMP_MIDI_2_0 { c"UMP MIDI2".as_ptr() }
    else { c"Unknown".as_ptr() }
}

/* CONFIG_SND_PROC_FS dump helpers are translated in outline; list traversal depends on container_of layout supplied by seq port definitions. */
#[no_mangle]
pub unsafe extern "C" fn snd_seq_info_clients_read(_entry: *mut snd_info_entry, buffer: *mut snd_info_buffer) {
    snd_iprintf(buffer, c"Client info\n".as_ptr());
    snd_iprintf(buffer, c"  cur  clients : %d\n".as_ptr(), client_usage.cur);
    snd_iprintf(buffer, c"  peak clients : %d\n".as_ptr(), client_usage.peak);
    snd_iprintf(buffer, c"  max  clients : %d\n".as_ptr(), SNDRV_SEQ_MAX_CLIENTS);
    snd_iprintf(buffer, c"\n".as_ptr());
    let mut c = 0;
    while c < SNDRV_SEQ_MAX_CLIENTS {
        let client = client_load_and_use_ptr(c);
        if !client.is_null() && (*client).type_ != NO_CLIENT {
            mutex_lock(&mut (*client).ioctl_mutex);
            snd_iprintf(buffer, c"Client %3d : \"%s\" [%s %s]\n".as_ptr(), c, (*client).name.as_ptr(), if (*client).type_ == USER_CLIENT { c"User".as_ptr() } else { c"Kernel".as_ptr() }, midi_version_string((*client).midi_version));
            if snd_seq_write_pool_allocated(client) != 0 { snd_iprintf(buffer, c"  Output pool :\n".as_ptr()); snd_seq_info_pool(buffer, (*client).pool, c"    ".as_ptr()); }
            if (*client).type_ == USER_CLIENT && !(*client).data.user.fifo.is_null() && !(*(*client).data.user.fifo).pool.is_null() { snd_iprintf(buffer, c"  Input pool :\n".as_ptr()); snd_seq_info_pool(buffer, (*(*client).data.user.fifo).pool, c"    ".as_ptr()); }
            mutex_unlock(&mut (*client).ioctl_mutex);
            snd_seq_client_unref(client);
        }
        c += 1;
    }
}

static mut snd_seq_f_ops: file_operations = file_operations {
    owner: ptr::null_mut(),
    read: Some(snd_seq_read),
    write: Some(snd_seq_write),
    open: Some(snd_seq_open),
    release: Some(snd_seq_release),
    poll: Some(snd_seq_poll),
    unlocked_ioctl: Some(snd_seq_ioctl),
    compat_ioctl: None, /* CONFIG_COMPAT: snd_seq_ioctl_compat from seq_compat.c */
};

#[no_mangle]
pub unsafe extern "C" fn snd_sequencer_device_init() -> c_int {
    let mut err = snd_device_alloc(&mut seq_dev, ptr::null_mut());
    if err < 0 { return err; }
    dev_set_name(seq_dev, c"seq".as_ptr());
    snd_seq_f_ops.owner = &mut THIS_MODULE;
    mutex_lock(&mut register_mutex);
    err = snd_register_device(SNDRV_DEVICE_TYPE_SEQUENCER, ptr::null_mut(), 0, &snd_seq_f_ops, ptr::null_mut(), seq_dev);
    mutex_unlock(&mut register_mutex);
    if err < 0 { put_device(seq_dev); return err; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_sequencer_device_done() {
    snd_unregister_device(seq_dev);
    put_device(seq_dev);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
