pub fn sin(degree: &f64) -> f64 {
	degree.to_radians().sin()
}

pub fn cos(degree: &f64) -> f64 {
	degree.to_radians().cos()
}

pub fn tan(degree: &f64) -> f64 {
	degree.to_radians().tan()
}

pub fn arcsin(degree: &f64) -> f64 {
	degree.asin().to_degrees()
}

pub fn arccos(degree: &f64) -> f64 {
	degree.acos().to_degrees()
}

pub fn arccot(degree: &f64) -> f64 {
	(1.0 / degree).atan().to_degrees()
}

pub fn arctan2(y: &f64, x: &f64) -> f64 {
	y.atan2(*x).to_degrees()
}

pub fn fix_angle(angle: f64) -> f64 {
	fix(angle, 360.0)
}

pub fn fix_hour(hour: f64) -> f64 {
	fix(hour, 24.0)
}

fn fix(a: f64, b: f64) -> f64 {
	let fixed = a % b;
	if fixed < 0.0 {
		fixed + b
	} else {
		fixed
	}
}
