use day_01::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../../res/input_day_01.txt");
    let result = process(file).context("process part 2")?;
    println!("{}", result);
    Ok(())
}
