/* SPDX-License-Identifier: GPL-2.0 */
/* Header file for DIO boards for the HP300 architecture. */

/* C dependencies: linux/device.h and asm/hp300hw.h. */

pub type DioId = u16;

#[repr(C)]
pub struct DioDev {
    pub bus: *mut DioBus,
    pub id: DioId,
    pub scode: i32,
    pub driver: *mut DioDriver,
    pub dev: Device,
    pub ipl: u8,
    pub name: [i8; 64],
    pub resource: Resource,
}

#[repr(C)]
pub struct DioBus {
    pub devices: ListHead,
    pub num_resources: u32,
    pub resources: [Resource; 2],
    pub dev: Device,
    pub name: [i8; 10],
}

extern "C" {
    pub static mut dio_bus: DioBus;
    pub static dio_bus_type: BusType;
}

#[repr(C)]
pub struct DioDeviceId {
    pub id: DioId,
    pub driver_data: usize,
}

#[repr(C)]
pub struct DioDriver {
    pub node: ListHead,
    pub name: *mut i8,
    pub id_table: *const DioDeviceId,
    pub probe: Option<unsafe extern "C" fn(*mut DioDev, *const DioDeviceId) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut DioDev)>,
    pub driver: DeviceDriver,
}

/* Device, Resource, ListHead, BusType, and DeviceDriver are supplied by linux/device.h. */

pub const DIO_IDOFF: usize = 0x01;
pub const DIO_IPLOFF: usize = 0x03;
pub const DIO_SECIDOFF: usize = 0x15;
pub const DIOII_SIZEOFF: usize = 0x101;
pub const DIO_VIRADDRBASE: usize = 0xf0000000;
pub const DIO_BASE: usize = 0x600000;
pub const DIO_END: usize = 0x1000000;
pub const DIO_DEVSIZE: usize = 0x10000;
pub const DIOII_BASE: usize = 0x01000000;
pub const DIOII_END: usize = 0x20000000;
pub const DIOII_DEVSIZE: usize = 0x00400000;
pub const DIOII_SCBASE: i32 = 132;
pub const DIO_WILDCARD: DioId = 0xff;

/* The C expression depends on the externally supplied hp300_model and HP_320. */
#[inline]
pub const fn dio_scmax(hp300_model: i32, hp_320: i32) -> i32 {
    if hp300_model == hp_320 { 32 } else { 256 }
}

#[inline] pub const fn dio_scin_hole(scode: i32) -> bool { scode >= 32 && scode < DIOII_SCBASE }
#[inline] pub const fn dio_isdioii(scode: i32) -> bool { scode >= 132 && scode < 256 }
#[inline] pub const fn dio_encode_id(pr: i32, sec: i32) -> i32 { ((sec & 0xff) << 8) | (pr & 0xff) }
#[inline] pub const fn dio_needssecid(id: DioId) -> bool { id == DIO_ID_FBUFFER }

/* Register accessors correspond to the external C in_8() primitive. */
extern "C" { pub fn in_8(addr: *const u8) -> u8; }
#[inline] pub unsafe fn dio_id(baseaddr: *const u8) -> u8 { in_8(baseaddr.add(DIO_IDOFF)) }
#[inline] pub unsafe fn dio_secid(baseaddr: *const u8) -> u8 { in_8(baseaddr.add(DIO_SECIDOFF)) }
#[inline] pub unsafe fn dio_ipl(baseaddr: *const u8) -> u8 { ((in_8(baseaddr.add(DIO_IPLOFF)) >> 4) & 0x03) + 3 }
#[inline] pub unsafe fn dioii_size(baseaddr: *const u8) -> u32 { (in_8(baseaddr.add(DIOII_SIZEOFF)) as u32 + 1) * 0x100000 }
#[inline] pub unsafe fn dio_size(scode: i32, base: *const u8) -> u32 { if dio_isdioii(scode) { dioii_size(base) } else { DIO_DEVSIZE as u32 } }

