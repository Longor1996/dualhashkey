use crate::*;

#[test]
fn test_from_dual_str() {
    let high = "High Half";
    let low = "Low Half";
    let hash = DualHashKey::from_dual_str(&high, &low).unwrap();
    eprintln!("{hash:?}");
}
