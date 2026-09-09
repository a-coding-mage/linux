/* Intel 82975X Memory Controller kernel module (source-level Rust translation). */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const EDAC_MOD_STR: &str = "i82975x_edac";
const PCI_DEVICE_ID_INTEL_82975_0: u16 = 0x277c;
const I82975X_NR_DIMMS: usize = 8;
const I82975X_DRB_SHIFT: u32 = 25;
const I82975X_EAP: u16 = 0x58;
const I82975X_DERRSYN: u16 = 0x5c;
const I82975X_DES: u16 = 0x5d;
const I82975X_ERRSTS: u16 = 0xc8;
const I82975X_ERRCMD: u16 = 0xca;
const I82975X_SMICMD: u16 = 0xcc;
const I82975X_SCICMD: u16 = 0xce;
const I82975X_XEAP: u16 = 0xfc;
const I82975X_MCHBAR: u16 = 0x44;
const I82975X_DRB: usize = 0x100;
const I82975X_DRB_CH0R0: usize = 0x100;
const I82975X_DRB_CH0R1: usize = 0x101;
const I82975X_DRB_CH0R2: usize = 0x102;
const I82975X_DRB_CH0R3: usize = 0x103;
const I82975X_DRB_CH1R0: usize = 0x180;
const I82975X_DRB_CH1R1: usize = 0x181;
const I82975X_DRB_CH1R2: usize = 0x182;
const I82975X_DRB_CH1R3: usize = 0x183;
const I82975X_DRA: usize = 0x108;
const I82975X_DRA_CH0R01: usize = 0x108;
const I82975X_DRA_CH0R23: usize = 0x109;
const I82975X_DRA_CH1R01: usize = 0x188;
const I82975X_DRA_CH1R23: usize = 0x189;
const I82975X_BNKARC: usize = 0x10e;
const I82975X_C0BNKARC: usize = 0x10e;
const I82975X_C1BNKARC: usize = 0x18e;
const I82975X_DRC: usize = 0x120;
const I82975X_DRC_CH0M0: usize = 0x120;
const I82975X_DRC_CH1M0: usize = 0x1a0;
const I82975X_DRC_M1: usize = 0x124;
const I82975X_DRC_CH0M1: usize = 0x124;
const I82975X_DRC_CH1M1: usize = 0x1a4;

type __iomem = c_void;
type u8 = core::primitive::u8;
type u16 = core::primitive::u16;
type u32 = core::primitive::u32;
type c_int = i32;

#[repr(C)] pub struct pci_dev { pub dev: device, }
#[repr(C)] pub struct device;
#[repr(C)] pub struct pci_device_id { pub driver_data: usize }
#[repr(C)] pub struct mem_ctl_info {
    pub pdev: *mut device, pub pvt_info: *mut c_void, pub nr_csrows: usize,
    pub csrows: *mut *mut csrow_info, pub mtype_cap: u32, pub edac_ctl_cap: u32,
    pub edac_cap: u32, pub mod_name: *const u8, pub ctl_name: *const u8,
    pub dev_name: *const u8, pub edac_check: Option<unsafe extern "C" fn(*mut mem_ctl_info)>,
    pub ctl_page_to_phys: *const c_void, pub scrub_mode: u32,
}
#[repr(C)] pub struct csrow_info { pub nr_channels: usize, pub channels: *mut *mut channel_info, pub first_page: usize, pub last_page: usize }
#[repr(C)] pub struct channel_info { pub dimm: *mut dimm_info }
#[repr(C)] pub struct dimm_info { pub nr_pages: u32, pub label: [u8; 32], pub grain: u32, pub dtype: u32, pub mtype: u32, pub edac_mode: u32 }
#[repr(C)] pub struct edac_mc_layer { pub type_: u32, pub size: usize, pub is_virt_csrow: bool }
#[repr(C)] pub struct pci_driver { pub name: *const u8, pub probe: Option<unsafe extern "C" fn(*mut pci_dev, *const pci_device_id) -> c_int>, pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>, pub id_table: *const pci_device_id }

