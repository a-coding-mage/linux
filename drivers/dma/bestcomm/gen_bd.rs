// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for MPC52xx processor BestComm General Buffer Descriptor
 *
 * Copyright (C) 2007 Sylvain Munaut <tnt@246tNt.com>
 * Copyright (C) 2006 AppSpec Computer Technologies Corp.
 *                    Jeff Gibbons <jeff.gibbons@appspec.com>
 */

/* External kernel, MPC52xx, and BestComm definitions are supplied by other
 * translation units. */

extern "C" {
    static mut bcom_gen_bd_rx_task: [u32; 0];
    static mut bcom_gen_bd_tx_task: [u32; 0];
}

#[repr(C)]
pub struct bcom_gen_bd_rx_var {
    pub enable: u32,
    pub fifo: u32,
    pub bd_base: u32,
    pub bd_last: u32,
    pub bd_start: u32,
    pub buffer_size: u32,
}

#[repr(C)]
pub struct bcom_gen_bd_rx_inc {
    pub pad0: u16,
    pub incr_bytes: i16,
    pub pad1: u16,
    pub incr_dst: i16,
}

#[repr(C)]
pub struct bcom_gen_bd_tx_var {
    pub fifo: u32,
    pub enable: u32,
    pub bd_base: u32,
    pub bd_last: u32,
    pub bd_start: u32,
    pub buffer_size: u32,
}

#[repr(C)]
pub struct bcom_gen_bd_tx_inc {
    pub pad0: u16,
    pub incr_bytes: i16,
    pub pad1: u16,
    pub incr_src: i16,
    pub pad2: u16,
    pub incr_src_ma: i16,
}

#[repr(C)]
pub struct bcom_gen_bd_priv {
    pub fifo: phys_addr_t,
    pub initiator: i32,
    pub ipr: i32,
    pub maxbufsize: i32,
}

pub unsafe fn bcom_gen_bd_rx_init(
    queue_len: i32,
    fifo: phys_addr_t,
    initiator: i32,
    ipr: i32,
    maxbufsize: i32,
) -> *mut bcom_task {
    let tsk = bcom_task_alloc(
        queue_len,
        core::mem::size_of::<bcom_gen_bd>(),
        core::mem::size_of::<bcom_gen_bd_priv>(),
    );
    if tsk.is_null() { return core::ptr::null_mut(); }

    (*tsk).flags = BCOM_FLAGS_NONE;
    let priv_ = (*tsk).priv_ as *mut bcom_gen_bd_priv;
    (*priv_).fifo = fifo;
    (*priv_).initiator = initiator;
    (*priv_).ipr = ipr;
    (*priv_).maxbufsize = maxbufsize;

    if bcom_gen_bd_rx_reset(tsk) != 0 {
        bcom_task_free(tsk);
        return core::ptr::null_mut();
    }
    tsk
}

pub unsafe fn bcom_gen_bd_rx_reset(tsk: *mut bcom_task) -> i32 {
    let priv_ = (*tsk).priv_ as *mut bcom_gen_bd_priv;
    bcom_disable_task((*tsk).tasknum);
    let var = bcom_task_var((*tsk).tasknum) as *mut bcom_gen_bd_rx_var;
    let inc = bcom_task_inc((*tsk).tasknum) as *mut bcom_gen_bd_rx_inc;
    if bcom_load_image((*tsk).tasknum, bcom_gen_bd_rx_task.as_ptr()) != 0 { return -1; }

    (*var).enable = bcom_eng.regs_base + core::mem::offset_of!(mpc52xx_sdma, tcr) as u32;
    (*var).fifo = (*priv_).fifo as u32;
    (*var).bd_base = (*tsk).bd_pa;
    (*var).bd_last = (*tsk).bd_pa + ((*tsk).num_bd - 1) * (*tsk).bd_size;
    (*var).bd_start = (*tsk).bd_pa;
    (*var).buffer_size = (*priv_).maxbufsize as u32;
    (*inc).incr_bytes = -(core::mem::size_of::<u32>() as i16);
    (*inc).incr_dst = core::mem::size_of::<u32>() as i16;
    (*tsk).index = 0;
    (*tsk).outdex = 0;
    memset_io((*tsk).bd, 0, (*tsk).num_bd * (*tsk).bd_size);
    bcom_set_task_pragma((*tsk).tasknum, BCOM_GEN_RX_BD_PRAGMA);
    bcom_set_task_auto_start((*tsk).tasknum, (*tsk).tasknum);
    out_8(&mut (*bcom_eng).regs.ipr[(*priv_).initiator as usize], (*priv_).ipr as u8);
    bcom_set_initiator((*tsk).tasknum, (*priv_).initiator);
    out_be32(&mut (*bcom_eng).regs.IntPend, 1u32 << (*tsk).tasknum);
    0
}

pub unsafe fn bcom_gen_bd_rx_release(tsk: *mut bcom_task) { bcom_task_free(tsk); }

pub unsafe fn bcom_gen_bd_tx_init(queue_len: i32, fifo: phys_addr_t, initiator: i32, ipr: i32) -> *mut bcom_task {
    let tsk = bcom_task_alloc(queue_len, core::mem::size_of::<bcom_gen_bd>(), core::mem::size_of::<bcom_gen_bd_priv>());
    if tsk.is_null() { return core::ptr::null_mut(); }
    (*tsk).flags = BCOM_FLAGS_NONE;
    let priv_ = (*tsk).priv_ as *mut bcom_gen_bd_priv;
    (*priv_).fifo = fifo; (*priv_).initiator = initiator; (*priv_).ipr = ipr;
    if bcom_gen_bd_tx_reset(tsk) != 0 { bcom_task_free(tsk); return core::ptr::null_mut(); }
    tsk
}

