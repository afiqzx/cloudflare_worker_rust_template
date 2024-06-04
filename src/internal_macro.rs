#[macro_export]
macro_rules! include_all_files {
    ($x:ident; $($y:literal; $z: literal),+ $(,)?) => {
        $(
            {
                let file = include_str!($z);
                $x.add_template($y, file).unwrap();
            }
        )+
    };
}
