use super::*;

pub(crate) fn build_tree(freqs: &[FreqType]) -> Rc<Option<TreeNode<u8>>> {
    let empty_node = TreeNode::<u8>::empty_node_ref();
    let mut freqs_nodes: BTreeMap<(FreqType, u16), Rc<Option<TreeNode<u8>>>> = freqs
        .iter()
        .enumerate()
        .filter(|(_, &freq)| freq != 0)
        .map(|(i, &freq)| {
            (
                (freq, i as u16),
                Rc::new(Some(TreeNode::<u8>::new(
                    i as u8,
                    empty_node.clone(),
                    empty_node.clone(),
                ))),
            )
        })
        .collect();

    while freqs_nodes.len() > 1 {
        let ((freq1, key), node1) = freqs_nodes.pop_first().unwrap();
        let ((freq2, _), node2) = freqs_nodes.pop_first().unwrap();

        let summing_node = Rc::new(Some(TreeNode::<u8>::new(0, node1, node2)));
        freqs_nodes.insert((freq1 + freq2, key), summing_node);
    }

    freqs_nodes.pop_first().unwrap().1
}

pub(crate) fn walk_tree(
    root: Rc<Option<TreeNode<u8>>>,
    prefix: &mut BitVec,
    codes: &mut Vec<BitVec>,
) {
    if let Some(ref root) = root.as_ref() {
        if root.is_leaf() {
            codes[root.data as usize] = prefix.clone();
        }
        if root.left.is_some() {
            prefix.push_back(0);
            walk_tree(root.left.clone(), prefix, codes);
            prefix.pop_back();
        }
        if root.right.is_some() {
            prefix.push_back(1);
            walk_tree(root.right.clone(), prefix, codes);
            prefix.pop_back();
        }
    }
}
