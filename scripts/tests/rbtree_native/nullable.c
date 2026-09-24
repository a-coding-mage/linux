#include <linux/rbtree_augmented.h>
extern void oracle___rb_insert_augmented(struct rb_node *, struct rb_root *, void (*)(struct rb_node *, struct rb_node *));
extern void oracle___rb_erase_color(struct rb_node *, struct rb_root *, void (*)(struct rb_node *, struct rb_node *));
#if ORACLE
#define INSERT oracle___rb_insert_augmented
#define ERASE oracle___rb_erase_color
#else
#define INSERT __rb_insert_augmented
#define ERASE __rb_erase_color
#endif
int test_main(void);
int test_main(void)
{
    struct rb_node n = {0};
    struct rb_root root = RB_ROOT;
#if ERASING
    struct rb_node g = {0}, s = {0}, q = {0};
    root.rb_node = &g;
    g.rb_left = &n; g.rb_right = &q; g.__rb_parent_color = RB_BLACK;
    n.rb_right = &s; n.__rb_parent_color = (unsigned long)&g;
    s.__rb_parent_color = (unsigned long)&n | RB_BLACK;
    q.__rb_parent_color = (unsigned long)&g | RB_BLACK;
    void (*volatile callback)(struct rb_node *, struct rb_root *, void (*)(struct rb_node *, struct rb_node *)) = ERASE;
    callback(&n, &root, NULL);
    return rb_is_red(&n);
#else
    void (*volatile callback)(struct rb_node *, struct rb_root *, void (*)(struct rb_node *, struct rb_node *)) = INSERT;
    rb_link_node(&n, NULL, &root.rb_node);
    callback(&n, &root, NULL);
    return rb_is_red(&n);
#endif
}
asm(".globl _start\n_start:\n call test_main\n mov %eax,%edi\n mov $60,%eax\n syscall\n ud2\n");
