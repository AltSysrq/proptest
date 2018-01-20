//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Arbitrary implementations for `std::mem`.

use std::mem::*;

use strategy::statics::static_map;
use arbitrary::*;

arbitrary!([A: Arbitrary] Discriminant<A>,
    SMapped<A, Self>, A::Parameters;
    args => static_map(any_with::<A>(args), |x| discriminant(&x))
);

lift1!(['static] Discriminant<A>;
    base => static_map(base, |x| discriminant(&x))
);

// The user is responsible for dropping!
wrap_ctor!(ManuallyDrop);

#[cfg(test)]
mod test {
    #[derive(Copy, Clone, Debug)]
    struct DummyStruct;
    arbitrary!(DummyStruct; DummyStruct);

    no_panic_test!(
        discriminant_struct => Discriminant<super::DummyStruct>,
        discriminant_enum   => Discriminant<::std::num::FpCategory>,
        manually_drop       => ManuallyDrop<u8> // Trivial destructor.
    );
}