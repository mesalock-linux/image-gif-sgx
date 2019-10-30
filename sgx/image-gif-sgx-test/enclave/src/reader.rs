use std::prelude::v1::*;
use std::untrusted::fs::*;
use gif::*;

pub fn test_simple_indexed() {
    let mut decoder = Decoder::new(File::open("tests/samples/sample_1.gif").unwrap()).read_info().unwrap();
    let frame = decoder.read_next_frame().unwrap().unwrap();
    assert_eq!(&*frame.buffer, &[
        1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
        1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
        1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
        1, 1, 1, 0, 0, 0, 0, 2, 2, 2,
        1, 1, 1, 0, 0, 0, 0, 2, 2, 2,
        2, 2, 2, 0, 0, 0, 0, 1, 1, 1,
        2, 2, 2, 0, 0, 0, 0, 1, 1, 1,
        2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
        2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
        2, 2, 2, 2, 2, 1, 1, 1, 1, 1
    ][..])
}

pub fn test_interlace_iterator() {
    for &(len, expect) in &[
        (0, &[][..]),
        (1, &[0][..]),
        (2, &[0, 1][..]),
        (3, &[0, 2, 1][..]),
        (4, &[0, 2, 1, 3][..]),
        (5, &[0, 4, 2, 1, 3][..]),
        (6, &[0, 4, 2, 1, 3, 5][..]),
        (7, &[0, 4, 2, 6, 1, 3, 5][..]),
        (8, &[0, 4, 2, 6, 1, 3, 5, 7][..]),
        (9, &[0, 8, 4, 2, 6, 1, 3, 5, 7][..]),
        (10, &[0, 8, 4, 2, 6, 1, 3, 5, 7, 9][..]),
        (11, &[0, 8, 4, 2, 6, 10, 1, 3, 5, 7, 9][..]),
        (12, &[0, 8, 4, 2, 6, 10, 1, 3, 5, 7, 9, 11][..]),
        (13, &[0, 8, 4, 12, 2, 6, 10, 1, 3, 5, 7, 9, 11][..]),
        (14, &[0, 8, 4, 12, 2, 6, 10, 1, 3, 5, 7, 9, 11, 13][..]),
        (15, &[0, 8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13][..]),
        (16, &[0, 8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15][..]),
        (17, &[0, 8, 16, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15][..]),
    ] {
        let iter = InterlaceIterator { len: len, next: 0, pass: 0 };
        let lines = iter.collect::<Vec<_>>();
        assert_eq!(lines, expect);
    }
}