#[macro_export]
macro_rules! concat_vec {
    [$elem:expr; $n:expr] => (
        Vec::from_elem($elem, $n).concat()
    );
    [$($x:expr),+ $(,)?] => (
        vec![$($x),+].concat()
    );
}