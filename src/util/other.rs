pub(crate) const fn array_max<const N: usize>(arr: [usize; N]) -> usize {
    let _ = N - 1; // assert!(N >= 1);

    let mut mx = arr[0];
    let mut i = 1;

    while i < arr.len() {
        if arr[i] > mx {
            mx = arr[i];
        }
        i += 1;
    }

    mx
}
