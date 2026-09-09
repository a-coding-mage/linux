// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PS3 device registration routines.
 *
 *  Copyright (C) 2007 Sony Computer Entertainment Inc.
 *  Copyright 2007 Sony Corp.
 */

// Linux and PS3 declarations referenced by this translation are supplied by
// the surrounding kernel translation unit.

unsafe fn ps3_register_lpm_devices() -> i32 {
    let mut result: i32;
    let mut tmp1: u64 = 0;
    let mut tmp2: u64 = 0;
    let dev = kzalloc_obj::<ps3_system_bus_device>();
    if dev.is_null() { return -ENOMEM; }
    (*dev).match_id = PS3_MATCH_ID_LPM;
    (*dev).dev_type = PS3_DEVICE_TYPE_LPM;
    result = ps3_repository_read_be_node_id(0, &mut (*dev).lpm.node_id);
    if result != 0 { goto_fail_read_repo!(); }
    result = ps3_repository_read_lpm_privileges((*dev).lpm.node_id, &mut tmp1, &mut (*dev).lpm.rights);
    if result != 0 { goto_fail_read_repo!(); }
    lv1_get_logical_partition_id(&mut tmp2);
    if tmp1 != tmp2 { result = -ENODEV; goto fail_rights; }
    if ((*dev).lpm.rights & PS3_LPM_RIGHTS_USE_LPM) == 0 { result = -EPERM; goto fail_rights; }
    result = ps3_repository_read_pu_id(0, &mut (*dev).lpm.pu_id);
    if result != 0 { goto fail_read_repo; }
    result = ps3_system_bus_device_register(dev);
    if result != 0 { goto fail_register; }
    return 0;
fail_register:
fail_rights:
fail_read_repo:
    kfree(dev as *mut _);
    result
}

unsafe fn ps3_setup_gelic_device(repo: *const ps3_repository_device) -> i32 {
    let mut result: i32;
    let p = kzalloc_obj::<GelicLayout>();
    if p.is_null() { return -ENOMEM; }
    BUG_ON((*repo).bus_type != PS3_BUS_TYPE_SB);
    BUG_ON((*repo).dev_type != PS3_DEV_TYPE_SB_GELIC);
    (*p).dev.match_id = PS3_MATCH_ID_GELIC;
    (*p).dev.dev_type = PS3_DEVICE_TYPE_SB;
    (*p).dev.bus_id = (*repo).bus_id;
    (*p).dev.dev_id = (*repo).dev_id;
    (*p).dev.d_region = &mut (*p).d_region;
    result = ps3_repository_find_interrupt(repo, PS3_INTERRUPT_TYPE_EVENT_PORT, &mut (*p).dev.interrupt_id);
    if result != 0 { goto fail_find_interrupt; }
    BUG_ON((*p).dev.interrupt_id != 0);
    result = ps3_dma_region_init(&mut (*p).dev, (*p).dev.d_region, PS3_DMA_64K, PS3_DMA_OTHER, core::ptr::null_mut(), 0);
    if result != 0 { goto fail_dma_init; }
    result = ps3_system_bus_device_register(&mut (*p).dev);
    if result != 0 { goto fail_device_register; }
    return result;
fail_device_register:
fail_dma_init:
fail_find_interrupt:
    kfree(p as *mut _);
    -ENOMEM
}

unsafe fn ps3_setup_uhc_device(repo: *const ps3_repository_device, match_id: ps3_match_id, interrupt_type: ps3_interrupt_type, reg_type: ps3_reg_type) -> i32 {
    let mut result: i32; let mut bus_addr = 0u64; let mut len = 0u64;
    let p = kzalloc_obj::<UhcLayout>();
    if p.is_null() { return -ENOMEM; }
    BUG_ON((*repo).bus_type != PS3_BUS_TYPE_SB); BUG_ON((*repo).dev_type != PS3_DEV_TYPE_SB_USB);
    (*p).dev.match_id = match_id; (*p).dev.dev_type = PS3_DEVICE_TYPE_SB;
    (*p).dev.bus_id = (*repo).bus_id; (*p).dev.dev_id = (*repo).dev_id;
    (*p).dev.d_region = &mut (*p).d_region; (*p).dev.m_region = &mut (*p).m_region;
    result = ps3_repository_find_interrupt(repo, interrupt_type, &mut (*p).dev.interrupt_id);
    if result != 0 { goto fail_find_interrupt; }
    result = ps3_repository_find_reg(repo, reg_type, &mut bus_addr, &mut len);
    if result != 0 { goto fail_find_reg; }
    result = ps3_dma_region_init(&mut (*p).dev, (*p).dev.d_region, PS3_DMA_64K, PS3_DMA_INTERNAL, core::ptr::null_mut(), 0);
    if result != 0 { goto fail_dma_init; }
    result = ps3_mmio_region_init(&mut (*p).dev, (*p).dev.m_region, bus_addr, len, PS3_MMIO_4K);
    if result != 0 { goto fail_mmio_init; }
    result = ps3_system_bus_device_register(&mut (*p).dev);
    if result != 0 { goto fail_device_register; }
    return result;
fail_device_register: fail_mmio_init: fail_dma_init: fail_find_reg: fail_find_interrupt:
    kfree(p as *mut _); result
}

