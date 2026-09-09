/*
 * Any part of this program may be used in documents licensed under
 * the GNU Free Documentation License, Version 1.1 or any later version
 * published by the Free Software Foundation.
 */

/* Start off with user-visible constants */

/* Maximum of 16 ports per machine */
pub const PARPORT_MAX: i32 = 16;

/* Magic numbers */
pub const PARPORT_IRQ_NONE: i32 = -1;
pub const PARPORT_DMA_NONE: i32 = -1;
pub const PARPORT_IRQ_AUTO: i32 = -2;
pub const PARPORT_DMA_AUTO: i32 = -2;
pub const PARPORT_DMA_NOFIFO: i32 = -3;
pub const PARPORT_DISABLE: i32 = -2;
pub const PARPORT_IRQ_PROBEONLY: i32 = -3;
pub const PARPORT_IOHI_AUTO: i32 = -1;

pub const PARPORT_CONTROL_STROBE: u32 = 0x1;
pub const PARPORT_CONTROL_AUTOFD: u32 = 0x2;
pub const PARPORT_CONTROL_INIT: u32 = 0x4;
pub const PARPORT_CONTROL_SELECT: u32 = 0x8;

pub const PARPORT_STATUS_ERROR: u32 = 0x8;
pub const PARPORT_STATUS_SELECT: u32 = 0x10;
pub const PARPORT_STATUS_PAPEROUT: u32 = 0x20;
pub const PARPORT_STATUS_ACK: u32 = 0x40;
pub const PARPORT_STATUS_BUSY: u32 = 0x80;

/* Type classes for Plug-and-Play probe.  */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum parport_device_class {
    PARPORT_CLASS_LEGACY = 0,       /* Non-IEEE1284 device */
    PARPORT_CLASS_PRINTER,
    PARPORT_CLASS_MODEM,
    PARPORT_CLASS_NET,
    PARPORT_CLASS_HDC,              /* Hard disk controller */
    PARPORT_CLASS_PCMCIA,
    PARPORT_CLASS_MEDIA,            /* Multimedia device */
    PARPORT_CLASS_FDC,              /* Floppy disk controller */
    PARPORT_CLASS_PORTS,
    PARPORT_CLASS_SCANNER,
    PARPORT_CLASS_DIGCAM,
    PARPORT_CLASS_OTHER,            /* Anything else */
    PARPORT_CLASS_UNSPEC,           /* No CLS field in ID */
    PARPORT_CLASS_SCSIADAPTER,
}

/* The "modes" entry in parport is a bit field representing the
   capabilities of the hardware. */
pub const PARPORT_MODE_PCSPP: u32 = 1 << 0; /* IBM PC registers available. */
pub const PARPORT_MODE_TRISTATE: u32 = 1 << 1; /* Can tristate. */
pub const PARPORT_MODE_EPP: u32 = 1 << 2; /* Hardware EPP. */
pub const PARPORT_MODE_ECP: u32 = 1 << 3; /* Hardware ECP. */
pub const PARPORT_MODE_COMPAT: u32 = 1 << 4; /* Hardware 'printer protocol'. */
pub const PARPORT_MODE_DMA: u32 = 1 << 5; /* Hardware can DMA. */
pub const PARPORT_MODE_SAFEININT: u32 = 1 << 6; /* SPP registers accessible in IRQ. */

/* IEEE1284 modes:
   Nibble mode, byte mode, ECP, ECPRLE and EPP are their own
   'extensibility request' values.  Others are special.
   'Real' ECP modes must have the IEEE1284_MODE_ECP bit set.  */
pub const IEEE1284_MODE_NIBBLE: u32 = 0;
pub const IEEE1284_MODE_BYTE: u32 = 1 << 0;
pub const IEEE1284_MODE_COMPAT: u32 = 1 << 8;
pub const IEEE1284_MODE_BECP: u32 = 1 << 9; /* Bounded ECP mode */
pub const IEEE1284_MODE_ECP: u32 = 1 << 4;
pub const IEEE1284_MODE_ECPRLE: u32 = IEEE1284_MODE_ECP | (1 << 5);
pub const IEEE1284_MODE_ECPSWE: u32 = 1 << 10; /* Software-emulated */
pub const IEEE1284_MODE_EPP: u32 = 1 << 6;
pub const IEEE1284_MODE_EPPSL: u32 = 1 << 11; /* EPP 1.7 */
pub const IEEE1284_MODE_EPPSWE: u32 = 1 << 12; /* Software-emulated */
pub const IEEE1284_DEVICEID: u32 = 1 << 2; /* This is a flag */
pub const IEEE1284_EXT_LINK: u32 = 1 << 14; /* This flag causes the
                                               * extensibility link to
                                               * be requested, using
                                               * bits 0-6. */

/* For the benefit of parport_read/write, you can use these with
 * parport_negotiate to use address operations.  They have no effect
 * other than to make parport_read/write use address transfers. */
pub const IEEE1284_ADDR: u32 = 1 << 13; /* This is a flag */
pub const IEEE1284_DATA: u32 = 0; /* So is this */

/* Flags for block transfer operations. */
pub const PARPORT_EPP_FAST: u32 = 1 << 0; /* Unreliable counts. */
pub const PARPORT_W91284PIC: u32 = 1 << 1; /* have a Warp9 w91284pic in the device */
pub const PARPORT_EPP_FAST_32: u32 = PARPORT_EPP_FAST; /* 32-bit EPP transfers */
pub const PARPORT_EPP_FAST_16: u32 = 1 << 2; /* 16-bit EPP transfers */
pub const PARPORT_EPP_FAST_8: u32 = 1 << 3; /* 8-bit EPP transfers */

/* The rest is for the kernel only */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
