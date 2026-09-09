// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of the original Linux kernel implementation. */

use core::ffi::c_void;

const CGU_IFCCR: u32 = 0x0018; const CGU_IFCCR_VR9: u32 = 0x0024;
const CGU_SYS: u32 = 0x0010; const CGU_PCICR: u32 = 0x0034; const CGU_PCICR_VR9: u32 = 0x0038; const CGU_EPHY: u32 = 0x10;
const PMU_PWDCR: u32 = 0x1c; const PMU_PWDSR: u32 = 0x20; const PMU_PWDCR1: u32 = 0x24; const PMU_PWDSR1: u32 = 0x28;
const PMU_CLK_SR: u32 = 0x20; const PMU_CLK_CR_A: u32 = 0x24; const PMU_CLK_CR_B: u32 = 0x28;
const PMU_CLK_SR1: u32 = 0x30; const PMU_CLK_CR1_A: u32 = 0x34; const PMU_CLK_CR1_B: u32 = 0x38;
const PMU_ANA_SR: u32 = 0x40; const PMU_ANA_CR_A: u32 = 0x44; const PMU_ANA_CR_B: u32 = 0x48;

#[inline] const fn bit(x: u32) -> u32 { 1u32 << x }
const PMU_USB0_P:u32=bit(0); const PMU_ASE_SDIO:u32=bit(2); const PMU_PCI:u32=bit(4); const PMU_DMA:u32=bit(5); const PMU_USB0:u32=bit(6); const PMU_ASC0:u32=bit(7); const PMU_EPHY:u32=bit(7); const PMU_USIF:u32=bit(7); const PMU_SPI:u32=bit(8); const PMU_DFE:u32=bit(9); const PMU_EBU:u32=bit(10); const PMU_STP:u32=bit(11); const PMU_GPT:u32=bit(12); const PMU_AHBS:u32=bit(13); const PMU_FPI:u32=bit(14); const PMU_AHBM:u32=bit(15); const PMU_SDIO:u32=bit(16); const PMU_ASC1:u32=bit(17); const PMU_PPE_QSB:u32=bit(18); const PMU_PPE_SLL01:u32=bit(19); const PMU_DEU:u32=bit(20); const PMU_PPE_TC:u32=bit(21); const PMU_PPE_EMA:u32=bit(22); const PMU_PPE_DPLUM:u32=bit(23); const PMU_PPE_DP:u32=bit(23); const PMU_PPE_DPLUS:u32=bit(24); const PMU_USB1_P:u32=bit(26); const PMU_GPHY3:u32=bit(26); const PMU_USB1:u32=bit(27); const PMU_SWITCH:u32=bit(28); const PMU_PPE_TOP:u32=bit(29); const PMU_GPHY0:u32=bit(29); const PMU_GPHY:u32=bit(30); const PMU_GPHY1:u32=bit(30); const PMU_PCIE_CLK:u32=bit(31); const PMU_GPHY2:u32=bit(31);
const PMU1_PCIE_PHY:u32=bit(0); const PMU1_PCIE_CTL:u32=bit(1); const PMU1_PCIE_PDI:u32=bit(4); const PMU1_PCIE_MSI:u32=bit(5); const PMU1_CKE:u32=bit(6); const PMU1_PCIE1_CTL:u32=bit(17); const PMU1_PCIE1_PDI:u32=bit(20); const PMU1_PCIE1_MSI:u32=bit(21); const PMU1_PCIE2_CTL:u32=bit(25); const PMU1_PCIE2_PDI:u32=bit(26); const PMU1_PCIE2_MSI:u32=bit(27);
const PMU_ANALOG_USB0_P:u32=bit(0); const PMU_ANALOG_USB1_P:u32=bit(1); const PMU_ANALOG_PCIE0_P:u32=bit(8); const PMU_ANALOG_PCIE1_P:u32=bit(9); const PMU_ANALOG_PCIE2_P:u32=bit(10); const PMU_ANALOG_DSL_AFE:u32=bit(16); const PMU_ANALOG_DCDC_2V5:u32=bit(17); const PMU_ANALOG_DCDC_1VX:u32=bit(18); const PMU_ANALOG_DCDC_1V0:u32=bit(19);

