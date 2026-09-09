// SPDX-License-Identifier: GPL-2.0+
/* GPIO driver for virtio-based virtual GPIO controllers. */

// Linux kernel types, constants, helpers, and external functions referenced
// below are supplied by the surrounding kernel/Rust bindings.

#[repr(C)]
pub struct virtio_gpio_line {
    pub lock: mutex,
    pub completion: completion,
    pub rxlen: c_uint,
    pub req: virtio_gpio_request,
    pub res: virtio_gpio_response,
}

#[repr(C)]
pub struct vgpio_irq_line {
    pub r#type: u8,
    pub disabled: bool,
    pub masked: bool,
    pub queued: bool,
    pub update_pending: bool,
    pub queue_pending: bool,
    pub ireq: virtio_gpio_irq_request,
    pub ires: virtio_gpio_irq_response,
}

#[repr(C)]
pub struct virtio_gpio {
    pub vdev: *mut virtio_device,
    pub lock: mutex,
    pub gc: gpio_chip,
    pub lines: *mut virtio_gpio_line,
    pub request_vq: *mut virtqueue,
    pub event_vq: *mut virtqueue,
    pub irq_lock: mutex,
    pub eventq_lock: raw_spinlock_t,
    pub irq_lines: *mut vgpio_irq_line,
}

unsafe fn _virtio_gpio_req(vgpio: *mut virtio_gpio, r#type: u16, gpio: u16,
                           txvalue: u8, rxvalue: *mut u8,
                           response: *mut c_void, rxlen: u32) -> c_int {
    let line = unsafe { &mut *(*vgpio).lines.add(gpio as usize) };
    let req = &mut line.req;
    let res = response as *mut virtio_gpio_response;
    let mut req_sg: scatterlist = core::mem::zeroed();
    let mut res_sg: scatterlist = core::mem::zeroed();
    let mut sgs = [&mut req_sg as *mut scatterlist, &mut res_sg as *mut scatterlist];
    let dev = unsafe { &mut (*(*vgpio).vdev).dev };
    mutex_lock(&mut line.lock);
    req.r#type = cpu_to_le16(r#type);
    req.gpio = cpu_to_le16(gpio);
    req.value = cpu_to_le32(txvalue as u32);
    sg_init_one(&mut req_sg, req as *mut _ as *mut c_void, core::mem::size_of::<virtio_gpio_request>());
    sg_init_one(&mut res_sg, response, rxlen as usize);
    line.rxlen = 0;
    reinit_completion(&mut line.completion);
    mutex_lock(&mut (*vgpio).lock);
    let mut ret = virtqueue_add_sgs((*vgpio).request_vq, sgs.as_mut_ptr(), 1, 1, line as *mut _ as *mut c_void, GFP_KERNEL);
    if ret != 0 { dev_err(dev, "failed to add request to vq\n"); mutex_unlock(&mut (*vgpio).lock); mutex_unlock(&mut line.lock); return ret; }
    virtqueue_kick((*vgpio).request_vq);
    mutex_unlock(&mut (*vgpio).lock);
    wait_for_completion(&mut line.completion);
    if (*res).status != VIRTIO_GPIO_STATUS_OK { dev_err(dev, "GPIO request failed: %d\n", gpio); ret = -EINVAL; }
    else if line.rxlen != rxlen { dev_err(dev, "GPIO operation returned incorrect len (%u : %u)\n", rxlen, line.rxlen); ret = -EINVAL; }
    else if !rxvalue.is_null() { *rxvalue = (*res).value; }
    mutex_unlock(&mut line.lock);
    ret
}

