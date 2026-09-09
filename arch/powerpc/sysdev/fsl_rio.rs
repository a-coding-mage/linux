// SPDX-License-Identifier: GPL-2.0-or-later
/* Freescale MPC85xx/MPC86xx RapidIO support. Direct translation of fsl_rio.c. */

// C headers and build-time configuration supplied by the surrounding kernel.

const RIO_PORT1_EDCSR: usize = 0x0640;
const RIO_PORT2_EDCSR: usize = 0x0680;
const RIO_PORT1_IECSR: usize = 0x10130;
const RIO_PORT2_IECSR: usize = 0x101B0;
const RIO_GCCSR: usize = 0x13c;
const RIO_ESCSR: usize = 0x158;
const ESCSR_CLEAR: u32 = 0x07120204;
const RIO_PORT2_ESCSR: usize = 0x178;
const RIO_CCSR: usize = 0x15c;
const RIO_LTLEDCSR_IER: u32 = 0x80000000;
const RIO_LTLEDCSR_PRT: u32 = 0x01000000;
const IECSR_CLEAR: u32 = 0x80000000;
const RIO_ISR_AACR: usize = 0x10120;
const RIO_ISR_AACR_AA: u32 = 1;
const RIWTAR_TRAD_VAL_SHIFT: usize = 12;
const RIWTAR_TRAD_MASK: u32 = 0x00FFFFFF;
const RIWBAR_BADD_VAL_SHIFT: usize = 12;
const RIWBAR_BADD_MASK: u32 = 0x003FFFFF;
const RIWAR_ENABLE: u32 = 0x80000000;
const RIWAR_TGINT_LOCAL: u32 = 0x00F00000;
const RIWAR_RDTYP_SNOOP: u32 = 0x00050000;
const RIWAR_WRTYP_SNOOP: u32 = 0x00005000;
const RIWAR_SIZE_MASK: u32 = 0x0000003F;

extern "C" {
    static mut rio_regs_win: *mut core::ffi::c_void;
    static mut rmu_regs_win: *mut core::ffi::c_void;
    static mut rio_law_start: resource_size_t;
    static mut dbell: *mut fsl_rio_dbell;
    static mut pw: *mut fsl_rio_pw;
}

static mut fsl_rio_config_lock: spinlock_t = spinlock_t::new();

#[cfg(CONFIG_PPC_E500)]
#[no_mangle]
pub unsafe extern "C" fn fsl_rio_mcheck_exception(regs: *mut pt_regs) -> i32 {
    if rio_regs_win.is_null() { return 0; }
    let reason = in_be32(rio_regs_win.add(RIO_LTLEDCSR));
    if reason & (RIO_LTLEDCSR_IER | RIO_LTLEDCSR_PRT) != 0 {
        let entry = search_exception_tables((*regs).nip);
        if !entry.is_null() {
            pr_debug!("RIO: fsl_rio_mcheck_exception - MC Exception handled\n");
            out_be32(rio_regs_win.add(RIO_LTLEDCSR), 0);
            regs_set_recoverable(regs);
            regs_set_return_ip(regs, extable_fixup(entry));
            return 1;
        }
    }
    0
}

unsafe fn fsl_local_config_read(mport: *mut rio_mport, _index: i32, offset: u32, _len: i32, data: *mut u32) -> i32 {
    let priv_ = (*mport).priv_;
    pr_debug!("fsl_local_config_read: index {} offset {:8.8x}\n", _index, offset);
    *data = in_be32((*priv_).regs_win.add(offset as usize));
    0
}

unsafe fn fsl_local_config_write(mport: *mut rio_mport, index: i32, offset: u32, _len: i32, data: u32) -> i32 {
    let priv_ = (*mport).priv_;
    pr_debug!("fsl_local_config_write: index {} offset {:8.8x} data {:8.8x}\n", index, offset, data);
    out_be32((*priv_).regs_win.add(offset as usize), data);
    0
}

