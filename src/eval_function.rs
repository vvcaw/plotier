pub fn sample_points_for_function(
    function_str: &str,
    sample_point_count: u32,
    x_axis_unit_count: u32,
) -> Vec<(f64, f64)> {
    let mut points = vec![];

    let expr: Result<meval::Expr, _> = function_str.parse();

    if let Ok(valid_expr) = expr {
        if let Ok(func) = valid_expr.bind("x") {
            // Calculate interval to calculate evenly spaced points.
            let step = x_axis_unit_count as f64 / sample_point_count as f64;

            (0..(sample_point_count + 1)).for_each(|i| {
                let x_value = i as f64 * step - (x_axis_unit_count as f64 / 2.0);

                points.push((x_value, func(x_value as f64)));
            });
        }
    }

    points
}
