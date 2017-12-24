extern crate advtools;
use advtools::prelude::*;

fn find_bridges(parts: &[(u32, u32)], used: &mut [bool], connect: u32, mut strength: u32,
                mut length: u32, strongest: &mut u32, longest: &mut (u32, u32)) {
    for (i, &(conn_a, conn_b)) in parts.iter().enumerate() {
        if (conn_a == connect || conn_b == connect) && !used[i] {
            let new_connect = if conn_a == connect { conn_b } else { conn_a };
            // Mark part as used and update our state.
            used[i] = true;
            length += 1;
            strength += conn_a + conn_b;
            // Check if we have a new strength/length record.
            // `longest` as a tuple automatically implements the right ordering relation
            // (compare length first, then strength).
            *strongest = strength.max(*strongest);
            *longest = (length, strength).max(*longest);
            find_bridges(parts, used, new_connect, strength, length, strongest, longest);
            // Restore previous state for trying the next part.
            strength -= conn_a + conn_b;
            length -= 1;
            used[i] = false;
        }
    }
}

fn main() {
    let parts = iter_input::<String>().map(
        |line| line.split('/').map(to_u32).collect_tuple().unwrap()
    ).collect_vec();

    let mut used = vec![false; parts.len()];
    let mut strongest = 0;
    let mut longest = (0, 0);
    // Go through all bridge combinations recursively, using DFS.
    find_bridges(&parts, &mut used, 0, 0, 0, &mut strongest, &mut longest);
    // Part 1: Find maximum strength of any bridge.
    println!("Max bridge strength: {}", strongest);
    // Part 2: Find maximum strength of longest bridges.
    println!("Longest bridge strength: {}", longest.1);
}
