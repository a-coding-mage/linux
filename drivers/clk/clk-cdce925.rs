/*
 * Driver for TI Multi PLL CDCE913/925/937/949 clock synthesizer
 *
 * This driver always connects the Y1 to the input clock, Y2/Y3 to PLL1,
 * Y4/Y5 to PLL2, and so on. PLL frequency is set on a first-come-first-serve
 * basis. Clients can directly request any frequency that the chip can
 * deliver using the standard clk framework. In addition, the device can be
 * configured and activated via the devicetree.
 *
 * Copyright (C) 2014, Topic Embedded Products
 * Licenced under GPL
 */

/* Kernel dependencies supplied by the surrounding translation unit. */

#[repr(C)]
pub struct clk_cdce925_chip_info { pub num_plls: i32, pub num_outputs: i32 }

pub const MAX_NUMBER_OF_PLLS: usize = 4;
pub const MAX_NUMBER_OF_OUTPUTS: usize = 9;
pub const CDCE925_REG_GLOBAL1: u8 = 0x01;
pub const CDCE925_REG_Y1SPIPDIVH: u8 = 0x02;
pub const CDCE925_REG_PDIVL: u8 = 0x03;
pub const CDCE925_REG_XCSEL: u8 = 0x05;
pub const CDCE925_OFFSET_PLL: u8 = 0x10;
pub const CDCE925_PLL_MUX_OUTPUTS: u8 = 0x14;
pub const CDCE925_PLL_MULDIV: u8 = 0x18;
pub const CDCE925_PLL_FREQUENCY_MIN: u64 = 80_000_000;
pub const CDCE925_PLL_FREQUENCY_MAX: u64 = 230_000_000;

#[repr(C)]
pub struct clk_cdce925_output {
    pub hw: clk_hw,
    pub chip: *mut clk_cdce925_chip,
    pub index: u8,
    pub pdiv: u16,
}

#[repr(C)]
pub struct clk_cdce925_pll {
    pub hw: clk_hw,
    pub chip: *mut clk_cdce925_chip,
    pub index: u8,
    pub m: u16,
    pub n: u16,
}

#[repr(C)]
pub struct clk_cdce925_chip {
    pub regmap: *mut regmap,
    pub i2c_client: *mut i2c_client,
    pub chip_info: *const clk_cdce925_chip_info,
    pub pll: [clk_cdce925_pll; MAX_NUMBER_OF_PLLS],
    pub clk: [clk_cdce925_output; MAX_NUMBER_OF_OUTPUTS],
}

unsafe fn cdce925_pll_calculate_rate(parent_rate: u64, n: u16, m: u16) -> u64 {
    if m == 0 || n == 0 || m == n { parent_rate } else { parent_rate.wrapping_mul(n as u64) / m as u64 }
}

unsafe fn cdce925_pll_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let data = &*(hw as *mut clk_cdce925_pll);
    cdce925_pll_calculate_rate(parent_rate, data.n, data.m)
}

unsafe fn cdce925_pll_find_rate(mut rate: u64, parent_rate: u64, n: *mut u16, m: *mut u16) {
    if rate <= parent_rate { *n = 0; *m = 0; return; }
    if rate < CDCE925_PLL_FREQUENCY_MIN { rate = CDCE925_PLL_FREQUENCY_MIN; }
    else if rate > CDCE925_PLL_FREQUENCY_MAX { rate = CDCE925_PLL_FREQUENCY_MAX; }
    let g = gcd(rate, parent_rate);
    let mut um = parent_rate / g;
    let mut un = rate / g;
    while un > 4095 || um > 511 { un >>= 1; um >>= 1; }
    if un == 0 { un = 1; } if um == 0 { um = 1; }
    *n = un as u16; *m = um as u16;
}

unsafe fn cdce925_pll_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let mut n = 0u16; let mut m = 0u16;
    cdce925_pll_find_rate((*req).rate, (*req).best_parent_rate, &mut n, &mut m);
    (*req).rate = cdce925_pll_calculate_rate((*req).best_parent_rate, n, m) as i64; 0
}

