use std::env;
use std::process;

const S:f64 = 1000.0;
const M:f64 = S * 60.0;
const H:f64 = M * 60.0;
const D:f64 = H * 24.0;
const W:f64 = D * 7.0;
const Y:f64 = D * 365.25;
const MO:f64 = Y / 12.0;

enum TimeUnit {
    Years,
    Months,
    Weeks,
    Days,
    Hours,
    Minutes,
    Seconds,
    Milliseconds
}

enum InputType {
    Number(f64),
    Combined(f64, TimeUnit)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = match args.len() {
        2 => {
            if let Ok(v) = args[1].parse::<f64>(){
                 InputType::Number(v.abs())
            } else {
                eprintln!("Usage ms <number> [unit]");
                process::exit(1);
            }
           
        },
        3 => {
            let val= args[1].parse::<f64>().unwrap_or_else(|_| {
                eprintln!("Usage ms <number> [unit]");
                process::exit(1);  
            }).abs();

            let unit = match args[2].to_lowercase().as_str() {
                "years" | "year" | "yrs" | "yr" | "y" => TimeUnit::Years,
                "months" | "month" | "mo" => TimeUnit::Months,
                "weeks" | "week" | "wk" | "w" => TimeUnit::Weeks,
                "days" | "day" | "d" => TimeUnit::Days,
                "hours" | "hour" | "hrs" | "hr" | "h" => TimeUnit::Hours,
                "minutes" | "minute" | "min" | "mins" | "m" => TimeUnit::Minutes,
                "seconds" | "second" | "secs" | "sec" | "s" => TimeUnit::Seconds,
                "milliseconds" | "millisecond" | "msecs" | "msec" | "ms" => TimeUnit::Milliseconds,
                _ => {
                    eprintln!("Error: Unknown unit '{}'", args[2]);
                    process::exit(1);
                }
            };

            InputType::Combined(val,unit)
        },
        _ => {
            eprintln!("Ivalid no of arguments");
            process::exit(1);
        }
    };

    let output: String = match input {
        InputType::Number(n) => format_number_short(n),
        InputType::Combined(n, unit ) => parse_and_format(n, unit)
    };

    println!("{}", output);

}


fn format_number_short(n: f64) -> String {
    match n {
        _ if n >= Y => format!("{} y", (n/Y).round()),
        _ if n>= MO => format!("{} mo", (n/MO).round()),
        _ if n>= W => format!("{} w", (n/W).round()),
        _ if n>= D => format!("{} d", (n/D).round()),
        _ if n>= H => format!("{} h", (n/H).round()),
        _ if n>= M => format!("{} min", (n/M).round()),
        _ if n>= S => format!("{} s", (n/S).round()),
        _ => format!("{} ms", n.round()),
    }
}

fn parse_and_format(n: f64, unit:TimeUnit) -> String {
    match unit {
        TimeUnit::Years => format!("{}", n * Y),
        TimeUnit::Months => format!("{}", n * MO),
        TimeUnit::Weeks => format!("{}", n * W),
        TimeUnit::Days => format!("{}", n * D),
        TimeUnit::Hours => format!("{}", n * H),
        TimeUnit::Minutes => format!("{}", n * M),
        TimeUnit::Seconds => format!("{}", n * S),
        TimeUnit::Milliseconds => format!("{}", n),
    }
}