pub enum calendar<Fn, T>
// where T is A
//  and Fn is a
{
    Solar {
        yotta_centuries: isize,
        second: i32,
        zepto_second: isize,
    },
    Lunar {
        reverse_arq: isize,
    },
    // Fn name() {
    //
    // }
}
