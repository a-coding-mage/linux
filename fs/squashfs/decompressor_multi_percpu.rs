// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013
 * Phillip Lougher <phillip@squashfs.org.uk>
 */

// Dependencies supplied by the surrounding kernel/SquashFS translation.

/*
 * This file implements multi-threaded decompression using percpu
 * variables, one thread per cpu core.
 */

#[repr(C)]
pub struct SquashfsStream {
    pub stream: *mut core::ffi::c_void,
    pub lock: LocalLock,
}

// External types and operations are supplied by the translated dependencies.
#[repr(C)]
pub struct LocalLock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SquashfsSbInfo {
    pub stream: *mut core::ffi::c_void,
    pub decompressor: *const SquashfsDecompressor,
}

#[repr(C)]
pub struct SquashfsDecompressor {
    pub name: *const core::ffi::c_char,
    pub init: unsafe extern "C" fn(
        msblk: *mut SquashfsSbInfo,
        comp_opts: *mut core::ffi::c_void,
    ) -> *mut core::ffi::c_void,
    pub free: unsafe extern "C" fn(stream: *mut core::ffi::c_void),
    pub decompress: unsafe extern "C" fn(
        msblk: *mut SquashfsSbInfo,
        stream: *mut core::ffi::c_void,
        bio: *mut Bio,
        offset: i32,
        length: i32,
        output: *mut SquashfsPageActor,
    ) -> i32,
}

#[repr(C)]
pub struct Bio {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SquashfsPageActor {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SquashfsDecompressorThreadOps {
    pub create: Option<unsafe extern "C" fn(*mut SquashfsSbInfo, *mut core::ffi::c_void) -> *mut core::ffi::c_void>,
    pub destroy: Option<unsafe extern "C" fn(*mut SquashfsSbInfo)>,
    pub decompress: Option<unsafe extern "C" fn(*mut SquashfsSbInfo, *mut Bio, i32, i32, *mut SquashfsPageActor) -> i32>,
    pub max_decompressors: Option<unsafe extern "C" fn() -> i32>,
}

extern "C" {
    fn alloc_percpu() -> *mut SquashfsStream;
    fn free_percpu(percpu: *mut SquashfsStream);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn local_lock_init(lock: *mut LocalLock);
    fn local_lock(lock: *mut LocalLock);
    fn local_unlock(lock: *mut LocalLock);
    fn num_possible_cpus() -> i32;
    fn per_cpu_ptr(percpu: *mut SquashfsStream, cpu: i32) -> *mut SquashfsStream;
    fn this_cpu_ptr(percpu: *mut SquashfsStream) -> *mut SquashfsStream;
    fn error_decompression_failed(name: *const core::ffi::c_char);
}

unsafe fn squashfs_decompressor_create(
    msblk: *mut SquashfsSbInfo,
    comp_opts: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let percpu = alloc_percpu();
    if percpu.is_null() {
        return (-12isize) as *mut core::ffi::c_void;
    }

    let mut cpu = 0;
    while cpu < num_possible_cpus() {
        let stream = per_cpu_ptr(percpu, cpu);
        (*stream).stream = ((*(*msblk).decompressor).init)(msblk, comp_opts);
        if ((*stream).stream as isize) < 0 {
            let err = (*stream).stream as isize;
            let mut cleanup_cpu = 0;
            while cleanup_cpu < num_possible_cpus() {
                let cleanup_stream = per_cpu_ptr(percpu, cleanup_cpu);
                if !(*cleanup_stream).stream.is_null()
                    && ((*cleanup_stream).stream as isize) >= 0
                {
                    ((*(*msblk).decompressor).free)((*cleanup_stream).stream);
                }
                cleanup_cpu += 1;
            }
            free_percpu(percpu);
            return err as *mut core::ffi::c_void;
        }
        local_lock_init(&mut (*stream).lock);
        cpu += 1;
    }

    kfree(comp_opts);
    percpu as *mut core::ffi::c_void
}

unsafe fn squashfs_decompressor_destroy(msblk: *mut SquashfsSbInfo) {
    let percpu = (*msblk).stream as *mut SquashfsStream;
    if !(*msblk).stream.is_null() {
        let mut cpu = 0;
        while cpu < num_possible_cpus() {
            let stream = per_cpu_ptr(percpu, cpu);
            ((*(*msblk).decompressor).free)((*stream).stream);
            cpu += 1;
        }
        free_percpu(percpu);
    }
}

unsafe fn squashfs_decompress(
    msblk: *mut SquashfsSbInfo,
    bio: *mut Bio,
    offset: i32,
    length: i32,
    output: *mut SquashfsPageActor,
) -> i32 {
    let percpu = (*msblk).stream as *mut SquashfsStream;
    local_lock(&mut (*percpu).lock);
    let stream = this_cpu_ptr(percpu);
    let res = ((*(*msblk).decompressor).decompress)(
        msblk,
        (*stream).stream,
        bio,
        offset,
        length,
        output,
    );
    local_unlock(&mut (*percpu).lock);

    if res < 0 {
        error_decompression_failed((*(*msblk).decompressor).name);
    }
    res
}

unsafe fn squashfs_max_decompressors() -> i32 {
    num_possible_cpus()
}

#[no_mangle]
pub static mut squashfs_decompressor_percpu: SquashfsDecompressorThreadOps =
    SquashfsDecompressorThreadOps {
        create: Some(squashfs_decompressor_create),
        destroy: Some(squashfs_decompressor_destroy),
        decompress: Some(squashfs_decompress),
        max_decompressors: Some(squashfs_max_decompressors),
    };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
