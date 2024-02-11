use std::{sync::Barrier, cmp};

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

pub fn sort<const SPLITTERS: usize, const BLOCKSIZE: usize, T: Copy + Ord>(
    full_array: &mut [T],  // unsafe, careful switching
    splitters: StackVec<SPLITTERS, T>,
    barrier: Barrier,
    rank: usize,
    thread_count: usize,
) {
    let splitters = splitters.as_slice();
    let mut buffers: [StackVec<BLOCKSIZE, T>; SPLITTERS] = unsafe { std::mem::zeroed() };
    let mut bucket_counter: [usize; SPLITTERS] = unsafe { std::mem::zeroed() };

    let elements_per_thread: usize = full_array.len() / thread_count + 1;
    let blocks_per_thread = elements_per_thread / BLOCKSIZE + 1;
    let left = BLOCKSIZE * blocks_per_thread * rank;
    let right = cmp::min(
        left + blocks_per_thread,
        full_array.len(),
    );
    // right: not aligned for last thread
    // left: always aligned

    let mut next_block_index = left;
    for i in left..right {
        let value = full_array[i];
        let bucket_index = flat(splitters.binary_search(&value));
        if buffers[bucket_index].full() {
            full_array[next_block_index..next_block_index + BLOCKSIZE].copy_from_slice(
                buffers[bucket_index].as_slice()
            );
            buffers[bucket_index].clear();
        }
        buffers[bucket_index].push(value);
    }
    barrier.wait();

    // Each thread has bucketized it's local stripe.
}
