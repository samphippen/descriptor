#[macro_use]
extern crate descriptor;
extern crate expector;

use std::time::Duration;
use std::thread::sleep;

use descriptor::*;
use expector::*;

fn main() {
    describe("example group 1", source_location!(), |eg| {
        it!(eg, "1", || {
            expect(1).to(eq(2));
        });
        it!(eg, "2", || {
            expect("abc").to(eq("def"));
        });
        it!(eg, "3", || {
            expect(None).to(eq(Some(3)));
        });

        it!(eg, "works", || {
        });
    });

    describe("example group 2", source_location!(), |eg| {
        it!(eg, "17", || {
        });

        it!(eg, "does a lot of hard work", || {
            sleep(Duration::new(3, 0));
        });
    });

    descriptor_main();
}
