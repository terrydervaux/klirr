use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum TargetPeriod {
    /// Current period, e.g. current month or current fortnight
    Current,
    /// Last period, e.g. last month of last fortnight
    Last,
    /// Specific period, e.g. "2025-11" or "2025-11-first-half"
    Specific(PeriodAnno),
}

impl Default for TargetPeriod {
    fn default() -> Self {
        Self::Last
    }
}

impl std::fmt::Display for TargetPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Current => write!(f, "current"),
            Self::Last => write!(f, "last"),
            Self::Specific(period) => write!(f, "{}", period),
        }
    }
}

impl std::str::FromStr for TargetPeriod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "current" => Ok(Self::Current),
            "last" => Ok(Self::Last),
            _ => {
                // Try to parse as a specific period
                let period = PeriodAnno::from_str(s)?;
                Ok(Self::Specific(period))
            }
        }
    }
}

impl TargetPeriod {
    /// Note we return `YearMonthAndFortnight` since it has higher granularity than
    /// `YearAndMonth`, so we can always turn a `YearMonthAndFortnight` into a
    /// `YearAndMonth`, later in the flow if that matches the invoice cadence.
    pub fn period(&self) -> YearMonthAndFortnight {
        match self {
            Self::Current => YearMonthAndFortnight::current(),
            Self::Last => YearMonthAndFortnight::last(),
            Self::Specific(period_anno) => match period_anno {
                PeriodAnno::YearAndMonth(ym) => {
                    // Convert YearAndMonth to YearMonthAndFortnight by using the second half
                    // This ensures we get the full month when invoicing
                    YearMonthAndFortnight::year_and_month_with_half(*ym, MonthHalf::Second)
                }
                PeriodAnno::YearMonthAndFortnight(ymf) => *ymf,
            },
        }
    }
}

impl HasSample for TargetPeriod {
    fn sample() -> Self {
        Self::Current
    }

    fn sample_other() -> Self {
        Self::Last
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    type Sut = TargetPeriod;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn target_month_current() {
        let target = Sut::Current;
        let period = target.period();
        assert_eq!(period, YearMonthAndFortnight::current());
    }

    #[test]
    fn target_month_last() {
        let target = Sut::Last;
        let period = target.period();
        assert_eq!(period, YearMonthAndFortnight::current().one_half_earlier());
    }

    #[test]
    fn from_str_current() {
        let target: Sut = "current".parse().unwrap();
        assert_eq!(target, Sut::Current);
    }

    #[test]
    fn from_str_last() {
        let target: Sut = "last".parse().unwrap();
        assert_eq!(target, Sut::Last);
    }

    #[test]
    fn from_str_specific_year_and_month() {
        let target: Sut = "2025-11".parse().unwrap();
        match target {
            Sut::Specific(PeriodAnno::YearAndMonth(ym)) => {
                assert_eq!(ym, YearAndMonth::november(2025));
            }
            _ => panic!("Expected Specific(YearAndMonth)"),
        }
    }

    #[test]
    fn from_str_specific_year_month_and_fortnight() {
        let target: Sut = "2025-11-first-half".parse().unwrap();
        match target {
            Sut::Specific(PeriodAnno::YearMonthAndFortnight(ymf)) => {
                assert_eq!(*ymf.year(), Year::from(2025));
                assert_eq!(*ymf.month(), Month::November);
                assert_eq!(*ymf.half(), MonthHalf::First);
            }
            _ => panic!("Expected Specific(YearMonthAndFortnight)"),
        }
    }

    #[test]
    fn target_period_specific_converts_correctly() {
        let target = Sut::Specific(PeriodAnno::YearAndMonth(YearAndMonth::november(2025)));
        let period = target.period();
        assert_eq!(*period.year(), Year::from(2025));
        assert_eq!(*period.month(), Month::November);
        assert_eq!(*period.half(), MonthHalf::Second);
    }

    #[test]
    fn display_current() {
        assert_eq!(Sut::Current.to_string(), "current");
    }

    #[test]
    fn display_last() {
        assert_eq!(Sut::Last.to_string(), "last");
    }

    #[test]
    fn display_specific() {
        let target = Sut::Specific(PeriodAnno::YearAndMonth(YearAndMonth::november(2025)));
        assert_eq!(target.to_string(), "2025-11");
    }
}
