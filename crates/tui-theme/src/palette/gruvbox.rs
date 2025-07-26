use std::borrow::Cow;

use crate::{Color, NamedColor, ThemeArray};

// Auto-generated file. Do not edit.

pub struct Gruvbox {}

impl Gruvbox {
    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_50: Color = Color::Rgb(::palette::Srgb::new(0.9337, 0.9450, 0.9524));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_100: Color = Color::Rgb(::palette::Srgb::new(0.8775, 0.9022, 0.9144));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_200: Color = Color::Rgb(::palette::Srgb::new(0.7420, 0.7998, 0.8152));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_300: Color = Color::Rgb(::palette::Srgb::new(0.6500, 0.7022, 0.7182));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_400: Color = Color::Rgb(::palette::Srgb::new(0.5606, 0.6039, 0.6196));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_500: Color = Color::Rgb(::palette::Srgb::new(0.4737, 0.5140, 0.5260));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_600: Color = Color::Rgb(::palette::Srgb::new(0.3840, 0.4119, 0.4238));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_700: Color = Color::Rgb(::palette::Srgb::new(0.3023, 0.3254, 0.3369));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_800: Color = Color::Rgb(::palette::Srgb::new(0.2226, 0.2434, 0.2516));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_900: Color = Color::Rgb(::palette::Srgb::new(0.1529, 0.1686, 0.1726));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_950: Color = Color::Rgb(::palette::Srgb::new(0.1134, 0.1255, 0.1296));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_50: Color = Color::Rgb(::palette::Srgb::new(0.9956, 0.9413, 0.9412));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_100: Color = Color::Rgb(::palette::Srgb::new(0.9962, 0.8863, 0.8823));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_200: Color = Color::Rgb(::palette::Srgb::new(0.9929, 0.7525, 0.7407));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_300: Color = Color::Rgb(::palette::Srgb::new(0.9892, 0.6269, 0.6072));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_400: Color = Color::Rgb(::palette::Srgb::new(0.9850, 0.4740, 0.4386));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_500: Color = Color::Rgb(::palette::Srgb::new(0.9851, 0.2850, 0.2027));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_600: Color = Color::Rgb(::palette::Srgb::new(0.8084, 0.1832, 0.0767));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_700: Color = Color::Rgb(::palette::Srgb::new(0.6237, 0.1328, 0.0502));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_800: Color = Color::Rgb(::palette::Srgb::new(0.4277, 0.0778, 0.0229));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_900: Color = Color::Rgb(::palette::Srgb::new(0.2666, 0.0354, 0.0079));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_950: Color = Color::Rgb(::palette::Srgb::new(0.1798, 0.0163, 0.0044));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_50: Color = Color::Rgb(::palette::Srgb::new(0.9726, 0.9882, 0.2130));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_100: Color = Color::Rgb(::palette::Srgb::new(0.9178, 0.9294, 0.1963));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_200: Color = Color::Rgb(::palette::Srgb::new(0.8157, 0.8274, 0.1725));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_300: Color = Color::Rgb(::palette::Srgb::new(0.7216, 0.7333, 0.1496));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_400: Color = Color::Rgb(::palette::Srgb::new(0.6078, 0.6196, 0.1250));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_500: Color = Color::Rgb(::palette::Srgb::new(0.4980, 0.5058, 0.0930));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_600: Color = Color::Rgb(::palette::Srgb::new(0.3961, 0.4039, 0.0606));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_700: Color = Color::Rgb(::palette::Srgb::new(0.2941, 0.2980, 0.0366));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_800: Color = Color::Rgb(::palette::Srgb::new(0.2038, 0.2078, 0.0208));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_900: Color = Color::Rgb(::palette::Srgb::new(0.1098, 0.1137, 0.0088));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_950: Color = Color::Rgb(::palette::Srgb::new(0.0667, 0.0706, 0.0038));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_50: Color = Color::Rgb(::palette::Srgb::new(0.9998, 0.9609, 0.9180));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_100: Color = Color::Rgb(::palette::Srgb::new(0.9962, 0.9098, 0.8000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_200: Color = Color::Rgb(::palette::Srgb::new(0.9920, 0.8314, 0.5886));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_300: Color = Color::Rgb(::palette::Srgb::new(0.9799, 0.7414, 0.1885));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_400: Color = Color::Rgb(::palette::Srgb::new(0.8314, 0.6274, 0.1490));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_500: Color = Color::Rgb(::palette::Srgb::new(0.6782, 0.5098, 0.1152));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_600: Color = Color::Rgb(::palette::Srgb::new(0.5450, 0.4039, 0.0828));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_700: Color = Color::Rgb(::palette::Srgb::new(0.4038, 0.2981, 0.0525));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_800: Color = Color::Rgb(::palette::Srgb::new(0.2787, 0.2038, 0.0223));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_900: Color = Color::Rgb(::palette::Srgb::new(0.1571, 0.1058, 0.0072));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_950: Color = Color::Rgb(::palette::Srgb::new(0.1056, 0.0707, 0.0045));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_50: Color = Color::Rgb(::palette::Srgb::new(0.9049, 0.9690, 0.9448));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_100: Color = Color::Rgb(::palette::Srgb::new(0.7761, 0.9335, 0.8705));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_200: Color = Color::Rgb(::palette::Srgb::new(0.6676, 0.8350, 0.7726));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_300: Color = Color::Rgb(::palette::Srgb::new(0.5962, 0.7450, 0.6862));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_400: Color = Color::Rgb(::palette::Srgb::new(0.5138, 0.6471, 0.5961));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_500: Color = Color::Rgb(::palette::Srgb::new(0.4199, 0.5294, 0.4863));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_600: Color = Color::Rgb(::palette::Srgb::new(0.3291, 0.4197, 0.3842));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_700: Color = Color::Rgb(::palette::Srgb::new(0.2401, 0.3095, 0.2824));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_800: Color = Color::Rgb(::palette::Srgb::new(0.1655, 0.2194, 0.1962));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_900: Color = Color::Rgb(::palette::Srgb::new(0.0902, 0.1216, 0.1098));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_950: Color = Color::Rgb(::palette::Srgb::new(0.0474, 0.0705, 0.0627));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_50: Color = Color::Rgb(::palette::Srgb::new(0.9775, 0.9487, 0.9527));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_100: Color = Color::Rgb(::palette::Srgb::new(0.9484, 0.8827, 0.8980));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_200: Color = Color::Rgb(::palette::Srgb::new(0.9019, 0.7647, 0.7960));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_300: Color = Color::Rgb(::palette::Srgb::new(0.8630, 0.6470, 0.6980));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_400: Color = Color::Rgb(::palette::Srgb::new(0.8282, 0.5250, 0.6077));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_500: Color = Color::Rgb(::palette::Srgb::new(0.7797, 0.3733, 0.5020));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_600: Color = Color::Rgb(::palette::Srgb::new(0.6199, 0.2899, 0.3920));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_700: Color = Color::Rgb(::palette::Srgb::new(0.4674, 0.2111, 0.2901));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_800: Color = Color::Rgb(::palette::Srgb::new(0.3211, 0.1377, 0.1961));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_900: Color = Color::Rgb(::palette::Srgb::new(0.1998, 0.0747, 0.1138));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_950: Color = Color::Rgb(::palette::Srgb::new(0.1252, 0.0394, 0.0666));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const AQUA_50: Color = Color::Rgb(::palette::Srgb::new(0.8701, 0.9727, 0.8383));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const AQUA_100: Color = Color::Rgb(::palette::Srgb::new(0.7456, 0.9567, 0.6751));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const AQUA_200: Color = Color::Rgb(::palette::Srgb::new(0.6357, 0.8587, 0.5574));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const AQUA_300: Color = Color::Rgb(::palette::Srgb::new(0.5565, 0.7530, 0.4857));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const AQUA_400: Color = Color::Rgb(::palette::Srgb::new(0.4661, 0.6316, 0.4030));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const AQUA_500: Color = Color::Rgb(::palette::Srgb::new(0.3847, 0.5215, 0.3338));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const AQUA_600: Color = Color::Rgb(::palette::Srgb::new(0.3062, 0.4195, 0.2632));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const AQUA_700: Color = Color::Rgb(::palette::Srgb::new(0.2193, 0.3100, 0.1877));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const AQUA_800: Color = Color::Rgb(::palette::Srgb::new(0.1495, 0.2155, 0.1262));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const AQUA_900: Color = Color::Rgb(::palette::Srgb::new(0.0784, 0.1176, 0.0627));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const AQUA_950: Color = Color::Rgb(::palette::Srgb::new(0.0434, 0.0744, 0.0317));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_50: Color = Color::Rgb(::palette::Srgb::new(0.9996, 0.9451, 0.9295));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_100: Color = Color::Rgb(::palette::Srgb::new(1.0000, 0.8863, 0.8548));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_200: Color = Color::Rgb(::palette::Srgb::new(0.9966, 0.7567, 0.6702));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_300: Color = Color::Rgb(::palette::Srgb::new(0.9963, 0.6351, 0.4741));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_400: Color = Color::Rgb(::palette::Srgb::new(0.9961, 0.5020, 0.0983));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_500: Color = Color::Rgb(::palette::Srgb::new(0.8271, 0.4081, 0.0461));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_600: Color = Color::Rgb(::palette::Srgb::new(0.6632, 0.3213, 0.0252));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_700: Color = Color::Rgb(::palette::Srgb::new(0.4901, 0.2314, 0.0121));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_800: Color = Color::Rgb(::palette::Srgb::new(0.3408, 0.1532, 0.0087));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_900: Color = Color::Rgb(::palette::Srgb::new(0.2034, 0.0827, 0.0047));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_950: Color = Color::Rgb(::palette::Srgb::new(0.1451, 0.0511, 0.0001));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_50: Color = Color::Rgb(::palette::Srgb::new(0.9643, 0.9531, 0.9455));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_100: Color = Color::Rgb(::palette::Srgb::new(0.9212, 0.8943, 0.8750));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_200: Color = Color::Rgb(::palette::Srgb::new(0.8596, 0.7997, 0.7558));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_300: Color = Color::Rgb(::palette::Srgb::new(0.7804, 0.6980, 0.6351));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_400: Color = Color::Rgb(::palette::Srgb::new(0.6738, 0.6080, 0.5499));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_500: Color = Color::Rgb(::palette::Srgb::new(0.5757, 0.5179, 0.4677));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_600: Color = Color::Rgb(::palette::Srgb::new(0.4857, 0.4355, 0.3930));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_700: Color = Color::Rgb(::palette::Srgb::new(0.3655, 0.3253, 0.2891));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_800: Color = Color::Rgb(::palette::Srgb::new(0.2468, 0.2197, 0.1966));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_900: Color = Color::Rgb(::palette::Srgb::new(0.1378, 0.1215, 0.1052));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BROWN_950: Color = Color::Rgb(::palette::Srgb::new(0.0867, 0.0744, 0.0622));

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

    pub const PURPLE: ThemeArray<11> = ThemeArray([
        Self::PURPLE_50,
        Self::PURPLE_100,
        Self::PURPLE_200,
        Self::PURPLE_300,
        Self::PURPLE_400,
        Self::PURPLE_500,
        Self::PURPLE_600,
        Self::PURPLE_700,
        Self::PURPLE_800,
        Self::PURPLE_900,
        Self::PURPLE_950,
    ]);

    pub const AQUA: ThemeArray<11> = ThemeArray([
        Self::AQUA_50,
        Self::AQUA_100,
        Self::AQUA_200,
        Self::AQUA_300,
        Self::AQUA_400,
        Self::AQUA_500,
        Self::AQUA_600,
        Self::AQUA_700,
        Self::AQUA_800,
        Self::AQUA_900,
        Self::AQUA_950,
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

    pub const ALL_COLORS: [NamedColor<'_>; 99] = [
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
            group: Cow::Borrowed("purple"),
            color: Self::PURPLE_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("purple"),
            color: Self::PURPLE_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("purple"),
            color: Self::PURPLE_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("purple"),
            color: Self::PURPLE_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("purple"),
            color: Self::PURPLE_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("purple"),
            color: Self::PURPLE_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("purple"),
            color: Self::PURPLE_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("purple"),
            color: Self::PURPLE_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("purple"),
            color: Self::PURPLE_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("purple"),
            color: Self::PURPLE_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("purple"),
            color: Self::PURPLE_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("aqua"),
            color: Self::AQUA_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("aqua"),
            color: Self::AQUA_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("aqua"),
            color: Self::AQUA_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("aqua"),
            color: Self::AQUA_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("aqua"),
            color: Self::AQUA_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("aqua"),
            color: Self::AQUA_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("aqua"),
            color: Self::AQUA_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("aqua"),
            color: Self::AQUA_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("aqua"),
            color: Self::AQUA_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("aqua"),
            color: Self::AQUA_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("aqua"),
            color: Self::AQUA_950,
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
    ];
}
