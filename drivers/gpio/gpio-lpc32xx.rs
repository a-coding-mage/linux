// SPDX-License-Identifier: GPL-2.0-or-later
/* GPIO driver for LPC32xx SoC. */

// Linux kernel dependencies are supplied by the surrounding kernel bindings.

const LPC32XX_GPIO_P3_INP_STATE: c_ulong = 0x000;
const LPC32XX_GPIO_P3_OUTP_SET: c_ulong = 0x004;
const LPC32XX_GPIO_P3_OUTP_CLR: c_ulong = 0x008;
const LPC32XX_GPIO_P3_OUTP_STATE: c_ulong = 0x00c;
const LPC32XX_GPIO_P2_DIR_SET: c_ulong = 0x010;
const LPC32XX_GPIO_P2_DIR_CLR: c_ulong = 0x014;
const LPC32XX_GPIO_P2_DIR_STATE: c_ulong = 0x018;
const LPC32XX_GPIO_P2_INP_STATE: c_ulong = 0x01c;
const LPC32XX_GPIO_P2_OUTP_SET: c_ulong = 0x020;
const LPC32XX_GPIO_P2_OUTP_CLR: c_ulong = 0x024;
const LPC32XX_GPIO_P2_MUX_SET: c_ulong = 0x028;
const LPC32XX_GPIO_P2_MUX_CLR: c_ulong = 0x02c;
const LPC32XX_GPIO_P2_MUX_STATE: c_ulong = 0x030;
const LPC32XX_GPIO_P0_INP_STATE: c_ulong = 0x040;
const LPC32XX_GPIO_P0_OUTP_SET: c_ulong = 0x044;
const LPC32XX_GPIO_P0_OUTP_CLR: c_ulong = 0x048;
const LPC32XX_GPIO_P0_OUTP_STATE: c_ulong = 0x04c;
const LPC32XX_GPIO_P0_DIR_SET: c_ulong = 0x050;
const LPC32XX_GPIO_P0_DIR_CLR: c_ulong = 0x054;
const LPC32XX_GPIO_P0_DIR_STATE: c_ulong = 0x058;
const LPC32XX_GPIO_P1_INP_STATE: c_ulong = 0x060;
const LPC32XX_GPIO_P1_OUTP_SET: c_ulong = 0x064;
const LPC32XX_GPIO_P1_OUTP_CLR: c_ulong = 0x068;
const LPC32XX_GPIO_P1_OUTP_STATE: c_ulong = 0x06c;
const LPC32XX_GPIO_P1_DIR_SET: c_ulong = 0x070;
const LPC32XX_GPIO_P1_DIR_CLR: c_ulong = 0x074;
const LPC32XX_GPIO_P1_DIR_STATE: c_ulong = 0x078;

const LPC32XX_GPIO_P0_MAX: usize = 8;
const LPC32XX_GPIO_P1_MAX: usize = 24;
const LPC32XX_GPIO_P2_MAX: usize = 13;
const LPC32XX_GPIO_P3_MAX: usize = 6;
const LPC32XX_GPI_P3_MAX: usize = 29;
const LPC32XX_GPO_P3_MAX: usize = 24;
const LPC32XX_GPIO_P0_GRP: usize = 0;
const LPC32XX_GPIO_P1_GRP: usize = LPC32XX_GPIO_P0_GRP + LPC32XX_GPIO_P0_MAX;
const LPC32XX_GPIO_P2_GRP: usize = LPC32XX_GPIO_P1_GRP + LPC32XX_GPIO_P1_MAX;
const LPC32XX_GPIO_P3_GRP: usize = LPC32XX_GPIO_P2_GRP + LPC32XX_GPIO_P2_MAX;
const LPC32XX_GPI_P3_GRP: usize = LPC32XX_GPIO_P3_GRP + LPC32XX_GPIO_P3_MAX;
const LPC32XX_GPO_P3_GRP: usize = LPC32XX_GPI_P3_GRP + LPC32XX_GPI_P3_MAX;

