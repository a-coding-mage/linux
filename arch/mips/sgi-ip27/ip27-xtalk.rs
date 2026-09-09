// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 1999, 2000 Ralf Baechle (ralf@gnu.org)
 * Copyright (C) 1999, 2000 Silcon Graphics, Inc.
 * Copyright (C) 2004 Christoph Hellwig.
 *
 * Generic XTALK initialization code
 */

// Dependencies supplied by the surrounding kernel translation.

const XBOW_WIDGET_PART_NUM: u32 = 0x0;
const XXBOW_WIDGET_PART_NUM: u32 = 0xd000; // Xbow in Xbridge
const BASE_XBOW_PORT: i32 = 8; // Lowest external port

unsafe fn bridge_platform_create(nasid: nasid_t, widget: i32, masterwid: i32) {
    let mut bd: *mut xtalk_bridge_platform_data;
    let mut wd: *mut sgi_w1_platform_data;
    let mut pdev_wd: *mut platform_device;
    let mut pdev_bd: *mut platform_device;
    let mut w1_res: resource = core::mem::zeroed();
    let offset: libc::c_ulong = NODE_OFFSET(nasid);

    wd = kzalloc_obj::<sgi_w1_platform_data>();
    if wd.is_null() {
        pr_warn!("xtalk:n%d/%x bridge create out of memory\n", nasid, widget);
        return;
    }

    snprintf((*wd).dev_id.as_mut_ptr(), core::mem::size_of_val(&(*wd).dev_id),
             "bridge-%012lx", offset + ((widget as libc::c_ulong) << SWIN_SIZE_BITS));

    w1_res.start = offset + ((widget as libc::c_ulong) << SWIN_SIZE_BITS)
        + core::mem::offset_of!(bridge_regs, b_nic);
    w1_res.end = w1_res.start + 3;
    w1_res.flags = IORESOURCE_MEM;

    pdev_wd = platform_device_alloc("sgi_w1", PLATFORM_DEVID_AUTO);
    if pdev_wd.is_null() {
        pr_warn!("xtalk:n%d/%x bridge create out of memory\n", nasid, widget);
        goto_err_kfree_wd(wd);
        return;
    }
    if platform_device_add_resources(pdev_wd, &mut w1_res, 1) != 0 {
        pr_warn!("xtalk:n%d/%x bridge failed to add platform resources.\n", nasid, widget);
        platform_device_put(pdev_wd);
        goto_err_kfree_wd(wd);
        return;
    }
    if platform_device_add_data(pdev_wd, wd as *const _, core::mem::size_of::<sgi_w1_platform_data>()) != 0 {
        pr_warn!("xtalk:n%d/%x bridge failed to add platform data.\n", nasid, widget);
        platform_device_put(pdev_wd);
        goto_err_kfree_wd(wd);
        return;
    }
    if platform_device_add(pdev_wd) != 0 {
        pr_warn!("xtalk:n%d/%x bridge failed to add platform device.\n", nasid, widget);
        platform_device_put(pdev_wd);
        goto_err_kfree_wd(wd);
        return;
    }
    // platform_device_add_data() duplicates the data
    kfree(wd);

    bd = kzalloc_obj::<xtalk_bridge_platform_data>();
    if bd.is_null() {
        pr_warn!("xtalk:n%d/%x bridge create out of memory\n", nasid, widget);
        platform_device_unregister(pdev_wd);
        return;
    }
    pdev_bd = platform_device_alloc("xtalk-bridge", PLATFORM_DEVID_AUTO);
    if pdev_bd.is_null() {
        pr_warn!("xtalk:n%d/%x bridge create out of memory\n", nasid, widget);
        kfree(bd);
        platform_device_unregister(pdev_wd);
        return;
    }

    (*bd).bridge_addr = RAW_NODE_SWIN_BASE(nasid, widget);
    (*bd).intr_addr = (1u64 << 47) + 0x01800000 + PI_INT_PEND_MOD;
    (*bd).nasid = nasid;
    (*bd).masterwid = masterwid;
    (*bd).mem.name = "Bridge PCI MEM";
    (*bd).mem.start = offset + ((widget as libc::c_ulong) << SWIN_SIZE_BITS) + BRIDGE_DEVIO0;
    (*bd).mem.end = offset + ((widget as libc::c_ulong) << SWIN_SIZE_BITS) + SWIN_SIZE - 1;
    (*bd).mem.flags = IORESOURCE_MEM;
    (*bd).mem_offset = offset;
    (*bd).io.name = "Bridge PCI IO";
    (*bd).io.start = offset + ((widget as libc::c_ulong) << SWIN_SIZE_BITS) + BRIDGE_DEVIO0;
    (*bd).io.end = offset + ((widget as libc::c_ulong) << SWIN_SIZE_BITS) + SWIN_SIZE - 1;
    (*bd).io.flags = IORESOURCE_IO;
    (*bd).io_offset = offset;

    if platform_device_add_data(pdev_bd, bd as *const _, core::mem::size_of::<xtalk_bridge_platform_data>()) != 0 {
        pr_warn!("xtalk:n%d/%x bridge failed to add platform data.\n", nasid, widget);
        platform_device_put(pdev_bd);
        kfree(bd);
        platform_device_unregister(pdev_wd);
        return;
    }
    if platform_device_add(pdev_bd) != 0 {
        pr_warn!("xtalk:n%d/%x bridge failed to add platform device.\n", nasid, widget);
        platform_device_put(pdev_bd);
        kfree(bd);
        platform_device_unregister(pdev_wd);
        return;
    }
    // platform_device_add_data() duplicates the data
    kfree(bd);
    pr_info!("xtalk:n%d/%x bridge widget\n", nasid, widget);
}

