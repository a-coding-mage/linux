// SPDX-License-Identifier: GPL-2.0-only
/*
 * A FSI master controller, using a simple GPIO bit-banging interface
 */

// C dependencies supplied by the surrounding kernel translation unit.

const FSI_GPIO_STD_DLY: u64 = 1;
const LAST_ADDR_INVALID: u32 = 0x1;

#[repr(C)]
struct fsi_master_gpio {
    master: fsi_master,
    dev: *mut device,
    cmd_lock: mutex,
    gpio_clk: *mut gpio_desc,
    gpio_data: *mut gpio_desc,
    gpio_trans: *mut gpio_desc,
    gpio_enable: *mut gpio_desc,
    gpio_mux: *mut gpio_desc,
    external_mode: bool,
    no_delays: bool,
    last_addr: u32,
    t_send_delay: u8,
    t_echo_delay: u8,
}

#[repr(C)]
struct fsi_gpio_msg {
    msg: u64,
    bits: u8,
}

unsafe fn clock_toggle(master: *mut fsi_master_gpio, count: i32) {
    for _ in 0..count {
        if !(*master).no_delays { ndelay(FSI_GPIO_STD_DLY); }
        gpiod_set_value((*master).gpio_clk, 0);
        if !(*master).no_delays { ndelay(FSI_GPIO_STD_DLY); }
        gpiod_set_value((*master).gpio_clk, 1);
    }
}

unsafe fn sda_clock_in(master: *mut fsi_master_gpio) -> i32 {
    if !(*master).no_delays { ndelay(FSI_GPIO_STD_DLY); }
    gpiod_set_value((*master).gpio_clk, 0);
    gpiod_get_value((*master).gpio_data);
    let input = gpiod_get_value((*master).gpio_data);
    if !(*master).no_delays { ndelay(FSI_GPIO_STD_DLY); }
    gpiod_set_value((*master).gpio_clk, 1);
    if input != 0 { 1 } else { 0 }
}

unsafe fn sda_out(master: *mut fsi_master_gpio, value: i32) { gpiod_set_value((*master).gpio_data, value); }

unsafe fn set_sda_input(master: *mut fsi_master_gpio) {
    gpiod_direction_input((*master).gpio_data);
    gpiod_set_value((*master).gpio_trans, 0);
}

unsafe fn set_sda_output(master: *mut fsi_master_gpio, value: i32) {
    gpiod_set_value((*master).gpio_trans, 1);
    gpiod_direction_output((*master).gpio_data, value);
}

unsafe fn clock_zeros(master: *mut fsi_master_gpio, count: i32) {
    trace_fsi_master_gpio_clock_zeros(master, count);
    set_sda_output(master, 1);
    clock_toggle(master, count);
}

unsafe fn echo_delay(master: *mut fsi_master_gpio) { clock_zeros(master, (*master).t_echo_delay as i32); }

unsafe fn serial_in(master: *mut fsi_master_gpio, msg: *mut fsi_gpio_msg, num_bits: u8) {
    set_sda_input(master);
    for _ in 0..num_bits {
        let in_bit = sda_clock_in(master);
        (*msg).msg <<= 1;
        (*msg).msg |= ((!in_bit as u64) & 1);
    }
    (*msg).bits = (*msg).bits.wrapping_add(num_bits);
    trace_fsi_master_gpio_in(master, num_bits, (*msg).msg);
}

unsafe fn serial_out(master: *mut fsi_master_gpio, cmd: *const fsi_gpio_msg) {
    let mut msg = !(*cmd).msg;
    let mut sda_mask = 1u64 << ((*cmd).bits.wrapping_sub(1));
    let mut last_bit = !0u64;
    trace_fsi_master_gpio_out(master, (*cmd).bits, (*cmd).msg);
    if (*cmd).bits == 0 { dev_warn((*master).dev, "trying to output 0 bits\n"); return; }
    set_sda_output(master, 0);
    sda_out(master, 0);
    clock_toggle(master, 1);
    for _ in 0..(*cmd).bits {
        let next_bit = ((msg & sda_mask) >> ((*cmd).bits - 1)) as i32;
        if (last_bit ^ next_bit as u64) != 0 { sda_out(master, next_bit); last_bit = next_bit as u64; }
        clock_toggle(master, 1);
        msg <<= 1;
        sda_mask = sda_mask;
    }
}

