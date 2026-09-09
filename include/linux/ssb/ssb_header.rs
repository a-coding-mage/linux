/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from linux/ssb/ssb.h. Included kernel types are external dependencies. */

use core::ffi::c_void;

#[repr(C)]
pub struct ssb_sprom_core_pwr_info { pub itssi_2g: u8, pub itssi_5g: u8, pub maxpwr_2g: u8, pub maxpwr_5gl: u8, pub maxpwr_5g: u8, pub maxpwr_5gh: u8, pub pa_2g: [u16;4], pub pa_5gl: [u16;4], pub pa_5g: [u16;4], pub pa_5gh: [u16;4] }

#[repr(C)]
pub struct ssb_sprom {
 pub revision:u8, pub il0mac:[u8;6], pub et0mac:[u8;6], pub et1mac:[u8;6], pub et2mac:[u8;6],
 pub et0phyaddr:u8, pub et1phyaddr:u8, pub et2phyaddr:u8, pub et0mdcport:u8, pub et1mdcport:u8, pub et2mdcport:u8,
 pub dev_id:u16, pub board_rev:u16, pub board_num:u16, pub board_type:u16, pub country_code:u8, pub alpha2:[i8;2],
 pub leddc_on_time:u8, pub leddc_off_time:u8, pub ant_available_a:u8, pub ant_available_bg:u8,
 pub pa0b0:u16,pub pa0b1:u16,pub pa0b2:u16,pub pa1b0:u16,pub pa1b1:u16,pub pa1b2:u16,pub pa1lob0:u16,pub pa1lob1:u16,pub pa1lob2:u16,pub pa1hib0:u16,pub pa1hib1:u16,pub pa1hib2:u16,
 pub gpio0:u8,pub gpio1:u8,pub gpio2:u8,pub gpio3:u8,pub maxpwr_bg:u8,pub maxpwr_al:u8,pub maxpwr_a:u8,pub maxpwr_ah:u8,pub itssi_a:u8,pub itssi_bg:u8,pub tri2g:u8,pub tri5gl:u8,pub tri5g:u8,pub tri5gh:u8,
 pub txpid2g:[u8;4],pub txpid5gl:[u8;4],pub txpid5g:[u8;4],pub txpid5gh:[u8;4],pub rxpo2g:i8,pub rxpo5g:i8,pub rssisav2g:u8,pub rssismc2g:u8,pub rssismf2g:u8,pub bxa2g:u8,pub rssisav5g:u8,pub rssismc5g:u8,pub rssismf5g:u8,pub bxa5g:u8,
 pub cck2gpo:u16,pub ofdm2gpo:u32,pub ofdm5glpo:u32,pub ofdm5gpo:u32,pub ofdm5ghpo:u32,pub boardflags:u32,pub boardflags2:u32,pub boardflags3:u32,pub boardflags_lo:u16,pub boardflags_hi:u16,pub boardflags2_lo:u16,pub boardflags2_hi:u16,
 pub core_pwr_info:[ssb_sprom_core_pwr_info;4], pub antenna_gain: ssb_antenna_gain, pub fem:ssb_fem,
 pub mcs2gpo:[u16;8],pub mcs5gpo:[u16;8],pub mcs5glpo:[u16;8],pub mcs5ghpo:[u16;8],pub opo:u8,
 pub rxgainerr2ga:[u8;3],pub rxgainerr5gla:[u8;3],pub rxgainerr5gma:[u8;3],pub rxgainerr5gha:[u8;3],pub rxgainerr5gua:[u8;3],pub noiselvl2ga:[u8;3],pub noiselvl5gla:[u8;3],pub noiselvl5gma:[u8;3],pub noiselvl5gha:[u8;3],pub noiselvl5gua:[u8;3],
 pub regrev:u8,pub txchain:u8,pub rxchain:u8,pub antswitch:u8,pub cddpo:u16,pub stbcpo:u16,pub bw40po:u16,pub bwduppo:u16,pub tempthresh:u8,pub tempoffset:u8,pub rawtempsense:u16,pub measpower:u8,pub tempsense_slope:u8,pub tempcorrx:u8,pub tempsense_option:u8,pub freqoffset_corr:u8,pub iqcal_swp_dis:u8,pub hw_iqcal_en:u8,pub elna2g:u8,pub elna5g:u8,pub phycal_tempdelta:u8,pub temps_period:u8,pub temps_hysteresis:u8,pub measpower1:u8,pub measpower2:u8,pub pcieingress_war:u8,
 pub cckbw202gpo:u16,pub cckbw20ul2gpo:u16,pub legofdmbw202gpo:u32,pub legofdmbw20ul2gpo:u32,pub legofdmbw205glpo:u32,pub legofdmbw20ul5glpo:u32,pub legofdmbw205gmpo:u32,pub legofdmbw20ul5gmpo:u32,pub legofdmbw205ghpo:u32,pub legofdmbw20ul5ghpo:u32,pub mcsbw202gpo:u32,pub mcsbw20ul2gpo:u32,pub mcsbw402gpo:u32,pub mcsbw205glpo:u32,pub mcsbw20ul5glpo:u32,pub mcsbw405glpo:u32,pub mcsbw205gmpo:u32,pub mcsbw20ul5gmpo:u32,pub mcsbw405gmpo:u32,pub mcsbw205ghpo:u32,pub mcsbw20ul5ghpo:u32,pub mcsbw405ghpo:u32,pub mcs32po:u16,pub legofdm40duppo:u16,pub sar2g:u8,pub sar5g:u8,
}
#[repr(C)] pub struct ssb_antenna_gain { pub a0:i8,pub a1:i8,pub a2:i8,pub a3:i8 }
#[repr(C)] pub struct ssb_fem { pub ghz2:ssb_fem_band, pub ghz5:ssb_fem_band }
#[repr(C)] pub struct ssb_fem_band { pub tssipos:u8,pub extpa_gain:u8,pub pdet_range:u8,pub tr_iso:u8,pub antswlut:u8 }
#[repr(C)] pub struct ssb_boardinfo { pub vendor:u16,pub type_:u16 }

