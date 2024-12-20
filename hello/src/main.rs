use collider::geom::{v2, Shape};
use collider::{Collider, HbEvent, HbId, HbProfile};

#[derive(Copy, Clone, Debug)]
struct DemoHbProfile {
    id: HbId,
} // add any additional identfying data to this struct

impl HbProfile for DemoHbProfile {
    fn id(&self) -> HbId {
        self.id
    }
    fn can_interact(&self, _other: &DemoHbProfile) -> bool {
        true
    }
    fn cell_width() -> f64 {
        4.0
    }
    fn padding() -> f64 {
        0.01
    }
}

fn main() {
    let mut collider: Collider<DemoHbProfile> = Collider::new();

    let hitbox = Shape::square(2.0)
        .place(v2(-10.0, 0.0))
        .moving(v2(1.0, 0.0));
    let overlaps = collider.add_hitbox(DemoHbProfile { id: 0 }, hitbox);
    assert!(overlaps.is_empty());

    let hitbox = Shape::square(2.0)
        .place(v2(10.0, 0.0))
        .moving(v2(-1.0, 0.0));
    let overlaps = collider.add_hitbox(DemoHbProfile { id: 1 }, hitbox);
    assert!(overlaps.is_empty());

    while collider.time() < 20.0 {
        let time = collider.next_time().min(20.0);
        collider.set_time(time);
        if let Some((event, profile_1, profile_2)) = collider.next() {
            println!(
                "{:?} between {:?} and {:?} at time {}.",
                event,
                profile_1,
                profile_2,
                collider.time()
            );
            if event == HbEvent::Collide {
                println!("Speed of collided hitboxes is halved.");
                for profile in [profile_1, profile_2].iter() {
                    let mut hb_vel = collider.get_hitbox(profile.id()).vel;
                    hb_vel.value *= 0.5;
                    collider.set_hitbox_vel(profile.id(), hb_vel);
                }
            }
        }
    }

    // the above loop prints the following events:
    //   Collide between DemoHbProfile { id: 0 } and DemoHbProfile { id: 1 } at time 9.
    //   Speed of collided hitboxes is halved.
    //   Separate between DemoHbProfile { id: 0 } and DemoHbProfile { id: 1 } at time 13.01.
}

// The isize and usize types hold pointer-sized signed
// and unsigned integers, 32 bits long on 32-bit platforms, and 64 bits long on
// 64-bit platforms.

//The ! character marks this as a macro invocation,
// not a function call.
//
// Unlike C and C++, in which
// assertions can be skipped, Rust always checks assertions regardless of how
// the program was compiled. There is also a debug_assert! macro, whose
// assertions are skipped when the program is compiled for speed.

//A let statement declares a local variable, like t in our function. We don’t
// need to write out t’s type, as long as Rust can infer it from how the variable
// is used.
//
// Rust only infers types within function bodies: you must write out the
// types of function parameters and return values, as we did before. If we
// wanted to spell out t’s type, we could write:
// let t:

//Rust has a return statement, but the gcd function doesn’t need one. If a
// function body ends with an expression that is not followed by a semicolon,
// that’s the function’s return value. In fact, any block surrounded by curly
// braces can function as an expression.
