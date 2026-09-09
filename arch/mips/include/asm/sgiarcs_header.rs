/* ARC firmware interface defines. Translated from sgiarcs.h. */

/* Various ARCS error codes. */
pub const PROM_ESUCCESS: u32 = 0x00;
pub const PROM_E2BIG: u32 = 0x01;
pub const PROM_EACCESS: u32 = 0x02;
pub const PROM_EAGAIN: u32 = 0x03;
pub const PROM_EBADF: u32 = 0x04;
pub const PROM_EBUSY: u32 = 0x05;
pub const PROM_EFAULT: u32 = 0x06;
pub const PROM_EINVAL: u32 = 0x07;
pub const PROM_EIO: u32 = 0x08;
pub const PROM_EISDIR: u32 = 0x09;
pub const PROM_EMFILE: u32 = 0x0a;
pub const PROM_EMLINK: u32 = 0x0b;
pub const PROM_ENAMETOOLONG: u32 = 0x0c;
pub const PROM_ENODEV: u32 = 0x0d;
pub const PROM_ENOENT: u32 = 0x0e;
pub const PROM_ENOEXEC: u32 = 0x0f;
pub const PROM_ENOMEM: u32 = 0x10;
pub const PROM_ENOSPC: u32 = 0x11;
pub const PROM_ENOTDIR: u32 = 0x12;
pub const PROM_ENOTTY: u32 = 0x13;
pub const PROM_ENXIO: u32 = 0x14;
pub const PROM_EROFS: u32 = 0x15;
pub const PROM_EADDRNOTAVAIL: u32 = 0x1f;
pub const PROM_ETIMEDOUT: u32 = 0x20;
pub const PROM_ECONNABORTED: u32 = 0x21;
pub const PROM_ENOCONNECT: u32 = 0x22;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum linux_devclass { system, processor, cache, adapter, controller, peripheral, memory }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum linux_devtypes {
    Arc, Cpu, Fpu, picache, pdcache, sicache, sdcache, sccache, memdev, eisa_adapter,
    tc_adapter, scsi_adapter, dti_adapter, multifunc_adapter, dsk_controller,
    tp_controller, cdrom_controller, worm_controller, serial_controller, net_controller,
    disp_controller, parallel_controller, ptr_controller, kbd_controller, audio_controller,
    misc_controller, disk_peripheral, flpy_peripheral, tp_peripheral, modem_peripheral,
    monitor_peripheral, printer_peripheral, ptr_peripheral, kbd_peripheral, term_peripheral,
    line_peripheral, net_peripheral, misc_peripheral, anon,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub enum linux_identifier { bogus, ronly, removable, consin, consout, input, output }

#[repr(C)]
pub struct linux_component {
    pub class: linux_devclass, pub r#type: linux_devtypes, pub iflags: linux_identifier,
    pub vers: u16, pub rev: u16, pub key: ULONG, pub amask: ULONG, pub cdsize: ULONG,
    pub ilen: ULONG, pub iname: _PULONG,
}
pub type pcomponent = linux_component;
#[repr(C)] pub struct linux_sysid { pub vend: [i8; 8], pub prod: [i8; 8] }

#[repr(C)]
pub enum arcs_memtypes { arcs_eblock, arcs_rvpage, arcs_fcontig, arcs_free, arcs_bmem, arcs_prog, arcs_atmp, arcs_aperm }
#[repr(C)]
pub enum arc_memtypes { arc_eblock, arc_rvpage, arc_free, arc_bmem, arc_prog, arc_atmp, arc_aperm, arc_fcontig }
#[repr(C)] pub union linux_memtypes { pub arcs: arcs_memtypes, pub arc: arc_memtypes }
#[repr(C)] pub struct linux_mdesc { pub r#type: linux_memtypes, pub base: ULONG, pub pages: ULONG }
#[repr(C)] pub struct linux_tinfo { pub yr:u16, pub mnth:u16, pub day:u16, pub hr:u16, pub min:u16, pub sec:u16, pub msec:u16 }
#[repr(C)] pub struct linux_vdirent { pub namelen: ULONG, pub attr: u8, pub fname: [i8;32] }
#[repr(C)] pub enum linux_omode { rdonly, wronly, rdwr, wronly_creat, rdwr_creat, wronly_ssede, rdwr_ssede, dirent, dirent_creat }
#[repr(C)] pub enum linux_seekmode { absolute, relative }
#[repr(C)] pub enum linux_mountops { media_load, media_unload }

#[repr(C)] pub struct linux_bigint { pub lo: u32, pub hi: i32 }
#[repr(C)] pub struct linux_finfo { pub begin: linux_bigint, pub end: linux_bigint, pub cur: linux_bigint, pub dtype: linux_devtypes, pub namelen: usize, pub attr: u8, pub name: [i8;32] }
#[repr(C)] pub struct linux_romvec {
    pub load: LONG, pub invoke: LONG, pub exec: LONG, pub halt: LONG, pub pdown: LONG, pub restart: LONG, pub reboot: LONG, pub imode: LONG, pub _unused1: LONG,
    pub next_component: LONG, pub child_component: LONG, pub parent_component: LONG, pub component_data: LONG, pub child_add: LONG, pub comp_del: LONG, pub component_by_path: LONG,
    pub cfg_save: LONG, pub get_sysid: LONG, pub get_mdesc: LONG, pub _unused2: LONG, pub get_tinfo: LONG, pub get_rtime: LONG,
    pub get_vdirent: LONG, pub open: LONG, pub close: LONG, pub read: LONG, pub get_rstatus: LONG, pub write: LONG, pub seek: LONG, pub mount: LONG,
    pub get_evar: LONG, pub set_evar: LONG, pub get_finfo: LONG, pub set_finfo: LONG, pub cache_flush: LONG, pub TestUnicodeCharacter: LONG, pub GetDisplayStatus: LONG,
}

