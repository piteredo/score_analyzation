pub mod step;
pub mod alter;
pub mod octave;
pub mod interval;

use interval::*;
use interval::IntervalType::*;
use interval::Degree::*;

const OCTAVE_SCALE_STEPS: u32 = 7;
const OCTAVE_CHROMATIC_STEPS: u32 = 12;

#[derive(Debug)]
pub struct Note {
    pub step: step::Step,
    pub alter: alter::Alter,
    pub octave: octave::Octave,
    // duration: Duration,
    // voice: voice,
    // stem: Stem,
    // type: Type,
}

pub fn interval(note_1: &Note, note_2: &Note) -> Interval {
    let degree_number = degree_number(note_1, note_2);
    let degree_prefix = degree_prefix(note_1, note_2, &degree_number);
    Interval {
        interval_type: degree_prefix,
        degree: degree_number
    }
}

fn degree_number(note_1: &Note, note_2: &Note) -> Degree {
    match scale_interval(note_1, note_2) {
        0 => Unison,
        1 => Second,
        2 => Third,
        3 => Fourth,
        4 => Fifth,
        5 => Sixth,
        6 => Seventh,
        _ => panic!()
        // TODO: add "Octave"
        // TODO: add errMsg
    }
}

fn degree_prefix(note_1: &Note, note_2: &Note, degree_number: &interval::Degree) -> IntervalType {
    let ci = chromatic_interval(note_1, note_2) as i32;
    match degree_number {
        Unison => {
            match ci {
                0 => Perfect,
                1 => Augment,
                2 => DoubleAugument,
                _ => panic!()
            }
        }
        Second => {
            match ci - 2 {
                -2 => Diminish,
                -1 => Minor,
                0 => Major,
                1 => Augment,
                2 => DoubleAugument,
                _ => panic!()
            }
        }
        Third => {
            match ci - 4 {
                -3 => DoubleDiminish,
                -2 => Diminish,
                -1 => Minor,
                0 => Major,
                1 => Augment,
                2 => DoubleAugument,
                _ => panic!()
            }
        },
        Fourth => {
            match ci - 5 {
                -2 => DoubleDiminish,
                -1 => Diminish,
                0 => Perfect,
                1 => Augment,
                2 => DoubleAugument,
                _ => panic!()
            }
        },
        Fifth => {
            match ci - 7 {
                -2 => DoubleDiminish,
                -1 => Diminish,
                0 => Perfect,
                1 => Augment,
                2 => DoubleAugument,
                _ => panic!()
            }
        },
        Sixth => {
            match ci - 9 {
                -3 => DoubleDiminish,
                -2 => Diminish,
                -1 => Minor,
                0 => Major,
                1 => Augment,
                2 => DoubleAugument,
                _ => panic!()
            }
        },
        Seventh => {
            match ci - 11 {
                -3 => DoubleDiminish,
                -2 => Diminish,
                -1 => Minor,
                0 => Major,
                1 => Augment,
                2 => DoubleAugument,
                _ => panic!()
            }
        },
        Octave => {
            match ci - 12 {
                0 => Perfect,
                1 => Augment,
                2 => DoubleAugument,
                _ => panic!()
            }
        }
        // TODO: add errMsg
    }
}

fn scale_interval(n1: &Note, n2: &Note) -> u32 {
    let n1_oct = n1.octave.value() * OCTAVE_SCALE_STEPS;
    let n2_oct = n2.octave.value() * OCTAVE_SCALE_STEPS;
    let n1_steps = n1_oct + n1.step.scale_index();
    let n2_steps = n2_oct + n2.step.scale_index();
    (n1_steps as i32 - n2_steps as i32).abs() as u32 % OCTAVE_SCALE_STEPS
}

fn chromatic_interval(n1: &Note, n2: &Note) -> u32 {
    let n1_oct = n1.octave.value() * OCTAVE_CHROMATIC_STEPS;
    let n2_oct = n2.octave.value() * OCTAVE_CHROMATIC_STEPS;
    let n1_steps = (n1_oct + n1.step.chromatic_index()) as i32 + n1.alter.value();
    let n2_steps = (n2_oct + n2.step.chromatic_index()) as i32 + n2.alter.value();
    (n1_steps as i32 - n2_steps as i32).abs() as u32 % OCTAVE_CHROMATIC_STEPS
}
