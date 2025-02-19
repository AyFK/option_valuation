pub fn stem_segments(y: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let n: usize = y.len();
    let x: Vec<f64> = (0..n).map(|v| v as f64 as f64).collect();

    // vectors to hold the line segments
    let mut x_segments = Vec::new();
    let mut y_segments = Vec::new();

    for (&x_i, &y_i) in x.iter().zip(y.iter()) {
        // start at the x-axis (0)
        x_segments.push(x_i);
        y_segments.push(0.0);

        // go to the data point (x_i, y_i)
        x_segments.push(x_i);
        y_segments.push(y_i);

        // insert a break to separate segments
        x_segments.push(f64::NAN);
        y_segments.push(f64::NAN);
    }


    return (x_segments, y_segments);
}
