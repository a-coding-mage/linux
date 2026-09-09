// SPDX-License-Identifier: GPL-2.0+
/* Direct low-level translation of comedi/drivers/ni_usb6501.c. */

const NI6501_TIMEOUT: i32 = 1000;

static READ_PORT_REQUEST: [u8; 16] = [0,1,0,0x10,0,0x0c,1,0x0e,2,0x10,0,0,0,3,0,0];
static WRITE_PORT_REQUEST: [u8; 20] = [0,1,0,0x14,0,0x10,1,0x0f,2,0x10,0,0,0,3,0,0,3,0,0,0];
static SET_PORT_DIR_REQUEST: [u8; 24] = [0,1,0,0x18,0,0x14,1,0x12,2,0x10,0,0,0,5,0,0,0,0,5,0,0,0,0,0];
static START_COUNTER_REQUEST: [u8; 12] = [0,1,0,0x0c,0,8,1,9,2,0x20,0,0];
static STOP_COUNTER_REQUEST: [u8; 12] = [0,1,0,0x0c,0,8,1,0x0c,2,0x20,0,0];
static READ_COUNTER_REQUEST: [u8; 12] = [0,1,0,0x0c,0,8,1,0x0e,2,0x20,0,0];
static WRITE_COUNTER_REQUEST: [u8; 16] = [0,1,0,0x10,0,0x0c,1,0x0f,2,0x20,0,0,0,0,0,0];
static GENERIC_RESPONSE: [u8; 12] = [0,1,0,0x0c,0,8,1,0,0,0,0,2];
static READ_PORT_RESPONSE: [u8; 16] = [0,1,0,0x10,0,0x0c,1,0,0,0,0,2,0,3,0,0];
static READ_COUNTER_RESPONSE: [u8; 16] = [0,1,0,0x10,0,0x0c,1,0,0,0,0,2,0,0,0,0];
const TX_MAX_SIZE: usize = SET_PORT_DIR_REQUEST.len();
const RX_MAX_SIZE: usize = READ_PORT_RESPONSE.len();

#[repr(i32)]
enum Commands { ReadPort, WritePort, SetPortDir, StartCounter, StopCounter, ReadCounter, WriteCounter }

#[repr(C)]
struct Ni6501Private {
    ep_rx: *mut usb_endpoint_descriptor,
    ep_tx: *mut usb_endpoint_descriptor,
    mut_: mutex,
    usb_rx_buf: *mut u8,
    usb_tx_buf: *mut u8,
}

unsafe fn ni6501_port_command(dev: *mut comedi_device, command: Commands, val: u32, bitmap: *mut u8) -> i32 {
    let usb = comedi_to_usb_dev(dev); let p = (*dev).private as *mut Ni6501Private;
    if !matches!(command, Commands::SetPortDir) && bitmap.is_null() { return -EINVAL; }
    mutex_lock(&mut (*p).mut_);
    let (request, response): (&[u8], &[u8]);
    let tx = (*p).usb_tx_buf;
    match command {
        Commands::ReadPort => { request=&READ_PORT_REQUEST; response=&READ_PORT_RESPONSE; std::ptr::copy_nonoverlapping(request.as_ptr(),tx,request.len()); *tx.add(14)=val as u8; }
        Commands::WritePort => { request=&WRITE_PORT_REQUEST; response=&GENERIC_RESPONSE; std::ptr::copy_nonoverlapping(request.as_ptr(),tx,request.len()); *tx.add(14)=val as u8; *tx.add(17)=*bitmap; }
        Commands::SetPortDir => { request=&SET_PORT_DIR_REQUEST; response=&GENERIC_RESPONSE; std::ptr::copy_nonoverlapping(request.as_ptr(),tx,request.len()); *tx.add(14)=val as u8; *tx.add(15)=(val>>8) as u8; *tx.add(16)=(val>>16) as u8; }
        _ => { mutex_unlock(&mut (*p).mut_); return -EINVAL; }
    }
    let mut ret=usb_bulk_msg(usb,usb_sndbulkpipe(usb,(*(*p).ep_tx).bEndpointAddress),(*p).usb_tx_buf,request.len() as i32,std::ptr::null_mut(),NI6501_TIMEOUT);
    if ret==0 { ret=usb_bulk_msg(usb,usb_rcvbulkpipe(usb,(*(*p).ep_rx).bEndpointAddress),(*p).usb_rx_buf,response.len() as i32,std::ptr::null_mut(),NI6501_TIMEOUT); }
    if ret==0 { if matches!(command,Commands::ReadPort) { *bitmap=*(*p).usb_rx_buf.add(14); *(*p).usb_rx_buf.add(14)=0; if libc::memcmp((*p).usb_rx_buf as *const _,READ_PORT_RESPONSE.as_ptr() as *const _,READ_PORT_RESPONSE.len())!=0 { ret=-EINVAL; } } else if libc::memcmp((*p).usb_rx_buf as *const _,GENERIC_RESPONSE.as_ptr() as *const _,GENERIC_RESPONSE.len())!=0 { ret=-EINVAL; } }
    mutex_unlock(&mut (*p).mut_); ret
}

