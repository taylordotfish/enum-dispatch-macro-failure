//! This code compiles on nightly (2026-01-20) but fails to compile on stable (1.92).
use enum_dispatch::enum_dispatch;

struct A;
impl Example for A {}

macro_rules! define_enum {
    ($type:ident) => {
        #[enum_dispatch]
        enum ExampleEnum {
            $type
        }
    };
}

#[enum_dispatch(ExampleEnum)]
trait Example {
    fn foo(&self) {}
}

// If you move this macro invocation above the declaration of `trait Example`, it compiles.
define_enum!(A);

fn main() {
    ExampleEnum::A(A).foo();
}
