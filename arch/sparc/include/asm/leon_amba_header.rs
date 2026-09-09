/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Translation of leon_amba.h.
 */

#[repr(C)]
pub struct amba_prom_registers {
    pub phys_addr: ::core::ffi::c_uint, /* The physical address of this register */
    pub reg_size: ::core::ffi::c_uint,  /* How many bytes does this register take up? */
}

pub const LEON_REG_UART_STATUS_DR: u32 = 0x00000001; /* Data Ready */
pub const LEON_REG_UART_STATUS_TSE: u32 = 0x00000002; /* TX Send Register Empty */
pub const LEON_REG_UART_STATUS_THE: u32 = 0x00000004; /* TX Hold Register Empty */
pub const LEON_REG_UART_STATUS_BR: u32 = 0x00000008; /* Break Error */
pub const LEON_REG_UART_STATUS_OE: u32 = 0x00000010; /* RX Overrun Error */
pub const LEON_REG_UART_STATUS_PE: u32 = 0x00000020; /* RX Parity Error */
pub const LEON_REG_UART_STATUS_FE: u32 = 0x00000040; /* RX Framing Error */
pub const LEON_REG_UART_STATUS_ERR: u32 = 0x00000078; /* Error Mask */

pub const LEON_REG_UART_CTRL_RE: u32 = 0x00000001; /* Receiver enable */
pub const LEON_REG_UART_CTRL_TE: u32 = 0x00000002; /* Transmitter enable */
pub const LEON_REG_UART_CTRL_RI: u32 = 0x00000004; /* Receiver interrupt enable */
pub const LEON_REG_UART_CTRL_TI: u32 = 0x00000008; /* Transmitter irq */
pub const LEON_REG_UART_CTRL_PS: u32 = 0x00000010; /* Parity select */
pub const LEON_REG_UART_CTRL_PE: u32 = 0x00000020; /* Parity enable */
pub const LEON_REG_UART_CTRL_FL: u32 = 0x00000040; /* Flow control enable */
pub const LEON_REG_UART_CTRL_LB: u32 = 0x00000080; /* Loop Back enable */

pub const LEON3_GPTIMER_EN: u32 = 1;
pub const LEON3_GPTIMER_RL: u32 = 2;
pub const LEON3_GPTIMER_LD: u32 = 4;
pub const LEON3_GPTIMER_IRQEN: u32 = 8;
pub const LEON3_GPTIMER_SEPIRQ: u32 = 8;
pub const LEON3_GPTIMER_TIMERS: u32 = 0x7;
pub const LEON23_REG_TIMER_CONTROL_EN: u32 = 0x00000001;
pub const LEON23_REG_TIMER_CONTROL_RL: u32 = 0x00000002;
pub const LEON23_REG_TIMER_CONTROL_LD: u32 = 0x00000004;
pub const LEON23_REG_TIMER_CONTROL_IQ: u32 = 0x00000008;

pub const LEON_REG_PS2_STATUS_DR: u32 = 0x00000001;
pub const LEON_REG_PS2_STATUS_PE: u32 = 0x00000002;
pub const LEON_REG_PS2_STATUS_FE: u32 = 0x00000004;
pub const LEON_REG_PS2_STATUS_KI: u32 = 0x00000008;
pub const LEON_REG_PS2_STATUS_RF: u32 = 0x00000010;
pub const LEON_REG_PS2_STATUS_TF: u32 = 0x00000020;
pub const LEON_REG_PS2_CTRL_RE: u32 = 0x00000001;
pub const LEON_REG_PS2_CTRL_TE: u32 = 0x00000002;
pub const LEON_REG_PS2_CTRL_RI: u32 = 0x00000004;
pub const LEON_REG_PS2_CTRL_TI: u32 = 0x00000008;

pub const LEON3_IRQMPSTATUS_CPUNR: u32 = 28;
pub const LEON3_IRQMPSTATUS_BROADCAST: u32 = 27;

