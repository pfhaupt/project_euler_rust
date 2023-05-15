struct Date {
    day: usize,
    month: usize,
    year: usize,
}
const DAY_PER_MONTH: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn is_in_leap_year(date: &Date) -> bool {
    if date.year % 100 == 0 {
        return date.year % 400 == 0;
    }
    return date.year % 4 == 0;
}

fn next_day(date: &Date) -> Date {
    let mut day = date.day + 1;
    let mut month = date.month;
    let mut year = date.year;
    let mut days_in_month = DAY_PER_MONTH[month - 1]; // -1 because zero-indexing for arrays: Jan is 0
    if month == 2 && is_in_leap_year(date) {
        days_in_month += 1;
    }
    if day > days_in_month {
        day -= days_in_month;
        month += 1;
    }
    if month > 12 {
        month -= 12;
        year += 1;
    }
    return Date { day: day, month: month, year: year };
}

fn is_first(date: &Date) -> bool {
    date.day == 1
}

fn main() {
    let mut current_date = Date { day: 1, month: 1, year: 1900 };
    let mut sunday_on_first = 0;
    let mut day_of_the_week = 0;
    while current_date.year < 2001 {
        let correct_day = is_first(&current_date);
        if correct_day && day_of_the_week == 6 {
            println!("{:02}.{:02}.{:4} is a Sunday!", current_date.day, current_date.month, current_date.year);
            sunday_on_first += 1;
        }
        day_of_the_week = (day_of_the_week + 1) % 7;
        current_date = next_day(&current_date);
    }
    println!("{}", sunday_on_first);
}