unsafe fn ps3_setup_ehci_device(r: *const ps3_repository_device) -> i32 { ps3_setup_uhc_device(r, PS3_MATCH_ID_EHCI, PS3_INTERRUPT_TYPE_SB_EHCI, PS3_REG_TYPE_SB_EHCI) }
unsafe fn ps3_setup_ohci_device(r: *const ps3_repository_device) -> i32 { ps3_setup_uhc_device(r, PS3_MATCH_ID_OHCI, PS3_INTERRUPT_TYPE_SB_OHCI, PS3_REG_TYPE_SB_OHCI) }

unsafe fn ps3_setup_vuart_device(match_id: ps3_match_id, port_number: u32) -> i32 {
    let p = kzalloc_obj::<ps3_system_bus_device>(); if p.is_null() { return -ENOMEM; }
    (*p).match_id = match_id; (*p).dev_type = PS3_DEVICE_TYPE_VUART; (*p).port_number = port_number;
    let result = ps3_system_bus_device_register(p);
    if result != 0 { kfree(p as *mut _); } result
}

unsafe fn ps3_setup_storage_dev(repo: *const ps3_repository_device, match_id: ps3_match_id) -> i32 {
    let (mut port, mut blk_size, mut num_blocks) = (0u64, 0u64, 0u64); let mut num_regions = 0u32;
    let mut result = ps3_repository_read_stor_dev_info((*repo).bus_index, (*repo).dev_index, &mut port, &mut blk_size, &mut num_blocks, &mut num_regions);
    if result != 0 { return -ENODEV; }
    let p = kzalloc_storage::<ps3_storage_device>(num_regions); if p.is_null() { return -ENOMEM; }
    (*p).sbd.match_id = match_id; (*p).sbd.dev_type = PS3_DEVICE_TYPE_SB; (*p).sbd.bus_id = (*repo).bus_id; (*p).sbd.dev_id = (*repo).dev_id;
    (*p).sbd.d_region = &mut (*p).dma_region; (*p).blk_size = blk_size; (*p).num_regions = num_regions;
    result = ps3_repository_find_interrupt(repo, PS3_INTERRUPT_TYPE_EVENT_PORT, &mut (*p).sbd.interrupt_id);
    if result != 0 { kfree(p as *mut _); return -ENODEV; }
    for i in 0..num_regions { let (mut id, mut start, mut size) = (0u32, 0u64, 0u64); result = ps3_repository_read_stor_dev_region((*repo).bus_index, (*repo).dev_index, i, &mut id, &mut start, &mut size); if result != 0 { kfree(p as *mut _); return -ENODEV; } (*p).regions[i as usize].id = id; (*p).regions[i as usize].start = start; (*p).regions[i as usize].size = size; }
    result = ps3_system_bus_device_register(&mut (*p).sbd); if result != 0 { kfree(p as *mut _); } result
}

unsafe fn ps3_register_vuart_devices() -> i32 { let mut port = 0u32; let mut result = ps3_repository_read_vuart_av_port(&mut port); if result != 0 { port = 0; } result = ps3_setup_vuart_device(PS3_MATCH_ID_AV_SETTINGS, port); WARN_ON(result); result = ps3_repository_read_vuart_sysmgr_port(&mut port); if result != 0 { port = 2; } result = ps3_setup_vuart_device(PS3_MATCH_ID_SYSTEM_MANAGER, port); WARN_ON(result); result }

