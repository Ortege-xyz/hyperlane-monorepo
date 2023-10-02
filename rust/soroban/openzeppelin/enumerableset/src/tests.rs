use soroban_sdk::{BytesN, Env, Vec};

use crate::EnumerableSet;

fn u32_to_u8_array(n: u32) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[28..32].copy_from_slice(&n.to_be_bytes());
    return bytes;
}

#[test]
fn add() {
    let env = Env::default();
    let mut set = EnumerableSet::new(env.clone());

    let one = BytesN::from_array(&env, &u32_to_u8_array(1));
    let two = BytesN::from_array(&env, &u32_to_u8_array(2));

    assert_eq!(set.add(one.clone()), true);
    assert_eq!(set.add(two.clone()), true);
    assert_eq!(set.add(one.clone()), false); // Duplicate entry

    let expected = Vec::from_array(&env, [one.clone(), two.clone()]);

    assert_eq!(set.values(), &expected);
}

#[test]
fn contains() {
    let env = Env::default();
    let mut set = EnumerableSet::new(env.clone());

    let one = BytesN::from_array(&env, &u32_to_u8_array(1));
    let two = BytesN::from_array(&env, &u32_to_u8_array(2));
    let three = BytesN::from_array(&env, &u32_to_u8_array(3));

    set.add(one.clone());
    set.add(two.clone());

    assert_eq!(set.contains(one), true);
    assert_eq!(set.contains(two), true);
    assert_eq!(set.contains(three), false);
}

#[test]
fn at() {
    let env = Env::default();
    let mut set = EnumerableSet::new(env.clone());

    let one = BytesN::from_array(&env, &u32_to_u8_array(1));
    let two = BytesN::from_array(&env, &u32_to_u8_array(2));

    set.add(one.clone());
    set.add(two.clone());

    assert_eq!(set.at(0), Some(one));
    assert_eq!(set.at(1), Some(two));
    assert_eq!(set.at(2), None);
}

#[test]
fn remove() {
    let env = Env::default();
    let mut set = EnumerableSet::new(env.clone());

    let one = BytesN::from_array(&env, &u32_to_u8_array(1));
    let two = BytesN::from_array(&env, &u32_to_u8_array(2));

    assert_eq!(set.remove(one.clone()), false);

    set.add(one.clone());
    set.add(two.clone());

    assert_eq!(set.remove(one.clone()), true);
    assert_eq!(set.remove(one), false); // No longer in the set

    let expected = Vec::from_array(&env, [two]);

    assert_eq!(set.values(), &expected);
}

#[test]
fn len() {
    let env = Env::default();
    let mut set = EnumerableSet::new(env.clone());

    assert_eq!(set.len(), 0);

    let one = BytesN::from_array(&env, &u32_to_u8_array(1));
    let two = BytesN::from_array(&env, &u32_to_u8_array(2));

    set.add(one.clone());
    assert_eq!(set.len(), 1);

    set.add(two);
    assert_eq!(set.len(), 2);

    set.remove(one);
    assert_eq!(set.len(), 1);
}
