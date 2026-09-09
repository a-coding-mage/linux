// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Direct Rust translation of clk-sp7021.c; Linux kernel symbols are external dependencies. */

const DIV_TV: i32 = 33;
const DIV_A: i32 = 34;
const SEL_FRA: usize = 0;
const SDM_MOD: usize = 1;
const PH_SEL: usize = 2;
const NFRA: usize = 3;
const DIVR: usize = 4;
const DIVN: usize = 5;
const DIVM: usize = 6;
const P_MAX: usize = 7;

const MASK_SEL_FRA: u32 = 1 << 1;
const MASK_SDM_MOD: u32 = 1 << 2;
const MASK_PH_SEL: u32 = 1 << 4;
const MASK_NFRA: u32 = 0x1fc0;
const MASK_DIVR: u32 = 0x180;
const MASK_DIVN: u32 = 0xff;
const MASK_DIVM: u32 = 0x7f00;
const M: u32 = 1_000_000;
const F_27M: u32 = 27 * M;
const FVCO_MIN: u32 = 100 * M;
const FVCO_MAX: u32 = 200 * M;
const F_MIN: u32 = FVCO_MIN / 8;
const F_MAX: u32 = FVCO_MAX;

#[repr(C)]
pub struct sp_pll {
    pub hw: clk_hw, pub reg: *mut u8, pub lock: spinlock_t,
    pub div_shift: i32, pub div_width: i32, pub pd_bit: i32, pub bp_bit: i32,
    pub brate: usize, pub p: [u32; P_MAX],
}

#[repr(C)] pub struct sp_clk_gate_info { pub reg: u16, pub ext_parent: u16 }
static SP_CLK_GATES: &[sp_clk_gate_info] = &[
    sp_clk_gate_info{reg:2,ext_parent:0},sp_clk_gate_info{reg:5,ext_parent:0},sp_clk_gate_info{reg:6,ext_parent:0},sp_clk_gate_info{reg:7,ext_parent:0},sp_clk_gate_info{reg:9,ext_parent:0},
    sp_clk_gate_info{reg:0xb,ext_parent:1},sp_clk_gate_info{reg:0xf,ext_parent:1},sp_clk_gate_info{reg:0x14,ext_parent:0},sp_clk_gate_info{reg:0x15,ext_parent:0},sp_clk_gate_info{reg:0x16,ext_parent:0},sp_clk_gate_info{reg:0x17,ext_parent:0},
    sp_clk_gate_info{reg:0x18,ext_parent:1},sp_clk_gate_info{reg:0x19,ext_parent:1},sp_clk_gate_info{reg:0x1a,ext_parent:1},sp_clk_gate_info{reg:0x1b,ext_parent:1},sp_clk_gate_info{reg:0x1c,ext_parent:1},sp_clk_gate_info{reg:0x1d,ext_parent:1},sp_clk_gate_info{reg:0x1e,ext_parent:0},sp_clk_gate_info{reg:0x1f,ext_parent:1},
    sp_clk_gate_info{reg:0x20,ext_parent:0},sp_clk_gate_info{reg:0x21,ext_parent:0},sp_clk_gate_info{reg:0x22,ext_parent:0},sp_clk_gate_info{reg:0x23,ext_parent:0},sp_clk_gate_info{reg:0x24,ext_parent:0},sp_clk_gate_info{reg:0x25,ext_parent:0},sp_clk_gate_info{reg:0x26,ext_parent:0},sp_clk_gate_info{reg:0x2a,ext_parent:0},sp_clk_gate_info{reg:0x2b,ext_parent:0},sp_clk_gate_info{reg:0x2d,ext_parent:0},sp_clk_gate_info{reg:0x2e,ext_parent:0},sp_clk_gate_info{reg:0x30,ext_parent:0},sp_clk_gate_info{reg:0x31,ext_parent:0},sp_clk_gate_info{reg:0x32,ext_parent:0},sp_clk_gate_info{reg:0x33,ext_parent:0},sp_clk_gate_info{reg:0x3d,ext_parent:0},sp_clk_gate_info{reg:0x3e,ext_parent:0},sp_clk_gate_info{reg:0x3f,ext_parent:0},sp_clk_gate_info{reg:0x42,ext_parent:0},sp_clk_gate_info{reg:0x44,ext_parent:0},sp_clk_gate_info{reg:0x4b,ext_parent:0},sp_clk_gate_info{reg:0x4c,ext_parent:0},sp_clk_gate_info{reg:0x4d,ext_parent:0},sp_clk_gate_info{reg:0x4e,ext_parent:0},sp_clk_gate_info{reg:0x4f,ext_parent:0},sp_clk_gate_info{reg:0x50,ext_parent:0},sp_clk_gate_info{reg:0x55,ext_parent:0},sp_clk_gate_info{reg:0x60,ext_parent:0},sp_clk_gate_info{reg:0x61,ext_parent:0},sp_clk_gate_info{reg:0x6a,ext_parent:0},sp_clk_gate_info{reg:0x73,ext_parent:0},sp_clk_gate_info{reg:0x86,ext_parent:0},sp_clk_gate_info{reg:0x8a,ext_parent:0},sp_clk_gate_info{reg:0x8b,ext_parent:0},sp_clk_gate_info{reg:0x8d,ext_parent:0},sp_clk_gate_info{reg:0x8e,ext_parent:0},sp_clk_gate_info{reg:0x8f,ext_parent:0},sp_clk_gate_info{reg:0x90,ext_parent:0},sp_clk_gate_info{reg:0x92,ext_parent:0},sp_clk_gate_info{reg:0x93,ext_parent:0},sp_clk_gate_info{reg:0x95,ext_parent:0},sp_clk_gate_info{reg:0x96,ext_parent:0},sp_clk_gate_info{reg:0x97,ext_parent:0},sp_clk_gate_info{reg:0x98,ext_parent:0},sp_clk_gate_info{reg:0x99,ext_parent:0}
];