#[repr(C)] pub struct i82975x_pvt { pub mch_window: *mut __iomem }
#[repr(C)] pub struct i82975x_dev_info { pub ctl_name: *const u8 }
#[repr(C)] pub struct i82975x_error_info { pub errsts: u16, pub eap: u32, pub des: u8, pub derrsyn: u8, pub errsts2: u16, pub chan: u8, pub xeap: u8 }
#[repr(u32)] enum i82975x_chips { I82975X = 0 }

extern "C" {
    fn to_pci_dev(dev: *mut device) -> *mut pci_dev;
    fn pci_read_config_word(p: *mut pci_dev, r: u16, v: *mut u16);
    fn pci_read_config_dword(p: *mut pci_dev, r: u16, v: *mut u32);
    fn pci_read_config_byte(p: *mut pci_dev, r: u16, v: *mut u8);
    fn pci_write_bits16(p: *mut pci_dev, r: u16, mask: u16, val: u16);
    fn readb(p: *mut __iomem) -> u8; fn readl(p: *mut __iomem) -> u32;
    fn ioremap(addr: u32, size: usize) -> *mut __iomem; fn iounmap(p: *mut __iomem);
    fn edac_mc_find_csrow_by_page(m: *mut mem_ctl_info, page: usize) -> c_int;
    fn edac_mc_handle_error(kind: u32, m: *mut mem_ctl_info, count: usize, page: usize, off: usize, syndrome: u8, row: c_int, chan: c_int, layer: c_int, msg: *const u8, detail: *const u8);
    fn edac_mc_alloc(a: usize, n: usize, l: *const edac_mc_layer, sz: usize) -> *mut mem_ctl_info;
    fn edac_mc_add_mc(m: *mut mem_ctl_info) -> c_int; fn edac_mc_free(m: *mut mem_ctl_info);
    fn edac_mc_del_mc(d: *mut device) -> *mut mem_ctl_info;
    fn pci_enable_device(p: *mut pci_dev) -> c_int; fn pci_dev_get(p: *mut pci_dev) -> *mut pci_dev; fn pci_dev_put(p: *mut pci_dev);
    fn pci_register_driver(d: *mut pci_driver) -> c_int; fn pci_unregister_driver(d: *mut pci_driver);
    fn pci_get_device(v: u16, d: u16, from: *mut pci_dev) -> *mut pci_dev;
    fn pci_name(p: *mut pci_dev) -> *const u8; fn opstate_init();
}

static mut MCI_PDEV: *mut pci_dev = core::ptr::null_mut();
static mut I82975X_REGISTERED: c_int = 1;
static I82975X_DEVS: [i82975x_dev_info; 1] = [i82975x_dev_info { ctl_name: b"i82975x\0".as_ptr() }];

unsafe fn i82975x_get_error_info(mci: *mut mem_ctl_info, info: *mut i82975x_error_info) {
    let pdev = to_pci_dev((*mci).pdev); pci_read_config_word(pdev,I82975X_ERRSTS,&mut (*info).errsts); pci_read_config_dword(pdev,I82975X_EAP,&mut (*info).eap); pci_read_config_byte(pdev,I82975X_XEAP,&mut (*info).xeap); pci_read_config_byte(pdev,I82975X_DES,&mut (*info).des); pci_read_config_byte(pdev,I82975X_DERRSYN,&mut (*info).derrsyn); pci_read_config_word(pdev,I82975X_ERRSTS,&mut (*info).errsts2); pci_write_bits16(pdev,I82975X_ERRSTS,3,3);
    if (*info).errsts2 & 3 == 0 { return; } if ((*info).errsts ^ (*info).errsts2) & 3 != 0 { pci_read_config_dword(pdev,I82975X_EAP,&mut (*info).eap); pci_read_config_byte(pdev,I82975X_XEAP,&mut (*info).xeap); pci_read_config_byte(pdev,I82975X_DES,&mut (*info).des); pci_read_config_byte(pdev,I82975X_DERRSYN,&mut (*info).derrsyn); }
}

unsafe fn dual_channel_active(w: *mut __iomem) -> c_int { let mut dual = 1; for row in 0..4 { if dual == 0 { break; } let a=readb(w.add(I82975X_DRB+row)); let b=readb(w.add(I82975X_DRB+row+0x80)); dual = (a==b) as c_int; } dual }

