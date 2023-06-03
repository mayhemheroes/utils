#![no_main]
use libfuzzer_sys::fuzz_target;
use block_padding::{
    NoPadding, Padding,
    generic_array::{GenericArray, typenum::{U16, U32, U20}}
};

fuzz_target!(|input: (&[u8], &[u8], &[u8])| {
    let (fst, snd, thrd) = input;
    let pos = fst.len();
    if pos > 14 {
        return
    }
    if fst.contains(&0xff) {
        return
    }
    let pos_2 = snd.len();
    if pos_2 > 30 {
        return
    }
    if snd.contains(&0xff) {
        return
    }
    let pos_3 = thrd.len();
    if pos_3 > 18 {
        return
    }
    if thrd.contains(&0xff) {
        return
    }

    let mut block_16: GenericArray::<u8, U16> = [0xff; 16].into();
    block_16[..pos].copy_from_slice(&fst[0..pos]);
    NoPadding::pad(&mut block_16, pos);
    assert!(block_16.contains(&0xff));

    let mut block_32: GenericArray::<u8, U32> = [0xff; 32].into();
    block_32[..pos_2].copy_from_slice(&snd[0..pos_2]);
    NoPadding::pad(&mut block_32, pos_2);
    assert!(block_32.contains(&0xff));

    let mut block_20: GenericArray::<u8, U20> = [0xff; 20].into();
    block_20[..pos_3].copy_from_slice(&thrd[0..pos_3]);
    NoPadding::pad(&mut block_20, pos_3);
    assert!(block_20.contains(&0xff));
});