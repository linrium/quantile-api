use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComputeErr {
    #[error("quantile prob must be from 0 to 1")]
    QuantileProbOutOfRange,

    #[error("quantile prob must be from 0 to 100")]
    PercentileProbOutOfRange,
}

pub fn quantile(arr: &Vec<i32>, prob: f64) -> Result<f64, ComputeErr> {
    if prob < 0_f64 || prob > 1_f64 {
        return Err(ComputeErr::QuantileProbOutOfRange);
    }

    if prob == 0_f64 {
        return Ok(arr[0] as f64);
    }

    if prob == 1_f64 {
        return Ok(arr[arr.len() - 1] as f64);
    }

    let len = arr.len() as f64;
    let k = (len - 1_f64) * prob;
    let f = k.floor();
    let c = k.ceil();

    if f == c {
        return Ok(arr[k as usize] as f64);
    }

    let d0 = arr[f as usize] as f64 * (c - k);
    let d1 = arr[c as usize] as f64 * (k - f);
    Ok(d0 + d1)
}

pub fn percentile(arr: &Vec<i32>, prob: f64) -> Result<f64, ComputeErr> {
    if prob < 0_f64 || prob > 100_f64 {
        return Err(ComputeErr::PercentileProbOutOfRange);
    }

    quantile(arr, prob / 100_f64)
}
