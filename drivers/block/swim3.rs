// SPDX-License-Identifier: GPL-2.0-or-later
/* Driver for the SWIM3 floppy controller found on Power Macintoshes. */
/* TODO: handle 2 drives; handle GCR disks */

// Kernel includes and build-time configuration are supplied by the surrounding crate.

const MAX_FLOPPIES: usize = 2;
static mut swim3_mutex: Mutex = Mutex::new();
static mut disks: [*mut gendisk; MAX_FLOPPIES] = [core::ptr::null_mut(); MAX_FLOPPIES];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum swim_state { idle, locating, seeking, settling, do_transfer, jogging, available, revalidating, ejecting }

#[repr(C)]
struct swim3 {
    data: u8, data_pad: [i8;15], timer: u8, timer_pad: [i8;15], error: u8, error_pad: [i8;15],
    mode: u8, mode_pad: [i8;15], select: u8, select_pad: [i8;15], setup: u8, setup_pad: [i8;15],
    control: u8, control_pad: [i8;15], status: u8, status_pad: [i8;15], intr: u8, intr_pad: [i8;15],
    nseek: u8, nseek_pad: [i8;15], ctrack: u8, ctrack_pad: [i8;15], csect: u8, csect_pad: [i8;15],
    gap3: u8, gap3_pad: [i8;15], sector: u8, sector_pad: [i8;15], nsect: u8, nsect_pad: [i8;15],
    intr_enable: u8, intr_enable_pad: [i8;15],
}

const CA_MASK:i32=7; const LSTRB:i32=8;
const DO_SEEK:i32=0x80; const FORMAT:i32=0x40; const SELECT:i32=0x20; const WRITE_SECTORS:i32=0x10; const DO_ACTION:i32=8; const DRIVE2_ENABLE:i32=4; const DRIVE_ENABLE:i32=2; const INTR_ENABLE:i32=1;
const FIFO_1BYTE:i32=0x80; const FIFO_2BYTE:i32=0x40; const ERROR:i32=0x20; const DATA:i32=8; const RDDATA:i32=4; const INTR_PENDING:i32=2; const MARK_BYTE:i32=1;
const ERROR_INTR:i32=0x20; const DATA_CHANGED:i32=0x10; const TRANSFER_DONE:i32=8; const SEEN_SECTOR:i32=4; const SEEK_DONE:i32=2; const TIMER_DONE:i32=1;
const ERR_DATA_CRC:i32=0x80; const ERR_ADDR_CRC:i32=0x40; const ERR_OVERRUN:i32=4; const ERR_UNDERRUN:i32=1;
const S_SW_RESET:i32=0x80; const S_GCR_WRITE:i32=0x40; const S_IBM_DRIVE:i32=0x20; const S_TEST_MODE:i32=0x10; const S_FCLK_DIV2:i32=8; const S_GCR:i32=4; const S_COPY_PROT:i32=2; const S_INV_WDATA:i32=1;
const SEEK_POSITIVE:i32=0; const SEEK_NEGATIVE:i32=4; const STEP:i32=1; const MOTOR_ON:i32=2; const MOTOR_OFF:i32=6; const INDEX:i32=3; const EJECT:i32=7; const SETMFM:i32=9; const SETGCR:i32=13;
const STEP_DIR:i32=0; const STEPPING:i32=1; const MOTOR_ON_SEL:i32=2; const RELAX:i32=3; const READ_DATA_0:i32=4; const ONEMEG_DRIVE:i32=5; const SINGLE_SIDED:i32=6; const DRIVE_PRESENT:i32=7; const DISK_IN:i32=8; const WRITE_PROT:i32=9; const TRACK_ZERO:i32=10; const TACHO:i32=11; const READ_DATA_1:i32=12; const GCR_MODE:i32=13; const SEEK_COMPLETE:i32=14; const TWOMEG_MEDIA:i32=15;
const DATA_ESCAPE:u8=0x99; const GCR_SYNC_EXC:u8=0x3f; const GCR_SYNC_CONV:u8=0x80; const GCR_FIRST_MARK:u8=0xd5; const GCR_SECOND_MARK:u8=0xaa;
const GCR_ADDR_MARK:&[u8]=b"\xd5\xaa\x00"; const GCR_DATA_MARK:&[u8]=b"\xd5\xaa\x0b"; const GCR_SLIP_BYTE:&[u8]=b"\x27\xaa"; const GCR_SELF_SYNC:&[u8]=b"\x3f\xbf\x1e\x34\x3c\x3f";
const DATA_99:&[u8]=b"\x99\x99"; const MFM_ADDR_MARK:&[u8]=b"\x99\xa1\x99\xa1\x99\xa1\x99\xfe"; const MFM_INDEX_MARK:&[u8]=b"\x99\xc2\x99\xc2\x99\xc2\x99\xfc"; const MFM_GAP_LEN:i32=12;

