// SPDX-License-Identifier: GPL-2.0
/* pci_msi.c: Sparc64 MSI support common layer.
 *
 * Copyright (C) 2007 David S. Miller (davem@davemloft.net)
 */

static unsafe fn sparc64_msiq_interrupt(irq: i32, cookie: *mut core::ffi::c_void) -> irqreturn_t {
    let msiq_cookie = cookie as *mut sparc64_msiq_cookie;
    let pbm = (*msiq_cookie).pbm;
    let msiqid = (*msiq_cookie).msiqid;
    let ops = (*pbm).msi_ops;
    let mut orig_head: c_ulong;
    let mut head: c_ulong = 0;
    let mut err: i32;

    err = ((*ops).get_head)(pbm, msiqid, &mut head);
    if err < 0 {
        goto_err_get_head!(msiqid, err);
    }

    orig_head = head;
    loop {
        let mut msi: c_ulong = 0;
        err = ((*ops).dequeue_msi)(pbm, msiqid, &mut head, &mut msi);
        if err > 0 {
            let irq = (*pbm).msi_irq_table[(msi - (*pbm).msi_first) as usize];
            generic_handle_irq(irq);
        }
        if err < 0 {
            printk!(KERN_EMERG, "MSI: Dequeue head[%lu] from msiqid[%lu] gives error %d\n", head, msiqid, err);
            return IRQ_NONE;
        }
        if err == 0 { break; }
    }
    if head != orig_head {
        err = ((*ops).set_head)(pbm, msiqid, head);
        if err < 0 {
            printk!(KERN_EMERG, "MSI: Set head[%lu] on msiqid[%lu] gives error %d\n", head, msiqid, err);
            return IRQ_NONE;
        }
    }
    IRQ_HANDLED
}

static unsafe fn pick_msiq(pbm: *mut pci_pbm_info) -> u32 {
    static mut ROTOR_LOCK: spinlock_t = DEFINE_SPINLOCK!();
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut ROTOR_LOCK, &mut flags);
    let mut rotor = (*pbm).msiq_rotor;
    let ret = (*pbm).msiq_first + rotor;
    rotor += 1;
    if rotor >= (*pbm).msiq_num { rotor = 0; }
    (*pbm).msiq_rotor = rotor;
    spin_unlock_irqrestore(&mut ROTOR_LOCK, flags);
    ret
}

static unsafe fn alloc_msi(pbm: *mut pci_pbm_info) -> i32 {
    for i in 0..(*pbm).msi_num {
        if test_and_set_bit(i as usize, (*pbm).msi_bitmap) == 0 {
            return (i + (*pbm).msi_first) as i32;
        }
    }
    -ENOENT
}

static unsafe fn free_msi(pbm: *mut pci_pbm_info, mut msi_num: i32) {
    msi_num -= (*pbm).msi_first as i32;
    clear_bit(msi_num as usize, (*pbm).msi_bitmap);
}

static mut msi_irq: irq_chip = irq_chip {
    name: cstr!("PCI-MSI"),
    irq_mask: Some(pci_msi_mask_irq),
    irq_unmask: Some(pci_msi_unmask_irq),
    irq_enable: Some(pci_msi_unmask_irq),
    irq_disable: Some(pci_msi_mask_irq),
    ..unsafe { core::mem::zeroed() }
};

static unsafe fn sparc64_setup_msi_irq(irq_p: *mut u32, pdev: *mut pci_dev, entry: *mut msi_desc) -> i32 {
    let pbm = (*(*pdev).dev.archdata.host_controller);
    let ops = (*pbm).msi_ops;
    let mut msg: msi_msg = core::mem::zeroed();
    let mut msi: i32;
    let mut err: i32;
    let msiqid: u32;

    *irq_p = irq_alloc(0, 0);
    err = -ENOMEM;
    if *irq_p == 0 { return err; }
    irq_set_chip_and_handler_name(*irq_p, &mut msi_irq, handle_simple_irq, cstr!("MSI"));
    err = alloc_msi(pbm);
    if err < 0 { irq_set_chip(*irq_p, core::ptr::null_mut()); irq_free(*irq_p); *irq_p = 0; return err; }
    msi = err;
    msiqid = pick_msiq(pbm);
    err = ((*ops).msi_setup)(pbm, msiqid, msi as u32, if (*entry).pci.msi_attrib.is_64 { 1 } else { 0 });
    if err != 0 { free_msi(pbm, msi); irq_set_chip(*irq_p, core::ptr::null_mut()); irq_free(*irq_p); *irq_p = 0; return err; }
    (*pbm).msi_irq_table[(msi as u32 - (*pbm).msi_first) as usize] = *irq_p;
    if (*entry).pci.msi_attrib.is_64 {
        msg.address_hi = ((*pbm).msi64_start >> 32) as u32;
        msg.address_lo = (*pbm).msi64_start as u32;
    } else { msg.address_hi = 0; msg.address_lo = (*pbm).msi32_start as u32; }
    msg.data = msi as u32;
    irq_set_msi_desc(*irq_p, entry);
    pci_write_msi_msg(*irq_p, &msg);
    0
}