pub const DIO_ID_DCA0: DioId = 0x02; pub const DIO_DESC_DCA0: &str = "98644A DCA0 serial";
pub const DIO_ID_DCA0REM: DioId = 0x82; pub const DIO_DESC_DCA0REM: &str = "98644A DCA0REM serial";
pub const DIO_ID_DCA1: DioId = 0x42; pub const DIO_DESC_DCA1: &str = "98644A DCA1 serial";
pub const DIO_ID_DCA1REM: DioId = 0xc2; pub const DIO_DESC_DCA1REM: &str = "98644A DCA1REM serial";
pub const DIO_ID_DCM: DioId = 0x05; pub const DIO_DESC_DCM: &str = "98642A DCM serial MUX";
pub const DIO_ID_DCMREM: DioId = 0x85; pub const DIO_DESC_DCMREM: &str = "98642A DCMREM serial MUX";
pub const DIO_ID_LAN: DioId = 0x15; pub const DIO_DESC_LAN: &str = "98643A LANCE ethernet";
pub const DIO_ID_FHPIB: DioId = 0x08; pub const DIO_DESC_FHPIB: &str = "98625A/98625B fast HPIB";
pub const DIO_ID_NHPIB: DioId = 0x01; pub const DIO_DESC_NHPIB: &str = "98624A HPIB";
pub const DIO_ID_SCSI0: DioId = 0x07; pub const DIO_DESC_SCSI0: &str = "98265A SCSI0";
pub const DIO_ID_SCSI1: DioId = 0x27; pub const DIO_DESC_SCSI1: &str = "98265A SCSI1";
pub const DIO_ID_SCSI2: DioId = 0x47; pub const DIO_DESC_SCSI2: &str = "98265A SCSI2";
pub const DIO_ID_SCSI3: DioId = 0x67; pub const DIO_DESC_SCSI3: &str = "98265A SCSI3";
pub const DIO_ID_FBUFFER: DioId = 0x39; pub const DIO_DESC_FBUFFER: &str = "bitmapped display";
pub const DIO_ID_MISC0: DioId = 0x03; pub const DIO_DESC_MISC0: &str = "98622A";
pub const DIO_ID_MISC1: DioId = 0x04; pub const DIO_DESC_MISC1: &str = "98623A";
pub const DIO_ID_PARALLEL: DioId = 0x06; pub const DIO_DESC_PARALLEL: &str = "internal parallel";
pub const DIO_ID_MISC2: DioId = 0x09; pub const DIO_DESC_MISC2: &str = "98287A keyboard";
pub const DIO_ID_MISC3: DioId = 0x0a; pub const DIO_DESC_MISC3: &str = "HP98635A FP accelerator";
pub const DIO_ID_MISC4: DioId = 0x0b; pub const DIO_DESC_MISC4: &str = "timer";
pub const DIO_ID_MISC5: DioId = 0x12; pub const DIO_DESC_MISC5: &str = "98640A";
pub const DIO_ID_MISC6: DioId = 0x16; pub const DIO_DESC_MISC6: &str = "98659A";
pub const DIO_ID_MISC7: DioId = 0x19; pub const DIO_DESC_MISC7: &str = "237 display";
pub const DIO_ID_MISC8: DioId = 0x1a; pub const DIO_DESC_MISC8: &str = "quad-wide card";
pub const DIO_ID_MISC9: DioId = 0x1b; pub const DIO_DESC_MISC9: &str = "98253A";
pub const DIO_ID_MISC10: DioId = 0x1c; pub const DIO_DESC_MISC10: &str = "98253A";
pub const DIO_ID_MISC11: DioId = 0x1d; pub const DIO_DESC_MISC11: &str = "98633A";
pub const DIO_ID_MISC12: DioId = 0x1e; pub const DIO_DESC_MISC12: &str = "98259A";
pub const DIO_ID_MISC13: DioId = 0x1f; pub const DIO_DESC_MISC13: &str = "8741";
pub const DIO_ID_VME: DioId = 0x31; pub const DIO_DESC_VME: &str = "98577A VME adapter";
pub const DIO_ID_DCL: DioId = 0x34; pub const DIO_DESC_DCL: &str = "98628A DCL serial";
pub const DIO_ID_DCLREM: DioId = 0xb4; pub const DIO_DESC_DCLREM: &str = "98628A DCLREM serial";

pub const DIO_ID2_GATORBOX: u8 = 0x01; pub const DIO_DESC2_GATORBOX: &str = "98700/98710 \"gatorbox\" display";
pub const DIO_ID2_TOPCAT: u8 = 0x02; pub const DIO_DESC2_TOPCAT: &str = "98544/98545/98547 \"topcat\" display";
pub const DIO_ID2_RENAISSANCE: u8 = 0x04; pub const DIO_DESC2_RENAISSANCE: &str = "98720/98721 \"renaissance\" display";
pub const DIO_ID2_LRCATSEYE: u8 = 0x05; pub const DIO_DESC2_LRCATSEYE: &str = "low-res catseye display";
pub const DIO_ID2_HRCCATSEYE: u8 = 0x06; pub const DIO_DESC2_HRCCATSEYE: &str = "high-res color catseye display";
pub const DIO_ID2_HRMCATSEYE: u8 = 0x07; pub const DIO_DESC2_HRMCATSEYE: &str = "high-res mono catseye display";
pub const DIO_ID2_DAVINCI: u8 = 0x08; pub const DIO_DESC2_DAVINCI: &str = "98730/98731 \"davinci\" display";
pub const DIO_ID2_XXXCATSEYE: u8 = 0x09; pub const DIO_DESC2_XXXCATSEYE: &str = "catseye display";
pub const DIO_ID2_HYPERION: u8 = 0x0e; pub const DIO_DESC2_HYPERION: &str = "A1096A \"hyperion\" display";
pub const DIO_ID2_XGENESIS: u8 = 0x0b; pub const DIO_DESC2_XGENESIS: &str = "\"x-genesis\" display";
pub const DIO_ID2_TIGER: u8 = 0x0c; pub const DIO_DESC2_TIGER: &str = "\"tiger\" display";
pub const DIO_ID2_YGENESIS: u8 = 0x0d; pub const DIO_DESC2_YGENESIS: &str = "\"y-genesis\" display";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
