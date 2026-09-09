// SPDX-License-Identifier: GPL-2.0-only
/*
 * EDAC PCI component
 *
 * Author: Dave Jiang <djiang@mvista.com>
 *
 * 2007 (c) MontaVista Software, Inc.
 */

// Declarations supplied by the Linux EDAC/kernel environment are intentionally
// left external; the original includes are not executable Rust.

static mut edac_pci_ctls_mutex: Mutex = Mutex::new();
static mut edac_pci_list: ListHead = ListHead::new();
static mut pci_indexes: Atomic = Atomic::new(0);

pub unsafe fn edac_pci_alloc_ctl_info(
    sz_pvt: u32,
    edac_pci_name: *const c_char,
) -> *mut edac_pci_ctl_info {
    edac_dbg(1, "\n");

    let pci = kzalloc_obj::<edac_pci_ctl_info>();
    if pci.is_null() {
        return core::ptr::null_mut();
    }

    if sz_pvt != 0 {
        (*pci).pvt_info = kzalloc(sz_pvt as usize, GFP_KERNEL);
        if (*pci).pvt_info.is_null() {
            kfree(pci as *mut c_void);
            return core::ptr::null_mut();
        }
    }

    (*pci).op_state = OP_ALLOC;
    snprintf((*pci).name.as_mut_ptr(), strlen(edac_pci_name) + 1, "%s", edac_pci_name);
    pci
}

pub unsafe fn edac_pci_free_ctl_info(pci: *mut edac_pci_ctl_info) {
    edac_dbg(1, "\n");
    edac_pci_remove_sysfs(pci);
}

unsafe fn find_edac_pci_by_dev(dev: *mut device) -> *mut edac_pci_ctl_info {
    let mut item: *mut list_head;
    edac_dbg(1, "\n");
    list_for_each(item, &mut edac_pci_list) {
        let pci = list_entry!(item, edac_pci_ctl_info, link);
        if (*pci).dev == dev { return pci; }
    }
    core::ptr::null_mut()
}

unsafe fn add_edac_pci_to_global_list(pci: *mut edac_pci_ctl_info) -> c_int {
    let mut item: *mut list_head;
    let mut insert_before = &mut edac_pci_list as *mut list_head;
    let mut rover = find_edac_pci_by_dev((*pci).dev);
    if !rover.is_null() { goto_fail0(rover); return 1; }

    list_for_each(item, &mut edac_pci_list) {
        rover = list_entry!(item, edac_pci_ctl_info, link);
        if (*rover).pci_idx >= (*pci).pci_idx {
            if (*rover).pci_idx == (*pci).pci_idx {
                edac_printk(KERN_WARNING, EDAC_PCI,
                    "but in low-level driver: attempt to assign\n\tduplicate pci_idx %d in %s()\n",
                    (*rover).pci_idx, c"add_edac_pci_to_global_list");
                return 1;
            }
            insert_before = item;
            break;
        }
    }
    list_add_tail_rcu(&mut (*pci).link, insert_before);
    return 0;

    unsafe fn goto_fail0(rover: *mut edac_pci_ctl_info) {
        edac_printk(KERN_WARNING, EDAC_PCI, "%s (%s) %s %s already assigned %d\n",
            dev_name((*rover).dev), edac_dev_name(rover), (*rover).mod_name,
            (*rover).ctl_name, (*rover).pci_idx);
    }
}

unsafe fn del_edac_pci_from_global_list(pci: *mut edac_pci_ctl_info) {
    list_del_rcu(&mut (*pci).link);
    synchronize_rcu();
    INIT_LIST_HEAD(&mut (*pci).link);
}

unsafe fn edac_pci_workq_function(work_req: *mut work_struct) {
    let d_work = to_delayed_work(work_req);
    let pci = to_edac_pci_ctl_work(d_work);
    edac_dbg(3, "checking\n");
    mutex_lock(&mut edac_pci_ctls_mutex);
    if (*pci).op_state != OP_RUNNING_POLL {
        mutex_unlock(&mut edac_pci_ctls_mutex); return;
    }
    if edac_pci_get_check_errors() { ((*pci).edac_check.unwrap())(pci); }
    let msec = edac_pci_get_poll_msec();
    let delay = if msec == 1000 { round_jiffies_relative(msecs_to_jiffies(msec)) } else { msecs_to_jiffies(msec) };
    edac_queue_work(&mut (*pci).work, delay);
    mutex_unlock(&mut edac_pci_ctls_mutex);
}

