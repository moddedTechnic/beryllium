
fn _start() {
    let mut x = 0;

    loop {
        x += 1;
        if (x < 10)
            continue;
        break;
    }

    exit(x);
}

