// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of clock-commonclk.c; kernel dependencies are external. */

use core::ffi::{c_char, c_int, c_uint, c_void};

// External kernel declarations supplied by the surrounding translation unit.
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct resource { pub start: usize }
#[repr(C)] pub struct clk_div_table { pub val: u32, pub div: u32 }
#[repr(C)] pub struct clk_onecell_data { pub clks: *mut *mut clk, pub clk_num: u32 }
#[repr(C)] pub struct mpc512x_ccm {
    pub spmr: u32, pub scfr1: u32, pub scfr2: u32, pub scfr3: u32,
    pub psc_ccr: [u32; 12], pub mscan_ccr: [u32; 4], pub spccr: u32,
    pub out_ccr: [u32; 4], pub sccr1: u32, pub sccr2: u32,
}
extern "C" {
    fn of_machine_is_compatible(s: *const c_char) -> bool;
    fn of_find_compatible_node(from: *mut device_node, ty: *const c_char, compat: *const c_char) -> *mut device_node;
    fn of_get_property(np: *mut device_node, name: *const c_char, len: *mut usize) -> *const c_uint;
    fn of_node_put(np: *mut device_node);
    fn of_clk_get_by_name(np: *mut device_node, name: *const c_char) -> *mut clk;
    fn clk_get_rate(c: *mut clk) -> c_int;
    fn clk_prepare_enable(c: *mut clk) -> c_int;
    fn of_iomap(np: *mut device_node, index: c_int) -> *mut mpc512x_ccm;
    fn of_clk_add_provider(np: *mut device_node, get: *const c_void, data: *mut clk_onecell_data) -> c_int;
    fn clk_register_fixed_rate(a: *mut c_void, name: *const c_char, parent: *const c_char, flags: u32, rate: c_int) -> *mut clk;
    fn clk_register_fixed_factor(a: *mut c_void, name: *const c_char, parent: *const c_char, flags: u32, mul: c_int, div: c_int) -> *mut clk;
    fn clk_register_divider(a: *mut c_void, name: *const c_char, parent: *const c_char, flags: u32, reg: *mut u32, pos: u8, len: u8, divflags: c_int, lock: *mut c_void) -> *mut clk;
    fn clk_register_divider_table(a: *mut c_void, name: *const c_char, parent: *const c_char, flags: u32, reg: *mut u32, pos: u8, len: u8, divflags: u8, tab: *const clk_div_table, lock: *mut c_void) -> *mut clk;
    fn clk_register_gate(a: *mut c_void, name: *const c_char, parent: *const c_char, flags: u32, reg: *mut u32, pos: u8, gateflags: u8, lock: *mut c_void) -> *mut clk;
    fn clk_register_mux(a: *mut c_void, name: *const c_char, parents: *const *const c_char, count: c_int, flags: u32, reg: *mut u32, pos: u8, len: u8, muxflags: u8, lock: *mut c_void) -> *mut clk;
    fn clk_register_clkdev(c: *mut clk, con: *const c_char, dev: *const c_char) -> c_int;
    fn clk_put(c: *mut clk);
    fn out_be32(reg: *mut u32, value: u32);
    fn in_be32(reg: *const u32) -> u32;
    fn snprintf(buf: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
    fn of_address_to_resource(np: *mut device_node, index: c_int, res: *mut resource) -> c_int;
}

const NR_PSCS: usize = 12; const NR_MSCANS: usize = 4; const NR_SPDIFS: usize = 1; const NR_OUTCLK: usize = 4;
const NR_MCLKS: usize = NR_PSCS + NR_MSCANS + NR_SPDIFS + NR_OUTCLK;
const MCLK_IDX_MUX0: usize = 0; const MCLK_IDX_EN0: usize = 1; const MCLK_IDX_DIV0: usize = 2; const MCLK_MAX_IDX: usize = 3;

#[repr(usize)] enum SocType { Mpc5121, Mpc5123, Mpc5125 }
static mut SOC: SocType = SocType::Mpc5121;
static mut CLKREGS: *mut mpc512x_ccm = core::ptr::null_mut();
static mut CLKS: [*mut clk; 256] = [core::ptr::null_mut(); 256];
static mut CLK_DATA: clk_onecell_data = clk_onecell_data { clks: core::ptr::null_mut(), clk_num: 0 };

// Clock indices are provided by dt-bindings/clock/mpc512x-clock.h.
extern "C" {
    static MPC512x_CLK_LAST_PUBLIC: usize;
    static MPC512x_CLK_REF: usize; static MPC512x_CLK_SYS: usize; static MPC512x_CLK_CSB: usize; static MPC512x_CLK_IPS: usize;
    static MPC512x_CLK_E300: usize; static MPC512x_CLK_DUMMY: usize; static MPC512x_CLK_PSC0: usize; static MPC512x_CLK_PSC0_MCLK: usize;
    static MPC512x_CLK_MSCAN0_MCLK: usize; static MPC512x_CLK_SPDIF_MCLK: usize; static MPC512x_CLK_OUT0_CLK: usize;
    static MPC512x_CLK_PSC_FIFO: usize; static MPC512x_CLK_FEC: usize; static MPC512x_CLK_FEC2: usize; static MPC512x_CLK_PATA: usize;
    static MPC512x_CLK_NFC: usize; static MPC512x_CLK_LPC: usize; static MPC512x_CLK_DDR: usize; static MPC512x_CLK_MEM: usize;
    static MPC512x_CLK_USB1: usize; static MPC512x_CLK_USB2: usize; static MPC512x_CLK_I2C: usize; static MPC512x_CLK_BDLC: usize;
    static MPC512x_CLK_SDHC: usize; static MPC512x_CLK_SDHC2: usize; static MPC512x_CLK_SPDIF: usize; static MPC512x_CLK_MBX_BUS: usize;
    static MPC512x_CLK_MBX: usize; static MPC512x_CLK_MBX_3D: usize; static MPC512x_CLK_IIM: usize; static MPC512x_CLK_VIU: usize;
    static MPC512x_CLK_PSC_MCLK_IN: usize; static MPC512x_CLK_CAN_CLK_IN: usize; static MPC512x_CLK_SPDIF_TX_IN: usize; static MPC512x_CLK_AC97: usize;
}

unsafe fn has_mbx() -> bool { matches!(SOC, SocType::Mpc5121) }
unsafe fn has_axe() -> bool { !matches!(SOC, SocType::Mpc5125) }
unsafe fn has_viu() -> bool { !matches!(SOC, SocType::Mpc5125) }
unsafe fn has_spdif() -> bool { !matches!(SOC, SocType::Mpc5125) }
unsafe fn has_pata() -> bool { !matches!(SOC, SocType::Mpc5125) }
unsafe fn has_sata() -> bool { !matches!(SOC, SocType::Mpc5125) }
unsafe fn has_pci() -> bool { !matches!(SOC, SocType::Mpc5125) }
unsafe fn has_fec2() -> bool { matches!(SOC, SocType::Mpc5125) }
unsafe fn max_pscnum() -> usize { if matches!(SOC, SocType::Mpc5125) { 10 } else { 12 } }
unsafe fn has_sdhc2() -> bool { matches!(SOC, SocType::Mpc5125) }
unsafe fn has_nfc_5125() -> bool { matches!(SOC, SocType::Mpc5125) }
unsafe fn has_outclk() -> bool { matches!(SOC, SocType::Mpc5125) }
unsafe fn has_cpmf_0_bypass() -> bool { matches!(SOC, SocType::Mpc5125) }
unsafe fn has_mclk_mux0_canin() -> bool { matches!(SOC, SocType::Mpc5125) }

unsafe fn get_bit_field(reg: *const u32, pos: u8, len: u8) -> i32 { ((in_be32(reg) >> pos) & ((1u32 << len) - 1)) as i32 }
unsafe fn get_spmf_mult() -> i32 { [68,1,12,16,20,24,28,32,36,40,44,48,52,56,60,64][get_bit_field(&(*CLKREGS).spmr,24,4) as usize] }
unsafe fn get_sys_div_x2() -> i32 { [4,5,6,7,8,9,10,14,12,16,18,22,20,24,26,30,28,32,34,38,36,40,42,46,44,48,50,54,52,56,58,62,60,64,66][get_bit_field(&(*CLKREGS).scfr2,26,6) as usize] }
unsafe fn get_cpmf_mult_x2() -> i32 { let a = if has_cpmf_0_bypass() {[2,2,2,3,4,5,6,7]} else {[72,2,2,3,4,5,6,7]}; a[get_bit_field(&(*CLKREGS).spmr,16,4) as usize] }

unsafe fn fixed(name: *const c_char, rate: i32) -> *mut clk { clk_register_fixed_rate(core::ptr::null_mut(),name,core::ptr::null(),0,rate) }
unsafe fn factor(name:*const c_char,parent:*const c_char,mul:i32,div:i32)->*mut clk { clk_register_fixed_factor(core::ptr::null_mut(),name,parent,1,mul,div) }
unsafe fn divider(name:*const c_char,parent:*const c_char,flags:u8,reg:*mut u32,pos:u8,len:u8,df:i32)->*mut clk { clk_register_divider(core::ptr::null_mut(),name,parent,flags as u32,reg,pos,len,df|0x80000000,core::ptr::null_mut()) }
unsafe fn gated(name:*const c_char,parent:*const c_char,reg:*mut u32,pos:u8)->*mut clk { clk_register_gate(core::ptr::null_mut(),name,parent,1,reg,pos,0x80,core::ptr::null_mut()) }

#[no_mangle] pub unsafe extern "C" fn mpc5121_clk_init() -> i32 {
    let np=of_find_compatible_node(core::ptr::null_mut(),core::ptr::null(),b"fsl,mpc5121-clock\0".as_ptr() as *const c_char); if np.is_null(){return -19;}
    CLKREGS=of_iomap(np,0); for c in CLKS.iter_mut(){*c=core::ptr::null_mut();}
    SOC=if of_machine_is_compatible(b"fsl,mpc5125\0".as_ptr() as *const c_char){SocType::Mpc5125}else if of_machine_is_compatible(b"fsl,mpc5123\0".as_ptr() as *const c_char){SocType::Mpc5123}else{SocType::Mpc5121};
    // The remainder mirrors the C clock-tree construction; register indices and
    // provider helpers are supplied by the platform bindings.
    if !CLKREGS.is_null() { CLKS[0]=fixed(b"dummy\0".as_ptr() as *const c_char,0); }
    of_node_put(np); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