#[inline]
pub const fn GPTIMER_CONFIG_IRQNT(a: u32) -> u32 { (a >> 3) & 0x1f }
#[inline]
pub const fn GPTIMER_CONFIG_ISSEP(a: u32) -> u32 { a & (1 << 8) }
#[inline]
pub const fn GPTIMER_CONFIG_NTIMERS(a: u32) -> u32 { a & 0x7 }
pub const LEON3_GPTIMER_CTRL_PENDING: u32 = 0x10;
#[inline]
pub unsafe fn LEON3_GPTIMER_CONFIG_NRTIMERS(c: *const leon3_gptimer_regs_map) -> u32 {
    (*c).config & 0x7
}
#[inline]
pub const fn LEON3_GPTIMER_CTRL_ISPENDING(r: u32) -> u32 {
    if (r & LEON3_GPTIMER_CTRL_PENDING) != 0 { 1 } else { 0 }
}

#[repr(C)]
pub struct leon3_irqctrl_regs_map {
    pub ilevel: u32, pub ipend: u32, pub iforce: u32, pub iclear: u32,
    pub mpstatus: u32, pub mpbroadcast: u32, pub notused02: u32, pub notused03: u32,
    pub ampctrl: u32, pub icsel: [u32; 2], pub notused13: u32,
    pub notused20: u32, pub notused21: u32, pub notused22: u32, pub notused23: u32,
    pub mask: [u32; 16], pub force: [u32; 16],
    pub intid: [u32; 16], /* 0xc0 */
    pub unused: [u32; (0x1000 - 0x100) / 4],
}

#[repr(C)]
pub struct leon3_apbuart_regs_map { pub data: u32, pub status: u32, pub ctrl: u32, pub scaler: u32 }

#[repr(C)]
pub struct leon3_gptimerelem_regs_map { pub val: u32, pub rld: u32, pub ctrl: u32, pub unused: u32 }

#[repr(C)]
pub struct leon3_gptimer_regs_map {
    pub scalar: u32, pub scalar_reload: u32, pub config: u32, pub unused: u32,
    pub e: [leon3_gptimerelem_regs_map; 8],
}

pub const AMBA_MAXAPB_DEVS: usize = 64;
pub const AMBA_MAXAPB_DEVS_PERBUS: usize = 16;

#[repr(C)]
pub struct amba_device_table {
    pub devnr: ::core::ffi::c_int,
    pub addr: [*mut ::core::ffi::c_uint; 16],
    pub allocbits: [::core::ffi::c_uint; 1],
}

#[repr(C)]
pub struct amba_apbslv_device_table {
    pub devnr: ::core::ffi::c_int,
    pub addr: [*mut ::core::ffi::c_uint; AMBA_MAXAPB_DEVS],
    pub apbmst: [::core::ffi::c_uint; AMBA_MAXAPB_DEVS],
    pub apbmstidx: [::core::ffi::c_uint; AMBA_MAXAPB_DEVS],
    pub allocbits: [::core::ffi::c_uint; 4],
}

