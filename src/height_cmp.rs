use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

enum CompareOperator {
    LessEqual,
    GreaterEqual,
}

struct Comparison {
    op: CompareOperator,
    height: Decimal,
}

impl Comparison {
    fn new(op:CompareOperator, height:Decimal) -> Comparison {
        Comparison{op, height}
    }
}

#[derive(Debug, PartialEq)]
struct Range {
    lower: Decimal,
    upper: Decimal,
}

impl Range {
    fn new(lower:Decimal, upper:Decimal) -> Range {
        Range{lower, upper}
    }
}

fn solve(cmps:Vec::<Comparison>) -> Range {
    let mut lower = dec!(0);
    let mut upper = dec!(999);
    for cmp in cmps {
        match cmp.op {
            CompareOperator::GreaterEqual => {
                if cmp.height >= lower {
                    lower = cmp.height;
                }
            },
            CompareOperator::LessEqual => {
                if cmp.height <= upper {
                    upper = cmp.height;
                }
            },
        }
    }
    Range{lower, upper}
}

#[cfg(test)]
mod tests {
    use super::Comparison;
    use super::CompareOperator;
    use super::Range;
    use rust_decimal_macros::dec;

    #[test]
    fn test0(){
        let cmps = vec![
            Comparison::new(CompareOperator::LessEqual, dec!(120.3)),
            Comparison::new(CompareOperator::GreaterEqual, dec!(115.7)),
            Comparison::new(CompareOperator::LessEqual, dec!(122.0)),
            Comparison::new(CompareOperator::GreaterEqual, dec!(116.9)),
            Comparison::new(CompareOperator::LessEqual, dec!(119.1)),
            Comparison::new(CompareOperator::LessEqual, dec!(117.6)),
        ];
        let want = Range::new(dec!(116.9), dec!(117.6));
        let got = super::solve(cmps);
        assert_eq!(want, got);
    }
}