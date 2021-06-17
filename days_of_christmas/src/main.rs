fn main() {
    let day_event = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "golden rings",
        "geese a laying",
        "swans a swimming",
        "maids a milking",
        "ladies dancing",
        "lords a leaping",
        "pipers piping",
        "drummers drumming",
    ];

    for current_day in 0..day_event.len() {
        let spelt_day: String = {
            let suffix = match current_day {
                0 => "st",
                1 => "nd",
                2 => "rd",
                _ => "th"   //'_' = catch all
            };

            (current_day + 1).to_string() + suffix
        };

        println!("On the {} day of Chirstmas my true love gave to me", spelt_day);

        for index in (0..current_day).rev() {
            println!(
                "{} {}", 
                index + 2,              //accomodate with the index skipping below.
                day_event[index + 1]    //skips the first day event.
            );
        }

        println!("{} {}\n", if current_day > 0 {"And a"} else {"A"}, day_event[0]);
    }
}