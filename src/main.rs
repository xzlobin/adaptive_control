use plotly::common::{Mode, Title};
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Scatter};

use std::fs;


fn main() -> std::io::Result<()> {
    let trace = Scatter::new(vec![1,  2, 3], vec![4, 8, 14]).mode(Mode::Markers);

    let layout = Layout::new().x_axis(Axis::new().title(Title::from("X Axis")))
        .y_axis(Axis::new().title(Title::from("Y Axis")))
        .title(Title::from("My Plot"));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);
    //plot.show();
    fs::create_dir_all("docs/_includes/figures")?;
    let content = plot.to_inline_html(None);
    fs::write("docs/_includes/figures/test.html", content)?;
    
    Ok(())
}