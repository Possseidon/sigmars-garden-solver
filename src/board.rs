use itertools::Itertools;

use crate::{
    element::Element,
    index::{SigCoord, SigIndex},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Board {
    bits: [u8; 46],
}

impl Board {
    pub(crate) const fn new() -> Self {
        Self { bits: [0; 46] }
    }

    pub(crate) fn get(self, index: impl Into<SigIndex>) -> Option<Element> {
        let index = index.into().to_usize();
        let bits = self.bits[index / 2];
        Element::from_index(if index % 2 == 0 {
            bits >> 4
        } else {
            bits & 0xF
        })
    }

    pub(crate) fn set(&mut self, index: impl Into<SigIndex>, element: Option<Element>) {
        let index = index.into().to_usize();
        let bits = &mut self.bits[index / 2];
        let element_index = Element::to_index(element);
        *bits = if index % 2 == 0 {
            *bits & 0x0F | element_index << 4
        } else {
            *bits & 0xF0 | element_index
        };
    }

    fn is_free(self, coord: impl Into<SigCoord>) -> bool {
        let adjacent_elements = coord
            .into()
            .adjacent_cw()
            .map(|coord| coord.and_then(|coord| self.get(coord)));

        matches!(
            adjacent_elements,
            [None, None, None, _, _, _]
                | [_, None, None, None, _, _]
                | [_, _, None, None, None, _]
                | [_, _, _, None, None, None]
                | [None, _, _, _, None, None]
                | [None, None, _, _, _, None]
        )
    }

    fn free_elements(self) -> impl Iterator<Item = SigIndex> + Clone {
        SigIndex::all().filter(move |&index| self.get(index).is_some() && self.is_free(index))
    }

    pub(crate) fn is_valid_initial_state(self) -> bool {
        SigIndex::all().fold([0; 14], |mut counts, index| {
            let element_index = Element::to_index(self.get(index));
            if element_index != 0 {
                counts[element_index as usize - 1] += 1;
            }
            counts
        }) == [4, 8, 8, 8, 8, 4, 4, 5, 1, 1, 1, 1, 1, 1]
    }

    /// Returns a list of all possible moves that can be made in the current state.
    ///
    /// The moves are sorted by likelihood of not leading to a rollback, with the most likely last,
    /// so that popping off the end of the list is more efficient.
    pub(crate) fn valid_steps(self) -> Vec<Step> {
        let mut steps = Vec::<Step>::new();

        steps.extend(self.self_combinations(Element::Salt));

        steps.extend(
            self.free_elements()
                .filter(|&index| self.get(index) == Some(Element::Salt))
                .flat_map(|salt_index| {
                    [Element::Air, Element::Fire, Element::Water, Element::Earth]
                        .into_iter()
                        .flat_map(move |element| {
                            self.free_elements()
                                .filter(move |&index| self.get(index) == Some(element))
                                .map(move |element_index| Step([element_index, salt_index]))
                        })
                }),
        );

        steps.extend(
            [Element::Air, Element::Fire, Element::Water, Element::Earth]
                .into_iter()
                .flat_map(|element| self.self_combinations(element)),
        );

        steps.extend(
            self.free_elements()
                .filter(|&mors_index| (self.get(mors_index) == Some(Element::Mors)))
                .flat_map(|mors_index| {
                    self.free_elements()
                        .filter(|&vitae_index| (self.get(vitae_index) == Some(Element::Vitae)))
                        .map(move |vitae_index| Step([mors_index, vitae_index]))
                }),
        );

        let metal_steps = [
            Element::Lead,
            Element::Tin,
            Element::Iron,
            Element::Copper,
            Element::Silver,
        ]
        .into_iter()
        .find_map(|metal| {
            SigIndex::all()
                .find(|index| self.get(*index) == Some(metal))
                .map(|metal_index| {
                    self.is_free(metal_index).then(|| {
                        self.free_elements()
                            .filter(|index| self.get(*index) == Some(Element::Quicksilver))
                            .map(move |quicksilver_index| Step([metal_index, quicksilver_index]))
                    })
                })
        });

        if let Some(metal_steps) = metal_steps {
            if let Some(metal_steps) = metal_steps {
                steps.extend(metal_steps);
            }
        } else {
            steps.extend(
                self.free_elements()
                    .find(|&index| self.get(index) == Some(Element::Gold))
                    .map(|gold_index| Step([gold_index, gold_index])),
            );
        }

        steps
    }

    fn self_combinations(self, element: Element) -> impl Iterator<Item = Step> + Clone {
        self.free_elements()
            .filter(move |&index| self.get(index) == Some(element))
            .tuple_combinations::<(_, _)>()
            .map(|indices| Step([indices.0, indices.1]))
    }

    pub(crate) fn is_solved(self) -> bool {
        self == Board::new()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Step(pub(crate) [SigIndex; 2]);
