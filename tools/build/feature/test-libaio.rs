// SPDX-License-Identifier: GPL-2.0
// Translated from C source that depends on <aio.h>.

use std::ffi::c_void;
use std::mem::zeroed;

#[repr(C)]
pub struct sigevent {
    pub sigev_notify: i32,
}

#[repr(C)]
pub struct aiocb {
    pub aio_fildes: i32,
    pub aio_offset: i64,
    pub aio_buf: *mut c_void,
    pub aio_nbytes: usize,
    pub aio_reqprio: i32,
    pub aio_sigevent: sigevent,
}

unsafe extern "C" {
    pub fn aio_return(aiocbp: *mut aiocb) -> isize;
}

pub unsafe fn main_0() -> i32 {
    let mut aiocb: aiocb = unsafe { zeroed() };

    aiocb.aio_fildes = 0;
    aiocb.aio_offset = 0;
    aiocb.aio_buf = 0 as *mut c_void;
    aiocb.aio_nbytes = 0;
    aiocb.aio_reqprio = 0;
    aiocb.aio_sigevent.sigev_notify = 1; /*SIGEV_NONE*/

    unsafe { aio_return(&mut aiocb) as i32 }
}

fn main() {
    unsafe {
        std::process::exit(main_0());
    }
}
