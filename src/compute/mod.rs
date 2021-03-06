use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ComputeErr {
    #[error("quantile prob must be from 0 to 1")]
    QuantileProbOutOfRange,

    #[error("quantile prob must be from 0 to 100")]
    PercentileProbOutOfRange,
}

pub fn quantile(arr: &Vec<i32>, prob: f32) -> Result<f32, ComputeErr> {
    if prob < 0_f32 || prob > 1_f32 {
        return Err(ComputeErr::QuantileProbOutOfRange);
    }

    if prob == 0_f32 {
        return Ok(arr[0] as f32);
    }

    if prob == 1_f32 {
        return Ok(arr[arr.len() - 1] as f32);
    }

    let len = arr.len() as f32;
    let k = (len - 1_f32) * prob;
    let f = k.floor();
    let c = k.ceil();

    if f == c {
        return Ok(arr[k as usize] as f32);
    }

    let d0 = arr[f as usize] as f32 * (c - k);
    let d1 = arr[c as usize] as f32 * (k - f);

    Ok(d0 + d1)
}

pub fn percentile(arr: &Vec<i32>, prob: f32) -> Result<f32, ComputeErr> {
    if prob < 0_f32 || prob > 100_f32 {
        return Err(ComputeErr::PercentileProbOutOfRange);
    }

    quantile(arr, prob / 100_f32)
}

#[cfg(test)]
mod tests {
    use crate::compute::{percentile, quantile, ComputeErr};

    #[test]
    fn test_percentile_success() -> Result<(), ComputeErr> {
        let arr: Vec<i32> = (1..=10).collect();
        // base on numpy library
        assert_eq!(percentile(&arr, 0.0)?, 1.0);
        assert_eq!(percentile(&arr, 10.0)?, 1.9000001);
        assert_eq!(percentile(&arr, 20.0)?, 2.8000000000000003);
        assert_eq!(percentile(&arr, 30.0)?, 3.6999999999999997);
        assert_eq!(percentile(&arr, 40.0)?, 4.6000004);
        assert_eq!(percentile(&arr, 50.0)?, 5.5);
        assert_eq!(percentile(&arr, 60.0)?, 6.3999999999999995);
        assert_eq!(percentile(&arr, 70.0)?, 7.2999997);
        assert_eq!(percentile(&arr, 80.0)?, 8.200001);
        assert_eq!(percentile(&arr, 90.0)?, 9.099999);
        assert_eq!(percentile(&arr, 100.0)?, 10.0);

        Ok(())
    }

    #[test]
    fn test_percentile_fail() -> Result<(), ComputeErr> {
        let arr: Vec<i32> = (1..=10).collect();
        assert_eq!(
            percentile(&arr, -1.0),
            Err(ComputeErr::PercentileProbOutOfRange)
        );
        assert_eq!(
            percentile(&arr, 101.0),
            Err(ComputeErr::PercentileProbOutOfRange)
        );

        Ok(())
    }

    #[test]
    fn test_quantile_fail() -> Result<(), ComputeErr> {
        let arr: Vec<i32> = (1..=10).collect();
        assert_eq!(
            quantile(&arr, -0.1),
            Err(ComputeErr::QuantileProbOutOfRange)
        );
        assert_eq!(quantile(&arr, 1.2), Err(ComputeErr::QuantileProbOutOfRange));

        Ok(())
    }
}
