use proptest::prelude::*;
use rquants::energy::Energy;

proptest! {
    #[test]
    fn finite_energy_ordering_has_exactly_one_relation(
        a in -1.0e18f64..1.0e18f64,
        b in -1.0e18f64..1.0e18f64,
    ) {
        let a = Energy::electron_volts(a);
        let b = Energy::electron_volts(b);
        let relation_count = u8::from(a < b) + u8::from(a == b) + u8::from(a > b);

        prop_assert_eq!(relation_count, 1);
    }
}

#[test]
fn electron_volt_equality_does_not_overlap_ordering() {
    let small = Energy::electron_volts(1.0);
    let large = Energy::electron_volts(1000.0);

    assert_ne!(small, large);
    assert!(small < large);
}
