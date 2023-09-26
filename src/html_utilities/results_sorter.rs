use crate::parser::ClubPoints;

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
        let pivot = partition(points, start as isize, end as isize);
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