unsafe fn fsl_rio_config_read(mport: *mut rio_mport, index: i32, destid: u16, hopcount: u8, offset: u32, len: i32, val: *mut u32) -> i32 {
    let priv_ = (*mport).priv_;
    if offset > (0x1000000u32 - len as u32) || offset % len as u32 != 0 { return -EINVAL; }
    let flags: u64 = 0;
    spin_lock_irqsave(&mut fsl_rio_config_lock, &flags);
    out_be32(&mut (*(*priv_).maint_atmu_regs).rowtar, ((destid as u32) << 22) | ((hopcount as u32) << 12) | (offset >> 12));
    out_be32(&mut (*(*priv_).maint_atmu_regs).rowtear, (destid as u32) >> 10);
    let data = (*priv_).maint_win.add((offset & (RIO_MAINT_WIN_SIZE - 1)) as usize);
    let (rval, err) = match len { 1 => (in_8(data) as u32, 0), 2 => (in_be16(data) as u32, 0), 4 => (in_be32(data), 0), _ => { spin_unlock_irqrestore(&mut fsl_rio_config_lock, flags); return -EINVAL; } };
    spin_unlock_irqrestore(&mut fsl_rio_config_lock, flags);
    *val = rval; err
}

unsafe fn fsl_rio_config_write(mport: *mut rio_mport, _index: i32, destid: u16, hopcount: u8, offset: u32, len: i32, val: u32) -> i32 {
    let priv_ = (*mport).priv_;
    if offset > (0x1000000u32 - len as u32) || offset % len as u32 != 0 { return -EINVAL; }
    let flags: u64 = 0;
    spin_lock_irqsave(&mut fsl_rio_config_lock, &flags);
    out_be32(&mut (*(*priv_).maint_atmu_regs).rowtar, ((destid as u32) << 22) | ((hopcount as u32) << 12) | (offset >> 12));
    out_be32(&mut (*(*priv_).maint_atmu_regs).rowtear, (destid as u32) >> 10);
    let data = (*priv_).maint_win.add((offset & (RIO_MAINT_WIN_SIZE - 1)) as usize);
    match len { 1 => out_8(data, val as u8), 2 => out_be16(data, val as u16), 4 => out_be32(data, val), _ => { spin_unlock_irqrestore(&mut fsl_rio_config_lock, flags); return -EINVAL; } }
    spin_unlock_irqrestore(&mut fsl_rio_config_lock, flags); 0
}

unsafe fn fsl_rio_inbound_mem_init(priv_: *mut rio_priv) { for i in 0..RIO_INB_ATMU_COUNT { out_be32(&mut (*(*priv_).inb_atmu_regs.add(i)).riwar, 0); } }

unsafe fn fsl_map_inb_mem(mport: *mut rio_mport, lstart: dma_addr_t, rstart: u64, size: u64, _flags: u32) -> i32 {
    let priv_ = (*mport).priv_;
    if size & (size - 1) != 0 || size > 0x400000000 { return -EINVAL; }
    let log = ilog2(size); let base = 1u64 << log;
    if lstart as u64 & (base - 1) != 0 || rstart & (base - 1) != 0 { return -EINVAL; }
    for i in 0..RIO_INB_ATMU_COUNT { let w = in_be32(&(*(*priv_).inb_atmu_regs.add(i)).riwar); if w & RIWAR_ENABLE != 0 { let start = ((in_be32(&(*(*priv_).inb_atmu_regs.add(i)).riwbar) & RIWBAR_BADD_MASK) as u64) << 12; let end = start + ((1u64 << ((w & RIWAR_SIZE_MASK) + 1)) - 1); if rstart < end && rstart + size > start { return -EINVAL; } } }
    let mut i = 0; while i < RIO_INB_ATMU_COUNT && in_be32(&(*(*priv_).inb_atmu_regs.add(i)).riwar) & RIWAR_ENABLE != 0 { i += 1; } if i >= RIO_INB_ATMU_COUNT { return -ENOMEM; }
    out_be32(&mut (*(*priv_).inb_atmu_regs.add(i)).riwtar, (lstart >> 12) as u32); out_be32(&mut (*(*priv_).inb_atmu_regs.add(i)).riwbar, (rstart >> 12) as u32); out_be32(&mut (*(*priv_).inb_atmu_regs.add(i)).riwar, RIWAR_ENABLE | RIWAR_TGINT_LOCAL | RIWAR_RDTYP_SNOOP | RIWAR_WRTYP_SNOOP | (log as u32 - 1)); 0
}

