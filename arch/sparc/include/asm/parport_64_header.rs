/* SPDX-License-Identifier: GPL-2.0 */
/* parport.h: sparc64 specific parport initialization and dma.
 *
 * Copyright (C) 1999  Eddie C. Dost  (ecd@skynet.be)
 */

// External Linux/kernel declarations and build-time configuration are supplied
// by the surrounding translation unit.

pub const PARPORT_PC_MAX_PORTS: usize = PARPORT_MAX as usize;

pub const HAS_DMA: bool = true;

#[cfg(feature = "CONFIG_PARPORT_PC_FIFO")]
static mut dma_spin_lock: DEFINE_SPINLOCK_TYPE = DEFINE_SPINLOCK!();

#[cfg(feature = "CONFIG_PARPORT_PC_FIFO")]
#[inline]
unsafe fn claim_dma_lock() -> c_ulong {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&raw mut dma_spin_lock, &mut flags);
    flags
}

#[cfg(feature = "CONFIG_PARPORT_PC_FIFO")]
#[inline]
unsafe fn release_dma_lock(flags: c_ulong) {
    spin_unlock_irqrestore(&raw mut dma_spin_lock, flags);
}

#[repr(C)]
struct sparc_ebus_info {
    info: ebus_dma_info,
    addr: c_uint,
    count: c_uint,
    lock: c_int,
    port: *mut parport,
}

static mut sparc_ebus_dmas: [sparc_ebus_info; PARPORT_PC_MAX_PORTS] =
    [sparc_ebus_info {
        info: unsafe { core::mem::zeroed() },
        addr: 0,
        count: 0,
        lock: 0,
        port: core::ptr::null_mut(),
    }; PARPORT_PC_MAX_PORTS];

static mut dma_slot_map: DECLARE_BITMAP_TYPE = DECLARE_BITMAP!(PARPORT_PC_MAX_PORTS);

#[inline]
unsafe fn request_dma(dmanr: c_uint, _device_id: *const c_char) -> c_int {
    if dmanr >= PARPORT_PC_MAX_PORTS as c_uint { return -EINVAL; }
    if xchg(&raw mut sparc_ebus_dmas[dmanr as usize].lock, 1) != 0 { return -EBUSY; }
    0
}

#[inline]
unsafe fn free_dma(dmanr: c_uint) {
    if dmanr >= PARPORT_PC_MAX_PORTS as c_uint {
        printk(KERN_WARNING, "Trying to free DMA%d\n", dmanr);
        return;
    }
    if xchg(&raw mut sparc_ebus_dmas[dmanr as usize].lock, 0) == 0 {
        printk(KERN_WARNING, "Trying to free free DMA%d\n", dmanr);
    }
}

#[inline]
unsafe fn enable_dma(dmanr: c_uint) {
    ebus_dma_enable(&raw mut sparc_ebus_dmas[dmanr as usize].info, 1);
    if ebus_dma_request(&raw mut sparc_ebus_dmas[dmanr as usize].info,
                        sparc_ebus_dmas[dmanr as usize].addr,
                        sparc_ebus_dmas[dmanr as usize].count) != 0 { BUG!(); }
}

#[inline]
unsafe fn disable_dma(dmanr: c_uint) {
    ebus_dma_enable(&raw mut sparc_ebus_dmas[dmanr as usize].info, 0);
}

#[inline]
unsafe fn clear_dma_ff(_dmanr: c_uint) { /* nothing */ }

#[inline]
unsafe fn set_dma_mode(dmanr: c_uint, mode: c_char) {
    ebus_dma_prepare(&raw mut sparc_ebus_dmas[dmanr as usize].info, mode != DMA_MODE_WRITE);
}

#[inline]
unsafe fn set_dma_addr(dmanr: c_uint, addr: c_uint) { sparc_ebus_dmas[dmanr as usize].addr = addr; }

#[inline]
unsafe fn set_dma_count(dmanr: c_uint, count: c_uint) { sparc_ebus_dmas[dmanr as usize].count = count; }

#[inline]
unsafe fn get_dma_residue(dmanr: c_uint) -> c_uint {
    ebus_dma_residue(&raw mut sparc_ebus_dmas[dmanr as usize].info)
}

