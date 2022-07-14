#[derive(Debug, PartialEq, Eq)]
struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

impl ParkingSystem {
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem { big, medium, small }
    }

    fn add_car_of_level(spaces: &mut i32) -> bool {
        match *spaces {
            0 => false,
            _ => {
                *spaces -= 1;
                true
            }
        }
    }

    pub fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => Self::add_car_of_level(&mut self.big),
            2 => Self::add_car_of_level(&mut self.medium),
            3 => Self::add_car_of_level(&mut self.small),
            _ => panic!("invalid car type"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            ParkingSystem::new(1, 1, 0),
            ParkingSystem {
                big: 1,
                medium: 1,
                small: 0
            }
        )
    }

    #[test]
    fn test_add_car() {
        let mut parking_system = ParkingSystem::new(1, 1, 0);

        assert_eq!(parking_system.add_car(1), true);
        assert_eq!(
            parking_system,
            ParkingSystem {
                big: 0,
                medium: 1,
                small: 0
            }
        );

        assert_eq!(parking_system.add_car(2), true);
        assert_eq!(
            parking_system,
            ParkingSystem {
                big: 0,
                medium: 0,
                small: 0
            }
        );

        assert_eq!(parking_system.add_car(3), false);
        assert_eq!(
            parking_system,
            ParkingSystem {
                big: 0,
                medium: 0,
                small: 0
            }
        );

        assert_eq!(parking_system.add_car(1), false);
    }
}
