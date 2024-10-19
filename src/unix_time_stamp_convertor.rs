pub fn unix_convert(unix_timestamp: i64) -> (u32, u32, u32, u32, u32, u32)
{
    const SECONDS_PER_MINUTE: i64 = 60;
    const SECONDS_PER_HOUR: i64 = SECONDS_PER_MINUTE * 60;
    const SECONDS_PER_DAY: i64 = SECONDS_PER_HOUR * 24;

    let days_since_epoch: i64 = unix_timestamp / SECONDS_PER_DAY;
    let (year, month, day) = ymd_since_epoch(days_since_epoch);
    
    let seconds_in_day:i64 = unix_timestamp % SECONDS_PER_DAY;
    let hour: u32 = (seconds_in_day / SECONDS_PER_HOUR) as u32;
    let minute: u32 = ((seconds_in_day % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as u32;
    let second: u32 = (seconds_in_day % SECONDS_PER_MINUTE) as u32;

    let utc_cor: u32 = 2; //Swiss is UTC+2

    return (year, month, day, hour + utc_cor, minute, second);
    //return (2024, 10, 7, 17, 55, 7);
}

fn ymd_since_epoch(days_since_epoch: i64) -> (u32, u32, u32)
{
    let mut days_in_year: i64 = 365;
    let mut year: i64 = 1970;
    let mut days: i64 = days_since_epoch;

    while days >= days_in_year
    {
        if is_leapyear(year)
        {
            days_in_year = 366;
        }
        else
        {
            days_in_year = 365;
        }

        days -= days_in_year;
        year += 1;
    }

    let (month, day) = get_md(days, is_leapyear(year));

    return (year as u32, month, day);
}

fn is_leapyear(year: i64) -> bool
{
    return (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
}

fn get_md(mut days: i64, is_leapyear: bool) -> (u32, u32)
{
    let mut month_ar: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leapyear {month_ar[1] += 1;}
    let mut month: usize = 0;

    days += 1;

    while month < 12
    {
        if days > month_ar[month]
        {
            days -= month_ar[month];
            month += 1;
        }
        else
        {
            break;
        }
    }

    month += 1;

    return (month as u32, days as u32);
}