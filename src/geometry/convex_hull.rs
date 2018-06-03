use std;

fn extract_convex_hull(points: &Vec<Point>, contain_on_segment: bool) -> Vec<usize> {
    let n = points.len();
    if n <= 1 {
        return vec![0];
    }

    let mut ps: Vec<usize> = (0..n).collect();
    ps.sort_by(|&a, &b| {
        if points[a].x == points[b].x {
            points[a].y.partial_cmp(&points[b].y).unwrap()
        } else {
            points[a].x.partial_cmp(&points[b].x).unwrap()
        }
    });

    let mut qs: Vec<usize> = Vec::new();
    for &i in &ps {
        while qs.len() > 1 {
            let k = qs.len();
            let det = (points[qs[k - 1]] - points[qs[k - 2]]).det(&(points[i] - points[qs[k - 1]]));
            if det < 0.0 || (det <= 0.0 && !contain_on_segment) {
                qs.pop();
            } else {
                break;
            }
        }
        qs.push(i);
    }

    let t = qs.len();
    for i in (0..(n - 1)).rev() {
        let i = ps[i];
        while qs.len() > t {
            let k = qs.len();
            let det = (points[qs[k - 1]] - points[qs[k - 2]]).det(&(points[i] - points[qs[k - 1]]));
            if det < 0.0 || (det <= 0.0 && !contain_on_segment) {
                qs.pop();
            } else {
                break;
            }
        }
        qs.push(i);
    }

    qs.pop();
    return qs;
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    fn det(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_helper::TestCaseProducer;

    /// Solve http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=CGL_4_A
    #[test]
    fn solve_cgl_4_a() {
        let mut input = TestCaseProducer::new_from_directory("./assets/CGL_4_A/in/");
        let mut output = TestCaseProducer::new_from_directory("./assets/CGL_4_A/out/");
        while !input.is_empty() {
            let n: usize = input.next();
            let mut points = Vec::new();
            for _ in 0..n {
                let x: f64 = input.next();
                let y: f64 = input.next();
                points.push(Point { x: x, y: y });
            }

            let convex_hull = extract_convex_hull(&points, true);
            assert_eq!(convex_hull.len(), output.next());

            let n = convex_hull.len();
            let mut start = 0;
            for i in 0..n {
                if points[convex_hull[i]].y < points[convex_hull[start]].y
                    || (points[convex_hull[i]].y == points[convex_hull[start]].y
                        && points[convex_hull[i]].x < points[convex_hull[start]].x)
                {
                    start = i;
                }
            }

            for i in 0..n {
                let i = (i + start) % n;
                let i = convex_hull[i];
                assert_eq!(points[i].x, output.next());
                assert_eq!(points[i].y, output.next());
            }
        }
    }
}