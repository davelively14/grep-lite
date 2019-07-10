fn main() {
    basic_grep();
    arrays();
}

fn basic_grep() {
    let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for (idx, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = idx + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

// Items in arrays are accessed directly on the stack instead of a pointer,
// like in C. The compiler knows the size of an array's members and calculates
// memory offsets itself, so no pointer math needed.
fn arrays() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank1, blank2];

    // &arrays reference a slice of contiguous memory
    // Can iterate on a slice without calling iter()
    for a in &arrays {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }

        print!("\t(Σ{:?} = {})", a, sum);
        println!("");
    }
}
