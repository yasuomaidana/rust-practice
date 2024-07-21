use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_area = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("y = x^2 - x + 6", ("sans-serif", 40.0))
        .x_label_area_size(35)
        .y_label_area_size(40)
        .build_cartesian_2d(-5.5..5.5, 5.0..40.0)?;

    chart
        .configure_mesh()
        .x_labels(11)
        .y_labels(5)
        .draw()?;

    chart.draw_series(LineSeries::new(
        (-50..=50).map(|x| x as f64 / 10.0).map(|x| (x, x * x - x + 6.0)),
        &BLUE,
    ))?;

    Ok(())
}