unsafe fn ps3_register_sound_devices() -> i32 { let p = kzalloc_obj::<SoundLayout>(); if p.is_null() { return -ENOMEM; } (*p).dev.match_id = PS3_MATCH_ID_SOUND; (*p).dev.dev_type = PS3_DEVICE_TYPE_IOC0; (*p).dev.d_region = &mut (*p).d_region; (*p).dev.m_region = &mut (*p).m_region; let r = ps3_system_bus_device_register(&mut (*p).dev); if r != 0 { kfree(p as *mut _); } r }
unsafe fn ps3_register_graphics_devices() -> i32 { let p = kzalloc_obj::<ps3_system_bus_device>(); if p.is_null() { return -ENOMEM; } (*p).match_id = PS3_MATCH_ID_GPU; (*p).match_sub_id = PS3_MATCH_SUB_ID_GPU_FB; (*p).dev_type = PS3_DEVICE_TYPE_IOC0; let r = ps3_system_bus_device_register(p); if r != 0 { kfree(p as *mut _); } r }
unsafe fn ps3_register_ramdisk_device() -> i32 { let p = kzalloc_obj::<ps3_system_bus_device>(); if p.is_null() { return -ENOMEM; } (*p).match_id = PS3_MATCH_ID_GPU; (*p).match_sub_id = PS3_MATCH_SUB_ID_GPU_RAMDISK; (*p).dev_type = PS3_DEVICE_TYPE_IOC0; let r = ps3_system_bus_device_register(p); if r != 0 { kfree(p as *mut _); } r }

unsafe fn ps3_setup_dynamic_device(repo: *const ps3_repository_device) -> i32 { match (*repo).dev_type { PS3_DEV_TYPE_STOR_DISK => { let mut r = ps3_setup_storage_dev(repo, PS3_MATCH_ID_STOR_DISK); if r == -ENODEV { r = 0; } r }, PS3_DEV_TYPE_STOR_ROM => ps3_setup_storage_dev(repo, PS3_MATCH_ID_STOR_ROM), PS3_DEV_TYPE_STOR_FLASH => ps3_setup_storage_dev(repo, PS3_MATCH_ID_STOR_FLASH), _ => 0 } }
unsafe fn ps3_setup_static_device(repo: *const ps3_repository_device) -> i32 { match (*repo).dev_type { PS3_DEV_TYPE_SB_GELIC => ps3_setup_gelic_device(repo), PS3_DEV_TYPE_SB_USB => { let _ = ps3_setup_ehci_device(repo); ps3_setup_ohci_device(repo) }, _ => ps3_setup_dynamic_device(repo) } }

unsafe fn ps3_find_and_add_device(bus_id: u64, dev_id: u64) { let mut repo = core::mem::zeroed::<ps3_repository_device>(); let mut retries = 0; while retries < 10 { if ps3_repository_find_device_by_id(&mut repo, bus_id, dev_id) == 0 { ps3_setup_dynamic_device(&repo); return; } if msleep_interruptible(100) != 0 { break; } retries += 1; } }

const PS3_NOTIFICATION_DEV_ID: u64 = ULONG_MAX as u64;
const PS3_NOTIFICATION_INTERRUPT_ID: u32 = 0;

#[repr(C)] struct ps3_notification_device { sbd: ps3_system_bus_device, lock: spinlock_t, tag: u64, lv1_status: u64, wait: rcuwait, done: bool }
#[repr(C)] enum ps3_notify_type { notify_device_ready = 0, notify_region_probe = 1, notify_region_update = 2 }
#[repr(C)] struct ps3_notify_cmd { operation_code: u64, event_mask: u64 }
#[repr(C)] struct ps3_notify_event { event_type: u64, bus_id: u64, dev_id: u64, dev_type: u64, dev_port: u64 }

unsafe fn ps3_notification_interrupt(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = data as *mut ps3_notification_device; let mut tag = 0u64; let mut status = 0u64;
    spin_lock(&mut (*dev).lock); let res = lv1_storage_get_async_status(PS3_NOTIFICATION_DEV_ID, &mut tag, &mut status);
    if res == 0 { (*dev).lv1_status = status; (*dev).done = true; rcuwait_wake_up(&mut (*dev).wait); }
    spin_unlock(&mut (*dev).lock); IRQ_HANDLED
}

unsafe fn ps3_notification_read_write(dev: *mut ps3_notification_device, lpar: u64, write: i32) -> i32 { let mut flags = 0ul; spin_lock_irqsave(&mut (*dev).lock, &mut flags); let r = if write != 0 { lv1_storage_write((*dev).sbd.dev_id, 0, 0, 1, 0, lpar, &mut (*dev).tag) } else { lv1_storage_read((*dev).sbd.dev_id, 0, 0, 1, 0, lpar, &mut (*dev).tag) }; (*dev).done = false; spin_unlock_irqrestore(&mut (*dev).lock, flags); if r != 0 { return -EPERM; } rcuwait_wait_event(&mut (*dev).wait, (*dev).done || kthread_should_stop(), TASK_IDLE); if kthread_should_stop() { return -EINTR; } if (*dev).lv1_status != 0 { return -EIO; } 0 }

static mut probe_task: *mut task_struct = core::ptr::null_mut();

