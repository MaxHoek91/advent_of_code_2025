use std::fs;

use anyhow::Result as Result;

pub fn solve_day_01(file: &str) -> Result<(u32, u32)> {
    let data = fs::read_to_string(file)?;
    count_zeros(&data)
}


fn count_zeros(data: &str) -> Result<(u32, u32)> {
    let mut position: i32 = 50;
    let mut zeros: u32 = 0;
    let mut zero_crosses: i32 = 0;

    for line in data.lines() {
        let direction = line.get(0..1).unwrap();
        let step_size = line.get(1..).unwrap().parse::<i32>()?;

        let start_zero = position == 0;

        match direction {
            "L" => position -= step_size,
            "R" => position += step_size,
            _ => unreachable!()
        }

        if position >= 100 {
            zero_crosses += (position / 100).abs();
        } else if position <= 0 {
            zero_crosses += (position / 100).abs() + 1;
            if start_zero {
                zero_crosses -= 1;
            }
        }


        position = position.rem_euclid(100);

        if position == 0 {
            zeros += 1;
        }
    }

    Ok((zeros, zero_crosses as u32))
}


#[cfg(test)]
mod tests {
    use super::*;

    const SEQUENCE: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn test_zeros() {
        let (zeros, zero_crosses) = count_zeros(SEQUENCE).unwrap();
        assert_eq!(zeros, 3);
        assert_eq!(zero_crosses, 6);
    }

    #[test]
    fn test_edge_cases() {
        let (_zeros, zero_crosses) = count_zeros("L50\nR50").unwrap();
        assert_eq!(zero_crosses, 1);

        let (_zeros, zero_crosses) = count_zeros("L50\nL50").unwrap();
        assert_eq!(zero_crosses, 1);

        let (_zeros, zero_crosses) = count_zeros("R50\nL50").unwrap();
        assert_eq!(zero_crosses, 1);

        let (_zeros, zero_crosses) = count_zeros("R50\nR50").unwrap();
        assert_eq!(zero_crosses, 1);

        let (_zeros, zero_crosses) = count_zeros("L150\nR50").unwrap();
        assert_eq!(zero_crosses, 2);

        let (_zeros, zero_crosses) = count_zeros("L150\nL50").unwrap();
        assert_eq!(zero_crosses, 2);

        let (_zeros, zero_crosses) = count_zeros("R150\nL50").unwrap();
        assert_eq!(zero_crosses, 2);

        let (_zeros, zero_crosses) = count_zeros("R150\nR50").unwrap();
        assert_eq!(zero_crosses, 2);
    }
}
