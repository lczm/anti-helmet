//!
//! Anti Helmet Advent of Code
//! Day 21: Reactor Reboot
//! Unit Tests
//!

mod bound_tests {
    use super::super::*;

    #[test]
    fn test_overlaps() {
        assert_eq!(Bound::new(2, 3).overlaps(&Bound::new(2, 3)), true);

        assert_eq!(Bound::new(0, 1).overlaps(&Bound::new(1, 2)), true);
        assert_eq!(Bound::new(0, 1).overlaps(&Bound::new(2, 3)), false);
        assert_eq!(Bound::new(2, 3).overlaps(&Bound::new(0, 1)), false);
        assert_eq!(Bound::new(2, 3).overlaps(&Bound::new(0, 2)), true);
    }

    #[test]
    fn test_intersect() {
        assert_eq!(
            Bound::new(2, 3).intersect(&Bound::new(0, 2)),
            Some(Bound::new(2, 2))
        );
        assert_eq!(
            Bound::new(0, 2).intersect(&Bound::new(2, 3)),
            Some(Bound::new(2, 2))
        );
        assert_eq!(
            Bound::new(0, 2).intersect(&Bound::new(0, 2)),
            Some(Bound::new(0, 2))
        );
        assert_eq!(
            Bound::new(1, 5).intersect(&Bound::new(0, 2)),
            Some(Bound::new(1, 2))
        );
        assert_eq!(Bound::new(0, 2).intersect(&Bound::new(3, 5)), None);
    }
}

mod cuboid_tests {
    use super::super::*;

    #[test]
    fn test_overlaps() {
        assert_eq!(
            Cuboid {
                x_bound: Bound::new(1, 2),
                y_bound: Bound::new(1, 2),
                z_bound: Bound::new(1, 2),
            }
            .overlaps(&Cuboid {
                x_bound: Bound::new(1, 2),
                y_bound: Bound::new(2, 3),
                z_bound: Bound::new(0, 4),
            }),
            true,
        );
        // check for false positives: overlap on < 3 axis
        assert_eq!(
            Cuboid {
                x_bound: Bound::new(1, 2),
                y_bound: Bound::new(1, 2),
                z_bound: Bound::new(1, 2),
            }
            .overlaps(&Cuboid {
                x_bound: Bound::new(1, 2),
                y_bound: Bound::new(3, 4),
                z_bound: Bound::new(1, 4),
            }),
            false,
        );
        assert_eq!(
            Cuboid {
                x_bound: Bound::new(1, 2),
                y_bound: Bound::new(1, 2),
                z_bound: Bound::new(1, 2),
            }
            .overlaps(&Cuboid {
                x_bound: Bound::new(1, 2),
                y_bound: Bound::new(3, 4),
                z_bound: Bound::new(3, 4),
            }),
            false,
        )
    }

    #[test]
    fn test_intersect() {
        assert_eq!(
            Cuboid {
                x_bound: Bound::new(1, 2),
                y_bound: Bound::new(1, 2),
                z_bound: Bound::new(1, 2),
            }
            .intersect(&Cuboid {
                x_bound: Bound::new(1, 2),
                y_bound: Bound::new(2, 3),
                z_bound: Bound::new(0, 4),
            }),
            Some(Cuboid {
                x_bound: Bound::new(1, 2),
                y_bound: Bound::new(2, 2),
                z_bound: Bound::new(1, 2),
            }),
        );
        assert_eq!(
            Cuboid {
                x_bound: Bound::new(1, 2),
                y_bound: Bound::new(1, 2),
                z_bound: Bound::new(1, 2),
            }
            .intersect(&Cuboid {
                x_bound: Bound::new(1, 2),
                y_bound: Bound::new(3, 4),
                z_bound: Bound::new(3, 4),
            }),
            None,
        )
    }
}
