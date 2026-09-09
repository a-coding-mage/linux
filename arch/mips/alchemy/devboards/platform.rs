// SPDX-License-Identifier: GPL-2.0
/*
 * devoard misc stuff.
 */

// C headers supplying the following kernel, platform, MTD, Alchemy, and
// board-control symbols are intentionally external dependencies.

pub unsafe fn prom_putchar(c: core::ffi::c_char) {
    if alchemy_get_cputype() == ALCHEMY_CPU_AU1300 {
        alchemy_uart_putchar(AU1300_UART2_PHYS_ADDR, c);
    } else {
        alchemy_uart_putchar(AU1000_UART0_PHYS_ADDR, c);
    }
}

static mut db1x00_rtc_dev: platform_device = platform_device {
    name: b"rtc-au1xxx\0".as_ptr() as *const core::ffi::c_char,
    id: -1,
};

unsafe fn db1x_power_off() -> ! {
    bcsr_write(BCSR_RESETS, 0);
    bcsr_write(BCSR_SYSTEM, BCSR_SYSTEM_PWROFF | BCSR_SYSTEM_RESET);
    loop {
        // sit and spin
        cpu_wait();
    }
}

unsafe fn db1x_reset(_c: *mut core::ffi::c_char) {
    bcsr_write(BCSR_RESETS, 0);
    bcsr_write(BCSR_SYSTEM, 0);
}

unsafe fn db1x_late_setup() -> core::ffi::c_int {
    if pm_power_off.is_none() {
        pm_power_off = Some(db1x_power_off);
    }
    if _machine_halt.is_none() {
        _machine_halt = Some(db1x_power_off);
    }
    if _machine_restart.is_none() {
        _machine_restart = Some(db1x_reset);
    }

    platform_device_register(&mut db1x00_rtc_dev);

    0
}

// device_initcall(db1x_late_setup);

/* register a pcmcia socket */
pub unsafe fn db1x_register_pcmcia_socket(
    pcmcia_attr_start: phys_addr_t,
    pcmcia_attr_end: phys_addr_t,
    pcmcia_mem_start: phys_addr_t,
    pcmcia_mem_end: phys_addr_t,
    pcmcia_io_start: phys_addr_t,
    pcmcia_io_end: phys_addr_t,
    card_irq: core::ffi::c_int,
    cd_irq: core::ffi::c_int,
    stschg_irq: core::ffi::c_int,
    eject_irq: core::ffi::c_int,
    id: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut cnt: usize = 5;
    let mut i: usize;
    let mut ret: core::ffi::c_int;
    let sr: *mut resource;
    let pd: *mut platform_device;

    if eject_irq != 0 {
        cnt += 1;
    }
    if stschg_irq != 0 {
        cnt += 1;
    }

    sr = kzalloc_objs_resource(cnt);
    if sr.is_null() {
        return -ENOMEM;
    }

    pd = platform_device_alloc(b"db1xxx_pcmcia\0".as_ptr() as *const core::ffi::c_char, id);
    if pd.is_null() {
        ret = -ENOMEM;
        kfree(sr as *mut core::ffi::c_void);
        return ret;
    }

    (*sr.add(0)).name = b"pcmcia-attr\0".as_ptr() as *const core::ffi::c_char;
    (*sr.add(0)).flags = IORESOURCE_MEM;
    (*sr.add(0)).start = pcmcia_attr_start;
    (*sr.add(0)).end = pcmcia_attr_end;

    (*sr.add(1)).name = b"pcmcia-mem\0".as_ptr() as *const core::ffi::c_char;
    (*sr.add(1)).flags = IORESOURCE_MEM;
    (*sr.add(1)).start = pcmcia_mem_start;
    (*sr.add(1)).end = pcmcia_mem_end;

    (*sr.add(2)).name = b"pcmcia-io\0".as_ptr() as *const core::ffi::c_char;
    (*sr.add(2)).flags = IORESOURCE_MEM;
    (*sr.add(2)).start = pcmcia_io_start;
    (*sr.add(2)).end = pcmcia_io_end;

    (*sr.add(3)).name = b"insert\0".as_ptr() as *const core::ffi::c_char;
    (*sr.add(3)).flags = IORESOURCE_IRQ;
    (*sr.add(3)).start = cd_irq as _;
    (*sr.add(3)).end = cd_irq as _;

    (*sr.add(4)).name = b"card\0".as_ptr() as *const core::ffi::c_char;
    (*sr.add(4)).flags = IORESOURCE_IRQ;
    (*sr.add(4)).start = card_irq as _;
    (*sr.add(4)).end = card_irq as _;

    i = 5;
    if stschg_irq != 0 {
        (*sr.add(i)).name = b"stschg\0".as_ptr() as *const core::ffi::c_char;
        (*sr.add(i)).flags = IORESOURCE_IRQ;
        (*sr.add(i)).start = stschg_irq as _;
        (*sr.add(i)).end = stschg_irq as _;
        i += 1;
    }
    if eject_irq != 0 {
        (*sr.add(i)).name = b"eject\0".as_ptr() as *const core::ffi::c_char;
        (*sr.add(i)).flags = IORESOURCE_IRQ;
        (*sr.add(i)).start = eject_irq as _;
        (*sr.add(i)).end = eject_irq as _;
    }

    (*pd).resource = sr;
    (*pd).num_resources = cnt;

    ret = platform_device_add(pd);
    if ret == 0 {
        return 0;
    }

    platform_device_put(pd);
    kfree(sr as *mut core::ffi::c_void);
    return ret;
}

