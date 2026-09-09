// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerNV OPAL asynchronous completion interfaces
 *
 * Copyright 2013-2017 IBM Corp.
 */

// Kernel headers and symbols are supplied by other translation units.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum OpalAsyncTokenState {
    ASYNC_TOKEN_UNALLOCATED = 0,
    ASYNC_TOKEN_ALLOCATED,
    ASYNC_TOKEN_DISPATCHED,
    ASYNC_TOKEN_ABANDONED,
    ASYNC_TOKEN_COMPLETED,
}

#[repr(C)]
struct OpalAsyncToken {
    state: OpalAsyncTokenState,
    response: OpalMsg,
}

extern "C" {
    static mut opal_async_wait: WaitQueueHead;
    static mut opal_async_comp_lock: Spinlock;
    static mut opal_async_sem: Semaphore;
    static mut opal_max_async_tokens: u32;
    static mut opal_async_tokens: *mut OpalAsyncToken;

    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut CULong);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: CULong);
    fn down_interruptible(sem: *mut Semaphore) -> CInt;
    fn up(sem: *mut Semaphore);
    fn opal_wake_poller();
    fn wait_event(wait: *mut WaitQueueHead, condition: bool);
    fn wait_event_interruptible(wait: *mut WaitQueueHead, condition: bool) -> CInt;
    fn memcpy(dst: *mut Core, src: *const Core, n: usize) -> *mut Core;
    fn wake_up(wait: *mut WaitQueueHead);
    fn be64_to_cpu(value: u64) -> u64;
    fn be32_to_cpup(value: *const u32) -> u32;
    fn of_find_node_by_path(path: *const u8) -> *mut DeviceNode;
    fn of_get_property(node: *mut DeviceNode, name: *const u8, len: *mut CInt) -> *const u32;
    fn of_node_put(node: *mut DeviceNode);
    fn kzalloc_objs<T>(size: usize, count: u32) -> *mut T;
    fn kfree(ptr: *mut Core);
    fn opal_message_notifier_register(msg_type: u32, nb: *mut NotifierBlock) -> CInt;
    fn sema_init(sem: *mut Semaphore, value: u32);
}

#[repr(C)] struct WaitQueueHead;
#[repr(C)] struct Spinlock;
#[repr(C)] struct Semaphore;
#[repr(C)] struct DeviceNode;
#[repr(C)] struct Core;
#[repr(C)] struct OpalMsg { params: [u64; 8] }
#[repr(C)] struct NotifierBlock {
    notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, CULong, *mut Core) -> CInt>,
    next: *mut NotifierBlock,
    priority: CInt,
}

type CInt = i32;
type CULong = usize;

const EBUSY: CInt = 16;
const EINVAL: CInt = 22;
const ENOENT: CInt = 2;
const ENOMEM: CInt = 12;
const ERESTARTSYS: CInt = 512;
const OPAL_MSG_ASYNC_COMP: u32 = 0;

unsafe fn __opal_async_get_token() -> CInt {
    let mut flags: CULong = 0;
    let mut token: CInt = -EBUSY;

    spin_lock_irqsave(&mut opal_async_comp_lock, &mut flags);
    for i in 0..opal_max_async_tokens {
        if (*opal_async_tokens.add(i as usize)).state == OpalAsyncTokenState::ASYNC_TOKEN_UNALLOCATED {
            (*opal_async_tokens.add(i as usize)).state = OpalAsyncTokenState::ASYNC_TOKEN_ALLOCATED;
            token = i as CInt;
            break;
        }
    }
    spin_unlock_irqrestore(&mut opal_async_comp_lock, flags);
    token
}

/*
 * If the returned token is used in an opal call and opal returns
 * OPAL_ASYNC_COMPLETION, one of the wait functions must be called before
 * another opal_async_* function.
 */
#[no_mangle]
pub unsafe extern "C" fn opal_async_get_token_interruptible() -> CInt {
    if down_interruptible(&mut opal_async_sem) != 0 { return -ERESTARTSYS; }
    let token = __opal_async_get_token();
    if token < 0 { up(&mut opal_async_sem); }
    token
}

unsafe fn __opal_async_release_token(token: CInt) -> CInt {
    if token < 0 || token as u32 >= opal_max_async_tokens { return -EINVAL; }
    let mut flags: CULong = 0;
    spin_lock_irqsave(&mut opal_async_comp_lock, &mut flags);
    let state = &mut (*opal_async_tokens.add(token as usize)).state;
    let rc = match *state {
        OpalAsyncTokenState::ASYNC_TOKEN_COMPLETED | OpalAsyncTokenState::ASYNC_TOKEN_ALLOCATED => {
            *state = OpalAsyncTokenState::ASYNC_TOKEN_UNALLOCATED; 0
        }
        OpalAsyncTokenState::ASYNC_TOKEN_DISPATCHED => {
            *state = OpalAsyncTokenState::ASYNC_TOKEN_ABANDONED; 1
        }
        _ => 1,
    };
    spin_unlock_irqrestore(&mut opal_async_comp_lock, flags);
    rc
}

