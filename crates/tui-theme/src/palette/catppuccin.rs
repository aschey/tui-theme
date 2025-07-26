use std::borrow::Cow;

use crate::{Color, NamedColor, ThemeArray};

// Auto-generated file. Do not edit.

pub struct Catppuccin {}

impl Catppuccin {
    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_50: Color = Color::Rgb(::palette::Srgb::new(0.9530, 0.9529, 0.9647));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_100: Color = Color::Rgb(::palette::Srgb::new(0.9058, 0.9098, 0.9300));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_200: Color = Color::Rgb(::palette::Srgb::new(0.8156, 0.8196, 0.8595));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_300: Color = Color::Rgb(::palette::Srgb::new(0.7136, 0.7215, 0.7852));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_400: Color = Color::Rgb(::palette::Srgb::new(0.6277, 0.6354, 0.7167));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_500: Color = Color::Rgb(::palette::Srgb::new(0.5411, 0.5489, 0.6517));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_600: Color = Color::Rgb(::palette::Srgb::new(0.4548, 0.4705, 0.5850));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_700: Color = Color::Rgb(::palette::Srgb::new(0.3766, 0.3922, 0.5130));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_800: Color = Color::Rgb(::palette::Srgb::new(0.2981, 0.3098, 0.4118));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_900: Color = Color::Rgb(::palette::Srgb::new(0.1528, 0.1607, 0.2203));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GRAY_950: Color = Color::Rgb(::palette::Srgb::new(0.0940, 0.0980, 0.1419));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSEWATER_50: Color = Color::Rgb(::palette::Srgb::new(0.9759, 0.9336, 0.9258));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSEWATER_100: Color = Color::Rgb(::palette::Srgb::new(0.9612, 0.8783, 0.8625));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSEWATER_200: Color = Color::Rgb(::palette::Srgb::new(0.9259, 0.7489, 0.7095));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSEWATER_300: Color = Color::Rgb(::palette::Srgb::new(0.8983, 0.6117, 0.5410));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSEWATER_400: Color = Color::Rgb(::palette::Srgb::new(0.8312, 0.4708, 0.3532));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSEWATER_500: Color = Color::Rgb(::palette::Srgb::new(0.6903, 0.3842, 0.2861));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSEWATER_600: Color = Color::Rgb(::palette::Srgb::new(0.5494, 0.3018, 0.2232));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSEWATER_700: Color = Color::Rgb(::palette::Srgb::new(0.4191, 0.2238, 0.1652));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSEWATER_800: Color = Color::Rgb(::palette::Srgb::new(0.2939, 0.1531, 0.1062));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSEWATER_900: Color = Color::Rgb(::palette::Srgb::new(0.1763, 0.0824, 0.0511));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const ROSEWATER_950: Color = Color::Rgb(::palette::Srgb::new(0.1175, 0.0472, 0.0276));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const FLAMINGO_50: Color = Color::Rgb(::palette::Srgb::new(0.9848, 0.9448, 0.9448));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const FLAMINGO_100: Color = Color::Rgb(::palette::Srgb::new(0.9737, 0.9055, 0.9054));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const FLAMINGO_200: Color = Color::Rgb(::palette::Srgb::new(0.9494, 0.8037, 0.8036));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const FLAMINGO_300: Color = Color::Rgb(::palette::Srgb::new(0.9172, 0.6591, 0.6590));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const FLAMINGO_400: Color = Color::Rgb(::palette::Srgb::new(0.8949, 0.4897, 0.4897));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const FLAMINGO_500: Color = Color::Rgb(::palette::Srgb::new(0.8790, 0.2858, 0.2859));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const FLAMINGO_600: Color = Color::Rgb(::palette::Srgb::new(0.7094, 0.2279, 0.2277));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const FLAMINGO_700: Color = Color::Rgb(::palette::Srgb::new(0.5377, 0.1602, 0.1603));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const FLAMINGO_800: Color = Color::Rgb(::palette::Srgb::new(0.3722, 0.1026, 0.1024));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const FLAMINGO_900: Color = Color::Rgb(::palette::Srgb::new(0.2355, 0.0506, 0.0507));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const FLAMINGO_950: Color = Color::Rgb(::palette::Srgb::new(0.1490, 0.0235, 0.0235));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_50: Color = Color::Rgb(::palette::Srgb::new(0.9876, 0.9416, 0.9759));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_100: Color = Color::Rgb(::palette::Srgb::new(0.9804, 0.8824, 0.9528));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_200: Color = Color::Rgb(::palette::Srgb::new(0.9605, 0.7609, 0.9056));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_300: Color = Color::Rgb(::palette::Srgb::new(0.9371, 0.5885, 0.8507));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_400: Color = Color::Rgb(::palette::Srgb::new(0.9132, 0.4086, 0.8034));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_500: Color = Color::Rgb(::palette::Srgb::new(0.8036, 0.2635, 0.6938));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_600: Color = Color::Rgb(::palette::Srgb::new(0.6471, 0.2041, 0.5608));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_700: Color = Color::Rgb(::palette::Srgb::new(0.4858, 0.1459, 0.4153));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_800: Color = Color::Rgb(::palette::Srgb::new(0.3449, 0.0946, 0.2940));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_900: Color = Color::Rgb(::palette::Srgb::new(0.2036, 0.0398, 0.1684));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PINK_950: Color = Color::Rgb(::palette::Srgb::new(0.1375, 0.0193, 0.1139));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAUVE_50: Color = Color::Rgb(::palette::Srgb::new(0.9649, 0.9451, 0.9961));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAUVE_100: Color = Color::Rgb(::palette::Srgb::new(0.9255, 0.8823, 0.9881));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAUVE_200: Color = Color::Rgb(::palette::Srgb::new(0.8627, 0.7727, 0.9798));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAUVE_300: Color = Color::Rgb(::palette::Srgb::new(0.7963, 0.6506, 0.9691));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAUVE_400: Color = Color::Rgb(::palette::Srgb::new(0.7215, 0.4944, 0.9524));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAUVE_500: Color = Color::Rgb(::palette::Srgb::new(0.6629, 0.3290, 0.9376));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAUVE_600: Color = Color::Rgb(::palette::Srgb::new(0.5645, 0.1699, 0.8384));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAUVE_700: Color = Color::Rgb(::palette::Srgb::new(0.4353, 0.1219, 0.6508));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAUVE_800: Color = Color::Rgb(::palette::Srgb::new(0.2981, 0.0703, 0.4550));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAUVE_900: Color = Color::Rgb(::palette::Srgb::new(0.1841, 0.0322, 0.2896));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAUVE_950: Color = Color::Rgb(::palette::Srgb::new(0.1138, 0.0117, 0.1883));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAROON_50: Color = Color::Rgb(::palette::Srgb::new(0.9847, 0.9449, 0.9488));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAROON_100: Color = Color::Rgb(::palette::Srgb::new(0.9697, 0.8780, 0.8899));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAROON_200: Color = Color::Rgb(::palette::Srgb::new(0.9456, 0.7528, 0.7803));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAROON_300: Color = Color::Rgb(::palette::Srgb::new(0.9210, 0.6279, 0.6746));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAROON_400: Color = Color::Rgb(::palette::Srgb::new(0.8990, 0.4621, 0.5448));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAROON_500: Color = Color::Rgb(::palette::Srgb::new(0.8466, 0.2792, 0.4119));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAROON_600: Color = Color::Rgb(::palette::Srgb::new(0.6863, 0.2158, 0.3294));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAROON_700: Color = Color::Rgb(::palette::Srgb::new(0.5136, 0.1531, 0.2392));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAROON_800: Color = Color::Rgb(::palette::Srgb::new(0.3687, 0.1018, 0.1646));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAROON_900: Color = Color::Rgb(::palette::Srgb::new(0.2157, 0.0431, 0.0862));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const MAROON_950: Color = Color::Rgb(::palette::Srgb::new(0.1491, 0.0235, 0.0509));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_50: Color = Color::Rgb(::palette::Srgb::new(0.9955, 0.9453, 0.9220));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_100: Color = Color::Rgb(::palette::Srgb::new(0.9913, 0.9061, 0.8634));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_200: Color = Color::Rgb(::palette::Srgb::new(0.9854, 0.8075, 0.7125));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_300: Color = Color::Rgb(::palette::Srgb::new(0.9796, 0.7024, 0.5307));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_400: Color = Color::Rgb(::palette::Srgb::new(0.9182, 0.5487, 0.1982));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_500: Color = Color::Rgb(::palette::Srgb::new(0.7613, 0.4508, 0.1596));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_600: Color = Color::Rgb(::palette::Srgb::new(0.6080, 0.3569, 0.1175));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_700: Color = Color::Rgb(::palette::Srgb::new(0.4630, 0.2664, 0.0812));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_800: Color = Color::Rgb(::palette::Srgb::new(0.3094, 0.1728, 0.0445));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_900: Color = Color::Rgb(::palette::Srgb::new(0.1846, 0.0940, 0.0152));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const PEACH_950: Color = Color::Rgb(::palette::Srgb::new(0.1295, 0.0588, 0.0078));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_50: Color = Color::Rgb(::palette::Srgb::new(0.9882, 0.9412, 0.8473));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_100: Color = Color::Rgb(::palette::Srgb::new(0.9760, 0.8863, 0.6876));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_200: Color = Color::Rgb(::palette::Srgb::new(0.9094, 0.7766, 0.3981));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_300: Color = Color::Rgb(::palette::Srgb::new(0.7762, 0.6589, 0.3348));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_400: Color = Color::Rgb(::palette::Srgb::new(0.6545, 0.5569, 0.2802));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_500: Color = Color::Rgb(::palette::Srgb::new(0.5408, 0.4589, 0.2250));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_600: Color = Color::Rgb(::palette::Srgb::new(0.4315, 0.3647, 0.1721));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_700: Color = Color::Rgb(::palette::Srgb::new(0.3254, 0.2706, 0.1218));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_800: Color = Color::Rgb(::palette::Srgb::new(0.2232, 0.1843, 0.0757));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_900: Color = Color::Rgb(::palette::Srgb::new(0.1293, 0.1059, 0.0318));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const YELLOW_950: Color = Color::Rgb(::palette::Srgb::new(0.0826, 0.0627, 0.0149));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_50: Color = Color::Rgb(::palette::Srgb::new(0.9057, 0.9726, 0.8977));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_100: Color = Color::Rgb(::palette::Srgb::new(0.7967, 0.9488, 0.7848));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_200: Color = Color::Rgb(::palette::Srgb::new(0.6515, 0.8900, 0.6317));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_300: Color = Color::Rgb(::palette::Srgb::new(0.5574, 0.7645, 0.5416));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_400: Color = Color::Rgb(::palette::Srgb::new(0.4753, 0.6546, 0.4595));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_500: Color = Color::Rgb(::palette::Srgb::new(0.3839, 0.5335, 0.3720));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_600: Color = Color::Rgb(::palette::Srgb::new(0.2981, 0.4196, 0.2902));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_700: Color = Color::Rgb(::palette::Srgb::new(0.2233, 0.3178, 0.2154));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_800: Color = Color::Rgb(::palette::Srgb::new(0.1447, 0.2158, 0.1408));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_900: Color = Color::Rgb(::palette::Srgb::new(0.0819, 0.1257, 0.0779));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const GREEN_950: Color = Color::Rgb(::palette::Srgb::new(0.0430, 0.0745, 0.0390));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_50: Color = Color::Rgb(::palette::Srgb::new(0.8632, 0.9803, 0.9568));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_100: Color = Color::Rgb(::palette::Srgb::new(0.7019, 0.9607, 0.9136));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_200: Color = Color::Rgb(::palette::Srgb::new(0.5812, 0.8861, 0.8352));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_300: Color = Color::Rgb(::palette::Srgb::new(0.4981, 0.7607, 0.7175));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_400: Color = Color::Rgb(::palette::Srgb::new(0.4213, 0.6507, 0.6116));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_500: Color = Color::Rgb(::palette::Srgb::new(0.3410, 0.5333, 0.4979));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_600: Color = Color::Rgb(::palette::Srgb::new(0.2637, 0.4155, 0.3921));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_700: Color = Color::Rgb(::palette::Srgb::new(0.1954, 0.3177, 0.2980));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_800: Color = Color::Rgb(::palette::Srgb::new(0.1254, 0.2157, 0.2000));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_900: Color = Color::Rgb(::palette::Srgb::new(0.0666, 0.1255, 0.1176));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const TEAL_950: Color = Color::Rgb(::palette::Srgb::new(0.0309, 0.0746, 0.0666));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const SAPPHIRE_50: Color = Color::Rgb(::palette::Srgb::new(0.9220, 0.9607, 0.9877));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const SAPPHIRE_100: Color = Color::Rgb(::palette::Srgb::new(0.8403, 0.9214, 0.9715));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const SAPPHIRE_200: Color = Color::Rgb(::palette::Srgb::new(0.6623, 0.8471, 0.9492));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const SAPPHIRE_300: Color = Color::Rgb(::palette::Srgb::new(0.4566, 0.7801, 0.9244));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const SAPPHIRE_400: Color = Color::Rgb(::palette::Srgb::new(0.3726, 0.6666, 0.7920));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const SAPPHIRE_500: Color = Color::Rgb(::palette::Srgb::new(0.3015, 0.5452, 0.6551));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const SAPPHIRE_600: Color = Color::Rgb(::palette::Srgb::new(0.2359, 0.4313, 0.5173));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const SAPPHIRE_700: Color = Color::Rgb(::palette::Srgb::new(0.1700, 0.3215, 0.3914));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const SAPPHIRE_800: Color = Color::Rgb(::palette::Srgb::new(0.1099, 0.2196, 0.2705));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const SAPPHIRE_900: Color = Color::Rgb(::palette::Srgb::new(0.0517, 0.1254, 0.1564));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const SAPPHIRE_950: Color = Color::Rgb(::palette::Srgb::new(0.0232, 0.0706, 0.0943));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_50: Color = Color::Rgb(::palette::Srgb::new(0.9211, 0.9451, 0.9971));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_100: Color = Color::Rgb(::palette::Srgb::new(0.8595, 0.9020, 0.9909));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_200: Color = Color::Rgb(::palette::Srgb::new(0.7098, 0.8040, 0.9883));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_300: Color = Color::Rgb(::palette::Srgb::new(0.5377, 0.7059, 0.9795));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_400: Color = Color::Rgb(::palette::Srgb::new(0.2636, 0.5961, 0.9718));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_500: Color = Color::Rgb(::palette::Srgb::new(0.1461, 0.4942, 0.8387));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_600: Color = Color::Rgb(::palette::Srgb::new(0.1082, 0.3960, 0.6752));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_700: Color = Color::Rgb(::palette::Srgb::new(0.0707, 0.2902, 0.5058));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_800: Color = Color::Rgb(::palette::Srgb::new(0.0400, 0.2001, 0.3604));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_900: Color = Color::Rgb(::palette::Srgb::new(0.0110, 0.1097, 0.2124));

