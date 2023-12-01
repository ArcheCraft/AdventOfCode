mod aoc2023;
mod util;

fn main() -> eyre::Result<()> {
    let mut puzzles = util::Puzzles::new()?;
    puzzles.register_all();

    let mut puzzles_to_run = vec![];
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 => println!("Where's my name?"),
        1 => {
            for year in puzzles.get_years() {
                for day in puzzles.get_days(year) {
                    puzzles_to_run.push((year, day));
                }
            }
        }
        2 => {
            let year = args[1].parse::<u32>()?;
            for day in puzzles.get_days(year) {
                puzzles_to_run.push((year, day));
            }
        }
        3 => puzzles_to_run.push((args[1].parse::<u32>()?, args[2].parse::<u8>()?)),
        _ => println!("Too many arguments."),
    }

    for (year, day) in puzzles_to_run {
        puzzles.run(year, day)?;
    }

    Ok(())
}
