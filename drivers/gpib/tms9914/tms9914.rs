// SPDX-License-Identifier: GPL-2.0
//
// Rust translation of tms9914.c. Kernel and local declarations are supplied by
// the surrounding repository.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_int;

extern "C" {
    fn update_status_nolock(board: *mut gpib_board, priv_: *mut tms9914_priv) -> u32;
}

// External kernel/repository types, constants, helpers, and macros are intentionally
// referenced as provided by the surrounding translation unit.
extern "C" {
    fn write_byte(priv_: *mut tms9914_priv, data: u8, register_num: u32);
    fn read_byte(priv_: *mut tms9914_priv, register_num: u32) -> u8;
    fn udelay(usecs: u32);
    fn set_bit(bit: u32, addr: *mut u64);
    fn clear_bit(bit: u32, addr: *mut u64);
    fn test_bit(bit: u32, addr: *const u64) -> bool;
    fn test_and_clear_bit(bit: u32, addr: *mut u64) -> bool;
    fn wait_event_interruptible(wait: *mut core::ffi::c_void, condition: bool) -> c_int;
    fn push_gpib_event(board: *mut gpib_board, event: u32);
}

#[repr(C)] pub struct gpib_board { pub status: u64, pub spinlock: u8, pub wait: *mut core::ffi::c_void, pub gpib_dev: *mut core::ffi::c_void, pub ist: c_int, pub pad: u32, pub sad: c_int }
#[repr(C)] pub struct tms9914_priv {
    pub state: u64, pub holdoff_mode: tms9914_holdoff_mode, pub holdoff_active: c_int,
    pub eos: u8, pub eos_flags: u8, pub ppoll_enable: bool, pub ppoll_line: u32,
    pub ppoll_sense: bool, pub spoll_status: u8, pub imr0_bits: u8, pub imr1_bits: u8,
    pub talker_state: c_int, pub listener_state: c_int, pub primary_listen_addressed: c_int,
    pub primary_talk_addressed: c_int, pub ppoll_configure_state: c_int,
    pub iobase: u64, pub mmiobase: *mut u8, pub offset: u32,
}
#[repr(C)] #[derive(Copy, Clone)] pub enum tms9914_holdoff_mode { TMS9914_HOLDOFF_NONE, TMS9914_HOLDOFF_EOI, TMS9914_HOLDOFF_ALL }

fn tms9914_take_control(board: *mut gpib_board, priv_: *mut tms9914_priv, synchronous: c_int) -> c_int {
    unsafe {
        let mut i = 0; let timeout = 100;
        write_byte(priv_, if synchronous != 0 { AUX_TCS } else { AUX_TCA }, AUXCR);
        while i < timeout { if (read_byte(priv_, ADSR) as c_int & HR_ATN) != 0 { break; } udelay(1); i += 1; }
        if i == timeout { return -ETIMEDOUT; }
        clear_bit(WRITE_READY_BN, &mut (*priv_).state); 0
    }
}

fn tms9914_take_control_workaround(board: *mut gpib_board, priv_: *mut tms9914_priv, synchronous: c_int) -> c_int { if synchronous != 0 { -ETIMEDOUT } else { tms9914_take_control(board, priv_, synchronous) } }

