// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//

extern crate mirai_annotations;
extern crate taint_error;

use std::sync::Arc;
use taint_error::{source, use_arc, Foo};
fn use_arc_test() {
    let f: Foo = source(Arc::new([1, 2, 3])); // get tainted Foo
    let (_, sum) = use_arc(f); // use tainted Foo
    println!("sum = {}", sum);
}

fn main() {
    use_arc_test();
    /*
        let result = 1;
        set_model_field!(&result, tainted, true);
        println!("error happened");
        result = result + 1;
        precondition!(!get_model_field!(&result, tainted, false));
    */
}
