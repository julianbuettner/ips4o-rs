use crate::stackvec::StackVec;


#[allow(dead_code)]
pub fn sample_splitters<T: Ord + Copy, const SPLITTERS: usize, const SAMPLES: usize>(
    arr: &[T],
) -> StackVec<SPLITTERS, T> {
    // samples to splitters:
    // [x x s x x] [x x s x x]
    debug_assert!(SAMPLES % SPLITTERS == 0, "SAMPLES must be multiple of SPLITTERS.");
    debug_assert!((SAMPLES / SPLITTERS) % 2 == 1, "SAMPLES / SPLITTERS must be odd, not even.");
    let mut samples: StackVec<SAMPLES, &T> = StackVec::new();
    let mut splitters: StackVec<SPLITTERS, T> = StackVec::new();
    for _ in 0..SAMPLES {
        let index = fastrand::usize(..arr.len());
        samples.push(&arr[index]);
    }
    samples.sort();
    let section_size = SAMPLES / SPLITTERS;
    let pad = (section_size - 1) / 2;

    for value in samples.as_slice().iter().skip(pad).step_by(pad * 2).take(SPLITTERS) {
        splitters.push(**value);
    }

    splitters
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_splitters() {
        let values = [6,2,66,2,45,67,12,34,12,11,13,88,12,3,12,111,98,34];
        let splitters: StackVec<4, i32> = sample_splitters::<_, 4, {4 * 7}>(&values);
        for pair in splitters.as_slice().windows(2) {
            assert!(pair[0] <= pair[1]);
        }
    }
}