pub unsafe fn bcom_gen_bd_tx_reset(tsk: *mut bcom_task) -> i32 {
    let priv_ = (*tsk).priv_ as *mut bcom_gen_bd_priv;
    bcom_disable_task((*tsk).tasknum);
    let var = bcom_task_var((*tsk).tasknum) as *mut bcom_gen_bd_tx_var;
    let inc = bcom_task_inc((*tsk).tasknum) as *mut bcom_gen_bd_tx_inc;
    if bcom_load_image((*tsk).tasknum, bcom_gen_bd_tx_task.as_ptr()) != 0 { return -1; }
    (*var).enable = bcom_eng.regs_base + core::mem::offset_of!(mpc52xx_sdma, tcr) as u32;
    (*var).fifo = (*priv_).fifo as u32; (*var).bd_base = (*tsk).bd_pa;
    (*var).bd_last = (*tsk).bd_pa + ((*tsk).num_bd - 1) * (*tsk).bd_size;
    (*var).bd_start = (*tsk).bd_pa;
    (*inc).incr_bytes = -(core::mem::size_of::<u32>() as i16);
    (*inc).incr_src = core::mem::size_of::<u32>() as i16;
    (*inc).incr_src_ma = core::mem::size_of::<u8>() as i16;
    (*tsk).index = 0; (*tsk).outdex = 0;
    memset_io((*tsk).bd, 0, (*tsk).num_bd * (*tsk).bd_size);
    bcom_set_task_pragma((*tsk).tasknum, BCOM_GEN_TX_BD_PRAGMA);
    bcom_set_task_auto_start((*tsk).tasknum, (*tsk).tasknum);
    out_8(&mut (*bcom_eng).regs.ipr[(*priv_).initiator as usize], (*priv_).ipr as u8);
    bcom_set_initiator((*tsk).tasknum, (*priv_).initiator);
    out_be32(&mut (*bcom_eng).regs.IntPend, 1u32 << (*tsk).tasknum);
    0
}

pub unsafe fn bcom_gen_bd_tx_release(tsk: *mut bcom_task) { bcom_task_free(tsk); }

#[repr(C)]
struct bcom_psc_params { rx_initiator: i32, rx_ipr: i32, tx_initiator: i32, tx_ipr: i32 }

static mut bcom_psc_params_table: [bcom_psc_params; 6] = [
    bcom_psc_params { rx_initiator: BCOM_INITIATOR_PSC1_RX, rx_ipr: BCOM_IPR_PSC1_RX, tx_initiator: BCOM_INITIATOR_PSC1_TX, tx_ipr: BCOM_IPR_PSC1_TX },
    bcom_psc_params { rx_initiator: BCOM_INITIATOR_PSC2_RX, rx_ipr: BCOM_IPR_PSC2_RX, tx_initiator: BCOM_INITIATOR_PSC2_TX, tx_ipr: BCOM_IPR_PSC2_TX },
    bcom_psc_params { rx_initiator: BCOM_INITIATOR_PSC3_RX, rx_ipr: BCOM_IPR_PSC3_RX, tx_initiator: BCOM_INITIATOR_PSC3_TX, tx_ipr: BCOM_IPR_PSC3_TX },
    bcom_psc_params { rx_initiator: BCOM_INITIATOR_PSC4_RX, rx_ipr: BCOM_IPR_PSC4_RX, tx_initiator: BCOM_INITIATOR_PSC4_TX, tx_ipr: BCOM_IPR_PSC4_TX },
    bcom_psc_params { rx_initiator: BCOM_INITIATOR_PSC5_RX, rx_ipr: BCOM_IPR_PSC5_RX, tx_initiator: BCOM_INITIATOR_PSC5_TX, tx_ipr: BCOM_IPR_PSC5_TX },
    bcom_psc_params { rx_initiator: BCOM_INITIATOR_PSC6_RX, rx_ipr: BCOM_IPR_PSC6_RX, tx_initiator: BCOM_INITIATOR_PSC6_TX, tx_ipr: BCOM_IPR_PSC6_TX },
];

pub unsafe fn bcom_psc_gen_bd_rx_init(psc_num: u32, queue_len: i32, fifo: phys_addr_t, maxbufsize: i32) -> *mut bcom_task {
    if psc_num >= MPC52xx_PSC_MAXNUM { return core::ptr::null_mut(); }
    let p = &bcom_psc_params_table[psc_num as usize];
    bcom_gen_bd_rx_init(queue_len, fifo, p.rx_initiator, p.rx_ipr, maxbufsize)
}

pub unsafe fn bcom_psc_gen_bd_tx_init(psc_num: u32, queue_len: i32, fifo: phys_addr_t) -> *mut bcom_task {
    let p = &bcom_psc_params_table[psc_num as usize];
    bcom_gen_bd_tx_init(queue_len, fifo, p.tx_initiator, p.tx_ipr)
}

// MODULE_DESCRIPTION("BestComm General Buffer Descriptor tasks driver");
// MODULE_AUTHOR("Jeff Gibbons <jeff.gibbons@appspec.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