pub unsafe fn edac_pci_alloc_index() -> c_int { pci_indexes.inc_return() - 1 }

pub unsafe fn edac_pci_add_device(pci: *mut edac_pci_ctl_info, edac_idx: c_int) -> c_int {
    edac_dbg(0, "\n");
    (*pci).pci_idx = edac_idx; (*pci).start_time = jiffies;
    mutex_lock(&mut edac_pci_ctls_mutex);
    if add_edac_pci_to_global_list(pci) != 0 { mutex_unlock(&mut edac_pci_ctls_mutex); return 1; }
    if edac_pci_create_sysfs(pci) != 0 {
        edac_pci_printk(pci, KERN_WARNING, "failed to create sysfs pci\n");
        del_edac_pci_from_global_list(pci); mutex_unlock(&mut edac_pci_ctls_mutex); return 1;
    }
    if (*pci).edac_check.is_some() {
        (*pci).op_state = OP_RUNNING_POLL;
        INIT_DELAYED_WORK(&mut (*pci).work, edac_pci_workq_function);
        edac_queue_work(&mut (*pci).work, msecs_to_jiffies(edac_pci_get_poll_msec()));
    } else { (*pci).op_state = OP_RUNNING_INTERRUPT; }
    edac_pci_printk(pci, KERN_INFO, "Giving out device to module %s controller %s: DEV %s (%s)\n",
        (*pci).mod_name, (*pci).ctl_name, (*pci).dev_name, edac_op_state_to_string((*pci).op_state));
    mutex_unlock(&mut edac_pci_ctls_mutex); 0
}

pub unsafe fn edac_pci_del_device(dev: *mut device) -> *mut edac_pci_ctl_info {
    edac_dbg(0, "\n"); mutex_lock(&mut edac_pci_ctls_mutex);
    let pci = find_edac_pci_by_dev(dev);
    if pci.is_null() { mutex_unlock(&mut edac_pci_ctls_mutex); return core::ptr::null_mut(); }
    (*pci).op_state = OP_OFFLINE; del_edac_pci_from_global_list(pci);
    mutex_unlock(&mut edac_pci_ctls_mutex);
    if (*pci).edac_check.is_some() { edac_stop_work(&mut (*pci).work); }
    edac_printk(KERN_INFO, EDAC_PCI, "Removed device %d for %s %s: DEV %s\n",
        (*pci).pci_idx, (*pci).mod_name, (*pci).ctl_name, edac_dev_name(pci)); pci
}

unsafe fn edac_pci_generic_check(_pci: *mut edac_pci_ctl_info) { edac_dbg(4, "\n"); edac_pci_do_parity_check(); }
static mut edac_pci_idx: c_int = 0;
pub const EDAC_PCI_GENCTL_NAME: &str = "EDAC PCI controller";

#[repr(C)] pub struct edac_pci_gen_data { pub edac_idx: c_int }

pub unsafe fn edac_pci_create_generic_ctl(dev: *mut device, mod_name: *const c_char) -> *mut edac_pci_ctl_info {
    let pci = edac_pci_alloc_ctl_info(core::mem::size_of::<edac_pci_gen_data>() as u32, c"EDAC PCI controller".as_ptr());
    if pci.is_null() { return core::ptr::null_mut(); }
    let pdata = (*pci).pvt_info as *mut edac_pci_gen_data;
    (*pci).dev = dev; dev_set_drvdata(dev, pci); (*pci).dev_name = pci_name(to_pci_dev(dev));
    (*pci).mod_name = mod_name; (*pci).ctl_name = c"EDAC PCI controller".as_ptr();
    if edac_op_state == EDAC_OPSTATE_POLL { (*pci).edac_check = Some(edac_pci_generic_check); }
    (*pdata).edac_idx = edac_pci_idx; edac_pci_idx += 1;
    if edac_pci_add_device(pci, (*pdata).edac_idx) > 0 { edac_dbg(3, "failed edac_pci_add_device()\n"); edac_pci_free_ctl_info(pci); return core::ptr::null_mut(); }
    pci
}

pub unsafe fn edac_pci_release_generic_ctl(pci: *mut edac_pci_ctl_info) {
    edac_dbg(0, "pci mod=%s\n", (*pci).mod_name); edac_pci_del_device((*pci).dev); edac_pci_free_ctl_info(pci);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
