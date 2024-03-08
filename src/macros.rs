macro_rules! fr2sq {
    ($file:expr, $rank:expr) => {
        (21 + $file) + ($rank * 10)
    };
}