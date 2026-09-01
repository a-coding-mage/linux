/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  The driver for the Cirrus Logic's Sound Fusion CS46XX based soundcards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *
 * NOTE: comments are copy/paste from cwcemb80.lst
 * provided by Tom Woller at Cirrus (my only
 * documentation about the SP OS running inside
 * the DSP)
 */

/* Depends on cs46xx_dsp_scb_types.h in the original C header. */

/*********************************************************************************************
Example hierarchy of stream control blocks in the SP

hfgTree
Ptr____Call (c)
       \
 -------+------         -------------      -------------      -------------      -----
| SBlaster IF  |______\| Foreground  |___\| Middlegr'nd |___\| Background  |___\| Nul |
|              |Goto  /| tree header |g  /| tree header |g  /| tree header |g  /| SCB |r
 -------------- (g)     -------------      -------------      -------------      -----
       |c                     |c                 |c                 |c
       |                      |                  |                  |
      \/                  -------------      -------------      -------------
                       | Foreground  |_\  | Middlegr'nd |_\  | Background  |_\
                       |     tree    |g/  |    tree     |g/  |     tree    |g/
                        -------------      -------------      -------------
                              |c                 |c                 |c
                              |                  |                  |
                             \/                 \/                 \/

*********************************************************************************************/

pub const HFG_FIRST_EXECUTE_MODE: u32 = 0x0001;
pub const HFG_FIRST_EXECUTE_MODE_BIT: u32 = 0;
pub const HFG_CONTEXT_SWITCH_MODE: u32 = 0x0002;
pub const HFG_CONTEXT_SWITCH_MODE_BIT: u32 = 1;

/* THESE NEED TO BE COMPUTED PROPERLY */
pub const MAX_FG_STACK_SIZE: u32 = 32;
pub const MAX_MG_STACK_SIZE: u32 = 16;
pub const MAX_BG_STACK_SIZE: u32 = 9;
pub const MAX_HFG_STACK_SIZE: u32 = 4;

/*
 * Enable task tree thread to go to sleep.
 * This should only ever be used on the Background thread.
 */
pub const SLEEP_ACTIVE_INCREMENT: u32 = 0;
/* Task tree thread normal operation */
pub const STANDARD_ACTIVE_INCREMENT: u32 = 1;
/*
 * Cause execution to suspend in the task tree thread.
 * This should only ever be used on the Background thread.
 */
pub const SUSPEND_ACTIVE_INCREMENT: u32 = 2;

/*
 * Host-controlled flag that determines whether we go to sleep
 * at the end of BG.
 */
pub const HOSTFLAGS_DISABLE_BG_SLEEP: u32 = 0;

/* Minimal context save area for Hyper Forground */
#[repr(C)]
pub struct dsp_hf_save_area {
    pub r10_save: u32,
    pub r54_save: u32,
    pub r98_save: u32,

    pub status_save: u16,
    pub ind_save: u16,

    pub rci1_save: u16,
    pub rci0_save: u16,

    pub r32_save: u32,
    pub r76_save: u32,
    pub rsd2_save: u32,

    /*
     * See TaskTreeParameterBlock for remainder of registers.
     * saved as part of HFG context.
     */
    pub rsi2_save: u16,
    pub rsa2Save: u16,
}

/* Task link data structure */
#[repr(C)]
pub struct dsp_tree_link {
    /* Pointer to sibling task control block */
    pub next_scb: u16,
    /* Pointer to child task control block */
    pub sub_ptr: u16,

    /* Pointer to code entry point */
    pub entry_point: u16,
    /* Pointer to local data */
    pub this_spb: u16,
}

#[repr(C)]
pub struct dsp_task_tree_data {
    /* Initial tock count; controls task tree execution rate */
    pub tock_count_limit: u16,
    /* Tock down counter */
    pub tock_count: u16,

    /*
     * Add to ActiveCount when TockCountLimit reached:
     * Subtract on task tree termination.
     */
    pub active_tncrement: u16,
    /* Number of pending activations for task tree */
    pub active_count: u16,

    /* BitNumber to enable modification of correct bit in ActiveTaskFlags */
    pub active_bit: u16,
    /* Pointer to OS location for indicating current activity on task level */
    pub active_task_flags_ptr: u16,

    /*
     * Data structure for controlling movement of memory blocks:-
     * currently unused.
     */
    pub mem_upd_ptr: u16,
    /* Data structure for controlling synchronous link update */
    pub link_upd_ptr: u16,

    /* Save area for remainder of full context. */
    pub save_area: u16,
    /* Address of start of local stack for data storage */
    pub data_stack_base_ptr: u16,
}

#[repr(C)]
pub struct dsp_interval_timer_data {
    /* These data items have the same relative locations to those */
    pub interval_timer_period: u16,
    pub itd_unused: u16,

    /* used for this data in the SPOS control block for SPOS 1.0 */
    pub num_FG_ticks_this_interval: u16,
    pub num_intervals: u16,
}

/*
 * This structure contains extra storage for the task tree
 * Currently, this additional data is related only to a full context save.
 */
#[repr(C)]
pub struct dsp_task_tree_context_block {
    /*
     * Up to 10 values are saved onto the stack.  8 for the task tree, 1 for
     * The access to the context switch (call or interrupt), and 1 spare that
     * users should never use.  This last may be required by the system.
     */
    pub stack1: u16,
    pub stack0: u16,
    pub stack3: u16,
    pub stack2: u16,
    pub stack5: u16,
    pub stack4: u16,
    pub stack7: u16,
    pub stack6: u16,
    pub stack9: u16,
    pub stack8: u16,

    pub saverfe: u32,

    /*
     * Value may be overwritten by stack save algorithm.
     * Retain the size of the stack data saved here if used.
     */
    pub reserved1: u16,
    pub stack_size: u16,
    pub saverba: u32,          /* (HFG) */
    pub saverdc: u32,
    pub savers_config_23: u32, /* (HFG) */
    pub savers_DMA23: u32,     /* (HFG) */
    pub saversa0: u32,
    pub saversi0: u32,
    pub saversa1: u32,
    pub saversi1: u32,
    pub saversa3: u32,
    pub saversd0: u32,
    pub saversd1: u32,
    pub saversd3: u32,
    pub savers_config01: u32,
    pub savers_DMA01: u32,
    pub saveacc0hl: u32,
    pub saveacc1hl: u32,
    pub saveacc0xacc1x: u32,
    pub saveacc2hl: u32,
    pub saveacc3hl: u32,
    pub saveacc2xacc3x: u32,
    pub saveaux0hl: u32,
    pub saveaux1hl: u32,
    pub saveaux0xaux1x: u32,
    pub saveaux2hl: u32,
    pub saveaux3hl: u32,
    pub saveaux2xaux3x: u32,
    pub savershouthl: u32,
    pub savershoutxmacmode: u32,
}

#[repr(C)]
pub struct dsp_task_tree_control_block {
    pub context: dsp_hf_save_area,
    pub links: dsp_tree_link,
    pub data: dsp_task_tree_data,
    pub context_blk: dsp_task_tree_context_block,
    pub int_timer: dsp_interval_timer_data,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
