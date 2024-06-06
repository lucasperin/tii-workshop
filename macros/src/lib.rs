macro_rules! expose {
    ($($name:ident)*) => {
        $(
            #[no_mangle]
            pub extern "C" fn $name() -> bool {
                true
            }
        )*
    }
}

expose! {
    asd
    dsa
    fgh
    jkl
}

#[no_mangle]
pub extern "C" fn something() -> bool {
    true
}
