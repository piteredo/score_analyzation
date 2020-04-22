pub mod step;
pub mod alter;
pub mod octave;
pub mod interval;

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

pub fn interval(note_1: &Note, note_2: &Note) -> interval::Interval {
    let degree_number = degree_number(note_1, note_2);
    let degree_prefix = degree_prefix(note_1, note_2, &degree_number);
    interval::Interval {
        interval_type: degree_prefix,
        degree: degree_number
    }
}

fn degree_number(note_1: &Note, note_2: &Note) -> interval::Degree {
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

fn degree_prefix(note_1: &Note, note_2: &Note, degree_number: &interval::Degree) -> interval::IntervalType {
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
        interval:: Degree::Fourth => {
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

fn scale_interval(note_1: &Note, note_2: &Note) -> u32 {
    let note_1_scale_steps = (note_1.octave.value() * OCTAVE_SCALE_STEPS) + note_1.step.scale_index();
    let note_2_scale_steps = (note_2.octave.value() * OCTAVE_SCALE_STEPS) + note_2.step.scale_index();
    let result = (note_1_scale_steps as i32 - note_2_scale_steps as i32).abs() as u32 % OCTAVE_SCALE_STEPS;
    // println!("scale_interval: {}", result);
    result
}

fn chromatic_interval(note_1: &Note, note_2: &Note) -> u32 {
    let note_1_chromatic_steps = ((note_1.octave.value() * OCTAVE_CHROMATIC_STEPS) + note_1.step.chromatic_index()) as i32 + note_1.alter.value();
    let note_2_chromatic_steps = ((note_2.octave.value() * OCTAVE_CHROMATIC_STEPS) + note_2.step.chromatic_index()) as i32 + note_2.alter.value();
    let result = (note_1_chromatic_steps as i32 - note_2_chromatic_steps as i32).abs() as u32 % OCTAVE_CHROMATIC_STEPS;
    // println!("chromatic_interval: {}", result);
    result
}
