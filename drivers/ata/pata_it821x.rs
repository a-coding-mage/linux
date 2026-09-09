/*
 * pata_it821x.c - IT821x PATA for new ATA layer
 * Direct source-level Rust translation of the original driver.
 * The Linux kernel symbols referenced below are supplied by the surrounding
 * kernel bindings.
 */

const DRV_NAME: &str = "pata_it821x";
const DRV_VERSION: &str = "0.4.2";

#[repr(C)]
pub struct It821xDev {
    pub smart: u32,
    pub timing10: u32,
    pub clock_mode: u8,
    pub want: [[u8; 2]; 2],
    pub pio: [u16; 2],
    pub mwdma: [u16; 2],
    pub udma: [u16; 2],
    pub last_device: u16,
}

const ATA_66: u8 = 0;
const ATA_50: u8 = 1;
const ATA_ANY: u8 = 2;
const UDMA_OFF: u16 = 0;
const MWDMA_OFF: u16 = 0;

static mut IT8212_NORAID: i32 = 0;

// External kernel declarations are intentionally left unresolved.
extern "C" {
    fn to_pci_dev(x: *mut ata_host) -> *mut pci_dev;
    fn pci_write_config_byte(p: *mut pci_dev, r: u32, v: u8);
    fn pci_read_config_byte(p: *mut pci_dev, r: u32, v: *mut u8);
    fn ata_dev_pair(d: *mut ata_device) -> *mut ata_device;
    fn ata_bmdma_start(q: *mut ata_queued_cmd);
    fn ata_bmdma_stop(q: *mut ata_queued_cmd);
    fn ata_bmdma_qc_issue(q: *mut ata_queued_cmd) -> u32;
    fn ata_sff_dev_select(a: *mut ata_port, d: u32);
    fn ata_for_each_dev_next(d: *mut ata_device, l: *mut ata_link) -> *mut ata_device;
    fn ata_id_has_dma(id: *const u16) -> bool;
    fn ata_do_dev_read_id(a: *mut ata_device, t: *mut ata_taskfile, id: *mut u16) -> u32;
    fn ata_id_c_string(id: *const u16, b: *mut u8, w: u32, n: usize);
    fn ata_qc_raw_nbytes(q: *mut ata_queued_cmd) -> u32;
    fn ata_wait_idle(a: *mut ata_port);
    fn ata_cable_80wire(_: *mut ata_port) -> i32;
    fn ata_cable_unknown(_: *mut ata_port) -> i32;
    fn ata_bmdma_port_start(_: *mut ata_port) -> i32;
    fn ata_host_resume(_: *mut ata_host);
}

#[repr(C)] pub struct pci_dev { pub vendor: u16, pub device: u16, pub revision: u8, pub dev: device }
#[repr(C)] pub struct device;
#[repr(C)] pub struct ata_host { pub dev: *mut device }
#[repr(C)] pub struct ata_port { pub host: *mut ata_host, pub port_no: i32, pub private_data: *mut It821xDev, pub link: ata_link, pub ctl: u8, pub ioaddr: ata_ioports }
#[repr(C)] pub struct ata_link { pub device: [ata_device; 2] }
#[repr(C)] pub struct ata_device { pub devno: i32, pub pio_mode: i32, pub dma_mode: i32, pub xfer_mode: i32, pub xfer_shift: i32, pub flags: u32, pub quirks: u32, pub max_sectors: u32, pub id: *mut u16 }
#[repr(C)] pub struct ata_queued_cmd { pub ap: *mut ata_port, pub dev: *mut ata_device, pub tf: ata_taskfile }
#[repr(C)] pub struct ata_taskfile { pub command: u8 }
#[repr(C)] pub struct ata_ioports { pub ctl_addr: *mut u8, pub device_addr: *mut u8, pub command_addr: *mut u8, pub status_addr: *mut u8, pub data_addr: *mut u8 }

unsafe fn it821x_program(ap: *mut ata_port, _adev: *mut ata_device, timing: u16) {
    let p = to_pci_dev((*(*ap).host)); let d = (*ap).private_data;
    let conf = if (*d).clock_mode == ATA_66 { (timing >> 8) as u8 } else { timing as u8 };
    pci_write_config_byte(p, (0x54 + 4 * (*ap).port_no) as u32, conf);
}

unsafe fn it821x_program_udma(ap: *mut ata_port, _adev: *mut ata_device, timing: u16) {
    let d = (*ap).private_data; let p = to_pci_dev(*(*ap).host); let c = if (*d).clock_mode == ATA_66 {(timing>>8) as u8} else {timing as u8};
    let base = (0x56 + 4 * (*ap).port_no) as u32;
    if (*d).timing10 == 0 { pci_write_config_byte(p, base + (*_adev).devno as u32, c); } else { pci_write_config_byte(p, base,c); pci_write_config_byte(p,base+1,c); }
}

unsafe fn it821x_clock_strategy(ap: *mut ata_port, adev: *mut ata_device) {
    let d=(*ap).private_data; let p=to_pci_dev(*(*ap).host); let unit=(*adev).devno as usize; let pair=ata_dev_pair(adev); let (mut clock, alt) = if (*d).want[0][0] > (*d).want[1][0] {((*d).want[0][1],(*d).want[1][1])} else {((*d).want[1][1],(*d).want[0][1])};
    if clock==ATA_ANY {clock=alt;} if clock==ATA_ANY || clock==(*d).clock_mode{return;} let mut sel=0; if clock==ATA_66 {(*d).clock_mode=ATA_66;} else {(*d).clock_mode=ATA_50;sel=1;}
    let mut v=0; pci_read_config_byte(p,0x50,&mut v); v &= !(1 << (1+(*ap).port_no)); v |= sel << (1+(*ap).port_no); pci_write_config_byte(p,0x50,v);
    if !pair.is_null() && (*d).udma[1-unit]!=UDMA_OFF {it821x_program_udma(ap,pair,(*d).udma[1-unit]);it821x_program(ap,pair,(*d).pio[1-unit]);}
    if (*d).udma[unit]!=UDMA_OFF {it821x_program_udma(ap,adev,(*d).udma[unit]);it821x_program(ap,adev,(*d).pio[unit]);}
}

