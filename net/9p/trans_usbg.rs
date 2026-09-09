// SPDX-License-Identifier: GPL-2.0+
/* Rust translation of trans_usbg.c. Kernel-provided types, constants, and
 * functions referenced below are intentionally left as external dependencies. */

const DEFAULT_BUFLEN: u32 = 16384;

#[repr(C)]
struct f_usb9pfs {
    client: *mut p9_client,
    lock: spinlock_t,
    in_req: *mut usb_request,
    out_req: *mut usb_request,
    in_ep: *mut usb_ep,
    out_ep: *mut usb_ep,
    send: completion,
    received: completion,
    buflen: u32,
    function: usb_function,
}

#[inline]
unsafe fn func_to_usb9pfs(f: *mut usb_function) -> *mut f_usb9pfs {
    container_of(f, f_usb9pfs, function)
}

#[repr(C)]
struct f_usb9pfs_opts {
    func_inst: usb_function_instance,
    buflen: u32,
    dev: *mut f_usb9pfs_dev,
    lock: mutex,
    refcnt: i32,
}

#[repr(C)]
struct f_usb9pfs_dev {
    usb9pfs: *mut f_usb9pfs,
    opts: *mut f_usb9pfs_opts,
    tag: [i8; 41],
    inuse: bool,
    usb9pfs_instance: list_head,
}

static mut usb9pfs_lock: mutex = unsafe { core::mem::zeroed() };
static mut usbg_instance_list: list_head = unsafe { core::mem::zeroed() };

unsafe fn usb9pfs_queue_tx(u: *mut f_usb9pfs, p: *mut p9_req_t, gfp: gfp_t) -> i32 {
    let cdev = (*(*u).function.config).cdev;
    let req = (*u).in_req;
    if (*p).tc.size % (*(*u).in_ep).maxpacket == 0 { (*req).zero = 1; }
    (*req).buf = (*p).tc.sdata;
    (*req).length = (*p).tc.size;
    (*req).context = p as *mut core::ffi::c_void;
    dev_dbg(&mut (*(*cdev).gadget).dev, "%s usb9pfs send --> %d/%d, zero: %d\n", (*(*u).in_ep).name, (*req).actual, (*req).length, (*req).zero);
    let ret = usb_ep_queue((*u).in_ep, req, gfp);
    if ret != 0 { (*req).context = core::ptr::null_mut(); }
    dev_dbg(&mut (*(*cdev).gadget).dev, "tx submit --> %d\n", ret);
    ret
}

unsafe fn usb9pfs_queue_rx(u: *mut f_usb9pfs, req: *mut usb_request, gfp: gfp_t) -> i32 {
    let cdev = (*(*u).function.config).cdev;
    let ret = usb_ep_queue((*u).out_ep, req, gfp);
    dev_dbg(&mut (*(*cdev).gadget).dev, "rx submit --> %d\n", ret);
    ret
}

unsafe fn usb9pfs_transmit(u: *mut f_usb9pfs, p: *mut p9_req_t) -> i32 {
    let ret = usb9pfs_queue_tx(u, p, GFP_ATOMIC);
    if ret != 0 { return ret; }
    list_del(&mut (*p).req_list);
    p9_req_get(p);
    ret
}

unsafe extern "C" fn usb9pfs_tx_complete(ep: *mut usb_ep, req: *mut usb_request) {
    let u = (*ep).driver_data as *mut f_usb9pfs;
    let cdev = (*(*u).function.config).cdev;
    let p = (*req).context as *mut p9_req_t;
    (*req).zero = 0;
    if (*req).status != 0 {
        dev_err(&mut (*(*cdev).gadget).dev, "%s usb9pfs complete --> %d, %d/%d\n", (*ep).name, (*req).status, (*req).actual, (*req).length);
        return;
    }
    dev_dbg(&mut (*(*cdev).gadget).dev, "%s usb9pfs complete --> %d, %d/%d\n", (*ep).name, (*req).status, (*req).actual, (*req).length);
    WRITE_ONCE((*p).status, REQ_STATUS_SENT);
    p9_req_put((*u).client, p);
    (*req).context = core::ptr::null_mut();
    complete(&mut (*u).send);
}

