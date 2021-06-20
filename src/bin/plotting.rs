use gnuplot::{AxesCommon, Figure, PlotOption::Caption};

fn main() {

    let mut fg = Figure::new();

    fg.axes2d().set_title("TSP Shortest Path", &[])
                .lines(
		&[-3., -2., -1., 0., 1., 2., 3.],
		&[9., 4., 1., 0., 1., 4., 9.],
		&[Caption("Path")],
	);

    fg.show().unwrap();


}