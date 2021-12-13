extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate advent_of_code_2021;

fn main() {
    let a = App::new("advent_of_code_2021").author("Christopher Wells <cwellsny@gmail.com>");

    let days = get_days();
    let app = days.iter().map(|d| d.0.clone()).fold(a, |b, day| {
        b.subcommand(
            SubCommand::with_name(&format!("day{}", day)).arg(
                Arg::with_name("part")
                    .help("Selects the part to run (one, two)")
                    .required(true)
                    .index(1),
            ),
        )
    });
    let matches = app.get_matches();

    for (day, part_one, part_two) in days {
        let day_string = format!("day{}", day);

        if let Some(matches) = matches.subcommand_matches(day_string) {
            let part = matches.value_of("part").unwrap();

            match part {
                "one" => part_one(),
                "two" => part_two(),
                p => println!("Unknown part: {}", p),
            }
        }
    }
}

type AdventOfCodeDay = (String, fn(), fn());

fn get_days() -> Vec<AdventOfCodeDay> {
    vec![
        (
            "1".to_string(),
            advent_of_code_2021::one::part_one as fn(),
            advent_of_code_2021::one::part_two as fn(),
        ),
        (
            "2".to_string(),
            advent_of_code_2021::two::part_one as fn(),
            advent_of_code_2021::two::part_two as fn(),
        ),
        (
            "3".to_string(),
            advent_of_code_2021::three::part_one as fn(),
            advent_of_code_2021::three::part_two as fn(),
        ),
        (
            "4".to_string(),
            advent_of_code_2021::four::part_one as fn(),
            advent_of_code_2021::four::part_two as fn(),
        ),
        (
            "5".to_string(),
            advent_of_code_2021::five::part_one as fn(),
            advent_of_code_2021::five::part_two as fn(),
        ),
        (
            "6".to_string(),
            advent_of_code_2021::six::part_one as fn(),
            advent_of_code_2021::six::part_two as fn(),
        ),
        (
            "7".to_string(),
            advent_of_code_2021::seven::part_one as fn(),
            advent_of_code_2021::seven::part_two as fn(),
        ),
        (
            "8".to_string(),
            advent_of_code_2021::eight::part_one as fn(),
            advent_of_code_2021::eight::part_two as fn(),
        ),
        (
            "9".to_string(),
            advent_of_code_2021::nine::part_one as fn(),
            advent_of_code_2021::nine::part_two as fn(),
        ),
        (
            "10".to_string(),
            advent_of_code_2021::ten::part_one as fn(),
            advent_of_code_2021::ten::part_two as fn(),
        ),
        (
            "11".to_string(),
            advent_of_code_2021::eleven::part_one as fn(),
            advent_of_code_2021::eleven::part_two as fn(),
        ),
        (
            "12".to_string(),
            advent_of_code_2021::twelve::part_one as fn(),
            advent_of_code_2021::twelve::part_two as fn(),
        ),
        (
            "13".to_string(),
            advent_of_code_2021::thirteen::part_one as fn(),
            advent_of_code_2021::thirteen::part_two as fn(),
        ),
    ]
}
