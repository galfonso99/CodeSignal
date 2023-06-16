fn solution(takeOffTime: String, mut minutes: Vec<i32>) -> i32 {
    const NEW_YEAR: i32 = 24 * 60;
    let ts: Vec<i32> = takeOffTime.split(':').map(|s| s.parse().unwrap()).collect();
    for i in (1..minutes.len()).rev() { minutes[i] -= minutes[i-1]; };
    let mut celeb = 0;
    let mut time = ts[0] * 60 + ts[1];
    if time == 0 { time = NEW_YEAR; }
    for m in minutes {
        if time <= NEW_YEAR && time + m >= NEW_YEAR {
            celeb += 1;
        }
        time += m - 60;
    }
    if time <= NEW_YEAR { celeb += 1; }
    celeb
}