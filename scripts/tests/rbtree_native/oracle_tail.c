/* Fixture appended to the complete original lib/rbtree.c. */
struct rb_node *oracle_edge(const struct rb_root *root, bool last);
struct rb_node *oracle_edge(const struct rb_root *root, bool last)
{
    return last ? rb_last(root) : rb_first(root);
}
bool oracle_add_linked(struct rb_node_linked *node, struct rb_root_linked *root,
                      bool (*less)(struct rb_node *, const struct rb_node *));
bool oracle_add_linked(struct rb_node_linked *node, struct rb_root_linked *root,
                      bool (*less)(struct rb_node *, const struct rb_node *))
{
    return rb_add_linked(node, root, less);
}
void oracle_erase_aug(struct rb_node *node, struct rb_root *root,
                      const struct rb_augment_callbacks *cb);
void oracle_erase_aug(struct rb_node *node, struct rb_root *root,
                      const struct rb_augment_callbacks *cb)
{
    rb_erase_augmented(node, root, cb);
}
