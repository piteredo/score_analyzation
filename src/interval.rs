use std::fmt;
use IntervalType::*;
use Degree::*;

pub struct Interval {
    pub interval_type: IntervalType,
    pub degree: Degree,
}

pub enum IntervalType {
    Perfect,
    Major,
    Minor,
    Diminish,
    Augment,
    DoubleDiminish,
    DoubleAugument
}

impl fmt::Display for IntervalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Perfect => write!(f, "Perfect"),
            Major => write!(f, "Major"),
            Minor => write!(f, "Minor"),
            Diminish => write!(f, "Diminish"),
            DoubleDiminish => write!(f, "DoubleDiminish"),
            Augment => write!(f, "Augment"),
            DoubleAugument => write!(f, "DoubleAugment"),
        }
    }
}

pub enum Degree {
    Unison,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Octave
}

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Unison => write!(f, "unison"),
            Second => write!(f, "second"),
            Third => write!(f, "third"),
            Fourth => write!(f, "fourth"),
            Fifth => write!(f, "fifth"),
            Sixth => write!(f, "sixth"),
            Seventh => write!(f, "seventh"),
            Octave => write!(f, "octave"),
        }
    }
}
