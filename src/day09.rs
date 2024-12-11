use aoc_2024::util;
use std::ops::Range;

fn external_defrag_and_checksum(file_map: &[Range<usize>]) -> usize {
    let mut reconstructed_map = Vec::new();
    let end = file_map.last().unwrap().end;
    reconstructed_map.reserve(end + 1);

    let mut file_idx_lo = 0;
    let mut file_idx_hi = file_map.len() - 1;
    let mut file_idx_hi_remain = file_map[file_idx_hi].end - file_map[file_idx_hi].start;
    let mut i = 0;

    while i < end && file_idx_lo <= file_idx_hi {
        if file_map[file_idx_lo].contains(&i) && file_idx_lo != file_idx_hi {
            reconstructed_map.push(file_idx_lo);
        } else if file_map[file_idx_lo].end == i {
            file_idx_lo += 1;
            // Don't increment i; not sure which branch yet.
            continue;
        } else {
            // empty; copy from file_idx_hi
            reconstructed_map.push(file_idx_hi);
            file_idx_hi_remain -= 1;
            if file_idx_hi_remain == 0 {
                file_idx_hi -= 1;
                file_idx_hi_remain = file_map[file_idx_hi].end - file_map[file_idx_hi].start
            }
        }
        i += 1;
    }
    reconstructed_map
        .iter()
        .enumerate()
        .map(|(i, &file)| i * file)
        .sum()
}

fn defrag_files_and_checksum(file_map: &[Range<usize>]) -> usize {
    let mut reconstructed_map = Vec::new();

    let end = file_map.last().unwrap().end;
    reconstructed_map.resize(end + 1, None);

    let mut file_idx_lo = 0;
    let mut file_idx_hi = file_map.len() - 1;
    let mut i = 0;

    while i < end && file_idx_lo <= file_idx_hi {
        if file_map[file_idx_lo].contains(&i) {
            assert!(reconstructed_map[i].is_none());
            reconstructed_map[i] = Some(file_idx_lo);
            i += 1;
        } else if file_map[file_idx_lo].end == i {
            file_idx_lo += 1;
            // Don't increment i; not sure which branch yet.
            continue;
        } else {
            let space_need = file_map[file_idx_hi].end - file_map[file_idx_hi].start;
            let mut tmp_file_idx = file_idx_lo;
            let mut tmp_recon_idx = i;
            let mut free_space = file_map[tmp_file_idx].start
                - std::cmp::max(file_map[tmp_file_idx - 1].end, tmp_recon_idx);
            while let Some(file_idx) = reconstructed_map[tmp_recon_idx] {
                free_space -= file_map[file_idx].end - file_map[file_idx].start;
                tmp_recon_idx += file_map[file_idx].end - file_map[file_idx].start;
            }
            while free_space < space_need && tmp_file_idx < file_idx_hi {
                tmp_recon_idx +=
                    free_space + (file_map[tmp_file_idx].end - file_map[tmp_file_idx].start);
                tmp_file_idx += 1;
                free_space = file_map[tmp_file_idx].start - file_map[tmp_file_idx - 1].end;
                while let Some(file_idx) = reconstructed_map[tmp_recon_idx] {
                    free_space -= file_map[file_idx].end - file_map[file_idx].start;
                    tmp_recon_idx += file_map[file_idx].end - file_map[file_idx].start;
                }
            }
            if free_space < space_need {
                tmp_recon_idx = file_map[file_idx_hi].start;
            }
            if i == tmp_recon_idx {
                i += space_need;
            }
            for _ in 0..space_need {
                assert!(reconstructed_map[tmp_recon_idx].is_none());
                reconstructed_map[tmp_recon_idx] = Some(file_idx_hi);
                tmp_recon_idx += 1;
            }
            file_idx_hi -= 1;
        }
    }

    reconstructed_map
        .iter()
        .enumerate()
        .map(|(i, &file)| i * file.unwrap_or(0))
        .sum()
}

fn main() {
    let input = util::get_all_input();
    let mut file_map = Vec::new();
    let mut offset = 0;
    for (i, c) in input.trim().chars().enumerate() {
        let end = offset + c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            file_map.push(offset..end);
        }
        offset = end;
    }
    println!("{}", external_defrag_and_checksum(&file_map));
    println!("{}", defrag_files_and_checksum(&file_map));
}
