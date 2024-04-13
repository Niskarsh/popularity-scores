#![allow(non_snake_case)]

pub fn popularityAnalysis(scores: Vec<i32>) -> bool {

    let mut incresing = true;
    let mut decresing = true;
    let mut initial = &scores[0];
    for i in 1..scores.len()-1 {
        let current = &scores[i];
        if current > initial {
            decresing = false;
        } else if current < initial {
            incresing = false;
        } else if current == initial {
            decresing = false;
            incresing = false;
        }
        initial = &scores[i];
    }
    incresing || decresing
}