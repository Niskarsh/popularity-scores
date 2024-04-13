use std::collections::HashMap;

use popularity_scores::popularityAnalysis;

fn main() {
    let mut popularityRecords = HashMap::new();
    popularityRecords.insert("p1", vec![1 , 2 ,3 ,4 ,5]);
    popularityRecords.insert("p2", vec![8,2,5,4]);

    for (product, scores) in popularityRecords {
        if popularityAnalysis(scores) {
            println!("Popularity of product {} is increasing/decreasing", product);
        } else {
            println!("Popularity of product {} is consolidating", product);
        }
    }
}
