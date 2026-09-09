// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2019,2023 NXP
 *
 * Implementation of the SCU IRQ functions using MU.
 *
 */

// Dependencies supplied by the surrounding kernel/Rust environment:
// dt-bindings/firmware/imx/rsrc.h, linux/firmware/imx/{ipc,sci}.h,
// linux/{kobject,mailbox_client,of,suspend,sysfs}.h

const IMX_SC_IRQ_FUNC_ENABLE: u32 = 1;
const IMX_SC_IRQ_FUNC_STATUS: u32 = 2;
const IMX_SC_IRQ_NUM_GROUP: usize = 9;

type U8 = u8;
type U16 = u16;
type U32 = u32;

#[repr(C)]
pub struct imx_sc_rpc_msg {
    pub ver: U8,
    pub svc: U8,
    pub func: U8,
    pub size: U8,
}

#[repr(C, packed)]
pub struct imx_sc_msg_irq_get_status {
    pub hdr: imx_sc_rpc_msg,
    pub data: imx_sc_msg_irq_get_status_data,
}

#[repr(C)]
pub union imx_sc_msg_irq_get_status_data {
    pub req: imx_sc_msg_irq_get_status_req,
    pub resp: imx_sc_msg_irq_get_status_resp,
}

#[repr(C, packed)]
pub struct imx_sc_msg_irq_get_status_req {
    pub resource: U16,
    pub group: U8,
    pub reserved: U8,
}

#[repr(C)]
pub struct imx_sc_msg_irq_get_status_resp {
    pub status: U32,
}

#[repr(C, packed)]
pub struct imx_sc_msg_irq_enable {
    pub hdr: imx_sc_rpc_msg,
    pub mask: U32,
    pub resource: U16,
    pub group: U8,
    pub enable: U8,
}

#[repr(C)]
pub struct scu_wakeup {
    pub mask: U32,
    pub wakeup_src: U32,
    pub valid: bool,
}

#[repr(C)]
pub struct kobject;
#[repr(C)]
pub struct kobj_attribute;
#[repr(C)]
pub struct notifier_block;
#[repr(C)]
pub struct work_struct;
#[repr(C)]
pub struct mbox_client;
#[repr(C)]
pub struct mbox_chan;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct imx_sc_ipc;
#[repr(C)]
pub struct of_phandle_args;

static mut mu_resource_id: U32 = 0;
static mut wakeup_obj: *mut kobject = core::ptr::null_mut();
static mut scu_irq_wakeup: [scu_wakeup; IMX_SC_IRQ_NUM_GROUP] = [
    scu_wakeup { mask: 0, wakeup_src: 0, valid: false }; IMX_SC_IRQ_NUM_GROUP
];
static mut imx_sc_irq_ipc_handle: *mut imx_sc_ipc = core::ptr::null_mut();
static mut imx_sc_irq_work: work_struct = work_struct {};
static mut imx_scu_irq_notifier_chain: () = ();

