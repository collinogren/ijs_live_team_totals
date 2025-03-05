/*
Copyright (c) 2023 Collin Ogren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::parser::ClubPoints;

// Quick sort implementation.

pub(crate) fn sort_results(club_points: &mut Vec<ClubPoints>) {
    sort(club_points);
    club_points.reverse();
}

fn sort(points: &mut Vec<ClubPoints>) {
    let start = 0;
    let end = points.len() - 1;
    sort_partition(points, start, end as isize);
}

fn sort_partition(points: &mut Vec<ClubPoints>, start: isize, end: isize) {
    if start < end && end - start >= 1 {
        let pivot = partition(points, start, end);
        sort_partition(points, start, pivot - 1);
        sort_partition(points, pivot + 1, end);
    }
}

fn partition(points: &mut Vec<ClubPoints>, l: isize, h: isize) -> isize {
    let pivot = points[h as usize].calc_total();
    let mut i = l - 1;

    for j in l..h {
        if &points[j as usize].calc_total() <= &pivot {
            i = i + 1;
            points.swap(i as usize, j as usize);
        }
    }

    points.swap((i + 1) as usize, h as usize);

    i + 1
}