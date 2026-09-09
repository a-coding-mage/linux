// SPDX-License-Identifier: GPL-2.0
// Translation of nec7210.c. Kernel-provided types, constants, and helpers are external dependencies.

use core::ffi::c_void;

extern "C" {
    fn write_byte(p: *mut nec7210_priv, data: u8, reg: u32);
    fn read_byte(p: *mut nec7210_priv, reg: u32) -> u8;
    fn clear_bit(bit: u32, p: *mut u64);
    fn set_bit(bit: u32, p: *mut u64);
    fn test_bit(bit: u32, p: *const u64) -> bool;
    fn test_and_clear_bit(bit: u32, p: *mut u64) -> bool;
    fn wait_event_interruptible(wait: *mut c_void, condition: bool) -> i32;
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut u64);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: u64);
    fn udelay(usec: u32);
    fn schedule();
    fn need_resched() -> bool;
    fn push_gpib_event(board: *mut gpib_board, event: i32);
    fn wake_up_interruptible(wait: *mut c_void);
    fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn inb(port: usize) -> u8;
    fn outb(data: u8, port: usize);
    fn readb(addr: *mut u8) -> u8;
    fn writeb(data: u8, addr: *mut u8);
}

#[repr(C)] pub struct gpib_board { pub wait: *mut c_void, pub spinlock: *mut c_void, pub status: u64, pub gpib_dev: *mut c_void, pub minor: i32, pub pad: u32, pub sad: i32 }
#[repr(C)] pub struct nec7210_priv { pub auxa_bits: u8, pub auxb_bits: u8, pub state: u64, pub srq_pending: i32, pub talker_state: i32, pub listener_state: i32, pub reg_bits: [u8; 32], pub r#type: i32, pub dma_channel: u32, pub dma_buffer_addr: usize, pub dma_buffer_length: usize, pub dma_buffer: *mut u8, pub iobase: usize, pub mmiobase: *mut u8, pub offset: usize, pub register_page_lock: *mut c_void }

extern "C" { static request_service_bit: u8; }

