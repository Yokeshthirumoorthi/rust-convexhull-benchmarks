use points::Point2D;

extern crate plotlib;
use plots::plotlib::style::Point;

pub fn draw_plot(points: &Vec<Point2D>) {
    let mut data = Vec::new();
    for point in points {
        data.push(point.to_tuple());
    }
    let s1 = plotlib::scatter::Scatter::from_slice(&data).style(
        plotlib::scatter::Style::new()
            .marker(plotlib::style::Marker::Square)
            .colour("burlywood")
            .size(2.),
    );
    let v = plotlib::view::ContinuousView::new()
        .add(&s1)
        .x_range(-1.5, 1.5)
        .y_range(-1.5, 1.5)
        .x_label("Some varying variable")
        .y_label("The response of something");
    plotlib::page::Page::single(&v).save("scatter.svg");
}