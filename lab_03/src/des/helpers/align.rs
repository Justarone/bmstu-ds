pub(crate) fn align_message(msg: &mut Vec<u8>) {
    let additional = (8 - (msg.len() % 8) as u8) % 8;
    msg.resize(msg.len() + additional as usize, 0);
    msg.push(additional);
}

pub(crate) fn unalign_message(msg: &mut Vec<u8>) {
    assert!(
        msg.len() >= 8 + 1,
        "minimal des encrypted message size = 8b + 1b (size of extended part)"
    );
    let to_remove = msg.pop().unwrap();
    msg.resize(msg.len() - to_remove as usize, 0);
}
