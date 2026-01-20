use enum_dispatch::enum_dispatch;

#[enum_dispatch(ExampleEnum)]
trait Example {
    fn value(&self) -> i32;
}

#[derive(Debug)]
struct Alpha;

impl Example for Alpha {
    fn value(&self) -> i32 {
        1
    }
}

#[derive(Debug)]
struct Beta;

impl Example for Beta {
    fn value(&self) -> i32 {
        2
    }
}

macro_rules! define_enum {
    ($name:ident, $($type:ident),* $(,)?) => {
        #[enum_dispatch]
        #[derive(Debug)]
        pub enum $name {
            $(
                $type,
            )*
        }
    };
}

define_enum!(ExampleEnum, Alpha, Beta);

fn main() {
    let value = ExampleEnum::Alpha(Alpha).value();
    println!("{value}");
}
