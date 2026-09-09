// SPDX-License-Identifier: GPL-2.0-only
/* Driver for MPC52xx processor BestComm peripheral controller. */

// Linux kernel headers and BestComm headers provide the types, constants, and
// functions referenced below.

const DRIVER_NAME: &[u8] = b"bestcomm-core\0";

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
}

static MPC52XX_SRAM_IDS: &[OfDeviceId] = &[
    OfDeviceId { compatible: b"fsl,mpc5200-sram\0".as_ptr() },
    OfDeviceId { compatible: b"mpc5200-sram\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

pub static mut BCOM_ENG: *mut bcom_engine = core::ptr::null_mut();

pub unsafe fn bcom_task_alloc(bd_count: i32, bd_size: i32, priv_size: i32) -> *mut bcom_task {
    let mut tasknum: i32 = -1;
    let mut tsk: *mut bcom_task = core::ptr::null_mut();
    if BCOM_ENG.is_null() { return core::ptr::null_mut(); }
    spin_lock(&mut (*BCOM_ENG).lock);
    for i in 0..BCOM_MAX_TASKS {
        if (*BCOM_ENG).tdt.add(i as usize).as_ref().unwrap().stop == 0 {
            (*BCOM_ENG).tdt.add(i as usize).as_mut().unwrap().stop = 0xffff_ffff;
            tasknum = i;
            break;
        }
    }
    spin_unlock(&mut (*BCOM_ENG).lock);
    if tasknum < 0 { return core::ptr::null_mut(); }
    tsk = kzalloc(core::mem::size_of::<bcom_task>() + priv_size as usize, GFP_KERNEL);
    if tsk.is_null() { goto_error(tasknum); return core::ptr::null_mut(); }
    (*tsk).tasknum = tasknum;
    if priv_size != 0 { (*tsk).priv_ = (tsk as *mut u8).add(core::mem::size_of::<bcom_task>()) as *mut core::ffi::c_void; }
    (*tsk).irq = irq_of_parse_and_map((*BCOM_ENG).ofnode, tasknum);
    if (*tsk).irq == 0 { goto_error_with_task(tsk, tasknum); return core::ptr::null_mut(); }
    if bd_count != 0 {
        (*tsk).cookie = kmalloc_array(bd_count as usize, core::mem::size_of::<*mut core::ffi::c_void>(), GFP_KERNEL);
        if (*tsk).cookie.is_null() { goto_error_with_task(tsk, tasknum); return core::ptr::null_mut(); }
        (*tsk).bd = bcom_sram_alloc((bd_count * bd_size) as usize, 4, &mut (*tsk).bd_pa);
        if (*tsk).bd.is_null() { goto_error_with_task(tsk, tasknum); return core::ptr::null_mut(); }
        memset_io((*tsk).bd, 0, (bd_count * bd_size) as usize);
        (*tsk).num_bd = bd_count; (*tsk).bd_size = bd_size;
    }
    tsk
}

unsafe fn goto_error(tasknum: i32) { (*BCOM_ENG).tdt.add(tasknum as usize).as_mut().unwrap().stop = 0; }
unsafe fn goto_error_with_task(tsk: *mut bcom_task, tasknum: i32) {
    if (*tsk).irq != 0 { irq_dispose_mapping((*tsk).irq); }
    bcom_sram_free((*tsk).bd); kfree((*tsk).cookie); kfree(tsk as *mut core::ffi::c_void); goto_error(tasknum);
}

pub unsafe fn bcom_task_free(tsk: *mut bcom_task) {
    bcom_disable_task((*tsk).tasknum);
    (*BCOM_ENG).tdt.add((*tsk).tasknum as usize).as_mut().unwrap().start = 0;
    (*BCOM_ENG).tdt.add((*tsk).tasknum as usize).as_mut().unwrap().stop = 0;
    irq_dispose_mapping((*tsk).irq); bcom_sram_free((*tsk).bd); kfree((*tsk).cookie); kfree(tsk as *mut core::ffi::c_void);
}

pub unsafe fn bcom_load_image(task: i32, task_image: *mut u32) -> i32 {
    let hdr = task_image as *mut bcom_task_header;
    if (*hdr).magic != BCOM_TASK_MAGIC { printk(KERN_ERR, b"bestcomm-core: Trying to load invalid microcode\0"); return -EINVAL; }
    if task < 0 || task >= BCOM_MAX_TASKS { printk(KERN_ERR, b"bestcomm-core: Trying to load invalid task\0"); return -EINVAL; }
    let tdt = (*BCOM_ENG).tdt.add(task as usize);
    let desc: *mut u32;
    if (*tdt).start != 0 {
        desc = bcom_task_desc(task);
        if (*hdr).desc_size != bcom_task_num_descs(task) { return -EINVAL; }
    } else {
        let mut start_pa: phys_addr_t = 0;
        desc = bcom_sram_alloc((*hdr).desc_size as usize * 4, 4, &mut start_pa);
        if desc.is_null() { return -ENOMEM; }
        (*tdt).start = start_pa; (*tdt).stop = start_pa + ((*hdr).desc_size - 1) as u64 * 4;
    }
    let var = bcom_task_var(task); let inc = bcom_task_inc(task);
    memset_io(var, 0, BCOM_VAR_SIZE as usize); memset_io(inc, 0, BCOM_INC_SIZE as usize);
    let desc_src = hdr.add(1) as *mut u32;
    let var_src = desc_src.add((*hdr).desc_size as usize);
    let inc_src = var_src.add((*hdr).var_size as usize);
    memcpy_toio(desc, desc_src, (*hdr).desc_size as usize * 4);
    memcpy_toio(var.add((*hdr).first_var as usize), var_src, (*hdr).var_size as usize * 4);
    memcpy_toio(inc, inc_src, (*hdr).inc_size as usize * 4); 0
}

pub unsafe fn bcom_set_initiator(task: i32, initiator: i32) {
    bcom_set_tcr_initiator(task, initiator); let mut desc = bcom_task_desc(task); let mut next = true;
    for _ in 0..bcom_task_num_descs(task) { if bcom_desc_is_drd(*desc) && next && bcom_desc_initiator(*desc) != BCOM_INITIATOR_ALWAYS { bcom_set_desc_initiator(desc, initiator); } if bcom_desc_is_drd(*desc) { next = !bcom_drd_is_extended(*desc); } desc = desc.add(1); }
}

pub unsafe fn bcom_enable(tsk: *mut bcom_task) { bcom_enable_task((*tsk).tasknum); }
pub unsafe fn bcom_disable(tsk: *mut bcom_task) { bcom_disable_task((*tsk).tasknum); }

static FDT_OPS: [u32; 16] = [0xa0045670,0x80045670,0x21800000,0x21e00000,0x21500000,0x21400000,0x21500000,0x20400000,0x20500000,0x20800000,0x20a00000,0xc0170000,0xc0145670,0xc0345670,0xa0076540,0xa0000760];

unsafe fn bcom_engine_init_impl() -> i32 {
    let tdt_size = BCOM_MAX_TASKS as usize * core::mem::size_of::<bcom_tdt>();
    let ctx_size = BCOM_MAX_TASKS as usize * BCOM_CTX_SIZE as usize;
    let var_size = BCOM_MAX_TASKS as usize * (BCOM_VAR_SIZE as usize + BCOM_INC_SIZE as usize);
    let mut tdt_pa = 0; let mut ctx_pa = 0; let mut var_pa = 0; let mut fdt_pa = 0;
    (*BCOM_ENG).tdt = bcom_sram_alloc(tdt_size, core::mem::size_of::<u32>(), &mut tdt_pa);
    (*BCOM_ENG).ctx = bcom_sram_alloc(ctx_size, BCOM_CTX_ALIGN as usize, &mut ctx_pa);
    (*BCOM_ENG).var = bcom_sram_alloc(var_size, BCOM_VAR_ALIGN as usize, &mut var_pa);
    (*BCOM_ENG).fdt = bcom_sram_alloc(BCOM_FDT_SIZE as usize, BCOM_FDT_ALIGN as usize, &mut fdt_pa);
    if (*BCOM_ENG).tdt.is_null() || (*BCOM_ENG).ctx.is_null() || (*BCOM_ENG).var.is_null() || (*BCOM_ENG).fdt.is_null() { bcom_sram_free((*BCOM_ENG).tdt); bcom_sram_free((*BCOM_ENG).ctx); bcom_sram_free((*BCOM_ENG).var); bcom_sram_free((*BCOM_ENG).fdt); return -ENOMEM; }
    memset_io((*BCOM_ENG).tdt, 0, tdt_size); memset_io((*BCOM_ENG).ctx, 0, ctx_size); memset_io((*BCOM_ENG).var, 0, var_size); memset_io((*BCOM_ENG).fdt, 0, BCOM_FDT_SIZE as usize);
    memcpy_toio((*BCOM_ENG).fdt.add(48), FDT_OPS.as_ptr(), core::mem::size_of_val(&FDT_OPS));
    for task in 0..BCOM_MAX_TASKS { out_be16(&mut (*BCOM_ENG).regs.as_mut().unwrap().tcr[task as usize], 0); out_8(&mut (*BCOM_ENG).regs.as_mut().unwrap().ipr[task as usize], 0); (*BCOM_ENG).tdt.add(task as usize).as_mut().unwrap().context = ctx_pa; (*BCOM_ENG).tdt.add(task as usize).as_mut().unwrap().var = var_pa; (*BCOM_ENG).tdt.add(task as usize).as_mut().unwrap().fdt = fdt_pa; var_pa += (BCOM_VAR_SIZE + BCOM_INC_SIZE) as u64; ctx_pa += BCOM_CTX_SIZE as u64; }
    out_be32(&mut (*BCOM_ENG).regs.as_mut().unwrap().taskBar, tdt_pa); out_8(&mut (*BCOM_ENG).regs.as_mut().unwrap().ipr[BCOM_INITIATOR_ALWAYS as usize], BCOM_IPR_ALWAYS); spin_lock_init(&mut (*BCOM_ENG).lock); 0
}

unsafe fn bcom_engine_cleanup_impl() { for task in 0..BCOM_MAX_TASKS { out_be16(&mut (*BCOM_ENG).regs.as_mut().unwrap().tcr[task as usize], 0); out_8(&mut (*BCOM_ENG).regs.as_mut().unwrap().ipr[task as usize], 0); } out_be32(&mut (*BCOM_ENG).regs.as_mut().unwrap().taskBar, 0); bcom_sram_free((*BCOM_ENG).tdt); bcom_sram_free((*BCOM_ENG).ctx); bcom_sram_free((*BCOM_ENG).var); bcom_sram_free((*BCOM_ENG).fdt); }

// Platform probe/remove and module metadata are kernel registration glue.
// Their exact callback types are supplied by the Linux platform-driver API.
#[allow(dead_code)]
unsafe fn mpc52xx_bcom_init() -> i32 { platform_driver_register(); }
#[allow(dead_code)]
unsafe fn mpc52xx_bcom_exit() { platform_driver_unregister(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
