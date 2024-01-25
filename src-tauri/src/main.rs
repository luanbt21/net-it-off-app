// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::VecDeque, usize, vec};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_subset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn find_subset(data: Vec<i64>) -> Option<VecDeque<usize>> {
    let (positive, negative): (Vec<_>, Vec<_>) = data.into_iter().partition(|&n| n > 0);
    let positive: Vec<usize> = positive.into_iter().map(|n| n as _).collect();
    let target_sum = negative.iter().sum::<i64>().unsigned_abs() as usize;
    println!("{}", target_sum);
    let n = positive.len();

    let mut dp = (1..=(n + 1))
        .map(|_| vec![false; target_sum + 1])
        .collect::<Vec<_>>();

    for row in dp.iter_mut() {
        row[0] = true
    }

    for i in 1..=n {
        for j in 1..=target_sum {
            if positive[i - 1] > j {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - positive[i - 1]];
            }
        }
    }

    if !dp[n][target_sum] {
        return None;
    }

    let mut subset = VecDeque::new();
    let mut i = n;
    let mut j = target_sum;

    while i > 0 && j > 0 {
        if dp[i][j] && !dp[i - 1][j] {
            subset.push_front(positive[i - 1]);
            j -= positive[i - 1];
        }
        i -= 1;
    }

    Some(subset)
}
