// SPDX-License-Identifier: GPL-2.0-only
/* Tom Kelly's Scalable TCP
 *
 * See http://www.deneholme.net/tom/scalable/
 *
 * John Heffner <jheffner@sc.edu>
 */

// Dependencies supplied by the Linux kernel module and TCP headers.

const TCP_SCALABLE_AI_CNT: u32 = 100;
const TCP_SCALABLE_MD_SCALE: u32 = 3;

unsafe fn tcp_scalable_cong_avoid(sk: *mut sock, ack: u32, mut acked: u32) {
    let tp: *mut tcp_sock = tcp_sk(sk);

    if !tcp_is_cwnd_limited(sk) {
        return;
    }

    if tcp_in_slow_start(tp) {
        acked = tcp_slow_start(tp, acked);
        if acked == 0 {
            return;
        }
    }
    tcp_cong_avoid_ai(tp, min(tcp_snd_cwnd(tp), TCP_SCALABLE_AI_CNT), acked);
}

unsafe fn tcp_scalable_ssthresh(sk: *mut sock) -> u32 {
    let tp: *const tcp_sock = tcp_sk(sk);

    max(
        tcp_snd_cwnd(tp) - (tcp_snd_cwnd(tp) >> TCP_SCALABLE_MD_SCALE),
        2,
    )
}

static mut tcp_scalable: tcp_congestion_ops = tcp_congestion_ops {
    ssthresh: Some(tcp_scalable_ssthresh),
    undo_cwnd: Some(tcp_reno_undo_cwnd),
    cong_avoid: Some(tcp_scalable_cong_avoid),

    owner: THIS_MODULE,
    name: *b"scalable\0",
};

unsafe fn tcp_scalable_register() -> i32 {
    tcp_register_congestion_control(&raw mut tcp_scalable)
}

unsafe fn tcp_scalable_unregister() {
    tcp_unregister_congestion_control(&raw mut tcp_scalable);
}

module_init!(tcp_scalable_register);
module_exit!(tcp_scalable_unregister);

module_author!("John Heffner");
module_license!("GPL");
module_description!("Scalable TCP");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
