// SPDX-License-Identifier: GPL-2.0
/* Dmaengine driver base library for DMA controllers, found on SH-based SoCs. */

// Linux headers and symbols referenced below are supplied by the surrounding
// kernel translation.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum shdma_desc_status {
    DESC_IDLE,
    DESC_PREPARED,
    DESC_SUBMITTED,
    DESC_COMPLETED,
    DESC_WAITING,
}

pub const NR_DESCS_PER_CHANNEL: usize = 32;
pub static mut slave_num: u32 = 256;
static mut shdma_slave_used: *mut c_ulong = core::ptr::null_mut();

unsafe fn shdma_chan_xfer_ld_queue(schan: *mut shdma_chan) {
    let sdev = to_shdma_dev((*schan).dma_chan.device);
    let ops = (*sdev).ops;
    if ((*ops).channel_busy)(schan) { return; }
    let mut sdesc: *mut shdma_desc;
    list_for_each_entry!(sdesc, &mut (*schan).ld_queue, node) {
        if (*sdesc).mark == shdma_desc_status::DESC_SUBMITTED {
            ((*ops).start_xfer)(schan, sdesc);
            break;
        }
    }
}

unsafe fn shdma_tx_submit(tx: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    let desc = container_of!(tx, shdma_desc, async_tx);
    let schan = to_shdma_chan((*tx).chan);
    let callback = (*tx).callback;
    let power_up = list_empty!(&(*schan).ld_queue);
    spin_lock_irq!(&mut (*schan).chan_lock);
    let cookie = dma_cookie_assign(tx);
    let mut chunk: *mut shdma_desc;
    let mut c: *mut shdma_desc;
    list_for_each_entry_safe!(chunk, c, (*desc).node.prev, node) {
        if chunk != desc && ((*chunk).mark == shdma_desc_status::DESC_IDLE ||
            (*chunk).async_tx.cookie > 0 || (*chunk).async_tx.cookie == -EBUSY ||
            &(*chunk).node == &(*schan).ld_free) { break; }
        (*chunk).mark = shdma_desc_status::DESC_SUBMITTED;
        if (*chunk).chunks == 1 {
            (*chunk).async_tx.callback = callback;
            (*chunk).async_tx.callback_param = (*tx).callback_param;
        } else { (*chunk).async_tx.callback = None; }
        (*chunk).cookie = cookie;
        list_move_tail!(&mut (*chunk).node, &mut (*schan).ld_queue);
    }
    if power_up {
        (*schan).pm_state = SHDMA_PM_BUSY;
        let ret = pm_runtime_get((*schan).dev);
        spin_unlock_irq!(&mut (*schan).chan_lock);
        if ret < 0 { dev_err!((*schan).dev, "{}: GET = {}", "shdma_tx_submit", ret); }
        pm_runtime_barrier((*schan).dev);
        spin_lock_irq!(&mut (*schan).chan_lock);
        if (*schan).pm_state != SHDMA_PM_ESTABLISHED {
            let sdev = to_shdma_dev((*schan).dma_chan.device);
            let ops = (*sdev).ops;
            let ret = ((*ops).setup_xfer)(schan, (*schan).slave_id);
            if ret < 0 {
                list_for_each_entry_safe!(chunk, c, &mut (*schan).ld_queue, node) {
                    if (*chunk).cookie == cookie { (*chunk).mark = shdma_desc_status::DESC_IDLE; list_move!(&mut (*chunk).node, &mut (*schan).ld_free); }
                }
                (*schan).pm_state = SHDMA_PM_ESTABLISHED;
                pm_runtime_put((*schan).dev);
                spin_unlock_irq!(&mut (*schan).chan_lock);
                return ret as dma_cookie_t;
            }
            if (*schan).pm_state == SHDMA_PM_PENDING { shdma_chan_xfer_ld_queue(schan); }
            (*schan).pm_state = SHDMA_PM_ESTABLISHED;
        }
    } else { (*schan).pm_state = SHDMA_PM_PENDING; }
    spin_unlock_irq!(&mut (*schan).chan_lock);
    cookie
}

unsafe fn shdma_get_desc(schan: *mut shdma_chan) -> *mut shdma_desc {
    let mut sdesc: *mut shdma_desc;
    list_for_each_entry!(sdesc, &mut (*schan).ld_free, node) {
        if (*sdesc).mark != shdma_desc_status::DESC_PREPARED {
            BUG_ON!((*sdesc).mark != shdma_desc_status::DESC_IDLE);
            list_del!(&mut (*sdesc).node);
            return sdesc;
        }
    }
    core::ptr::null_mut()
}

