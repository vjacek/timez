use std::env;
use chrono::{DateTime, Utc, Local, FixedOffset};


fn main() {

    let args: Vec<String> = env::args().collect();
    let input_time = &args[1];
    // dbg!(&input_time);

    // 2018-08-04T13:42:19-07:00
    // 1533415339

    // outputs
    let mut output_rfc3339 = String::new();
    let mut output_rfc3339_local = String::new();
    let mut output_unix = 0;


    // attempt to parse as a unix timestamp
    let unix_input = match input_time.parse::<i64>() {
        Ok(num) => num,
        Err(_) => -1,
    };

    // if conversion of the input to an int works, we're dealing with a unix timestamp
    if unix_input >= 0 {
        let dt_utc: DateTime<Utc> = DateTime::from_timestamp(unix_input, 0).expect("invalid timestamp");
        let dt_local: DateTime<Local> = DateTime::with_timezone(&dt_utc, &Local);
        output_rfc3339 = dt_utc.to_rfc3339();
        output_rfc3339_local = dt_local.to_rfc3339();
        output_unix = unix_input;
    }
    else {

        // attempt to parse as an RFC3339 string
        let rfc3339_input: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(&input_time).unwrap();
        let dt_utc: DateTime<Utc> = DateTime::with_timezone(&rfc3339_input, &Utc);
        let dt_local: DateTime<Local> = DateTime::with_timezone(&dt_utc, &Local);
        output_rfc3339 = dt_utc.to_rfc3339();
        output_rfc3339_local = dt_local.to_rfc3339();
        output_unix = DateTime::timestamp(&dt_utc);
    }
    




    println!("Input:         {input_time}");
    println!("UNIX:          {output_unix}");
    println!("RFC3339 UTC:   {output_rfc3339}");
    println!("RFC3339 Local: {output_rfc3339_local}");
    
    

}