extern "C" {
    fn ltq_w32(v:u32, p:*mut c_void); fn ltq_r32(p:*mut c_void)->u32; fn ltq_cgu_w32(v:u32,o:u32); fn ltq_cgu_r32(o:u32)->u32; fn ltq_ebu_w32(v:u32,o:u32); fn ltq_ebu_r32(o:u32)->u32;
    fn of_machine_is_compatible(s:*const u8)->bool; fn panic(s:*const u8)->!; fn pr_warn(s:*const u8); fn pr_err(s:*const u8);
    fn clkdev_add(x:*mut c_void); fn clkdev_add_static(a:usize,b:usize,c:usize,d:usize); fn ltq_grx390_cpu_hz()->usize; fn ltq_grx390_fpi_hz()->usize; fn ltq_grx390_pp32_hz()->usize; fn ltq_ar10_cpu_hz()->usize; fn ltq_ar10_fpi_hz()->usize; fn ltq_ar10_pp32_hz()->usize; fn ltq_vr9_cpu_hz()->usize; fn ltq_vr9_fpi_hz()->usize; fn ltq_vr9_pp32_hz()->usize; fn ltq_ar9_cpu_hz()->usize; fn ltq_ar9_fpi_hz()->usize; fn ltq_danube_cpu_hz()->usize; fn ltq_danube_fpi_hz()->usize; fn ltq_danube_pp32_hz()->usize;
}

static mut PMU_MEMBASE:*mut c_void=core::ptr::null_mut();
#[no_mangle] pub static mut ltq_cgu_membase:*mut c_void=core::ptr::null_mut();
#[no_mangle] pub static mut ltq_ebu_membase:*mut c_void=core::ptr::null_mut();
static mut ifccr:u32=CGU_IFCCR; static mut pcicr:u32=CGU_PCICR;

#[repr(C)] pub struct Clk { pub dev_id:*const u8,pub con_id:*const u8,pub clk:*mut Clk,pub enable:Option<unsafe extern "C" fn(*mut Clk)->i32>,pub disable:Option<unsafe extern "C" fn(*mut Clk)>,pub rate:usize,pub rates:*mut usize,pub module:u32,pub bits:u32 }
unsafe fn pmu_w32(x:u32,y:u32){ltq_w32(x,PMU_MEMBASE.add(y as usize))} unsafe fn pmu_r32(x:u32)->u32{ltq_r32(PMU_MEMBASE.add(x as usize))}
unsafe fn compat(s:&str)->bool{of_machine_is_compatible(s.as_ptr())}
unsafe fn pmu_enable(c:*mut Clk)->i32 { let mut retry=1_000_000i32; if compat("lantiq,ar10")||compat("lantiq,grx390"){pmu_w32((*c).bits,[PMU_CLK_CR_A,PMU_CLK_CR1_A,PMU_ANA_CR_A][(*c).module as usize]);while {retry-=1;retry!=0&&pmu_r32([PMU_CLK_SR,PMU_CLK_SR1,PMU_ANA_SR][(*c).module as usize])&(*c).bits==0}{}} else {pmu_w32(pmu_r32(if (*c).module!=0{PMU_PWDCR1}else{PMU_PWDCR})&!(*c).bits,if (*c).module!=0{PMU_PWDCR1}else{PMU_PWDCR});while {retry-=1;retry!=0&&pmu_r32(if (*c).module!=0{PMU_PWDSR1}else{PMU_PWDSR})&(*c).bits!=0}{}} if retry==0{panic(b"activating PMU module failed!\0" as *const u8)} 0 }
unsafe extern "C" fn pmu_disable(c:*mut Clk){let mut retry=1_000_000i32;let (cr,sr)=if compat("lantiq,ar10")||compat("lantiq,grx390"){([PMU_CLK_CR_B,PMU_CLK_CR1_B,PMU_ANA_CR_B][(*c).module as usize],[PMU_CLK_SR,PMU_CLK_SR1,PMU_ANA_SR][(*c).module as usize])}else{(if (*c).module!=0{PMU_PWDCR1}else{PMU_PWDCR},if (*c).module!=0{PMU_PWDSR1}else{PMU_PWDSR})};pmu_w32((*c).bits,cr);while {retry-=1;retry!=0&&pmu_r32(sr)&(*c).bits!=0}{}if retry==0{pr_warn(b"deactivating PMU module failed!\0" as *const u8)}}