static unsafe fn sparc64_teardown_msi_irq(irq: u32, pdev: *mut pci_dev) {
    let pbm = (*(*pdev).dev.archdata.host_controller);
    let ops = (*pbm).msi_ops;
    let mut i = 0;
    while i < (*pbm).msi_num as usize && (*pbm).msi_irq_table[i] != irq { i += 1; }
    if i >= (*pbm).msi_num as usize { pci_err!(pdev, "{}: teardown: No MSI for irq {}\n", (*pbm).name, irq); return; }
    let msi_num = (*pbm).msi_first + i as u32;
    (*pbm).msi_irq_table[i] = !0u32;
    let err = ((*ops).msi_teardown)(pbm, msi_num);
    if err != 0 { pci_err!(pdev, "{}: teardown: ops->teardown() on MSI {}, irq {}, gives error {}\n", (*pbm).name, msi_num, irq, err); return; }
    free_msi(pbm, msi_num as i32); irq_set_chip(irq, core::ptr::null_mut()); irq_free(irq);
}

static unsafe fn msi_bitmap_alloc(pbm: *mut pci_pbm_info) -> i32 {
    let bits_per_ulong = core::mem::size_of::<c_ulong>() * 8;
    let mut size = ((*pbm).msi_num as usize + bits_per_ulong - 1) & !(bits_per_ulong - 1);
    size /= 8;
    BUG_ON!(size % core::mem::size_of::<c_ulong>() != 0);
    (*pbm).msi_bitmap = kzalloc(size, GFP_KERNEL);
    if (*pbm).msi_bitmap.is_null() { return -ENOMEM; }
    0
}

static unsafe fn msi_bitmap_free(pbm: *mut pci_pbm_info) { kfree((*pbm).msi_bitmap); (*pbm).msi_bitmap = core::ptr::null_mut(); }

static unsafe fn msi_table_alloc(pbm: *mut pci_pbm_info) -> i32 {
    let size = (*pbm).msiq_num as usize * core::mem::size_of::<sparc64_msiq_cookie>();
    (*pbm).msiq_irq_cookies = kzalloc(size, GFP_KERNEL);
    if (*pbm).msiq_irq_cookies.is_null() { return -ENOMEM; }
    for i in 0..(*pbm).msiq_num as usize { (*pbm).msiq_irq_cookies.add(i).write(sparc64_msiq_cookie { pbm, msiqid: (*pbm).msiq_first + i as u32 }); }
    let size = (*pbm).msi_num as usize * core::mem::size_of::<u32>();
    (*pbm).msi_irq_table = kzalloc(size, GFP_KERNEL);
    if (*pbm).msi_irq_table.is_null() { kfree((*pbm).msiq_irq_cookies); (*pbm).msiq_irq_cookies = core::ptr::null_mut(); return -ENOMEM; }
    0
}

static unsafe fn msi_table_free(pbm: *mut pci_pbm_info) { kfree((*pbm).msiq_irq_cookies); (*pbm).msiq_irq_cookies = core::ptr::null_mut(); kfree((*pbm).msi_irq_table); (*pbm).msi_irq_table = core::ptr::null_mut(); }

static unsafe fn bringup_one_msi_queue(pbm: *mut pci_pbm_info, ops: *const sparc64_msiq_ops, msiqid: c_ulong, devino: c_ulong) -> i32 {
    let irq = ((*ops).msiq_build_irq)(pbm, msiqid, devino);
    if irq < 0 { return irq; }
    if (*pbm).numa_node != -1 { irq_set_affinity(irq, cpumask_of_node((*pbm).numa_node)); }
    let err = request_irq(irq, Some(sparc64_msiq_interrupt), 0, cstr!("MSIQ"), &mut *(*pbm).msiq_irq_cookies.add((msiqid - (*pbm).msiq_first as c_ulong) as usize) as *mut _ as *mut c_void);
    if err != 0 { return err; } 0
}