const YAMON_SIZE: usize = 0x00100000;
const YAMON_ENV_SIZE: usize = 0x00040000;

pub unsafe fn db1x_register_norflash(
    size: usize,
    width: core::ffi::c_int,
    swapped: core::ffi::c_int,
) -> core::ffi::c_int {
    let pfd: *mut physmap_flash_data;
    let pd: *mut platform_device;
    let parts: *mut mtd_partition;
    let res: *mut resource;
    let mut ret: core::ffi::c_int;
    let mut i: usize;

    if size < 8 * 1024 * 1024 {
        return -EINVAL;
    }

    ret = -ENOMEM;
    parts = kzalloc_objs_mtd_partition(5);
    if parts.is_null() { return ret; }
    res = kzalloc_obj_resource();
    if res.is_null() { kfree(parts as *mut core::ffi::c_void); return ret; }
    pfd = kzalloc_obj_physmap_flash_data();
    if pfd.is_null() { kfree(res as *mut core::ffi::c_void); kfree(parts as *mut core::ffi::c_void); return ret; }
    pd = platform_device_alloc(b"physmap-flash\0".as_ptr() as *const core::ffi::c_char, 0);
    if pd.is_null() { kfree(pfd as *mut core::ffi::c_void); kfree(res as *mut core::ffi::c_void); kfree(parts as *mut core::ffi::c_void); return ret; }

    // NOR flash ends at 0x20000000, regardless of size
    (*res).start = 0x20000000usize - size;
    (*res).end = 0x20000000usize - 1;
    (*res).flags = IORESOURCE_MEM;

    /* partition setup.  Most Develboards have a switch which allows
     * to swap the physical locations of the 2 NOR flash banks.
     */
    i = 0;
    if swapped == 0 {
        // first NOR chip
        (*parts.add(i)).offset = 0;
        (*parts.add(i)).name = b"User FS\0".as_ptr() as *const core::ffi::c_char;
        (*parts.add(i)).size = size / 2;
        i += 1;
    }
    (*parts.add(i)).offset = MTDPART_OFS_APPEND;
    (*parts.add(i)).name = b"User FS 2\0".as_ptr() as *const core::ffi::c_char;
    (*parts.add(i)).size = (size / 2) - (0x20000000usize - 0x1fc00000usize);
    i += 1;
    (*parts.add(i)).offset = MTDPART_OFS_APPEND;
    (*parts.add(i)).name = b"YAMON\0".as_ptr() as *const core::ffi::c_char;
    (*parts.add(i)).size = YAMON_SIZE;
    (*parts.add(i)).mask_flags = MTD_WRITEABLE;
    i += 1;
    (*parts.add(i)).offset = MTDPART_OFS_APPEND;
    (*parts.add(i)).name = b"raw kernel\0".as_ptr() as *const core::ffi::c_char;
    (*parts.add(i)).size = 0x00400000usize - YAMON_SIZE - YAMON_ENV_SIZE;
    i += 1;
    (*parts.add(i)).offset = MTDPART_OFS_APPEND;
    (*parts.add(i)).name = b"YAMON Env\0".as_ptr() as *const core::ffi::c_char;
    (*parts.add(i)).size = YAMON_ENV_SIZE;
    (*parts.add(i)).mask_flags = MTD_WRITEABLE;
    i += 1;
    if swapped != 0 {
        (*parts.add(i)).offset = MTDPART_OFS_APPEND;
        (*parts.add(i)).name = b"User FS\0".as_ptr() as *const core::ffi::c_char;
        (*parts.add(i)).size = size / 2;
        i += 1;
    }
    (*pfd).width = width;
    (*pfd).parts = parts;
    (*pfd).nr_parts = 5;
    (*pd).dev.platform_data = pfd as *mut core::ffi::c_void;
    (*pd).resource = res;
    (*pd).num_resources = 1;
    ret = platform_device_add(pd);
    if ret == 0 { return ret; }
    platform_device_put(pd);
    kfree(pfd as *mut core::ffi::c_void);
    kfree(res as *mut core::ffi::c_void);
    kfree(parts as *mut core::ffi::c_void);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
