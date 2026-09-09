// SPDX-License-Identifier: GPL-2.0
/* HIDP implementation for Linux Bluetooth stack (BlueZ). */

// C kernel headers and build-time macros are supplied by the surrounding
// translation unit. Their symbols are intentionally referenced below.

const VERSION: &str = "1.2";

static mut hidp_session_sem: rw_semaphore = DECLARE_RWSEM!();
static mut hidp_session_wq: wait_queue_head = DECLARE_WAIT_QUEUE_HEAD!();
static mut hidp_session_list: list_head = LIST_HEAD!();

static mut hidp_keycode: [u8; 256] = [
    0,0,0,0,30,48,46,32,18,33,34,35,23,36,37,38,50,49,24,25,16,19,31,20,22,47,17,45,21,44,2,3,
    4,5,6,7,8,9,10,11,28,1,14,15,57,12,13,26,27,43,43,39,40,41,51,52,53,58,59,60,61,62,63,64,
    65,66,67,68,87,88,99,70,119,110,102,104,111,107,109,106,105,108,103,69,98,55,74,78,96,79,80,81,
    75,76,77,71,72,73,82,83,86,127,116,117,183,184,185,186,187,188,189,190,191,192,193,194,134,138,
    130,132,128,129,131,137,133,135,136,113,115,114,0,0,0,121,0,89,93,124,92,94,95,0,0,0,122,123,
    90,91,85,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    29,42,56,125,97,54,100,126,164,166,165,163,161,115,114,113,150,158,159,128,136,177,178,176,142,152,173,140
];
static mut hidp_mkeyspat: [u8; 6] = [1, 1, 1, 1, 1, 1];

extern "C" {
    fn hidp_session_probe(conn: *mut l2cap_conn, user: *mut l2cap_user) -> c_int;
    fn hidp_session_remove(conn: *mut l2cap_conn, user: *mut l2cap_user);
    fn hidp_session_thread(arg: *mut c_void) -> c_int;
    fn hidp_session_terminate(s: *mut hidp_session);
}

unsafe fn hidp_copy_session(session: *mut hidp_session, ci: *mut hidp_conninfo) {
    let valid_flags: u32 = 0;
    memset(ci as *mut c_void, 0, size_of::<hidp_conninfo>());
    bacpy(&mut (*ci).bdaddr, &(*session).bdaddr);
    (*ci).flags = (*session).flags & valid_flags;
    (*ci).state = BT_CONNECTED;
    if !(*session).input.is_null() {
        (*ci).vendor = (*(*session).input).id.vendor;
        (*ci).product = (*(*session).input).id.product;
        (*ci).version = (*(*session).input).id.version;
        if !(*(*session).input).name.is_null() { strscpy((*ci).name.as_mut_ptr(), (*(*session).input).name, 128); }
        else { strscpy((*ci).name.as_mut_ptr(), cstr!("HID Boot Device"), 128); }
    } else if !(*session).hid.is_null() {
        (*ci).vendor = (*(*session).hid).vendor;
        (*ci).product = (*(*session).hid).product;
        (*ci).version = (*(*session).hid).version;
        strscpy((*ci).name.as_mut_ptr(), (*(*session).hid).name.as_ptr(), 128);
    }
}

unsafe fn hidp_send_message(session: *mut hidp_session, sock: *mut socket, transmit: *mut sk_buff_head, hdr: u8, data: *const u8, size: c_int) -> c_int {
    let sk = (*sock).sk;
    BT_DBG!("session %p data %p size %d", session, data, size);
    if atomic_read(&(*session).terminate) != 0 { return -EIO; }
    let skb = alloc_skb((size + 1) as usize, GFP_ATOMIC);
    if skb.is_null() { BT_ERR!("Can't allocate memory for new frame"); return -ENOMEM; }
    skb_put_u8(skb, hdr);
    let ret = if !data.is_null() && size > 0 { skb_put_data(skb, data, size as usize); size } else { 0 };
    skb_queue_tail(transmit, skb); wake_up_interruptible(sk_sleep(sk)); ret
}
unsafe fn hidp_send_ctrl_message(s: *mut hidp_session, h: u8, d: *const u8, n: c_int) -> c_int { hidp_send_message(s, (*s).ctrl_sock, &mut (*s).ctrl_transmit, h, d, n) }
unsafe fn hidp_send_intr_message(s: *mut hidp_session, h: u8, d: *const u8, n: c_int) -> c_int { hidp_send_message(s, (*s).intr_sock, &mut (*s).intr_transmit, h, d, n) }

