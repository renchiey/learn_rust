#[macro_export]
macro_rules! test {
    ({ $ (($x:expr, fart)), * }) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
