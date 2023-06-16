fn solution(lrc: Vec<String>, songLength: String) -> Vec<String> {
    use::std::collections::VecDeque;
    let mut sub = VecDeque::new();
    let mut end = format!("{},000", songLength);
    for (i,mut line) in lrc.into_iter().enumerate().rev() {
        if !line.contains(" ") {line = format!("{} ", line)};
        let (mut time, lyric) =  line.split_once(" ").unwrap();
        let mins = time[1..3].parse::<u32>().unwrap();
        let hour_min = if mins > 60 {(mins / 60, mins-60)} else {(0,mins)};
        let format_string = format!("{:0>2}:{:0>2}:{:0>2},{:0<3} --> {:0>2}:{:0>2}:{:0>2},{:0<3}", 
                    hour_min.0, hour_min.1, &time[4..6], &time[7..9], &end[0..2], &end[3..5], &end[6..8], &end[9..12]);
        sub.push_front(lyric.to_string());
        sub.push_front(format_string);
        sub.push_front((i+1).to_string());
        sub.push_front("".to_string());
        end = format!("{:0>2}:{:0>2}:{:0>2},{:0<3}", hour_min.0, hour_min.1, &time[4..6], &time[7..9]);
    }
    sub.pop_front();
    sub.into()
}