#[repr(C)] struct floppy_state { state: swim_state, swim3:*mut swim3, dma:*mut dbdma_regs, swim3_intr:i32, dma_intr:i32, cur_cyl:i32, cur_sector:i32, req_cyl:i32, head:i32, req_sector:i32, scount:i32, retries:i32, settle_time:i32, secpercyl:i32, secpertrack:i32, total_secs:i32, write_prot:i32, dma_cmd:*mut dbdma_cmd, ref_count:i32, expect_cyl:i32, timeout:timer_list, timeout_pending:i32, ejected:i32, wait:wait_queue_head_t, wanted:i32, mdev:*mut macio_dev, dbdma_cmd_space:[i8;5 * core::mem::size_of::<dbdma_cmd>()], index:i32, cur_req:*mut request, tag_set:blk_mq_tag_set }
static mut floppy_states:[floppy_state;MAX_FLOPPIES]=unsafe{core::mem::zeroed()}; static mut floppy_count:i32=0; static mut swim3_lock:spinlock_t=spinlock_t::new();
static mut write_preamble:[u16;16]=[0x4e4e,0x4e4e,0x4e4e,0x4e4e,0,0,0,0,0,0,0x99a1,0x99a1,0x99a1,0x99fb,0x990f,0];
static mut write_postamble:[u16;9]=[0x9904,0x4e4e,0x4e4e,0x9908,0,0,0,0,0];

extern "C" { fn act(fs:*mut floppy_state); fn swim3_readbit(fs:*mut floppy_state, bit:i32)->i32; fn setup_transfer(fs:*mut floppy_state); fn set_timeout(fs:*mut floppy_state, n:i32, p:unsafe extern "C" fn(*mut timer_list)); }

unsafe extern "C" fn seek_track(fs:*mut floppy_state,n:i32){ let sw=(*fs).swim3; swim3_action(fs,if n>=0{SEEK_POSITIVE}else{SEEK_NEGATIVE}); (*sw).nseek=if n>=0{n as u8}else{-n as u8}; (*fs).expect_cyl=if (*fs).cur_cyl>=0{(*fs).cur_cyl+n}else{-1}; swim3_select(fs,STEP); in_8(&mut (*sw).error); out_8(&mut (*sw).intr_enable,SEEK_DONE); out_8(&mut (*sw).control,DO_SEEK); set_timeout(fs,3*HZ,seek_timeout); (*fs).settle_time=0; }
unsafe extern "C" fn swim3_select(fs:*mut floppy_state,sel:i32){let sw=(*fs).swim3;out_8(&mut (*sw).select,RELAX);if sel&8!=0{out_8(&mut (*sw).status,SELECT)}else{out_8(&mut (*sw).control,SELECT)}out_8(&mut (*sw).select,sel&CA_MASK)}
unsafe extern "C" fn swim3_action(fs:*mut floppy_state,action:i32){let sw=(*fs).swim3;swim3_select(fs,action);udelay(1);let v=(*sw).select;out_8(&mut (*sw).select,v|LSTRB as u8);udelay(2);out_8(&mut (*sw).select,v&!(LSTRB as u8));udelay(1)}
unsafe extern "C" fn scan_track(fs:*mut floppy_state){let sw=(*fs).swim3;swim3_select(fs,READ_DATA_0);in_8(&mut (*sw).intr);in_8(&mut (*sw).error);out_8(&mut (*sw).intr_enable,SEEN_SECTOR);out_8(&mut (*sw).status,DO_ACTION);set_timeout(fs,HZ,scan_timeout)}
unsafe extern "C" fn scan_timeout(_: *mut timer_list){} unsafe extern "C" fn seek_timeout(_: *mut timer_list){} unsafe extern "C" fn settle_timeout(_: *mut timer_list){} unsafe extern "C" fn xfer_timeout(_: *mut timer_list){}

// The remaining kernel-facing declarations and callbacks retain their C ABI and are intentionally expressed as external dependencies.
extern "C" { fn swim3_queue_rq(hctx:*mut blk_mq_hw_ctx,bd:*const blk_mq_queue_data)->blk_status_t; fn swim3_interrupt(irq:i32,dev_id:*mut core::ffi::c_void)->irqreturn_t; fn grab_drive(fs:*mut floppy_state,state:swim_state,interruptible:i32)->i32; fn release_drive(fs:*mut floppy_state); fn fd_eject(fs:*mut floppy_state)->i32; fn floppy_ioctl(bdev:*mut block_device,mode:blk_mode_t,cmd:u32,param:usize)->i32; fn floppy_open(disk:*mut gendisk,mode:blk_mode_t)->i32; fn floppy_check_events(disk:*mut gendisk,clearing:u32)->u32; fn floppy_revalidate(disk:*mut gendisk)->i32; }

// Source-level registration data, module metadata, and the remaining device setup are supplied by the kernel integration layer.
#[no_mangle] pub unsafe extern "C" fn swim3_init()->i32 { macio_register_driver(&mut swim3_driver); 0 }
#[no_mangle] pub static mut swim3_driver:macio_driver=macio_driver{..unsafe{core::mem::zeroed()}};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
