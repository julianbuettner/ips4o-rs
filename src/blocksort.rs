use crate::stackvec::StackVec;

fn flat(r: Result<usize, usize>) -> usize {
    match r {
        Ok(i) => i,
        Err(i) => i,
    }
}

pub fn blocksort<T: Ord + Copy, const BLOCKSIZE: usize, const SPLITTERS: usize>(
    arr: &mut [T],
    splitters: StackVec<SPLITTERS, T>,
) {
    // Zeroed StackVec is empty stack vec and are fine.
    let splitters = splitters.as_slice();
    let mut buffers: [StackVec<BLOCKSIZE, T>; SPLITTERS] = unsafe { std::mem::zeroed() };
    let mut bucket_counter: [usize; SPLITTERS] = unsafe { std::mem::zeroed() };

    let mut next_block_index = 0;
    for i in 0..arr.len() {
        let value = arr[i];
        let index = flat(splitters.binary_search(&value));
        buffers[index].push(value);
        bucket_counter[index] += 1;
        if buffers[index].full() {
            arr[next_block_index..next_block_index + BLOCKSIZE]
                .copy_from_slice(buffers[index].as_slice());
            next_block_index += BLOCKSIZE;
            buffers[index].clear();
        }
    }
}
