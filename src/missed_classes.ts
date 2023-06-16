function solution(year: number, daysOfTheWeek: number[], holidays: string[]): number {
    // Store days of the week in a hashset
    // For each holiday, check if dayofweek is in the hashset
    // if so count ++

    let days = new Set(Array.from(daysOfTheWeek, x => x % 7));
    let count = 0;
    for (let i = 0; i < holidays.length; i++) {
        let date = holidays[i].split("-");
        let yr = parseInt(date[0]) >= 9 ? year : year + 1
        let day = new Date(yr, parseInt(date[0])-1, parseInt(date[1]));
        if (days.has(day.getDay())) {
            count++;
        }
    }
    return count;
}