#[repr(C)] pub struct ssb_bus_ops { pub read8: Option<unsafe extern "C" fn(*mut ssb_device,u16)->u8>, pub read16: Option<unsafe extern "C" fn(*mut ssb_device,u16)->u16>, pub read32: Option<unsafe extern "C" fn(*mut ssb_device,u16)->u32>, pub write8: Option<unsafe extern "C" fn(*mut ssb_device,u16,u8)>, pub write16: Option<unsafe extern "C" fn(*mut ssb_device,u16,u16)>, pub write32: Option<unsafe extern "C" fn(*mut ssb_device,u16,u32)> }

/* Core-ID values. */
pub const SSB_DEV_CHIPCOMMON:u16=0x800; pub const SSB_DEV_ILINE20:u16=0x801; pub const SSB_DEV_SDRAM:u16=0x803; pub const SSB_DEV_PCI:u16=0x804; pub const SSB_DEV_MIPS:u16=0x805; pub const SSB_DEV_ETHERNET:u16=0x806; pub const SSB_DEV_V90:u16=0x807; pub const SSB_DEV_USB11_HOSTDEV:u16=0x808; pub const SSB_DEV_ADSL:u16=0x809; pub const SSB_DEV_ILINE100:u16=0x80a; pub const SSB_DEV_IPSEC:u16=0x80b; pub const SSB_DEV_PCMCIA:u16=0x80d; pub const SSB_DEV_INTERNAL_MEM:u16=0x80e; pub const SSB_DEV_MEMC_SDRAM:u16=0x80f; pub const SSB_DEV_EXTIF:u16=0x811; pub const SSB_DEV_80211:u16=0x812; pub const SSB_DEV_MIPS_3302:u16=0x816; pub const SSB_DEV_USB11_HOST:u16=0x817; pub const SSB_DEV_USB11_DEV:u16=0x818; pub const SSB_DEV_USB20_HOST:u16=0x819; pub const SSB_DEV_USB20_DEV:u16=0x81a; pub const SSB_DEV_SDIO_HOST:u16=0x81b; pub const SSB_DEV_ROBOSWITCH:u16=0x81c; pub const SSB_DEV_PARA_ATA:u16=0x81d; pub const SSB_DEV_SATA_XORDMA:u16=0x81e; pub const SSB_DEV_ETHERNET_GBIT:u16=0x81f; pub const SSB_DEV_PCIE:u16=0x820; pub const SSB_DEV_MIMO_PHY:u16=0x821; pub const SSB_DEV_SRAM_CTRLR:u16=0x822; pub const SSB_DEV_MINI_MACPHY:u16=0x823; pub const SSB_DEV_ARM_1176:u16=0x824; pub const SSB_DEV_ARM_7TDMI:u16=0x825; pub const SSB_DEV_ARM_CM3:u16=0x82a;
pub const SSB_VENDOR_BROADCOM:u16=0x4243;

