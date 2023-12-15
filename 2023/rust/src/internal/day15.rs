use std::collections::HashMap;

pub fn process1(input: &str) -> u32 {
    input.split(',').map(hash).sum()
}

type Boxes<'a> = HashMap<u8, Vec<(&'a str, u8)>, std::hash::BuildHasherDefault<nohash::NoHashHasher<u8>>>;

pub fn process2(input: &str) -> u64 {
    let mut boxes: Boxes = HashMap::with_capacity_and_hasher(
        256,
        std::hash::BuildHasherDefault::default(),
    );

    input.split(',').for_each(|part| {
        let mut split = part.split(|c| c == '-' || c == '=');
        let lens_type = split.next().unwrap();
        let hash = hash(lens_type) as u8;
        if part.contains('-') {
            if let Some(lenses) = boxes.get_mut(&hash) {
                if let Some(stored_idx) = lenses
                    .iter()
                    .position(|(lens_type2, _)| &lens_type == lens_type2)
                {
                    lenses.remove(stored_idx);
                }
            }
        } else {
            let lens_strength = split.next().unwrap().parse::<u8>().unwrap();
            if let Some(lenses) = boxes.get_mut(&hash) {
                if let Some(stored_idx) = lenses
                    .iter()
                    .position(|(lens_type2, _)| &lens_type == lens_type2)
                {
                    *lenses.get_mut(stored_idx).unwrap() = (lens_type, lens_strength);
                } else {
                    lenses.push((lens_type, lens_strength));
                }
            } else {
                boxes.insert(hash, vec![(lens_type, lens_strength)]);
            }
        }
    });

    boxes
        .iter()
        .map(|(idx, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_idx, (_, strength))| {
                    (*idx as u64 + 1) * (lens_idx as u64 + 1) * *strength as u64
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

fn hash(input: &str) -> u32 {
    input.chars().fold(0, |acc, c| {
        ((acc + c as u32) * 17) % 256
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    const EXAMPLE_1: &str = indoc! {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    };

    #[rstest]
    #[case::c1(EXAMPLE_1, 1320)]
    fn process1_test(#[case] input: &str, #[case] expected: u32) {
        let result = process1(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(EXAMPLE_1, 145)]
    fn process2_test(#[case] input: &str, #[case] expected: u64) {
        let result = process2(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c01("rn=1", 30)]
    #[case::c02("cm-", 253)]
    #[case::c03("qp=3", 97)]
    #[case::c04("cm=2", 47)]
    #[case::c05("qp-", 14)]
    #[case::c06("pc=4", 180)]
    #[case::c07("ot=9", 9)]
    #[case::c08("ab=5", 197)]
    #[case::c09("pc-", 48)]
    #[case::c10("pc=6", 214)]
    #[case::c11("ot=7", 231)]
    fn hash_test(#[case] input: &str, #[case] expected: u32) {
        let result = hash(input);

        assert_eq!(result, expected);
    }
}
