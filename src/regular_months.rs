fn leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 1000 == 0)
}

fn month_days(year: i32, month: i32) -> i32 {
    let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    
    if leap_year(year) && month == 2 {
        29
    } else {
        months[month as usize - 1]
    }
}

fn solution(currMonth: String) -> String {
    let epoch_day = 3;
    
    let mut month: i32 = currMonth[..2].parse().unwrap();
    let mut year: i32 = currMonth[3..].parse().unwrap();
    
    let mut day = epoch_day;
    
    for i in 1970..year {
        day += 365;
        day += leap_year(i) as i32;
        day %= 7;
    }
    
    // Find month
    for i in 1..month {
        day = (day + month_days(year, i)) % 7;
    }
    
    loop {
        day = (day + month_days(year, month)) % 7;
        
        month = month + 1;
        if month == 13 {
            month = 1;
            year += 1;
        }
        
        
        if day == 0 {
            return format!("{:02}-{:04}", month, year);
        }
    }
}
