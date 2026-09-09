// SPDX-License-Identifier: GPL-2.0+
/* Translation of module/drivers.c. Kernel includes and external declarations
 * are supplied by the surrounding repository. */

pub static mut comedi_drivers: *mut comedi_driver = core::ptr::null_mut();
pub static mut comedi_drivers_list_lock: mutex = mutex {};

pub unsafe fn comedi_set_hw_dev(dev: *mut comedi_device, hw_dev: *mut device) -> i32 {
    if hw_dev == (*dev).hw_dev { return 0; }
    if !(*dev).hw_dev.is_null() { return -EEXIST; }
    (*dev).hw_dev = get_device(hw_dev);
    0
}

unsafe fn comedi_clear_hw_dev(dev: *mut comedi_device) {
    put_device((*dev).hw_dev);
    (*dev).hw_dev = core::ptr::null_mut();
}

pub unsafe fn comedi_alloc_devpriv(dev: *mut comedi_device, size: usize) -> *mut core::ffi::c_void {
    (*dev).private = kzalloc(size, GFP_KERNEL);
    (*dev).private
}

pub unsafe fn comedi_alloc_subdevices(dev: *mut comedi_device, num_subdevices: i32) -> i32 {
    if num_subdevices < 1 { return -EINVAL; }
    let mut s = kzalloc_objs::<comedi_subdevice>(num_subdevices as usize, GFP_KERNEL);
    if s.is_null() { return -ENOMEM; }
    (*dev).subdevices = s;
    (*dev).n_subdevices = num_subdevices;
    for i in 0..num_subdevices {
        s = (*dev).subdevices.add(i as usize);
        (*s).device = dev;
        (*s).index = i;
        (*s).async_dma_dir = DMA_NONE;
        spin_lock_init(&mut (*s).spin_lock);
        (*s).minor = -1;
    }
    0
}

pub unsafe fn comedi_alloc_subdev_readback(s: *mut comedi_subdevice) -> i32 {
    if (*s).n_chan == 0 { return -EINVAL; }
    (*s).readback = kcalloc((*s).n_chan as usize, core::mem::size_of::<u32>(), GFP_KERNEL);
    if (*s).readback.is_null() { return -ENOMEM; }
    if (*s).insn_read.is_none() { (*s).insn_read = Some(comedi_readback_insn_read); }
    0
}

unsafe fn comedi_device_detach_cleanup(dev: *mut comedi_device) {
    if !(*dev).subdevices.is_null() {
        for i in 0..(*dev).n_subdevices {
            let s = (*dev).subdevices.add(i as usize);
            if comedi_can_auto_free_spriv(s) { kfree((*s).private); }
            comedi_free_subdevice_minor(s);
            if !(*s).async_.is_null() {
                comedi_buf_alloc(dev, s, 0);
                kfree((*s).async_ as *mut _);
            }
            kfree((*s).readback as *mut _);
        }
        kfree((*dev).subdevices as *mut _);
        (*dev).subdevices = core::ptr::null_mut();
        (*dev).n_subdevices = 0;
    }
    kfree((*dev).private);
    if !IS_ERR((*dev).pacer) { kfree((*dev).pacer as *mut _); }
    (*dev).private = core::ptr::null_mut(); (*dev).pacer = core::ptr::null_mut();
    (*dev).driver = core::ptr::null_mut(); (*dev).board_name = core::ptr::null();
    (*dev).board_ptr = core::ptr::null_mut(); (*dev).mmio = core::ptr::null_mut();
    (*dev).iobase = 0; (*dev).iolen = 0; (*dev).ioenabled = false; (*dev).irq = 0;
    (*dev).read_subdev = core::ptr::null_mut(); (*dev).write_subdev = core::ptr::null_mut();
    (*dev).open = None; (*dev).close = None;
    comedi_clear_hw_dev(dev);
}

pub unsafe fn comedi_device_detach_locked(dev: *mut comedi_device) {
    comedi_device_cancel_all(dev); (*dev).attached = false; (*dev).detach_count += 1;
    if !(*dev).driver.is_null() { ((*(*dev).driver).detach)(dev); }
    comedi_device_detach_cleanup(dev);
}

