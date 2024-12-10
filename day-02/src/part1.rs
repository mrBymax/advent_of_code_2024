use itertools::Itertools;
use miette::miette;
use nom::character::complete;
use nom::character::complete::{line_ending, space1};
use nom::multi::separated_list1;
use nom::IResult;
use tracing::instrument;

enum Direction {
    Increasing,
    Decreasing,
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, reports) = parse(_input).map_err(|e| miette!("parse failed {}", e))?;
    let result = reports
        .iter()
        .filter(|report| check_safety(report).is_ok())
        .count();

    Ok(result.to_string())
}

#[instrument(ret)]
fn check_safety(report: &Report) -> Result<(), String> {
    let mut direction: Option<Direction> = None;

    for (a, b) in report.iter().tuple_windows() {
        let diff = b - a; // Assuming we calculate `b - a` for direction.
        let diff_abs = diff.abs();

        if !(1..=3).contains(&diff_abs) {
            return Err(format!(
                "Invalid difference between {} and {}: {}",
                a, b, diff_abs
            ));
        }

        match diff.signum() {
            1 => match direction {
                Some(Direction::Decreasing) => {
                    return Err(format!(
                        "Direction switched to increasing at {} and {}",
                        a, b
                    ));
                }
                _ => direction = Some(Direction::Increasing),
            },
            -1 => match direction {
                Some(Direction::Increasing) => {
                    return Err(format!(
                        "Direction switched to decreasing at {} and {}",
                        a, b
                    ));
                }
                _ => direction = Some(Direction::Decreasing),
            },
            0 => {
                return Err(format!("No difference (0) between {} and {}", a, b));
            }
            _ => {
                return Err(format!(
                    "Unexpected difference signum between {} and {}",
                    a, b
                ));
            }
        }
    }

    Ok(())
}

type Report = Vec<i32>;

fn parse(_input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(line_ending, separated_list1(space1, complete::i32))(_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
