use day_01::part1::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../../res/input_day_01.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}