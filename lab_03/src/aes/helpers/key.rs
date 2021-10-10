
use super::*;

const RCON: [[u8; 4]; 16] = [
    [0x8d, 0x00, 0x00, 0x00],
    [0x00, 0x00, 0x00, 0x00],
    [0x01, 0x00, 0x00, 0x00],
    [0x02, 0x00, 0x00, 0x00],
    [0x04, 0x00, 0x00, 0x00],
    [0x08, 0x00, 0x00, 0x00],
    [0x10, 0x00, 0x00, 0x00],
    [0x20, 0x00, 0x00, 0x00],
    [0x40, 0x00, 0x00, 0x00],
    [0x80, 0x00, 0x00, 0x00],
    [0x1b, 0x00, 0x00, 0x00],
    [0x36, 0x00, 0x00, 0x00],
    [0x6c, 0x00, 0x00, 0x00],
    [0xd8, 0x00, 0x00, 0x00],
    [0xab, 0x00, 0x00, 0x00],
    [0x4d, 0x00, 0x00, 0x00],
];

pub(crate) fn generate_keys(key: &[u8]) -> Vec<Vec<u8>> {
    let nb = 4;
    let nk = match key.len() {
        16 => 4,
        24 => 6,
        32 => 8,
        _ => unreachable!("Wrong key size!"),
    };
    let nr = nk + 6;

    let mut output = vec![vec![0; 16]; nr + 1];

    for i in 0..nk {
        let j = i / 4;
        let k = 4 * i;
        let ok = (4 * i) % 16;
        output[j][ok] = key[k];
        output[j][ok + 1] = key[k + 1];
        output[j][ok + 2] = key[k + 2];
        output[j][ok + 3] = key[k + 3];
    }

    for i in nk..(nb * (nr + 1)) {
        let mut tmp = {
            let j = (i - 1) / 4;
            let k = ((i - 1) * 4) % 16;
            output[j][k..(k + 4)].to_vec()
        };

        if i % nk == 0 {
            rot_slice_left(&mut tmp, 1);
            sub_slice(&mut tmp);
            xor(&mut tmp, &RCON[i / nk]);
        } else if nk > 6 && i % nk == 4 {
            sub_slice(&mut tmp);
        }

        let range_begin = ((i - nk) * 4) % 16;
        let range = range_begin..(range_begin + 4);
        xor(&mut tmp, &output[(i - nk) / 4][range]);

        let range_begin = (i * 4) % 16;
        let range = range_begin..(range_begin + 4);
        output[i / 4][range].copy_from_slice(&tmp);
    }

    output
}
