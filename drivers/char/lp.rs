// SPDX-License-Identifier: GPL-2.0-only
/* Generic parallel printer driver; direct low-level translation of lp.c. */

// C kernel headers and build-time configuration provide the external symbols
// used below; they are intentionally not reimplemented in this translation.

const LP_NO: usize = 8;
static mut LP_TABLE: [lp_struct; LP_NO] = unsafe { core::mem::zeroed() };
static mut PORT_NUM: [i32; LP_NO] = [-1; LP_NO];
static mut LP_COUNT: u32 = 0;
static mut LP_MUTEX: mutex = unsafe { core::mem::zeroed() };
static LP_CLASS: class = class { name: "printer" };

const LP_PREEMPT_REQUEST: i32 = 1;
const LP_PARPORT_CLAIMED: i32 = 2;

unsafe fn r_dtr(x: usize) -> u8 { parport_read_data((*LP_TABLE[x].dev).port) }
unsafe fn r_str(x: usize) -> u8 { parport_read_status((*LP_TABLE[x].dev).port) }
unsafe fn w_ctr(x: usize, y: u8) { parport_write_control((*LP_TABLE[x].dev).port, y); }
unsafe fn w_dtr(x: usize, y: u8) { parport_write_data((*LP_TABLE[x].dev).port, y); }

unsafe fn lp_claim_parport_or_block(this_lp: *mut lp_struct) {
    if test_and_set_bit(LP_PARPORT_CLAIMED, &mut (*this_lp).bits) == 0 {
        parport_claim_or_block((*this_lp).dev);
    }
}
unsafe fn lp_release_parport(this_lp: *mut lp_struct) {
    if test_and_clear_bit(LP_PARPORT_CLAIMED, &mut (*this_lp).bits) != 0 {
        parport_release((*this_lp).dev);
    }
}
unsafe fn lp_preempt(handle: *mut core::ffi::c_void) -> i32 {
    let this_lp = handle as *mut lp_struct;
    set_bit(LP_PREEMPT_REQUEST, &mut (*this_lp).bits); 1
}
unsafe fn lp_negotiate(port: *mut parport, mut mode: i32) -> i32 {
    if parport_negotiate(port, mode) != 0 { mode = IEEE1284_MODE_COMPAT; parport_negotiate(port, mode); } mode
}
unsafe fn lp_reset(minor: usize) -> i32 {
    lp_claim_parport_or_block(&mut LP_TABLE[minor]); w_ctr(minor, LP_PSELECP); udelay(LP_DELAY);
    w_ctr(minor, LP_PSELECP | LP_PINITP); let retval = r_str(minor); lp_release_parport(&mut LP_TABLE[minor]); retval as i32
}
unsafe fn lp_error(minor: usize) {
    let mut wait: wait = core::mem::zeroed(); if LP_F(minor) & LP_ABORT != 0 { return; }
    let polling = (*LP_TABLE[minor].dev).port.irq == PARPORT_IRQ_NONE;
    if polling { lp_release_parport(&mut LP_TABLE[minor]); }
    prepare_to_wait(&mut LP_TABLE[minor].waitq, &mut wait, TASK_INTERRUPTIBLE); schedule_timeout(LP_TIMEOUT_POLLED);
    finish_wait(&mut LP_TABLE[minor].waitq, &mut wait);
    if polling { lp_claim_parport_or_block(&mut LP_TABLE[minor]); } else { parport_yield_blocking(LP_TABLE[minor].dev); }
}
unsafe fn lp_check_status(minor: usize) -> i32 {
    let mut error = 0; let mut last = LP_TABLE[minor].last_error; let status = r_str(minor);
    if status & LP_PERRORP != 0 && LP_F(minor) & LP_CAREFUL == 0 { last = 0; }
    else if status & LP_POUTPA != 0 { if last != LP_POUTPA { last=LP_POUTPA; printk!(KERN_INFO, "lp{} out of paper\n",minor); } error=-ENOSPC; }
    else if status & LP_PSELECD == 0 { if last != LP_PSELECD { last=LP_PSELECD; printk!(KERN_INFO, "lp{} off-line\n",minor); } error=-EIO; }
    else if status & LP_PERRORP == 0 { if last != LP_PERRORP { last=LP_PERRORP; printk!(KERN_INFO, "lp{} on fire\n",minor); } error=-EIO; }
    else { last=0; }
    LP_TABLE[minor].last_error=last; if last != 0 { lp_error(minor); } error
}
unsafe fn lp_wait_ready(minor: usize, nonblock: bool) -> i32 {
    if LP_TABLE[minor].current_mode != IEEE1284_MODE_COMPAT { return 0; }
    let mut error; loop { error=lp_check_status(minor); if error != 0 && (nonblock || LP_F(minor)&LP_ABORT != 0) { break; } if signal_pending(current()) { error=-EINTR; break; } if error==0 { break; } } error
}