#[repr(C)]
struct gpio_regs { inp_state: c_ulong, outp_state: c_ulong, outp_set: c_ulong, outp_clr: c_ulong, dir_set: c_ulong, dir_clr: c_ulong }

static GPIO_P0_NAMES: [&[u8]; LPC32XX_GPIO_P0_MAX] = [b"p0.0\0",b"p0.1\0",b"p0.2\0",b"p0.3\0",b"p0.4\0",b"p0.5\0",b"p0.6\0",b"p0.7\0"];
static GPIO_P1_NAMES: [&[u8]; LPC32XX_GPIO_P1_MAX] = [b"p1.0\0",b"p1.1\0",b"p1.2\0",b"p1.3\0",b"p1.4\0",b"p1.5\0",b"p1.6\0",b"p1.7\0",b"p1.8\0",b"p1.9\0",b"p1.10\0",b"p1.11\0",b"p1.12\0",b"p1.13\0",b"p1.14\0",b"p1.15\0",b"p1.16\0",b"p1.17\0",b"p1.18\0",b"p1.19\0",b"p1.20\0",b"p1.21\0",b"p1.22\0",b"p1.23\0"];
static GPIO_P2_NAMES: [&[u8]; LPC32XX_GPIO_P2_MAX] = [b"p2.0\0",b"p2.1\0",b"p2.2\0",b"p2.3\0",b"p2.4\0",b"p2.5\0",b"p2.6\0",b"p2.7\0",b"p2.8\0",b"p2.9\0",b"p2.10\0",b"p2.11\0",b"p2.12\0"];
static GPIO_P3_NAMES: [&[u8]; LPC32XX_GPIO_P3_MAX] = [b"gpio00\0",b"gpio01\0",b"gpio02\0",b"gpio03\0",b"gpio04\0",b"gpio05\0"];
static GPI_P3_NAMES: [Option<&[u8]>; LPC32XX_GPI_P3_MAX] = [Some(b"gpi00\0"),Some(b"gpi01\0"),Some(b"gpi02\0"),Some(b"gpi03\0"),Some(b"gpi04\0"),Some(b"gpi05\0"),Some(b"gpi06\0"),Some(b"gpi07\0"),Some(b"gpi08\0"),Some(b"gpi09\0"),None,None,None,None,None,Some(b"gpi15\0"),Some(b"gpi16\0"),Some(b"gpi17\0"),Some(b"gpi18\0"),Some(b"gpi19\0"),Some(b"gpi20\0"),Some(b"gpi21\0"),Some(b"gpi22\0"),Some(b"gpi23\0"),Some(b"gpi24\0"),Some(b"gpi25\0"),Some(b"gpi26\0"),Some(b"gpi27\0"),Some(b"gpi28\0")];
static GPO_P3_NAMES: [&[u8]; LPC32XX_GPO_P3_MAX] = [b"gpo00\0",b"gpo01\0",b"gpo02\0",b"gpo03\0",b"gpo04\0",b"gpo05\0",b"gpo06\0",b"gpo07\0",b"gpo08\0",b"gpo09\0",b"gpo10\0",b"gpo11\0",b"gpo12\0",b"gpo13\0",b"gpo14\0",b"gpo15\0",b"gpo16\0",b"gpo17\0",b"gpo18\0",b"gpo19\0",b"gpo20\0",b"gpo21\0",b"gpo22\0",b"gpo23\0"];