unsafe fn ni6501_counter_command(dev:*mut comedi_device, command:Commands, val:*mut u32)->i32 {
    let usb=comedi_to_usb_dev(dev); let p=(*dev).private as *mut Ni6501Private;
    if matches!(command,Commands::ReadCounter|Commands::WriteCounter)&&val.is_null(){return -EINVAL;} mutex_lock(&mut (*p).mut_);
    let (request,response)=match command { Commands::StartCounter=>(&START_COUNTER_REQUEST[..],&GENERIC_RESPONSE[..]),Commands::StopCounter=>(&STOP_COUNTER_REQUEST[..],&GENERIC_RESPONSE[..]),Commands::ReadCounter=>(&READ_COUNTER_REQUEST[..],&READ_COUNTER_RESPONSE[..]),Commands::WriteCounter=>(&WRITE_COUNTER_REQUEST[..],&GENERIC_RESPONSE[..]),_=>{mutex_unlock(&mut (*p).mut_);return -EINVAL}};
    std::ptr::copy_nonoverlapping(request.as_ptr(),(*p).usb_tx_buf,request.len()); if matches!(command,Commands::WriteCounter){(*p).usb_tx_buf.add(12).cast::<u32>().write((*val).to_be());}
    let mut ret=usb_bulk_msg(usb,usb_sndbulkpipe(usb,(*(*p).ep_tx).bEndpointAddress),(*p).usb_tx_buf,request.len() as i32,std::ptr::null_mut(),NI6501_TIMEOUT); if ret==0{ret=usb_bulk_msg(usb,usb_rcvbulkpipe(usb,(*(*p).ep_rx).bEndpointAddress),(*p).usb_rx_buf,response.len() as i32,std::ptr::null_mut(),NI6501_TIMEOUT);}
    if ret==0&&matches!(command,Commands::ReadCounter){*val=(*p).usb_rx_buf.add(12).cast::<u32>().read().from_be(); for i in 12..16{*(*p).usb_rx_buf.add(i)=0;}}
    if ret==0&&libc::memcmp((*p).usb_rx_buf as *const _,response.as_ptr() as *const _,response.len())!=0{ret=-EINVAL;} mutex_unlock(&mut (*p).mut_);ret
}

// External kernel/comedi declarations and remaining driver callbacks are preserved as declarations.
extern "C" { fn ni6501_dio_insn_config(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32; fn ni6501_dio_insn_bits(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32; fn ni6501_cnt_insn_config(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32; fn ni6501_cnt_insn_read(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32; fn ni6501_cnt_insn_write(dev:*mut comedi_device,s:*mut comedi_subdevice,insn:*mut comedi_insn,data:*mut u32)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
