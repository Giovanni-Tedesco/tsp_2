use gnuplot::{AxesCommon, Figure, PlotOption::Caption};

use crate::{city::Map, cycle::Cycle};

pub fn plot_graph(cycle: &Cycle, town_map: &Map) {
    let (x, y) = generate_inputs(cycle, town_map);

    let mut fg = Figure::new();

    fg.axes2d().set_title("TSP Shortest Path", &[])
                .lines(
		&[-3., -2., -1., 0., 1., 2., 3.],
		&[9., 4., 1., 0., 1., 4., 9.],
		&[Caption("Path")],
	);

    fg.show().unwrap();

}

pub fn generate_inputs(cycle: &Cycle, town_map: &Map) -> (Vec<f64>, Vec<f64>) {
    let mut x_coords: Vec<f64> = Vec::new();
    let mut y_coords: Vec<f64> = Vec::new();

    for city_number in &cycle.path {
        let city = town_map.get(city_number).unwrap();

        x_coords.push(city.x);
        y_coords.push(city.y);

    }

    return (x_coords, y_coords);

}