#[repr(C)] pub struct SYSTEM_PARAMETER_BLOCK {
    pub magic: ULONG, pub len: ULONG, pub ver: u16, pub rev: u16, pub rs_block: _PLONG, pub dbg_block: _PLONG, pub gevect: _PLONG, pub utlbvect: _PLONG,
    pub rveclen: ULONG, pub romvec: _PVOID, pub pveclen: ULONG, pub pvector: _PVOID, pub adap_cnt: ULONG, pub adap_typ0: ULONG, pub adap_vcnt0: ULONG, pub adap_vector: _PVOID,
    pub adap_typ1: ULONG, pub adap_vcnt1: ULONG, pub adap_vector1: _PVOID,
}
pub type PSYSTEM_PARAMETER_BLOCK = *mut SYSTEM_PARAMETER_BLOCK;
pub const PROMBLOCK_MAGIC: ULONG = 0x53435241;
pub const SGIPROM_STDIN: i32 = 0; pub const SGIPROM_STDOUT: i32 = 1;
pub const SGIPROM_ROFILE: u32=0x01; pub const SGIPROM_HFILE:u32=0x02; pub const SGIPROM_SFILE:u32=0x04; pub const SGIPROM_AFILE:u32=0x08; pub const SGIPROM_DFILE:u32=0x10; pub const SGIPROM_DELFILE:u32=0x20;

#[repr(C)] pub struct param { pub size:u16, pub lsize:u8, pub bsize:u8 }
#[repr(C)] pub union linux_cache_key { pub info:param, pub allinfo: usize }
#[repr(C)] pub struct linux_cdata { pub name:*mut i8, pub mlen:i32, pub r#type:linux_devtypes }
#[repr(C)] pub struct sgi_partition { pub flag:u8, pub shead:u8, pub ssect:u8, pub scyl:u8, pub systype:u8, pub ehead:u8, pub esect:u8, pub ecyl:u8, pub rsect0:u8, pub rsect1:u8, pub rsect2:u8, pub rsect3:u8, pub tsect0:u8, pub tsect1:u8, pub tsect2:u8, pub tsect3:u8 }
pub const SGIPART_UNUSED:u8=0; pub const SGIPART_ACTIVE:u8=0x80; pub const SGIBBLOCK_MAGIC:u16=0xaa55; pub const SGIBBLOCK_MAXPART:usize=4;
#[repr(C)] pub struct sgi_bootblock { pub _unused:[u8;446], pub partitions:[sgi_partition;4], pub magic:u16 }
#[repr(C)] pub struct sgi_bparm_block { pub bytes_sect:u16, pub sect_clust:u8, pub sect_resv:u16, pub nfats:u8, pub nroot_dirents:u16, pub sect_volume:u16, pub media_type:u8, pub sect_fat:u16, pub sect_track:u16, pub nheads:u16, pub nhsects:u16 }
#[repr(C)] pub struct sgi_bsector { pub jmpinfo:[u8;3], pub manuf_name:[u8;8], pub info:sgi_bparm_block }
#[repr(C)] pub struct linux_smonblock { pub magic:usize, pub handler:Option<unsafe extern "C" fn()>, pub dtable_base:usize, pub printf:Option<unsafe extern "C" fn(*const i8, ...) -> i32>, pub btable_base:usize, pub mpflushreqs:usize, pub ntab:usize, pub stab:usize, pub smax:i32 }
pub const SMB_DEBUG_MAGIC:usize=0xfeeddead;

/* These scalar and pointer types are supplied by the translated dependencies. */
pub const SGI_PROMBLOCK_ADDRESS: usize = 0xA0001000;
pub const O32_STK_ENTRIES: usize = 4096;

/* C's configuration-selected ARC_CALL macros are represented as Rust macros. */
#[cfg(all(target_pointer_width = "64", feature = "fw_arc32"))]
extern "C" {
    pub fn call_o32(vec: i64, stack: *mut core::ffi::c_void, ...) -> i64;
    pub static mut o32_stk: [u64; O32_STK_ENTRIES];
}

