/* Private authoritative C header/type proof; no production source changes. */
#include <linux/rbtree_augmented.h>
typedef void (*rotate_t)(struct rb_node *, struct rb_node *);
void probe_insert(struct rb_node *, struct rb_root *, rotate_t);
void probe_erase(struct rb_node *, struct rb_root *, rotate_t);
void probe_callback(struct rb_node *, struct rb_node *);
void probe_first(rotate_t);
void probe_insert(struct rb_node *n, struct rb_root *r, rotate_t cb)
{ __rb_insert_augmented(n, r, cb); }
void probe_erase(struct rb_node *n, struct rb_root *r, rotate_t cb)
{ __rb_erase_color(n, r, cb); }
void probe_callback(struct rb_node *n, struct rb_node *r) { asm volatile("" : : "r"(n), "r"(r)); }
void probe_first(rotate_t cb) { asm volatile("" : : "r"(cb)); }
const unsigned long c_layout[] = {
 sizeof(rotate_t), __alignof__(rotate_t), sizeof(struct rb_node),
 __alignof__(struct rb_node), sizeof(struct rb_root), __alignof__(struct rb_root)
};