pub unsafe fn comedi_device_detach(dev: *mut comedi_device) {
    down_write(&mut (*dev).attach_lock); comedi_device_detach_locked(dev); up_write(&mut (*dev).attach_lock);
}

unsafe fn poll_invalid(_: *mut comedi_device, _: *mut comedi_subdevice) -> i32 { -EINVAL }
unsafe fn insn_device_inval(_: *mut comedi_device, _: *mut comedi_insn, _: *mut u32) -> i32 { -EINVAL }
unsafe fn get_zero_valid_routes(_: *mut comedi_device, _: u32, _: *mut u32) -> u32 { 0 }
pub unsafe fn insn_inval(_: *mut comedi_device, _: *mut comedi_subdevice, _: *mut comedi_insn, _: *mut u32) -> i32 { -EINVAL }

pub unsafe fn comedi_readback_insn_read(_: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32) -> i32 {
    if (*s).readback.is_null() { return -EINVAL; }
    let chan = CR_CHAN((*insn).chanspec);
    for i in 0..(*insn).n { *data.add(i as usize) = *(*s).readback.add(chan as usize); }
    (*insn).n as i32
}

pub unsafe fn comedi_timeout(dev: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, cb: unsafe fn(*mut comedi_device,*mut comedi_subdevice,*mut comedi_insn,usize)->i32, context: usize) -> i32 {
    let timeout = jiffies() + msecs_to_jiffies(COMEDI_TIMEOUT_MS);
    while time_before(jiffies(), timeout) { let ret = cb(dev,s,insn,context); if ret != -EBUSY { return ret; } cpu_relax(); }
    -ETIMEDOUT
}

pub unsafe fn comedi_dio_insn_config(_: *mut comedi_device, s: *mut comedi_subdevice, insn: *mut comedi_insn, data: *mut u32, mut mask: u32) -> i32 {
    let chan = CR_CHAN((*insn).chanspec); if mask == 0 && chan < 32 { mask = 1u32 << chan; }
    match *data { INSN_CONFIG_DIO_INPUT => (*s).io_bits &= !mask, INSN_CONFIG_DIO_OUTPUT => (*s).io_bits |= mask,
        INSN_CONFIG_DIO_QUERY => { *data.add(1) = if (*s).io_bits & mask != 0 { COMEDI_OUTPUT } else { COMEDI_INPUT }; return (*insn).n as i32; },
        _ => return -EINVAL }
    0
}

pub unsafe fn comedi_dio_update_state(s: *mut comedi_subdevice, data: *mut u32) -> u32 {
    let chanmask = if (*s).n_chan < 32 { (1u32 << (*s).n_chan) - 1 } else { 0xffffffff };
    let mask = *data & chanmask; if mask != 0 { (*s).state &= !mask; (*s).state |= *data.add(1) & mask; } mask
}

pub unsafe fn comedi_bytes_per_scan_cmd(s: *mut comedi_subdevice, cmd: *mut comedi_cmd) -> u32 {
    let n = match (*s).type_ { COMEDI_SUBD_DI|COMEDI_SUBD_DO|COMEDI_SUBD_DIO => { let b=8*comedi_bytes_per_sample(s); DIV_ROUND_UP((*cmd).scan_end_arg,b) }, _ => (*cmd).scan_end_arg };
    comedi_samples_to_bytes(s,n)
}
unsafe fn _comedi_bytes_per_scan(s: *mut comedi_subdevice) -> u32 { comedi_bytes_per_scan_cmd(s,&mut (*(*s).async_).cmd) }
pub unsafe fn comedi_bytes_per_scan(s: *mut comedi_subdevice) -> u32 { if comedi_get_is_subdevice_running(s) { let n=_comedi_bytes_per_scan(s); comedi_put_is_subdevice_running(s); n } else { comedi_samples_to_bytes(s,1) } }