static unsafe fn sparc64_bringup_msi_queues(pbm: *mut pci_pbm_info, ops: *const sparc64_msiq_ops) -> i32 {
    for i in 0..(*pbm).msiq_num { let err = bringup_one_msi_queue(pbm, ops, i as c_ulong + (*pbm).msiq_first as c_ulong, i as c_ulong + (*pbm).msiq_first_devino as c_ulong); if err != 0 { return err; } }
    0
}

pub unsafe fn sparc64_pbm_msi_init(pbm: *mut pci_pbm_info, ops: *const sparc64_msiq_ops) {
    let mut len = 0;
    let val = of_get_property((*(*pbm).op).dev.of_node, cstr!("#msi-eqs"), &mut len);
    if val.is_null() || len != 4 { (*pbm).msiq_num = 0; printk!(KERN_INFO, "{}: No MSI support.\n", (*pbm).name); return; }
    (*pbm).msiq_num = *(val as *const u32);
    if (*pbm).msiq_num == 0 { return; }
    let val = of_get_property((*(*pbm).op).dev.of_node, cstr!("msi-eq-size"), &mut len); if val.is_null() || len != 4 { (*pbm).msiq_num = 0; return; } (*pbm).msiq_ent_count = *(val as *const u32);
    let mqp = of_get_property((*(*pbm).op).dev.of_node, cstr!("msi-eq-to-devino"), &mut len); if mqp.is_null() || len != core::mem::size_of::<msiq_prop>() as i32 { (*pbm).msiq_num = 0; return; }
    let mqp = mqp as *const msiq_prop; (*pbm).msiq_first = (*mqp).first_msiq; (*pbm).msiq_first_devino = (*mqp).first_devino;
    let val = of_get_property((*(*pbm).op).dev.of_node, cstr!("#msi"), &mut len); if val.is_null() || len != 4 { (*pbm).msiq_num = 0; return; } (*pbm).msi_num = *(val as *const u32);
    let mrng = of_get_property((*(*pbm).op).dev.of_node, cstr!("msi-ranges"), &mut len); if mrng.is_null() || len != core::mem::size_of::<msi_range_prop>() as i32 { (*pbm).msiq_num = 0; return; } (*pbm).msi_first = (*(mrng as *const msi_range_prop)).first_msi;
    let val = of_get_property((*(*pbm).op).dev.of_node, cstr!("msi-data-mask"), &mut len); if val.is_null() || len != 4 { (*pbm).msiq_num = 0; return; } (*pbm).msi_data_mask = *(val as *const u32);
    let val = of_get_property((*(*pbm).op).dev.of_node, cstr!("msix-data-width"), &mut len); if val.is_null() || len != 4 { (*pbm).msiq_num = 0; return; } (*pbm).msix_data_width = *(val as *const u32);
    let arng = of_get_property((*(*pbm).op).dev.of_node, cstr!("msi-address-ranges"), &mut len); if arng.is_null() || len != core::mem::size_of::<addr_range_prop>() as i32 { (*pbm).msiq_num = 0; return; }
    let arng = arng as *const addr_range_prop; (*pbm).msi32_start = ((*arng).msi32_high as u64) << 32 | (*arng).msi32_low as u64; (*pbm).msi64_start = ((*arng).msi64_high as u64) << 32 | (*arng).msi64_low as u64; (*pbm).msi32_len = (*arng).msi32_len; (*pbm).msi64_len = (*arng).msi64_len;
    if msi_bitmap_alloc(pbm) != 0 { (*pbm).msiq_num = 0; return; } if msi_table_alloc(pbm) != 0 { msi_bitmap_free(pbm); (*pbm).msiq_num = 0; return; } if ((*ops).msiq_alloc)(pbm) != 0 { msi_table_free(pbm); msi_bitmap_free(pbm); (*pbm).msiq_num = 0; return; } if sparc64_bringup_msi_queues(pbm, ops) != 0 { ((*ops).msiq_free)(pbm); msi_table_free(pbm); msi_bitmap_free(pbm); (*pbm).msiq_num = 0; return; }
    (*pbm).msi_ops = ops; (*pbm).setup_msi_irq = Some(sparc64_setup_msi_irq); (*pbm).teardown_msi_irq = Some(sparc64_teardown_msi_irq);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