unsafe fn ps3_probe_thread(data: *mut core::ffi::c_void) -> i32 {
    let local = kzalloc_obj::<ProbeLayout>();
    if local.is_null() { return -ENOMEM; }
    let lpar = ps3_mm_phys_to_lpar(__pa(&mut (*local).buf) as u64);
    let notify_cmd = &mut (*local).buf as *mut _ as *mut ps3_notify_cmd;
    let notify_event = &mut (*local).buf as *mut _ as *mut ps3_notify_event;
    (*local).dev.sbd.bus_id = data as u64;
    (*local).dev.sbd.dev_id = PS3_NOTIFICATION_DEV_ID;
    (*local).dev.sbd.interrupt_id = PS3_NOTIFICATION_INTERRUPT_ID;
    let mut irq = 0u32;
    let mut res = lv1_open_device((*local).dev.sbd.bus_id, (*local).dev.sbd.dev_id, 0);
    if res != 0 { kfree(local as *mut _); return res; }
    res = ps3_sb_event_receive_port_setup(&mut (*local).dev.sbd, PS3_BINDING_CPU_ANY, &mut irq);
    if res != 0 { lv1_close_device((*local).dev.sbd.bus_id, (*local).dev.sbd.dev_id); kfree(local as *mut _); return res; }
    spin_lock_init(&mut (*local).dev.lock); rcuwait_init(&mut (*local).dev.wait);
    res = request_irq(irq, ps3_notification_interrupt, 0, "ps3_notification", &mut (*local).dev as *mut _);
    if res != 0 { ps3_sb_event_receive_port_destroy(&mut (*local).dev.sbd, irq); lv1_close_device((*local).dev.sbd.bus_id, (*local).dev.sbd.dev_id); kfree(local as *mut _); return res; }
    (*notify_cmd).operation_code = 0; (*notify_cmd).event_mask = 1u64 << notify_region_probe as u64;
    res = ps3_notification_read_write(&mut (*local).dev, lpar, 1);
    if res == 0 { set_freezable(); while !kthread_should_stop() { try_to_freeze(); memset(notify_event as *mut _, 0, core::mem::size_of::<ps3_notify_event>()); res = ps3_notification_read_write(&mut (*local).dev, lpar, 0); if res != 0 { break; } if (*notify_event).event_type == notify_region_probe as u64 && (*notify_event).bus_id == (*local).dev.sbd.bus_id { ps3_find_and_add_device((*local).dev.sbd.bus_id, (*notify_event).dev_id); } } }
    free_irq(irq, &mut (*local).dev as *mut _); ps3_sb_event_receive_port_destroy(&mut (*local).dev.sbd, irq); lv1_close_device((*local).dev.sbd.bus_id, (*local).dev.sbd.dev_id); kfree(local as *mut _); probe_task = core::ptr::null_mut(); 0
}

unsafe fn ps3_stop_probe_thread(_nb: *mut notifier_block, _code: u64, _data: *mut core::ffi::c_void) -> i32 { if !probe_task.is_null() { kthread_stop(probe_task); } 0 }
static mut nb: notifier_block = notifier_block { notifier_call: ps3_stop_probe_thread };

unsafe fn ps3_start_probe_thread(bus_type: ps3_bus_type) -> i32 { let mut repo = core::mem::zeroed::<ps3_repository_device>(); repo.bus_type = bus_type; if ps3_repository_find_bus(repo.bus_type, 0, &mut repo.bus_index) != 0 { return -ENODEV; } if ps3_repository_read_bus_id(repo.bus_index, &mut repo.bus_id) != 0 { return -ENODEV; } let task = kthread_run(ps3_probe_thread, repo.bus_id as *mut _, "ps3-probe-%u"); if IS_ERR(task) { return PTR_ERR(task); } probe_task = task; register_reboot_notifier(&mut nb); 0 }

unsafe fn ps3_register_devices() -> i32 { if !firmware_has_feature(FW_FEATURE_PS3_LV1) { return -ENODEV; } ps3_start_probe_thread(PS3_BUS_TYPE_STORAGE); ps3_register_vuart_devices(); ps3_register_graphics_devices(); ps3_repository_find_devices(PS3_BUS_TYPE_SB, ps3_setup_static_device); ps3_register_sound_devices(); ps3_register_lpm_devices(); ps3_register_ramdisk_device(); 0 }

// Local aggregate layouts corresponding to the C function-local structs.
#[repr(C)] struct GelicLayout { dev: ps3_system_bus_device, d_region: ps3_dma_region }
#[repr(C)] struct UhcLayout { dev: ps3_system_bus_device, d_region: ps3_dma_region, m_region: ps3_mmio_region }
#[repr(C)] struct SoundLayout { dev: ps3_system_bus_device, d_region: ps3_dma_region, m_region: ps3_mmio_region }
#[repr(C)] struct ProbeLayout { dev: ps3_notification_device, buf: [u8; 512] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
