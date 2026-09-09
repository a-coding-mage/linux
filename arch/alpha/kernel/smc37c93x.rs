// SPDX-License-Identifier: GPL-2.0
/*
 * SMC 37C93X initialization code
 */

// Linux kernel, architecture, and local header dependencies from the C source.

const SMC_DEBUG: bool = false;

const KB: usize = 1024;
const MB: usize = 1024 * KB;
const GB: usize = 1024 * MB;

/* device "activate" register contents */
const DEVICE_ON: u64 = 1;
const DEVICE_OFF: u64 = 0;

/* configuration on/off keys */
const CONFIG_ON_KEY: u8 = 0x55;
const CONFIG_OFF_KEY: u8 = 0xaa;

/* configuration space device definitions */
const FDC: u64 = 0;
const IDE1: u64 = 1;
const IDE2: u64 = 2;
const PARP: u64 = 3;
const SER1: u64 = 4;
const SER2: u64 = 5;
const RTCL: u64 = 6;
const KYBD: u64 = 7;
const AUXIO: u64 = 8;

/* Chip register offsets from base */
const CONFIG_CONTROL: u64 = 0x02;
const INDEX_ADDRESS: u64 = 0x03;
const LOGICAL_DEVICE_NUMBER: u64 = 0x07;
const DEVICE_ID: u64 = 0x20;
const DEVICE_REV: u64 = 0x21;
const POWER_CONTROL: u64 = 0x22;
const POWER_MGMT: u64 = 0x23;
const OSC: u64 = 0x24;
const ACTIVATE: u64 = 0x30;
const ADDR_HI: u64 = 0x60;
const ADDR_LO: u64 = 0x61;
const INTERRUPT_SEL: u64 = 0x70;
const INTERRUPT_SEL_2: u64 = 0x72;
const DMA_CHANNEL_SEL: u64 = 0x74;
const FDD_MODE_REGISTER: u64 = 0x90;
const FDD_OPTION_REGISTER: u64 = 0x91;

const VALID_DEVICE_ID: u8 = 2;
const KYBD_INTERRUPT: u64 = 1;
const MOUS_INTERRUPT: u64 = 12;
const COM2_BASE: u64 = 0x2f8;
const COM2_INTERRUPT: u64 = 3;
const COM1_BASE: u64 = 0x3f8;
const COM1_INTERRUPT: u64 = 4;
const PARP_BASE: u64 = 0x3bc;
const PARP_INTERRUPT: u64 = 7;

extern "C" {
    fn outb(value: u8, port: u64);
    fn inb(port: u64) -> u8;
    fn udelay(usecs: u64);
    fn local_irq_save(flags: *mut u64);
    fn local_irq_restore(flags: u64);
    fn printk(fmt: *const i8, ...);
}

unsafe fn smc_config_state(base_addr: u64) -> u64 {
    let (config_port, index_port) = (base_addr, base_addr);
    let data_port = config_port + 1;
    let mut dev_id: u8;
    let mut i = 0;

    while i < 5 {
        outb(CONFIG_ON_KEY, config_port);
        outb(CONFIG_ON_KEY, config_port);
        outb(DEVICE_ID as u8, index_port);
        dev_id = inb(data_port);
        if dev_id == VALID_DEVICE_ID {
            outb(DEVICE_REV as u8, index_port);
            let _ = inb(data_port);
            break;
        } else {
            udelay(100);
        }
        i += 1;
    }
    if i != 5 { base_addr } else { 0 }
}

unsafe fn smc_run_state(base_addr: u64) { outb(CONFIG_OFF_KEY, base_addr); }

unsafe fn smc_detect_ultra_io() -> u64 {
    let mut base_addr = 0x3f0;
    base_addr = smc_config_state(base_addr);
    if base_addr == 0x3f0 { return base_addr; }
    base_addr = 0x370;
    base_addr = smc_config_state(base_addr);
    if base_addr == 0x370 { return base_addr; }
    0
}

unsafe fn smc_enable_device(base_addr: u64, device: u64, portaddr: u64, interrupt: u64) {
    let index_port = base_addr;
    let data_port = base_addr + 1;
    outb(LOGICAL_DEVICE_NUMBER as u8, index_port); outb(device as u8, data_port);
    outb(ADDR_LO as u8, index_port); outb((portaddr & 0xff) as u8, data_port);
    outb(ADDR_HI as u8, index_port); outb(((portaddr >> 8) & 0xff) as u8, data_port);
    outb(INTERRUPT_SEL as u8, index_port); outb(interrupt as u8, data_port);
    outb(ACTIVATE as u8, index_port); outb(DEVICE_ON as u8, data_port);
}

unsafe fn smc_enable_kybd(base_addr: u64) {
    let index_port = base_addr; let data_port = base_addr + 1;
    outb(LOGICAL_DEVICE_NUMBER as u8, index_port); outb(KYBD as u8, data_port);
    outb(INTERRUPT_SEL as u8, index_port); outb(KYBD_INTERRUPT as u8, data_port);
    outb(INTERRUPT_SEL_2 as u8, index_port); outb(MOUS_INTERRUPT as u8, data_port);
    outb(ACTIVATE as u8, index_port); outb(DEVICE_ON as u8, data_port);
}

unsafe fn smc_enable_fdc(base_addr: u64) {
    let index_port = base_addr; let data_port = base_addr + 1;
    outb(LOGICAL_DEVICE_NUMBER as u8, index_port); outb(FDC as u8, data_port);
    outb(FDD_MODE_REGISTER as u8, index_port);
    let old_value = inb(data_port) | 0x0e;
    outb(old_value, data_port);
    outb(INTERRUPT_SEL as u8, index_port); outb(0x06, data_port);
    outb(DMA_CHANNEL_SEL as u8, index_port); outb(0x02, data_port);
    outb(ACTIVATE as u8, index_port); outb(DEVICE_ON as u8, data_port);
}

pub unsafe fn smc93x_init() -> i32 {
    let mut smc_ultra_base: u64;
    let mut flags = 0u64;
    local_irq_save(&mut flags);
    smc_ultra_base = smc_detect_ultra_io();
    if smc_ultra_base != 0 {
        smc_enable_device(smc_ultra_base, SER1, COM1_BASE, COM1_INTERRUPT);
        smc_enable_device(smc_ultra_base, SER2, COM2_BASE, COM2_INTERRUPT);
        smc_enable_device(smc_ultra_base, PARP, PARP_BASE, PARP_INTERRUPT);
        smc_enable_kybd(smc_ultra_base);
        smc_enable_fdc(smc_ultra_base);
        smc_run_state(smc_ultra_base);
        local_irq_restore(flags);
        printk(b"SMC FDC37C93X Ultra I/O Controller found @ 0x%lx\0".as_ptr() as *const i8, smc_ultra_base);
        1
    } else {
        local_irq_restore(flags);
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