unsafe fn __comedi_nscans_left(s:*mut comedi_subdevice, mut n:u32)->u32 { let a=(*s).async_; let c=&(*a).cmd; if c.stop_src==TRIG_COUNT { let left=if (*a).scans_done<c.stop_arg { c.stop_arg-(*a).scans_done } else {0}; if n>left {n=left;} } n }
unsafe fn _comedi_nscans_left(s:*mut comedi_subdevice, mut n:u32)->u32 { if n==0 { n=_comedi_buf_read_n_available(s)/_comedi_bytes_per_scan(s); } __comedi_nscans_left(s,n) }
pub unsafe fn comedi_nscans_left(s:*mut comedi_subdevice, mut n:u32)->u32 { if comedi_get_is_subdevice_running(s) {n=_comedi_nscans_left(s,n);comedi_put_is_subdevice_running(s);} else {n=0;} n }
unsafe fn _comedi_nsamples_left(s:*mut comedi_subdevice, n:u32)->u32 { let a=(*s).async_; let c=&(*a).cmd; if c.stop_src!=TRIG_COUNT{return n;} let scans=__comedi_nscans_left(s,c.stop_arg) as u64; if scans==0{return 0;} let left=scans*c.scan_end_arg as u64-comedi_bytes_to_samples(s,(*a).scan_progress) as u64; if left<n as u64 {left as u32}else{n} }
pub unsafe fn comedi_nsamples_left(s:*mut comedi_subdevice, mut n:u32)->u32 {if comedi_get_is_subdevice_running(s){n=_comedi_nsamples_left(s,n);comedi_put_is_subdevice_running(s)}else{n=0}n}

pub unsafe fn _comedi_inc_scan_progress(s:*mut comedi_subdevice,num:u32){let a=(*s).async_;let c=&(*a).cmd;let len=_comedi_bytes_per_scan(s);if (*s).subdev_flags&SDF_PACKED==0{(*a).cur_chan+=comedi_bytes_to_samples(s,num);(*a).cur_chan%=c.chanlist_len;}(*a).scan_progress+=num;if (*a).scan_progress>=len{let n=(*a).scan_progress/len; if (*a).scans_done<UINT_MAX-n{(*a).scans_done+=n}else{(*a).scans_done=UINT_MAX}(*a).scan_progress%=len;(*a).events|=COMEDI_CB_EOS}}
pub unsafe fn comedi_inc_scan_progress(s:*mut comedi_subdevice,n:u32){if comedi_get_is_subdevice_running(s){_comedi_inc_scan_progress(s,n);comedi_put_is_subdevice_running(s)}}
unsafe fn _comedi_handle_events(dev:*mut comedi_device,s:*mut comedi_subdevice)->u32{let e=(*(*s).async_).events;if e==0{return e}if e&COMEDI_CB_CANCEL_MASK!=0{if let Some(f)=(*s).cancel{f(dev,s)}}_comedi_event(dev,s);e}
pub unsafe fn comedi_handle_events(dev:*mut comedi_device,s:*mut comedi_subdevice)->u32{if comedi_get_is_subdevice_running(s){let e=_comedi_handle_events(dev,s);comedi_put_is_subdevice_running(s);e}else{0}}

unsafe fn insn_rw_emulate_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32{let mut x=comedi_insn::default();let chan=CR_CHAN((*insn).chanspec);let base=if chan<32{0}else{chan};let mut d=[0u32;2];x.insn=INSN_BITS;x.chanspec=base;x.n=2;x.subdev=(*insn).subdev;if (*insn).insn==INSN_WRITE{if (*s).subdev_flags&SDF_WRITABLE==0{return -EINVAL}d[0]=1u32<<(chan-base);}for i in 0..(*insn).n as usize{if (*insn).insn==INSN_WRITE{d[1]=if *data.add(i)!=0{d[0]}else{0}}let r=((*s).insn_bits.unwrap())(dev,s,&mut x,d.as_mut_ptr());if r<0{return r}if (*insn).insn==INSN_READ{*data.add(i)=(d[1]>>(chan-base))&1}}(*insn).n as i32}

