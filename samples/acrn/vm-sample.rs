// SPDX-License-Identifier: GPL-2.0
/*
 * A sample program to run a User VM on the ACRN hypervisor
 *
 * This sample runs in a Service VM, which is a privileged VM of ACRN.
 * CONFIG_ACRN_HSM needs to be enabled in the Service VM.
 *
 * Guest VM code in guest16.s will be executed after the VM launched.
 *
 * Copyright (C) 2020 Intel Corporation. All rights reserved.
 */

use std::ffi::c_void;
use std::ptr;

// These names are supplied by the ACRN and libc dependencies.
use acrn::*;

const GUEST_MEMORY_SIZE: usize = 1024 * 1024;

static mut GUEST_MEMORY: *mut c_void = ptr::null_mut();

extern "C" {
    static guest16: u8;
    static guest16_end: u8;
    fn posix_memalign(memptr: *mut *mut c_void, alignment: usize, size: usize) -> i32;
    fn open(pathname: *const i8, flags: i32, ...) -> i32;
    fn ioctl(fd: i32, request: libc::c_ulong, ...) -> i32;
    fn close(fd: i32) -> i32;
    fn free(ptr: *mut c_void);
    fn printf(format: *const i8, ...) -> i32;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn signal(signum: i32, handler: extern "C" fn(i32)) -> extern "C" fn(i32);
}

#[repr(align(4096))]
struct IoRequestPage([u8; 4096]);

static mut IO_REQUEST_PAGE: IoRequestPage = IoRequestPage([0; 4096]);
static mut IO_REQ_BUF: *mut acrn_io_request = ptr::null_mut();

static mut VCPU_NUM: u16 = 0;
static mut VMID: u16 = 0;
static mut HSM_FD: i32 = 0;
static mut IS_RUNNING: i32 = 1;

extern "C" fn vm_exit(sig: i32) {
    let _ = sig;
    unsafe {
        IS_RUNNING = 0;
        ioctl(HSM_FD, ACRN_IOCTL_PAUSE_VM, VMID);
        ioctl(HSM_FD, ACRN_IOCTL_DESTROY_IOREQ_CLIENT, 0);
    }
}

pub unsafe fn main(_argc: i32, _argv: *mut *mut i8) -> i32 {
    IO_REQ_BUF = IO_REQUEST_PAGE.0.as_mut_ptr() as *mut acrn_io_request;

    let mut vcpu_id: i32;
    let mut ret: i32;
    let mut create_vm: acrn_vm_creation = std::mem::zeroed();
    let mut ram_map: acrn_vm_memmap = std::mem::zeroed();
    let mut regs: acrn_vcpu_regs = std::mem::zeroed();
    let mut io_req: *mut acrn_io_request;
    let mut notify: acrn_ioreq_notify = std::mem::zeroed();

    ret = posix_memalign(&mut GUEST_MEMORY, 4096, GUEST_MEMORY_SIZE);
    if ret < 0 {
        printf(b"Not enough memory!\n\0".as_ptr() as *const i8);
        return -1;
    }
    HSM_FD = open(b"/dev/acrn_hsm\0".as_ptr() as *const i8, O_RDWR | O_CLOEXEC);

    create_vm.ioreq_buf = IO_REQ_BUF as u64;
    ret = ioctl(HSM_FD, ACRN_IOCTL_CREATE_VM, &mut create_vm);
    printf(b"Created VM! [%d]\n\0".as_ptr() as *const i8, ret);
    VCPU_NUM = create_vm.vcpu_num;
    VMID = create_vm.vmid;

    ram_map.r#type = ACRN_MEMMAP_RAM;
    ram_map.vma_base = GUEST_MEMORY as u64;
    ram_map.len = GUEST_MEMORY_SIZE as u64;
    ram_map.user_vm_pa = 0;
    ram_map.attr = ACRN_MEM_ACCESS_RWX;
    ret = ioctl(HSM_FD, ACRN_IOCTL_SET_MEMSEG, &mut ram_map);
    printf(b"Set up VM memory! [%d]\n\0".as_ptr() as *const i8, ret);

    memcpy(GUEST_MEMORY, &guest16 as *const u8 as *const c_void,
           (&guest16_end as *const u8 as usize) - (&guest16 as *const u8 as usize));

    memset(&mut regs as *mut _ as *mut c_void, 0, std::mem::size_of::<acrn_vcpu_regs>());
    regs.vcpu_id = 0;
    regs.vcpu_regs.rip = 0;
    regs.vcpu_regs.cr0 = 0x30u64;
    regs.vcpu_regs.cs_ar = 0x009fu64;
    regs.vcpu_regs.cs_sel = 0xf000u64;
    regs.vcpu_regs.cs_limit = 0xffffu64;
    regs.vcpu_regs.cs_base = 0u64 & 0xffff0000u64;
    regs.vcpu_regs.rip = 0u64 & 0xffffu64;

    ret = ioctl(HSM_FD, ACRN_IOCTL_SET_VCPU_REGS, &mut regs);
    printf(b"Set up VM BSP registers! [%d]\n\0".as_ptr() as *const i8, ret);

    ret = ioctl(HSM_FD, ACRN_IOCTL_CREATE_IOREQ_CLIENT, 0);
    printf(b"Created IO request client! [%d]\n\0".as_ptr() as *const i8, ret);
    ret = ioctl(HSM_FD, ACRN_IOCTL_START_VM, VMID);
    printf(b"Start VM! [%d]\n\0".as_ptr() as *const i8, ret);

    signal(libc::SIGINT, vm_exit);
    while IS_RUNNING != 0 {
        ret = ioctl(HSM_FD, ACRN_IOCTL_ATTACH_IOREQ_CLIENT, 0);
        let _ = ret;
        for vcpu_id in 0..(VCPU_NUM as i32) {
            io_req = IO_REQ_BUF.add(vcpu_id as usize);
            if (*io_req).processed.load(std::sync::atomic::Ordering::SeqCst) == ACRN_IOREQ_STATE_PROCESSING
                && (*io_req).kernel_handled == 0
                && (*io_req).r#type == ACRN_IOREQ_TYPE_PORTIO
            {
                let port = (*io_req).reqs.pio_request.address;
                let bytes = (*io_req).reqs.pio_request.size;
                let input = (*io_req).reqs.pio_request.direction == ACRN_IOREQ_DIR_READ;
                printf(b"Guest VM %s PIO[%x] with size[%x]\n\0".as_ptr() as *const i8,
                       if input { b"read\0".as_ptr() } else { b"write\0".as_ptr() }, port, bytes);
                notify.vmid = VMID;
                notify.vcpu = vcpu_id as u16;
                ioctl(HSM_FD, ACRN_IOCTL_NOTIFY_REQUEST_FINISH, &mut notify);
            }
        }
    }

    ret = ioctl(HSM_FD, ACRN_IOCTL_DESTROY_VM, ptr::null_mut::<c_void>());
    printf(b"Destroy VM! [%d]\n\0".as_ptr() as *const i8, ret);
    close(HSM_FD);
    free(GUEST_MEMORY);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
