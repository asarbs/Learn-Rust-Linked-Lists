pub fn magnitude(_vec: [i32; 3]) -> f64 {
    let res: f64 = (_vec[0].pow(2) + _vec[1].pow(2) + _vec[2].pow(2)).into();
    res.sqrt()
}

pub fn normalize(_vec: [i32; 3]) -> [f64; 3]  {
    let mag = magnitude(_vec);
    let mut out_vec:[f64; 3] = [0.0, 0.0, 0.0];
    for x in 0..3 {
        let y: f64 = _vec[x].into();
        out_vec[x] = y / mag;
    }
    out_vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magnitude_001() {
        let vec: [i32; 3] = [0,0,1];
        let res: f64 = magnitude(vec);
        assert_eq!(res, 1.0);
    }

    #[test]
    fn magnitude_32_1() {
        let vec: [i32; 3] = [3,2,-1];
        let res: f64 = magnitude(vec);
        assert_eq!(res, 14.0_f64.sqrt() );
    }

    #[test]
    fn normalize_32_1() {
        let vec: [i32; 3] = [3,2,-1];
        let res: [f64; 3] = normalize(vec);
        let mag = 14.0_f64.sqrt();
        assert_eq!(res, [3.0/mag, 2.0/mag, -1.0/mag] );
    }
}