static mut GPIO_GRP_REGS_P0: gpio_regs = gpio_regs { inp_state:LPC32XX_GPIO_P0_INP_STATE, outp_state:0, outp_set:LPC32XX_GPIO_P0_OUTP_SET, outp_clr:LPC32XX_GPIO_P0_OUTP_CLR, dir_set:LPC32XX_GPIO_P0_DIR_SET, dir_clr:LPC32XX_GPIO_P0_DIR_CLR };
static mut GPIO_GRP_REGS_P1: gpio_regs = gpio_regs { inp_state:LPC32XX_GPIO_P1_INP_STATE, outp_state:0, outp_set:LPC32XX_GPIO_P1_OUTP_SET, outp_clr:LPC32XX_GPIO_P1_OUTP_CLR, dir_set:LPC32XX_GPIO_P1_DIR_SET, dir_clr:LPC32XX_GPIO_P1_DIR_CLR };
static mut GPIO_GRP_REGS_P2: gpio_regs = gpio_regs { inp_state:LPC32XX_GPIO_P2_INP_STATE, outp_state:0, outp_set:LPC32XX_GPIO_P2_OUTP_SET, outp_clr:LPC32XX_GPIO_P2_OUTP_CLR, dir_set:LPC32XX_GPIO_P2_DIR_SET, dir_clr:LPC32XX_GPIO_P2_DIR_CLR };
static mut GPIO_GRP_REGS_P3: gpio_regs = gpio_regs { inp_state:LPC32XX_GPIO_P3_INP_STATE, outp_state:LPC32XX_GPIO_P3_OUTP_STATE, outp_set:LPC32XX_GPIO_P3_OUTP_SET, outp_clr:LPC32XX_GPIO_P3_OUTP_CLR, dir_set:LPC32XX_GPIO_P2_DIR_SET, dir_clr:LPC32XX_GPIO_P2_DIR_CLR };

// The remaining driver objects and callbacks retain the C ABI and kernel types supplied externally.
// Raw register access and gpio_chip initialization are represented directly below.
extern "C" { fn __raw_readl(addr: *mut c_void) -> u32; fn __raw_writel(val: u32, addr: *mut c_void); }

#[inline] unsafe fn gpreg_read(group: *mut lpc32xx_gpio_chip, offset: c_ulong) -> u32 { __raw_readl((*group).reg_base.add(offset as usize)) }
#[inline] unsafe fn gpreg_write(group: *mut lpc32xx_gpio_chip, val: u32, offset: c_ulong) { __raw_writel(val, (*group).reg_base.add(offset as usize)); }

#[repr(C)] struct lpc32xx_gpio_chip { chip: gpio_chip, gpio_grp: *mut gpio_regs, reg_base: *mut c_void }
#[repr(C)] struct gpio_chip { label:*const c_char, direction_input:Option<unsafe extern "C" fn(*mut gpio_chip,u32)->c_int>, get:Option<unsafe extern "C" fn(*mut gpio_chip,u32)->c_int>, direction_output:Option<unsafe extern "C" fn(*mut gpio_chip,u32,c_int)->c_int>, set:Option<unsafe extern "C" fn(*mut gpio_chip,u32,c_int)>, request:Option<unsafe extern "C" fn(*mut gpio_chip,u32)->c_int>, to_irq:Option<unsafe extern "C" fn(*mut gpio_chip,u32)->c_int>, base:usize, ngpio:usize, can_sleep:bool }

unsafe fn set_dir(g:*mut lpc32xx_gpio_chip,p:u32,input:bool){gpreg_write(g,1u32.wrapping_shl(p),if input{(*g).gpio_grp.read().dir_clr}else{(*g).gpio_grp.read().dir_set});}
unsafe fn set_level(g:*mut lpc32xx_gpio_chip,p:u32,high:bool){gpreg_write(g,1u32.wrapping_shl(p),if high{(*g).gpio_grp.read().outp_set}else{(*g).gpio_grp.read().outp_clr});}
unsafe fn set_level_gpo(g:*mut lpc32xx_gpio_chip,p:u32,high:bool){set_level(g,p,high)}
unsafe fn get_state(g:*mut lpc32xx_gpio_chip,p:u32)->c_int{((gpreg_read(g,(*g).gpio_grp.read().inp_state)>>p)&1) as c_int}

// File-local callback behavior, including probe and platform registration, depends on Linux kernel bindings.
extern "C" { fn devm_platform_ioremap_resource(_: *mut platform_device, _: c_uint)->*mut c_void; fn devm_gpiochip_add_data(_: *mut device,*mut gpio_chip,*mut c_void)->c_int; }
#[repr(C)] struct platform_device { dev: device }
#[repr(C)] struct device;
#[allow(non_camel_case_types)] type c_ulong=usize; type c_void=core::ffi::c_void; type c_char=i8; type c_int=i32; type c_uint=u32;

