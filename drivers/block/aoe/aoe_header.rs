/* Copyright (c) 2013 Coraid, Inc.  See COPYING for GPL terms. */
// Dependency: Linux block-mq and related kernel types/constants are supplied externally.

pub const VERSION: &str = "85";
pub const AOE_MAJOR: i32 = 152;
pub const DEVICE_NAME: &str = "aoe";
// AOE_PARTITIONS defaults to 16 when not supplied by the build configuration.
pub const AOE_PARTITIONS: i32 = 16;
pub const WHITESPACE: &str = " \t\x0b\x0c\n,";

pub const AOECMD_ATA: i32 = 0;
pub const AOECMD_CFG: i32 = 1;
pub const AOECMD_VEND_MIN: i32 = 0xf0;
pub const AOEFL_RSP: i32 = 1 << 3;
pub const AOEFL_ERR: i32 = 1 << 2;
pub const AOEAFL_EXT: i32 = 1 << 6;
pub const AOEAFL_DEV: i32 = 1 << 4;
pub const AOEAFL_ASYNC: i32 = 1 << 1;
pub const AOEAFL_WRITE: i32 = 1;
pub const AOECCMD_READ: i32 = 0;
pub const AOECCMD_TEST: i32 = 1;
pub const AOECCMD_PTEST: i32 = 2;
pub const AOECCMD_SET: i32 = 3;
pub const AOECCMD_FSET: i32 = 4;
pub const AOE_HVER: i32 = 0x10;

