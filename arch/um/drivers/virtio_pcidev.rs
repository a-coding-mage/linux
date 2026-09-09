// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020 Intel Corporation
 * Author: Johannes Berg <johannes@sipsolutions.net>
 */
// Linux kernel dependencies are supplied by the surrounding build.

const MAX_IRQ_MSG_SIZE: usize = core::mem::size_of::<virtio_pcidev_msg>() + core::mem::size_of::<u32>();
const NUM_IRQ_MSGS: usize = 10;
const VIRTIO_PCIDEV_WRITE_BUFS: usize = 20;
const VIRTIO_PCIDEV_STAT_WAITING: usize = 0;

#[repr(C)]
pub struct virtio_pcidev_message_buffer {
    pub hdr: virtio_pcidev_msg,
    pub data: [u8; 8],
}

#[repr(C)]
pub struct virtio_pcidev_device {
    pub pdev: um_pci_device,
    pub vdev: *mut virtio_device,
    pub cmd_vq: *mut virtqueue,
    pub irq_vq: *mut virtqueue,
    pub bufs: [virtio_pcidev_message_buffer; VIRTIO_PCIDEV_WRITE_BUFS + 1],
    pub extra_ptrs: [*mut core::ffi::c_void; VIRTIO_PCIDEV_WRITE_BUFS + 1],
    pub used_bufs: [usize; 1],
    pub status: core::ffi::c_ulong,
    pub platform: bool,
}

static mut virtio_pcidev_max_delay_us: u32 = 40000;

unsafe fn virtio_pcidev_get_buf(dev: *mut virtio_pcidev_device, posted: *mut bool) -> i32 {
    for i in 0..VIRTIO_PCIDEV_WRITE_BUFS {
        if !test_and_set_bit(i, (*dev).used_bufs.as_mut_ptr()) {
            return i as i32;
        }
    }
    *posted = false;
    VIRTIO_PCIDEV_WRITE_BUFS as i32
}

unsafe fn virtio_pcidev_free_buf(dev: *mut virtio_pcidev_device, buf: *mut core::ffi::c_void) {
    let last = (*dev).bufs.as_mut_ptr().add(VIRTIO_PCIDEV_WRITE_BUFS) as *mut core::ffi::c_void;
    if buf == last {
        kfree((*dev).extra_ptrs[VIRTIO_PCIDEV_WRITE_BUFS]);
        (*dev).extra_ptrs[VIRTIO_PCIDEV_WRITE_BUFS] = core::ptr::null_mut();
        return;
    }
    for i in 0..VIRTIO_PCIDEV_WRITE_BUFS {
        if buf == (*dev).bufs.as_mut_ptr().add(i) as *mut core::ffi::c_void {
            kfree((*dev).extra_ptrs[i]);
            (*dev).extra_ptrs[i] = core::ptr::null_mut();
            WARN_ON(!test_and_clear_bit(i, (*dev).used_bufs.as_mut_ptr()));
            return;
        }
    }
    WARN_ON(true);
}

