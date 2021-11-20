use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct TreeNode<T> {
    pub(crate) data: T,
    pub(crate) left: NodeRef<T>,
    pub(crate) right: NodeRef<T>,
}

type NodeRef<T> = Rc<Option<TreeNode<T>>>;

impl<T> TreeNode<T> {
    pub(crate) fn new(data: T, left: NodeRef<T>, right: NodeRef<T>) -> Self {
        Self { data, left, right }
    }

    pub(crate) fn empty_node_ref() -> NodeRef<T> {
        Rc::new(None)
    }

    pub(crate) fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}