#[repr(C)] pub struct clk_hw { pub init: *const clk_init_data }
#[repr(C)] pub struct clk_init_data { pub name: *const u8, pub parent_data: *const clk_parent_data, pub ops: *const clk_ops, pub num_parents: usize, pub flags: usize }
#[repr(C)] pub struct clk_parent_data { pub hw: *mut clk_hw, pub index: u32 }
#[repr(C)] pub struct clk_ops { pub enable: Option<unsafe extern "C" fn(*mut clk_hw)->i32>, pub disable: Option<unsafe extern "C" fn(*mut clk_hw)>, pub is_enabled: Option<unsafe extern "C" fn(*mut clk_hw)->i32>, pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw,*mut clk_rate_request)->i32>, pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw,usize)->usize>, pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw,usize,usize)->i32> }
#[repr(C)] pub struct clk_rate_request { pub rate: usize, pub best_parent_rate: usize }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }

const fn field_get(mask: u32, v: u32) -> u32 { (v & mask) >> mask.trailing_zeros() }
const fn field_prep(mask: u32, v: u32) -> u32 { (v << mask.trailing_zeros()) & mask }
unsafe fn rd(p: *mut u8) -> u32 { core::ptr::read_volatile(p as *const u32) }
unsafe fn wr(v: u32, p: *mut u8) { core::ptr::write_volatile(p as *mut u32, v) }

static PT: [[u32;5];2] = [[1,5,1,F_27M,1],[10,54,2,F_27M/10,5]];
static SDM_MOD_VALS: [u32;2] = [91,55];

unsafe fn plltv_integer_div(c: *mut sp_pll, mut freq: usize) -> isize {
    let mt: [u32;19] = [1,2,3,4,5,6,8,9,10,12,15,16,18,20,24,25,27,30,32];
    freq = freq.clamp(F_MIN as usize,F_MAX as usize); let mut r=0; let fvco;
    loop { fvco=freq << r; if fvco <= FVCO_MAX as usize || r==3 {break} r+=1; }
    let mut found=None; for m in 0..mt.len() { let nf=fvco*mt[m] as usize; let n=nf/F_27M as usize; if n*F_27M as usize==nf {found=Some((m,n));break;} }
    let (m,n)=match found {Some(x)=>x,None=>return -22}; (*c).p=[0;P_MAX]; (*c).p[DIVR]=r;(*c).p[DIVN]=n as u32;(*c).p[DIVM]=mt[m]; freq as isize
}

unsafe fn plltv_fractional_div(c:*mut sp_pll, mut freq:usize)->isize {
    freq=freq.clamp(F_MIN as usize,F_MAX as usize); let mut r=0; let fvco; loop{fvco=freq<<r;if fvco<=FVCO_MAX as usize||r==3{break}r+=1;} let f=F_27M>>r; let mut best=(210000000u32,0u32);let mut fout=0usize;
    for ph in (0..2).rev(){for sdm in 0..2{for m in 1..=32{let pp=PT[ph];let nf=fvco*m;let ni=nf/pp[3] as usize;if ni<pp[1] as usize{continue}if ni>pp[1] as usize{break}let nfra=(((nf%pp[3] as usize)*SDM_MOD_VALS[sdm] as usize*pp[4] as usize)+(F_27M as usize/2))/(F_27M as usize);let df=if nfra!=0{(f as usize*(ni+pp[2] as usize)/pp[0] as usize)-(f as usize*(SDM_MOD_VALS[sdm]-nfra as u32) as usize/SDM_MOD_VALS[sdm] as usize/pp[4] as usize)}else{f as usize*ni/pp[0] as usize};let mut q=df/m;let mut rem=((df%m)*1000)/m;if freq>q{q=freq-q-1;rem=1000-rem}else{q-=freq}if q<best.0||(q==best.0&&rem<best.1){best=(q as u32,rem as u32);(*c).p[SEL_FRA]=1;(*c).p[SDM_MOD]=sdm as u32;(*c).p[PH_SEL]=ph as u32;(*c).p[NFRA]=nfra as u32;(*c).p[DIVR]=r;(*c).p[DIVM]=m;fout=df/m;}}}} if fout==0{-22}else{fout as isize}
}
unsafe fn plltv_div(c:*mut sp_pll,f:usize)->isize{if f%100!=0{plltv_fractional_div(c,f)}else{plltv_integer_div(c,f)}}

