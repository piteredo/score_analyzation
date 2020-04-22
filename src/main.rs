// TODO:: 10度まではオクターブ圧縮なし、11度以上は１oct+X度に(その場合は８度から計算必要)
// TODO:: 変数としては u32 だけど、i32 と計算必須な変数は キャストがベターか、全部 i32 がベターか？
// TODO:: 範囲外音程、エラー検証（重増・重減 以上の音程は未対応
// TODO:: mod の住みわけ・命名変更、use で記述簡略

extern crate score_analyzation;
use score_analyzation::*;
use score_analyzation::step::Step;
use score_analyzation::alter::Alter;
use score_analyzation::octave::Octave;
use score_analyzation::interval;

fn main() {
    println!("\n---- calc pitch interval of 2 musical note ----");

    let note_1 = Note {
        step: Step::F,
        alter: Alter::Sharp,
        octave: Octave(4),
        // duration: Duration(4),
    };
    println!("note_1 {:?}", note_1);

    let note_2 = Note {
        step: Step::F,
        alter: Alter::Natural,
        octave: Octave(4),
        // duration: Duration(4),
    };
    println!("note_2 {:?}", note_2);


    let interval = interval(&note_1, &note_2);
    println!("result: {} {}", interval.interval_type, interval.degree);
    println!("---- end ----\n");
}