#[repr(C)]
pub struct amba_confarea_type {
    pub next: *mut amba_confarea_type,
    pub ahbmst: amba_device_table,
    pub ahbslv: amba_device_table,
    pub apbslv: amba_apbslv_device_table,
    pub apbmst: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct amba_apb_device {
    pub start: ::core::ffi::c_uint, pub irq: ::core::ffi::c_uint, pub bus_id: ::core::ffi::c_uint,
    pub bus: *mut amba_confarea_type,
}

#[repr(C)]
pub struct amba_ahb_device {
    pub start: [::core::ffi::c_uint; 4], pub irq: ::core::ffi::c_uint, pub bus_id: ::core::ffi::c_uint,
    pub bus: *mut amba_confarea_type,
}

pub struct device_node;
unsafe extern "C" { pub fn _amba_init(dp: *mut device_node, nextp: *mut *mut *mut device_node); }
unsafe extern "C" {
    pub static mut amba_system_id: ::core::ffi::c_ulong;
    pub static mut leon3_irqctrl_regs: *mut leon3_irqctrl_regs_map;
    pub static mut leon3_gptimer_regs: *mut leon3_gptimer_regs_map;
    pub static mut leon_percpu_timer_dev: [amba_apb_device; 16];
    pub static mut leondebug_irq_disable: ::core::ffi::c_int;
    pub static mut leon_debug_irqout: ::core::ffi::c_int;
    pub static mut leon3_gptimer_irq: ::core::ffi::c_ulong;
    pub static mut sparc_leon_eirq: ::core::ffi::c_uint;
}

pub const LEON3_IO_AREA: u32 = 0xfff00000;
pub const LEON3_CONF_AREA: u32 = 0xff000;
pub const LEON3_AHB_SLAVE_CONF_AREA: u32 = 1 << 11;
pub const LEON3_AHB_CONF_WORDS: u32 = 8;
pub const LEON3_APB_CONF_WORDS: u32 = 2;
pub const LEON3_AHB_MASTERS: u32 = 16;
pub const LEON3_AHB_SLAVES: u32 = 16;
pub const LEON3_APB_SLAVES: u32 = 16;
pub const LEON3_APBUARTS: u32 = 8;

pub const VENDOR_GAISLER: u32 = 1;
pub const VENDOR_PENDER: u32 = 2;
pub const VENDOR_ESA: u32 = 4;
pub const VENDOR_OPENCORES: u32 = 8;

pub const GAISLER_LEON3: u32 = 0x003; pub const GAISLER_LEON3DSU: u32 = 0x004;
pub const GAISLER_ETHAHB: u32 = 0x005; pub const GAISLER_APBMST: u32 = 0x006;
pub const GAISLER_AHBUART: u32 = 0x007; pub const GAISLER_SRCTRL: u32 = 0x008;
pub const GAISLER_SDCTRL: u32 = 0x009; pub const GAISLER_APBUART: u32 = 0x00C;
pub const GAISLER_IRQMP: u32 = 0x00D; pub const GAISLER_AHBRAM: u32 = 0x00E;
pub const GAISLER_GPTIMER: u32 = 0x011; pub const GAISLER_PCITRG: u32 = 0x012;
pub const GAISLER_PCISBRG: u32 = 0x013; pub const GAISLER_PCIFBRG: u32 = 0x014;
pub const GAISLER_PCITRACE: u32 = 0x015; pub const GAISLER_PCIDMA: u32 = 0x016;
pub const GAISLER_AHBTRACE: u32 = 0x017; pub const GAISLER_ETHDSU: u32 = 0x018;
pub const GAISLER_PIOPORT: u32 = 0x01A; pub const GAISLER_GRGPIO: u32 = 0x01A;
pub const GAISLER_AHBJTAG: u32 = 0x01c; pub const GAISLER_ETHMAC: u32 = 0x01D;
pub const GAISLER_AHB2AHB: u32 = 0x020; pub const GAISLER_USBDC: u32 = 0x021;
pub const GAISLER_ATACTRL: u32 = 0x024; pub const GAISLER_DDRSPA: u32 = 0x025;
pub const GAISLER_USBEHC: u32 = 0x026; pub const GAISLER_USBUHC: u32 = 0x027;
pub const GAISLER_I2CMST: u32 = 0x028; pub const GAISLER_SPICTRL: u32 = 0x02D;
pub const GAISLER_DDR2SPA: u32 = 0x02E; pub const GAISLER_SPIMCTRL: u32 = 0x045;
pub const GAISLER_LEON4: u32 = 0x048; pub const GAISLER_LEON4DSU: u32 = 0x049;
pub const GAISLER_AHBSTAT: u32 = 0x052; pub const GAISLER_FTMCTRL: u32 = 0x054;
pub const GAISLER_KBD: u32 = 0x060; pub const GAISLER_VGA: u32 = 0x061;
pub const GAISLER_SVGA: u32 = 0x063; pub const GAISLER_GRSYSMON: u32 = 0x066;
pub const GAISLER_GRACECTRL: u32 = 0x067;
pub const GAISLER_L2TIME: u32 = 0xffd; pub const GAISLER_L2C: u32 = 0xffe;
pub const GAISLER_PLUGPLAY: u32 = 0xfff;
pub const AEROFLEX_UT699: u32 = 0x0699; pub const LEON4_NEXTREME1: u32 = 0x0102;
pub const GAISLER_GR712RC: u32 = 0x0712;

#[inline]
pub const fn amba_vendor(x: u32) -> u32 { (x >> 24) & 0xff }
#[inline]
pub const fn amba_device(x: u32) -> u32 { (x >> 12) & 0xfff }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