#[repr(C)]
pub struct aoe_hdr { pub dst: [u8; 6], pub src: [u8; 6], pub r#type: __be16, pub verfl: u8, pub err: u8, pub major: __be16, pub minor: u8, pub cmd: u8, pub tag: __be32 }

#[repr(C)]
pub struct aoe_atahdr { pub aflags: u8, pub errfeat: u8, pub scnt: u8, pub cmdstat: u8, pub lba0: u8, pub lba1: u8, pub lba2: u8, pub lba3: u8, pub lba4: u8, pub lba5: u8, pub res: [u8; 2] }

#[repr(C)]
pub struct aoe_cfghdr { pub bufcnt: __be16, pub fwver: __be16, pub scnt: u8, pub aoeccmd: u8, pub cslen: [u8; 2] }

pub const DEVFL_UP: i32 = 1; // device is installed in system and ready for AoE->ATA commands
pub const DEVFL_TKILL: i32 = 1 << 1; // flag for timer to know when to kill self
pub const DEVFL_EXT: i32 = 1 << 2; // device accepts lba48 commands
pub const DEVFL_GDALLOC: i32 = 1 << 3; // need to alloc gendisk
pub const DEVFL_GD_NOW: i32 = 1 << 4; // allocating gendisk
pub const DEVFL_KICKME: i32 = 1 << 5; // slow polling network card catch
pub const DEVFL_NEWSIZE: i32 = 1 << 6; // need to update dev size in block layer
pub const DEVFL_FREEING: i32 = 1 << 7; // set when device is being cleaned up
pub const DEVFL_FREED: i32 = 1 << 8; // device has been cleaned up
pub const DEVFL_DEAD: i32 = 1 << 9; // device has timed out of aoe_deadsecs

pub const DEFAULTBCNT: i32 = 2 * 512;
pub const MIN_BUFS: i32 = 16;
pub const NTARGETS: i32 = 4;
pub const NAOEIFS: i32 = 8;
pub const NSKBPOOLMAX: i32 = 256;
pub const NFACTIVE: i32 = 61;
pub const TIMERTICK: i32 = HZ / 10;
pub const RTTSCALE: i32 = 8;
pub const RTTDSCALE: i32 = 3;
pub const RTTAVG_INIT: i32 = USEC_PER_SEC / 4 << RTTSCALE;
pub const RTTDEV_INIT: i32 = RTTAVG_INIT / 4;
pub const HARD_SCORN_SECS: i32 = 10;
pub const MAX_TAINT: i32 = 1000;

#[repr(C)] pub struct aoe_req { pub nr_bios: ulong }
#[repr(C)] pub struct buf { pub nframesout: ulong, pub bio: *mut bio, pub iter: bvec_iter, pub rq: *mut request }
pub const FFL_PROBE: i32 = 1;

#[repr(C)] pub struct frame { pub head: list_head, pub tag: u32, pub sent: ktime_t, pub waited: ulong, pub waited_total: ulong, pub t: *mut aoetgt, pub skb: *mut sk_buff, pub r_skb: *mut sk_buff, pub buf: *mut buf, pub iter: bvec_iter, pub flags: i8 }
#[repr(C)] pub struct aoeif { pub nd: *mut net_device, pub lost: ulong, pub bcnt: i32 }
#[repr(C)] pub struct aoetgt { pub addr: [u8; 6], pub nframes: ushort, pub d: *mut aoedev, pub ffree: list_head, pub ifs: [aoeif; NAOEIFS as usize], pub ifp: *mut aoeif, pub nout: ushort, pub maxout: ushort, pub next_cwnd: ushort, pub ssthresh: ushort, pub falloc: ulong, pub taint: i32, pub minbcnt: i32, pub wpkts: i32, pub rpkts: i32, pub nout_probes: i8 }

#[repr(C)] pub struct aoedev { pub next: *mut aoedev, pub sysminor: ulong, pub aoemajor: ulong, pub rttavg: u32, pub rttdev: u32, pub aoeminor: u16, pub flags: u16, pub nopen: u16, pub fw_ver: u16, pub lasttag: u16, pub useme: u16, pub ref_: ulong, pub work: work_struct, pub gd: *mut gendisk, pub debugfs: *mut dentry, pub blkq: *mut request_queue, pub rq_list: list_head, pub tag_set: blk_mq_tag_set, pub geo: hd_geometry, pub ssize: sector_t, pub timer: timer_list, pub lock: spinlock_t, pub skbpool: sk_buff_head, pub bufpool: *mut mempool_t, pub ip: aoedev_ip, pub maxbcnt: ulong, pub factive: [list_head; NFACTIVE as usize], pub rexmitq: list_head, pub targets: *mut *mut aoetgt, pub ntargets: ulong, pub tgt: *mut *mut aoetgt, pub kicked: ulong, pub ident: [i8; 512] }
#[repr(C)] pub struct aoedev_ip { pub buf: *mut buf, pub nxbio: *mut bio, pub rq: *mut request }

#[repr(C)] pub struct ktstate { pub rendez: completion, pub task: *mut task_struct, pub waitq: *mut wait_queue_head_t, pub fn_: Option<unsafe extern "C" fn(i32) -> i32>, pub name: [i8; 12], pub lock: *mut spinlock_t, pub id: i32, pub active: i32 }

extern "C" {
    pub fn aoeblk_init() -> i32; pub fn aoeblk_exit(); pub fn aoeblk_gdalloc(arg: *mut core::ffi::c_void); pub fn aoedisk_rm_debugfs(d: *mut aoedev);
    pub fn aoechr_init() -> i32; pub fn aoechr_exit(); pub fn aoechr_error(s: *mut i8);
    pub fn aoecmd_work(d: *mut aoedev); pub fn aoecmd_cfg(aoemajor: ushort, aoeminor: u8); pub fn aoecmd_ata_rsp(skb: *mut sk_buff) -> *mut sk_buff; pub fn aoecmd_cfg_rsp(skb: *mut sk_buff); pub fn aoecmd_sleepwork(work: *mut work_struct); pub fn aoecmd_wreset(t: *mut aoetgt); pub fn aoecmd_cleanslate(d: *mut aoedev); pub fn aoecmd_exit(); pub fn aoecmd_init() -> i32; pub fn aoecmd_ata_id(d: *mut aoedev) -> *mut sk_buff;
    pub fn aoe_freetframe(f: *mut frame); pub fn aoe_flush_iocq(); pub fn aoe_flush_iocq_by_index(i: i32); pub fn aoe_end_request(d: *mut aoedev, rq: *mut request, error: i32); pub fn aoe_ktstart(k: *mut ktstate) -> i32; pub fn aoe_ktstop(k: *mut ktstate);
    pub fn aoedev_init() -> i32; pub fn aoedev_exit(); pub fn aoedev_by_aoeaddr(maj: ulong, min: i32, do_alloc: i32) -> *mut aoedev; pub fn aoedev_downdev(d: *mut aoedev); pub fn aoedev_flush(s: *const i8, size: size_t) -> i32; pub fn aoe_failbuf(d: *mut aoedev, b: *mut buf);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
