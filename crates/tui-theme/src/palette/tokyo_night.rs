use std::borrow::Cow;

use crate::{Color, NamedColor, ThemeArray};

// Auto-generated file. Do not edit.

pub struct TokyoNight {}

impl TokyoNight {
    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_50: Color = Color::Rgb(::palette::Srgb::new(0.9531, 0.9530, 0.9678));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_100: Color = Color::Rgb(::palette::Srgb::new(0.8903, 0.8980, 0.9294));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_200: Color = Color::Rgb(::palette::Srgb::new(0.7842, 0.7961, 0.8634));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_300: Color = Color::Rgb(::palette::Srgb::new(0.6943, 0.7098, 0.8030));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_400: Color = Color::Rgb(::palette::Srgb::new(0.5881, 0.6117, 0.7418));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_500: Color = Color::Rgb(::palette::Srgb::new(0.4902, 0.5177, 0.6747));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_600: Color = Color::Rgb(::palette::Srgb::new(0.3920, 0.4273, 0.6007));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_700: Color = Color::Rgb(::palette::Srgb::new(0.3214, 0.3529, 0.4987));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_800: Color = Color::Rgb(::palette::Srgb::new(0.2472, 0.2706, 0.3875));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_900: Color = Color::Rgb(::palette::Srgb::new(0.1767, 0.1923, 0.2816));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_950: Color = Color::Rgb(::palette::Srgb::new(0.1414, 0.1569, 0.2304));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_50: Color = Color::Rgb(::palette::Srgb::new(0.9413, 0.9530, 0.9960));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_100: Color = Color::Rgb(::palette::Srgb::new(0.8828, 0.9098, 0.9909));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_200: Color = Color::Rgb(::palette::Srgb::new(0.7606, 0.8196, 0.9850));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_300: Color = Color::Rgb(::palette::Srgb::new(0.6201, 0.7216, 0.9753));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_400: Color = Color::Rgb(::palette::Srgb::new(0.4787, 0.6354, 0.9683));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_500: Color = Color::Rgb(::palette::Srgb::new(0.2072, 0.5176, 0.9574));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_600: Color = Color::Rgb(::palette::Srgb::new(0.1445, 0.4117, 0.7730));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_700: Color = Color::Rgb(::palette::Srgb::new(0.1031, 0.3099, 0.5911));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_800: Color = Color::Rgb(::palette::Srgb::new(0.0552, 0.2040, 0.4037));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_900: Color = Color::Rgb(::palette::Srgb::new(0.0197, 0.1138, 0.2430));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_950: Color = Color::Rgb(::palette::Srgb::new(0.0117, 0.0745, 0.1727));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_50: Color = Color::Rgb(::palette::Srgb::new(0.8542, 0.9610, 0.9965));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_100: Color = Color::Rgb(::palette::Srgb::new(0.7217, 0.9333, 0.9960));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_200: Color = Color::Rgb(::palette::Srgb::new(0.2097, 0.8707, 0.9924));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_300: Color = Color::Rgb(::palette::Srgb::new(0.1676, 0.7646, 0.8701));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_400: Color = Color::Rgb(::palette::Srgb::new(0.1354, 0.6430, 0.7330));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_500: Color = Color::Rgb(::palette::Srgb::new(0.1040, 0.5333, 0.6076));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_600: Color = Color::Rgb(::palette::Srgb::new(0.0782, 0.4273, 0.4897));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_700: Color = Color::Rgb(::palette::Srgb::new(0.0440, 0.3137, 0.3646));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_800: Color = Color::Rgb(::palette::Srgb::new(0.0204, 0.2198, 0.2554));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_900: Color = Color::Rgb(::palette::Srgb::new(0.0092, 0.1214, 0.1446));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const CYAN_950: Color = Color::Rgb(::palette::Srgb::new(0.0034, 0.0746, 0.0943));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_50: Color = Color::Rgb(::palette::Srgb::new(0.8747, 0.9881, 0.9880));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_100: Color = Color::Rgb(::palette::Srgb::new(0.7072, 0.9761, 0.9721));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_200: Color = Color::Rgb(::palette::Srgb::new(0.4950, 0.8705, 0.8665));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_300: Color = Color::Rgb(::palette::Srgb::new(0.4273, 0.7569, 0.7529));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_400: Color = Color::Rgb(::palette::Srgb::new(0.3549, 0.6349, 0.6310));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_500: Color = Color::Rgb(::palette::Srgb::new(0.2887, 0.5257, 0.5217));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_600: Color = Color::Rgb(::palette::Srgb::new(0.2284, 0.4195, 0.4195));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_700: Color = Color::Rgb(::palette::Srgb::new(0.1623, 0.3095, 0.3095));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_800: Color = Color::Rgb(::palette::Srgb::new(0.1060, 0.2157, 0.2157));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_900: Color = Color::Rgb(::palette::Srgb::new(0.0467, 0.1177, 0.1177));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_950: Color = Color::Rgb(::palette::Srgb::new(0.0237, 0.0744, 0.0744));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_50: Color = Color::Rgb(::palette::Srgb::new(0.8867, 0.9880, 0.8047));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_100: Color = Color::Rgb(::palette::Srgb::new(0.7919, 0.9806, 0.6068));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_200: Color = Color::Rgb(::palette::Srgb::new(0.6860, 0.8942, 0.4615));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_300: Color = Color::Rgb(::palette::Srgb::new(0.6201, 0.8077, 0.4171));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_400: Color = Color::Rgb(::palette::Srgb::new(0.5174, 0.6786, 0.3439));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_500: Color = Color::Rgb(::palette::Srgb::new(0.4236, 0.5569, 0.2785));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_600: Color = Color::Rgb(::palette::Srgb::new(0.3331, 0.4393, 0.2150));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_700: Color = Color::Rgb(::palette::Srgb::new(0.2473, 0.3293, 0.1538));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_800: Color = Color::Rgb(::palette::Srgb::new(0.1646, 0.2236, 0.0976));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_900: Color = Color::Rgb(::palette::Srgb::new(0.0863, 0.1295, 0.0430));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_950: Color = Color::Rgb(::palette::Srgb::new(0.0550, 0.0862, 0.0239));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MINT_50: Color = Color::Rgb(::palette::Srgb::new(0.8779, 0.9884, 0.9686));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MINT_100: Color = Color::Rgb(::palette::Srgb::new(0.7370, 0.9804, 0.9371));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MINT_200: Color = Color::Rgb(::palette::Srgb::new(0.4917, 0.9292, 0.8626));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MINT_300: Color = Color::Rgb(::palette::Srgb::new(0.4490, 0.8552, 0.7921));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MINT_400: Color = Color::Rgb(::palette::Srgb::new(0.3757, 0.7179, 0.6667));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MINT_500: Color = Color::Rgb(::palette::Srgb::new(0.3043, 0.5964, 0.5490));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MINT_600: Color = Color::Rgb(::palette::Srgb::new(0.2340, 0.4668, 0.4314));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MINT_700: Color = Color::Rgb(::palette::Srgb::new(0.1733, 0.3528, 0.3254));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MINT_800: Color = Color::Rgb(::palette::Srgb::new(0.1058, 0.2353, 0.2156));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MINT_900: Color = Color::Rgb(::palette::Srgb::new(0.0502, 0.1374, 0.1256));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MINT_950: Color = Color::Rgb(::palette::Srgb::new(0.0230, 0.0863, 0.0784));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const LAVENDER_50: Color = Color::Rgb(::palette::Srgb::new(0.9608, 0.9491, 0.9955));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const LAVENDER_100: Color = Color::Rgb(::palette::Srgb::new(0.9255, 0.8979, 0.9921));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const LAVENDER_200: Color = Color::Rgb(::palette::Srgb::new(0.8629, 0.8036, 0.9851));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const LAVENDER_300: Color = Color::Rgb(::palette::Srgb::new(0.7923, 0.7019, 0.9768));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const LAVENDER_400: Color = Color::Rgb(::palette::Srgb::new(0.7333, 0.6043, 0.9678));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const LAVENDER_500: Color = Color::Rgb(::palette::Srgb::new(0.6430, 0.4394, 0.9523));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const LAVENDER_600: Color = Color::Rgb(::palette::Srgb::new(0.5645, 0.2321, 0.9325));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const LAVENDER_700: Color = Color::Rgb(::palette::Srgb::new(0.4429, 0.1306, 0.7600));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const LAVENDER_800: Color = Color::Rgb(::palette::Srgb::new(0.3021, 0.0773, 0.5300));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const LAVENDER_900: Color = Color::Rgb(::palette::Srgb::new(0.1803, 0.0318, 0.3291));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const LAVENDER_950: Color = Color::Rgb(::palette::Srgb::new(0.1177, 0.0156, 0.2275));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSE_50: Color = Color::Rgb(::palette::Srgb::new(0.9994, 0.9257, 0.9411));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSE_100: Color = Color::Rgb(::palette::Srgb::new(1.0000, 0.8664, 0.8940));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSE_200: Color = Color::Rgb(::palette::Srgb::new(0.9990, 0.7260, 0.7922));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSE_300: Color = Color::Rgb(::palette::Srgb::new(0.9999, 0.5767, 0.6902));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSE_400: Color = Color::Rgb(::palette::Srgb::new(0.9993, 0.3969, 0.5882));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSE_500: Color = Color::Rgb(::palette::Srgb::new(0.9995, 0.0048, 0.4862));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSE_600: Color = Color::Rgb(::palette::Srgb::new(0.8040, 0.0000, 0.3842));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSE_700: Color = Color::Rgb(::palette::Srgb::new(0.6123, 0.0000, 0.2861));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSE_800: Color = Color::Rgb(::palette::Srgb::new(0.4309, 0.0019, 0.1923));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSE_900: Color = Color::Rgb(::palette::Srgb::new(0.2626, 0.0003, 0.1059));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSE_950: Color = Color::Rgb(::palette::Srgb::new(0.1688, 0.0000, 0.0588));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_50: Color = Color::Rgb(::palette::Srgb::new(1.0000, 0.9448, 0.9247));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_100: Color = Color::Rgb(::palette::Srgb::new(0.9997, 0.8746, 0.8277));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_200: Color = Color::Rgb(::palette::Srgb::new(1.0000, 0.7565, 0.6500));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_300: Color = Color::Rgb(::palette::Srgb::new(0.9995, 0.6199, 0.3931));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_400: Color = Color::Rgb(::palette::Srgb::new(0.9137, 0.4864, 0.0019));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_500: Color = Color::Rgb(::palette::Srgb::new(0.7601, 0.4005, 0.0067));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_600: Color = Color::Rgb(::palette::Srgb::new(0.6043, 0.3095, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_700: Color = Color::Rgb(::palette::Srgb::new(0.4624, 0.2355, 0.0018));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_800: Color = Color::Rgb(::palette::Srgb::new(0.3218, 0.1528, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_900: Color = Color::Rgb(::palette::Srgb::new(0.1958, 0.0865, 0.0005));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ORANGE_950: Color = Color::Rgb(::palette::Srgb::new(0.1259, 0.0428, 0.0000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_50: Color = Color::Rgb(::palette::Srgb::new(0.9491, 0.9372, 0.9804));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_100: Color = Color::Rgb(::palette::Srgb::new(0.9018, 0.8747, 0.9595));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_200: Color = Color::Rgb(::palette::Srgb::new(0.7960, 0.7414, 0.9208));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_300: Color = Color::Rgb(::palette::Srgb::new(0.7061, 0.6154, 0.8871));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_400: Color = Color::Rgb(::palette::Srgb::new(0.6159, 0.4860, 0.8477));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_500: Color = Color::Rgb(::palette::Srgb::new(0.5374, 0.3646, 0.8042));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_600: Color = Color::Rgb(::palette::Srgb::new(0.4432, 0.2586, 0.7020));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_700: Color = Color::Rgb(::palette::Srgb::new(0.3335, 0.1880, 0.5336));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_800: Color = Color::Rgb(::palette::Srgb::new(0.2392, 0.1292, 0.3884));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_900: Color = Color::Rgb(::palette::Srgb::new(0.1411, 0.0671, 0.2426));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PURPLE_950: Color = Color::Rgb(::palette::Srgb::new(0.0901, 0.0356, 0.1642));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_50: Color = Color::Rgb(::palette::Srgb::new(0.9953, 0.9415, 0.9491));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_100: Color = Color::Rgb(::palette::Srgb::new(0.9894, 0.8702, 0.8860));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_200: Color = Color::Rgb(::palette::Srgb::new(0.9798, 0.7533, 0.7844));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_300: Color = Color::Rgb(::palette::Srgb::new(0.9725, 0.6117, 0.6705));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_400: Color = Color::Rgb(::palette::Srgb::new(0.9688, 0.4626, 0.5567));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_500: Color = Color::Rgb(::palette::Srgb::new(0.9654, 0.1982, 0.4074));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_600: Color = Color::Rgb(::palette::Srgb::new(0.7804, 0.1256, 0.3177));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_700: Color = Color::Rgb(::palette::Srgb::new(0.5964, 0.0857, 0.2313));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_800: Color = Color::Rgb(::palette::Srgb::new(0.4202, 0.0455, 0.1527));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_900: Color = Color::Rgb(::palette::Srgb::new(0.2554, 0.0146, 0.0821));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_950: Color = Color::Rgb(::palette::Srgb::new(0.1644, 0.0081, 0.0432));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_50: Color = Color::Rgb(::palette::Srgb::new(0.9804, 0.9333, 0.9332));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_100: Color = Color::Rgb(::palette::Srgb::new(0.9641, 0.8787, 0.8786));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_200: Color = Color::Rgb(::palette::Srgb::new(0.9291, 0.7453, 0.7452));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_300: Color = Color::Rgb(::palette::Srgb::new(0.9062, 0.6156, 0.6155));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_400: Color = Color::Rgb(::palette::Srgb::new(0.8818, 0.4633, 0.4631));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_500: Color = Color::Rgb(::palette::Srgb::new(0.8584, 0.2948, 0.2946));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_600: Color = Color::Rgb(::palette::Srgb::new(0.6866, 0.2269, 0.2271));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_700: Color = Color::Rgb(::palette::Srgb::new(0.5294, 0.1686, 0.1686));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_800: Color = Color::Rgb(::palette::Srgb::new(0.3689, 0.1055, 0.1056));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_900: Color = Color::Rgb(::palette::Srgb::new(0.2313, 0.0551, 0.0550));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const RED_950: Color = Color::Rgb(::palette::Srgb::new(0.1490, 0.0235, 0.0235));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_50: Color = Color::Rgb(::palette::Srgb::new(0.9835, 0.9492, 0.9148));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_100: Color = Color::Rgb(::palette::Srgb::new(0.9730, 0.8980, 0.8229));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_200: Color = Color::Rgb(::palette::Srgb::new(0.9498, 0.7840, 0.5748));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_300: Color = Color::Rgb(::palette::Srgb::new(0.8789, 0.6862, 0.4068));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_400: Color = Color::Rgb(::palette::Srgb::new(0.7457, 0.5802, 0.3397));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_500: Color = Color::Rgb(::palette::Srgb::new(0.6118, 0.4746, 0.2747));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_600: Color = Color::Rgb(::palette::Srgb::new(0.4904, 0.3804, 0.2152));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_700: Color = Color::Rgb(::palette::Srgb::new(0.3689, 0.2823, 0.1524));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_800: Color = Color::Rgb(::palette::Srgb::new(0.2510, 0.1882, 0.0980));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_900: Color = Color::Rgb(::palette::Srgb::new(0.1487, 0.1099, 0.0479));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_950: Color = Color::Rgb(::palette::Srgb::new(0.0906, 0.0626, 0.0189));

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

    pub const TEAL: ThemeArray<11> = ThemeArray([
        Self::TEAL_50,
        Self::TEAL_100,
        Self::TEAL_200,
        Self::TEAL_300,
        Self::TEAL_400,
        Self::TEAL_500,
        Self::TEAL_600,
        Self::TEAL_700,
        Self::TEAL_800,
        Self::TEAL_900,
        Self::TEAL_950,
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

    pub const MINT: ThemeArray<11> = ThemeArray([
        Self::MINT_50,
        Self::MINT_100,
        Self::MINT_200,
        Self::MINT_300,
        Self::MINT_400,
        Self::MINT_500,
        Self::MINT_600,
        Self::MINT_700,
        Self::MINT_800,
        Self::MINT_900,
        Self::MINT_950,
    ]);

    pub const LAVENDER: ThemeArray<11> = ThemeArray([
        Self::LAVENDER_50,
        Self::LAVENDER_100,
        Self::LAVENDER_200,
        Self::LAVENDER_300,
        Self::LAVENDER_400,
        Self::LAVENDER_500,
        Self::LAVENDER_600,
        Self::LAVENDER_700,
        Self::LAVENDER_800,
        Self::LAVENDER_900,
        Self::LAVENDER_950,
    ]);

    pub const ROSE: ThemeArray<11> = ThemeArray([
        Self::ROSE_50,
        Self::ROSE_100,
        Self::ROSE_200,
        Self::ROSE_300,
        Self::ROSE_400,
        Self::ROSE_500,
        Self::ROSE_600,
        Self::ROSE_700,
        Self::ROSE_800,
        Self::ROSE_900,
        Self::ROSE_950,
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

    pub const PINK: ThemeArray<11> = ThemeArray([
        Self::PINK_50,
        Self::PINK_100,
        Self::PINK_200,
        Self::PINK_300,
        Self::PINK_400,
        Self::PINK_500,
        Self::PINK_600,
        Self::PINK_700,
        Self::PINK_800,
        Self::PINK_900,
        Self::PINK_950,
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

    pub const ALL_COLORS: [NamedColor<'_>; 143] = [
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
            group: Cow::Borrowed("teal"),
            color: Self::TEAL_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("teal"),
            color: Self::TEAL_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("teal"),
            color: Self::TEAL_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("teal"),
            color: Self::TEAL_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("teal"),
            color: Self::TEAL_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("teal"),
            color: Self::TEAL_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("teal"),
            color: Self::TEAL_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("teal"),
            color: Self::TEAL_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("teal"),
            color: Self::TEAL_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("teal"),
            color: Self::TEAL_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("teal"),
            color: Self::TEAL_950,
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
            group: Cow::Borrowed("mint"),
            color: Self::MINT_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("mint"),
            color: Self::MINT_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("mint"),
            color: Self::MINT_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("mint"),
            color: Self::MINT_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("mint"),
            color: Self::MINT_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("mint"),
            color: Self::MINT_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("mint"),
            color: Self::MINT_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("mint"),
            color: Self::MINT_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("mint"),
            color: Self::MINT_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("mint"),
            color: Self::MINT_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("mint"),
            color: Self::MINT_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("lavender"),
            color: Self::LAVENDER_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("lavender"),
            color: Self::LAVENDER_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("lavender"),
            color: Self::LAVENDER_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("lavender"),
            color: Self::LAVENDER_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("lavender"),
            color: Self::LAVENDER_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("lavender"),
            color: Self::LAVENDER_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("lavender"),
            color: Self::LAVENDER_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("lavender"),
            color: Self::LAVENDER_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("lavender"),
            color: Self::LAVENDER_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("lavender"),
            color: Self::LAVENDER_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("lavender"),
            color: Self::LAVENDER_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("rose"),
            color: Self::ROSE_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("rose"),
            color: Self::ROSE_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("rose"),
            color: Self::ROSE_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("rose"),
            color: Self::ROSE_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("rose"),
            color: Self::ROSE_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("rose"),
            color: Self::ROSE_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("rose"),
            color: Self::ROSE_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("rose"),
            color: Self::ROSE_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("rose"),
            color: Self::ROSE_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("rose"),
            color: Self::ROSE_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("rose"),
            color: Self::ROSE_950,
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
            group: Cow::Borrowed("pink"),
            color: Self::PINK_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("pink"),
            color: Self::PINK_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("pink"),
            color: Self::PINK_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("pink"),
            color: Self::PINK_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("pink"),
            color: Self::PINK_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("pink"),
            color: Self::PINK_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("pink"),
            color: Self::PINK_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("pink"),
            color: Self::PINK_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("pink"),
            color: Self::PINK_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("pink"),
            color: Self::PINK_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("pink"),
            color: Self::PINK_950,
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
    ];
}
