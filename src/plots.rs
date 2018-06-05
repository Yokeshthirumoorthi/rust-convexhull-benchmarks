// Copyright © 2018 Yokesh Thirumoorthi
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use points::Point2D;

extern crate plotlib;
use plots::plotlib::style::Point;

// This code is reffered from plotlib crate's example
pub fn draw_plot(points: &Vec<Point2D>) {
    let mut data = Vec::new();
    for point in points {
        data.push(point.to_tuple());
    }
    let s1 = plotlib::scatter::Scatter::from_slice(&data).style(
        plotlib::scatter::Style::new()
            .marker(plotlib::style::Marker::Circle)
            .colour("black")
            .size(1.75),
    );
    let v = plotlib::view::ContinuousView::new()
        .add(&s1)
        .x_range(-3.5, 3.5)
        .y_range(-3.5, 3.5)
        .x_label("X-axis")
        .y_label("Y-axis");
    plotlib::page::Page::single(&v).save("scatter.svg");
}