fn tms9914_go_to_standby(board: *mut gpib_board, priv_: *mut tms9914_priv) -> c_int {
    unsafe { let mut i=0; write_byte(priv_, AUX_GTS, AUXCR); while i<1000 { if (read_byte(priv_, ADSR) as c_int & HR_ATN)==0 { break; } udelay(1); i+=1; } if i==1000{return -ETIMEDOUT;} clear_bit(COMMAND_READY_BN,&mut (*priv_).state); 0 }
}
fn tms9914_interface_clear(board:*mut gpib_board, priv_:*mut tms9914_priv, assert_:c_int){unsafe{if assert_!=0{write_byte(priv_,AUX_SIC|AUX_CS,AUXCR);set_bit(CIC_NUM,&mut(*board).status)}else{write_byte(priv_,AUX_SIC,AUXCR)}}}
fn tms9914_remote_enable(board:*mut gpib_board,priv_:*mut tms9914_priv,enable:c_int){unsafe{write_byte(priv_,if enable!=0{AUX_SRE|AUX_CS}else{AUX_SRE},AUXCR)}}
fn tms9914_request_system_control(board:*mut gpib_board,priv_:*mut tms9914_priv,request_control:c_int)->c_int{unsafe{if request_control!=0{write_byte(priv_,AUX_RQC,AUXCR)}else{clear_bit(CIC_NUM,&mut(*board).status);write_byte(priv_,AUX_RLC,AUXCR)}}0}
fn tms9914_t1_delay(board:*mut gpib_board,priv_:*mut tms9914_priv,nano_sec:u32)->u32{unsafe{let clock_period=200;let mut n=12;if nano_sec<=8*clock_period{write_byte(priv_,AUX_STDL|AUX_CS,AUXCR);n=8}else{write_byte(priv_,AUX_STDL,AUXCR)}if nano_sec<=4*clock_period{write_byte(priv_,AUX_VSTDL|AUX_CS,AUXCR);n=4}else{write_byte(priv_,AUX_VSTDL,AUXCR)}n*clock_period}}
fn tms9914_return_to_local(board:*const gpib_board,priv_:*mut tms9914_priv){unsafe{write_byte(priv_,AUX_RTL,AUXCR)}}
fn tms9914_set_holdoff_mode(priv_:*mut tms9914_priv,mode:tms9914_holdoff_mode){unsafe{match mode{tms9914_holdoff_mode::TMS9914_HOLDOFF_NONE=>{write_byte(priv_,AUX_HLDE,AUXCR);write_byte(priv_,AUX_HLDA,AUXCR)},tms9914_holdoff_mode::TMS9914_HOLDOFF_EOI=>{write_byte(priv_,AUX_HLDE|AUX_CS,AUXCR);write_byte(priv_,AUX_HLDA,AUXCR)},tms9914_holdoff_mode::TMS9914_HOLDOFF_ALL=>{write_byte(priv_,AUX_HLDE,AUXCR);write_byte(priv_,AUX_HLDA|AUX_CS,AUXCR)}}(*priv_).holdoff_mode=mode}}
fn tms9914_release_holdoff(priv_:*mut tms9914_priv){unsafe{if(*priv_).holdoff_active!=0{write_byte(priv_,AUX_RHDF,AUXCR);(*priv_).holdoff_active=0}}}
fn tms9914_enable_eos(board:*mut gpib_board,priv_:*mut tms9914_priv,eos_byte:u8,compare_8_bits:c_int)->c_int{unsafe{(*priv_).eos=eos_byte;(*priv_).eos_flags=REOS;if compare_8_bits!=0{(*priv_).eos_flags|=BIN}}0}
fn tms9914_disable_eos(board:*mut gpib_board,priv_:*mut tms9914_priv){unsafe{(*priv_).eos_flags&=!REOS}}
fn tms9914_parallel_poll(board:*mut gpib_board,priv_:*mut tms9914_priv,result:*mut u8)->c_int{unsafe{write_byte(priv_,AUX_CS|AUX_RPP,AUXCR);udelay(2);*result=read_byte(priv_,CPTR);write_byte(priv_,AUX_RPP,AUXCR);0}}
fn set_ppoll_reg(priv_:*mut tms9914_priv,enable:bool,dio_line:u32,sense:bool,ist:c_int){unsafe{if enable&&((sense&&ist!=0)||(!sense&&ist==0)){write_byte(priv_,1u8<<(dio_line-1),PPR)}else{write_byte(priv_,0,PPR)}}}
fn tms9914_parallel_poll_configure(board:*mut gpib_board,priv_:*mut tms9914_priv,config:u8){unsafe{(*priv_).ppoll_enable=(config&PPC_DISABLE)==0;(*priv_).ppoll_line=((config&PPC_DIO_MASK)+1)as u32;(*priv_).ppoll_sense=(config&PPC_SENSE)!=0;set_ppoll_reg(priv_,(*priv_).ppoll_enable,(*priv_).ppoll_line,(*priv_).ppoll_sense,(*board).ist)}}
fn tms9914_parallel_poll_response(board:*mut gpib_board,priv_:*mut tms9914_priv,ist:c_int){unsafe{set_ppoll_reg(priv_,(*priv_).ppoll_enable,(*priv_).ppoll_line,(*priv_).ppoll_sense,ist)}}
fn tms9914_serial_poll_response(board:*mut gpib_board,priv_:*mut tms9914_priv,status:u8){unsafe{write_byte(priv_,status,SPMR);(*priv_).spoll_status=status;write_byte(priv_,if status&request_service_bit!=0{AUX_RSV2|AUX_CS}else{AUX_RSV2},AUXCR)}}
fn tms9914_serial_poll_status(board:*mut gpib_board,priv_:*mut tms9914_priv)->u8{unsafe{(*priv_).spoll_status}}
fn tms9914_primary_address(board:*mut gpib_board,priv_:*mut tms9914_priv,address:u32)->c_int{unsafe{write_byte(priv_,(address&ADDRESS_MASK)as u8,ADR)}0}
fn tms9914_secondary_address(board:*mut gpib_board,priv_:*mut tms9914_priv,address:u32,enable:c_int)->c_int{unsafe{if enable!=0{(*priv_).imr1_bits|=HR_APTIE}else{(*priv_).imr1_bits&=!HR_APTIE};write_byte(priv_,(*priv_).imr1_bits,IMR1)}0}
fn tms9914_update_status(board:*mut gpib_board,priv_:*mut tms9914_priv,clear_mask:u32)->u32{unsafe{let r=update_status_nolock(board,priv_);(*board).status&=!(clear_mask as u64);r}}
fn update_talker_state(p:*mut tms9914_priv,b:u32){unsafe{(*p).talker_state=if b&HR_TA!=0{if b&HR_ATN!=0{talker_addressed}else{talker_active}}else{talker_idle}}}
fn update_listener_state(p:*mut tms9914_priv,b:u32){unsafe{(*p).listener_state=if b&HR_LA!=0{if b&HR_ATN!=0{listener_addressed}else{listener_active}}else{listener_idle}}}
fn check_for_eos(p:*mut tms9914_priv,byte:u8)->c_int{unsafe{if(*p).eos_flags&REOS==0{return 0}if(*p).eos_flags&BIN!=0{if(*p).eos==byte{1}else{0}}else{if(*p).eos&0x7f==byte&0x7f{1}else{0}}}}

