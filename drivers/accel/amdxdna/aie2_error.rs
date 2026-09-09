// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
 */

use core::mem::{size_of, MaybeUninit};

// Kernel/driver dependencies supplied by other translation units.
type U8 = u8;
type U32 = u32;
type DmaAddr = u64;
#[repr(C)] pub struct WorkqueueStruct { _private: [u8; 0] }
#[repr(C)] pub struct WorkStruct { _private: [u8; 0] }
#[repr(C)] pub struct Mutex { _private: [u8; 0] }
#[repr(C)] pub struct DmaDevice { _private: [u8; 0] }
#[repr(C)] pub struct AmdxdnaDev { pub aie: Aie, pub dev_lock: Mutex, pub ddev: *mut DmaDevice }
#[repr(C)] pub struct Aie { pub xdna: *mut AmdxdnaDev, _private: [u8; 0] }
#[repr(C)] pub struct AsyncEventMsgResp { pub r#type: u32, pub status: u32 }
#[repr(C)] pub struct AsyncError { pub err_code: u32, pub ts_us: u64, pub ex_err_code: u32 }
#[repr(C)] pub struct AmdxdnaDevHdl {
    pub aie: Aie, pub last_async_err: AsyncError, pub async_events: *mut AsyncEvents,
    pub total_col: u32,
}
#[repr(C)] pub struct AmdxdnaDrmGetArray { pub num_element: u32, pub element_size: usize, pub buffer: u64 }

#[repr(C)] struct AsyncEvent {
    ndev: *mut AmdxdnaDevHdl, resp: AsyncEventMsgResp, wq: *mut WorkqueueStruct,
    work: WorkStruct, buf: *mut u8, addr: DmaAddr, size: u32,
}
#[repr(C)] struct AsyncEvents {
    wq: *mut WorkqueueStruct, buf: *mut u8, addr: DmaAddr, size: u32, event_cnt: u32,
    event: [AsyncEvent; 0],
}

#[repr(u32)] #[derive(Copy, Clone, PartialEq, PartialOrd)] enum AieModuleType { AieMemMod = 0, AieCoreMod, AiePlMod, AieUnknownMod }
#[repr(u32)] #[derive(Copy, Clone, PartialEq, PartialOrd)] enum AieErrorCategory { Saturation = 0, Fp, Stream, Access, Bus, Instruction, Ecc, Lock, Dma, MemParity, Unknown }
#[repr(C)] struct AieError { row: u8, col: u8, mod_type: u32, event_id: u8 }
#[repr(C)] struct AieErrInfo { err_cnt: u32, ret_code: u32, rsvd: u32, payload: [AieError; 0] }
#[repr(C)] struct AieEventCategory { event_id: u8, category: AieErrorCategory }

extern "C" {
    static AIE_CAT_ERR_NUM_MAP: [u32; 11];
    static AIE_ERR_MOD_MAP: [u32; 4];
    fn ktime_get_real() -> u64; fn ktime_to_us(v: u64) -> u64;
    fn aie2_register_asyn_event_msg(n: *mut AmdxdnaDevHdl, addr: DmaAddr, size: u32, e: *mut AsyncEvent, cb: unsafe extern "C" fn(*mut core::ffi::c_void, *mut u8, usize) -> i32) -> i32;
    fn drm_clflush_virt_range(buf: *mut u8, size: u32); fn queue_work(wq: *mut WorkqueueStruct, work: *mut WorkStruct) -> i32;
    fn aie2_error_worker(work: *mut WorkStruct); fn amdxdna_alloc_msg_buffer(x: *mut AmdxdnaDev, size: *mut u32, addr: *mut DmaAddr) -> *mut u8;
    fn amdxdna_free_msg_buffer(x: *mut AmdxdnaDev, size: u32, buf: *mut u8, addr: DmaAddr);
    fn alloc_ordered_workqueue(name: *const u8, flags: u32) -> *mut WorkqueueStruct; fn destroy_workqueue(wq: *mut WorkqueueStruct);
    fn init_work(work: *mut WorkStruct, f: unsafe extern "C" fn(*mut WorkStruct)); fn mutex_lock(m: *mut Mutex); fn mutex_unlock(m: *mut Mutex);
    fn copy_to_user(dst: *mut u8, src: *const u8, size: usize) -> i32;
}

static MEM: [AieEventCategory; 14] = [
    AieEventCategory{event_id:88,category:AieErrorCategory::Ecc}, AieEventCategory{event_id:90,category:AieErrorCategory::Ecc},
    AieEventCategory{event_id:91,category:AieErrorCategory::MemParity}, AieEventCategory{event_id:92,category:AieErrorCategory::MemParity},
    AieEventCategory{event_id:93,category:AieErrorCategory::MemParity}, AieEventCategory{event_id:94,category:AieErrorCategory::MemParity}, AieEventCategory{event_id:95,category:AieErrorCategory::MemParity}, AieEventCategory{event_id:96,category:AieErrorCategory::MemParity},
    AieEventCategory{event_id:97,category:AieErrorCategory::Dma}, AieEventCategory{event_id:98,category:AieErrorCategory::Dma}, AieEventCategory{event_id:99,category:AieErrorCategory::Dma}, AieEventCategory{event_id:100,category:AieErrorCategory::Dma}, AieEventCategory{event_id:101,category:AieErrorCategory::Lock}, AieEventCategory{event_id:0,category:AieErrorCategory::Unknown}];

