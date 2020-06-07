use druid_icon::prelude::*;


pub const MODULATION_SOURCE: IconStaticData = IconStaticData {
  size: Size::new(23.0, 11.0),
  paths: &[
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: None,
      stroke: Some(IconPathStroke { opacity: 1.0, width: 1.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(2.5, 5.5)),
        PathEl::CurveTo(Point::new(4.5, 1.5), Point::new(5.5, 5.5), Point::new(5.5, 5.5)),
        PathEl::CurveTo(Point::new(5.5, 5.5), Point::new(6.5, 9.5), Point::new(8.5, 5.5)),
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: None,
      stroke: Some(IconPathStroke { opacity: 1.0, width: 2.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(10.0, 5.5729)),
        PathEl::LineTo(Point::new(15.1, 5.5729)),
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: Some(IconPathStroke { opacity: 1.0, width: 1.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(15.5, 9.0)),
        PathEl::LineTo(Point::new(15.5, 2.0)),
        PathEl::LineTo(Point::new(22.4089, 5.4999)),
        PathEl::ClosePath,
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: None,
      stroke: Some(IconPathStroke { opacity: 1.0, width: 1.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(10.0, 5.5)),
        PathEl::CurveTo(Point::new(10.0, 7.98528137423857), Point::new(7.985281374238571, 10.0), Point::new(5.5, 10.0)),
        PathEl::CurveTo(Point::new(3.01471862576143, 10.0), Point::new(1.0000000000000009, 7.985281374238571), Point::new(1.0, 5.500000000000001)),
        PathEl::CurveTo(Point::new(0.9999999999999991, 3.0147186257614322), Point::new(3.0147186257614287, 1.0000000000000009), Point::new(5.499999999999999, 1.0)),
        PathEl::CurveTo(Point::new(7.98528137423857, 0.9999999999999991), Point::new(10.0, 3.0147186257614296), Point::new(10.0, 5.5)),
        PathEl::ClosePath,
      ],
    },
  ],
};

pub const MODULATION_PARAM: IconStaticData = IconStaticData {
  size: Size::new(23.0, 11.0),
  paths: &[
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: None,
      stroke: Some(IconPathStroke { opacity: 1.0, width: 2.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(0.0, 5.5729)),
        PathEl::LineTo(Point::new(5.1, 5.5729)),
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: Some(IconPathStroke { opacity: 1.0, width: 1.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(5.5, 9.0)),
        PathEl::LineTo(Point::new(5.5, 2.0)),
        PathEl::LineTo(Point::new(12.4089, 5.4999)),
        PathEl::ClosePath,
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: None,
      stroke: Some(IconPathStroke { opacity: 1.0, width: 1.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(16.487, 9.738)),
        PathEl::CurveTo(Point::new(14.420699999999998, 9.00023), Point::new(13.187699999999998, 6.879899999999999), Point::new(13.568499999999998, 4.719099999999999)),
        PathEl::CurveTo(Point::new(13.949239999999998, 2.5583999999999993), Point::new(15.832699999999999, 0.9871999999999992), Point::new(18.026699999999998, 1.0000999999999993)),
        PathEl::CurveTo(Point::new(20.220699999999997, 1.0130219999999994), Point::new(22.0856, 2.606299999999999), Point::new(22.4408, 4.7714)),
        PathEl::CurveTo(Point::new(22.79606, 6.9365), Point::new(21.538149999999998, 9.042200000000001), Point::new(19.4633, 9.7555)),
        PathEl::LineTo(Point::new(18.0, 5.567399999999999)),
      ],
    },
  ],
};
