use druid_icon::prelude::*;


pub const MODULATION_SOURCE: IconStaticData = IconStaticData {
  size: Size::new(11.0, 11.0),
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

pub const MODULATION_ARROW: IconStaticData = IconStaticData {
  size: Size::new(10.0, 11.0),
  paths: &[
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: None,
      elements: &[
        PathEl::MoveTo(Point::new(0.5, 1.2625)),
        PathEl::LineTo(Point::new(0.5, 9.737499999999999)),
        PathEl::CurveTo(Point::new(0.50042, 10.157039999999999), Point::new(0.9506399999999999, 10.428529999999999), Point::new(1.33081, 10.23848)),
        PathEl::LineTo(Point::new(8.73021, 6.000979999999999)),
        PathEl::CurveTo(Point::new(9.14578, 5.79212), Point::new(9.14578, 5.207839999999999), Point::new(8.73021, 4.9989799999999995)),
        PathEl::LineTo(Point::new(1.3308099999999996, 0.7614799999999997)),
        PathEl::CurveTo(Point::new(1.0173199999999996, 0.5819499999999997), Point::new(0.49999999999999956, 0.8406029999999998), Point::new(0.49999999999999956, 1.2624599999999997)),
        PathEl::ClosePath,
      ],
    },
  ],
};

pub const MODULATION_PARAM: IconStaticData = IconStaticData {
  size: Size::new(11.0, 11.0),
  paths: &[
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: None,
      stroke: Some(IconPathStroke { opacity: 1.0, width: 1.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(3.9868, 9.8602)),
        PathEl::CurveTo(Point::new(1.9205, 9.122430000000001), Point::new(0.6875, 7.0021), Point::new(1.0683000000000002, 4.8413)),
        PathEl::CurveTo(Point::new(1.4490400000000003, 2.6806000000000005), Point::new(3.3325000000000005, 1.1094000000000004), Point::new(5.5265, 1.1223000000000005)),
        PathEl::CurveTo(Point::new(7.7205, 1.1352220000000006), Point::new(9.5854, 2.7285000000000004), Point::new(9.9406, 4.893600000000001)),
        PathEl::CurveTo(Point::new(10.29586, 7.058700000000001), Point::new(9.03795, 9.1644), Point::new(6.9631, 9.8777)),
        PathEl::LineTo(Point::new(5.4998, 5.6896)),
      ],
    },
  ],
};

pub const MODULATION_NEW: IconStaticData = IconStaticData {
  size: Size::new(11.0, 11.0),
  paths: &[
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: None,
      stroke: Some(IconPathStroke { opacity: 1.0, width: 2.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(0.5, 5.5)),
        PathEl::LineTo(Point::new(10.5, 5.5)),
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: None,
      stroke: Some(IconPathStroke { opacity: 1.0, width: 2.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(5.5, 0.5)),
        PathEl::LineTo(Point::new(5.5, 10.5)),
      ],
    },
  ],
};

pub const MODULATION_SOURCES: IconStaticData = IconStaticData {
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
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, -2.0, 0.0]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: None,
      elements: &[
        PathEl::MoveTo(Point::new(15.5, 1.2625)),
        PathEl::LineTo(Point::new(15.5, 9.737499999999999)),
        PathEl::CurveTo(Point::new(15.50042, 10.157039999999999), Point::new(15.95064, 10.428529999999999), Point::new(16.33081, 10.23848)),
        PathEl::LineTo(Point::new(23.73021, 6.000979999999999)),
        PathEl::CurveTo(Point::new(24.14578, 5.79212), Point::new(24.14578, 5.207839999999999), Point::new(23.73021, 4.9989799999999995)),
        PathEl::LineTo(Point::new(16.33081, 0.7614799999999997)),
        PathEl::CurveTo(Point::new(16.017319999999998, 0.5819499999999997), Point::new(15.5, 0.8406029999999998), Point::new(15.5, 1.2624599999999997)),
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

pub const MODULATION_PARAMS: IconStaticData = IconStaticData {
  size: Size::new(23.0, 11.0),
  paths: &[
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: None,
      stroke: Some(IconPathStroke { opacity: 1.0, width: 1.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(15.487, 9.738)),
        PathEl::CurveTo(Point::new(13.4207, 9.00023), Point::new(12.1877, 6.879899999999999), Point::new(12.5685, 4.719099999999999)),
        PathEl::CurveTo(Point::new(12.94924, 2.5583999999999993), Point::new(14.8327, 0.9871999999999992), Point::new(17.026699999999998, 1.0000999999999993)),
        PathEl::CurveTo(Point::new(19.220699999999997, 1.0130219999999994), Point::new(21.0856, 2.606299999999999), Point::new(21.4408, 4.7714)),
        PathEl::CurveTo(Point::new(21.79606, 6.9365), Point::new(20.538149999999998, 9.042200000000001), Point::new(18.4633, 9.7555)),
        PathEl::LineTo(Point::new(17.0, 5.567399999999999)),
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: None,
      elements: &[
        PathEl::MoveTo(Point::new(0.5, 1.2625)),
        PathEl::LineTo(Point::new(0.5, 9.737499999999999)),
        PathEl::CurveTo(Point::new(0.50042, 10.157039999999999), Point::new(0.9506399999999999, 10.428529999999999), Point::new(1.33081, 10.23848)),
        PathEl::LineTo(Point::new(8.73021, 6.000979999999999)),
        PathEl::CurveTo(Point::new(9.14578, 5.79212), Point::new(9.14578, 5.207839999999999), Point::new(8.73021, 4.9989799999999995)),
        PathEl::LineTo(Point::new(1.3308099999999996, 0.7614799999999997)),
        PathEl::CurveTo(Point::new(1.0173199999999996, 0.5819499999999997), Point::new(0.49999999999999956, 0.8406029999999998), Point::new(0.49999999999999956, 1.2624599999999997)),
        PathEl::ClosePath,
      ],
    },
  ],
};