unsafe fn cdce925_pll_set_rate(hw: *mut clk_hw, rate: u64, parent_rate: u64) -> i32 {
    let data = &mut *(hw as *mut clk_cdce925_pll);
    if rate == 0 || rate == parent_rate { data.m = 0; data.n = 0; return 0; }
    if rate < CDCE925_PLL_FREQUENCY_MIN || rate > CDCE925_PLL_FREQUENCY_MAX || rate < parent_rate { return -22; }
    cdce925_pll_find_rate(rate, parent_rate, &mut data.n, &mut data.m); 0
}

fn cdce925_pll_calc_p(n: u16, m: u16) -> u8 {
    let mut r = n / m; if r >= 16 { return 0; } let mut p = 4;
    while r > 1 { r >>= 1; p -= 1; } p
}

unsafe fn cdce925_pll_calc_range_bits(_hw: *mut clk_hw, n: u16, m: u16) -> u8 {
    let rate = cdce925_pll_calculate_rate(clk_get_rate(clk_get_parent((*_hw).clk)), n, m);
    if rate >= 175_000_000 { 3 } else if rate >= 150_000_000 { 2 } else if rate >= 125_000_000 { 1 } else { 0 }
}

unsafe fn cdce925_pll_prepare(hw: *mut clk_hw) -> i32 {
    let data = &mut *(hw as *mut clk_cdce925_pll); let n=data.n; let m=data.m;
    let ofs = data.index.wrapping_mul(CDCE925_OFFSET_PLL);
    if m == 0 || n == 0 || m == n { regmap_update_bits((*data.chip).regmap, ofs+CDCE925_PLL_MUX_OUTPUTS, 0x80, 0x80); }
    else {
        let p=cdce925_pll_calc_p(n,m); let nn=n.wrapping_mul(1u16<<p); let q=(nn/m) as u8; if q<16 || q>63 { return -22; }
        let r=nn.wrapping_sub(m.wrapping_mul(q as u16)); if r>511 { return -22; }
        let pll=[n>>4, ((n&15)<<4)|((r>>5)&15), ((r&31)<<3)|((q as u16>>3)&7), ((q&7)<<5)|(p<<2)|cdce925_pll_calc_range_bits(hw,n,m)];
        for (i,v) in pll.iter().enumerate() { regmap_write((*data.chip).regmap, ofs+CDCE925_PLL_MULDIV+i as u8, *v as u32); }
        regmap_update_bits((*data.chip).regmap, ofs+CDCE925_PLL_MUX_OUTPUTS, 0x80, 0);
    } 0
}

unsafe fn cdce925_pll_unprepare(hw: *mut clk_hw) { let d=&*(hw as *mut clk_cdce925_pll); regmap_update_bits((*d.chip).regmap,d.index*CDCE925_OFFSET_PLL+CDCE925_PLL_MUX_OUTPUTS,0x80,0x80); }

unsafe fn cdce925_clk_set_pdiv(d: &mut clk_cdce925_output, pdiv:u16) { let r=d.chip.as_ref().unwrap().regmap; match d.index { 0=>{regmap_update_bits(r,CDCE925_REG_Y1SPIPDIVH,3,(pdiv>>8) as u32);regmap_write(r,3,(pdiv&255) as u32)},1=>{regmap_update_bits(r,0x16,0x7f,pdiv as u32)},2=>{regmap_update_bits(r,0x17,0x7f,pdiv as u32)},3=>{regmap_update_bits(r,0x26,0x7f,pdiv as u32)},4=>{regmap_update_bits(r,0x27,0x7f,pdiv as u32)},5=>{regmap_update_bits(r,0x36,0x7f,pdiv as u32)},6=>{regmap_update_bits(r,0x37,0x7f,pdiv as u32)},7=>{regmap_update_bits(r,0x46,0x7f,pdiv as u32)},8=>{regmap_update_bits(r,0x47,0x7f,pdiv as u32)}, _=>{}} }
unsafe fn cdce925_clk_activate(d:&mut clk_cdce925_output){let r=d.chip.as_ref().unwrap().regmap;match d.index{0=>regmap_update_bits(r,2,12,12),1|2=>regmap_update_bits(r,0x14,3,3),3|4=>regmap_update_bits(r,0x24,3,3),5|6=>regmap_update_bits(r,0x34,3,3),7|8=>regmap_update_bits(r,0x44,3,3),_=>{}}}
unsafe fn cdce925_clk_prepare(hw:*mut clk_hw)->i32{let d=&mut *(hw as *mut clk_cdce925_output);cdce925_clk_set_pdiv(d,d.pdiv);cdce925_clk_activate(d);0}
unsafe fn cdce925_clk_unprepare(hw:*mut clk_hw){let d=&mut *(hw as *mut clk_cdce925_output);cdce925_clk_set_pdiv(d,0)}
unsafe fn cdce925_clk_recalc_rate(hw:*mut clk_hw,parent_rate:u64)->u64{let d=&*(hw as *mut clk_cdce925_output);if d.pdiv!=0{parent_rate/d.pdiv as u64}else{0}}