    #[allow(clippy::excessive_precision, clippy::approx_constant)]
    pub const BLUE_950: Color = Color::Rgb(::palette::Srgb::new(0.0074, 0.0666, 0.1457));

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

    pub const ROSEWATER: ThemeArray<11> = ThemeArray([
        Self::ROSEWATER_50,
        Self::ROSEWATER_100,
        Self::ROSEWATER_200,
        Self::ROSEWATER_300,
        Self::ROSEWATER_400,
        Self::ROSEWATER_500,
        Self::ROSEWATER_600,
        Self::ROSEWATER_700,
        Self::ROSEWATER_800,
        Self::ROSEWATER_900,
        Self::ROSEWATER_950,
    ]);

    pub const FLAMINGO: ThemeArray<11> = ThemeArray([
        Self::FLAMINGO_50,
        Self::FLAMINGO_100,
        Self::FLAMINGO_200,
        Self::FLAMINGO_300,
        Self::FLAMINGO_400,
        Self::FLAMINGO_500,
        Self::FLAMINGO_600,
        Self::FLAMINGO_700,
        Self::FLAMINGO_800,
        Self::FLAMINGO_900,
        Self::FLAMINGO_950,
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

    pub const MAUVE: ThemeArray<11> = ThemeArray([
        Self::MAUVE_50,
        Self::MAUVE_100,
        Self::MAUVE_200,
        Self::MAUVE_300,
        Self::MAUVE_400,
        Self::MAUVE_500,
        Self::MAUVE_600,
        Self::MAUVE_700,
        Self::MAUVE_800,
        Self::MAUVE_900,
        Self::MAUVE_950,
    ]);

    pub const MAROON: ThemeArray<11> = ThemeArray([
        Self::MAROON_50,
        Self::MAROON_100,
        Self::MAROON_200,
        Self::MAROON_300,
        Self::MAROON_400,
        Self::MAROON_500,
        Self::MAROON_600,
        Self::MAROON_700,
        Self::MAROON_800,
        Self::MAROON_900,
        Self::MAROON_950,
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

    pub const SAPPHIRE: ThemeArray<11> = ThemeArray([
        Self::SAPPHIRE_50,
        Self::SAPPHIRE_100,
        Self::SAPPHIRE_200,
        Self::SAPPHIRE_300,
        Self::SAPPHIRE_400,
        Self::SAPPHIRE_500,
        Self::SAPPHIRE_600,
        Self::SAPPHIRE_700,
        Self::SAPPHIRE_800,
        Self::SAPPHIRE_900,
        Self::SAPPHIRE_950,
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

    pub const ALL_COLORS: [NamedColor<'_>; 132] = [
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
            group: Cow::Borrowed("rosewater"),
            color: Self::ROSEWATER_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("rosewater"),
            color: Self::ROSEWATER_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("rosewater"),
            color: Self::ROSEWATER_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("rosewater"),
            color: Self::ROSEWATER_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("rosewater"),
            color: Self::ROSEWATER_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("rosewater"),
            color: Self::ROSEWATER_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("rosewater"),
            color: Self::ROSEWATER_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("rosewater"),
            color: Self::ROSEWATER_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("rosewater"),
            color: Self::ROSEWATER_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("rosewater"),
            color: Self::ROSEWATER_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("rosewater"),
            color: Self::ROSEWATER_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("flamingo"),
            color: Self::FLAMINGO_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("flamingo"),
            color: Self::FLAMINGO_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("flamingo"),
            color: Self::FLAMINGO_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("flamingo"),
            color: Self::FLAMINGO_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("flamingo"),
            color: Self::FLAMINGO_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("flamingo"),
            color: Self::FLAMINGO_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("flamingo"),
            color: Self::FLAMINGO_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("flamingo"),
            color: Self::FLAMINGO_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("flamingo"),
            color: Self::FLAMINGO_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("flamingo"),
            color: Self::FLAMINGO_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("flamingo"),
            color: Self::FLAMINGO_950,
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
            group: Cow::Borrowed("mauve"),
            color: Self::MAUVE_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("mauve"),
            color: Self::MAUVE_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("mauve"),
            color: Self::MAUVE_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("mauve"),
            color: Self::MAUVE_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("mauve"),
            color: Self::MAUVE_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("mauve"),
            color: Self::MAUVE_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("mauve"),
            color: Self::MAUVE_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("mauve"),
            color: Self::MAUVE_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("mauve"),
            color: Self::MAUVE_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("mauve"),
            color: Self::MAUVE_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("mauve"),
            color: Self::MAUVE_950,
        },
        NamedColor {
            variant: Cow::Borrowed("50"),
            group: Cow::Borrowed("maroon"),
            color: Self::MAROON_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("maroon"),
            color: Self::MAROON_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("maroon"),
            color: Self::MAROON_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("maroon"),
            color: Self::MAROON_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("maroon"),
            color: Self::MAROON_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("maroon"),
            color: Self::MAROON_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("maroon"),
            color: Self::MAROON_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("maroon"),
            color: Self::MAROON_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("maroon"),
            color: Self::MAROON_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("maroon"),
            color: Self::MAROON_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("maroon"),
            color: Self::MAROON_950,
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
            group: Cow::Borrowed("sapphire"),
            color: Self::SAPPHIRE_50,
        },
        NamedColor {
            variant: Cow::Borrowed("100"),
            group: Cow::Borrowed("sapphire"),
            color: Self::SAPPHIRE_100,
        },
        NamedColor {
            variant: Cow::Borrowed("200"),
            group: Cow::Borrowed("sapphire"),
            color: Self::SAPPHIRE_200,
        },
        NamedColor {
            variant: Cow::Borrowed("300"),
            group: Cow::Borrowed("sapphire"),
            color: Self::SAPPHIRE_300,
        },
        NamedColor {
            variant: Cow::Borrowed("400"),
            group: Cow::Borrowed("sapphire"),
            color: Self::SAPPHIRE_400,
        },
        NamedColor {
            variant: Cow::Borrowed("500"),
            group: Cow::Borrowed("sapphire"),
            color: Self::SAPPHIRE_500,
        },
        NamedColor {
            variant: Cow::Borrowed("600"),
            group: Cow::Borrowed("sapphire"),
            color: Self::SAPPHIRE_600,
        },
        NamedColor {
            variant: Cow::Borrowed("700"),
            group: Cow::Borrowed("sapphire"),
            color: Self::SAPPHIRE_700,
        },
        NamedColor {
            variant: Cow::Borrowed("800"),
            group: Cow::Borrowed("sapphire"),
            color: Self::SAPPHIRE_800,
        },
        NamedColor {
            variant: Cow::Borrowed("900"),
            group: Cow::Borrowed("sapphire"),
            color: Self::SAPPHIRE_900,
        },
        NamedColor {
            variant: Cow::Borrowed("950"),
            group: Cow::Borrowed("sapphire"),
            color: Self::SAPPHIRE_950,
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
    ];
}
