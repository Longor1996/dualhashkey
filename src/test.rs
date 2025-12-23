use crate::*;

#[test]
fn test_from_dual_str() {
    let high = "High Half";
    let low = "Low Half";
    let hash = DualHashKey::from_dual_str(&high, &low).unwrap();
    println!("({high:?}, {low:?}) = {hash}");
}

#[test]
fn test_from_dual_pathstr() {
    let high = "root/mid/low";
    let low = "root/mid/low/name";
    let hash = DualHashKey::from_dual_str(&high, &low).unwrap();
    println!("({high:?}, {low:?}) = {hash}");
}