unsafe fn probe_one_port(nasid: nasid_t, widget: i32, masterwid: i32) -> i32 {
    let widget_id: widgetreg_t = core::ptr::read_volatile((RAW_NODE_SWIN_BASE(nasid, widget) + WIDGET_ID) as *const _);
    let partnum = XWIDGET_PART_NUM(widget_id);
    match partnum {
        BRIDGE_WIDGET_PART_NUM | XBRIDGE_WIDGET_PART_NUM => bridge_platform_create(nasid, widget, masterwid),
        _ => pr_info!("xtalk:n%d/%d unknown widget (0x%x)\n", nasid, widget, partnum),
    }
    0
}

unsafe fn xbow_probe(nasid: nasid_t) -> i32 {
    let brd = find_lboard(KL_CONFIG_INFO(nasid), KLTYPE_MIDPLANE8);
    if brd.is_null() { return -ENODEV; }
    let xbow_p = find_component(brd, core::ptr::null_mut(), KLSTRUCT_XBOW);
    if xbow_p.is_null() { return -ENODEV; }
    let mut i: i32 = HUB_WIDGET_ID_MIN - 1;
    loop { i += 1; if XBOW_PORT_TYPE_HUB(xbow_p, i) && XBOW_PORT_IS_ENABLED(xbow_p, i) { break; } }
    let masterwid = i;
    if nasid != XBOW_PORT_NASID(xbow_p, i) { return 1; }
    for i in HUB_WIDGET_ID_MIN..=HUB_WIDGET_ID_MAX {
        if XBOW_PORT_IS_ENABLED(xbow_p, i) && XBOW_PORT_TYPE_IO(xbow_p, i) { probe_one_port(nasid, i, masterwid); }
    }
    0
}

unsafe fn xtalk_probe_node(nasid: nasid_t) {
    let hubreg = REMOTE_HUB_L(nasid, IIO_LLP_CSR);
    if hubreg & IIO_LLP_CSR_IS_UP == 0 { return; }
    let widget_id: widgetreg_t = core::ptr::read_volatile((RAW_NODE_SWIN_BASE(nasid, 0x0) + WIDGET_ID) as *const _);
    match XWIDGET_PART_NUM(widget_id) {
        BRIDGE_WIDGET_PART_NUM => bridge_platform_create(nasid, 0x8, 0xa),
        XBOW_WIDGET_PART_NUM | XXBOW_WIDGET_PART_NUM => { pr_info!("xtalk:n%d/0 xbow widget\n", nasid); xbow_probe(nasid); },
        _ => pr_info!("xtalk:n%d/0 unknown widget (0x%x)\n", nasid, XWIDGET_PART_NUM(widget_id)),
    }
}

unsafe fn xtalk_init() -> i32 {
    for_each_online_node!(nasid, { xtalk_probe_node(nasid); });
    0
}

arch_initcall!(xtalk_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