unsafe fn __comedi_device_postconfig_async(dev:*mut comedi_device,s:*mut comedi_subdevice)->i32{if (*s).subdev_flags&(SDF_CMD_READ|SDF_CMD_WRITE)==0{return -EINVAL}if (*s).do_cmdtest.is_none(){return -EINVAL}let a=kzalloc_obj::<comedi_async>();if a.is_null(){return -ENOMEM}init_waitqueue_head(&mut (*a).wait_head);init_completion(&mut (*a).run_complete);(*s).async_=a;(*a).max_bufsize=comedi_default_buf_maxsize_kb*1024;let mut size=comedi_default_buf_size_kb*1024;if size>(*a).max_bufsize{size=(*a).max_bufsize}if comedi_buf_alloc(dev,s,size)<0{return -ENOMEM}if let Some(f)=(*s).buf_change{let r=f(dev,s);if r<0{return r}}comedi_alloc_subdevice_minor(s);0}
unsafe fn __comedi_device_postconfig(dev:*mut comedi_device)->i32{if (*dev).insn_device_config.is_none(){(*dev).insn_device_config=Some(insn_device_inval)}if (*dev).get_valid_routes.is_none(){(*dev).get_valid_routes=Some(get_zero_valid_routes)}for i in 0..(*dev).n_subdevices as usize{let s=(*dev).subdevices.add(i);if (*s).type_==COMEDI_SUBD_UNUSED{continue}if (*s).type_==COMEDI_SUBD_DO{(*s).io_bits=if (*s).n_chan<32{(1u32<<(*s).n_chan)-1}else{0xffffffff}}if (*s).len_chanlist==0{(*s).len_chanlist=1}if (*s).do_cmd.is_some(){let r=__comedi_device_postconfig_async(dev,s);if r!=0{return r}}if (*s).range_table.is_null()&&(*s).range_table_list.is_null(){(*s).range_table=&mut range_unknown}if (*s).insn_read.is_none()&&(*s).insn_bits.is_some(){(*s).insn_read=Some(insn_rw_emulate_bits)}if (*s).insn_write.is_none()&&(*s).insn_bits.is_some(){(*s).insn_write=Some(insn_rw_emulate_bits)}if (*s).insn_read.is_none(){(*s).insn_read=Some(insn_inval)}if (*s).insn_write.is_none(){(*s).insn_write=Some(insn_inval)}if (*s).insn_bits.is_none(){(*s).insn_bits=Some(insn_inval)}if (*s).insn_config.is_none(){(*s).insn_config=Some(insn_inval)}if (*s).poll.is_none(){(*s).poll=Some(poll_invalid)}}0}
unsafe fn comedi_device_postconfig(dev:*mut comedi_device)->i32{let r=__comedi_device_postconfig(dev);if r<0{return r}down_write(&mut (*dev).attach_lock);(*dev).attached=true;up_write(&mut (*dev).attach_lock);0}

unsafe fn comedi_recognize(driv:*mut comedi_driver,name:*const i8)->*mut core::ffi::c_void{let mut p=(*driv).board_name as *mut *mut i8;for _ in 0..(*driv).num_names{if strcmp(*p,name)==0{return p as *mut _}p=(p as *mut u8).add((*driv).offset as usize) as *mut *mut i8}core::ptr::null_mut()}
unsafe fn comedi_report_boards(d:*mut comedi_driver){let mut p=(*d).board_name as *const *const i8;for _ in 0..(*d).num_names{pr_info(" %s\n",*p);p=(p as *const u8).add((*d).offset as usize) as *const *const i8}if (*d).num_names==0{pr_info(" %s\n",(*d).driver_name)}}
pub unsafe fn comedi_load_firmware(dev:*mut comedi_device,device:*mut device,name:*const i8,cb:Option<unsafe fn(*mut comedi_device,*const u8,usize,usize)->i32>,context:usize)->i32{let f: *const firmware=core::ptr::null();if cb.is_none(){return -EINVAL}let mut fw=f;let mut r=request_firmware(&mut fw,name,device);if r==0{r=cb.unwrap()(dev,(*fw).data,(*fw).size,context);release_firmware(fw)}if r<0{r}else{0}}

