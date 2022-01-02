enum PointsFilter<T> {
    ToLow,
    ToHigh,
    Hit(T),
}

fn limes(v: i64) -> i64 {
    v * (v + 1) / 2
}

fn p1(x_range: (i64, i64), y_range: (i64, i64)) -> i64 {
    (1..=x_range.1)
        .skip_while(|xv| x_range.0 > limes(*xv))
        .map(|xv| {
            (1..).map(move |i| match i {
                i if i <= xv => i * xv - i * (i - 1) / 2,
                _ => limes(xv),
            })
        })
        .flat_map(|x_points| {
            (1..100)
                .map(move |yv| {
                    (1..)
                        .map(move |i| i * yv - i * (i - 1) / 2)
                        .zip(x_points.clone())
                })
                .map(move |points| {
                    let mut y_points = Vec::new();
                    for (y, x) in points {
                        if x > x_range.1 {
                            return PointsFilter::ToHigh;
                        }
                        if y < y_range.0 {
                            if x < x_range.0 {
                                return PointsFilter::ToLow;
                            }
                            return PointsFilter::ToHigh;
                        }
                        y_points.push(y);
                        if x >= x_range.0 && y <= y_range.1 {
                            return PointsFilter::Hit(*y_points.iter().max().unwrap());
                        }
                    }
                    PointsFilter::ToHigh
                })
                .skip_while(|x| matches!(x, PointsFilter::ToLow))
                .map(|x| match x {
                    PointsFilter::Hit(val) => val,
                    _ => 0,
                })
        })
        .max()
        .unwrap()
}

fn p2(x_range: (i64, i64), y_range: (i64, i64)) -> i64 {
    (1..=x_range.1)
        .skip_while(|xv| x_range.0 > limes(*xv))
        .map(|xv| {
            (1..).map(move |i| match i {
                i if i <= xv => i * xv - i * (i - 1) / 2,
                _ => limes(xv),
            })
        })
        .flat_map(|x_points| {
            (-200..100)
                .map(move |yv| {
                    (1..)
                        .map(move |i| i * yv - i * (i - 1) / 2)
                        .zip(x_points.clone())
                })
                .map(move |points| {
                    for (y, x) in points {
                        if x > x_range.1 {
                            return PointsFilter::ToHigh;
                        }
                        if y < y_range.0 {
                            if x < x_range.0 {
                                return PointsFilter::ToLow;
                            }
                            return PointsFilter::ToHigh;
                        }
                        if x >= x_range.0 && y <= y_range.1 {
                            return PointsFilter::Hit(true);
                        }
                    }
                    PointsFilter::ToHigh
                })
                .skip_while(|x| matches!(x, PointsFilter::ToLow))
                .map(|x| match x {
                    PointsFilter::Hit(_) => 1,
                    _ => 0,
                })
        })
        .sum()
}

fn main() {
    // input
    let x_range = (241, 275);
    let y_range = (-75, -49);

    println!("Part 1: {}", p1(x_range, y_range));
    println!("Part 2: {}", p2(x_range, y_range));
}