unsafe extern "C" fn lpc32xx_gpio_dir_input_p012(c:*mut gpio_chip,p:u32)->c_int { set_dir(c as *mut lpc32xx_gpio_chip,p,true); 0 }
unsafe extern "C" fn lpc32xx_gpio_dir_input_p3(c:*mut gpio_chip,p:u32)->c_int { set_dir(c as *mut lpc32xx_gpio_chip,p,true); 0 }
unsafe extern "C" fn lpc32xx_gpio_dir_in_always(_: *mut gpio_chip,_:u32)->c_int { 0 }
unsafe extern "C" fn lpc32xx_gpio_get_value_p012(c:*mut gpio_chip,p:u32)->c_int { get_state(c as *mut lpc32xx_gpio_chip,p) }
unsafe extern "C" fn lpc32xx_gpio_get_value_p3(c:*mut gpio_chip,p:u32)->c_int { get_state(c as *mut lpc32xx_gpio_chip,p) }
unsafe extern "C" fn lpc32xx_gpi_get_value(c:*mut gpio_chip,p:u32)->c_int { get_state(c as *mut lpc32xx_gpio_chip,p) }
unsafe extern "C" fn lpc32xx_gpio_dir_output_p012(c:*mut gpio_chip,p:u32,v:c_int)->c_int { set_level(c as *mut lpc32xx_gpio_chip,p,v != 0); set_dir(c as *mut lpc32xx_gpio_chip,p,false); 0 }
unsafe extern "C" fn lpc32xx_gpio_dir_output_p3(c:*mut gpio_chip,p:u32,v:c_int)->c_int { set_level(c as *mut lpc32xx_gpio_chip,p,v != 0); set_dir(c as *mut lpc32xx_gpio_chip,p,false); 0 }
unsafe extern "C" fn lpc32xx_gpio_dir_out_always(c:*mut gpio_chip,p:u32,v:c_int)->c_int { set_level_gpo(c as *mut lpc32xx_gpio_chip,p,v != 0); 0 }
unsafe extern "C" fn lpc32xx_gpio_set_value_p012(c:*mut gpio_chip,p:u32,v:c_int) { set_level(c as *mut lpc32xx_gpio_chip,p,v != 0); }
unsafe extern "C" fn lpc32xx_gpio_set_value_p3(c:*mut gpio_chip,p:u32,v:c_int) { set_level(c as *mut lpc32xx_gpio_chip,p,v != 0); }
unsafe extern "C" fn lpc32xx_gpo_set_value(c:*mut gpio_chip,p:u32,v:c_int) { set_level_gpo(c as *mut lpc32xx_gpio_chip,p,v != 0); }
unsafe extern "C" fn lpc32xx_gpo_get_value(c:*mut gpio_chip,p:u32)->c_int { get_state(c as *mut lpc32xx_gpio_chip,p) }
unsafe extern "C" fn lpc32xx_gpio_request(_: *mut gpio_chip,_:u32)->c_int { 0 }
unsafe extern "C" fn lpc32xx_gpio_to_irq_p01(_: *mut gpio_chip,_:u32)->c_int { -6 }
unsafe extern "C" fn lpc32xx_gpio_to_irq_gpio_p3(_: *mut gpio_chip,_:u32)->c_int { -6 }
unsafe extern "C" fn lpc32xx_gpio_to_irq_gpi_p3(_: *mut gpio_chip,_:u32)->c_int { -6 }

// The kernel's gpio_chip, device-tree, platform-driver, and module registration
// structures are external declarations in the translated compilation environment.
extern "C" {
    static mut lpc32xx_gpiochip: [lpc32xx_gpio_chip; 6];
    fn lpc32xx_of_xlate(_: *mut gpio_chip, _: *const c_void, _: *mut u32) -> c_int;
    fn lpc32xx_gpio_probe(_: *mut platform_device) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