#[cfg(all(target_pointer_width = "64", feature = "fw_arc32"))]
macro_rules! ARC_CALL0 { ($romvec:expr, $dest:ident) => {{ unsafe { call_o32(($romvec).$dest as i64, o32_stk.as_mut_ptr().add(O32_STK_ENTRIES)) } }}; }
#[cfg(all(target_pointer_width = "64", feature = "fw_arc32"))]
macro_rules! ARC_CALL1 { ($romvec:expr, $dest:ident, $a1:expr) => {{ unsafe { call_o32(($romvec).$dest as i64, o32_stk.as_mut_ptr().add(O32_STK_ENTRIES), ($a1 as i64) as i32) } }}; }
#[cfg(all(target_pointer_width = "64", feature = "fw_arc32"))]
macro_rules! ARC_CALL2 { ($romvec:expr, $dest:ident, $a1:expr, $a2:expr) => {{ unsafe { call_o32(($romvec).$dest as i64, o32_stk.as_mut_ptr().add(O32_STK_ENTRIES), ($a1 as i64) as i32, ($a2 as i64) as i32) } }}; }
#[cfg(all(target_pointer_width = "64", feature = "fw_arc32"))]
macro_rules! ARC_CALL3 { ($romvec:expr, $dest:ident, $a1:expr, $a2:expr, $a3:expr) => {{ unsafe { call_o32(($romvec).$dest as i64, o32_stk.as_mut_ptr().add(O32_STK_ENTRIES), ($a1 as i64) as i32, ($a2 as i64) as i32, ($a3 as i64) as i32) } }}; }
#[cfg(all(target_pointer_width = "64", feature = "fw_arc32"))]
macro_rules! ARC_CALL4 { ($romvec:expr, $dest:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {{ unsafe { call_o32(($romvec).$dest as i64, o32_stk.as_mut_ptr().add(O32_STK_ENTRIES), ($a1 as i64) as i32, ($a2 as i64) as i32, ($a3 as i64) as i32, ($a4 as i64) as i32) } }}; }
#[cfg(all(target_pointer_width = "64", feature = "fw_arc32"))]
macro_rules! ARC_CALL5 { ($romvec:expr, $dest:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {{ unsafe { call_o32(($romvec).$dest as i64, o32_stk.as_mut_ptr().add(O32_STK_ENTRIES), ($a1 as i64) as i32, ($a2 as i64) as i32, ($a3 as i64) as i32, ($a4 as i64) as i32, ($a5 as i64) as i32) } }}; }

/* Native-width ARC calls preserve the firmware function-pointer interface. */
#[cfg(any(all(target_pointer_width = "32", feature = "fw_arc32"), all(target_pointer_width = "64", feature = "fw_arc64")))]
macro_rules! ARC_CALL0 { ($romvec:expr, $dest:ident) => {{ unsafe { core::mem::transmute::<_, unsafe extern "C" fn() -> isize>(($romvec).$dest)() } }}; }
#[cfg(any(all(target_pointer_width = "32", feature = "fw_arc32"), all(target_pointer_width = "64", feature = "fw_arc64")))]
macro_rules! ARC_CALL1 { ($romvec:expr, $dest:ident, $a1:expr) => {{ unsafe { core::mem::transmute::<_, unsafe extern "C" fn(isize)->isize>(($romvec).$dest)($a1 as isize) } }}; }
#[cfg(any(all(target_pointer_width = "32", feature = "fw_arc32"), all(target_pointer_width = "64", feature = "fw_arc64")))]
macro_rules! ARC_CALL2 { ($romvec:expr, $dest:ident, $a1:expr, $a2:expr) => {{ unsafe { core::mem::transmute::<_, unsafe extern "C" fn(isize,isize)->isize>(($romvec).$dest)($a1 as isize,$a2 as isize) } }}; }
#[cfg(any(all(target_pointer_width = "32", feature = "fw_arc32"), all(target_pointer_width = "64", feature = "fw_arc64")))]
macro_rules! ARC_CALL3 { ($romvec:expr, $dest:ident, $a1:expr, $a2:expr, $a3:expr) => {{ unsafe { core::mem::transmute::<_, unsafe extern "C" fn(isize,isize,isize)->isize>(($romvec).$dest)($a1 as isize,$a2 as isize,$a3 as isize) } }}; }
#[cfg(any(all(target_pointer_width = "32", feature = "fw_arc32"), all(target_pointer_width = "64", feature = "fw_arc64")))]
macro_rules! ARC_CALL4 { ($romvec:expr, $dest:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {{ unsafe { core::mem::transmute::<_, unsafe extern "C" fn(isize,isize,isize,isize)->isize>(($romvec).$dest)($a1 as isize,$a2 as isize,$a3 as isize,$a4 as isize) } }}; }
#[cfg(any(all(target_pointer_width = "32", feature = "fw_arc32"), all(target_pointer_width = "64", feature = "fw_arc64")))]
macro_rules! ARC_CALL5 { ($romvec:expr, $dest:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {{ unsafe { core::mem::transmute::<_, unsafe extern "C" fn(isize,isize,isize,isize,isize)->isize>(($romvec).$dest)($a1 as isize,$a2 as isize,$a3 as isize,$a4 as isize,$a5 as isize) } }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