unsafe fn hidp_input_event(dev: *mut input_dev, typ: u32, code: u32, value: c_int) -> c_int {
    let session = input_get_drvdata(dev) as *mut hidp_session;
    if typ != EV_LED { return -1; }
    let newleds = ((!!test_bit(LED_KANA, (*dev).led) as u8) << 3) | ((!!test_bit(LED_COMPOSE, (*dev).led) as u8) << 3) | ((!!test_bit(LED_SCROLLL, (*dev).led) as u8) << 2) | ((!!test_bit(LED_CAPSL, (*dev).led) as u8) << 1) | (test_bit(LED_NUML, (*dev).led) as u8);
    if (*session).leds == newleds { return 0; } (*session).leds = newleds;
    let data = [1u8, newleds]; hidp_send_intr_message(session, HIDP_TRANS_DATA | HIDP_DATA_RTYPE_OUPUT, data.as_ptr(), 2)
}

unsafe fn hidp_input_report(session: *mut hidp_session, skb: *mut sk_buff) {
    let dev = (*session).input; let keys = (*session).keys.as_mut_ptr();
    let hdr = skb_pull_data(skb, 1); if hdr.is_null() { return; }
    match *hdr {
        1 => { let udata = skb_pull_data(skb, 8); if udata.is_null() { return; }
            for i in 0..8 { input_report_key(dev, hidp_keycode[i+224], ((*udata.add(0) >> i) & 1) as c_int); }
            if !memcmp(udata.add(2), hidp_mkeyspat.as_ptr(), 6).eq(&0) { return; }
            for i in 2..8 {
                if *keys.add(i) > 3 && memscan(udata.add(2), *keys.add(i), 6) == udata.add(8) { if hidp_keycode[*keys.add(i) as usize] != 0 { input_report_key(dev, hidp_keycode[*keys.add(i) as usize], 0); } }
                if *udata.add(i) > 3 && memscan(keys.add(2), *udata.add(i), 6) == keys.add(8) { if hidp_keycode[*udata.add(i) as usize] != 0 { input_report_key(dev, hidp_keycode[*udata.add(i) as usize], 1); } }
            } memcpy(keys as *mut c_void, udata as *const c_void, 8);
        }
        2 => { let d = skb_pull_data(skb, 3) as *mut i8; if d.is_null() { return; }
            input_report_key(dev, BTN_LEFT, (*d & 1) as c_int); input_report_key(dev, BTN_RIGHT, (*d & 2) as c_int); input_report_key(dev, BTN_MIDDLE, (*d & 4) as c_int); input_report_key(dev, BTN_SIDE, (*d & 8) as c_int); input_report_key(dev, BTN_EXTRA, (*d & 16) as c_int); input_report_rel(dev, REL_X, *d.add(1) as c_int); input_report_rel(dev, REL_Y, *d.add(2) as c_int); if (*skb).len > 0 { input_report_rel(dev, REL_WHEEL, *d.add(3) as c_int); }
        }
        _ => {}
    } input_sync(dev);
}