#[no_mangle]
pub unsafe extern "C" fn opal_async_release_token(token: CInt) -> CInt {
    let ret = __opal_async_release_token(token);
    if ret == 0 { up(&mut opal_async_sem); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn opal_async_wait_response(token: u64, msg: *mut OpalMsg) -> CInt {
    if token >= opal_max_async_tokens as u64 || msg.is_null() { return -EINVAL; }
    opal_wake_poller();
    wait_event(&mut opal_async_wait, (*opal_async_tokens.add(token as usize)).state == OpalAsyncTokenState::ASYNC_TOKEN_COMPLETED);
    memcpy(msg as *mut Core, &(*opal_async_tokens.add(token as usize)).response as *const OpalMsg as *const Core, core::mem::size_of::<OpalMsg>());
    0
}

#[no_mangle]
pub unsafe extern "C" fn opal_async_wait_response_interruptible(token: u64, msg: *mut OpalMsg) -> CInt {
    if token >= opal_max_async_tokens as u64 || msg.is_null() { return -EINVAL; }
    if (*opal_async_tokens.add(token as usize)).state == OpalAsyncTokenState::ASYNC_TOKEN_ALLOCATED {
        let mut flags: CULong = 0;
        spin_lock_irqsave(&mut opal_async_comp_lock, &mut flags);
        if (*opal_async_tokens.add(token as usize)).state == OpalAsyncTokenState::ASYNC_TOKEN_ALLOCATED {
            (*opal_async_tokens.add(token as usize)).state = OpalAsyncTokenState::ASYNC_TOKEN_DISPATCHED;
        }
        spin_unlock_irqrestore(&mut opal_async_comp_lock, flags);
    }
    opal_wake_poller();
    let ret = wait_event_interruptible(&mut opal_async_wait, (*opal_async_tokens.add(token as usize)).state == OpalAsyncTokenState::ASYNC_TOKEN_COMPLETED);
    if ret == 0 { memcpy(msg as *mut Core, &(*opal_async_tokens.add(token as usize)).response as *const OpalMsg as *const Core, core::mem::size_of::<OpalMsg>()); }
    ret
}

unsafe extern "C" fn opal_async_comp_event(_nb: *mut NotifierBlock, msg_type: CULong, msg: *mut Core) -> CInt {
    if msg_type as u32 != OPAL_MSG_ASYNC_COMP { return 0; }
    let comp_msg = msg as *mut OpalMsg;
    let token = be64_to_cpu((*comp_msg).params[0]);
    let mut flags: CULong = 0;
    spin_lock_irqsave(&mut opal_async_comp_lock, &mut flags);
    let state = (*opal_async_tokens.add(token as usize)).state;
    (*opal_async_tokens.add(token as usize)).state = OpalAsyncTokenState::ASYNC_TOKEN_COMPLETED;
    spin_unlock_irqrestore(&mut opal_async_comp_lock, flags);
    if state == OpalAsyncTokenState::ASYNC_TOKEN_ABANDONED { opal_async_release_token(token as CInt); return 0; }
    memcpy(&mut (*opal_async_tokens.add(token as usize)).response as *mut OpalMsg as *mut Core, comp_msg as *const Core, core::mem::size_of::<OpalMsg>());
    wake_up(&mut opal_async_wait);
    0
}

static mut opal_async_comp_nb: NotifierBlock = NotifierBlock { notifier_call: Some(opal_async_comp_event), next: core::ptr::null_mut(), priority: 0 };

#[no_mangle]
pub unsafe extern "C" fn opal_async_comp_init() -> CInt {
    let opal_node = of_find_node_by_path(b"/ibm,opal\0".as_ptr());
    if opal_node.is_null() { return -ENOENT; }
    let async_count = of_get_property(opal_node, b"opal-msg-async-num\0".as_ptr(), core::ptr::null_mut());
    if async_count.is_null() { of_node_put(opal_node); return -ENOENT; }
    opal_max_async_tokens = be32_to_cpup(async_count);
    opal_async_tokens = kzalloc_objs::<OpalAsyncToken>(core::mem::size_of::<OpalAsyncToken>(), opal_max_async_tokens);
    if opal_async_tokens.is_null() { of_node_put(opal_node); return -ENOMEM; }
    let err = opal_message_notifier_register(OPAL_MSG_ASYNC_COMP, &mut opal_async_comp_nb);
    if err != 0 { kfree(opal_async_tokens as *mut Core); of_node_put(opal_node); return err; }
    sema_init(&mut opal_async_sem, opal_max_async_tokens);
    of_node_put(opal_node);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
