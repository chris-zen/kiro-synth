use druid_icon::prelude::*;


#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
pub const MODULATION_REMOVE: IconStaticData = IconStaticData {
  size: Size::new(10.0, 11.0),
  paths: &[
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: Some(IconPathStroke { opacity: 1.0, width: 1.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(1.0, 1.0)),
        PathEl::LineTo(Point::new(1.0, 2.0)),
        PathEl::LineTo(Point::new(8.0, 10.0)),
        PathEl::LineTo(Point::new(9.0, 10.0)),
        PathEl::LineTo(Point::new(9.0, 9.0)),
        PathEl::LineTo(Point::new(2.0, 1.0)),
        PathEl::ClosePath,
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: Some(IconPathStroke { opacity: 1.0, width: 1.0 }),
      elements: &[
        PathEl::MoveTo(Point::new(8.0, 1.0)),
        PathEl::LineTo(Point::new(9.0, 1.0)),
        PathEl::LineTo(Point::new(9.0, 2.0)),
        PathEl::LineTo(Point::new(2.0, 10.0)),
        PathEl::LineTo(Point::new(1.0, 10.0)),
        PathEl::LineTo(Point::new(1.0, 9.0)),
        PathEl::ClosePath,
      ],
    },
  ],
};

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
pub const LOGO_KIRO_SYNTH: IconStaticData = IconStaticData {
  size: Size::new(156.0, 56.0),
  paths: &[
    IconStaticPath {
      transform: Affine::new([1.3333, 0.0, 0.0, 1.3333, 0.0, 0.0000281]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: None,
      elements: &[
        PathEl::MoveTo(Point::new(1.6998, 4.6594)),
        PathEl::LineTo(Point::new(5.356, 4.6594)),
        PathEl::LineTo(Point::new(5.356, 17.8074)),
        PathEl::LineTo(Point::new(10.4537, 4.6594000000000015)),
        PathEl::LineTo(Point::new(14.1451, 4.6594000000000015)),
        PathEl::LineTo(Point::new(8.8013, 17.8074)),
        PathEl::LineTo(Point::new(14.5669, 31.799400000000002)),
        PathEl::LineTo(Point::new(10.453600000000002, 31.799400000000002)),
        PathEl::LineTo(Point::new(5.355900000000002, 17.8074)),
        PathEl::LineTo(Point::new(5.355900000000002, 31.799400000000002)),
        PathEl::LineTo(Point::new(1.6997000000000018, 31.799400000000002)),
        PathEl::ClosePath,
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.3333, 0.0, 0.0, 1.3333, 0.0, 0.0000281]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: None,
      elements: &[
        PathEl::MoveTo(Point::new(16.694, 13.607)),
        PathEl::LineTo(Point::new(20.069, 13.607)),
        PathEl::LineTo(Point::new(20.069, 31.8)),
        PathEl::LineTo(Point::new(16.694, 31.8)),
        PathEl::ClosePath,
        PathEl::MoveTo(Point::new(18.276, 5.872599999999999)),
        PathEl::CurveTo(Point::new(18.885373333333334, 5.872599999999999), Point::new(19.412706666666665, 6.095256666666667), Point::new(19.858, 6.540569999999999)),
        PathEl::CurveTo(Point::new(20.303313333333335, 6.985883333333333), Point::new(20.52597, 7.519083333333332), Point::new(20.52597, 8.14017)),
        PathEl::CurveTo(Point::new(20.52597, 8.772983333333334), Point::new(20.303313333333335, 9.31205), Point::new(19.858, 9.75737)),
        PathEl::CurveTo(Point::new(19.41268666666667, 10.202689999999999), Point::new(18.87362, 10.425346666666668), Point::new(18.2408, 10.42534)),
        PathEl::CurveTo(Point::new(17.607986666666665, 10.42534), Point::new(17.068920000000002, 10.202683333333333), Point::new(16.6236, 9.75737)),
        PathEl::CurveTo(Point::new(16.190006666666665, 9.323776666666667), Point::new(15.97321, 8.778843333333333), Point::new(15.97321, 8.12257)),
        PathEl::CurveTo(Point::new(15.97321, 7.513196666666666), Point::new(16.195866666666664, 6.9858633333333335), Point::new(16.64118, 6.54057)),
        PathEl::CurveTo(Point::new(17.121646666666667, 6.095256666666667), Point::new(17.66658, 5.872600000000001), Point::new(18.275979999999997, 5.8726)),
        PathEl::ClosePath,
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.3333, 0.0, 0.0, 1.3333, 0.0, 0.0000281]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: None,
      elements: &[
        PathEl::MoveTo(Point::new(23.866, 13.607)),
        PathEl::LineTo(Point::new(27.223399999999998, 13.607)),
        PathEl::LineTo(Point::new(27.223399999999998, 15.8746)),
        PathEl::CurveTo(Point::new(27.797619999999995, 14.913666666666666), Point::new(28.365986666666668, 14.239833333333332), Point::new(28.9285, 13.8531)),
        PathEl::CurveTo(Point::new(29.491, 13.46638), Point::new(30.235133333333334, 13.243723333333334), Point::new(31.160899999999998, 13.18513)),
        PathEl::LineTo(Point::new(31.160899999999998, 16.70073)),
        PathEl::CurveTo(Point::new(30.926526666666664, 16.66557), Point::new(30.727306666666664, 16.64799), Point::new(30.563239999999997, 16.64799)),
        PathEl::CurveTo(Point::new(29.356239999999996, 16.64799), Point::new(28.49490666666666, 16.958536666666667), Point::new(27.979239999999997, 17.57963)),
        PathEl::CurveTo(Point::new(27.475333333333328, 18.189003333333336), Point::new(27.223380000000002, 19.255403333333334), Point::new(27.22338, 20.778830000000003)),
        PathEl::LineTo(Point::new(27.22338, 31.799830000000004)),
        PathEl::LineTo(Point::new(23.86598, 31.799830000000004)),
        PathEl::ClosePath,
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.3333, 0.0, 0.0, 1.3333, 0.0, 0.0000281]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: None,
      elements: &[
        PathEl::MoveTo(Point::new(44.38, 22.853)),
        PathEl::CurveTo(Point::new(44.38, 29.00533333333334), Point::new(42.4757, 32.081500000000005), Point::new(38.667100000000005, 32.081500000000005)),
        PathEl::CurveTo(Point::new(34.85850000000001, 32.081500000000005), Point::new(32.95420000000001, 28.95846666666667), Point::new(32.95420000000001, 22.712400000000006)),
        PathEl::CurveTo(Point::new(32.95420000000001, 16.442866666666674), Point::new(34.85263333333334, 13.308100000000005), Point::new(38.6495, 13.308100000000007)),
        PathEl::CurveTo(Point::new(39.633900000000004, 13.308100000000005), Point::new(40.4835, 13.495600000000005), Point::new(41.1983, 13.870600000000007)),
        PathEl::CurveTo(Point::new(41.92483333333333, 14.245600000000005), Point::new(42.5225, 14.825666666666672), Point::new(42.9913, 15.610800000000006)),
        PathEl::CurveTo(Point::new(43.460100000000004, 16.39593333333334), Point::new(43.805800000000005, 17.38616666666667), Point::new(44.028400000000005, 18.581500000000005)),
        PathEl::CurveTo(Point::new(44.262773333333335, 19.77683333333334), Point::new(44.379960000000004, 21.200666666666674), Point::new(44.379960000000004, 22.853000000000005)),
        PathEl::ClosePath,
        PathEl::MoveTo(Point::new(40.8995, 22.76511)),
        PathEl::CurveTo(Point::new(40.8995, 20.37451), Point::new(40.74129666666667, 18.710443333333334), Point::new(40.424890000000005, 17.77291)),
        PathEl::CurveTo(Point::new(40.10848333333334, 16.823710000000002), Point::new(39.54598333333334, 16.34911), Point::new(38.737390000000005, 16.34911)),
        PathEl::CurveTo(Point::new(37.94052333333334, 16.34911), Point::new(37.37215666666667, 16.817843333333332), Point::new(37.03229, 17.755309999999998)),
        PathEl::CurveTo(Point::new(36.71588333333334, 18.692776666666663), Point::new(36.557680000000005, 20.327543333333328), Point::new(36.557680000000005, 22.659609999999997)),
        PathEl::CurveTo(Point::new(36.557680000000005, 24.991676666666663), Point::new(36.71588333333334, 26.620576666666665), Point::new(37.03229, 27.54631)),
        PathEl::CurveTo(Point::new(37.37213666666667, 28.48377666666666), Point::new(37.94050333333334, 28.95251), Point::new(38.737390000000005, 28.952509999999997)),
        PathEl::CurveTo(Point::new(39.53425666666667, 28.95251), Point::new(40.09089000000001, 28.48961), Point::new(40.40729, 27.563809999999997)),
        PathEl::CurveTo(Point::new(40.73541666666667, 26.696609999999996), Point::new(40.899480000000004, 25.097009999999997), Point::new(40.899480000000004, 22.765009999999997)),
        PathEl::ClosePath,
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.3333, 0.0, 0.0, 1.3333, 0.0, 0.0000281]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: None,
      elements: &[
        PathEl::MoveTo(Point::new(64.19, 5.2219)),
        PathEl::LineTo(Point::new(64.19, 8.7903)),
        PathEl::CurveTo(Point::new(63.07673333333333, 7.993433333333333), Point::new(62.07476666666667, 7.5950000000000015), Point::new(61.1841, 7.595000000000001)),
        PathEl::CurveTo(Point::new(60.223166666666664, 7.5950000000000015), Point::new(59.437999999999995, 7.9348333333333345), Point::new(58.8286, 8.614500000000001)),
        PathEl::CurveTo(Point::new(58.219226666666664, 9.259033333333335), Point::new(57.91454000000001, 10.120366666666667), Point::new(57.91454, 11.198500000000001)),
        PathEl::CurveTo(Point::new(57.91454000000001, 12.1477), Point::new(58.13133666666667, 12.962166666666668), Point::new(58.564930000000004, 13.641900000000001)),
        PathEl::CurveTo(Point::new(58.78758333333334, 14.005180000000001), Point::new(59.15085, 14.473913333333334), Point::new(59.65473, 15.048100000000002)),
        PathEl::CurveTo(Point::new(60.17035666666667, 15.622320000000002), Point::new(60.832456666666666, 16.313720000000004), Point::new(61.64103, 17.122300000000003)),
        PathEl::CurveTo(Point::new(63.12929666666667, 18.63403333333334), Point::new(64.14296333333333, 19.94066666666667), Point::new(64.68203, 21.0422)),
        PathEl::CurveTo(Point::new(65.22108999999999, 22.1086), Point::new(65.49061999999999, 23.415233333333333), Point::new(65.49061999999999, 24.9621)),
        PathEl::CurveTo(Point::new(65.49061999999999, 27.059766666666665), Point::new(64.89881999999999, 28.78826666666667), Point::new(63.715219999999995, 30.1476)),
        PathEl::CurveTo(Point::new(62.53162, 31.471799999999998), Point::new(61.01405333333333, 32.1339), Point::new(59.162519999999994, 32.1339)),
        PathEl::CurveTo(Point::new(57.603919999999995, 32.1339), Point::new(56.27385333333333, 31.70616666666666), Point::new(55.17231999999999, 30.850699999999996)),
        PathEl::LineTo(Point::new(55.17231999999999, 27.229599999999998)),
        PathEl::CurveTo(Point::new(56.473119999999994, 28.178799999999995), Point::new(57.656719999999986, 28.653399999999994), Point::new(58.723119999999994, 28.653399999999998)),
        PathEl::CurveTo(Point::new(59.719186666666666, 28.653399999999994), Point::new(60.49848666666666, 28.32527333333333), Point::new(61.06101999999999, 27.669019999999996)),
        PathEl::CurveTo(Point::new(61.62351999999999, 27.001019999999997), Point::new(61.90476999999999, 26.098686666666662), Point::new(61.90476999999999, 24.962019999999995)),
        PathEl::CurveTo(Point::new(61.90476999999999, 23.97761999999999), Point::new(61.687973333333325, 23.092853333333327), Point::new(61.25437999999999, 22.307719999999996)),
        PathEl::CurveTo(Point::new(61.03172666666666, 21.93272), Point::new(60.72118, 21.504986666666664), Point::new(60.32273999999999, 21.024519999999995)),
        PathEl::CurveTo(Point::new(59.92429999999999, 20.53233333333333), Point::new(59.426266666666656, 19.987399999999997), Point::new(58.828639999999986, 19.389719999999997)),
        PathEl::CurveTo(Point::new(57.90283999999999, 18.47565333333333), Point::new(57.135273333333316, 17.66705333333333), Point::new(56.525939999999984, 16.963919999999998)),
        PathEl::CurveTo(Point::new(55.916566666666654, 16.260786666666665), Point::new(55.46539999999999, 15.64555333333333), Point::new(55.17243999999999, 15.118219999999997)),
        PathEl::CurveTo(Point::new(54.60993999999999, 14.086953333333332), Point::new(54.32868999999999, 12.786186666666664), Point::new(54.32868999999999, 11.215919999999997)),
        PathEl::CurveTo(Point::new(54.32868999999999, 9.094853333333331), Point::new(54.87945666666665, 7.413219999999998), Point::new(55.980989999999984, 6.171019999999997)),
        PathEl::CurveTo(Point::new(57.094256666666645, 4.9170866666666635), Point::new(58.58838999999998, 4.290119999999996), Point::new(60.46338999999998, 4.290119999999996)),
        PathEl::CurveTo(Point::new(61.764189999999985, 4.290119999999996), Point::new(63.00638999999998, 4.600666666666663), Point::new(64.18998999999998, 5.221759999999996)),
        PathEl::ClosePath,
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.3333, 0.0, 0.0, 1.3333, 0.0, 0.0000281]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: None,
      elements: &[
        PathEl::MoveTo(Point::new(66.616, 13.607)),
        PathEl::LineTo(Point::new(70.4129, 13.607)),
        PathEl::LineTo(Point::new(73.08479999999999, 26.298000000000002)),
        PathEl::LineTo(Point::new(76.21369999999999, 13.607000000000001)),
        PathEl::LineTo(Point::new(79.62389999999999, 13.607000000000001)),
        PathEl::LineTo(Point::new(72.18839999999999, 39.253)),
        PathEl::LineTo(Point::new(68.6552, 39.253)),
        PathEl::LineTo(Point::new(71.45009999999999, 30.340899999999998)),
        PathEl::ClosePath,
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.3333, 0.0, 0.0, 1.3333, 0.0, 0.0000281]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: None,
      elements: &[
        PathEl::MoveTo(Point::new(81.663, 13.607)),
        PathEl::LineTo(Point::new(85.0204, 13.607)),
        PathEl::LineTo(Point::new(85.0204, 15.698799999999999)),
        PathEl::CurveTo(Point::new(85.98133333333332, 14.105066666666666), Point::new(87.22353333333332, 13.3082), Point::new(88.747, 13.3082)),
        PathEl::CurveTo(Point::new(90.22353333333332, 13.3082), Point::new(91.27823333333333, 13.888266666666667), Point::new(91.9111, 15.048399999999999)),
        PathEl::CurveTo(Point::new(92.26266, 15.704653333333333), Point::new(92.43844, 16.976153333333333), Point::new(92.43844, 18.8629)),
        PathEl::LineTo(Point::new(92.43844, 31.8009)),
        PathEl::LineTo(Point::new(89.08104, 31.8009)),
        PathEl::LineTo(Point::new(89.06346, 19.636899999999997)),
        PathEl::CurveTo(Point::new(89.06346, 18.40643333333333), Point::new(88.91697666666668, 17.533399999999997), Point::new(88.62401000000001, 17.017799999999998)),
        PathEl::CurveTo(Point::new(88.33104333333335, 16.50217333333333), Point::new(87.82714333333335, 16.244359999999997), Point::new(87.11231000000001, 16.244359999999997)),
        PathEl::CurveTo(Point::new(86.72559000000001, 16.244359999999997), Point::new(86.39746333333335, 16.31467333333333), Point::new(86.12793, 16.455299999999998)),
        PathEl::CurveTo(Point::new(85.87011666666668, 16.584206666666663), Point::new(85.65918, 16.80100333333333), Point::new(85.49512, 17.10569)),
        PathEl::CurveTo(Point::new(85.33106, 17.398656666666668), Point::new(85.20801333333334, 17.78539), Point::new(85.12598, 18.26589)),
        PathEl::CurveTo(Point::new(85.05566666666667, 18.746356666666667), Point::new(85.02051, 19.338156666666666), Point::new(85.02051, 20.04129)),
        PathEl::LineTo(Point::new(85.02051, 31.80129)),
        PathEl::LineTo(Point::new(81.66311, 31.80129)),
        PathEl::ClosePath,
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.3333, 0.0, 0.0, 1.3333, 0.0, 0.0000281]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: None,
      elements: &[
        PathEl::MoveTo(Point::new(96.622, 8.509)),
        PathEl::LineTo(Point::new(99.9794, 8.509)),
        PathEl::LineTo(Point::new(99.9794, 13.6067)),
        PathEl::LineTo(Point::new(101.8075, 13.6067)),
        PathEl::LineTo(Point::new(101.8075, 16.2786)),
        PathEl::LineTo(Point::new(99.9794, 16.2786)),
        PathEl::LineTo(Point::new(99.9794, 31.7996)),
        PathEl::LineTo(Point::new(96.622, 31.7996)),
        PathEl::LineTo(Point::new(96.622, 16.2786)),
        PathEl::LineTo(Point::new(94.7763, 16.2786)),
        PathEl::LineTo(Point::new(94.7763, 13.6067)),
        PathEl::LineTo(Point::new(96.622, 13.6067)),
        PathEl::ClosePath,
      ],
    },
    IconStaticPath {
      transform: Affine::new([1.3333, 0.0, 0.0, 1.3333, 0.0, 0.0000281]),
      fill: Some(IconPathFill { opacity: 1.0 }),
      stroke: None,
      elements: &[
        PathEl::MoveTo(Point::new(104.23, 1.9348)),
        PathEl::LineTo(Point::new(107.5874, 1.9348)),
        PathEl::LineTo(Point::new(107.5874, 15.698799999999999)),
        PathEl::CurveTo(Point::new(108.54833333333333, 14.105066666666666), Point::new(109.79053333333333, 13.3082), Point::new(111.31400000000001, 13.3082)),
        PathEl::CurveTo(Point::new(112.79053333333333, 13.3082), Point::new(113.84523333333334, 13.888266666666667), Point::new(114.47810000000001, 15.048399999999999)),
        PathEl::CurveTo(Point::new(114.82966, 15.704653333333333), Point::new(115.00544000000001, 16.976153333333333), Point::new(115.00544000000001, 18.8629)),
        PathEl::LineTo(Point::new(115.00544000000001, 31.8009)),
        PathEl::LineTo(Point::new(111.64804000000001, 31.8009)),
        PathEl::LineTo(Point::new(111.63044000000001, 19.636899999999997)),
        PathEl::CurveTo(Point::new(111.63044000000001, 18.40643333333333), Point::new(111.48395666666669, 17.533399999999997), Point::new(111.19099000000001, 17.017799999999998)),
        PathEl::CurveTo(Point::new(110.89802333333336, 16.50217333333333), Point::new(110.39412333333335, 16.244359999999997), Point::new(109.67929000000001, 16.244359999999997)),
        PathEl::CurveTo(Point::new(109.29257000000001, 16.244359999999997), Point::new(108.96444333333335, 16.31467333333333), Point::new(108.69491000000001, 16.455299999999998)),
        PathEl::CurveTo(Point::new(108.43710333333335, 16.584206666666663), Point::new(108.22616666666669, 16.80100333333333), Point::new(108.0621, 17.10569)),
        PathEl::CurveTo(Point::new(107.89804, 17.398656666666668), Point::new(107.77499333333333, 17.78539), Point::new(107.69296, 18.26589)),
        PathEl::CurveTo(Point::new(107.62264666666665, 18.746356666666667), Point::new(107.58749, 19.338156666666666), Point::new(107.58749, 20.04129)),
        PathEl::LineTo(Point::new(107.58749, 31.80129)),
        PathEl::LineTo(Point::new(104.23009, 31.80129)),
        PathEl::ClosePath,
      ],
    },
  ],
};

#[allow(unused)]
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