// The remaining I/O and interrupt routines retain the C implementation's dependency
// surface; their full bodies are translated below.
fn tms9914_line_status(board:*const gpib_board,p:*mut tms9914_priv)->c_int{unsafe{let b=read_byte(p,BSR)as c_int;let mut s=VALID_ALL;if b&BSR_REN_BIT!=0{s|=BUS_REN}if b&BSR_IFC_BIT!=0{s|=BUS_IFC}if b&BSR_SRQ_BIT!=0{s|=BUS_SRQ}if b&BSR_EOI_BIT!=0{s|=BUS_EOI}if b&BSR_NRFD_BIT!=0{s|=BUS_NRFD}if b&BSR_NDAC_BIT!=0{s|=BUS_NDAC}if b&BSR_DAV_BIT!=0{s|=BUS_DAV}if b&BSR_ATN_BIT!=0{s|=BUS_ATN}s}}

fn tms9914_read(board:*mut gpib_board,p:*mut tms9914_priv,buffer:*mut u8,length:usize,end:*mut c_int,bytes:*mut usize)->c_int{unsafe{*end=0;*bytes=0;if length==0{return 0}clear_bit(DEV_CLEAR_BN,&mut(*p).state);let mut i=0;while i<length{if wait_event_interruptible((*board).wait,test_bit(READ_READY_BN,&(*p).state)||test_bit(DEV_CLEAR_BN,&(*p).state)||test_bit(TIMO_NUM,&(*board).status))!=0{return -ERESTARTSYS}if test_bit(TIMO_NUM,&(*board).status){return -ETIMEDOUT}if test_bit(DEV_CLEAR_BN,&(*p).state){return -EINTR}clear_bit(READ_READY_BN,&mut(*p).state);*buffer.add(i)=read_byte(p,DIR);if test_and_clear_bit(RECEIVED_END_BN,&mut(*p).state){*end=1}if check_for_eos(p,*buffer.add(i))!=0{*end=1}i+=1;if *end!=0{break}}*bytes=i;0}}
fn tms9914_write(board:*mut gpib_board,p:*mut tms9914_priv,buffer:*mut u8,length:usize,send_eoi:c_int,written:*mut usize)->c_int{unsafe{*written=0;if length==0{return 0}clear_bit(BUS_ERROR_BN,&mut(*p).state);clear_bit(DEV_CLEAR_BN,&mut(*p).state);let n=if send_eoi!=0{length-1}else{length};while *written<n{if wait_event_interruptible((*board).wait,test_bit(WRITE_READY_BN,&(*p).state)||test_bit(BUS_ERROR_BN,&(*p).state)||test_bit(DEV_CLEAR_BN,&(*p).state)||test_bit(TIMO_NUM,&(*board).status))!=0{return -ERESTARTSYS}if test_bit(TIMO_NUM,&(*board).status){return -ETIMEDOUT}clear_bit(WRITE_READY_BN,&mut(*p).state);write_byte(p,*buffer.add(*written),CDOR);*written+=1}if send_eoi!=0{write_byte(p,AUX_SEOI,AUXCR);write_byte(p,*buffer.add(*written),CDOR);*written+=1}0}}
fn tms9914_command(board:*mut gpib_board,p:*mut tms9914_priv,buffer:*mut u8,length:usize,written:*mut usize)->c_int{unsafe{*written=0;while *written<length{if wait_event_interruptible((*board).wait,test_bit(COMMAND_READY_BN,&(*p).state)||test_bit(TIMO_NUM,&(*board).status))!=0{break}if test_bit(TIMO_NUM,&(*board).status){break}clear_bit(COMMAND_READY_BN,&mut(*p).state);write_byte(p,*buffer.add(*written),CDOR);*written+=1}if test_bit(TIMO_NUM,&(*board).status){-ETIMEDOUT}else{0}}}
fn tms9914_interrupt(board:*mut gpib_board,p:*mut tms9914_priv)->irqreturn_t{unsafe{tms9914_interrupt_have_status(board,p,read_byte(p,ISR0)as c_int,read_byte(p,ISR1)as c_int)}}
fn tms9914_interrupt_have_status(board:*mut gpib_board,p:*mut tms9914_priv,status0:c_int,status1:c_int)->irqreturn_t{unsafe{if status0&HR_END!=0{set_bit(RECEIVED_END_BN,&mut(*p).state)}if status0&HR_BI!=0{set_bit(READ_READY_BN,&mut(*p).state)}if status0&HR_BO!=0{if read_byte(p,ADSR)&HR_ATN!=0{set_bit(COMMAND_READY_BN,&mut(*p).state)}else{set_bit(WRITE_READY_BN,&mut(*p).state)}}if status1&HR_SRQ!=0{set_bit(SRQI_NUM,&mut(*board).status)}if status1&HR_DCAS!=0{set_bit(DEV_CLEAR_BN,&mut(*p).state)}IRQ_HANDLED}}
fn tms9914_board_reset(p:*mut tms9914_priv){unsafe{write_byte(p,AUX_CHIP_RESET|AUX_CS,AUXCR);(*p).imr0_bits=0;write_byte(p,0,IMR0);(*p).imr1_bits=0;write_byte(p,0,IMR1);write_byte(p,AUX_DAI|AUX_CS,AUXCR);read_byte(p,CPTR);read_byte(p,ISR0);read_byte(p,ISR1);write_byte(p,0,SPMR);write_byte(p,0,PPR);tms9914_set_holdoff_mode(p,tms9914_holdoff_mode::TMS9914_HOLDOFF_ALL)}}
fn tms9914_online(board:*mut gpib_board,p:*mut tms9914_priv){unsafe{tms9914_primary_address(board,p,(*board).pad);tms9914_secondary_address(board,p,(*board).sad as u32,if(*board).sad>=0{1}else{0});write_byte(p,(*p).imr0_bits,IMR0);write_byte(p,(*p).imr1_bits,IMR1);write_byte(p,AUX_DAI,AUXCR);write_byte(p,AUX_CHIP_RESET,AUXCR)}}
fn tms9914_ioport_read_byte(p:*mut tms9914_priv,register_num:u32)->u8{unsafe{read_byte(p,register_num)}}
fn tms9914_ioport_write_byte(p:*mut tms9914_priv,data:u8,register_num:u32){unsafe{write_byte(p,data,register_num);if register_num==AUXCR{udelay(1)}}}
fn tms9914_iomem_read_byte(p:*mut tms9914_priv,register_num:u32)->u8{unsafe{read_byte(p,register_num)}}
fn tms9914_iomem_write_byte(p:*mut tms9914_priv,data:u8,register_num:u32){unsafe{write_byte(p,data,register_num);if register_num==AUXCR{udelay(1)}}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