#[repr(C)] pub struct ssb_device { pub ops:*const ssb_bus_ops, pub dev:*mut c_void, pub dma_dev:*mut c_void, pub bus:*mut ssb_bus, pub id:ssb_device_id, pub core_index:u8, pub irq:u32, pub drvdata:*mut c_void, pub devtypedata:*mut c_void }
#[repr(C)] pub struct ssb_device_id { pub vendor:u16,pub core:u16,pub revision:u8,pub _pad:u8 }
#[repr(C)] pub struct ssb_driver { pub name:*const i8,pub id_table:*const ssb_device_id,pub probe:Option<unsafe extern "C" fn(*mut ssb_device,*const ssb_device_id)->i32>,pub remove:Option<unsafe extern "C" fn(*mut ssb_device)>,pub suspend:Option<unsafe extern "C" fn(*mut ssb_device,*mut c_void)->i32>,pub resume:Option<unsafe extern "C" fn(*mut ssb_device)->i32>,pub shutdown:Option<unsafe extern "C" fn(*mut ssb_device)>,pub drv:c_void }

#[repr(C)] pub struct ssb_bus { pub mmio:*mut c_void,pub ops:*const ssb_bus_ops,pub mapped_device:*mut ssb_device,pub mapped_pcmcia_seg:u8,pub _sdio_sbaddr:u32,pub bar_lock:c_void,pub bustype:ssb_bustype,pub host_pci:*mut c_void,pub quirks:u32,pub chip_id:u16,pub chip_rev:u8,pub sprom_offset:u16,pub sprom_size:u16,pub chip_package:u8,pub devices:[ssb_device;16],pub nr_devices:u8,pub busnumber:u32,pub boardinfo:ssb_boardinfo,pub sprom:ssb_sprom,pub has_cardbus_slot:bool,pub list:c_void,pub powered_up:bool,pub power_warn_count:i32 }
#[repr(C)] pub struct ssb_init_invariants { pub boardinfo:ssb_boardinfo,pub sprom:ssb_sprom,pub has_cardbus_slot:bool }
#[repr(i32)] pub enum ssb_bustype { SSB_BUSTYPE_SSB, SSB_BUSTYPE_PCI, SSB_BUSTYPE_PCMCIA, SSB_BUSTYPE_SDIO }
#[repr(i32)] pub enum ssb_quirks { SSB_QUIRK_SDIO_READ_AFTER_WRITE32=1 }

pub const SSB_DMA_TRANSLATION_MASK:u32=0xc0000000; pub const SSB_DMA_TRANSLATION_SHIFT:u32=30;
pub const SSB_BOARDVENDOR_BCM:u16=0x14e4; pub const SSB_BOARDVENDOR_DELL:u16=0x1028; pub const SSB_BOARDVENDOR_HP:u16=0x0e11;
pub const SSB_BOARD_BCM94301CB:u16=0x0406; pub const SSB_BOARD_BCM94301MP:u16=0x0407; pub const SSB_BOARD_BU4309:u16=0x040a; pub const SSB_BOARD_BCM94309CB:u16=0x040b; pub const SSB_BOARD_BCM4309MP:u16=0x040c; pub const SSB_BOARD_BU4306:u16=0x0416; pub const SSB_BOARD_BCM94306MP:u16=0x0418; pub const SSB_BOARD_BCM4309G:u16=0x0421; pub const SSB_BOARD_BCM4306CB:u16=0x0417; pub const SSB_BOARD_BCM94306PC:u16=0x0425; pub const SSB_BOARD_BCM94306CBSG:u16=0x042b; pub const SSB_BOARD_PCSG94306:u16=0x042d; pub const SSB_BOARD_BU4704SD:u16=0x042e; pub const SSB_BOARD_BCM94704AGR:u16=0x042f; pub const SSB_BOARD_BCM94308MP:u16=0x0430; pub const SSB_BOARD_BU4318:u16=0x0447; pub const SSB_BOARD_CB4318:u16=0x0448; pub const SSB_BOARD_MPG4318:u16=0x0449; pub const SSB_BOARD_MP4318:u16=0x044a; pub const SSB_BOARD_SD4318:u16=0x044b; pub const SSB_BOARD_BCM94306P:u16=0x044c; pub const SSB_BOARD_BCM94303MP:u16=0x044e; pub const SSB_BOARD_BCM94306MPM:u16=0x0450; pub const SSB_BOARD_BCM94306MPL:u16=0x0453; pub const SSB_BOARD_PC4303:u16=0x0454; pub const SSB_BOARD_BCM94306MPLNA:u16=0x0457; pub const SSB_BOARD_BCM94306MPH:u16=0x045b; pub const SSB_BOARD_BCM94306PCIV:u16=0x045c; pub const SSB_BOARD_BCM94318MPGH:u16=0x0463; pub const SSB_BOARD_BU4311:u16=0x0464; pub const SSB_BOARD_BCM94311MC:u16=0x0465; pub const SSB_BOARD_BCM94311MCAG:u16=0x0466; pub const SSB_BOARD_BU4321:u16=0x046b; pub const SSB_BOARD_BU4321E:u16=0x047c; pub const SSB_BOARD_MP4321:u16=0x046c; pub const SSB_BOARD_CB2_4321:u16=0x046d; pub const SSB_BOARD_CB2_4321_AG:u16=0x0066; pub const SSB_BOARD_MC4321:u16=0x046e; pub const SSB_BOARD_BCM94325DEVBU:u16=0x0490; pub const SSB_BOARD_BCM94325BGABU:u16=0x0491; pub const SSB_BOARD_BCM94325SDGWB:u16=0x0492; pub const SSB_BOARD_BCM94325SDGMDL:u16=0x04aa; pub const SSB_BOARD_BCM94325SDGMDL2:u16=0x04c6; pub const SSB_BOARD_BCM94325SDGMDL3:u16=0x04c9; pub const SSB_BOARD_BCM94325SDABGWBA:u16=0x04e1; pub const SSB_BOARD_BCM94322MC:u16=0x04a4; pub const SSB_BOARD_BCM94322USB:u16=0x04a8; pub const SSB_BOARD_BCM94322HM:u16=0x04b0; pub const SSB_BOARD_BCM94322USB2D:u16=0x04bf; pub const SSB_BOARD_BU4312:u16=0x048a; pub const SSB_BOARD_BCM4312MCGSG:u16=0x04b5;
pub const SSB_CHIPPACK_BCM4712S:u16=1; pub const SSB_CHIPPACK_BCM4712M:u16=2; pub const SSB_CHIPPACK_BCM4712L:u16=0;

