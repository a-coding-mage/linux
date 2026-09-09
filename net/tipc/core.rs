/*
 * net/tipc/core.c: TIPC module code
 *
 * Copyright (c) 2003-2006, 2013, Ericsson AB
 * Copyright (c) 2005-2006, 2010-2013, Wind River Systems
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the names of the copyright holders nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 *
 * Alternatively, this software may be distributed under the terms of the
 * GNU General Public License ("GPL") version 2 as published by the Free
 * Software Foundation.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

// Dependencies supplied by the surrounding TIPC implementation and kernel bindings.

/* configurable TIPC parameters */
#[no_mangle]
pub static mut tipc_net_id: u32 = 0;
#[no_mangle]
pub static mut sysctl_tipc_rmem: [i32; 3] = [0; 3]; /* min/default/max */

unsafe fn tipc_init_net(net: *mut net) -> i32 {
    let tn: *mut tipc_net = net_generic(net, tipc_net_id);
    let mut err: i32;

    (*tn).net_id = 4711;
    (*tn).node_addr = 0;
    (*tn).trial_addr = 0;
    (*tn).addr_trial_end = 0;
    (*tn).capabilities = TIPC_NODE_CAPABILITIES;
    INIT_WORK(&mut (*tn).work, tipc_net_finalize_work);
    core::ptr::write_bytes((*tn).node_id.as_mut_ptr(), 0, (*tn).node_id.len());
    core::ptr::write_bytes((*tn).node_id_string.as_mut_ptr(), 0, (*tn).node_id_string.len());
    (*tn).mon_threshold = TIPC_DEF_MON_THRESHOLD;
    get_random_bytes(&mut (*tn).random as *mut _, core::mem::size_of::<i32>());
    INIT_LIST_HEAD(&mut (*tn).node_list);
    spin_lock_init(&mut (*tn).node_list_lock);

    // CONFIG_TIPC_CRYPTO controls this build-time conditional section.
    #[cfg(feature = "CONFIG_TIPC_CRYPTO")]
    {
        err = tipc_crypto_start(&mut (*tn).crypto_tx, net, core::ptr::null_mut());
        if err != 0 {
            return err;
        }
    }
    err = tipc_sk_rht_init(net);
    if err != 0 {
        #[cfg(feature = "CONFIG_TIPC_CRYPTO")]
        tipc_crypto_stop(&mut (*tn).crypto_tx);
        return err;
    }

    err = tipc_nametbl_init(net);
    if err != 0 {
        tipc_sk_rht_destroy(net);
        #[cfg(feature = "CONFIG_TIPC_CRYPTO")]
        tipc_crypto_stop(&mut (*tn).crypto_tx);
        return err;
    }

    err = tipc_bcast_init(net);
    if err != 0 {
        tipc_nametbl_stop(net);
        tipc_sk_rht_destroy(net);
        #[cfg(feature = "CONFIG_TIPC_CRYPTO")]
        tipc_crypto_stop(&mut (*tn).crypto_tx);
        return err;
    }

    err = tipc_attach_loopback(net);
    if err != 0 {
        tipc_nametbl_stop(net);
        tipc_sk_rht_destroy(net);
        #[cfg(feature = "CONFIG_TIPC_CRYPTO")]
        tipc_crypto_stop(&mut (*tn).crypto_tx);
        return err;
    }

    0
}

unsafe fn tipc_exit_net(net: *mut net) {
    let tn: *mut tipc_net = tipc_net(net);

    tipc_detach_loopback(net);
    tipc_net_stop(net);
    /* Make sure the tipc_net_finalize_work() finished */
    cancel_work_sync(&mut (*tn).work);
    tipc_bcast_stop(net);
    tipc_nametbl_stop(net);
    tipc_sk_rht_destroy(net);
    #[cfg(feature = "CONFIG_TIPC_CRYPTO")]
    tipc_crypto_stop(&mut (*tipc_net(net)).crypto_tx);
    wait_var_event(&mut (*tn).wq_count, atomic_read(&(*tn).wq_count) == 0);
}

