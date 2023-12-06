use std::{collections::BTreeMap, io::Cursor};

use image::{
    imageops::{self, FilterType},
    io::Reader,
    GenericImageView, ImageFormat, RgbImage, SubImage,
};

use crate::{
    board::Board,
    element::Element,
    index::{SigCoord, SigIndex},
    screen::coord_to_screen,
    solver::InitialBoard,
};

pub(crate) struct Scanner {
    ref_images: BTreeMap<ElementImageKey, RgbImage>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ElementImageKey {
    Empty,
    Normal(Element),
    Blocked(Element),
}

impl Scanner {
    pub(crate) fn new() -> Self {
        Self {
            ref_images: ElementImageKey::load_ref_images(),
        }
    }

    pub(crate) fn scan_image(&self, image: &RgbImage) -> Option<InitialBoard> {
        let mut board = Board::new();
        for index in SigIndex::all() {
            board.set(index, self.scan_element(image, index));
        }
        InitialBoard::new(board)
    }

    fn scan_element(&self, image: &RgbImage, index: SigIndex) -> Option<Element> {
        let (x, y, width, height) = scan_position(index);
        let scan_image = image.view(x, y, width, height);

        // scan_image
        //     .to_image()
        //     .save(format!("scan/{x}_{y}.png"))
        //     .unwrap();

        best_element(&self.score(scan_image))
    }

    fn score(&self, scan_image: SubImage<&RgbImage>) -> Box<[(ElementImageKey, u32)]> {
        self.ref_images
            .iter()
            .map(|(element, ref_image)| (*element, compare_images(scan_image, ref_image)))
            .collect()
    }
}

fn best_element(scores: &[(ElementImageKey, u32)]) -> Option<Element> {
    scores
        .iter()
        .min_by_key(|(_, score)| score)
        .unwrap()
        .0
        .element()
}

impl ElementImageKey {
    const ALL: [Self; 29] = [
        Self::Empty,
        Self::Normal(Element::Salt),
        Self::Normal(Element::Air),
        Self::Normal(Element::Fire),
        Self::Normal(Element::Water),
        Self::Normal(Element::Earth),
        Self::Normal(Element::Vitae),
        Self::Normal(Element::Mors),
        Self::Normal(Element::Quicksilver),
        Self::Normal(Element::Lead),
        Self::Normal(Element::Tin),
        Self::Normal(Element::Iron),
        Self::Normal(Element::Copper),
        Self::Normal(Element::Silver),
        Self::Normal(Element::Gold),
        Self::Blocked(Element::Salt),
        Self::Blocked(Element::Air),
        Self::Blocked(Element::Fire),
        Self::Blocked(Element::Water),
        Self::Blocked(Element::Earth),
        Self::Blocked(Element::Vitae),
        Self::Blocked(Element::Mors),
        Self::Blocked(Element::Quicksilver),
        Self::Blocked(Element::Lead),
        Self::Blocked(Element::Tin),
        Self::Blocked(Element::Iron),
        Self::Blocked(Element::Copper),
        Self::Blocked(Element::Silver),
        Self::Blocked(Element::Gold),
    ];

    const fn image_bytes(self) -> &'static [u8] {
        match self {
            Self::Empty => include_bytes!("../elements/empty.png"),
            Self::Normal(element) => match element {
                Element::Salt => include_bytes!("../elements/normal/salt.png"),
                Element::Air => include_bytes!("../elements/normal/air.png"),
                Element::Fire => include_bytes!("../elements/normal/fire.png"),
                Element::Water => include_bytes!("../elements/normal/water.png"),
                Element::Earth => include_bytes!("../elements/normal/earth.png"),
                Element::Vitae => include_bytes!("../elements/normal/vitae.png"),
                Element::Mors => include_bytes!("../elements/normal/mors.png"),
                Element::Quicksilver => include_bytes!("../elements/normal/quicksilver.png"),
                Element::Lead => include_bytes!("../elements/normal/lead.png"),
                Element::Tin => include_bytes!("../elements/normal/tin.png"),
                Element::Iron => include_bytes!("../elements/normal/iron.png"),
                Element::Copper => include_bytes!("../elements/normal/copper.png"),
                Element::Silver => include_bytes!("../elements/normal/silver.png"),
                Element::Gold => include_bytes!("../elements/normal/gold.png"),
            },
            Self::Blocked(element) => match element {
                Element::Salt => include_bytes!("../elements/blocked/salt.png"),
                Element::Air => include_bytes!("../elements/blocked/air.png"),
                Element::Fire => include_bytes!("../elements/blocked/fire.png"),
                Element::Water => include_bytes!("../elements/blocked/water.png"),
                Element::Earth => include_bytes!("../elements/blocked/earth.png"),
                Element::Vitae => include_bytes!("../elements/blocked/vitae.png"),
                Element::Mors => include_bytes!("../elements/blocked/mors.png"),
                Element::Quicksilver => include_bytes!("../elements/blocked/quicksilver.png"),
                Element::Lead => include_bytes!("../elements/blocked/lead.png"),
                Element::Tin => include_bytes!("../elements/blocked/tin.png"),
                Element::Iron => include_bytes!("../elements/blocked/iron.png"),
                Element::Copper => include_bytes!("../elements/blocked/copper.png"),
                Element::Silver => include_bytes!("../elements/blocked/silver.png"),
                Element::Gold => include_bytes!("../elements/blocked/gold.png"),
            },
        }
    }

    fn load_image(self) -> RgbImage {
        let mut reader = Reader::new(Cursor::new(self.image_bytes()));
        reader.set_format(ImageFormat::Png);
        reader.decode().unwrap().to_rgb8()
    }

    fn load_ref_images() -> BTreeMap<ElementImageKey, RgbImage> {
        Self::ALL.map(|key| (key, key.load_image())).into()
    }

    fn element(&self) -> Option<Element> {
        match self {
            Self::Empty => None,
            Self::Normal(element) | Self::Blocked(element) => Some(*element),
        }
    }
}

fn compare_images(image: SubImage<&RgbImage>, ref_image: &RgbImage) -> u32 {
    assert_eq!(image.dimensions(), ref_image.dimensions());

    let buffer = image
        .pixels()
        .zip(ref_image.pixels())
        .flat_map(|((_, _, pixel), ref_pixel)| {
            [
                pixel[0].abs_diff(ref_pixel[0]),
                pixel[1].abs_diff(ref_pixel[1]),
                pixel[2].abs_diff(ref_pixel[2]),
            ]
        })
        .collect();
    let diff = RgbImage::from_vec(image.width(), image.height(), buffer).unwrap();

    let edges = imageops::filter3x3(&diff, &EDGE_FILTER);

    // edges.save(format!("diff/{row}_{col}_{key:?}.png")).unwrap();

    let average = imageops::resize(&edges, 1, 1, FilterType::Triangle);
    let average_pixel = average.get_pixel(0, 0);

    average_pixel[0] as u32 + average_pixel[1] as u32 + average_pixel[2] as u32
}

const EDGE_FILTER: [f32; 9] = [
    -1.0, -1.0, -1.0, //
    -1.0, 8.0, -1.0, //
    -1.0, -1.0, -1.0,
];

fn scan_position(coord: impl Into<SigCoord>) -> (u32, u32, u32, u32) {
    const SCAN_SIZE: u32 = 20;

    let (x, y) = coord_to_screen(coord);
    (x - SCAN_SIZE / 2, y - SCAN_SIZE / 2, SCAN_SIZE, SCAN_SIZE)
}