#[inline] pub unsafe fn ssb_set_drvdata(dev:*mut ssb_device,data:*mut c_void){(*dev).drvdata=data}
#[inline] pub unsafe fn ssb_get_drvdata(dev:*mut ssb_device)->*mut c_void{(*dev).drvdata}
#[inline] pub unsafe fn ssb_get_devtypedata(dev:*mut ssb_device)->*mut c_void{(*dev).devtypedata}
extern "C" { pub fn ssb_set_devtypedata(dev:*mut ssb_device,data:*mut c_void); pub fn ssb_arch_register_fallback_sprom(cb:Option<unsafe extern "C" fn(*mut ssb_bus,*mut ssb_sprom)->i32>)->i32; }

pub type ssb_invariants_func_t = unsafe extern "C" fn(*mut ssb_bus,*mut ssb_init_invariants)->i32;
extern "C" { pub fn ssb_bus_host_soc_register(bus:*mut ssb_bus,baseaddr:usize)->i32; pub fn ssb_bus_unregister(bus:*mut ssb_bus); pub fn ssb_is_sprom_available(bus:*mut ssb_bus)->bool; pub fn ssb_bus_suspend(bus:*mut ssb_bus)->i32; pub fn ssb_bus_resume(bus:*mut ssb_bus)->i32; pub fn ssb_clockspeed(bus:*mut ssb_bus)->u32; pub fn ssb_device_is_enabled(dev:*mut ssb_device)->i32; pub fn ssb_device_enable(dev:*mut ssb_device,flags:u32); pub fn ssb_device_disable(dev:*mut ssb_device,flags:u32); pub fn ssb_dma_translation(dev:*mut ssb_device)->u32; pub fn ssb_bus_may_powerdown(bus:*mut ssb_bus)->i32; pub fn ssb_bus_powerup(bus:*mut ssb_bus,dynamic_pctl:bool)->i32; pub fn ssb_commit_settings(bus:*mut ssb_bus); pub fn ssb_admatch_base(adm:u32)->u32; pub fn ssb_admatch_size(adm:u32)->u32; }

#[inline] pub unsafe fn ssb_read8(dev:*mut ssb_device,offset:u16)->u8 { ((*(*dev).ops).read8.unwrap())(dev,offset) }
#[inline] pub unsafe fn ssb_read16(dev:*mut ssb_device,offset:u16)->u16 { ((*(*dev).ops).read16.unwrap())(dev,offset) }
#[inline] pub unsafe fn ssb_read32(dev:*mut ssb_device,offset:u16)->u32 { ((*(*dev).ops).read32.unwrap())(dev,offset) }
#[inline] pub unsafe fn ssb_write8(dev:*mut ssb_device,offset:u16,value:u8) { ((*(*dev).ops).write8.unwrap())(dev,offset,value) }
#[inline] pub unsafe fn ssb_write16(dev:*mut ssb_device,offset:u16,value:u16) { ((*(*dev).ops).write16.unwrap())(dev,offset,value) }
#[inline] pub unsafe fn ssb_write32(dev:*mut ssb_device,offset:u16,value:u32) { ((*(*dev).ops).write32.unwrap())(dev,offset,value) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