unsafe fn tipc_pernet_pre_exit(net: *mut net) {
    tipc_node_pre_cleanup_net(net);
}

static mut tipc_pernet_pre_exit_ops: pernet_operations = pernet_operations {
    pre_exit: Some(tipc_pernet_pre_exit),
};

static mut tipc_net_ops: pernet_operations = pernet_operations {
    init: Some(tipc_init_net),
    exit: Some(tipc_exit_net),
    id: unsafe { &mut tipc_net_id },
    size: core::mem::size_of::<tipc_net>(),
};

static mut tipc_topsrv_net_ops: pernet_operations = pernet_operations {
    init: Some(tipc_topsrv_init_net),
    exit: Some(tipc_topsrv_exit_net),
};

unsafe fn tipc_init() -> i32 {
    let mut err: i32;

    pr_info!("Activated (version {})\n", TIPC_MOD_VER);

    sysctl_tipc_rmem[0] = RCVBUF_MIN;
    sysctl_tipc_rmem[1] = RCVBUF_DEF;
    sysctl_tipc_rmem[2] = RCVBUF_MAX;

    err = tipc_register_sysctl();
    if err != 0 { return err; }
    err = register_pernet_device(&mut tipc_net_ops);
    if err != 0 { tipc_unregister_sysctl(); return err; }
    err = tipc_socket_init();
    if err != 0 { unregister_pernet_device(&mut tipc_net_ops); tipc_unregister_sysctl(); return err; }
    err = register_pernet_device(&mut tipc_topsrv_net_ops);
    if err != 0 { tipc_socket_stop(); unregister_pernet_device(&mut tipc_net_ops); tipc_unregister_sysctl(); return err; }
    err = register_pernet_subsys(&mut tipc_pernet_pre_exit_ops);
    if err != 0 { unregister_pernet_device(&mut tipc_topsrv_net_ops); tipc_socket_stop(); unregister_pernet_device(&mut tipc_net_ops); tipc_unregister_sysctl(); return err; }
    err = tipc_bearer_setup();
    if err != 0 { unregister_pernet_subsys(&mut tipc_pernet_pre_exit_ops); unregister_pernet_device(&mut tipc_topsrv_net_ops); tipc_socket_stop(); unregister_pernet_device(&mut tipc_net_ops); tipc_unregister_sysctl(); return err; }
    err = tipc_netlink_start();
    if err != 0 { tipc_bearer_cleanup(); unregister_pernet_subsys(&mut tipc_pernet_pre_exit_ops); unregister_pernet_device(&mut tipc_topsrv_net_ops); tipc_socket_stop(); unregister_pernet_device(&mut tipc_net_ops); tipc_unregister_sysctl(); return err; }
    err = tipc_netlink_compat_start();
    if err != 0 { tipc_netlink_stop(); tipc_bearer_cleanup(); unregister_pernet_subsys(&mut tipc_pernet_pre_exit_ops); unregister_pernet_device(&mut tipc_topsrv_net_ops); tipc_socket_stop(); unregister_pernet_device(&mut tipc_net_ops); tipc_unregister_sysctl(); return err; }

    pr_info!("Started in single node mode\n");
    0
}

unsafe fn tipc_exit() {
    tipc_netlink_compat_stop();
    tipc_netlink_stop();
    tipc_bearer_cleanup();
    unregister_pernet_subsys(&mut tipc_pernet_pre_exit_ops);
    unregister_pernet_device(&mut tipc_topsrv_net_ops);
    tipc_socket_stop();
    unregister_pernet_device(&mut tipc_net_ops);
    tipc_unregister_sysctl();

    /* TODO: Wait for all timers that called call_rcu() to finish before
     * calling rcu_barrier().
     */
    rcu_barrier();

    pr_info!("Deactivated\n");
}

// module_init(tipc_init);
// module_exit(tipc_exit);
// MODULE_DESCRIPTION("TIPC: Transparent Inter Process Communication");
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_VERSION(TIPC_MOD_VER);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