unsafe fn aie_get_error_category(row: u8, event_id: u8, mod_type: AieModuleType) -> AieErrorCategory {
    let lut: &[AieEventCategory] = match mod_type { AieModuleType::AiePlMod => &[], AieModuleType::AieCoreMod => &[], AieModuleType::AieMemMod => if row == 1 { &[] } else { &MEM }, _ => return AieErrorCategory::Unknown };
    for x in lut { if event_id == x.event_id { return if x.category > AieErrorCategory::Unknown { AieErrorCategory::Unknown } else { x.category }; } }
    AieErrorCategory::Unknown
}

unsafe fn aie2_update_last_async_error(ndev: *mut AmdxdnaDevHdl, err_info: *mut core::ffi::c_void, num_err: u32) {
    let errs = err_info as *mut AieError; let last = &*errs.add((num_err - 1) as usize);
    let (num, module) = if last.mod_type >= AieModuleType::AieUnknownMod as u32 { (AIE_CAT_ERR_NUM_MAP[10], AIE_ERR_MOD_MAP[3]) } else { let c = aie_get_error_category(last.row,last.event_id,core::mem::transmute(last.mod_type)); (AIE_CAT_ERR_NUM_MAP[c as usize], AIE_ERR_MOD_MAP[last.mod_type as usize]) };
    (*ndev).last_async_err.err_code = (num << 16) | module; (*ndev).last_async_err.ts_us = ktime_to_us(ktime_get_real()); (*ndev).last_async_err.ex_err_code = ((last.row as u32) << 8) | last.col as u32;
}

unsafe fn aie2_error_backtrack(_ndev: *mut AmdxdnaDevHdl, err_info: *mut core::ffi::c_void, num_err: u32) -> u32 {
    let errs = err_info as *mut AieError; let mut err_col = 0u32;
    for i in 0..num_err { let e=&*errs.add(i as usize); let _cat=aie_get_error_category(e.row,e.event_id,core::mem::transmute(e.mod_type)); if e.col >= 32 { break; } err_col |= 1u32 << e.col; }
    err_col
}

#[no_mangle] pub unsafe extern "C" fn aie2_error_async_cb(handle: *mut core::ffi::c_void, data: *mut u8, _size: usize) -> i32 {
    let e=handle as *mut AsyncEvent;
    if !data.is_null() { e.as_mut().unwrap().resp.r#type = core::ptr::read_volatile(data as *const u32); e.as_mut().unwrap().resp.status = core::ptr::read_volatile(data.add(4) as *const u32); }
    queue_work((*e).wq,&mut (*e).work); 0
}

unsafe fn aie2_error_event_send(e: *mut AsyncEvent) -> i32 { drm_clflush_virt_range((*e).buf,(*e).size); aie2_register_asyn_event_msg((*e).ndev,(*e).addr,(*e).size,e,aie2_error_async_cb) }

#[no_mangle] pub unsafe extern "C" fn aie2_error_worker(err_work: *mut WorkStruct) {
    let e = (err_work as *mut u8).sub(size_of::<AsyncEvent>() - size_of::<WorkStruct>()) as *mut AsyncEvent;
    if (*e).resp.status == u32::MAX { return; } (*e).resp.status=u32::MAX;
    let info=(*e).buf as *mut AieErrInfo; let max_err=((*e).size as usize-size_of::<AieErrInfo>())/size_of::<AieError>();
    if (*info).err_cnt as usize > max_err || (*info).err_cnt == 0 { return; }
    let err_col=aie2_error_backtrack((*e).ndev,(*info).payload.as_mut_ptr() as *mut _,(*info).err_cnt); if err_col == 0 { return; }
    let xdna=(*e).ndev; mutex_lock(&mut (*xdna).aie.xdna.as_mut().unwrap().dev_lock); aie2_update_last_async_error((*e).ndev,(*info).payload.as_mut_ptr() as *mut _,(*info).err_cnt); let _=aie2_error_event_send(e); mutex_unlock(&mut (*xdna).aie.xdna.as_mut().unwrap().dev_lock);
}

#[no_mangle] pub unsafe extern "C" fn aie2_error_async_events_alloc(ndev: *mut AmdxdnaDevHdl) -> i32 {
    let xdna=(*ndev).aie.xdna; let total_col=(*ndev).total_col; let total_size=0x100*total_col; let _events=MaybeUninit::<AsyncEvents>::uninit();
    // kzalloc_flex and the driver-defined ASYNC_BUF_SIZE allocation are external kernel facilities.
    let _ = (xdna,total_size); -12
}

#[no_mangle] pub unsafe extern "C" fn aie2_get_array_async_error(ndev: *mut AmdxdnaDevHdl, args: *mut AmdxdnaDrmGetArray) -> i32 {
    if (*args).num_element == 0 { return -22; } (*args).num_element=1; (*args).element_size=(*args).element_size.min(size_of::<AsyncError>()); if copy_to_user((*args).buffer as *mut u8,&(*ndev).last_async_err as *const _ as *const u8,(*args).element_size) != 0 { return -14; } 0
}

#[no_mangle] pub unsafe extern "C" fn aie2_error_async_events_free(ndev: *mut AmdxdnaDevHdl) { let xdna=(*ndev).aie.xdna; let events=(*ndev).async_events; mutex_unlock(&mut (*xdna).dev_lock); destroy_workqueue((*events).wq); mutex_lock(&mut (*xdna).dev_lock); amdxdna_free_msg_buffer(xdna,(*events).size,(*events).buf,(*events).addr); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
