use std::borrow::Cow;

use crate::{Color, NamedColor, ThemeArray};

// Auto-generated file. Do not edit.

pub struct OneDark {}

impl OneDark {
    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_50: Color = Color::Rgb(::palette::Srgb::new(0.9530, 0.9529, 0.9647));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_100: Color = Color::Rgb(::palette::Srgb::new(0.8903, 0.8980, 0.9174));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_200: Color = Color::Rgb(::palette::Srgb::new(0.7920, 0.8117, 0.8513));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_300: Color = Color::Rgb(::palette::Srgb::new(0.6859, 0.7137, 0.7732));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_400: Color = Color::Rgb(::palette::Srgb::new(0.5810, 0.6196, 0.6969));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_500: Color = Color::Rgb(::palette::Srgb::new(0.4978, 0.5373, 0.6203));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_600: Color = Color::Rgb(::palette::Srgb::new(0.4118, 0.4470, 0.5174));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_700: Color = Color::Rgb(::palette::Srgb::new(0.3414, 0.3726, 0.4311));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_800: Color = Color::Rgb(::palette::Srgb::new(0.2624, 0.2863, 0.3341));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_900: Color = Color::Rgb(::palette::Srgb::new(0.1886, 0.2078, 0.2423));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_950: Color = Color::Rgb(::palette::Srgb::new(0.1565, 0.1725, 0.2046));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_50: Color = Color::Rgb(::palette::Srgb::new(0.9803, 0.9334, 0.9372));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_100: Color = Color::Rgb(::palette::Srgb::new(0.9620, 0.8663, 0.8702));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_200: Color = Color::Rgb(::palette::Srgb::new(0.9293, 0.7295, 0.7411));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_300: Color = Color::Rgb(::palette::Srgb::new(0.9020, 0.5844, 0.6079));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_400: Color = Color::Rgb(::palette::Srgb::new(0.8789, 0.4233, 0.4586));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_500: Color = Color::Rgb(::palette::Srgb::new(0.8079, 0.2823, 0.3333));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_600: Color = Color::Rgb(::palette::Srgb::new(0.6467, 0.2201, 0.2591));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_700: Color = Color::Rgb(::palette::Srgb::new(0.4936, 0.1615, 0.1925));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_800: Color = Color::Rgb(::palette::Srgb::new(0.3485, 0.1027, 0.1258));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_900: Color = Color::Rgb(::palette::Srgb::new(0.2160, 0.0507, 0.0665));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_950: Color = Color::Rgb(::palette::Srgb::new(0.1448, 0.0238, 0.0355));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_50: Color = Color::Rgb(::palette::Srgb::new(0.8980, 0.9805, 0.8469));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_100: Color = Color::Rgb(::palette::Srgb::new(0.7488, 0.9610, 0.5991));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_200: Color = Color::Rgb(::palette::Srgb::new(0.6749, 0.8665, 0.5418));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_300: Color = Color::Rgb(::palette::Srgb::new(0.5964, 0.7645, 0.4752));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_400: Color = Color::Rgb(::palette::Srgb::new(0.5061, 0.6509, 0.4004));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_500: Color = Color::Rgb(::palette::Srgb::new(0.4117, 0.5294, 0.3252));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_600: Color = Color::Rgb(::palette::Srgb::new(0.3214, 0.4158, 0.2505));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_700: Color = Color::Rgb(::palette::Srgb::new(0.2396, 0.3174, 0.1852));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_800: Color = Color::Rgb(::palette::Srgb::new(0.1567, 0.2158, 0.1172));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_900: Color = Color::Rgb(::palette::Srgb::new(0.0903, 0.1255, 0.0630));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_950: Color = Color::Rgb(::palette::Srgb::new(0.0473, 0.0744, 0.0279));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_50: Color = Color::Rgb(::palette::Srgb::new(0.9889, 0.9646, 0.9281));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_100: Color = Color::Rgb(::palette::Srgb::new(0.9722, 0.9138, 0.8320));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_200: Color = Color::Rgb(::palette::Srgb::new(0.9451, 0.8274, 0.6314));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_300: Color = Color::Rgb(::palette::Srgb::new(0.8981, 0.7529, 0.4823));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_400: Color = Color::Rgb(::palette::Srgb::new(0.7572, 0.6313, 0.4031));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_500: Color = Color::Rgb(::palette::Srgb::new(0.6274, 0.5216, 0.3296));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_600: Color = Color::Rgb(::palette::Srgb::new(0.4908, 0.4077, 0.2495));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_700: Color = Color::Rgb(::palette::Srgb::new(0.3688, 0.3059, 0.1838));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_800: Color = Color::Rgb(::palette::Srgb::new(0.2550, 0.2078, 0.1213));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_900: Color = Color::Rgb(::palette::Srgb::new(0.1409, 0.1099, 0.0559));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_950: Color = Color::Rgb(::palette::Srgb::new(0.0939, 0.0706, 0.0317));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_50: Color = Color::Rgb(::palette::Srgb::new(0.9852, 0.9488, 0.9245));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_100: Color = Color::Rgb(::palette::Srgb::new(0.9697, 0.8978, 0.8497));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_200: Color = Color::Rgb(::palette::Srgb::new(0.9363, 0.7963, 0.6874));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_300: Color = Color::Rgb(::palette::Srgb::new(0.9182, 0.6900, 0.4813));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_400: Color = Color::Rgb(::palette::Srgb::new(0.8198, 0.6038, 0.3996));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_500: Color = Color::Rgb(::palette::Srgb::new(0.6707, 0.4901, 0.3214));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_600: Color = Color::Rgb(::palette::Srgb::new(0.5331, 0.3883, 0.2513));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_700: Color = Color::Rgb(::palette::Srgb::new(0.4046, 0.2899, 0.1830));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_800: Color = Color::Rgb(::palette::Srgb::new(0.2824, 0.2000, 0.1216));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_900: Color = Color::Rgb(::palette::Srgb::new(0.1574, 0.1057, 0.0539));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_950: Color = Color::Rgb(::palette::Srgb::new(0.1061, 0.0666, 0.0311));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_50: Color = Color::Rgb(::palette::Srgb::new(0.9328, 0.9569, 0.9930));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_100: Color = Color::Rgb(::palette::Srgb::new(0.8630, 0.9137, 0.9839));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_200: Color = Color::Rgb(::palette::Srgb::new(0.7375, 0.8431, 0.9683));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_300: Color = Color::Rgb(::palette::Srgb::new(0.5769, 0.7646, 0.9524));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_400: Color = Color::Rgb(::palette::Srgb::new(0.3812, 0.6862, 0.9366));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_500: Color = Color::Rgb(::palette::Srgb::new(0.2690, 0.5687, 0.8010));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_600: Color = Color::Rgb(::palette::Srgb::new(0.2081, 0.4470, 0.6272));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_700: Color = Color::Rgb(::palette::Srgb::new(0.1501, 0.3372, 0.4816));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_800: Color = Color::Rgb(::palette::Srgb::new(0.0907, 0.2274, 0.3291));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_900: Color = Color::Rgb(::palette::Srgb::new(0.0419, 0.1294, 0.2008));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_950: Color = Color::Rgb(::palette::Srgb::new(0.0203, 0.0823, 0.1288));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_50: Color = Color::Rgb(::palette::Srgb::new(0.9693, 0.9330, 0.9812));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_100: Color = Color::Rgb(::palette::Srgb::new(0.9412, 0.8785, 0.9684));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_200: Color = Color::Rgb(::palette::Srgb::new(0.8822, 0.7414, 0.9328));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_300: Color = Color::Rgb(::palette::Srgb::new(0.8312, 0.6159, 0.9016));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_400: Color = Color::Rgb(::palette::Srgb::new(0.7770, 0.4701, 0.8673));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_500: Color = Color::Rgb(::palette::Srgb::new(0.7016, 0.3261, 0.7992));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_600: Color = Color::Rgb(::palette::Srgb::new(0.5531, 0.2506, 0.6316));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_700: Color = Color::Rgb(::palette::Srgb::new(0.4200, 0.1837, 0.4828));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_800: Color = Color::Rgb(::palette::Srgb::new(0.2982, 0.1215, 0.3452));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_900: Color = Color::Rgb(::palette::Srgb::new(0.1723, 0.0592, 0.1996));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAGENTA_950: Color = Color::Rgb(::palette::Srgb::new(0.1139, 0.0313, 0.1374));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_50: Color = Color::Rgb(::palette::Srgb::new(0.8774, 0.9728, 0.9886));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_100: Color = Color::Rgb(::palette::Srgb::new(0.7379, 0.9450, 0.9801));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_200: Color = Color::Rgb(::palette::Srgb::new(0.4295, 0.8860, 0.9445));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_300: Color = Color::Rgb(::palette::Srgb::new(0.3844, 0.8038, 0.8586));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_400: Color = Color::Rgb(::palette::Srgb::new(0.3349, 0.7140, 0.7612));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_500: Color = Color::Rgb(::palette::Srgb::new(0.2726, 0.5802, 0.6191));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_600: Color = Color::Rgb(::palette::Srgb::new(0.2076, 0.4627, 0.4941));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_700: Color = Color::Rgb(::palette::Srgb::new(0.1511, 0.3409, 0.3641));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_800: Color = Color::Rgb(::palette::Srgb::new(0.0939, 0.2353, 0.2549));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_900: Color = Color::Rgb(::palette::Srgb::new(0.0402, 0.1293, 0.1410));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_950: Color = Color::Rgb(::palette::Srgb::new(0.0193, 0.0863, 0.0942));

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

    pub const PEACH: ThemeArray<11> = ThemeArray([
        Self::PEACH_50,
        Self::PEACH_100,
        Self::PEACH_200,
        Self::PEACH_300,
        Self::PEACH_400,
        Self::PEACH_500,
        Self::PEACH_600,
        Self::PEACH_700,
        Self::PEACH_800,
        Self::PEACH_900,
        Self::PEACH_950,
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

    pub const ALL_COLORS: [NamedColor<'_>; 88] = [
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
            group: Cow::Borrowed("peach"),
            color: Self::PEACH_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("peach"),
            color: Self::PEACH_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("peach"),
            color: Self::PEACH_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("peach"),
            color: Self::PEACH_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("peach"),
            color: Self::PEACH_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("peach"),
            color: Self::PEACH_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("peach"),
            color: Self::PEACH_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("peach"),
            color: Self::PEACH_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("peach"),
            color: Self::PEACH_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("peach"),
            color: Self::PEACH_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("peach"),
            color: Self::PEACH_950,
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
    ];
}