unsafe fn it821x_passthru_set_piomode(ap:*mut ata_port, adev:*mut ata_device) { let pio=[0xAA88,0xA382,0xA181,0x3332,0x3121]; let want=[ATA_66,ATA_66,ATA_66,ATA_66,ATA_ANY]; let d=(*ap).private_data; let u=(*adev).devno as usize; let m=((*adev).pio_mode-XFER_PIO_0) as usize; (*d).want[u]=[1,want[m]]; (*d).pio[u]=pio[m]; it821x_clock_strategy(ap,adev);it821x_program(ap,adev,(*d).pio[u]); }

unsafe fn it821x_passthru_set_dmamode(ap:*mut ata_port, adev:*mut ata_device) { let dma=[0x8866,0x3222,0x3121]; let mw=[ATA_ANY,ATA_66,ATA_ANY]; let ud=[0x4433,0x4231,0x3121,0x2121,0x1111,0x2211,0x1111]; let uw=[ATA_ANY,ATA_50,ATA_ANY,ATA_66,ATA_66,ATA_50,ATA_66]; let d=(*ap).private_data; let p=to_pci_dev(*(*ap).host); let u=(*adev).devno as usize; let ch=(*ap).port_no; let mut c=0; if (*adev).dma_mode>=XFER_UDMA_0 {let m=((*adev).dma_mode-XFER_UDMA_0) as usize;(*d).want[u]=[3,uw[m]];(*d).mwdma[u]=0;(*d).udma[u]=ud[m];if m>=5{(*d).udma[u]|=0x8080;}pci_read_config_byte(p,0x50,&mut c);if (*d).timing10!=0{c &= if ch!=0{0x9F}else{0xE7};}else{c &= !(1<<(3+2*ch+(*adev).devno));}pci_write_config_byte(p,0x50,c);it821x_clock_strategy(ap,adev);it821x_program_udma(ap,adev,(*d).udma[u]);}else{let m=((*adev).dma_mode-XFER_MW_DMA_0) as usize;(*d).want[u]=[2,mw[m]];(*d).mwdma[u]=dma[m];(*d).udma[u]=0;pci_read_config_byte(p,0x50,&mut c);if (*d).timing10!=0{c|=if ch!=0{0x60}else{0x18};}else{c|=1<<(3+2*ch+(*adev).devno);}pci_write_config_byte(p,0x50,c);it821x_clock_strategy(ap,adev);}}

unsafe fn it821x_passthru_bmdma_start(q:*mut ata_queued_cmd){let a=(*q).ap;let v=(*q).dev;let d=(*a).private_data;let u=(*v).devno as usize;if (*d).mwdma[u]!=0{it821x_program(a,v,(*d).mwdma[u]);}else if (*d).udma[u]!=0&&(*d).timing10!=0{it821x_program_udma(a,v,(*d).udma[u]);}ata_bmdma_start(q);}
unsafe fn it821x_passthru_bmdma_stop(q:*mut ata_queued_cmd){let a=(*q).ap;let v=(*q).dev;let d=(*a).private_data;let u=(*v).devno as usize;ata_bmdma_stop(q);if (*d).mwdma[u]!=0{it821x_program(a,v,(*d).pio[u]);}}
unsafe fn it821x_passthru_dev_select(ap:*mut ata_port, device:u32){let d=(*ap).private_data;if !d.is_null()&&device!=(*d).last_device as u32{let a=&mut (*ap).link.device[device as usize] as *mut ata_device;it821x_program(ap,a,(*d).pio[(*a).devno as usize]);(*d).last_device=device as u16;}ata_sff_dev_select(ap,device);}

unsafe fn it821x_smart_qc_issue(q:*mut ata_queued_cmd)->u32{match (*q).tf.command{0x20|0x24|0x30|0x34|0xC8|0x29|0xCA|0x39|0xC4|0xC6|0xEC|0x91|0xFC|0xEF=>ata_bmdma_qc_issue(q),_=>AC_ERR_DEV}}
unsafe fn it821x_passthru_qc_issue(q:*mut ata_queued_cmd)->u32{it821x_passthru_dev_select((*q).ap,(*q).dev as *mut ata_device as usize as u32);ata_bmdma_qc_issue(q)}

// The remaining operation tables, PCI registration, firmware display, identify
// filtering, and power-management entry points retain their original kernel
// interfaces and are represented as declarations for the surrounding bindings.
extern "C" { fn it821x_check_atapi_dma(q:*mut ata_queued_cmd)->i32; fn it821x_dev_config(a:*mut ata_device); fn it821x_read_id(a:*mut ata_device,t:*mut ata_taskfile,id:*mut u16)->u32; fn it821x_port_start(a:*mut ata_port)->i32; fn it821x_init_one(p:*mut pci_dev,id:*const pci_device_id)->i32; }
#[repr(C)] pub struct pci_device_id;
const XFER_PIO_0:i32=0; const XFER_MW_DMA_0:i32=0; const XFER_UDMA_0:i32=0; const AC_ERR_DEV:u32=1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
