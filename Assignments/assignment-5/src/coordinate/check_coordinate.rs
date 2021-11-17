use log::*;
#[derive(PartialEq, Eq, Debug)]
///Coordinate enum which used to encapsulate the X_axis as Abscissa and Y_axis as Ordinate
///
/// #variant
///
/// Abscissa:-Abscissa is variant of enum Coordinate and associated with integer type
///
/// Ordinate:-Ordinate is variant of enum Coordinate and associated with integer type
pub enum Coordinate {
    Abscissa(i32),
    Ordinate(i32),
}
#[derive(PartialEq, Eq, Debug)]
///Coordinate enum which used to describe the Position of Quadrant
///
/// #variant
///
/// First:- First is variant of enum Position and associated with Coordinate type
///
/// Second:- Second is variant of enum Position and associated with Coordinate type
///
/// Third:-  Third is variant of enum Position and associated with Coordinate type
///
/// Fourth: Fourth is variant of enum Position and associated with Coordinate type
///
/// XAxis: XAxis is variant of enum Position and associated with Coordinate type
///
/// YAxis: YAxis is variant of enum Position and associated with Coordinate type
///
/// Origin: Origin is variant of enum Position and associated with Coordinate type
pub enum Position {
    First(Coordinate, Coordinate),
    Second(Coordinate, Coordinate),
    Third(Coordinate, Coordinate),
    Fourth(Coordinate, Coordinate),
    XAxis(Coordinate, Coordinate),
    YAxis(Coordinate, Coordinate),
    Origin(Coordinate, Coordinate),
}

/// check_coordinate function is used check the Quadrant of the given point
///
/// #Arguments
///
///point: A point is tuple object of integer type
///
/// #Return
///
/// No return
pub fn check_coordinate((x_axis, y_axis): (i32, i32)) -> Result<Position, String> {
    match (x_axis, y_axis) {
        (x_axis, y_axis) if x_axis > 0 && y_axis > 0 => Ok(Position::First(
            Coordinate::Abscissa(x_axis),
            Coordinate::Ordinate(y_axis),
        )),
        (x_axis, y_axis) if x_axis < 0 && y_axis > 0 => Ok(Position::Second(
            Coordinate::Abscissa(x_axis),
            Coordinate::Ordinate(y_axis),
        )),
        (x_axis, y_axis) if x_axis < 0 && y_axis < 0 => Ok(Position::Third(
            Coordinate::Abscissa(x_axis),
            Coordinate::Ordinate(y_axis),
        )),
        (x_axis, y_axis) if x_axis > 0 && y_axis < 0 => Ok(Position::Fourth(
            Coordinate::Abscissa(x_axis),
            Coordinate::Ordinate(y_axis),
        )),
        (x_axis, y_axis) if x_axis == 0 && y_axis != 0 => Ok(Position::YAxis(
            Coordinate::Abscissa(x_axis),
            Coordinate::Ordinate(y_axis),
        )),
        (x_axis, y_axis) if x_axis != 0 && y_axis == 0 => Ok(Position::XAxis(
            Coordinate::Abscissa(x_axis),
            Coordinate::Ordinate(y_axis),
        )),
        (x_axis, y_axis) if x_axis == 0 && y_axis == 0 => Ok(Position::Origin(
            Coordinate::Abscissa(x_axis),
            Coordinate::Ordinate(y_axis),
        )),
        _ =>{
            error!("Wrong Ip Provided");
            Err("Wrong input".to_string())
    }
    }
}