unsafe fn msg_push_bits(msg: *mut fsi_gpio_msg, data: u64, bits: i32) {
    (*msg).msg <<= bits;
    (*msg).msg |= data & ((1u64 << bits) - 1);
    (*msg).bits = (*msg).bits.wrapping_add(bits as u8);
}

unsafe fn msg_push_crc(msg: *mut fsi_gpio_msg) {
    let top = ((*msg).bits & 0x3) as i32;
    let mut crc = crc4(0, (1u64 << top) | ((*msg).msg >> ((*msg).bits as i32 - top)), (top + 1) as u8);
    crc = crc4(crc, (*msg).msg, ((*msg).bits as i32 - top) as u8);
    msg_push_bits(msg, crc as u64, 4);
}

unsafe fn check_same_address(master: *mut fsi_master_gpio, id: i32, addr: u32) -> bool {
    (*master).last_addr == (((id & 0x3) as u32) << 21 | (addr & !0x3))
}

unsafe fn check_relative_address(master: *mut fsi_master_gpio, id: i32, addr: u32, rel: *mut u32) -> bool {
    let last = (*master).last_addr;
    if last == LAST_ADDR_INVALID || (((last >> 21) & 0x3) as i32) != id { return false; }
    let last = last & ((1 << 21) - 1);
    let relative = addr.wrapping_sub(last) as i32;
    if relative > 255 || relative < -256 { return false; }
    *rel = relative as u32;
    true
}

unsafe fn last_address_update(master: *mut fsi_master_gpio, id: i32, valid: bool, addr: u32) {
    (*master).last_addr = if valid { ((id & 0x3) as u32) << 21 | (addr & !0x3) } else { LAST_ADDR_INVALID };
}

unsafe fn build_ar_command(master: *mut fsi_master_gpio, cmd: *mut fsi_gpio_msg, id: u8, mut addr: u32, size: usize, data: *const u8) {
    let write = !data.is_null();
    let mut rel_addr = 0u32;
    (*cmd).bits = 0; (*cmd).msg = 0;
    addr &= (1 << 21) - 1;
    let (addr_bits, opcode_bits, opcode) = if check_same_address(master, id as i32, addr) { trace_fsi_master_gpio_cmd_same_addr(master); (2, 2, FSI_CMD_SAME_AR) } else if check_relative_address(master, id as i32, addr, &mut rel_addr) { addr = rel_addr; trace_fsi_master_gpio_cmd_rel_addr(master, rel_addr); (9, 3, FSI_CMD_REL_AR) } else { trace_fsi_master_gpio_cmd_abs_addr(master, addr); (21, 3, FSI_CMD_ABS_AR) };
    let ds = if size > 1 { 1 } else { 0 };
    addr &= !(size as u32 - 1);
    if size == 4 { addr |= 1; }
    msg_push_bits(cmd, id as u64, 2); msg_push_bits(cmd, opcode as u64, opcode_bits); msg_push_bits(cmd, if write { 0 } else { 1 }, 1); msg_push_bits(cmd, addr as u64, addr_bits); msg_push_bits(cmd, ds, 1);
    for i in 0..size { if write { msg_push_bits(cmd, *data.add(i) as u64, 8); } }
    msg_push_crc(cmd);
}