pub unsafe fn __comedi_check_request_region(dev:*mut comedi_device,start:usize,len:usize,minstart:usize,maxend:usize,minalign:usize)->i32{if start==0{return -EINVAL}if start<minstart||start>maxend||maxend-start<len-1{return -EINVAL}if !IS_ALIGNED(start,minalign){return -EINVAL}if request_region(start,len,(*dev).board_name).is_null(){return -EIO}0}
pub unsafe fn comedi_check_request_region(dev:*mut comedi_device,start:usize,len:usize,minstart:usize,maxend:usize,minalign:usize)->i32{let r=__comedi_check_request_region(dev,start,len,minstart,maxend,minalign);if r==0{(*dev).iobase=start;(*dev).iolen=len}r}
pub unsafe fn comedi_legacy_detach(dev:*mut comedi_device){if (*dev).irq!=0{free_irq((*dev).irq,dev);(*dev).irq=0}if (*dev).iobase!=0&&(*dev).iolen!=0{release_region((*dev).iobase,(*dev).iolen);(*dev).iobase=0;(*dev).iolen=0}}

pub unsafe fn comedi_driver_register(d:*mut comedi_driver)->i32{(*d).next=comedi_drivers;comedi_drivers=d;0}
pub unsafe fn comedi_auto_unconfig(hw:*mut device){if !hw.is_null(){comedi_release_hardware_device(hw)}}

pub unsafe fn comedi_device_attach(dev:*mut comedi_device,it:*mut comedi_devconfig)->i32{if (*dev).attached{return -EBUSY}for mut d=comedi_drivers;!d.is_null();d=(*d).next{if !try_module_get((*d).module){continue}if (*d).num_names!=0{(*dev).board_ptr=comedi_recognize(d,(*it).board_name);if !(*dev).board_ptr.is_null(){(*dev).driver=d;break}}else if strcmp((*d).driver_name,(*it).board_name)==0{(*dev).driver=d;break}module_put((*d).module)}if (*dev).driver.is_null(){return -EIO}if (*(*dev).driver).attach.is_none(){module_put((*(*dev).driver).module);return -EIO}(*dev).board_name=if !(*dev).board_ptr.is_null(){*((*dev).board_ptr as *const *const i8)}else{(*(*dev).driver).driver_name};let r=((*(*dev).driver).attach.unwrap())(dev,it);let r=if r>=0{comedi_device_postconfig(dev)}else{r};if r<0{comedi_device_detach(dev);module_put((*(*dev).driver).module)}r}

pub unsafe fn comedi_auto_config(hw:*mut device,driver:*mut comedi_driver,context:usize)->i32{if hw.is_null()||driver.is_null()||(*driver).auto_attach.is_none(){return -EINVAL}let dev=comedi_alloc_board_minor(hw);if IS_ERR(dev){return PTR_ERR(dev)}(*dev).driver=driver;(*dev).board_name=(*driver).driver_name;let mut r=((*driver).auto_attach.unwrap())(dev,context);if r>=0{r=comedi_device_postconfig(dev)}if r<0{mutex_unlock(&mut (*dev).mutex);comedi_release_hardware_device(hw)}else{mutex_unlock(&mut (*dev).mutex)}r}

pub unsafe fn comedi_driver_unregister(driver:*mut comedi_driver){if comedi_drivers==driver{comedi_drivers=(*driver).next}else{let mut p=comedi_drivers;while !p.is_null()&&!(*p).next.is_null(){if (*p).next==driver{(*p).next=(*driver).next;break}p=(*p).next}}for i in 0..COMEDI_NUM_BOARD_MINORS{let dev=comedi_dev_get_from_minor(i);if dev.is_null(){continue}mutex_lock(&mut (*dev).mutex);if (*dev).attached&&(*dev).driver==driver{comedi_device_detach(dev)}mutex_unlock(&mut (*dev).mutex);comedi_dev_put(dev)}}

// External kernel and COMEDI declarations are intentionally unresolved here;
// they are provided by the translated dependency units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
