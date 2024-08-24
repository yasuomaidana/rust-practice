mod page_rank;

use ndarray::Array2;
use textwrap::fill;

fn main() {
    let initial_l = Array2::from_shape_vec(
        (4, 4),
        vec![0.0, 0.5, 0.0, 0.0,
             1.0/3.0, 0.0, 0.0, 0.5,
             1.0/3.0,0.0, 0.0, 0.5,
             1.0/3.0,0.5, 1.0, 0.0
        ]
    ).unwrap();

    println!("{}", fill(&format!("{:.4?}", initial_l), 100));
    let page_rank = page_rank::PageRank::new(1.0, 100);
    let result = page_rank.rank(initial_l);
    println!("Page rank: ");
    println!("{}", fill(&format!("{:.4?}", result.into_raw_vec_and_offset().0), 80));
}