unsafe fn build_dpoll_command(cmd: *mut fsi_gpio_msg, slave_id: u8) { (*cmd).bits=0; (*cmd).msg=0; msg_push_bits(cmd, slave_id as u64,2); msg_push_bits(cmd, FSI_CMD_DPOLL as u64,3); msg_push_crc(cmd); }
unsafe fn build_epoll_command(cmd: *mut fsi_gpio_msg, slave_id: u8) { (*cmd).bits=0; (*cmd).msg=0; msg_push_bits(cmd, slave_id as u64,2); msg_push_bits(cmd, FSI_CMD_EPOLL as u64,3); msg_push_crc(cmd); }
unsafe fn build_term_command(cmd: *mut fsi_gpio_msg, slave_id: u8) { (*cmd).bits=0; (*cmd).msg=0; msg_push_bits(cmd, slave_id as u64,2); msg_push_bits(cmd, FSI_CMD_TERM as u64,6); msg_push_crc(cmd); }

unsafe fn read_one_response(master:*mut fsi_master_gpio,data_size:u8,msgp:*mut fsi_gpio_msg,tagp:*mut u8)->i32 {
    let mut msg=fsi_gpio_msg{msg:0,bits:0}; let mut i=0;
    local_irq_save();
    while i<FSI_MASTER_MTOE_COUNT { msg=fsi_gpio_msg{msg:0,bits:0}; serial_in(master,&mut msg,1); if msg.msg!=0 {break;} i+=1; }
    if i==FSI_MASTER_MTOE_COUNT { dev_dbg((*master).dev,"Master time out waiting for response\n"); local_irq_restore(); return -ETIMEDOUT; }
    msg=fsi_gpio_msg{msg:0,bits:0}; serial_in(master,&mut msg,4); let tag=(msg.msg&3) as u8;
    if tag==FSI_RESP_ACK && data_size!=0 { serial_in(master,&mut msg,data_size*8); }
    serial_in(master,&mut msg,FSI_CRC_SIZE); local_irq_restore();
    let mut crc=crc4(0,1,1); crc=crc4(crc,msg.msg,msg.bits);
    if crc!=0 { if (!msg.msg & ((1u64<<msg.bits)-1))==0 {return -ENODEV;} dev_dbg((*master).dev,"ERR response CRC msg: 0x%016llx (%d bits)\n",msg.msg,msg.bits); return -EAGAIN; }
    if !msgp.is_null(){*msgp=msg;} if !tagp.is_null(){*tagp=tag;} 0
}

unsafe fn issue_term(master:*mut fsi_master_gpio,slave:u8)->i32 { let mut cmd=fsi_gpio_msg{msg:0,bits:0}; build_term_command(&mut cmd,slave); local_irq_save(); serial_out(master,&cmd); echo_delay(master); local_irq_restore(); let mut tag=0; let rc=read_one_response(master,0,std::ptr::null_mut(),&mut tag); if rc<0||tag!=FSI_RESP_ACK {dev_err((*master).dev,"TERM failed\n");return -EIO;} 0 }

unsafe fn poll_for_response(master:*mut fsi_master_gpio,slave:u8,size:u8,data:*mut u8)->i32 {
    let mut busy=0; let mut retries=0; loop { let mut response=fsi_gpio_msg{msg:0,bits:0}; let mut tag=0; let rc=read_one_response(master,size,&mut response,&mut tag);
        if rc==-EAGAIN { retries+=1; if retries>FSI_CRC_ERR_RETRIES {return -EIO;} let mut cmd=fsi_gpio_msg{msg:0,bits:0}; build_epoll_command(&mut cmd,slave); local_irq_save(); clock_zeros(master,FSI_MASTER_EPOLL_CLOCKS); serial_out(master,&cmd); echo_delay(master); local_irq_restore(); continue; }
        if rc!=0{return rc;} match tag { FSI_RESP_ACK=>{if size!=0&&!data.is_null(){let mut val=(response.msg>>4)&((1u64<<(size*8))-1);for i in 0..size as usize{*data.add(size as usize-i-1)=val as u8;val>>=8;}}}, FSI_RESP_BUSY=>{if busy<FSI_MASTER_MAX_BUSY{busy+=1;let mut cmd=fsi_gpio_msg{msg:0,bits:0};build_dpoll_command(&mut cmd,slave);local_irq_save();clock_zeros(master,FSI_MASTER_DPOLL_CLOCKS);serial_out(master,&cmd);echo_delay(master);local_irq_restore();continue;}local_irq_save();clock_zeros(master,FSI_MASTER_DPOLL_CLOCKS);local_irq_restore();issue_term(master,slave);return -EIO}, FSI_RESP_ERRA=>return -EIO, FSI_RESP_ERRC=>return -EAGAIN, _=>{} } local_irq_save();clock_zeros(master,(*master).t_send_delay as i32);local_irq_restore();return 0;
    }
}

