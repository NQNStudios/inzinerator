extern crate magick_rust;
use magick_rust::{MagickWand, magick_wand_genesis};
use std::sync::{Once, ONCE_INIT};

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when the tests are done.
static START: Once = ONCE_INIT;

fn main() {
    START.call_once(|| {
        magick_wand_genesis();
    });


    // Open 8 pages
    let top_sequence = [ 1, 8, 7, 6 ];
    let bottom_sequence = [ 2, 3, 4, 5 ];

    let mut top_wand = MagickWand::new();
    let mut bottom_wand = MagickWand::new();

    for page in top_sequence.iter() {
        let file: String = format!("test-pages/page{}.jpg",page);
        println!("{}",file);
        top_wand.read_image(file.as_str()).expect("failed to read this");
        top_wand.flop_image().expect("Failed to flop??"); 
        top_wand.flip_image().expect("Failed to flop??"); 
    }

    for page in bottom_sequence.iter() {
        let file: String = format!("test-pages/page{}.jpg",page);
        println!("{}",file);
        bottom_wand.read_image(file.as_str()).expect("failed to read this");
    }

    let mut top_wand = top_wand.append_all(false);
    let bottom_wand = bottom_wand.append_all(false);

    top_wand.add_image(&bottom_wand);
    let all_wand = top_wand.append_all(true);

    all_wand.write_image("zine.jpg").expect("failed to write this");
}