unsafe fn i82975x_check(mci: *mut mem_ctl_info) { let mut info = core::mem::zeroed::<i82975x_error_info>(); i82975x_get_error_info(mci,&mut info); }

unsafe fn i82975x_process_error_info(mci:*mut mem_ctl_info, i:*mut i82975x_error_info, handle:c_int)->c_int {
    if (*i).errsts2&3==0{return 0} if handle==0{return 1}
    let mut page=(*i).eap as usize; page>>=1; if (*i).xeap&1!=0 {page|=0x80000000}; page>>=12;
    let row=edac_mc_find_csrow_by_page(mci,page); if row<0{return 0}
    let cs=*(*mci).csrows.add(row as usize); let chan=if (*cs).nr_channels==1{0}else{((*i).eap&1) as c_int};
    let off=((*i).eap as usize)&((1usize<<12)-(1usize<<7)); let kind=if (*i).errsts&2!=0{1}else{0};
    edac_mc_handle_error(kind,mci,1,page,off,if kind==0{(*i).derrsyn}else{0},row,chan,-1,b"i82975x\0".as_ptr(),b"\0".as_ptr()); 1
}

unsafe fn i82975x_init_csrows(mci:*mut mem_ctl_info, w:*mut __iomem) {
    let mut last=0u32;
    for index in 0..(*mci).nr_csrows { let cs=*(*mci).csrows.add(index); let mut size=readb(w.add(I82975X_DRB+index+if index>=4{0x80}else{0})) as u32; size<<=I82975X_DRB_SHIFT-12; if (*cs).nr_channels>1{size<<=1}; let pages=size-last; if pages==0{continue}
        for chan in 0..(*cs).nr_channels { let d=(*(*cs).channels.add(chan)).dimm; (*d).nr_pages=pages/(*cs).nr_channels as u32; (*d).grain=1<<7; (*d).dtype=8; (*d).mtype=2; (*d).edac_mode=1; } (*cs).first_page=last as usize; (*cs).last_page=(size-1) as usize; last=size;
    }
}

unsafe extern "C" fn i82975x_probe1(pdev:*mut pci_dev, _idx:c_int)->c_int { let mut bar=0u32; pci_read_config_dword(pdev,I82975X_MCHBAR,&mut bar); if bar&1==0{return -19}; let w=ioremap(bar&0xffffc000,0x1000); if w.is_null(){return -19}; let mut drc0=0; pci_read_config_dword(pdev,I82975X_DRC_CH0M0,&mut drc0); if (drc0>>21)&3!=1 { iounmap(w); return -19 }; iounmap(w); 0 }
unsafe extern "C" fn i82975x_init_one(p:*mut pci_dev,e:*const pci_device_id)->c_int { if pci_enable_device(p)<0{return -5}; let r=i82975x_probe1(p,(*e).driver_data as c_int); if MCI_PDEV.is_null(){MCI_PDEV=pci_dev_get(p)} r }
unsafe extern "C" fn i82975x_remove_one(p:*mut pci_dev) { let m=edac_mc_del_mc(&mut (*p).dev); if !m.is_null(){edac_mc_free(m)} }

static I82975X_PCI_TBL:[pci_device_id;2]=[pci_device_id{driver_data:0},pci_device_id{driver_data:0}];
static mut I82975X_DRIVER:pci_driver=pci_driver{name:b"i82975x_edac\0".as_ptr(),probe:Some(i82975x_init_one),remove:Some(i82975x_remove_one),id_table:I82975X_PCI_TBL.as_ptr()};
unsafe extern "C" fn i82975x_init()->c_int { opstate_init(); let r=pci_register_driver(&mut I82975X_DRIVER); if r<0{pci_dev_put(MCI_PDEV)} r }
unsafe extern "C" fn i82975x_exit(){pci_unregister_driver(&mut I82975X_DRIVER); if I82975X_REGISTERED==0{i82975x_remove_one(MCI_PDEV);pci_dev_put(MCI_PDEV)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
