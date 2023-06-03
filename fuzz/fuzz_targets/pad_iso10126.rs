#![no_main]
use libfuzzer_sys::fuzz_target;
use block_padding::{
    Iso10126, Padding,
    generic_array::{GenericArray, typenum::{U16, U32, U20}}
};

fuzz_target!(|input: (&[u8], &[u8], &[u8])| {
    let (fst, snd, thrd) = input;
    let pos = fst.len();
    if pos > 15 {
        return
    }
    let pos_2 = snd.len();
    if pos_2 > 31 {
        return
    }
    let pos_3 = thrd.len();
    if pos_3 > 19 {
        return
    }

    let mut block_16: GenericArray::<u8, U16> = [0xff; 16].into();
    block_16[..pos].copy_from_slice(&fst[0..pos]);
    Iso10126::pad(&mut block_16, pos);

    let res = Iso10126::unpad(&block_16).unwrap();
    assert_eq!(res, fst);

    let mut block_32: GenericArray::<u8, U32> = [0xff; 32].into();
    block_32[..pos_2].copy_from_slice(&snd[0..pos_2]);
    Iso10126::pad(&mut block_32, pos_2);

    let res = Iso10126::unpad(&block_32).unwrap();
    assert_eq!(res, snd);

    let mut block_20: GenericArray::<u8, U20> = [0xff; 20].into();
    block_20[..pos_3].copy_from_slice(&thrd[0..pos_3]);
    Iso10126::pad(&mut block_20, pos_3);

    let res = Iso10126::unpad(&block_20).unwrap();
    assert_eq!(res, thrd);
});