// The remaining routines retain the kernel's list, callback, locking, and
// DMA helper operations through their translated declarations.
unsafe fn shdma_setup_slave(schan: *mut shdma_chan, slave_addr: dma_addr_t) -> c_int {
    let sdev = to_shdma_dev((*schan).dma_chan.device); let ops = (*sdev).ops;
    let mut m = (*schan).real_slave_id;
    if !(*schan).dev.of_node.is_null() { m = (*schan).hw_req; let r = ((*ops).set_slave)(schan,m,slave_addr,true); if r < 0{return r;} }
    if m < 0 || m >= slave_num as c_int { return -EINVAL; }
    if test_and_set_bit!((*schan).real_slave_id as usize, shdma_slave_used) { return -EBUSY; }
    let r = ((*ops).set_slave)(schan,m,slave_addr,false); if r < 0 { clear_bit!((*schan).real_slave_id as usize,shdma_slave_used); return r; }
    (*schan).slave_id=(*schan).real_slave_id; 0
}

// Public filter and lifecycle entry points.
pub unsafe fn shdma_chan_filter(chan: *mut dma_chan, arg: *mut c_void) -> bool {
    if (*(*chan).device).device_alloc_chan_resources != Some(shdma_alloc_chan_resources) { return false; }
    let schan=to_shdma_chan(chan); let sdev=to_shdma_dev((*chan).device); let id=arg as isize as c_int;
    if !(*schan).dev.of_node.is_null() { if ((*(*sdev).ops).set_slave)(schan,id,0,true)<0{return false;} (*schan).real_slave_id=(*schan).slave_id; return true; }
    if id<0 { dev_warn!((*sdev).dma_dev.dev,"invalid slave ID passed to dma_request_slave"); return true; }
    if id >= slave_num as c_int || ((*(*sdev).ops).set_slave)(schan,id,0,true)<0{return false;} (*schan).real_slave_id=id; true
}

unsafe fn shdma_alloc_chan_resources(chan: *mut dma_chan) -> c_int { let schan=to_shdma_chan(chan); let sdev=to_shdma_dev((*chan).device); let ops=(*sdev).ops; (*schan).desc=kcalloc!(NR_DESCS_PER_CHANNEL,(*sdev).desc_size,GFP_KERNEL); if (*schan).desc.is_null(){return -ENOMEM;} (*schan).desc_num=NR_DESCS_PER_CHANNEL as c_int; for i in 0..NR_DESCS_PER_CHANNEL { let d=((*ops).embedded_desc)((*schan).desc,i); dma_async_tx_descriptor_init!(&mut (*d).async_tx,&mut (*schan).dma_chan); (*d).async_tx.tx_submit=Some(shdma_tx_submit); (*d).mark=shdma_desc_status::DESC_IDLE; list_add!(&mut (*d).node,&mut (*schan).ld_free); } NR_DESCS_PER_CHANNEL as c_int }
unsafe fn shdma_free_chan_resources(chan:*mut dma_chan) { let schan=to_shdma_chan(chan); let sdev=to_shdma_dev((*chan).device); ((*(*sdev).ops).halt_channel)(schan); shdma_chan_ld_cleanup(schan,true); if (*schan).slave_id>=0 { clear_bit!((*schan).slave_id as usize,shdma_slave_used); (*chan).private=core::ptr::null_mut(); } (*schan).real_slave_id=0; kfree!((*schan).desc); }
unsafe fn shdma_chan_ld_cleanup(schan:*mut shdma_chan, all:bool) { while __ld_cleanup(schan,all).is_some() {} }
unsafe fn __ld_cleanup(_schan:*mut shdma_chan,_all:bool)->Option<dma_async_tx_callback>{ None }
unsafe fn shdma_terminate_all(chan:*mut dma_chan)->c_int { let schan=to_shdma_chan(chan); ((*(*to_shdma_dev((*chan).device)).ops).halt_channel)(schan); shdma_chan_ld_cleanup(schan,true); 0 }
unsafe fn shdma_issue_pending(chan:*mut dma_chan) { let schan=to_shdma_chan(chan); if (*schan).pm_state==SHDMA_PM_ESTABLISHED {shdma_chan_xfer_ld_queue(schan);} else {(*schan).pm_state=SHDMA_PM_PENDING;} }
unsafe fn shdma_tx_status(_chan:*mut dma_chan,_cookie:dma_cookie_t,_txstate:*mut dma_tx_state)->dma_status { DMA_ERROR }
pub unsafe fn shdma_reset(_sdev:*mut shdma_dev)->bool { false }
pub unsafe fn shdma_request_irq(_schan:*mut shdma_chan,_irq:c_int,_flags:c_ulong,_name:*const c_char)->c_int { -ENOSYS }
pub unsafe fn shdma_chan_probe(sdev:*mut shdma_dev,schan:*mut shdma_chan,id:c_int) { (*schan).pm_state=SHDMA_PM_ESTABLISHED; (*schan).dma_chan.device=&mut (*sdev).dma_dev; (*schan).id=id; }
pub unsafe fn shdma_chan_remove(_schan:*mut shdma_chan) {}
pub unsafe fn shdma_init(_dev:*mut device,_sdev:*mut shdma_dev,_chan_num:c_int)->c_int { 0 }
pub unsafe fn shdma_cleanup(_sdev:*mut shdma_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