unsafe fn send_request(master:*mut fsi_master_gpio,cmd:*mut fsi_gpio_msg)->i32 { if (*master).external_mode{return -EBUSY;}local_irq_save();serial_out(master,cmd);echo_delay(master);local_irq_restore();0 }
unsafe fn fsi_master_gpio_xfer(master:*mut fsi_master_gpio,slave:u8,cmd:*mut fsi_gpio_msg,len:u8,resp:*mut u8)->i32 { let mut rc=-EAGAIN;let mut retries=0;while retries<FSI_CRC_ERR_RETRIES{retries+=1;rc=send_request(master,cmd);if rc!=0{break;}rc=poll_for_response(master,slave,len,resp);if rc!=-EAGAIN{break;}msleep(1);}rc }

unsafe fn fsi_master_gpio_read(_master:*mut fsi_master,link:i32,id:u8,addr:u32,val:*mut u8,size:usize)->i32 { let m=to_fsi_master_gpio(_master);if link!=0{return -ENODEV;}mutex_lock(&mut (*m).cmd_lock);let mut c=fsi_gpio_msg{msg:0,bits:0};build_ar_command(m,&mut c,id,addr,size,std::ptr::null());let rc=fsi_master_gpio_xfer(m,id,&mut c,size as u8,val);last_address_update(m,id as i32,rc==0,addr);mutex_unlock(&mut (*m).cmd_lock);rc }
unsafe fn fsi_master_gpio_write(_master:*mut fsi_master,link:i32,id:u8,addr:u32,val:*const u8,size:usize)->i32 { let m=to_fsi_master_gpio(_master);if link!=0{return -ENODEV;}mutex_lock(&mut (*m).cmd_lock);let mut c=fsi_gpio_msg{msg:0,bits:0};build_ar_command(m,&mut c,id,addr,size,val);let rc=fsi_master_gpio_xfer(m,id,&mut c,0,std::ptr::null_mut());last_address_update(m,id as i32,rc==0,addr);mutex_unlock(&mut (*m).cmd_lock);rc }
unsafe fn fsi_master_gpio_term(_master:*mut fsi_master,link:i32,id:u8)->i32 {let m=to_fsi_master_gpio(_master);if link!=0{return -ENODEV;}mutex_lock(&mut (*m).cmd_lock);let mut c=fsi_gpio_msg{msg:0,bits:0};build_term_command(&mut c,id);let rc=fsi_master_gpio_xfer(m,id,&mut c,0,std::ptr::null_mut());last_address_update(m,id as i32,false,0);mutex_unlock(&mut (*m).cmd_lock);rc}

extern "C" {
    fn local_irq_save(); fn local_irq_restore(); fn dev_warn(dev:*mut device,fmt:&str,...); fn dev_dbg(dev:*mut device,fmt:&str,...); fn dev_err(dev:*mut device,fmt:&str,...);
    fn mutex_lock(m:*mut mutex); fn mutex_unlock(m:*mut mutex); fn to_fsi_master_gpio(m:*mut fsi_master)->*mut fsi_master_gpio;
}

