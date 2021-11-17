#[cfg(test)]
mod tests {
    use crate::coordinate::check_coordinate::{check_coordinate, Coordinate, Position};
    use crate::ipcheck::ip_check::{check_ip_address, IpAddress};

    #[test]
    fn first_coordinate() {
        let point = (7, 8);
        assert_eq!(
            check_coordinate(point),
            Ok(Position::First(
                Coordinate::Abscissa(7),
                Coordinate::Ordinate(8),
            ))
        )
    }

    #[test]
    fn second_coordinate() {
        let point = (-10, 30);
        assert_eq!(
            check_coordinate(point),
            Ok(Position::Second(
                Coordinate::Abscissa(-10),
                Coordinate::Ordinate(30),
            ))
        )
    }

    #[test]
    fn third_coordinate() {
        let point = (-5, -6);
        assert_eq!(
            check_coordinate(point),
            Ok(Position::Third(
                Coordinate::Abscissa(-5),
                Coordinate::Ordinate(-6),
            ))
        )
    }

    #[test]
    fn fourth_coordinate() {
        let point = (7, -7);
        assert_eq!(
            check_coordinate(point),
            Ok(Position::Fourth(
                Coordinate::Abscissa(7),
                Coordinate::Ordinate(-7),
            ))
        )
    }

    #[test]
    fn x_axis() {
        let point = (7, 0);
        assert_eq!(
            check_coordinate(point),
            Ok(Position::XAxis(
                Coordinate::Abscissa(7),
                Coordinate::Ordinate(0),
            ))
        )
    }

    #[test]
    fn y_axis() {
        let point = (0, 5);
        assert_eq!(
            check_coordinate(point),
            Ok(Position::YAxis(
                Coordinate::Abscissa(0),
                Coordinate::Ordinate(5),
            ))
        )
    }

    #[test]
    fn origin() {
        let point = (0, 0);
        assert_eq!(
            check_coordinate(point),
            Ok(Position::Origin(
                Coordinate::Abscissa(0),
                Coordinate::Ordinate(0),
            ))
        );
    }

    #[test]
    fn check_ip_for_class_a() {
        assert_eq!(
            check_ip_address((98, 1, 7, 10)),
            Ok(IpAddress::ClassA("98.1.7.10".to_string()))
        );
    }

    #[test]
    fn check_ip_for_class_b() {
        assert_eq!(
            check_ip_address((150, 98, 100, 100)),
            Ok(IpAddress::ClassB("150.98.100.100".to_string()))
        );
    }

    #[test]
    fn check_ip_for_class_c() {
        assert_eq!(
            check_ip_address((222, 155, 52, 42)),
            Ok(IpAddress::ClassC("222.155.52.42".to_string()))
        );
    }

    #[test]
    fn check_ip_for_class_d() {
        assert_eq!(
            check_ip_address((235, 100, 78, 74)),
            Ok(IpAddress::ClassD("235.100.78.74".to_string()))
        );
    }

    #[test]
    fn check_ip_for_class_e() {
        assert_eq!(
            check_ip_address((250, 98, 100, 25)),
            Ok(IpAddress::ClassE("250.98.100.25".to_string()))
        );
    }

    #[test]
    fn check_ip_for_invalid_class() {
        assert_eq!(
            check_ip_address((458, 10000, 2859, 7849)),
            Err("Unwanted Input".to_owned())
        );
    }
}
