pub fn py_engine(n: u64) {
    let a: u64 = n;
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    let mut z: u64 = 0;
    let mut count: u64 = 1;

    while count < n/2 {
        if a*((a/2)-count) % (((a*2)/2)-count) == 0 {
            x = count;
            y = a*((a/2)-count) / (((a*2)/2)-count);
            z = n - (x+y);

            println!("{:?}", (x, y, z));
        }
        count = count+1;
    }
}
