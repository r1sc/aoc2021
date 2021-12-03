#[macro_export]
macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().unwrap().parse::<$x>().unwrap(),)*)
    }}
}