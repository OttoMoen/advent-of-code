use crate::utils::{Pos3D, Pos3DPairWithDistSq};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

pub fn n_closest_pairs_fast(points: &[Pos3D], n: usize) -> Vec<(Pos3D, Pos3D, f64)> {
    if points.len() < 2 {
        return vec![];
    }

    let mut heap = BinaryHeap::with_capacity(n + 1);

    for (i, a) in points.iter().enumerate() {
        for b in &points[i + 1..] {
            let dist_sq = a.distance_squared_to(b);
            let entry = Reverse(Pos3DPairWithDistSq {
                a: *a,
                b: *b,
                dist_sq,
            });

            if heap.len() < n {
                heap.push(entry);
            } else if dist_sq < heap.peek().unwrap().0.dist_sq {
                heap.pop();
                heap.push(entry);
            }
        }
    }

    let mut result: Vec<_> = heap
        .into_iter()
        .map(|Reverse(p)| (p.a, p.b, (p.dist_sq as f64).sqrt()))
        .collect();

    result.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    result
}

pub fn clusters_from_pairs(pairs: &[(Pos3D, Pos3D, f64)]) -> Vec<Vec<Pos3D>> {
    let mut clusters: Vec<Vec<Pos3D>> = Vec::new();

    for &(a, b, _) in pairs {
        // Find indices first (immutable), then borrow mutably only once
        let idx_a = clusters.iter().position(|c| c.contains(&a));
        let idx_b = clusters.iter().position(|c| c.contains(&b));

        match (idx_a, idx_b) {
            (Some(i), Some(j)) if i == j => {}
            (Some(i), Some(j)) => {
                let remove_idx = i.max(j);
                let keep_idx = i.min(j);
                let mut other = clusters.swap_remove(remove_idx);
                clusters[keep_idx].append(&mut other);
            }
            (Some(i), None) => clusters[i].push(b),
            (None, Some(j)) => clusters[j].push(a),
            (None, None) => clusters.push(vec![a, b]),
        }
    }

    clusters.sort_by_key(|c| std::cmp::Reverse(c.len()));
    clusters
}

pub fn part1() {
    let positions: Vec<Pos3D> = fs::read_to_string("data/day8.txt")
        .expect("Could not read data")
        .lines()
        .map(|line| line.parse::<Pos3D>().expect("Failed to parse line"))
        .collect();

    let pairs = n_closest_pairs_fast(&positions, 1000);

    let circuits = clusters_from_pairs(&pairs);

    let mut total = 1;
    for circuit in circuits.iter().take(3) {
        total *= circuit.len();
    }

    println!("Product of top 3 circuits: {total}");
}

pub fn exact_final_connecting_pair(points: &[Pos3D]) -> Option<(Pos3D, Pos3D, f64)> {
    let n = points.len();
    if n <= 1 {
        return None;
    }

    let pos_to_idx: HashMap<Pos3D, usize> =
        points.iter().enumerate().map(|(i, &p)| (p, i)).collect();

    let mut parent: Vec<usize> = (0..n).collect();
    let mut components = n;

    let find = |parent: &mut Vec<usize>, mut x: usize| -> usize {
        while parent[x] != x {
            x = parent[x];
        }
        x // simple, no full path compression (fine for small n)
    };

    // Better: with path compression
    let find = |parent: &mut Vec<usize>, x: usize| -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    };

    let mut pair_idx = 1; // start from closest
    loop {
        let pairs = n_closest_pairs_fast(points, pair_idx);
        if pairs.is_empty() {
            return None;
        }

        // The newest pair is the last one (farthest among top pair_idx)
        let &(a, b, dist) = pairs.last().unwrap();

        if let (Some(&i), Some(&j)) = (pos_to_idx.get(&a), pos_to_idx.get(&b)) {
            let pi = find(&mut parent, i);
            let pj = find(&mut parent, j);

            if pi != pj {
                // This pair merges two different components
                parent[pi] = pj; // union (simple, no rank for brevity; add rank if needed)
                components -= 1;

                if components == 1 {
                    return Some((a, b, dist));
                }
            }
        }

        pair_idx += 1;
        if pair_idx > 100_000 {
            return None;
        }
    }
}

pub fn part2() {
    let positions: Vec<Pos3D> = fs::read_to_string("data/day8.txt")
        .expect("Could not read data")
        .lines()
        .map(|line| line.parse::<Pos3D>().expect("Failed to parse line"))
        .collect();

    let pairs = exact_final_connecting_pair(&positions).unwrap();

    println!("Product of X coordinates are: {:?}", pairs.0.x * pairs.1.x);
}