unsafe fn fsi_master_gpio_break(_master:*mut fsi_master,link:i32)->i32 {let m=to_fsi_master_gpio(_master);if link!=0{return -ENODEV;}mutex_lock(&mut (*m).cmd_lock);if (*m).external_mode{mutex_unlock(&mut (*m).cmd_lock);return -EBUSY;}local_irq_save();set_sda_output(m,1);sda_out(m,1);clock_toggle(m,FSI_PRE_BREAK_CLOCKS);sda_out(m,0);clock_toggle(m,FSI_BREAK_CLOCKS);echo_delay(m);sda_out(m,1);clock_toggle(m,FSI_POST_BREAK_CLOCKS);local_irq_restore();last_address_update(m,0,false,0);mutex_unlock(&mut (*m).cmd_lock);udelay(200);0}
unsafe fn fsi_master_gpio_init(m:*mut fsi_master_gpio){gpiod_direction_output((*m).gpio_mux,1);gpiod_direction_output((*m).gpio_trans,1);gpiod_direction_output((*m).gpio_enable,1);gpiod_direction_output((*m).gpio_clk,1);gpiod_direction_output((*m).gpio_data,1);local_irq_save();clock_zeros(m,FSI_INIT_CLOCKS);local_irq_restore();}
unsafe fn fsi_master_gpio_init_external(m:*mut fsi_master_gpio){gpiod_direction_output((*m).gpio_mux,0);gpiod_direction_output((*m).gpio_trans,0);gpiod_direction_output((*m).gpio_enable,1);gpiod_direction_input((*m).gpio_clk);gpiod_direction_input((*m).gpio_data);}
unsafe fn fsi_master_gpio_link_enable(_m:*mut fsi_master,link:i32,enable:bool)->i32{let m=to_fsi_master_gpio(_m);if link!=0{return -ENODEV;}mutex_lock(&mut (*m).cmd_lock);let rc=if !(*m).external_mode{gpiod_set_value((*m).gpio_enable,if enable{1}else{0});0}else{-EBUSY};mutex_unlock(&mut (*m).cmd_lock);rc}
unsafe fn fsi_master_gpio_link_config(_m:*mut fsi_master,link:i32,send:u8,echo:u8)->i32{let m=to_fsi_master_gpio(_m);if link!=0{return -ENODEV;}mutex_lock(&mut (*m).cmd_lock);(*m).t_send_delay=send;(*m).t_echo_delay=echo;mutex_unlock(&mut (*m).cmd_lock);0}

// Driver registration and probe/remove declarations retain the source driver's external kernel integration points.
extern "C" {
    fn fsi_master_gpio_probe(pdev:*mut platform_device)->i32;
    fn fsi_master_gpio_remove(pdev:*mut platform_device);
}
extern "C" {
    fn ndelay(ns: u64); fn udelay(us: u64); fn msleep(ms: u64);
    fn crc4(crc: u32, data: u64, bits: u8) -> u32;
    fn gpiod_set_value(desc: *mut gpio_desc, value: i32); fn gpiod_get_value(desc: *mut gpio_desc) -> i32;
    fn gpiod_direction_input(desc: *mut gpio_desc) -> i32; fn gpiod_direction_output(desc: *mut gpio_desc, value: i32) -> i32;
    fn trace_fsi_master_gpio_clock_zeros(m: *mut fsi_master_gpio, c: i32); fn trace_fsi_master_gpio_in(m:*mut fsi_master_gpio,b:u8,v:u64); fn trace_fsi_master_gpio_out(m:*mut fsi_master_gpio,b:u8,v:u64);
    fn trace_fsi_master_gpio_cmd_same_addr(m:*mut fsi_master_gpio); fn trace_fsi_master_gpio_cmd_rel_addr(m:*mut fsi_master_gpio,a:u32); fn trace_fsi_master_gpio_cmd_abs_addr(m:*mut fsi_master_gpio,a:u32);
}

// Types and constants from the included kernel headers are intentionally referenced as external dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