extern "C" {
    fn blocking_notifier_chain_register(chain: *mut (), nb: *mut notifier_block) -> i32;
    fn blocking_notifier_chain_unregister(chain: *mut (), nb: *mut notifier_block) -> i32;
    fn blocking_notifier_call_chain(chain: *mut (), status: usize, data: *mut core::ffi::c_void) -> i32;
    fn imx_scu_irq_get_status(group: U8, irq_status: *mut U32) -> i32;
    fn imx_scu_call_rpc(handle: *mut imx_sc_ipc, msg: *mut core::ffi::c_void, wait: bool) -> i32;
    fn schedule_work(work: *mut work_struct);
    fn pm_system_wakeup();
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn sprintf(buf: *mut core::ffi::c_char, fmt: *const core::ffi::c_char, ... ) -> i32;
    fn strlen(buf: *const core::ffi::c_char) -> usize;
    fn imx_scu_get_handle(handle: *mut *mut imx_sc_ipc) -> i32;
    fn of_parse_phandle_with_args(node: *mut core::ffi::c_void, name: *const core::ffi::c_char, cells: *const core::ffi::c_char, index: i32, spec: *mut of_phandle_args) -> i32;
    fn of_alias_get_id(node: *mut core::ffi::c_void, stem: *const core::ffi::c_char) -> i32;
    fn of_node_put(node: *mut core::ffi::c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_kfree(dev: *mut device, ptr: *mut core::ffi::c_void);
    fn mbox_request_channel_byname(cl: *mut mbox_client, name: *const core::ffi::c_char) -> *mut mbox_chan;
    fn mbox_free_channel(ch: *mut mbox_chan);
    fn kobject_create_and_add(name: *const core::ffi::c_char, parent: *mut kobject) -> *mut kobject;
    fn sysfs_create_file(kobj: *mut kobject, attr: *mut core::ffi::c_void) -> i32;
    fn kobject_put(kobj: *mut kobject);
}

pub unsafe fn imx_scu_irq_register_notifier(nb: *mut notifier_block) -> i32 {
    blocking_notifier_chain_register(&mut imx_scu_irq_notifier_chain, nb)
}

pub unsafe fn imx_scu_irq_unregister_notifier(nb: *mut notifier_block) -> i32 {
    blocking_notifier_chain_unregister(&mut imx_scu_irq_notifier_chain, nb)
}

unsafe fn imx_scu_irq_notifier_call_chain(status: usize, group: *mut U8) -> i32 {
    blocking_notifier_call_chain(&mut imx_scu_irq_notifier_chain, status, group.cast())
}

unsafe fn imx_scu_irq_work_handler(_work: *mut work_struct) {
    let mut irq_status: U32 = 0;
    for i in 0..IMX_SC_IRQ_NUM_GROUP {
        if scu_irq_wakeup[i].mask != 0 {
            scu_irq_wakeup[i].valid = false;
            scu_irq_wakeup[i].wakeup_src = 0;
        }
        let ret = imx_scu_irq_get_status(i as U8, &mut irq_status);
        if ret != 0 { return; }
        if irq_status == 0 { continue; }
        if scu_irq_wakeup[i].mask & irq_status != 0 {
            scu_irq_wakeup[i].valid = true;
            scu_irq_wakeup[i].wakeup_src = irq_status & scu_irq_wakeup[i].mask;
        } else {
            scu_irq_wakeup[i].wakeup_src = irq_status;
        }
        pm_system_wakeup();
        imx_scu_irq_notifier_call_chain(irq_status as usize, &mut (i as U8));
    }
}

pub unsafe fn imx_scu_irq_get_status(group: U8, irq_status: *mut U32) -> i32 {
    let mut msg: imx_sc_msg_irq_get_status = core::mem::zeroed();
    msg.hdr.ver = 1;
    msg.hdr.svc = 5;
    msg.hdr.func = IMX_SC_IRQ_FUNC_STATUS as U8;
    msg.hdr.size = 2;
    msg.data.req.resource = mu_resource_id;
    msg.data.req.group = group;
    let ret = imx_scu_call_rpc(imx_sc_irq_ipc_handle, (&mut msg).cast(), true);
    if ret != 0 { return ret; }
    if !irq_status.is_null() { *irq_status = msg.data.resp.status; }
    0
}

pub unsafe fn imx_scu_irq_group_enable(group: U8, mask: U32, enable: U8) -> i32 {
    if imx_sc_irq_ipc_handle.is_null() { return -517; }
    let mut msg: imx_sc_msg_irq_enable = core::mem::zeroed();
    msg.hdr.ver = 1;
    msg.hdr.svc = 5;
    msg.hdr.func = IMX_SC_IRQ_FUNC_ENABLE as U8;
    msg.hdr.size = 3;
    msg.resource = mu_resource_id;
    msg.group = group;
    msg.mask = mask;
    msg.enable = enable;
    let ret = imx_scu_call_rpc(imx_sc_irq_ipc_handle, (&mut msg).cast(), true);
    if enable != 0 { scu_irq_wakeup[group as usize].mask |= mask; }
    else { scu_irq_wakeup[group as usize].mask &= !mask; }
    ret
}

unsafe fn imx_scu_irq_callback(_c: *mut mbox_client, _msg: *mut core::ffi::c_void) {
    schedule_work(&mut imx_sc_irq_work);
}

unsafe fn wakeup_source_show(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut core::ffi::c_char) -> isize {
    for i in 0..IMX_SC_IRQ_NUM_GROUP {
        if scu_irq_wakeup[i].wakeup_src == 0 { continue; }
        if scu_irq_wakeup[i].valid {
            sprintf(buf, b"Wakeup source group = %d, irq = 0x%x\0".as_ptr().cast(), i as i32, scu_irq_wakeup[i].wakeup_src);
        } else {
            sprintf(buf, b"Spurious SCU wakeup, group = %d, irq = 0x%x\0".as_ptr().cast(), i as i32, scu_irq_wakeup[i].wakeup_src);
        }
    }
    strlen(buf) as isize
}

pub unsafe fn imx_scu_enable_general_irq_channel(dev: *mut device) -> i32 {
    let mut spec: of_phandle_args = core::mem::zeroed();
    let mut cl: *mut mbox_client;
    let mut ch: *mut mbox_chan;
    let mut ret = 0;
    let mut i = 0;

    if of_parse_phandle_with_args(core::ptr::null_mut(), b"mboxes\0".as_ptr().cast(), b"#mbox-cells\0".as_ptr().cast(), 0, &mut spec) == 0 {
        i = of_alias_get_id(core::ptr::null_mut(), b"mu\0".as_ptr().cast());
        of_node_put(core::ptr::null_mut());
    }
    // use mu1 as general mu irq channel if failed
    if i < 0 { i = 1; }
    mu_resource_id = 0x800 + i as U32;
    ret = imx_scu_get_handle(&mut imx_sc_irq_ipc_handle);
    if ret != 0 { return ret; }
    cl = devm_kzalloc(dev, core::mem::size_of::<mbox_client>(), 0) as *mut mbox_client;
    if cl.is_null() { return -12; }
    // cl->dev = dev; cl->rx_callback = imx_scu_irq_callback;
    // INIT_WORK(&imx_sc_irq_work, imx_scu_irq_work_handler);
    ch = mbox_request_channel_byname(cl, b"gip3\0".as_ptr().cast());
    if ch.is_null() {
        ret = -1;
        devm_kfree(dev, cl.cast());
        return ret;
    }
    // Create directory under /sysfs/firmware
    wakeup_obj = kobject_create_and_add(b"scu_wakeup_source\0".as_ptr().cast(), core::ptr::null_mut());
    if wakeup_obj.is_null() {
        ret = -12;
        mbox_free_channel(ch);
        devm_kfree(dev, cl.cast());
        return ret;
    }
    ret = sysfs_create_file(wakeup_obj, core::ptr::null_mut());
    if ret != 0 {
        kobject_put(wakeup_obj);
        mbox_free_channel(ch);
        devm_kfree(dev, cl.cast());
        return ret;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
