use std::borrow::Cow;

use crate::{Color, NamedColor, ThemeArray};

// Auto-generated file. Do not edit.

pub struct Solarized {}

impl Solarized {
    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BASE_BLUE_50: Color = Color::Rgb(::palette::Srgb::new(0.8971, 0.9649, 1.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BASE_BLUE_100: Color = Color::Rgb(::palette::Srgb::new(0.7581, 0.9252, 0.9990));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BASE_BLUE_200: Color = Color::Rgb(::palette::Srgb::new(0.4027, 0.8550, 1.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BASE_BLUE_300: Color = Color::Rgb(::palette::Srgb::new(0.0000, 0.7648, 0.9257));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BASE_BLUE_400: Color = Color::Rgb(::palette::Srgb::new(0.0112, 0.6626, 0.7994));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BASE_BLUE_500: Color = Color::Rgb(::palette::Srgb::new(0.0000, 0.5766, 0.6943));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BASE_BLUE_600: Color = Color::Rgb(::palette::Srgb::new(0.0054, 0.4783, 0.5799));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BASE_BLUE_700: Color = Color::Rgb(::palette::Srgb::new(0.0000, 0.3844, 0.4710));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BASE_BLUE_800: Color = Color::Rgb(::palette::Srgb::new(0.0028, 0.2940, 0.3603));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BASE_BLUE_900: Color = Color::Rgb(::palette::Srgb::new(0.0022, 0.2078, 0.2583));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BASE_BLUE_950: Color = Color::Rgb(::palette::Srgb::new(0.0000, 0.1687, 0.2122));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_50: Color = Color::Rgb(::palette::Srgb::new(0.9137, 0.9490, 0.9646));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_100: Color = Color::Rgb(::palette::Srgb::new(0.8428, 0.9099, 0.9335));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_200: Color = Color::Rgb(::palette::Srgb::new(0.6758, 0.8232, 0.8696));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_300: Color = Color::Rgb(::palette::Srgb::new(0.5764, 0.7137, 0.7568));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_400: Color = Color::Rgb(::palette::Srgb::new(0.4994, 0.6155, 0.6541));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_500: Color = Color::Rgb(::palette::Srgb::new(0.4190, 0.5217, 0.5572));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_600: Color = Color::Rgb(::palette::Srgb::new(0.3459, 0.4312, 0.4583));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_700: Color = Color::Rgb(::palette::Srgb::new(0.2581, 0.3256, 0.3454));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_800: Color = Color::Rgb(::palette::Srgb::new(0.1639, 0.2159, 0.2318));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_900: Color = Color::Rgb(::palette::Srgb::new(0.0907, 0.1215, 0.1292));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_950: Color = Color::Rgb(::palette::Srgb::new(0.0550, 0.0784, 0.0862));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_50: Color = Color::Rgb(::palette::Srgb::new(0.9647, 0.9530, 0.9218));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_100: Color = Color::Rgb(::palette::Srgb::new(0.9333, 0.9098, 0.8354));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_200: Color = Color::Rgb(::palette::Srgb::new(0.8313, 0.8000, 0.6986));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_300: Color = Color::Rgb(::palette::Srgb::new(0.7181, 0.6902, 0.6026));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_400: Color = Color::Rgb(::palette::Srgb::new(0.5999, 0.5764, 0.5023));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_500: Color = Color::Rgb(::palette::Srgb::new(0.4940, 0.4745, 0.4160));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_600: Color = Color::Rgb(::palette::Srgb::new(0.3963, 0.3803, 0.3286));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_700: Color = Color::Rgb(::palette::Srgb::new(0.2983, 0.2863, 0.2463));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_800: Color = Color::Rgb(::palette::Srgb::new(0.2001, 0.1882, 0.1605));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_900: Color = Color::Rgb(::palette::Srgb::new(0.1136, 0.1059, 0.0906));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_950: Color = Color::Rgb(::palette::Srgb::new(0.0706, 0.0667, 0.0512));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_50: Color = Color::Rgb(::palette::Srgb::new(0.9995, 0.9334, 0.8518));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_100: Color = Color::Rgb(::palette::Srgb::new(0.9999, 0.8706, 0.6709));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_200: Color = Color::Rgb(::palette::Srgb::new(0.9765, 0.7412, 0.0020));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_300: Color = Color::Rgb(::palette::Srgb::new(0.8428, 0.6393, 0.0073));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_400: Color = Color::Rgb(::palette::Srgb::new(0.7099, 0.5373, 0.0004));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_500: Color = Color::Rgb(::palette::Srgb::new(0.5886, 0.4431, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_600: Color = Color::Rgb(::palette::Srgb::new(0.4702, 0.3530, 0.0036));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_700: Color = Color::Rgb(::palette::Srgb::new(0.3573, 0.2626, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_800: Color = Color::Rgb(::palette::Srgb::new(0.2472, 0.1804, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_900: Color = Color::Rgb(::palette::Srgb::new(0.1451, 0.1020, 0.0002));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_950: Color = Color::Rgb(::palette::Srgb::new(0.0945, 0.0626, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_50: Color = Color::Rgb(::palette::Srgb::new(0.9968, 0.9291, 0.9211));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_100: Color = Color::Rgb(::palette::Srgb::new(0.9967, 0.8585, 0.8387));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_200: Color = Color::Rgb(::palette::Srgb::new(0.9922, 0.7098, 0.6666));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_300: Color = Color::Rgb(::palette::Srgb::new(0.9889, 0.5487, 0.4582));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_400: Color = Color::Rgb(::palette::Srgb::new(0.9573, 0.3604, 0.1124));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_500: Color = Color::Rgb(::palette::Srgb::new(0.7957, 0.2945, 0.0875));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_600: Color = Color::Rgb(::palette::Srgb::new(0.6319, 0.2268, 0.0569));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_700: Color = Color::Rgb(::palette::Srgb::new(0.4935, 0.1693, 0.0373));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_800: Color = Color::Rgb(::palette::Srgb::new(0.3494, 0.1096, 0.0151));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_900: Color = Color::Rgb(::palette::Srgb::new(0.2152, 0.0555, 0.0085));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_950: Color = Color::Rgb(::palette::Srgb::new(0.1457, 0.0269, 0.0034));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_50: Color = Color::Rgb(::palette::Srgb::new(0.9913, 0.9298, 0.9298));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_100: Color = Color::Rgb(::palette::Srgb::new(0.9793, 0.8594, 0.8593));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_200: Color = Color::Rgb(::palette::Srgb::new(0.9655, 0.7134, 0.7133));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_300: Color = Color::Rgb(::palette::Srgb::new(0.9531, 0.5450, 0.5410));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_400: Color = Color::Rgb(::palette::Srgb::new(0.9447, 0.3613, 0.3533));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_500: Color = Color::Rgb(::palette::Srgb::new(0.8621, 0.1975, 0.1853));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_600: Color = Color::Rgb(::palette::Srgb::new(0.6865, 0.1485, 0.1368));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_700: Color = Color::Rgb(::palette::Srgb::new(0.5222, 0.1006, 0.0931));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_800: Color = Color::Rgb(::palette::Srgb::new(0.3805, 0.0627, 0.0588));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_900: Color = Color::Rgb(::palette::Srgb::new(0.2356, 0.0270, 0.0232));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_950: Color = Color::Rgb(::palette::Srgb::new(0.1605, 0.0121, 0.0120));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_50: Color = Color::Rgb(::palette::Srgb::new(0.9872, 0.9298, 0.9488));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_100: Color = Color::Rgb(::palette::Srgb::new(0.9802, 0.8551, 0.8980));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_200: Color = Color::Rgb(::palette::Srgb::new(0.9608, 0.7098, 0.7960));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_300: Color = Color::Rgb(::palette::Srgb::new(0.9452, 0.5489, 0.7019));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_400: Color = Color::Rgb(::palette::Srgb::new(0.9337, 0.3565, 0.6117));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_500: Color = Color::Rgb(::palette::Srgb::new(0.8269, 0.2131, 0.5098));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_600: Color = Color::Rgb(::palette::Srgb::new(0.6708, 0.1644, 0.4079));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_700: Color = Color::Rgb(::palette::Srgb::new(0.5097, 0.1178, 0.3058));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_800: Color = Color::Rgb(::palette::Srgb::new(0.3608, 0.0706, 0.2118));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_900: Color = Color::Rgb(::palette::Srgb::new(0.2238, 0.0308, 0.1215));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_950: Color = Color::Rgb(::palette::Srgb::new(0.1527, 0.0159, 0.0784));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const VIOLET_50: Color = Color::Rgb(::palette::Srgb::new(0.9412, 0.9411, 0.9764));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const VIOLET_100: Color = Color::Rgb(::palette::Srgb::new(0.8786, 0.8824, 0.9518));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const VIOLET_200: Color = Color::Rgb(::palette::Srgb::new(0.7608, 0.7686, 0.9063));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const VIOLET_300: Color = Color::Rgb(::palette::Srgb::new(0.6432, 0.6549, 0.8621));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const VIOLET_400: Color = Color::Rgb(::palette::Srgb::new(0.5413, 0.5570, 0.8226));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const VIOLET_500: Color = Color::Rgb(::palette::Srgb::new(0.4236, 0.4432, 0.7683));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const VIOLET_600: Color = Color::Rgb(::palette::Srgb::new(0.3176, 0.3411, 0.7061));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const VIOLET_700: Color = Color::Rgb(::palette::Srgb::new(0.2196, 0.2432, 0.5526));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const VIOLET_800: Color = Color::Rgb(::palette::Srgb::new(0.1490, 0.1646, 0.3927));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const VIOLET_900: Color = Color::Rgb(::palette::Srgb::new(0.0783, 0.0899, 0.2439));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const VIOLET_950: Color = Color::Rgb(::palette::Srgb::new(0.0471, 0.0549, 0.1646));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_50: Color = Color::Rgb(::palette::Srgb::new(0.9300, 0.9568, 0.9951));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_100: Color = Color::Rgb(::palette::Srgb::new(0.8425, 0.9060, 0.9933));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_200: Color = Color::Rgb(::palette::Srgb::new(0.6708, 0.8118, 0.9880));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_300: Color = Color::Rgb(::palette::Srgb::new(0.4676, 0.7215, 0.9794));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_400: Color = Color::Rgb(::palette::Srgb::new(0.1808, 0.6353, 0.9526));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_500: Color = Color::Rgb(::palette::Srgb::new(0.1512, 0.5451, 0.8227));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_600: Color = Color::Rgb(::palette::Srgb::new(0.1093, 0.4275, 0.6512));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_700: Color = Color::Rgb(::palette::Srgb::new(0.0715, 0.3177, 0.4899));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_800: Color = Color::Rgb(::palette::Srgb::new(0.0368, 0.2156, 0.3420));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_900: Color = Color::Rgb(::palette::Srgb::new(0.0112, 0.1215, 0.2042));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_950: Color = Color::Rgb(::palette::Srgb::new(0.0083, 0.0823, 0.1446));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_50: Color = Color::Rgb(::palette::Srgb::new(0.7219, 0.9960, 0.9645));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_100: Color = Color::Rgb(::palette::Srgb::new(0.2789, 0.9842, 0.9293));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_200: Color = Color::Rgb(::palette::Srgb::new(0.2347, 0.8631, 0.8158));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_300: Color = Color::Rgb(::palette::Srgb::new(0.2041, 0.7447, 0.7017));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_400: Color = Color::Rgb(::palette::Srgb::new(0.1640, 0.6315, 0.5961));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_500: Color = Color::Rgb(::palette::Srgb::new(0.1287, 0.5178, 0.4902));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_600: Color = Color::Rgb(::palette::Srgb::new(0.0929, 0.4118, 0.3882));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_700: Color = Color::Rgb(::palette::Srgb::new(0.0660, 0.3096, 0.2900));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_800: Color = Color::Rgb(::palette::Srgb::new(0.0291, 0.2158, 0.2001));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_900: Color = Color::Rgb(::palette::Srgb::new(0.0130, 0.1253, 0.1136));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_950: Color = Color::Rgb(::palette::Srgb::new(0.0048, 0.0783, 0.0705));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_50: Color = Color::Rgb(::palette::Srgb::new(0.8786, 1.0000, 0.2050));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_100: Color = Color::Rgb(::palette::Srgb::new(0.8235, 0.9412, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_200: Color = Color::Rgb(::palette::Srgb::new(0.7217, 0.8274, 0.0132));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_300: Color = Color::Rgb(::palette::Srgb::new(0.6196, 0.7137, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_400: Color = Color::Rgb(::palette::Srgb::new(0.5216, 0.6000, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_500: Color = Color::Rgb(::palette::Srgb::new(0.4313, 0.4981, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_600: Color = Color::Rgb(::palette::Srgb::new(0.3412, 0.3961, 0.0004));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_700: Color = Color::Rgb(::palette::Srgb::new(0.2549, 0.2980, 0.0006));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_800: Color = Color::Rgb(::palette::Srgb::new(0.1725, 0.2040, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_900: Color = Color::Rgb(::palette::Srgb::new(0.0980, 0.1176, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_950: Color = Color::Rgb(::palette::Srgb::new(0.0588, 0.0744, 0.0003));

    pub const BASE_BLUE: ThemeArray<11> = ThemeArray([
        Self::BASE_BLUE_50,
        Self::BASE_BLUE_100,
        Self::BASE_BLUE_200,
        Self::BASE_BLUE_300,
        Self::BASE_BLUE_400,
        Self::BASE_BLUE_500,
        Self::BASE_BLUE_600,
        Self::BASE_BLUE_700,
        Self::BASE_BLUE_800,
        Self::BASE_BLUE_900,
        Self::BASE_BLUE_950,
    ]);

    pub const GRAY: ThemeArray<11> = ThemeArray([
        Self::GRAY_50,
        Self::GRAY_100,
        Self::GRAY_200,
        Self::GRAY_300,
        Self::GRAY_400,
        Self::GRAY_500,
        Self::GRAY_600,
        Self::GRAY_700,
        Self::GRAY_800,
        Self::GRAY_900,
        Self::GRAY_950,
    ]);

    pub const BROWN: ThemeArray<11> = ThemeArray([
        Self::BROWN_50,
        Self::BROWN_100,
        Self::BROWN_200,
        Self::BROWN_300,
        Self::BROWN_400,
        Self::BROWN_500,
        Self::BROWN_600,
        Self::BROWN_700,
        Self::BROWN_800,
        Self::BROWN_900,
        Self::BROWN_950,
    ]);

    pub const YELLOW: ThemeArray<11> = ThemeArray([
        Self::YELLOW_50,
        Self::YELLOW_100,
        Self::YELLOW_200,
        Self::YELLOW_300,
        Self::YELLOW_400,
        Self::YELLOW_500,
        Self::YELLOW_600,
        Self::YELLOW_700,
        Self::YELLOW_800,
        Self::YELLOW_900,
        Self::YELLOW_950,
    ]);

    pub const ORANGE: ThemeArray<11> = ThemeArray([
        Self::ORANGE_50,
        Self::ORANGE_100,
        Self::ORANGE_200,
        Self::ORANGE_300,
        Self::ORANGE_400,
        Self::ORANGE_500,
        Self::ORANGE_600,
        Self::ORANGE_700,
        Self::ORANGE_800,
        Self::ORANGE_900,
        Self::ORANGE_950,
    ]);

    pub const RED: ThemeArray<11> = ThemeArray([
        Self::RED_50,
        Self::RED_100,
        Self::RED_200,
        Self::RED_300,
        Self::RED_400,
        Self::RED_500,
        Self::RED_600,
        Self::RED_700,
        Self::RED_800,
        Self::RED_900,
        Self::RED_950,
    ]);

    pub const MAGENTA: ThemeArray<11> = ThemeArray([
        Self::MAGENTA_50,
        Self::MAGENTA_100,
        Self::MAGENTA_200,
        Self::MAGENTA_300,
        Self::MAGENTA_400,
        Self::MAGENTA_500,
        Self::MAGENTA_600,
        Self::MAGENTA_700,
        Self::MAGENTA_800,
        Self::MAGENTA_900,
        Self::MAGENTA_950,
    ]);

    pub const VIOLET: ThemeArray<11> = ThemeArray([
        Self::VIOLET_50,
        Self::VIOLET_100,
        Self::VIOLET_200,
        Self::VIOLET_300,
        Self::VIOLET_400,
        Self::VIOLET_500,
        Self::VIOLET_600,
        Self::VIOLET_700,
        Self::VIOLET_800,
        Self::VIOLET_900,
        Self::VIOLET_950,
    ]);

    pub const BLUE: ThemeArray<11> = ThemeArray([
        Self::BLUE_50,
        Self::BLUE_100,
        Self::BLUE_200,
        Self::BLUE_300,
        Self::BLUE_400,
        Self::BLUE_500,
        Self::BLUE_600,
        Self::BLUE_700,
        Self::BLUE_800,
        Self::BLUE_900,
        Self::BLUE_950,
    ]);

    pub const CYAN: ThemeArray<11> = ThemeArray([
        Self::CYAN_50,
        Self::CYAN_100,
        Self::CYAN_200,
        Self::CYAN_300,
        Self::CYAN_400,
        Self::CYAN_500,
        Self::CYAN_600,
        Self::CYAN_700,
        Self::CYAN_800,
        Self::CYAN_900,
        Self::CYAN_950,
    ]);

    pub const GREEN: ThemeArray<11> = ThemeArray([
        Self::GREEN_50,
        Self::GREEN_100,
        Self::GREEN_200,
        Self::GREEN_300,
        Self::GREEN_400,
        Self::GREEN_500,
        Self::GREEN_600,
        Self::GREEN_700,
        Self::GREEN_800,
        Self::GREEN_900,
        Self::GREEN_950,
    ]);

    pub const ALL_COLORS: [NamedColor<'_>; 121] = [
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("base_blue"),
            color: Self::BASE_BLUE_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("base_blue"),
            color: Self::BASE_BLUE_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("base_blue"),
            color: Self::BASE_BLUE_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("base_blue"),
            color: Self::BASE_BLUE_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("base_blue"),
            color: Self::BASE_BLUE_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("base_blue"),
            color: Self::BASE_BLUE_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("base_blue"),
            color: Self::BASE_BLUE_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("base_blue"),
            color: Self::BASE_BLUE_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("base_blue"),
            color: Self::BASE_BLUE_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("base_blue"),
            color: Self::BASE_BLUE_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("base_blue"),
            color: Self::BASE_BLUE_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("gray"),
            color: Self::GRAY_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("gray"),
            color: Self::GRAY_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("gray"),
            color: Self::GRAY_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("gray"),
            color: Self::GRAY_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("gray"),
            color: Self::GRAY_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("gray"),
            color: Self::GRAY_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("gray"),
            color: Self::GRAY_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("gray"),
            color: Self::GRAY_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("gray"),
            color: Self::GRAY_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("gray"),
            color: Self::GRAY_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("gray"),
            color: Self::GRAY_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("brown"),
            color: Self::BROWN_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("brown"),
            color: Self::BROWN_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("brown"),
            color: Self::BROWN_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("brown"),
            color: Self::BROWN_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("brown"),
            color: Self::BROWN_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("brown"),
            color: Self::BROWN_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("brown"),
            color: Self::BROWN_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("brown"),
            color: Self::BROWN_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("brown"),
            color: Self::BROWN_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("brown"),
            color: Self::BROWN_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("brown"),
            color: Self::BROWN_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("yellow"),
            color: Self::YELLOW_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("yellow"),
            color: Self::YELLOW_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("yellow"),
            color: Self::YELLOW_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("yellow"),
            color: Self::YELLOW_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("yellow"),
            color: Self::YELLOW_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("yellow"),
            color: Self::YELLOW_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("yellow"),
            color: Self::YELLOW_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("yellow"),
            color: Self::YELLOW_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("yellow"),
            color: Self::YELLOW_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("yellow"),
            color: Self::YELLOW_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("yellow"),
            color: Self::YELLOW_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("orange"),
            color: Self::ORANGE_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("orange"),
            color: Self::ORANGE_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("orange"),
            color: Self::ORANGE_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("orange"),
            color: Self::ORANGE_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("orange"),
            color: Self::ORANGE_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("orange"),
            color: Self::ORANGE_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("orange"),
            color: Self::ORANGE_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("orange"),
            color: Self::ORANGE_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("orange"),
            color: Self::ORANGE_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("orange"),
            color: Self::ORANGE_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("orange"),
            color: Self::ORANGE_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("red"),
            color: Self::RED_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("red"),
            color: Self::RED_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("red"),
            color: Self::RED_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("red"),
            color: Self::RED_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("red"),
            color: Self::RED_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("red"),
            color: Self::RED_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("red"),
            color: Self::RED_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("red"),
            color: Self::RED_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("red"),
            color: Self::RED_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("red"),
            color: Self::RED_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("red"),
            color: Self::RED_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("magenta"),
            color: Self::MAGENTA_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("magenta"),
            color: Self::MAGENTA_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("magenta"),
            color: Self::MAGENTA_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("magenta"),
            color: Self::MAGENTA_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("magenta"),
            color: Self::MAGENTA_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("magenta"),
            color: Self::MAGENTA_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("magenta"),
            color: Self::MAGENTA_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("magenta"),
            color: Self::MAGENTA_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("magenta"),
            color: Self::MAGENTA_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("magenta"),
            color: Self::MAGENTA_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("magenta"),
            color: Self::MAGENTA_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("violet"),
            color: Self::VIOLET_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("violet"),
            color: Self::VIOLET_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("violet"),
            color: Self::VIOLET_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("violet"),
            color: Self::VIOLET_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("violet"),
            color: Self::VIOLET_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("violet"),
            color: Self::VIOLET_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("violet"),
            color: Self::VIOLET_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("violet"),
            color: Self::VIOLET_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("violet"),
            color: Self::VIOLET_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("violet"),
            color: Self::VIOLET_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("violet"),
            color: Self::VIOLET_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("blue"),
            color: Self::BLUE_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("blue"),
            color: Self::BLUE_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("blue"),
            color: Self::BLUE_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("blue"),
            color: Self::BLUE_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("blue"),
            color: Self::BLUE_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("blue"),
            color: Self::BLUE_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("blue"),
            color: Self::BLUE_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("blue"),
            color: Self::BLUE_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("blue"),
            color: Self::BLUE_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("blue"),
            color: Self::BLUE_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("blue"),
            color: Self::BLUE_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("cyan"),
            color: Self::CYAN_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("cyan"),
            color: Self::CYAN_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("cyan"),
            color: Self::CYAN_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("cyan"),
            color: Self::CYAN_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("cyan"),
            color: Self::CYAN_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("cyan"),
            color: Self::CYAN_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("cyan"),
            color: Self::CYAN_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("cyan"),
            color: Self::CYAN_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("cyan"),
            color: Self::CYAN_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("cyan"),
            color: Self::CYAN_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("cyan"),
            color: Self::CYAN_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("green"),
            color: Self::GREEN_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("green"),
            color: Self::GREEN_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("green"),
            color: Self::GREEN_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("green"),
            color: Self::GREEN_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("green"),
            color: Self::GREEN_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("green"),
            color: Self::GREEN_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("green"),
            color: Self::GREEN_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("green"),
            color: Self::GREEN_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("green"),
            color: Self::GREEN_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("green"),
            color: Self::GREEN_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("green"),
            color: Self::GREEN_950,
        },
    ];
}