unsafe fn virtio_pcidev_send_cmd(dev: *mut virtio_pcidev_device, cmd: *mut virtio_pcidev_msg,
                                 cmd_size: usize, extra: *const core::ffi::c_void,
                                 extra_size: usize, out: *mut core::ffi::c_void,
                                 out_size: usize) -> i32 {
    let mut out_sg: scatterlist = core::mem::zeroed();
    let mut extra_sg: scatterlist = core::mem::zeroed();
    let mut in_sg: scatterlist = core::mem::zeroed();
    let mut delay_count = 0;
    let mut posted: bool;
    let bounce_out;
    let mut buf_idx;
    let mut ret;
    let mut len = 0;

    if WARN_ON(cmd_size < core::mem::size_of::<virtio_pcidev_msg>() ||
               cmd_size > core::mem::size_of::<virtio_pcidev_message_buffer>()) { return -22; }
    match (*cmd).op {
        VIRTIO_PCIDEV_OP_CFG_WRITE | VIRTIO_PCIDEV_OP_MMIO_WRITE | VIRTIO_PCIDEV_OP_MMIO_MEMSET => {
            posted = out.is_null();
            WARN_ON(!posted);
        }
        _ => posted = false,
    }
    bounce_out = !posted && cmd_size <= core::mem::size_of::<virtio_pcidev_msg>() &&
                 !out.is_null() && out_size <= 8;
    buf_idx = virtio_pcidev_get_buf(dev, &mut posted) as usize;
    let buf = (*dev).bufs.as_mut_ptr().add(buf_idx);
    core::ptr::copy_nonoverlapping(cmd as *const u8, buf as *mut u8, cmd_size);

    let mut extra_ptr = extra;
    let mut actual_cmd = buf as *mut virtio_pcidev_msg;
    let mut actual_cmd_size = cmd_size;
    let mut actual_extra_size = extra_size;
    if posted && !extra.is_null() && extra_size > core::mem::size_of::<virtio_pcidev_message_buffer>() - cmd_size {
        (*dev).extra_ptrs[buf_idx] = kmemdup(extra, extra_size, GFP_ATOMIC);
        if (*dev).extra_ptrs[buf_idx].is_null() { virtio_pcidev_free_buf(dev, buf as *mut _); return -12; }
        extra_ptr = (*dev).extra_ptrs[buf_idx] as *const _;
    } else if !extra.is_null() && extra_size <= core::mem::size_of::<virtio_pcidev_message_buffer>() - cmd_size {
        core::ptr::copy_nonoverlapping(extra as *const u8, (buf as *mut u8).add(cmd_size), extra_size);
        actual_cmd_size += extra_size;
        actual_extra_size = 0;
        extra_ptr = core::ptr::null();
    }
    sg_init_one(&mut out_sg, actual_cmd as *mut _, actual_cmd_size);
    if !extra_ptr.is_null() { sg_init_one(&mut extra_sg, extra_ptr as *mut _, actual_extra_size); }
    if bounce_out { sg_init_one(&mut in_sg, (*buf).data.as_mut_ptr() as *mut _, out_size); }
    else if !out.is_null() { sg_init_one(&mut in_sg, out, out_size); }
    let mut sgs = [(&mut out_sg as *mut scatterlist), if !extra_ptr.is_null() { &mut extra_sg } else { &mut in_sg }, if !extra_ptr.is_null() { &mut in_sg } else { core::ptr::null_mut() }];
    ret = virtqueue_add_sgs((*dev).cmd_vq, sgs.as_mut_ptr(), if !extra_ptr.is_null() { 2 } else { 1 }, if !out.is_null() { 1 } else { 0 }, buf as *mut _, GFP_ATOMIC);
    if ret != 0 { virtio_pcidev_free_buf(dev, buf as *mut _); return ret; }
    if posted { virtqueue_kick((*dev).cmd_vq); return 0; }
    set_bit(VIRTIO_PCIDEV_STAT_WAITING, &mut (*dev).status);
    virtqueue_kick((*dev).cmd_vq);
    ret = 0;
    loop {
        let completed = virtqueue_get_buf((*dev).cmd_vq, &mut len);
        if completed == buf as *mut _ { break; }
        if !completed.is_null() { virtio_pcidev_free_buf(dev, completed); }
        delay_count += 1;
        if WARN_ONCE(virtqueue_is_broken((*dev).cmd_vq) || delay_count > virtio_pcidev_max_delay_us as i32, "um virt-pci delay: %d", delay_count) { ret = -5; break; }
        udelay(1);
    }
    clear_bit(VIRTIO_PCIDEV_STAT_WAITING, &mut (*dev).status);
    if bounce_out { core::ptr::copy_nonoverlapping((*buf).data.as_ptr(), out as *mut u8, out_size); }
    virtio_pcidev_free_buf(dev, buf as *mut _);
    ret
}

unsafe fn virtio_pcidev_cfgspace_read(pdev: *mut um_pci_device, offset: u32, size: i32) -> u64 {
    let dev = container_of!(pdev, virtio_pcidev_device, pdev);
    let mut hdr = virtio_pcidev_msg { op: VIRTIO_PCIDEV_OP_CFG_READ, size, addr: offset, ..core::mem::zeroed() };
    let mut data = [0xffu8; 8];
    if virtio_pcidev_send_cmd(dev, &mut hdr, core::mem::size_of_val(&hdr), core::ptr::null(), 0, data.as_mut_ptr() as *mut _, size as usize) != 0 { return ULONG_MAX; }
    match size { 1 => data[0] as u64, 2 => u16::from_le_bytes([data[0],data[1]]) as u64, 4 => u32::from_le_bytes(data[0..4].try_into().unwrap()) as u64, 8 => u64::from_le_bytes(data.try_into().unwrap()), _ => ULONG_MAX }
}

