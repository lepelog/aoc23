pub fn main() {
    let input = [(59, 543), (68, 1020), (82, 1664), (74, 1022)];
    // let input = [
    //     (7, 9),
    //     (15, 40),
    //     (30, 200),
    // ];
    let out1: usize = input
        .iter()
        .map(|(time, record_distance)| get_ways_to_beat_race2(*time, *record_distance))
        .product();
    println!("{out1}");
    println!("{}", get_ways_to_beat_race2(71530, 940200));
    println!("{}", get_ways_to_beat_race2(59688274, 543102016641022));
}

pub fn get_ways_to_beat_race1(time: usize, record_distance: usize) -> usize {
    get_possible_distances(time)
        .filter(|distance| *distance > record_distance)
        .count()
}

// 0 = x * (time - x) - dist
// 0 = -x^2 + x * time - dist
// 0 = x^2 - x * time + dist

// x = -(-time/2) +- sqrt((-time/2)^2 -dist)

pub fn get_ways_to_beat_race2(time: usize, record_distance: usize) -> usize {
    let back_half = ((time as f64 / 2f64) * (time as f64 / 2f64) - record_distance as f64).sqrt();
    let lower_bound = ((time as f64 / 2f64) - back_half).ceil() as usize;
    let upper_bound = ((time as f64 / 2f64) + back_half).floor() as usize;
    (upper_bound - lower_bound) + 1
}

pub fn get_possible_distances(time: usize) -> impl Iterator<Item = usize> {
    (1..time).map(move |charge_time| charge_time * (time - charge_time))
}