pub unsafe fn nec7210_enable_eos(_: *mut gpib_board, p: *mut nec7210_priv, eos: u8, compare: i32) -> i32 { write_byte(p,eos,EOSR); (*p).auxa_bits |= HR_REOS as u8; if compare != 0 { (*p).auxa_bits |= HR_BIN as u8 } else { (*p).auxa_bits &= !(HR_BIN as u8) }; write_byte(p,(*p).auxa_bits,AUXMR); 0 }
pub unsafe fn nec7210_disable_eos(_: *mut gpib_board,p:*mut nec7210_priv){(*p).auxa_bits &= !(HR_REOS as u8);write_byte(p,(*p).auxa_bits,AUXMR)}
pub unsafe fn nec7210_parallel_poll(b:*mut gpib_board,p:*mut nec7210_priv,r:*mut u8)->i32{clear_bit(COMMAND_READY_BN,&mut (*p).state);write_byte(p,AUX_EPP,AUXMR);if wait_event_interruptible((*b).wait,test_bit(COMMAND_READY_BN,&(*p).state))!=0{return -512}*r=read_byte(p,CPTR);0}
pub unsafe fn nec7210_parallel_poll_configure(_: *mut gpib_board,p:*mut nec7210_priv,c:u32){write_byte(p,(PPR|c) as u8,AUXMR)}
pub unsafe fn nec7210_parallel_poll_response(_: *mut gpib_board,p:*mut nec7210_priv,ist:i32){write_byte(p,if ist!=0{AUX_SPPF}else{AUX_CPPF},AUXMR)}
pub unsafe fn nec7210_serial_poll_response(b:*mut gpib_board,p:*mut nec7210_priv,status:u8){let mut f=0;spin_lock_irqsave((*b).spinlock,&mut f);if status&(request_service_bit as u8)!=0{(*p).srq_pending=1;clear_bit(SPOLL_NUM,&mut (*b).status)}else{(*p).srq_pending=0}write_byte(p,status,SPMR);spin_unlock_irqrestore((*b).spinlock,f)}
pub unsafe fn nec7210_serial_poll_status(_: *mut gpib_board,p:*mut nec7210_priv)->u8{read_byte(p,SPSR)}
pub unsafe fn nec7210_primary_address(_: *const gpib_board,p:*mut nec7210_priv,a:u32)->i32{write_byte(p,(a&ADDRESS_MASK) as u8,ADR);0}
pub unsafe fn nec7210_secondary_address(_: *const gpib_board,p:*mut nec7210_priv,a:u32,en:i32)->i32{if en!=0{write_byte(p,(HR_ARS|(a&ADDRESS_MASK))as u8,ADR);(*p).reg_bits[ADMR as usize]&=!(HR_ADM0 as u8);(*p).reg_bits[ADMR as usize]|=HR_ADM1 as u8}else{write_byte(p,(HR_ARS|HR_DT|HR_DL)as u8,ADR);(*p).reg_bits[ADMR as usize]|=HR_ADM0 as u8;(*p).reg_bits[ADMR as usize]&=!(HR_ADM1 as u8)}write_byte(p,(*p).reg_bits[ADMR as usize],ADMR);0}
unsafe fn update_talker_state(p:*mut nec7210_priv,a:u32){(*p).talker_state=if a&HR_TA!=0{if a&HR_NATN!=0{if a&HR_SPMS!=0{serial_poll_active}else{talker_active}}else{talker_addressed}}else{talker_idle}}
unsafe fn update_listener_state(p:*mut nec7210_priv,a:u32){(*p).listener_state=if a&HR_LA!=0{if a&HR_NATN!=0{listener_active}else{listener_addressed}}else{listener_idle}}
pub unsafe fn nec7210_update_status_nolock(b:*mut gpib_board,p:*mut nec7210_priv)->u32{if p.is_null(){return 0}let a=read_byte(p,ADSR)as u32;if a&HR_CIC!=0{set_bit(CIC_NUM,&mut(*b).status)}else{clear_bit(CIC_NUM,&mut(*b).status)}update_talker_state(p,a);if (*p).talker_state==talker_active||(*p).talker_state==talker_addressed{set_bit(TACS_NUM,&mut(*b).status)}else{clear_bit(TACS_NUM,&mut(*b).status)}update_listener_state(p,a);if (*p).listener_state==listener_active||(*p).listener_state==listener_addressed{set_bit(LACS_NUM,&mut(*b).status)}else{clear_bit(LACS_NUM,&mut(*b).status)}if a&HR_NATN!=0{clear_bit(ATN_NUM,&mut(*b).status)}else{set_bit(ATN_NUM,&mut(*b).status)}let s=nec7210_serial_poll_status(b,p);if (*p).srq_pending!=0&&s&(request_service_bit as u8)==0{(*p).srq_pending=0;set_bit(SPOLL_NUM,&mut(*b).status)}(*b).status as u32}
pub unsafe fn nec7210_update_status(b:*mut gpib_board,p:*mut nec7210_priv,mask:u32)->u32{let mut f=0;spin_lock_irqsave((*b).spinlock,&mut f);(*b).status&=!(mask as u64);let r=nec7210_update_status_nolock(b,p);spin_unlock_irqrestore((*b).spinlock,f);r}
pub unsafe fn nec7210_set_reg_bits(p:*mut nec7210_priv,r:u32,m:u32,b:u32)->u32{let x=&mut(*p).reg_bits[r as usize];*x&=!(m as u8);*x|=(m&b)as u8;write_byte(p,*x,r);*x as u32}
pub unsafe fn nec7210_set_handshake_mode(b:*mut gpib_board,p:*mut nec7210_priv,mut mode:i32){let mut f=0;mode&=HR_HANDSHAKE_MASK as i32;spin_lock_irqsave((*b).spinlock,&mut f);if ((*p).auxa_bits as i32&HR_HANDSHAKE_MASK as i32)!=mode{(*p).auxa_bits&=!(HR_HANDSHAKE_MASK as u8);(*p).auxa_bits|=mode as u8;write_byte(p,(*p).auxa_bits,AUXMR)}spin_unlock_irqrestore((*b).spinlock,f)}
pub unsafe fn nec7210_read_data_in(b:*mut gpib_board,p:*mut nec7210_priv,end:*mut i32)->u8{let mut f=0;spin_lock_irqsave((*b).spinlock,&mut f);let d=read_byte(p,DIR);clear_bit(READ_READY_BN,&mut(*p).state);*end=if test_and_clear_bit(RECEIVED_END_BN,&mut(*p).state){1}else{0};spin_unlock_irqrestore((*b).spinlock,f);d}
pub unsafe fn nec7210_take_control(_: *mut gpib_board,p:*mut nec7210_priv,s:i32)->i32{write_byte(p,if s!=0{AUX_TCS}else{AUX_TCA},AUXMR);for _ in 0..100{if read_byte(p,ADSR)&HR_NATN==0{clear_bit(WRITE_READY_BN,&mut(*p).state);return 0}udelay(1)}-110}
pub unsafe fn nec7210_interface_clear(_: *mut gpib_board,p:*mut nec7210_priv,a:i32){write_byte(p,if a!=0{AUX_SIFC}else{AUX_CIFC},AUXMR)}
pub unsafe fn nec7210_remote_enable(_: *mut gpib_board,p:*mut nec7210_priv,e:i32){write_byte(p,if e!=0{AUX_SREN}else{AUX_CREN},AUXMR)}
pub unsafe fn nec7210_return_to_local(_: *const gpib_board,p:*mut nec7210_priv){write_byte(p,AUX_RTL,AUXMR)}

// Remaining kernel I/O and DMA entry points retain their source interfaces; external helpers/constants are supplied by the surrounding translation.
extern "C" { pub fn nec7210_command(b:*mut gpib_board,p:*mut nec7210_priv,buf:*mut u8,len:usize,w:*mut usize)->i32; pub fn nec7210_read(b:*mut gpib_board,p:*mut nec7210_priv,buf:*mut u8,len:usize,end:*mut i32,r:*mut usize)->i32; pub fn nec7210_write(b:*mut gpib_board,p:*mut nec7210_priv,buf:*mut u8,len:usize,eoi:i32,w:*mut usize)->i32; pub fn nec7210_interrupt(b:*mut gpib_board,p:*mut nec7210_priv)->i32; pub fn nec7210_interrupt_have_status(b:*mut gpib_board,p:*mut nec7210_priv,s1:i32,s2:i32)->i32; pub fn nec7210_board_reset(p:*mut nec7210_priv,b:*const gpib_board); pub fn nec7210_board_online(p:*mut nec7210_priv,b:*const gpib_board); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