#[repr(C)] pub struct pa_entry { pub rate:u32,pub regs:[u32;5] }
static PA:[pa_entry;3]=[pa_entry{rate:135475200,regs:[0x4801,0x02df,0x248f,0x0211,0x33e9]},pa_entry{rate:147456000,regs:[0x4801,0x1adf,0x2490,0x0349,0x33e9]},pa_entry{rate:196608000,regs:[0x4801,0x42ef,0x2495,0x01c6,0x33e9]}];

unsafe fn plla_round_rate(c:*mut sp_pll,rate:usize)->usize{let mut i=PA.len();while i>1{ i-=1;if rate>=PA[i].rate as usize{break}}(*c).p[0]=i as u32;PA[i].rate as usize}
unsafe fn sp_pll_calc_div(c:*mut sp_pll,rate:usize)->u32{let max=1u32<<(*c).div_width;let mut v=((rate+(*c).brate/2)/(*c).brate) as u32;if v>max{v=max}v}
unsafe fn sp_pll_determine_rate(hw:*mut clk_hw,req:*mut clk_rate_request)->i32{let c=hw as *mut sp_pll;let ret=if (*req).rate==(*req).best_parent_rate{(*req).best_parent_rate}else if (*c).div_width==DIV_A{plla_round_rate(c,(*req).rate)}else if (*c).div_width==DIV_TV{let x=plltv_div(c,(*req).rate);if x<0{(*req).best_parent_rate}else{x as usize}}else{sp_pll_calc_div(c,(*req).rate) as usize*(*c).brate};(*req).rate=ret;0}
unsafe fn sp_pll_recalc_rate(hw:*mut clk_hw,prate:usize)->usize{let c=hw as *mut sp_pll;let reg=rd((*c).reg);if reg&(1<<(*c).bp_bit)!=0{prate}else if (*c).div_width==DIV_A{PA[(*c).p[0] as usize].rate as usize}else if (*c).div_width==DIV_TV{let r=field_get(MASK_DIVR,rd((*c).reg.add(4)));let reg2=rd((*c).reg.add(8));let m=field_get(MASK_DIVM,reg2)+1;if reg&MASK_SEL_FRA!=0{let s=field_get(MASK_SDM_MOD,reg) as usize;let ph=field_get(MASK_PH_SEL,reg) as usize;let n=field_get(MASK_NFRA,reg);let p=PT[ph];let z=prate>>r;let r0=z as usize*(p[1]+p[2]) as usize/p[0] as usize;let r1=z as usize*(SDM_MOD_VALS[s]-n) as usize/SDM_MOD_VALS[s] as usize/p[4] as usize;(r0-r1)/m as usize}else{let n=field_get(MASK_DIVN,reg2)+1;(prate/m as usize*n as usize)>>r}}else{let mask=(1u32<<(*c).div_width)-1;(*c).brate*(((reg>>(*c).div_shift)&mask)+1) as usize}}
unsafe fn sp_pll_enable(hw:*mut clk_hw)->i32{let c=hw as *mut sp_pll;wr((1<<((*c).pd_bit+16))|(1<<(*c).pd_bit),(*c).reg);0}
unsafe fn sp_pll_disable(hw:*mut clk_hw){let c=hw as *mut sp_pll;wr(1<<((*c).pd_bit+16),(*c).reg)}
unsafe fn sp_pll_is_enabled(hw:*mut clk_hw)->i32{let c=hw as *mut sp_pll;(rd((*c).reg)&(1<<(*c).pd_bit)) as i32}

// Kernel registration, probe, gate setup, device tables, and module metadata remain external-kernel integration points.
extern "C" { pub fn sp7021_clk_probe(pdev:*mut platform_device)->i32; }
#[repr(C)] pub struct platform_device { _private:[u8;0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
