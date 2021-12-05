use bgc::*;

fn main() {
    let mut curves = Vec::new();

    curves.push(geo::Line { start_point: geo::Point { x: 0.0, y: 0.0, z: 0.0 },
                            end_point: geo::Point { x: 70.0, y: 0.0, z: 0.0 } });
    curves.push(geo::Line { start_point: geo::Point { x: 70.0, y: 0.0, z: 0.0 },
                            end_point: geo::Point { x: 70.0, y: 20.0, z: 0.0 } });
    curves.push(geo::Line { start_point: geo::Point { x: 70.0, y: 20.0, z: 0.0 },
                            end_point: geo::Point { x: 40.0, y: 20.0, z: 0.0 } });
    curves.push(geo::Line { start_point: geo::Point { x: 40.0, y: 20.0, z: 0.0 },
                            end_point: geo::Point { x: 40.0, y: 40.0, z: 0.0 } });
    curves.push(geo::Line { start_point: geo::Point { x: 40.0, y: 40.0, z: 0.0 },
                            end_point: geo::Point { x: 0.0, y: 40.0, z: 0.0 } });
    curves.push(geo::Line { start_point: geo::Point { x: 0.0, y: 40.0, z: 0.0 },
                            end_point: geo::Point { x: 0.0, y: 0.0, z: 0.0 } });

    for l in curves.iter() {
        println!("{:?} {:?}", l.start_point, l.end_point);
    }
}