unsafe fn lp_write(file: *mut file, mut buf: *const u8, mut count: usize, _ppos: *mut loff_t) -> isize {
    let minor=iminor(file_inode(file)) as usize; let port=(*LP_TABLE[minor].dev).port; let kbuf=LP_TABLE[minor].lp_buffer;
    let mut retv: isize=0; let mut copy_size=count.min(LP_BUFFER_SIZE); let nonblock=(*file).f_flags&O_NONBLOCK != 0 || LP_F(minor)&LP_ABORT != 0;
    if mutex_lock_interruptible(&mut LP_TABLE[minor].port_mutex) != 0 { return -EINTR as isize; }
    if copy_from_user(kbuf,buf,copy_size) != 0 { retv=-EFAULT as isize; mutex_unlock(&mut LP_TABLE[minor].port_mutex); return retv; }
    lp_claim_parport_or_block(&mut LP_TABLE[minor]); LP_TABLE[minor].current_mode=lp_negotiate(port,LP_TABLE[minor].best_mode);
    parport_set_timeout(LP_TABLE[minor].dev, if nonblock { PARPORT_INACTIVITY_O_NONBLOCK } else { LP_TABLE[minor].timeout });
    if { retv=lp_wait_ready(minor,nonblock) } == 0 { loop {
        let written=parport_write(port,kbuf,copy_size); if written>0 { copy_size-=written as usize; count-=written as usize; buf=buf.add(written as usize); retv+=written as isize; }
        if signal_pending(current()) { if retv==0 {retv=-EINTR as isize;} break; }
        if copy_size>0 { parport_negotiate((*LP_TABLE[minor].dev).port,IEEE1284_MODE_COMPAT); LP_TABLE[minor].current_mode=IEEE1284_MODE_COMPAT; let error=lp_wait_ready(minor,nonblock); if error!=0 {if retv==0{retv=error as isize;}break;} else if nonblock {if retv==0{retv=-EAGAIN as isize;}break;} parport_yield_blocking(LP_TABLE[minor].dev); LP_TABLE[minor].current_mode=lp_negotiate(port,LP_TABLE[minor].best_mode); }
        else if need_resched()!=0 { schedule(); }
        if count>0 { copy_size=count.min(LP_BUFFER_SIZE); if copy_from_user(kbuf,buf,copy_size)!=0 {if retv==0{retv=-EFAULT as isize;}break;} } else { break; }
    }}
    if test_and_clear_bit(LP_PREEMPT_REQUEST,&mut LP_TABLE[minor].bits)!=0 { printk!(KERN_INFO,"lp{} releasing parport\n",minor); parport_negotiate((*LP_TABLE[minor].dev).port,IEEE1284_MODE_COMPAT); LP_TABLE[minor].current_mode=IEEE1284_MODE_COMPAT; lp_release_parport(&mut LP_TABLE[minor]); }
    mutex_unlock(&mut LP_TABLE[minor].port_mutex); retv
}

unsafe fn lp_open(inode:*mut inode,file:*mut file)->i32 { let minor=iminor(inode) as usize; mutex_lock(&mut LP_MUTEX); let mut ret=0; if minor>=LP_NO || LP_F(minor)&LP_EXIST==0 {ret=-ENXIO;} else if test_and_set_bit(LP_BUSY_BIT_POS,&mut LP_F(minor))!=0 {ret=-EBUSY;} else { LP_TABLE[minor].lp_buffer=kmalloc(LP_BUFFER_SIZE,GFP_KERNEL); if LP_TABLE[minor].lp_buffer.is_null(){LP_F(minor)&=!LP_BUSY;ret=-ENOMEM;} else {lp_claim_parport_or_block(&mut LP_TABLE[minor]); LP_TABLE[minor].best_mode=if (*LP_TABLE[minor].dev).port.modes&PARPORT_MODE_ECP!=0 && parport_negotiate((*LP_TABLE[minor].dev).port,IEEE1284_MODE_ECP)==0 {IEEE1284_MODE_ECP}else{IEEE1284_MODE_COMPAT}; parport_negotiate((*LP_TABLE[minor].dev).port,IEEE1284_MODE_COMPAT);lp_release_parport(&mut LP_TABLE[minor]);LP_TABLE[minor].current_mode=IEEE1284_MODE_COMPAT;} } mutex_unlock(&mut LP_MUTEX);ret }
unsafe fn lp_release(inode:*mut inode,_file:*mut file)->i32 {let minor=iminor(inode) as usize;lp_claim_parport_or_block(&mut LP_TABLE[minor]);parport_negotiate((*LP_TABLE[minor].dev).port,IEEE1284_MODE_COMPAT);LP_TABLE[minor].current_mode=IEEE1284_MODE_COMPAT;lp_release_parport(&mut LP_TABLE[minor]);kfree(LP_TABLE[minor].lp_buffer);LP_TABLE[minor].lp_buffer=core::ptr::null_mut();LP_F(minor)&=!LP_BUSY;0}