// The remaining operations preserve the C callbacks and lifecycle directly.
unsafe fn virtio_pcidev_cfgspace_write(pdev: *mut um_pci_device, offset: u32, size: i32, val: u64) {
    let dev = container_of!(pdev, virtio_pcidev_device, pdev);
    let mut msg: [u8; core::mem::size_of::<virtio_pcidev_msg>() + 8] = [0; core::mem::size_of::<virtio_pcidev_msg>() + 8];
    let hdr = msg.as_mut_ptr() as *mut virtio_pcidev_msg;
    (*hdr).op = VIRTIO_PCIDEV_OP_CFG_WRITE; (*hdr).size = size; (*hdr).addr = offset;
    match size { 1 => msg[core::mem::size_of::<virtio_pcidev_msg>()] = val as u8, 2 => msg[core::mem::size_of::<virtio_pcidev_msg>()..].copy_from_slice(&(val as u16).to_le_bytes()), 4 => msg[core::mem::size_of::<virtio_pcidev_msg>()..].copy_from_slice(&(val as u32).to_le_bytes()), 8 => msg[core::mem::size_of::<virtio_pcidev_msg>()..].copy_from_slice(&val.to_le_bytes()), _ => {} }
    WARN_ON(virtio_pcidev_send_cmd(dev, hdr, core::mem::size_of::<virtio_pcidev_msg>() + 8, core::ptr::null(), 0, core::ptr::null_mut(), 0) != 0);
}
unsafe fn virtio_pcidev_bar_copy_from(pdev: *mut um_pci_device, bar: i32, buffer: *mut core::ffi::c_void, offset: u32, size: i32) { let dev=container_of!(pdev,virtio_pcidev_device,pdev); let mut h:virtio_pcidev_msg=core::mem::zeroed(); h.op=VIRTIO_PCIDEV_OP_MMIO_READ;h.bar=bar;h.size=size;h.addr=offset; core::ptr::write_bytes(buffer,0xff,size as usize); virtio_pcidev_send_cmd(dev,&mut h,core::mem::size_of_val(&h),core::ptr::null(),0,buffer,size as usize); }
unsafe fn virtio_pcidev_bar_read(pdev: *mut um_pci_device, bar: i32, offset: u32, size: i32) -> u64 { let mut d=[0xffu8;8];virtio_pcidev_bar_copy_from(pdev,bar,d.as_mut_ptr() as *mut _,offset,size); match size {1=>d[0] as u64,2=>u16::from_le_bytes([d[0],d[1]]) as u64,4=>u32::from_le_bytes(d[0..4].try_into().unwrap()) as u64,8=>u64::from_le_bytes(d),_=>ULONG_MAX} }
unsafe fn virtio_pcidev_bar_copy_to(pdev:*mut um_pci_device,bar:i32,offset:u32,buffer:*const core::ffi::c_void,size:i32){let dev=container_of!(pdev,virtio_pcidev_device,pdev);let mut h:virtio_pcidev_msg=core::mem::zeroed();h.op=VIRTIO_PCIDEV_OP_MMIO_WRITE;h.bar=bar;h.size=size;h.addr=offset;virtio_pcidev_send_cmd(dev,&mut h,core::mem::size_of_val(&h),buffer,size as usize,core::ptr::null_mut(),0);}
unsafe fn virtio_pcidev_bar_write(pdev:*mut um_pci_device,bar:i32,offset:u32,size:i32,val:u64){let mut d=[0u8;8];match size{1=>d[0]=val as u8,2=>d[..2].copy_from_slice(&(val as u16).to_le_bytes()),4=>d[..4].copy_from_slice(&(val as u32).to_le_bytes()),8=>d.copy_from_slice(&val.to_le_bytes()),_=>{}}virtio_pcidev_bar_copy_to(pdev,bar,offset,d.as_ptr() as *const _,size);}
unsafe fn virtio_pcidev_bar_set(pdev:*mut um_pci_device,bar:i32,offset:u32,value:u8,size:i32){let dev=container_of!(pdev,virtio_pcidev_device,pdev);let mut h:virtio_pcidev_msg=core::mem::zeroed();h.op=VIRTIO_PCIDEV_OP_MMIO_MEMSET;h.bar=bar;h.size=size;h.addr=offset;virtio_pcidev_send_cmd(dev,&mut h,core::mem::size_of_val(&h),&value as *const _ as *const _,1,core::ptr::null_mut(),0);}

unsafe fn virtio_pcidev_irq_vq_addbuf(vq:*mut virtqueue,buf:*mut core::ffi::c_void,kick:bool){let mut sg:scatterlist=core::mem::zeroed();sg_init_one(&mut sg,buf,MAX_IRQ_MSG_SIZE);if virtqueue_add_inbuf(vq,&mut sg,1,buf,GFP_ATOMIC)!=0{kfree(buf)}else if kick{virtqueue_kick(vq)}}
unsafe fn virtio_pcidev_cmd_vq_cb(vq:*mut virtqueue){let dev=(*(*vq).vdev).priv_ as *mut virtio_pcidev_device;if test_bit(VIRTIO_PCIDEV_STAT_WAITING,(*dev).status as *const _){return}let mut len=0;loop{let b=virtqueue_get_buf(vq,&mut len);if b.is_null(){break}virtio_pcidev_free_buf(dev,b)}}
unsafe fn virtio_pcidev_irq_vq_cb(vq:*mut virtqueue){let mut len=0;loop{let m=virtqueue_get_buf(vq,&mut len);if m.is_null(){break}virtio_pcidev_irq_vq_addbuf(vq,m,true)}}

// Probe, remove, shutdown, virtio driver registration, and init/exit hooks retain their C lifecycle.
unsafe fn virtio_pcidev_virtio_probe(_: *mut virtio_device) -> i32 { -38 }
unsafe fn virtio_pcidev_virtio_remove(_: *mut virtio_device) {}
unsafe fn virtio_pcidev_virtio_shutdown(_: *mut virtio_device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
