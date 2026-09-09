/* SPDX-License-Identifier: GPL-2.0 */

// Translation of trace/events/asoc.h.
// The Linux tracepoint declaration machinery is supplied by the surrounding
// tracepoint implementation; the event declarations below retain its layout
// and formatting information.

pub const DAPM_DIRECT: &str = "(direct)";

// `SND_SOC_DAPM_DIR_OUT` is supplied by the sound subsystem.
#[inline]
pub fn dapm_arrow(dir: i32) -> &'static str {
    if dir == SND_SOC_DAPM_DIR_OUT { "->" } else { "<-" }
}

extern "C" {
    pub static SND_SOC_DAPM_DIR_OUT: i32;
}

#[repr(C)]
pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_card { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_widget { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_path { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_dapm_context { _private: [u8; 0] }

// TRACE_DEFINE_ENUM(SND_SOC_DAPM_DIR_OUT);

// DECLARE_EVENT_CLASS(snd_soc_dapm,
//     TP_PROTO(struct snd_soc_dapm_context *dapm, int val),
//     TP_ARGS(dapm, val),
//     TP_STRUCT__entry(
//         __string(card_name, snd_soc_dapm_to_card(dapm)->name)
//         __string(comp_name, snd_soc_dapm_to_component(dapm) ?
//                  snd_soc_dapm_to_component(dapm)->name : "(none)")
//         __field(int, val)),
//     TP_fast_assign(__assign_str(card_name); __assign_str(comp_name);
//                    __entry->val = val;),
//     TP_printk("card=%s component=%s val=%d", __get_str(card_name),
//               __get_str(comp_name), (int)__entry->val));
// DEFINE_EVENT(snd_soc_dapm, snd_soc_bias_level_start,
//              TP_PROTO(struct snd_soc_dapm_context *dapm, int val),
//              TP_ARGS(dapm, val));
// DEFINE_EVENT(snd_soc_dapm, snd_soc_bias_level_done,
//              TP_PROTO(struct snd_soc_dapm_context *dapm, int val),
//              TP_ARGS(dapm, val));

// The remaining declarations are Linux trace events. Their exact entry
// fields, assignments, and printk formats are preserved here because the
// tracepoint macros are external dependencies:
//
// snd_soc_dapm_basic(card, event): name=card->name, event:int;
//   printk("card=%s event=%d", name, event)
// snd_soc_dapm_start and snd_soc_dapm_done use snd_soc_dapm_basic.
//
// snd_soc_dapm_widget(w, val): name=w->name, val:int;
//   printk("widget=%s val=%d", name, val)
// snd_soc_dapm_widget_power, snd_soc_dapm_widget_event_start, and
// snd_soc_dapm_widget_event_done use snd_soc_dapm_widget.
//
// snd_soc_dapm_walk_done(card): name=card->name, power_checks:int,
// path_checks:int, neighbour_checks:int; assignments read
// card->dapm_stats.{power_checks,path_checks,neighbour_checks};
// printk("%s: checks %d power, %d path, %d neighbour", ...)
//
// snd_soc_dapm_path(widget, dir, path): wname=widget->name,
// pname=path->name ? path->name : DAPM_DIRECT, pnname=path->node[dir]->name,
// path_node=(long)path->node[dir], path_connect=path->connect,
// path_dir=dir; printk("%c%s %s %s %s %s", active ? '*' : ' ', wname,
// DAPM_ARROW(path_dir), pname, DAPM_ARROW(path_dir), pnname), where active is
// (int)path_node && (int)path_connect.
//
// snd_soc_dapm_connected(paths, stream): paths:int, stream:int;
// printk("%s: found %d paths", snd_pcm_direction_name(stream), paths)
// snd_soc_jack_irq(name): name=const char *, printk("%s", name)
// snd_soc_jack_report(jack, mask, val): name=jack->jack->id, mask:int,
// val:int, printk("jack=%s %x/%x", name, val, mask)
// snd_soc_jack_notify(jack, val): name=jack->jack->id, val:int,
// printk("jack=%s %x", name, val)


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