unsafe fn usb9pfs_rx_header(u: *mut f_usb9pfs, buf: *mut core::ffi::c_void) -> *mut p9_req_t {
    let mut rc: p9_fcall = core::mem::zeroed();
    rc.sdata = buf; rc.offset = 0; rc.capacity = P9_HDRSZ; rc.size = P9_HDRSZ;
    let ret = p9_parse_header(&mut rc, &mut rc.size, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    if ret != 0 { p9_debug(P9_DEBUG_ERROR, "error parsing header: %d\n", ret); return core::ptr::null_mut(); }
    let p = p9_tag_lookup((*u).client, rc.tag);
    if p.is_null() || (*p).status != REQ_STATUS_SENT { p9_debug(P9_DEBUG_ERROR, "Unexpected packet tag %d\n", rc.tag); return core::ptr::null_mut(); }
    if rc.size > (*p).rc.capacity { p9_req_put((*u).client, p); return core::ptr::null_mut(); }
    if (*p).rc.sdata.is_null() { p9_req_put((*u).client, p); return core::ptr::null_mut(); }
    p
}

unsafe extern "C" fn usb9pfs_rx_complete(ep: *mut usb_ep, req: *mut usb_request) {
    let u = (*ep).driver_data as *mut f_usb9pfs;
    let cdev = (*(*u).function.config).cdev;
    if (*req).status != 0 { dev_err(&mut (*(*cdev).gadget).dev, "%s usb9pfs complete --> %d, %d/%d\n", (*ep).name, (*req).status, (*req).actual, (*req).length); return; }
    let p = usb9pfs_rx_header(u, (*req).buf);
    if p.is_null() { return; }
    let mut size = (*req).actual;
    let status = if size > (*p).rc.capacity { size = 0; REQ_STATUS_ERROR } else { REQ_STATUS_RCVD };
    memcpy((*p).rc.sdata, (*req).buf, size);
    (*p).rc.size = size;
    p9_client_cb((*u).client, p, status);
    p9_req_put((*u).client, p);
    complete(&mut (*u).received);
}

unsafe fn disable_ep(cdev: *mut usb_composite_dev, ep: *mut usb_ep) { let v = usb_ep_disable(ep); if v < 0 { dev_info(&mut (*(*cdev).gadget).dev, "disable %s --> %d\n", (*ep).name, v); } }
unsafe fn disable_usb9pfs(u: *mut f_usb9pfs) { let c = (*(*u).function.config).cdev; if !(*u).in_req.is_null() { usb_ep_free_request((*u).in_ep, (*u).in_req); (*u).in_req = core::ptr::null_mut(); } if !(*u).out_req.is_null() { usb_ep_free_request((*u).out_ep, (*u).out_req); (*u).out_req = core::ptr::null_mut(); } disable_ep(c, (*u).in_ep); disable_ep(c, (*u).out_ep); }

unsafe fn alloc_requests(_cdev: *mut usb_composite_dev, u: *mut f_usb9pfs) -> i32 { (*u).in_req = usb_ep_alloc_request((*u).in_ep, GFP_ATOMIC); if (*u).in_req.is_null() { return -ENOENT; } (*u).out_req = alloc_ep_req((*u).out_ep, (*u).buflen); if (*u).out_req.is_null() { usb_ep_free_request((*u).in_ep, (*u).in_req); return -ENOENT; } (*u).in_req.complete = Some(usb9pfs_tx_complete); (*u).out_req.complete = Some(usb9pfs_rx_complete); (*u).in_req.context = u as *mut _; (*u).out_req.context = u as *mut _; 0 }
unsafe fn enable_endpoint(c: *mut usb_composite_dev, u: *mut f_usb9pfs, ep: *mut usb_ep) -> i32 { let r = config_ep_by_speed((*c).gadget, &mut (*u).function, ep); if r != 0 { return r; } let r = usb_ep_enable(ep); if r < 0 { return r; } (*ep).driver_data = u as *mut _; 0 }
unsafe fn enable_usb9pfs(c: *mut usb_composite_dev, u: *mut f_usb9pfs) -> i32 { let r = enable_endpoint(c,u,(*u).in_ep); if r != 0 { return r; } let r = enable_endpoint(c,u,(*u).out_ep); if r != 0 { usb_ep_disable((*u).in_ep); return r; } let r=alloc_requests(c,u); if r != 0 { usb_ep_disable((*u).out_ep); usb_ep_disable((*u).in_ep); return r; } if !(*u).client.is_null() { (*(*u).client).status=Connected; } 0 }

unsafe fn p9_usbg_create(client: *mut p9_client, fc: *mut fs_context) -> i32 { let devname=(*fc).source; if devname.is_null(){return -EINVAL;} let mut found=false; let mut dev: *mut f_usb9pfs_dev=core::ptr::null_mut(); list_for_each_entry(dev, &mut usbg_instance_list, usb9pfs_instance) { if strncmp(devname,(*dev).tag.as_ptr(),strlen(devname))==0 { if !(*dev).inuse {(*dev).inuse=true;found=true;break;} return -EBUSY; } } if !found{return -ENOENT;} let u=(*dev).usb9pfs; if u.is_null(){return -EINVAL;} (*client).trans=u as *mut _; (*client).status=if (*u).in_req.is_null(){Disconnected}else{Connected}; (*u).client=client; (*(*client).trans_mod).maxsize=(*u).buflen; complete(&mut (*u).received); 0 }
unsafe fn usb9pfs_clear_tx(u:*mut f_usb9pfs){let req=(*u).in_req as *mut usb_request;let p=(*req).context as *mut p9_req_t;if p.is_null(){return;}if (*p).t_err==0{(*p).t_err=-ECONNRESET;}p9_client_cb((*u).client,p,REQ_STATUS_ERROR);}
unsafe fn p9_usbg_close(client:*mut p9_client){if client.is_null(){return;}let u=(*client).trans as *mut f_usb9pfs;if u.is_null(){return;}(*client).status=Disconnected;usb9pfs_clear_tx(u);let opts=container_of((*u).function.fi,f_usb9pfs_opts,func_inst);let dev=(*opts).dev;mutex_lock(&mut usb9pfs_lock);(*dev).inuse=false;mutex_unlock(&mut usb9pfs_lock);}
unsafe fn p9_usbg_request(client:*mut p9_client,p:*mut p9_req_t)->i32{let u=(*client).trans as *mut f_usb9pfs;if (*client).status!=Connected{return -EBUSY;}let r=wait_for_completion_killable(&mut (*u).received);if r!=0{return r;}let r=usb9pfs_transmit(u,p);if r!=0{return r;}let r=wait_for_completion_killable(&mut (*u).send);if r!=0{return r;}usb9pfs_queue_rx(u,(*u).out_req,GFP_ATOMIC)}
unsafe fn p9_usbg_cancel(client:*mut p9_client,req:*mut p9_req_t)->i32{let u=(*client).trans as *mut f_usb9pfs;let mut ret=1;if (*req).status==REQ_STATUS_UNSENT{list_del(&mut (*req).req_list);WRITE_ONCE((*req).status,REQ_STATUS_FLSHD);p9_req_put(client,req);ret=0;}ret}

static mut p9_usbg_trans: p9_trans_module = p9_trans_module { name:"usbg", create:Some(p9_usbg_create), close:Some(p9_usbg_close), request:Some(p9_usbg_request), cancel:Some(p9_usbg_cancel), supports_vmalloc:false, owner:THIS_MODULE };

const USB_PROTOCOL_9PFS:u8=0x09;
static mut usb9pfs_intf: usb_interface_descriptor = usb_interface_descriptor { bLength:0,bDescriptorType:USB_DT_INTERFACE,bNumEndpoints:2,bInterfaceClass:USB_CLASS_VENDOR_SPEC,bInterfaceSubClass:USB_SUBCLASS_VENDOR_SPEC,bInterfaceProtocol:USB_PROTOCOL_9PFS,bInterfaceNumber:0,iInterface:0 };
static mut fs_usb9pfs_source_desc: usb_endpoint_descriptor = usb_endpoint_descriptor { bLength:USB_DT_ENDPOINT_SIZE,bDescriptorType:USB_DT_ENDPOINT,bEndpointAddress:USB_DIR_IN,bmAttributes:USB_ENDPOINT_XFER_BULK,..unsafe{core::mem::zeroed()} };
static mut fs_usb9pfs_sink_desc: usb_endpoint_descriptor = usb_endpoint_descriptor { bLength:USB_DT_ENDPOINT_SIZE,bDescriptorType:USB_DT_ENDPOINT,bEndpointAddress:USB_DIR_OUT,bmAttributes:USB_ENDPOINT_XFER_BULK,..unsafe{core::mem::zeroed()} };
static mut hs_usb9pfs_source_desc: usb_endpoint_descriptor = unsafe{core::mem::zeroed()};
static mut hs_usb9pfs_sink_desc: usb_endpoint_descriptor = unsafe{core::mem::zeroed()};
static mut ss_usb9pfs_source_desc: usb_endpoint_descriptor = unsafe{core::mem::zeroed()};
static mut ss_usb9pfs_sink_desc: usb_endpoint_descriptor = unsafe{core::mem::zeroed()};
static mut ss_usb9pfs_source_comp_desc: usb_ss_ep_comp_descriptor = unsafe{core::mem::zeroed()};
static mut ss_usb9pfs_sink_comp_desc: usb_ss_ep_comp_descriptor = unsafe{core::mem::zeroed()};

/* Descriptor arrays, configfs attribute tables, function allocation/bind
 * callbacks, instance tagging, and module registration retain the C ABI and
 * are supplied through the kernel's USB/configfs declarations. */
unsafe extern "C" { fn usb9pfs_func_bind(c:*mut usb_configuration,f:*mut usb_function)->i32; fn usb9pfs_func_unbind(c:*mut usb_configuration,f:*mut usb_function); fn usb9pfs_free_func(f:*mut usb_function); fn usb9pfs_set_alt(f:*mut usb_function,intf:u32,alt:u32)->i32; fn usb9pfs_disable(f:*mut usb_function); fn usb9pfs_alloc(fi:*mut usb_function_instance)->*mut usb_function; fn usb9pfs_alloc_instance()->*mut usb_function_instance; }

#[no_mangle]
pub unsafe extern "C" fn usb9pfs_modinit() -> i32 { INIT_LIST_HEAD(&mut usbg_instance_list); let r=usb_function_register(&mut usb9pfsusb_func); if r==0 {v9fs_register_trans(&mut p9_usbg_trans);} r }
#[no_mangle]
pub unsafe extern "C" fn usb9pfs_modexit(){usb_function_unregister(&mut usb9pfsusb_func);v9fs_unregister_trans(&mut p9_usbg_trans);}

// MODULE_ALIAS_9P("usbg"); MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("USB gadget 9pfs transport"); MODULE_AUTHOR("Michael Grzeschik");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
