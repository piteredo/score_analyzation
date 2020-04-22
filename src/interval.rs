use std::fmt;

pub struct Interval {
    pub interval_type: IntervalType,
    pub degree: Degree,
}

pub enum IntervalType { Perfect, Major, Minor, Diminish, Augment, DoubleDiminish, DoubleAugument }

impl fmt::Display for IntervalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IntervalType::Perfect => write!(f, "Perfect"),
            IntervalType::Major => write!(f, "Major"),
            IntervalType::Minor => write!(f, "Minor"),
            IntervalType::Diminish => write!(f, "Diminish"),
            IntervalType::DoubleDiminish => write!(f, "DoubleDiminish"),
            IntervalType::Augment => write!(f, "Augment"),
            IntervalType::DoubleAugument => write!(f, "DoubleAugment"),
        }
    }
}

pub enum Degree { Unison, Second, Third, Fourth, Fifth, Sixth, Seventh, Octave }
impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Degree::Unison => write!(f, "unison"),
            Degree::Second => write!(f, "second"),
            Degree::Third => write!(f, "third"),
            Degree::Fourth => write!(f, "fourth"),
            Degree::Fifth => write!(f, "fifth"),
            Degree::Sixth => write!(f, "sixth"),
            Degree::Seventh => write!(f, "seventh"),
            Degree::Octave => write!(f, "octave"),
        }
    }
}
