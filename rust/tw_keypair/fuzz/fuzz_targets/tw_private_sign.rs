// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tw_hash::H256;
use tw_keypair::tw::{Curve, PrivateKey};

#[derive(Arbitrary, Debug)]
struct TWPrivateSignInput {
    private: Vec<u8>,
    hash: Vec<u8>,
    curve: Curve,
}

fuzz_target!(|input: TWPrivateSignInput| {
    if let Ok(private) = PrivateKey::new(input.private) {
        private.sign(&input.hash, input.curve).ok();
    }
});