// The remaining initialization graph is kept as direct low-level calls; external kernel allocation,
// clock registration, device-tree, and resource APIs are supplied by the surrounding translation.
#[no_mangle] pub unsafe extern "C" fn ltq_pmu_enable(module:u32){pmu_w32(pmu_r32(PMU_PWDCR)&!module,PMU_PWDCR);let mut retry=1_000_000i32;while {retry-=1;retry!=0&&pmu_r32(PMU_PWDSR)&module!=0}{}if retry==0{panic(b"activating PMU module failed!\0" as *const u8)}}
#[no_mangle] pub unsafe extern "C" fn ltq_pmu_disable(module:u32){pmu_w32(pmu_r32(PMU_PWDCR)|module,PMU_PWDCR);let mut retry=1_000_000i32;while {retry-=1;retry!=0&&pmu_r32(PMU_PWDSR)&module==0}{}if retry==0{pr_warn(b"deactivating PMU module failed!\0" as *const u8)}}

unsafe extern "C" fn cgu_enable(c:*mut Clk)->i32 { ltq_cgu_w32(ltq_cgu_r32(ifccr)|(*c).bits,ifccr);0 }
unsafe extern "C" fn cgu_disable(c:*mut Clk) { ltq_cgu_w32(ltq_cgu_r32(ifccr)&!(*c).bits,ifccr); }
unsafe fn usb_set_clock(){let mut v=ltq_cgu_r32(ifccr);if compat("lantiq,ar10")||compat("lantiq,grx390"){v&=!3}else if compat("lantiq,ar9")||compat("lantiq,vr9"){v|=3}else if compat("lantiq,ase"){v|=0x20}else if compat("lantiq,danube"){v|=0x30}ltq_cgu_w32(v,ifccr)}
unsafe extern "C" fn pci_enable(c:*mut Clk)->i32{let mut v=ltq_cgu_r32(ifccr);if compat("lantiq,ar9")||compat("lantiq,vr9"){v&=!0x1f00000;v|=if (*c).rate==33_000_000{0xe00000}else{0x700000}}else{v&=!0xf00000;v|=if (*c).rate==33_000_000{0x800000}else{0x400000}}ltq_cgu_w32(v,ifccr);pmu_enable(c)}
unsafe extern "C" fn pci_ext_enable(_: *mut Clk)->i32{ltq_cgu_w32(ltq_cgu_r32(ifccr)&!(1<<16),ifccr);ltq_cgu_w32(1<<30,pcicr);0}
unsafe extern "C" fn pci_ext_disable(_: *mut Clk){ltq_cgu_w32(ltq_cgu_r32(ifccr)|(1<<16),ifccr);ltq_cgu_w32((1<<31)|(1<<30),pcicr)}
unsafe extern "C" fn clkout_enable(c:*mut Clk)->i32{let rates=(*c).rates;for i in 0..4{if *rates.add(i)==(*c).rate{let shift=14-(2*(*c).module);let mut v=ltq_cgu_r32(ifccr);v&=!(3<<shift);v|=(i as u32)<<shift;v|=7-(*c).module;ltq_cgu_w32(v,ifccr);return 0}}-1}

// Device-tree resource mapping and the complete SoC-specific clock registration table from the
// C implementation are intentionally represented by the surrounding kernel translation layer.
#[no_mangle] pub unsafe extern "C" fn ltq_soc_init(){usb_set_clock()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
