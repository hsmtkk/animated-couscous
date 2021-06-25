use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn cal_rest(seisen:Decimal, p:Decimal, q:Decimal) -> Decimal{
    let seisen_hanbai = seisen * p;
    let souzai = seisen - seisen_hanbai;
    let souzai_hanbai = souzai * q;
    let urenokori = souzai - souzai_hanbai;
    urenokori
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    #[test]
    fn test_cal_rest(){
        let want = dec!(0.12);
        let got = super::cal_rest(dec!(1), dec!(0.8), dec!(0.4));
        assert_eq!(want, got)
    }

    #[test]
    fn test_cal_rest2(){
        let want = dec!(3.312);
        let got = super::cal_rest(dec!(10), dec!(0.31), dec!(0.52));
        assert_eq!(want, got)
    }
}