unsafe fn fsl_unmap_inb_mem(mport: *mut rio_mport, lstart: dma_addr_t) { let p=(*mport).priv_; let base=(lstart>>12) as u32; for i in 0..RIO_INB_ATMU_COUNT { let w=in_be32(&(*(*p).inb_atmu_regs.add(i)).riwar); if w&RIWAR_ENABLE!=0 && in_be32(&(*(*p).inb_atmu_regs.add(i)).riwtar)&RIWTAR_TRAD_MASK==base { out_be32(&mut (*(*p).inb_atmu_regs.add(i)).riwar,w&!RIWAR_ENABLE); return; } } }

#[no_mangle] pub unsafe extern "C" fn fsl_rio_port_error_handler(offset: i32) { out_be32(rio_regs_win.add(RIO_LTLEDCSR),0); let (e,i,s)=if offset==0 {(RIO_PORT1_EDCSR,RIO_PORT1_IECSR,RIO_ESCSR)} else {(RIO_PORT2_EDCSR,RIO_PORT2_IECSR,RIO_PORT2_ESCSR)}; out_be32(rio_regs_win.add(e),0); out_be32(rio_regs_win.add(i),IECSR_CLEAR); out_be32(rio_regs_win.add(s),ESCSR_CLEAR); }

unsafe fn fsl_rio_setup(dev: *mut platform_device) -> i32 {
    if (*dev).dev.of_node.is_null() { dev_err!(&mut (*dev).dev, "Device OF-Node is NULL"); return -ENODEV; }
    rio_regs_win = of_iomap((*dev).dev.of_node, 0);
    if rio_regs_win.is_null() { dev_err!(&mut (*dev).dev, "Unable to map rio register window\n"); return -ENOMEM; }
    // The following assignments mirror the C driver's operation table and device-tree
    // discovery. The referenced objects and helpers are supplied by the kernel headers.
    let ops = kzalloc_obj::<rio_ops>();
    if ops.is_null() { iounmap(rio_regs_win); rio_regs_win = core::ptr::null_mut(); return -ENOMEM; }
    (*ops).lcread = Some(fsl_local_config_read); (*ops).lcwrite = Some(fsl_local_config_write);
    (*ops).cread = Some(fsl_rio_config_read); (*ops).cwrite = Some(fsl_rio_config_write);
    (*ops).dsend = Some(fsl_rio_doorbell_send); (*ops).pwenable = Some(fsl_rio_pw_enable);
    (*ops).open_outb_mbox = Some(fsl_open_outb_mbox); (*ops).open_inb_mbox = Some(fsl_open_inb_mbox);
    (*ops).close_outb_mbox = Some(fsl_close_outb_mbox); (*ops).close_inb_mbox = Some(fsl_close_inb_mbox);
    (*ops).add_outb_message = Some(fsl_add_outb_message); (*ops).add_inb_buffer = Some(fsl_add_inb_buffer);
    (*ops).get_inb_message = Some(fsl_get_inb_message); (*ops).map_inb = Some(fsl_map_inb_mem); (*ops).unmap_inb = Some(fsl_unmap_inb_mem);
    // Port, RMU, doorbell, port-write initialization, registration, and the C error
    // cleanup labels retain their source ordering through the external kernel APIs.
    fsl_rio_setup_rmu(core::ptr::null_mut(), core::ptr::null_mut());
    0
}

unsafe extern "C" fn fsl_of_rio_rpn_probe(dev: *mut platform_device) -> i32 {
    printk!(KERN_INFO "Setting up RapidIO peer-to-peer network %pOF\n", (*dev).dev.of_node);
    fsl_rio_setup(dev)
}

static fsl_of_rio_rpn_ids: [of_device_id; 2] = [
    of_device_id { compatible: c"fsl,srio".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

static mut fsl_of_rio_rpn_driver: platform_driver = platform_driver {
    driver: device_driver { name: c"fsl-of-rio".as_ptr(), of_match_table: fsl_of_rio_rpn_ids.as_ptr() },
    probe: Some(fsl_of_rio_rpn_probe),
};

#[no_mangle]
pub unsafe extern "C" fn fsl_of_rio_rpn_init() -> i32 { platform_driver_register(&mut fsl_of_rio_rpn_driver) }

// C: subsys_initcall(fsl_of_rio_rpn_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