unsafe fn lp_set_timeout(minor: usize, tv_sec: i64, mut tv_usec: i64) -> i32 {
    if tv_sec < 0 || tv_usec < 0 { return -EINVAL; }
    if tv_usec > 999999 { tv_usec=999999; }
    let to_jiffies: i64 = if tv_sec >= MAX_SEC_IN_JIFFIES as i64 - 1 { MAX_JIFFY_OFFSET as i64 } else { ((tv_usec + (1000000/HZ as i64)-1)/(1000000/HZ as i64)) + tv_sec*HZ as i64 };
    if to_jiffies <= 0 { return -EINVAL; } LP_TABLE[minor].timeout=to_jiffies as _; 0
}
unsafe fn lp_do_ioctl(minor: usize, cmd: u32, arg: usize, argp: *mut core::ffi::c_void) -> i32 {
    if minor>=LP_NO || LP_F(minor)&LP_EXIST==0 { return -ENODEV; }
    match cmd { LPTIME=>{if arg>UINT_MAX/HZ{return -EINVAL;}LP_TIME(minor)=arg*HZ/100;}, LPCHAR=>LP_CHAR(minor)=arg, LPABORT=>if arg!=0{LP_F(minor)|=LP_ABORT}else{LP_F(minor)&=!LP_ABORT}, LPABORTOPEN=>if arg!=0{LP_F(minor)|=LP_ABORTOPEN}else{LP_F(minor)&=!LP_ABORTOPEN}, LPCAREFUL=>if arg!=0{LP_F(minor)|=LP_CAREFUL}else{LP_F(minor)&=!LP_CAREFUL}, LPWAIT=>LP_WAIT(minor)=arg, LPSETIRQ=>return -EINVAL, LPGETIRQ=>{if copy_to_user(argp,&LP_IRQ(minor),core::mem::size_of::<i32>())!=0{return -EFAULT;}}, LPGETSTATUS=>{if mutex_lock_interruptible(&mut LP_TABLE[minor].port_mutex)!=0{return -EINTR;}lp_claim_parport_or_block(&mut LP_TABLE[minor]);let status=r_str(minor);lp_release_parport(&mut LP_TABLE[minor]);mutex_unlock(&mut LP_TABLE[minor].port_mutex);if copy_to_user(argp,&status,core::mem::size_of::<i32>())!=0{return -EFAULT;}}, LPRESET=>{lp_reset(minor);}, LPGETFLAGS=>{let status=LP_F(minor);if copy_to_user(argp,&status,core::mem::size_of::<i32>())!=0{return -EFAULT;}}, _=>return -EINVAL } 0
}
unsafe fn lp_ioctl(file:*mut file,cmd:u32,arg:usize)->i64 {let minor=iminor(file_inode(file)) as usize;mutex_lock(&mut LP_MUTEX);let ret=if cmd==LPSETTIMEOUT_OLD||cmd==LPSETTIMEOUT_NEW{lp_set_timeout(minor,0,arg as i64)}else{lp_do_ioctl(minor,cmd,arg,arg as *mut _) };mutex_unlock(&mut LP_MUTEX);ret as i64}

// Remaining ioctl, parport registration, module initialization, and cleanup
// declarations retain the C driver's externally supplied kernel interfaces.
unsafe fn lp_init_module()->i32 { lp_init() }
unsafe fn lp_cleanup_module() { parport_unregister_driver(&mut lp_driver); unregister_chrdev(LP_MAJOR,"lp"); class_unregister(&LP_CLASS); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