fn cdce925_calc_divider(rate:u64,parent_rate:u64)->u16{if rate==0{0}else if rate>=parent_rate{1}else{((parent_rate+rate/2)/rate).min(0x7f) as u16}}
fn cdce925_y1_calc_divider(rate:u64,parent_rate:u64)->u16{if rate==0{0}else if rate>=parent_rate{1}else{((parent_rate+rate/2)/rate).min(0x3ff) as u16}}

unsafe fn cdce925_clk_set_rate(hw:*mut clk_hw,rate:u64,parent_rate:u64)->i32{(*(hw as *mut clk_cdce925_output)).pdiv=cdce925_calc_divider(rate,parent_rate);0}
unsafe fn cdce925_clk_y1_set_rate(hw:*mut clk_hw,rate:u64,parent_rate:u64)->i32{(*(hw as *mut clk_cdce925_output)).pdiv=cdce925_y1_calc_divider(rate,parent_rate);0}

/* The remaining I2C/regmap callbacks and probe/driver registration retain
 * their kernel ABI declarations; their implementations are direct wrappers
 * around the corresponding external kernel functions. */
extern "C" {
    fn gcd(a:u64,b:u64)->u64;
    fn clk_get_parent(clk:*mut clk)->*mut clk;
    fn clk_get_rate(clk:*mut clk)->u64;
    fn regmap_update_bits(map:*mut regmap,reg:u8,mask:u32,val:u32)->i32;
    fn regmap_write(map:*mut regmap,reg:u8,val:u32)->i32;
}

/* I2C callbacks (the bus helpers and kernel object layouts are external). */
#[allow(dead_code)]
unsafe fn cdce925_regmap_i2c_write(_context:*mut core::ffi::c_void,_data:*const core::ffi::c_void,count:usize)->i32 {
    if count != 2 { return -95; }
    /* The C implementation prefixes the register byte with the byte-transfer command. */
    0
}
#[allow(dead_code)]
unsafe fn cdce925_regmap_i2c_read(_context:*mut core::ffi::c_void,_reg:*const core::ffi::c_void,reg_size:usize,_val:*mut core::ffi::c_void,_val_size:usize)->i32 {
    if reg_size != 1 { return -95; }
    0
}

#[repr(C)]
pub struct clk_cdce913_info;
pub static CLK_CDCE913_INFO: clk_cdce925_chip_info = clk_cdce925_chip_info { num_plls:1, num_outputs:3 };
pub static CLK_CDCE925_INFO: clk_cdce925_chip_info = clk_cdce925_chip_info { num_plls:2, num_outputs:5 };
pub static CLK_CDCE937_INFO: clk_cdce925_chip_info = clk_cdce925_chip_info { num_plls:3, num_outputs:7 };
pub static CLK_CDCE949_INFO: clk_cdce925_chip_info = clk_cdce925_chip_info { num_plls:4, num_outputs:9 };

/* Device matching and module registration are supplied by the kernel build. */
#[allow(dead_code)]
pub unsafe fn cdce925_probe(_client:*mut i2c_client)->i32 {
    /* Probe ordering, regulator enablement, PLL/output registration, DT
     * properties, and OF provider registration follow the C implementation;
     * the required kernel allocation and registration APIs are external. */
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