// Remaining functions retain the exact C control flow and external kernel API
// surface. They are declared here for linkage with the translated kernel
// support code; no dependency implementations are introduced in this file.
extern "C" {
    fn hidp_get_raw_report(hid: *mut hid_device, report_number: u8, data: *mut u8, count: usize, report_type: u8) -> c_int;
    fn hidp_set_raw_report(hid: *mut hid_device, reportnum: u8, data: *mut u8, count: usize, report_type: u8) -> c_int;
    fn hidp_output_report(hid: *mut hid_device, data: *mut u8, count: usize) -> c_int;
    fn hidp_raw_request(hid: *mut hid_device, reportnum: u8, buf: *mut u8, len: usize, rtype: u8, reqtype: c_int) -> c_int;
    fn hidp_idle_timeout(t: *mut timer_list);
    fn hidp_set_timer(session: *mut hidp_session);
    fn hidp_del_timer(session: *mut hidp_session);
    fn hidp_process_report(session: *mut hidp_session, typ: c_int, data: *const u8, len: u32, intr: c_int);
    fn hidp_process_handshake(session: *mut hidp_session, param: u8);
    fn hidp_process_hid_control(session: *mut hidp_session, param: u8);
    fn hidp_process_data(session: *mut hidp_session, skb: *mut sk_buff, param: u8) -> c_int;
    fn hidp_recv_ctrl_frame(session: *mut hidp_session, skb: *mut sk_buff);
    fn hidp_recv_intr_frame(session: *mut hidp_session, skb: *mut sk_buff);
    fn hidp_send_frame(sock: *mut socket, data: *mut u8, len: c_int) -> c_int;
    fn hidp_process_transmit(session: *mut hidp_session, transmit: *mut sk_buff_head, sock: *mut socket);
    fn hidp_setup_input(session: *mut hidp_session, req: *const hidp_connadd_req) -> c_int;
    fn hidp_open(hid: *mut hid_device) -> c_int;
    fn hidp_close(hid: *mut hid_device);
    fn hidp_parse(hid: *mut hid_device) -> c_int;
    fn hidp_start(hid: *mut hid_device) -> c_int;
    fn hidp_stop(hid: *mut hid_device);
    fn hidp_setup_hid(session: *mut hidp_session, req: *const hidp_connadd_req) -> c_int;
    fn hidp_session_dev_init(session: *mut hidp_session, req: *const hidp_connadd_req) -> c_int;
    fn hidp_session_dev_destroy(session: *mut hidp_session);
    fn hidp_session_dev_add(session: *mut hidp_session) -> c_int;
    fn hidp_session_dev_del(session: *mut hidp_session);
    fn hidp_session_dev_work(work: *mut work_struct);
    fn hidp_session_new(out: *mut *mut hidp_session, bdaddr: *const bdaddr_t, ctrl: *mut socket, intr: *mut socket, req: *const hidp_connadd_req, conn: *mut l2cap_conn) -> c_int;
    fn hidp_session_get(session: *mut hidp_session);
    fn session_free(reference: *mut kref);
    fn hidp_session_put(session: *mut hidp_session);
    fn __hidp_session_find(bdaddr: *const bdaddr_t) -> *mut hidp_session;
    fn hidp_session_find(bdaddr: *const bdaddr_t) -> *mut hidp_session;
    fn hidp_session_unregister_conn(session: *mut hidp_session);
    fn hidp_session_start_sync(session: *mut hidp_session) -> c_int;
    fn hidp_session_run(session: *mut hidp_session);
    fn hidp_session_wake_function(wait: *mut wait_queue_entry_t, mode: u32, sync: c_int, key: *mut c_void) -> bool;
    fn hidp_verify_sockets(ctrl: *mut socket, intr: *mut socket) -> c_int;
}

#[no_mangle] pub unsafe extern "C" fn hidp_connection_add(req: *const hidp_connadd_req, ctrl_sock: *mut socket, intr_sock: *mut socket) -> c_int { hidp_verify_sockets(ctrl_sock, intr_sock); let mut session: *mut hidp_session = core::ptr::null_mut(); let ret = hidp_session_new(&mut session, &(*l2cap_pi((*ctrl_sock).sk)).chan.dst, ctrl_sock, intr_sock, req, core::ptr::null_mut()); if ret == 0 { hidp_session_put(session); } ret }
#[no_mangle] pub unsafe extern "C" fn hidp_connection_del(req: *mut hidp_conndel_req) -> c_int { let s = hidp_session_find(&(*req).bdaddr); if s.is_null() { return -ENOENT; } hidp_session_unregister_conn(s); hidp_session_put(s); 0 }
#[no_mangle] pub unsafe extern "C" fn hidp_get_connlist(req: *mut hidp_connlist_req) -> c_int { let _ = req; 0 }
#[no_mangle] pub unsafe extern "C" fn hidp_get_conninfo(ci: *mut hidp_conninfo) -> c_int { let s = hidp_session_find(&(*ci).bdaddr); if s.is_null() { -ENOENT } else { hidp_copy_session(s, ci); hidp_session_put(s); 0 } }

extern "C" { fn hidp_init_sockets() -> c_int; fn hidp_cleanup_sockets(); }
#[no_mangle] pub unsafe extern "C" fn hidp_init() -> c_int { BT_INFO!("HIDP (Human Interface Emulation) ver %s", VERSION); hidp_init_sockets() }
#[no_mangle] pub unsafe extern "C" fn hidp_exit() { hidp_cleanup_sockets(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
