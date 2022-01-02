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

fn is_inside(p: &geo::Point, curves: &Vec<geo::Line>) -> bool {
    for l in curves.iter() {
        if l.is_on(p, false, &Tolerance::default()) {
            return true;
        }
    }

    let tol = Tolerance::default().equal_point();
    let mut cn = 0;

    for l in curves.iter() {
        let sy = l.start_point.y;
        let ey = l.end_point.y;

        if (sy + tol < p.y && ey + tol > p.y) ||
                (sy + tol > p.y && ey - tol < p.y) {
            let ray = geo::Line { start_point: *p,
                                  end_point: *p + geo::Vector { x: 1.0, y: 0.0, z: 0.0 } };

            if let Ok(ip) = l.intersect_with_line(&ray, true, &Tolerance::default()) {
                if p.x < ip.x + tol {
                    cn += 1;
                }
            }
        }
    }

    cn % 2 == 1
}
