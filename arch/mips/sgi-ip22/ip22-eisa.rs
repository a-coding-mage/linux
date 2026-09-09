/*
 * Basic EISA bus support for the SGI Indigo-2.
 *
 * (C) 2002 Pascal Dameme <netinet@freesurf.fr>
 *	and Marc Zyngier <mzyngier@freesurf.fr>
 *
 * This code is released under both the GPL version 2 and BSD
 * licenses.  Either license may be used.
 *
 * This code offers a very basic support for this EISA bus present in
 * the SGI Indigo-2. It currently only supports PIO (forget about DMA
 * for the time being). This is enough for a low-end ethernet card,
 * but forget about your favorite SCSI card...
 *
 * TODO :
 * - Fix bugs...
 * - Add ISA support
 * - Add DMA (yeah, right...).
 * - Fix more bugs.
 */

// C headers supplied by the surrounding kernel translation unit.

const IP22_EISA_MAX_SLOTS: i32 = 4;
const EISA_MAX_IRQ: i32 = 16;

const EIU_MODE_REG: u64 = 0x0001ffc0;
const EIU_STAT_REG: u64 = 0x0001ffc4;
const EIU_PREMPT_REG: u64 = 0x0001ffc8;
const EIU_QUIET_REG: u64 = 0x0001ffcc;
const EIU_INTRPT_ACK: u64 = 0x00010004;

extern "C" {
    static mut sgimc: *mut Sgimc;
    static mut EISA_bus: i32;

    fn inb(addr: u64) -> u8;
    fn outb(value: u8, addr: u64);
    fn outl(value: u32, addr: u64);
    fn udelay(usecs: u32);
    fn do_IRQ(irq: u8);
    fn init_i8259_irqs();
    fn request_irq(irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
                   flags: u32, name: *const u8, dev_id: *mut core::ffi::c_void) -> i32;
    fn printk(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
}

#[repr(C)]
struct Sgimc {
    systemid: u32,
}

const SGIMC_SYSID_EPRESENT: u32 = 0x00000001;
const EISA_VENDOR_ID_OFFSET: u64 = 0x000000c8;
const EISA_DMA1_STATUS: u64 = 0x00000008;
const EISA_DMA2_STATUS: u64 = 0x000000d0;
const EISA_INT2_CTRL: u64 = 0x000000a0;
const EISA_INT1_CTRL: u64 = 0x00000020;
const EISA_EXT_NMI_RESET_CTRL: u64 = 0x00000061;
const EISA_DMA2_WRITE_SINGLE: u64 = 0x000000d4;
const SGI_EISA_IRQ: i32 = 6;

static mut SIG_STR: [u8; 9] = [0; 9];

unsafe fn decode_eisa_sig(addr: u64) -> *mut u8 {
    let mut sig = [0u8; 4];
    let mut rev: u16;
    let mut i = 0;

    while i < 4 {
        sig[i] = inb(addr + i as u64);
        if i == 0 && (sig[0] & 0x80) != 0 {
            return core::ptr::null_mut();
        }
        i += 1;
    }

    SIG_STR[0] = (((sig[0] >> 2) & 0x1f) + (b'A' - 1)) as u8;
    SIG_STR[1] = ((((sig[0] & 3) << 3) | (sig[1] >> 5)) + (b'A' - 1)) as u8;
    SIG_STR[2] = ((sig[1] & 0x1f) + (b'A' - 1)) as u8;
    rev = ((sig[2] as u16) << 8) | sig[3] as u16;
    let digits = *b"0123456789ABCDEF";
    let mut n = 0;
    while n < 4 {
        SIG_STR[3 + n] = digits[((rev >> (12 - 4 * n)) & 0xf) as usize];
        n += 1;
    }
    SIG_STR[7] = 0;
    SIG_STR.as_mut_ptr()
}

unsafe extern "C" fn ip22_eisa_intr(_irq: i32, _dev_id: *mut core::ffi::c_void) -> i32 {
    let eisa_irq = inb(EIU_INTRPT_ACK);

    inb(EISA_DMA1_STATUS);
    inb(EISA_DMA2_STATUS);

    if eisa_irq < EISA_MAX_IRQ as u8 {
        do_IRQ(eisa_irq);
        return 1; // IRQ_HANDLED
    }

    printk(b"eisa_irq %d out of bound\n\0".as_ptr(), eisa_irq as i32);
    outb(0x20, EISA_INT2_CTRL);
    outb(0x20, EISA_INT1_CTRL);
    0 // IRQ_NONE
}

pub unsafe fn ip22_eisa_init() -> i32 {
    let mut i: i32;
    let mut c = 0;

    if ((*sgimc).systemid & SGIMC_SYSID_EPRESENT) == 0 {
        printk(b"EISA: bus not present.\n\0".as_ptr());
        return 1;
    }

    printk(b"EISA: Probing bus...\n\0".as_ptr());
    i = 1;
    while i <= IP22_EISA_MAX_SLOTS {
        let str_ptr = decode_eisa_sig(0x1000 * i as u64 + EISA_VENDOR_ID_OFFSET);
        if !str_ptr.is_null() {
            printk(b"EISA: slot %d : %s detected.\n\0".as_ptr(), i, str_ptr);
            c += 1;
        }
        i += 1;
    }
    printk(b"EISA: Detected %d card%s.\n\0".as_ptr(), c,
           if c < 2 { b"\0".as_ptr() } else { b"s\0".as_ptr() });

    /* CONFIG_ISA: ISA support compiled in. */

    /* Warning : BlackMagicAhead(tm).
       Please wave your favorite dead chicken over the busses */
    outl(0x0000ffff, EIU_PREMPT_REG);
    outl(1, EIU_QUIET_REG);
    outl(0x40f3c07f, EIU_MODE_REG);

    outb(1, EISA_EXT_NMI_RESET_CTRL);
    udelay(50);
    outb(0, EISA_EXT_NMI_RESET_CTRL);
    outb(0, EISA_DMA2_WRITE_SINGLE);

    init_i8259_irqs();
    if request_irq(SGI_EISA_IRQ, ip22_eisa_intr, 0, b"EISA\0".as_ptr(), core::ptr::null_mut()) != 0 {
        pr_err(b"Failed to request irq %d (EISA)\n\0".as_ptr(), SGI_EISA_IRQ);
    }

    EISA_bus = 1;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