unsafe fn ecpp_probe(op: *mut platform_device) -> c_int {
    let base = (*op).resource[0].start;
    let config = (*op).resource[1].start;
    let d_base = (*op).resource[2].start;
    let mut d_len: c_ulong;
    let parent = (*(*op).dev.of_node).parent;
    let mut p: *mut parport;
    let mut slot: c_int;
    let mut err: c_int;
    if of_node_name_eq(parent, c_str!("dma")) {
        p = parport_pc_probe_port(base, base + 0x400, (*op).archdata.irqs[0], PARPORT_DMA_NOFIFO,
                                  (*(*op).dev.parent).parent, 0);
        if p.is_null() { return -ENOMEM; }
        dev_set_drvdata(&raw mut (*op).dev, p);
        return 0;
    }
    slot = 0;
    while slot < PARPORT_PC_MAX_PORTS as c_int {
        if test_and_set_bit(slot as usize, &raw mut dma_slot_map) == 0 { break; }
        slot += 1;
    }
    err = -ENODEV;
    if slot >= PARPORT_PC_MAX_PORTS as c_int { return err; }
    spin_lock_init(&raw mut sparc_ebus_dmas[slot as usize].info.lock);
    d_len = ((*op).resource[2].end - d_base) + 1;
    sparc_ebus_dmas[slot as usize].info.regs = of_ioremap(&(*op).resource[2], 0, d_len, c_str!("ECPP DMA"));
    if sparc_ebus_dmas[slot as usize].info.regs.is_null() { clear_bit(slot as usize, &raw mut dma_slot_map); return err; }
    sparc_ebus_dmas[slot as usize].info.flags = 0;
    sparc_ebus_dmas[slot as usize].info.callback = None;
    sparc_ebus_dmas[slot as usize].info.client_cookie = core::ptr::null_mut();
    sparc_ebus_dmas[slot as usize].info.irq = 0xdeadbeef;
    strscpy(sparc_ebus_dmas[slot as usize].info.name.as_mut_ptr(), c_str!("parport"));
    if ebus_dma_register(&raw mut sparc_ebus_dmas[slot as usize].info) != 0 { of_iounmap(&(*op).resource[2], sparc_ebus_dmas[slot as usize].info.regs, d_len); clear_bit(slot as usize, &raw mut dma_slot_map); return err; }
    ebus_dma_irq_enable(&raw mut sparc_ebus_dmas[slot as usize].info, 1);
    outb(0x04, base + 0x02);
    ns87303_modify(config, PCR, PCR_EPP_ENABLE | PCR_IRQ_ODRAIN, PCR_ECP_ENABLE | PCR_ECP_CLK_ENA | PCR_IRQ_POLAR);
    ns87303_modify(config, PTR, 0, PTR_LPT_REG_DIR);
    p = parport_pc_probe_port(base, base + 0x400, (*op).archdata.irqs[0], slot, (*op).dev.parent, 0);
    err = -ENOMEM;
    if p.is_null() { ebus_dma_irq_enable(&raw mut sparc_ebus_dmas[slot as usize].info, 0); ebus_dma_unregister(&raw mut sparc_ebus_dmas[slot as usize].info); of_iounmap(&(*op).resource[2], sparc_ebus_dmas[slot as usize].info.regs, d_len); clear_bit(slot as usize, &raw mut dma_slot_map); return err; }
    dev_set_drvdata(&raw mut (*op).dev, p);
    0
}

unsafe fn ecpp_remove(op: *mut platform_device) {
    let p = dev_get_drvdata(&(*op).dev);
    let slot = (*p).dma;
    parport_pc_unregister_port(p);
    if slot != PARPORT_DMA_NOFIFO { let d_base = (*op).resource[2].start; let d_len = ((*op).resource[2].end - d_base) + 1; ebus_dma_irq_enable(&raw mut sparc_ebus_dmas[slot as usize].info, 0); ebus_dma_unregister(&raw mut sparc_ebus_dmas[slot as usize].info); of_iounmap(&(*op).resource[2], sparc_ebus_dmas[slot as usize].info.regs, d_len); clear_bit(slot as usize, &raw mut dma_slot_map); }
}

static ecpp_match: [of_device_id; 5] = [
    of_device_id { name: c_str!("ecpp"), ..unsafe { core::mem::zeroed() } },
    of_device_id { name: c_str!("parallel"), compatible: c_str!("ecpp"), ..unsafe { core::mem::zeroed() } },
    of_device_id { name: c_str!("parallel"), compatible: c_str!("ns87317-ecpp"), ..unsafe { core::mem::zeroed() } },
    of_device_id { name: c_str!("parallel"), compatible: c_str!("pnpALI,1533,3"), ..unsafe { core::mem::zeroed() } },
    unsafe { core::mem::zeroed() },
];

static mut ecpp_driver: platform_driver = platform_driver { driver: driver { name: c_str!("ecpp"), of_match_table: &raw const ecpp_match, ..unsafe { core::mem::zeroed() } }, probe: Some(ecpp_probe), remove: Some(ecpp_remove), ..unsafe { core::mem::zeroed() } };

unsafe fn parport_pc_find_nonpci_ports(_autoirq: c_int, _autodma: c_int) -> c_int {
    platform_driver_register(&raw mut ecpp_driver)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