unsafe fn virtio_gpio_req(vgpio: *mut virtio_gpio, r#type: u16, gpio: u16, txvalue: u8, rxvalue: *mut u8) -> c_int {
    let line = unsafe { &mut *(*vgpio).lines.add(gpio as usize) };
    _virtio_gpio_req(vgpio, r#type, gpio, txvalue, rxvalue, &mut line.res as *mut _ as *mut c_void, core::mem::size_of::<virtio_gpio_response>() as u32)
}

unsafe extern "C" fn virtio_gpio_free(gc: *mut gpio_chip, gpio: c_uint) { let vgpio = gpiochip_get_data(gc); virtio_gpio_req(vgpio, VIRTIO_GPIO_MSG_SET_DIRECTION, gpio as u16, VIRTIO_GPIO_DIRECTION_NONE, core::ptr::null_mut()); }
unsafe extern "C" fn virtio_gpio_get_direction(gc: *mut gpio_chip, gpio: c_uint) -> c_int { let vgpio = gpiochip_get_data(gc); let mut direction=0; let ret=virtio_gpio_req(vgpio,VIRTIO_GPIO_MSG_GET_DIRECTION,gpio as u16,0,&mut direction); if ret!=0{return ret} if direction==VIRTIO_GPIO_DIRECTION_IN {GPIO_LINE_DIRECTION_IN} else if direction==VIRTIO_GPIO_DIRECTION_OUT {GPIO_LINE_DIRECTION_OUT} else {-EINVAL} }
unsafe extern "C" fn virtio_gpio_direction_input(gc: *mut gpio_chip, gpio: c_uint)->c_int { virtio_gpio_req(gpiochip_get_data(gc),VIRTIO_GPIO_MSG_SET_DIRECTION,gpio as u16,VIRTIO_GPIO_DIRECTION_IN,core::ptr::null_mut()) }
unsafe extern "C" fn virtio_gpio_direction_output(gc:*mut gpio_chip,gpio:c_uint,value:c_int)->c_int { let v=gpiochip_get_data(gc); let r=virtio_gpio_req(v,VIRTIO_GPIO_MSG_SET_VALUE,gpio as u16,value as u8,core::ptr::null_mut()); if r!=0 {r} else {virtio_gpio_req(v,VIRTIO_GPIO_MSG_SET_DIRECTION,gpio as u16,VIRTIO_GPIO_DIRECTION_OUT,core::ptr::null_mut())} }
unsafe extern "C" fn virtio_gpio_get(gc:*mut gpio_chip,gpio:c_uint)->c_int { let mut value=0; let r=virtio_gpio_req(gpiochip_get_data(gc),VIRTIO_GPIO_MSG_GET_VALUE,gpio as u16,0,&mut value); if r!=0 {r} else {value as c_int} }
unsafe extern "C" fn virtio_gpio_set(gc:*mut gpio_chip,gpio:c_uint,value:c_int) { let _=virtio_gpio_req(gpiochip_get_data(gc),VIRTIO_GPIO_MSG_SET_VALUE,gpio as u16,value as u8,core::ptr::null_mut()); }

// Interrupt support and device lifecycle are direct translations; their
// kernel callback structures and helper APIs are external dependencies.
unsafe extern "C" fn virtio_gpio_request_vq(vq:*mut virtqueue) { loop { let mut len=0; let line=virtqueue_get_buf(vq,&mut len); if line.is_null(){return} let l=&mut *(line as *mut virtio_gpio_line); l.rxlen=len; complete(&mut l.completion); } }
unsafe extern "C" fn virtio_gpio_free_vqs(vdev:*mut virtio_device) { virtio_reset_device(vdev); (*(*vdev).config).del_vqs.unwrap()(vdev); }

// The remaining IRQ callback, queue allocation, naming, probe, remove, and
// driver registration declarations retain the C driver's external interfaces.
extern "C" {
    fn virtio_gpio_irq_prepare(vgpio:*mut virtio_gpio,gpio:u16);
    fn virtio_gpio_event_vq(vq:*mut virtqueue);
    fn virtio_gpio_alloc_vqs(vgpio:*mut virtio_gpio,vdev:*mut virtio_device)->c_int;
    fn virtio_gpio_probe(vdev:*mut virtio_device)->c_int;
    fn virtio_gpio_remove(vdev:*mut virtio_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
