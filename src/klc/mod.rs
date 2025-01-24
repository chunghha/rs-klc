/**
 * The MIT License (MIT)
 *
 * Copyright (c) 2018 usingsky(usingsky@gmail.com)
 * Copyright (c) 2022 chunghha(chunghha@users.noreply.github.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#[derive(Debug, Default)]
pub struct LunarSolarConverter {
    lunar_year: i32,
    lunar_month: u32,
    lunar_day: u32,
    is_intercalation: bool,
    solar_year: u32,
    solar_month: u32,
    solar_day: u32,
    gapja_year_inx: [Option<usize>; 3],
    gapja_month_inx: [Option<usize>; 3],
    gapja_day_inx: [Option<usize>; 3],
}

const KOREAN_LUNAR_MIN_VALUE: u32 = 13910101;
const KOREAN_LUNAR_MAX_VALUE: u32 = 20501118;
const KOREAN_SOLAR_MIN_VALUE: u32 = 13910205;
const KOREAN_SOLAR_MAX_VALUE: u32 = 20501231;

const KOREAN_LUNAR_BASE_YEAR: i32 = 1391;
const SOLAR_LUNAR_DAY_DIFF: u32 = 35;

const LUNAR_SMALL_MONTH_DAY: u32 = 29;
const LUNAR_BIG_MONTH_DAY: u32 = 30;
const SOLAR_SMALL_YEAR_DAY: u32 = 365;
const SOLAR_BIG_YEAR_DAY: u32 = 366;

const SOLAR_DAYS: [u32; 13] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 29];
const KOREAN_CHEONGAN: [char; 10] = [
    '\u{ac11}', '\u{c744}', '\u{bcd1}', '\u{c815}', '\u{bb34}', '\u{ae30}', '\u{acbd}', '\u{c2e0}',
    '\u{c784}', '\u{acc4}',
];
const KOREAN_GANJI: [char; 12] = [
    '\u{c790}', '\u{cd95}', '\u{c778}', '\u{bb18}', '\u{c9c4}', '\u{c0ac}', '\u{c624}', '\u{bbf8}',
    '\u{c2e0}', '\u{c720}', '\u{c220}', '\u{d574}',
];
const KOREAN_GAPJA_UNIT: [char; 3] = ['\u{b144}', '\u{c6d4}', '\u{c77c}'];

const CHINESE_CHEONGAN: [char; 10] = [
    '\u{7532}', '\u{4e59}', '\u{4e19}', '\u{4e01}', '\u{620a}', '\u{5df1}', '\u{5e9a}', '\u{8f9b}',
    '\u{58ec}', '\u{7678}',
];
const CHINESE_GANJI: [char; 12] = [
    '\u{5b50}', '\u{4e11}', '\u{5bc5}', '\u{536f}', '\u{8fb0}', '\u{5df3}', '\u{5348}', '\u{672a}',
    '\u{7533}', '\u{9149}', '\u{620c}', '\u{4ea5}',
];
const CHINESE_GAPJA_UNIT: [char; 3] = ['\u{5e74}', '\u{6708}', '\u{65e5}'];

const INTERCALATION_STR: [char; 2] = ['\u{c724}', '\u{958f}'];

const KOREAN_LUNAR_DATA: [u32; 660] = [
    0x82c40653, 0xc301c6a9, 0x82c405aa, 0x82c60ab5, 0x830092bd, 0xc2c402b6, 0x82c60c37, 0x82fe552e,
    0x82c40c96, 0xc2c60e4b, 0x82fe3752, 0x82c60daa, 0x8301b5b4, 0xc2c6056d, 0x82c402ae, 0x83007a3d,
    0x82c40a2d, 0xc2c40d15, 0x83004d95, 0x82c40b52, 0x8300cb69, 0xc2c60ada, 0x82c6055d, 0x8301925b,
    0x82c4045b, 0xc2c40a2b, 0x83005aab, 0x82c40a95, 0x82c40b52, 0xc3001eaa, 0x82c60ab6, 0x8300c55b,
    0x82c604b7, 0xc2c40457, 0x83007537, 0x82c4052b, 0x82c40695, 0xc3014695, 0x82c405aa, 0x8300c9b5,
    0x82c60a6e, 0xc2c404ae, 0x83008a5e, 0x82c40a56, 0x82c40d2a, 0xc3006eaa, 0x82c60d55, 0x82c4056a,
    0x8301295a, 0xc2c6095e, 0x8300b4af, 0x82c4049b, 0x82c40a4d, 0xc3007d2e, 0x82c40b2a, 0x82c60b55,
    0x830045d5, 0xc2c402da, 0x82c6095b, 0x83011157, 0x82c4049b, 0xc3009a4f, 0x82c4064b, 0x82c406a9,
    0x83006aea, 0xc2c606b5, 0x82c402b6, 0x83002aae, 0x82c60937, 0xc2ffb496, 0x82c40c96, 0x82c60e4b,
    0x82fe76b2, 0xc2c60daa, 0x82c605ad, 0x8300336d, 0x82c4026e, 0xc2c4092e, 0x83002d2d, 0x82c40c95,
    0x83009d4d, 0xc2c40b4a, 0x82c60b69, 0x8301655a, 0x82c6055b, 0xc2c4025d, 0x83002a5b, 0x82c4092b,
    0x8300aa97, 0xc2c40695, 0x82c4074a, 0x83008b5a, 0x82c60ab6, 0xc2c6053b, 0x830042b7, 0x82c40257,
    0x82c4052b, 0xc3001d2b, 0x82c40695, 0x830096ad, 0x82c405aa, 0xc2c60ab5, 0x830054ed, 0x82c404ae,
    0x82c60a57, 0xc2ff344e, 0x82c40d2a, 0x8301bd94, 0x82c60b55, 0xc2c4056a, 0x8300797a, 0x82c6095d,
    0x82c404ae, 0xc3004a9b, 0x82c40a4d, 0x82c40d25, 0x83011aaa, 0xc2c60b55, 0x8300956d, 0x82c402da,
    0x82c6095b, 0xc30054b7, 0x82c40497, 0x82c40a4b, 0x83004b4b, 0xc2c406a9, 0x8300cad5, 0x82c605b5,
    0x82c402b6, 0xc300895e, 0x82c6092f, 0x82c40497, 0x82fe4696, 0xc2c40d4a, 0x8300cea5, 0x82c60d69,
    0x82c6056d, 0xc301a2b5, 0x82c4026e, 0x82c4052e, 0x83006cad, 0xc2c40c95, 0x82c40d4a, 0x83002f4a,
    0x82c60b59, 0xc300c56d, 0x82c6055b, 0x82c4025d, 0x8300793b, 0xc2c4092b, 0x82c40a95, 0x83015b15,
    0x82c406ca, 0xc2c60ad5, 0x830112b6, 0x82c604bb, 0x8300925f, 0xc2c40257, 0x82c4052b, 0x82fe6aaa,
    0x82c60e95, 0xc2c406aa, 0x83003baa, 0x82c60ab5, 0x8300b4b7, 0xc2c404ae, 0x82c60a57, 0x82fe752d,
    0x82c40d26, 0xc2c60d95, 0x830055d5, 0x82c4056a, 0x82c6096d, 0xc300255d, 0x82c404ae, 0x8300aa4f,
    0x82c40a4d, 0xc2c40d25, 0x83006d69, 0x82c60b55, 0x82c4035a, 0xc3002aba, 0x82c6095b, 0x8301c49b,
    0x82c40497, 0xc2c40a4b, 0x83008b2b, 0x82c406a5, 0x82c406d4, 0xc3034ab5, 0x82c402b6, 0x82c60937,
    0x8300252f, 0xc2c40497, 0x82fe964e, 0x82c40d4a, 0x82c60ea5, 0xc30166a9, 0x82c6056d, 0x82c402b6,
    0x8301385e, 0xc2c4092e, 0x8300bc97, 0x82c40a95, 0x82c40d4a, 0xc3008daa, 0x82c60b4d, 0x82c6056b,
    0x830042db, 0xc2c4025d, 0x82c4092d, 0x83002d33, 0x82c40a95, 0xc3009b4d, 0x82c406aa, 0x82c60ad5,
    0x83006575, 0xc2c604bb, 0x82c4025b, 0x83013457, 0x82c4052b, 0xc2ffba94, 0x82c60e95, 0x82c406aa,
    0x83008ada, 0xc2c609b5, 0x82c404b6, 0x83004aae, 0x82c60a4f, 0xc2c20526, 0x83012d26, 0x82c60d55,
    0x8301a5a9, 0xc2c4056a, 0x82c6096d, 0x8301649d, 0x82c4049e, 0xc2c40a4d, 0x83004d4d, 0x82c40d25,
    0x8300bd53, 0xc2c40b54, 0x82c60b5a, 0x8301895a, 0x82c6095b, 0xc2c4049b, 0x83004a97, 0x82c40a4b,
    0x82c40aa5, 0xc3001ea5, 0x82c406d4, 0x8302badb, 0x82c402b6, 0xc2c60937, 0x830064af, 0x82c40497,
    0x82c4064b, 0xc2fe374a, 0x82c60da5, 0x8300b6b5, 0x82c6056d, 0xc2c402ba, 0x8300793e, 0x82c4092e,
    0x82c40c96, 0xc3015d15, 0x82c40d4a, 0x82c60da5, 0x83013555, 0xc2c4056a, 0x83007a7a, 0x82c60a5d,
    0x82c4092d, 0xc3006aab, 0x82c40a95, 0x82c40b4a, 0x83004baa, 0xc2c60ad5, 0x82c4055a, 0x830128ba,
    0x82c60a5b, 0xc3007537, 0x82c4052b, 0x82c40693, 0x83015715, 0xc2c406aa, 0x82c60ad9, 0x830035b5,
    0x82c404b6, 0xc3008a5e, 0x82c40a4e, 0x82c40d26, 0x83006ea6, 0xc2c40d52, 0x82c60daa, 0x8301466a,
    0x82c6056d, 0xc2c404ae, 0x83003a9d, 0x82c40a4d, 0x83007d2b, 0xc2c40b25, 0x82c40d52, 0x83015d54,
    0x82c60b5a, 0xc2c6055d, 0x8300355b, 0x82c4049d, 0x83007657, 0x82c40a4b, 0x82c40aa5, 0x83006b65,
    0x82c406d2, 0xc2c60ada, 0x830045b6, 0x82c60937, 0x82c40497, 0xc3003697, 0x82c40a4d, 0x82fe76aa,
    0x82c60da5, 0xc2c405aa, 0x83005aec, 0x82c60aae, 0x82c4092e, 0xc3003d2e, 0x82c40c96, 0x83018d45,
    0x82c40d4a, 0xc2c60d55, 0x83016595, 0x82c4056a, 0x82c60a6d, 0xc300455d, 0x82c4052d, 0x82c40a95,
    0x83003e95, 0xc2c40b4a, 0x83017b4a, 0x82c609d5, 0x82c4055a, 0xc3015a3a, 0x82c60a5b, 0x82c4052b,
    0x83014a17, 0xc2c40693, 0x830096ab, 0x82c406aa, 0x82c60ab5, 0xc30064f5, 0x82c404b6, 0x82c60a57,
    0x82fe452e, 0xc2c40d16, 0x82c60e93, 0x82fe3752, 0x82c60daa, 0xc30175aa, 0x82c6056d, 0x82c404ae,
    0x83015a1b, 0xc2c40a2d, 0x82c40d15, 0x83004da5, 0x82c40b52, 0xc3009d6a, 0x82c60ada, 0x82c6055d,
    0x8301629b, 0xc2c4045b, 0x82c40a2b, 0x83005b2b, 0x82c40a95, 0xc2c40b52, 0x83012ab2, 0x82c60ad6,
    0x83017556, 0xc2c60537, 0x82c40457, 0x83005657, 0x82c4052b, 0xc2c40695, 0x83003795, 0x82c405aa,
    0x8300aab6, 0xc2c60a6d, 0x82c404ae, 0x8300696e, 0x82c40a56, 0xc2c40d2a, 0x83005eaa, 0x82c60d55,
    0x82c405aa, 0xc3003b6a, 0x82c60a6d, 0x830074bd, 0x82c404ab, 0xc2c40a8d, 0x83005d55, 0x82c40b2a,
    0x82c60b55, 0xc30045d5, 0x82c404da, 0x82c6095d, 0x83002557, 0xc2c4049b, 0x83006a97, 0x82c4064b,
    0x82c406a9, 0x83004baa, 0x82c606b5, 0x82c402ba, 0x83002ab6, 0xc2c60937, 0x82fe652e, 0x82c40d16,
    0x82c60e4b, 0xc2fe56d2, 0x82c60da9, 0x82c605b5, 0x8300336d, 0xc2c402ae, 0x82c40a2e, 0x83002e2d,
    0x82c40c95, 0xc3006d55, 0x82c40b52, 0x82c60b69, 0x830045da, 0xc2c6055d, 0x82c4025d, 0x83003a5b,
    0x82c40a2b, 0xc3017a8b, 0x82c40a95, 0x82c40b4a, 0x83015b2a, 0xc2c60ad5, 0x82c6055b, 0x830042b7,
    0x82c40257, 0xc300952f, 0x82c4052b, 0x82c40695, 0x830066d5, 0xc2c405aa, 0x82c60ab5, 0x8300456d,
    0x82c404ae, 0xc2c60a57, 0x82ff3456, 0x82c40d2a, 0x83017e8a, 0xc2c60d55, 0x82c405aa, 0x83005ada,
    0x82c6095d, 0xc2c404ae, 0x83004aab, 0x82c40a4d, 0x83008d2b, 0xc2c40b29, 0x82c60b55, 0x83007575,
    0x82c402da, 0xc2c6095d, 0x830054d7, 0x82c4049b, 0x82c40a4b, 0xc3013a4b, 0x82c406a9, 0x83008ad9,
    0x82c606b5, 0xc2c402b6, 0x83015936, 0x82c60937, 0x82c40497, 0xc2fe4696, 0x82c40e4a, 0x8300aea6,
    0x82c60da9, 0xc2c605ad, 0x830162ad, 0x82c402ae, 0x82c4092e, 0xc3005cad, 0x82c40c95, 0x82c40d4a,
    0x83013d4a, 0xc2c60b69, 0x8300757a, 0x82c6055b, 0x82c4025d, 0xc300595b, 0x82c4092b, 0x82c40a95,
    0x83004d95, 0xc2c40b4a, 0x82c60b55, 0x830026d5, 0x82c6055b, 0xc3006277, 0x82c40257, 0x82c4052b,
    0x82fe5aaa, 0xc2c60e95, 0x82c406aa, 0x83003baa, 0x82c60ab5, 0x830084bd, 0x82c404ae, 0x82c60a57,
    0x82fe554d, 0xc2c40d26, 0x82c60d95, 0x83014655, 0x82c4056a, 0xc2c609ad, 0x8300255d, 0x82c404ae,
    0x83006a5b, 0xc2c40a4d, 0x82c40d25, 0x83005da9, 0x82c60b55, 0xc2c4056a, 0x83002ada, 0x82c6095d,
    0x830074bb, 0xc2c4049b, 0x82c40a4b, 0x83005b4b, 0x82c406a9, 0xc2c40ad4, 0x83024bb5, 0x82c402b6,
    0x82c6095b, 0xc3002537, 0x82c40497, 0x82fe6656, 0x82c40e4a, 0xc2c60ea5, 0x830156a9, 0x82c605b5,
    0x82c402b6, 0xc30138ae, 0x82c4092e, 0x83017c8d, 0x82c40c95, 0xc2c40d4a, 0x83016d8a, 0x82c60b69,
    0x82c6056d, 0xc301425b, 0x82c4025d, 0x82c4092d, 0x83002d2b, 0xc2c40a95, 0x83007d55, 0x82c40b4a,
    0x82c60b55, 0xc3015555, 0x82c604db, 0x82c4025b, 0x83013857, 0xc2c4052b, 0x83008a9b, 0x82c40695,
    0x82c406aa, 0xc3006aea, 0x82c60ab5, 0x82c404b6, 0x83004aae, 0xc2c60a57, 0x82c40527, 0x82fe3726,
    0x82c60d95, 0xc30076b5, 0x82c4056a, 0x82c609ad, 0x830054dd, 0xc2c404ae, 0x82c40a4e, 0x83004d4d,
    0x82c40d25, 0xc3008d59, 0x82c40b54, 0x82c60d6a, 0x8301695a, 0xc2c6095b, 0x82c4049b, 0x83004a9b,
    0x82c40a4b, 0xc300ab27, 0x82c406a5, 0x82c406d4, 0x83026b75, 0xc2c402b6, 0x82c6095b, 0x830054b7,
    0x82c40497, 0xc2c4064b, 0x82fe374a, 0x82c60ea5, 0x830086d9, 0xc2c605ad, 0x82c402b6, 0x8300596e,
    0x82c4092e, 0xc2c40c96, 0x83004e95, 0x82c40d4a, 0x82c60da5, 0xc3002755, 0x82c4056c, 0x83027abb,
    0x82c4025d, 0xc2c4092d, 0x83005cab, 0x82c40a95, 0x82c40b4a, 0xc3013b4a, 0x82c60b55, 0x8300955d,
    0x82c404ba, 0xc2c60a5b, 0x83005557, 0x82c4052b, 0x82c40a95, 0xc3004b95, 0x82c406aa, 0x82c60ad5,
    0x830026b5, 0xc2c404b6, 0x83006a6e, 0x82c60a57, 0x82c40527, 0xc2fe56a6, 0x82c60d93, 0x82c405aa,
    0x83003b6a, 0xc2c6096d, 0x8300b4af, 0x82c404ae, 0x82c40a4d, 0xc3016d0d, 0x82c40d25, 0x82c40d52,
    0x83005dd4, 0xc2c60b6a, 0x82c6096d, 0x8300255b, 0x82c4049b, 0xc3007a57, 0x82c40a4b, 0x82c40b25,
    0x83015b25, 0xc2c406d4, 0x82c60ada, 0x830138b6,
];

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DayOfWeek {
    Monday,    // Corresponds to JDN % 7 = 0
    Tuesday,   // Corresponds to JDN % 7 = 1
    Wednesday, // Corresponds to JDN % 7 = 2
    Thursday,  // Corresponds to JDN % 7 = 3
    Friday,    // Corresponds to JDN % 7 = 4
    Saturday,  // Corresponds to JDN % 7 = 5
    Sunday,    // Corresponds to JDN % 7 = 6
}

impl LunarSolarConverter {
    pub fn new() -> Self {
        LunarSolarConverter::default()
    }

    fn get_lunar_data(year: i32) -> u32 {
        if year < KOREAN_LUNAR_BASE_YEAR {
            0
        } else {
            *KOREAN_LUNAR_DATA
                .get((year - KOREAN_LUNAR_BASE_YEAR) as usize)
                .unwrap_or(&0)
        }
    }

    fn get_lunar_intercalation_month(lunar_data: u32) -> u32 {
        (lunar_data >> 12) & 0x000F
    }

    fn shift_lunar_days(year: i32) -> u32 {
        let lunar_data = Self::get_lunar_data(year);
        if lunar_data == 0 {
            return 0;
        }

        let mut total_days = 0;
        let month_bits = lunar_data & 0xFFF;

        for month in 1..=12 {
            if ((month_bits >> (12 - month)) & 0x01) > 0 {
                total_days += LUNAR_BIG_MONTH_DAY;
            } else {
                total_days += LUNAR_SMALL_MONTH_DAY;
            }
        }

        let intercalation_month = Self::get_lunar_intercalation_month(lunar_data);
        if intercalation_month > 0 {
            if ((lunar_data >> 16) & 0x01) > 0 {
                total_days += LUNAR_BIG_MONTH_DAY;
            } else {
                total_days += LUNAR_SMALL_MONTH_DAY;
            }
        }

        total_days
    }

    fn get_lunar_days(year: i32, month: u32, is_intercalation: bool) -> u32 {
        let mut days = 0;
        if year < KOREAN_LUNAR_BASE_YEAR {
            return 0;
        }
        let lunar_data = Self::get_lunar_data(year);
        let intercalation_month = Self::get_lunar_intercalation_month(lunar_data);

        if is_intercalation && intercalation_month == month {
            if ((lunar_data >> 16) & 0x01) > 0 {
                days = LUNAR_BIG_MONTH_DAY;
            } else {
                days = LUNAR_SMALL_MONTH_DAY;
            }
        } else if month > 0 && month < 13 {
            if ((lunar_data >> (12 - month)) & 0x01) > 0 {
                days = LUNAR_BIG_MONTH_DAY;
            } else {
                days = LUNAR_SMALL_MONTH_DAY;
            }
        }

        days
    }

    fn get_lunar_days_before_base_year(year: i32) -> u32 {
        let mut days = 0;

        for base_year in KOREAN_LUNAR_BASE_YEAR..year {
            days += Self::shift_lunar_days(base_year);
        }

        days
    }

    fn get_lunar_days_before_base_month(year: i32, month: u32, is_intercalation: bool) -> u32 {
        let mut days = 0;
        if year < KOREAN_LUNAR_BASE_YEAR || month == 0 {
            return 0;
        }

        let lunar_data = Self::get_lunar_data(year);
        let intercalation_month = Self::get_lunar_intercalation_month(lunar_data);

        for base_month in 1..month {
            days += Self::get_lunar_days(year, base_month, false);

            if intercalation_month > 0 && intercalation_month == base_month {
                days += Self::get_lunar_days(year, intercalation_month, true);
            }
        }

        if is_intercalation && intercalation_month == month {
            days += Self::get_lunar_days(year, month, false);
        }

        days
    }

    fn get_lunar_abs_days(year: i32, month: u32, day: u32, is_intercalation: bool) -> u32 {
        if year < KOREAN_LUNAR_BASE_YEAR {
            0
        } else {
            Self::get_lunar_days_before_base_year(year)
                + Self::get_lunar_days_before_base_month(year, month, is_intercalation)
                + day
        }
    }

    fn is_gregorian_leap(year: i32) -> bool {
        if year <= 1582 {
            // Before Gregorian reform, Julian calendar used
            year % 4 == 0
        } else {
            // Gregorian calendar rules
            (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
        }
    }

    fn shift_solar_days(year: i32) -> u32 {
        let mut days;

        // Use standard Gregorian leap year calculation
        if Self::is_gregorian_leap(year) {
            days = SOLAR_BIG_YEAR_DAY;
        } else {
            days = SOLAR_SMALL_YEAR_DAY;
        }

        if year == 1582 {
            days -= 10;
        }

        days
    }

    fn get_solar_days(year: i32, month: u32) -> u32 {
        let mut days = 0;

        if year < KOREAN_LUNAR_BASE_YEAR {
            return 0;
        }

        // Use standard Gregorian leap year calculation for February
        if month == 2 && Self::is_gregorian_leap(year) {
            days = *SOLAR_DAYS.get(12).unwrap_or(&0); // Index 12 is 29
        } else if month > 0 && month < 13 {
            days = *SOLAR_DAYS.get((month - 1) as usize).unwrap_or(&0);
        }

        if year == 1582 && month == 10 {
            days -= 10;
        }

        days
    }

    fn get_solar_day_before_base_year(year: i32) -> u32 {
        let mut days = 0;

        for base_year in KOREAN_LUNAR_BASE_YEAR..year {
            days += Self::shift_solar_days(base_year);
        }

        days
    }

    fn get_solar_days_before_base_month(year: i32, month: u32) -> u32 {
        let mut days = 0;

        if year < KOREAN_LUNAR_BASE_YEAR {
            return 0;
        }

        for base_month in 1..month {
            days += Self::get_solar_days(year, base_month);
        }

        days
    }

    fn get_solar_abs_days(year: i32, month: u32, day: u32) -> u32 {
        if year < KOREAN_LUNAR_BASE_YEAR {
            0
        } else {
            let mut days = Self::get_solar_day_before_base_year(year)
                + Self::get_solar_days_before_base_month(year, month)
                + day;
            days -= SOLAR_LUNAR_DAY_DIFF;

            days
        }
    }

    fn set_solar_date_by_lunar_date(
        &mut self,
        lunar_year: i32,
        lunar_month: u32,
        lunar_day: u32,
        is_intercalation: bool,
    ) {
        let abs_days =
            Self::get_lunar_abs_days(lunar_year, lunar_month, lunar_day, is_intercalation);

        if abs_days < Self::get_solar_abs_days(lunar_year + 1, 1, 1) {
            self.solar_year = lunar_year as u32;
        } else {
            self.solar_year = (lunar_year + 1) as u32;
        }

        for month in (1..=12).rev() {
            let abs_days_by_month = Self::get_solar_abs_days(self.solar_year as i32, month, 1);

            if abs_days >= abs_days_by_month {
                self.solar_month = month;
                self.solar_day = abs_days - abs_days_by_month + 1;
                break;
            }
        }

        if self.solar_year == 1582 && self.solar_month == 10 && self.solar_day > 4 {
            self.solar_day += 10;
        }
    }

    fn set_lunar_date_by_solar_date(&mut self, solar_year: u32, solar_month: u32, solar_day: u32) {
        let abs_days = Self::get_solar_abs_days(solar_year as i32, solar_month, solar_day);

        self.is_intercalation = false;

        if abs_days >= Self::get_lunar_abs_days(solar_year as i32, 1, 1, false) {
            self.lunar_year = solar_year as i32;
        } else {
            self.lunar_year = solar_year as i32 - 1;
        }

        for month in (1..=12).rev() {
            let abs_days_by_month = Self::get_lunar_abs_days(self.lunar_year, month, 1, false);

            if abs_days >= abs_days_by_month {
                self.lunar_month = month;

                if Self::get_lunar_intercalation_month(Self::get_lunar_data(self.lunar_year))
                    == month
                {
                    self.is_intercalation =
                        abs_days >= Self::get_lunar_abs_days(self.lunar_year, month, 1, true);
                }

                self.lunar_day = abs_days
                    - Self::get_lunar_abs_days(
                        self.lunar_year,
                        self.lunar_month,
                        1,
                        self.is_intercalation,
                    )
                    + 1;

                break;
            }
        }
    }

    fn is_valid_min(is_lunar: bool, date_value: u32) -> bool {
        if is_lunar {
            KOREAN_LUNAR_MIN_VALUE <= date_value
        } else {
            KOREAN_SOLAR_MIN_VALUE <= date_value
        }
    }

    fn is_valid_max(is_lunar: bool, date_value: u32) -> bool {
        if is_lunar {
            KOREAN_LUNAR_MAX_VALUE >= date_value
        } else {
            KOREAN_SOLAR_MAX_VALUE >= date_value
        }
    }

    fn check_valid_date(
        is_lunar: bool,
        is_intercalation: bool,
        year: u32,
        month: u32,
        day: u32,
    ) -> bool {
        let mut is_valid = false;
        let date_value = year * 10000 + month * 100 + day;

        //1582. 10. 5 ~ 1582. 10. 14 is not enable
        if Self::is_valid_min(is_lunar, date_value) && Self::is_valid_max(is_lunar, date_value) {
            let mut day_limit;

            if month > 0 && month < 13 && day > 0 {
                if is_lunar {
                    if is_intercalation
                        && Self::get_lunar_intercalation_month(Self::get_lunar_data(year as i32))
                            != month
                    {
                        return false;
                    }
                    day_limit = Self::get_lunar_days(year as i32, month, is_intercalation);
                } else {
                    day_limit = Self::get_solar_days(year as i32, month);
                }

                if !is_lunar && year == 1582 && month == 10 {
                    if day > 4 && day < 15 {
                        return false;
                    } else {
                        day_limit += 10;
                    }
                }

                if day <= day_limit {
                    is_valid = true;
                }
            }
        }

        is_valid
    }

    pub fn set_lunar_date(
        &mut self,
        lunar_year: i32,
        lunar_month: u32,
        lunar_day: u32,
        is_intercalation: bool,
    ) -> bool {
        let mut is_valid = false;

        if Self::check_valid_date(
            true,
            is_intercalation,
            lunar_year as u32,
            lunar_month,
            lunar_day,
        ) {
            self.is_intercalation = is_intercalation
                && (Self::get_lunar_intercalation_month(Self::get_lunar_data(lunar_year))
                    == lunar_month);
            self.set_solar_date_by_lunar_date(
                lunar_year,
                lunar_month,
                lunar_day,
                self.is_intercalation,
            );

            is_valid = true;
        }

        is_valid
    }

    pub fn set_solar_date(&mut self, solar_year: u32, solar_month: u32, solar_day: u32) -> bool {
        let mut is_valid = false;

        if Self::check_valid_date(false, false, solar_year, solar_month, solar_day) {
            self.set_lunar_date_by_solar_date(solar_year, solar_month, solar_day);
            is_valid = true;
        }

        is_valid
    }

    fn get_gapja(&mut self) {
        let abs_days = Self::get_lunar_abs_days(
            self.lunar_year,
            self.lunar_month,
            self.lunar_day,
            self.is_intercalation,
        );

        if abs_days > 0 {
            self.gapja_year_inx[0] = Some(
                ((self.lunar_year + 7) - KOREAN_LUNAR_BASE_YEAR) as usize % KOREAN_CHEONGAN.len(),
            );
            self.gapja_year_inx[1] = Some(
                ((self.lunar_year + 7) - KOREAN_LUNAR_BASE_YEAR) as usize % KOREAN_GANJI.len(),
            );

            let mut month_count = self.lunar_month;
            month_count += 12 * ((self.lunar_year - KOREAN_LUNAR_BASE_YEAR) as u32);
            self.gapja_month_inx[0] = Some((month_count + 5) as usize % KOREAN_CHEONGAN.len());
            self.gapja_month_inx[1] = Some((month_count + 1) as usize % KOREAN_GANJI.len());

            self.gapja_day_inx[0] = Some((abs_days + 4) as usize % KOREAN_CHEONGAN.len());
            self.gapja_day_inx[1] = Some(abs_days as usize % KOREAN_GANJI.len());
        } else {
            self.gapja_year_inx = [None, None, None];
            self.gapja_month_inx = [None, None, None];
            self.gapja_day_inx = [None, None, None];
        }
    }

    pub fn get_gapja_string(&mut self) -> String {
        self.get_gapja();

        let mut gapja_string = String::new();

        if let (Some(year_cheongan), Some(year_ganji)) =
            (self.gapja_year_inx[0], self.gapja_year_inx[1])
        {
            gapja_string.push(KOREAN_CHEONGAN[year_cheongan]);
            gapja_string.push(KOREAN_GANJI[year_ganji]);
            gapja_string.push(KOREAN_GAPJA_UNIT[0]);
        } else {
            return "".to_string();
        }

        gapja_string.push(' ');

        if let (Some(month_cheongan), Some(month_ganji)) =
            (self.gapja_month_inx[0], self.gapja_month_inx[1])
        {
            gapja_string.push(KOREAN_CHEONGAN[month_cheongan]);
            gapja_string.push(KOREAN_GANJI[month_ganji]);
            gapja_string.push(KOREAN_GAPJA_UNIT[1]);
        } else {
            return "".to_string();
        }

        gapja_string.push(' ');

        if let (Some(day_cheongan), Some(day_ganji)) =
            (self.gapja_day_inx[0], self.gapja_day_inx[1])
        {
            gapja_string.push(KOREAN_CHEONGAN[day_cheongan]);
            gapja_string.push(KOREAN_GANJI[day_ganji]);
            gapja_string.push(KOREAN_GAPJA_UNIT[2]);
        } else {
            return "".to_string();
        }

        if self.is_intercalation {
            gapja_string.push_str(" (");
            gapja_string.push(INTERCALATION_STR[0]);
            gapja_string.push(KOREAN_GAPJA_UNIT[1]);
            gapja_string.push(')');
        }

        gapja_string
    }

    pub fn get_chinese_gapja_string(&mut self) -> String {
        self.get_gapja();

        let mut gapja_string = String::new();

        if let (Some(year_cheongan), Some(year_ganji)) =
            (self.gapja_year_inx[0], self.gapja_year_inx[1])
        {
            gapja_string.push(CHINESE_CHEONGAN[year_cheongan]);
            gapja_string.push(CHINESE_GANJI[year_ganji]);
            gapja_string.push(CHINESE_GAPJA_UNIT[0]);
        } else {
            return "".to_string();
        }

        gapja_string.push(' ');

        if let (Some(month_cheongan), Some(month_ganji)) =
            (self.gapja_month_inx[0], self.gapja_month_inx[1])
        {
            gapja_string.push(CHINESE_CHEONGAN[month_cheongan]);
            gapja_string.push(CHINESE_GANJI[month_ganji]);
            gapja_string.push(CHINESE_GAPJA_UNIT[1]);
        } else {
            return "".to_string();
        }

        gapja_string.push(' ');

        if let (Some(day_cheongan), Some(day_ganji)) =
            (self.gapja_day_inx[0], self.gapja_day_inx[1])
        {
            gapja_string.push(CHINESE_CHEONGAN[day_cheongan]);
            gapja_string.push(CHINESE_GANJI[day_ganji]);
            gapja_string.push(CHINESE_GAPJA_UNIT[2]);
        } else {
            return "".to_string();
        }

        if self.is_intercalation {
            gapja_string.push_str(" (");
            gapja_string.push(INTERCALATION_STR[1]);
            gapja_string.push(CHINESE_GAPJA_UNIT[1]);
            gapja_string.push(')');
        }

        gapja_string
    }

    pub fn get_lunar_iso_format(&self) -> String {
        let mut iso_str = format!(
            "{:04}-{:02}-{:02}",
            self.lunar_year, self.lunar_month, self.lunar_day
        );

        if self.is_intercalation {
            iso_str.push_str(" Intercalation");
        }

        iso_str
    }

    pub fn get_solar_iso_format(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}",
            self.solar_year, self.solar_month, self.solar_day
        )
    }

    /// Calculates the Julian Day Number (JDN) for a given Solar date.
    /// The JDN is the integer number of days elapsed since noon UTC on January 1, 4713 BC.
    ///
    /// Handles the transition from the Julian to the Gregorian calendar in October 1582.
    /// Returns `None` if the date is invalid (e.g., within the 1582 gap).
    pub fn get_julian_day_number(year: u32, month: u32, day: u32) -> Option<u32> {
        // Check for invalid date in the Gregorian reform gap
        if year == 1582 && month == 10 && day > 4 && day < 15 {
            return None;
        }
        // Basic month/day validation (simplified, primarily for algorithm safety)
        if month == 0 || month > 12 || day == 0 || day > 31 {
            return None;
        }

        // Use i32 for calculations
        let y = year as i32;
        let m = month as i32;
        let d = day as i32;

        // Adjust month/year for Jan/Feb for calculation
        let adj_y = if m <= 2 { y - 1 } else { y };
        let adj_m = if m <= 2 { m + 12 } else { m };

        // Calculate base Julian part using integer arithmetic
        let julian_base = (1461 * (adj_y + 4716)) / 4 + (153 * (adj_m + 1)) / 5 + d;

        // Determine Gregorian correction term 'b'
        let b = if y > 1582 || (y == 1582 && m > 10) || (y == 1582 && m == 10 && d >= 15) {
            // Apply correction only for Gregorian dates (starting from 1582-10-15)
            let term1 = adj_y / 100; // Note: Use adj_y here consistent with algorithm derivations
            2 - term1 + term1 / 4
        } else {
            // No correction for Julian dates (up to 1582-10-04)
            0
        };

        // Combine base, correction, and standard offset (-1524)
        let jdn = julian_base + b - 1524;

        Some(jdn as u32)
    }

    /// Calculates the day of the week for a given Solar date.
    /// Returns `None` if the date is invalid (e.g., within the 1582 gap).
    pub fn get_day_of_week(year: u32, month: u32, day: u32) -> Option<DayOfWeek> {
        Self::get_julian_day_number(year, month, day).map(|jdn| {
            // JDN mod 7: 0=Mon, 1=Tue, 2=Wed, 3=Thu, 4=Fri, 5=Sat, 6=Sun
            match jdn % 7 {
                0 => DayOfWeek::Monday,
                1 => DayOfWeek::Tuesday,
                2 => DayOfWeek::Wednesday,
                3 => DayOfWeek::Thursday,
                4 => DayOfWeek::Friday,
                5 => DayOfWeek::Saturday,
                _ => DayOfWeek::Sunday, // 6 and any unexpected remainder
            }
        })
    }

    /// Checks if a given solar year is a leap year.
    /// Uses Gregorian rules (and Julian rules for years <= 1582).
    pub fn is_solar_leap_year(year: u32) -> bool {
        // Reuse the internal logic which handles the Gregorian reform
        Self::is_gregorian_leap(year as i32)
    }

    /// Gets the intercalary month (윤달) for a given lunar year, if one exists.
    /// Returns `Some(month)` if an intercalary month exists, otherwise `None`.
    pub fn get_lunar_intercalary_month(year: i32) -> Option<u32> {
        if year < KOREAN_LUNAR_BASE_YEAR
            || year > KOREAN_LUNAR_BASE_YEAR + KOREAN_LUNAR_DATA.len() as i32 - 1
        {
            return None; // Year out of supported range
        }
        let lunar_data = Self::get_lunar_data(year);
        let intercalary_month = Self::get_lunar_intercalation_month(lunar_data);
        if intercalary_month > 0 {
            Some(intercalary_month)
        } else {
            None
        }
    }

    // --- Getters for date fields ---
    #[allow(dead_code)]
    pub fn solar_year(&self) -> u32 {
        self.solar_year
    }
    #[allow(dead_code)]
    pub fn solar_month(&self) -> u32 {
        self.solar_month
    }
    #[allow(dead_code)]
    pub fn solar_day(&self) -> u32 {
        self.solar_day
    }
    pub fn lunar_year(&self) -> i32 {
        self.lunar_year
    }
    #[allow(dead_code)]
    pub fn lunar_month(&self) -> u32 {
        self.lunar_month
    }
    #[allow(dead_code)]
    pub fn lunar_day(&self) -> u32 {
        self.lunar_day
    }
    #[allow(dead_code)]
    pub fn is_intercalation(&self) -> bool {
        self.is_intercalation
    }
    // ------------------------------
}

#[cfg(test)]
mod tests {
    use super::DayOfWeek;
    use crate::LunarSolarConverter;

    #[test]
    fn test_lunar_iso_format() {
        let mut converter = LunarSolarConverter::new();
        converter.set_solar_date(2022, 7, 10);
        let lunar = converter.get_lunar_iso_format();
        let want = "2022-06-12";

        println!("{}", lunar);

        assert_eq!(lunar, want, "got {:?} want {:?}", lunar, want);
    }

    #[test]
    fn test_gapja_string() {
        let mut converter = LunarSolarConverter::new();
        converter.set_solar_date(2022, 7, 10);
        let lunar_gapja = converter.get_gapja_string();
        let want = "임인년 정미월 갑자일";

        println!("{}", lunar_gapja);

        assert_eq!(lunar_gapja, want, "got {:?} want {:?}", lunar_gapja, want);
    }

    #[test]
    fn test_chinese_gapja_string() {
        let mut converter = LunarSolarConverter::new();
        converter.set_solar_date(2022, 7, 10);
        let lunar_chinese_gapja = converter.get_chinese_gapja_string();
        let want = "壬寅年 丁未月 甲子日";

        println!("{}", lunar_chinese_gapja);

        assert_eq!(
            lunar_chinese_gapja, want,
            "got {:?} want {:?}",
            lunar_chinese_gapja, want
        );
    }

    #[test]
    fn test_solar_iso_format() {
        let mut converter = LunarSolarConverter::new();
        converter.set_lunar_date(2022, 6, 12, false);
        let solar = converter.get_solar_iso_format();
        let want = "2022-07-10";

        println!("{}", solar);

        assert_eq!(solar, want, "got {:?} want {:?}", solar, want);
    }

    #[test]
    fn test_gapja_string_intercalation() {
        let mut converter = LunarSolarConverter::new();
        let is_valid = converter.set_lunar_date(2022, 6, 12, true);

        assert!(
            !is_valid,
            "Expected set_lunar_date to return false for non-existent intercalary month"
        );
    }

    #[test]
    fn test_chinese_gapja_string_intercalation() {
        let mut converter = LunarSolarConverter::new();
        let is_valid = converter.set_lunar_date(2022, 6, 12, true);

        assert!(
            !is_valid,
            "Expected set_lunar_date to return false for non-existent intercalary month"
        );
    }

    #[test]
    fn test_set_solar_date() {
        let mut converter = LunarSolarConverter::new();
        let is_valid = converter.set_solar_date(2022, 7, 10);

        assert!(is_valid, "Expected solar date to be valid");
        assert_eq!(converter.lunar_year, 2022);
        assert_eq!(converter.lunar_month, 6);
        assert_eq!(converter.lunar_day, 12);
    }

    #[test]
    fn test_set_lunar_date() {
        let mut converter = LunarSolarConverter::new();
        let is_valid = converter.set_lunar_date(2022, 6, 12, false);

        assert!(is_valid, "Expected lunar date to be valid");
        assert_eq!(converter.solar_year, 2022);
        assert_eq!(converter.solar_month, 7);
        assert_eq!(converter.solar_day, 10);
    }

    #[test]
    fn test_invalid_solar_date() {
        let mut converter = LunarSolarConverter::new();
        let is_valid = converter.set_solar_date(1582, 10, 10);

        assert!(!is_valid, "Expected solar date to be invalid");
    }

    #[test]
    fn test_invalid_lunar_date() {
        let mut converter = LunarSolarConverter::new();
        let is_valid = converter.set_lunar_date(1390, 12, 31, false);

        assert!(!is_valid, "Expected lunar date to be invalid");
    }

    #[test]
    fn test_get_lunar_days() {
        let days = LunarSolarConverter::get_lunar_days(2022, 6, false);

        assert_eq!(days, 30, "Expected 30 days for June 2022");
    }

    #[test]
    fn test_get_lunar_days_invalid_month() {
        let days = LunarSolarConverter::get_lunar_days(2022, 13, false);

        assert_eq!(days, 0, "Expected 0 days for invalid month 2022");
    }

    #[test]
    fn test_get_lunar_days_invalid_year() {
        let days = LunarSolarConverter::get_lunar_days(1390, 6, false);
        assert_eq!(days, 0, "Expected 0 days for invalid year 1390");
    }

    #[test]
    fn test_get_solar_days() {
        let days = LunarSolarConverter::get_solar_days(2022, 2);

        assert_eq!(days, 28, "Expected 28 days for February 2022");
    }

    #[test]
    fn test_get_solar_days_invalid_month() {
        let days = LunarSolarConverter::get_solar_days(2022, 13);

        assert_eq!(days, 0, "Expected 0 days for invalid month 2022");
    }

    #[test]
    fn test_get_lunar_abs_days() {
        let days = LunarSolarConverter::get_lunar_abs_days(2022, 6, 12, false);

        assert_eq!(days, 230616, "Expected 230616 absolute lunar days");
    }

    #[test]
    fn test_get_lunar_abs_days_invalid_year() {
        let days = LunarSolarConverter::get_lunar_abs_days(1390, 6, 12, false);

        assert_eq!(
            days, 0,
            "Expected 0 absolute lunar days for invalid year 1390"
        );
    }

    #[test]
    fn test_get_solar_abs_days() {
        let days = LunarSolarConverter::get_solar_abs_days(2022, 7, 10);

        // Correct assertion back to the library's internal relative day count
        assert_eq!(days, 230616, "Expected 230616 absolute solar days");
    }

    #[test]
    fn test_get_solar_abs_days_invalid_year() {
        let days = LunarSolarConverter::get_solar_abs_days(1390, 7, 10);

        assert_eq!(
            days, 0,
            "Expected 0 absolute solar days for invalid year 1390"
        );
    }

    #[test]
    fn test_invalid_date_for_get_gapja_string() {
        let mut converter = LunarSolarConverter::new();
        converter.set_lunar_date(1390, 12, 31, false);
        let gapja = converter.get_gapja_string();

        assert_eq!(gapja, "", "Expected empty string since the date is invalid");
    }

    #[test]
    fn test_invalid_date_for_get_chinese_gapja_string() {
        let mut converter = LunarSolarConverter::new();
        converter.set_lunar_date(1390, 12, 31, false);
        let gapja = converter.get_chinese_gapja_string();

        assert_eq!(gapja, "", "Expected empty string since the date is invalid");
    }

    #[test]
    fn test_get_julian_day_number_gregorian() {
        let jdn = LunarSolarConverter::get_julian_day_number(2022, 7, 10);
        let want = Some(2459771);
        assert_eq!(
            jdn, want,
            "Expected JDN {:?} for 2022-07-10, got {:?}",
            want, jdn
        );
    }

    #[test]
    fn test_get_julian_day_number_julian() {
        let jdn = LunarSolarConverter::get_julian_day_number(1500, 3, 1);
        let want = Some(2268993);
        assert_eq!(
            jdn, want,
            "Expected JDN {:?} for 1500-03-01, got {:?}",
            want, jdn
        );
    }

    #[test]
    fn test_get_julian_day_number_reform_before() {
        let jdn = LunarSolarConverter::get_julian_day_number(1582, 10, 4);
        let want = Some(2299160);
        assert_eq!(
            jdn, want,
            "Expected JDN {:?} for 1582-10-04, got {:?}",
            want, jdn
        );
    }

    #[test]
    fn test_get_julian_day_number_reform_after() {
        let jdn = LunarSolarConverter::get_julian_day_number(1582, 10, 15);
        let want = Some(2299161);
        assert_eq!(
            jdn, want,
            "Expected JDN {:?} for 1582-10-15, got {:?}",
            want, jdn
        );
    }

    #[test]
    fn test_get_julian_day_number_min_date() {
        // Corresponds to KOREAN_SOLAR_MIN_VALUE
        let jdn = LunarSolarConverter::get_julian_day_number(1391, 2, 5);
        let want = Some(2229156);
        assert_eq!(
            jdn, want,
            "Expected JDN {:?} for 1391-02-05, got {:?}",
            want, jdn
        );
    }

    #[test]
    fn test_get_julian_day_number_invalid_gap() {
        let jdn = LunarSolarConverter::get_julian_day_number(1582, 10, 10);
        assert_eq!(
            jdn, None,
            "Expected None for invalid date 1582-10-10 (Gregorian gap)"
        );
    }

    #[test]
    fn test_get_julian_day_number_invalid_range_before() {
        // JDN function should work outside library range, only failing for 1582 gap.
        let jdn = LunarSolarConverter::get_julian_day_number(1391, 2, 4);
        let want = Some(2229155);
        assert_eq!(
            jdn, want,
            "Expected JDN {:?} for 1391-02-04, got {:?}",
            want, jdn
        );
    }

    #[test]
    fn test_get_julian_day_number_invalid_range_after() {
        // Using a date beyond the KOREAN_SOLAR_MAX_VALUE
        // JDN function should work outside library range.
        let jdn = LunarSolarConverter::get_julian_day_number(2051, 1, 1);
        let want = Some(2470173);
        assert_eq!(
            jdn, want,
            "Expected JDN {:?} for 2051-01-01, got {:?}",
            want, jdn
        );
    }

    #[test]
    fn test_get_day_of_week_monday() {
        // JDN 2459771 % 7 = 0 (Monday)
        let dow = LunarSolarConverter::get_day_of_week(2022, 7, 11);
        let want = Some(DayOfWeek::Monday);
        assert_eq!(
            dow, want,
            "Expected {:?} for 2022-07-11, got {:?}",
            want, dow
        );
    }

    #[test]
    fn test_get_day_of_week_sunday() {
        // JDN 2459770 % 7 = 6 (Sunday)
        let dow = LunarSolarConverter::get_day_of_week(2022, 7, 10);
        let want = Some(DayOfWeek::Sunday);
        assert_eq!(
            dow, want,
            "Expected {:?} for 2022-07-10, got {:?}",
            want, dow
        );
    }

    #[test]
    fn test_get_day_of_week_reform_before() {
        // October 4, 1582 was a Thursday (Julian)
        let result = LunarSolarConverter::get_day_of_week(1582, 10, 4);
        assert_eq!(result, Some(DayOfWeek::Thursday)); // Corrected from Friday
    }

    #[test]
    fn test_get_day_of_week_reform_after() {
        // October 15, 1582 was a Friday (Gregorian)
        let result = LunarSolarConverter::get_day_of_week(1582, 10, 15);
        assert_eq!(result, Some(DayOfWeek::Friday)); // Corrected from Saturday
    }

    #[test]
    fn test_get_day_of_week_invalid_gap() {
        let dow = LunarSolarConverter::get_day_of_week(1582, 10, 10);
        assert_eq!(dow, None, "Expected None for invalid date 1582-10-10");
    }

    #[test]
    fn test_is_solar_leap_year() {
        assert!(
            LunarSolarConverter::is_solar_leap_year(2024),
            "2024 should be a leap year"
        );
        assert!(
            !LunarSolarConverter::is_solar_leap_year(2023),
            "2023 should not be a leap year"
        );
        assert!(
            !LunarSolarConverter::is_solar_leap_year(1900),
            "1900 should not be a leap year"
        );
        assert!(
            LunarSolarConverter::is_solar_leap_year(2000),
            "2000 should be a leap year"
        );
        assert!(
            LunarSolarConverter::is_solar_leap_year(1500),
            "1500 should be a leap year (Julian)"
        );
        assert!(
            !LunarSolarConverter::is_solar_leap_year(1582),
            "1582 should not be a leap year"
        );
        assert!(
            LunarSolarConverter::is_solar_leap_year(1600),
            "1600 should be a leap year"
        );
    }

    #[test]
    fn test_get_lunar_intercalary_month() {
        // Year 2023 actually has intercalary month 2 (윤2월)
        assert_eq!(
            LunarSolarConverter::get_lunar_intercalary_month(2023),
            Some(2),
            "Year 2023 should have intercalary month 2"
        );
        // Year 2020 actually has intercalary month 4 (윤4월)
        assert_eq!(
            LunarSolarConverter::get_lunar_intercalary_month(2020),
            Some(4),
            "Year 2020 should have intercalary month 4"
        );
        // Year 2022 has no intercalary month
        assert_eq!(
            LunarSolarConverter::get_lunar_intercalary_month(2022),
            None,
            "Year 2022 should not have an intercalary month"
        );
        assert_eq!(
            LunarSolarConverter::get_lunar_intercalary_month(1391),
            None,
            "Year 1391 should not have an intercalary month"
        ); // First year in data
    }

    #[test]
    fn test_get_lunar_intercalary_month_out_of_range() {
        assert_eq!(
            LunarSolarConverter::get_lunar_intercalary_month(1390),
            None,
            "Year 1390 is out of range"
        );
        // Max year is 2050 (1391 + 660 - 1)
        assert_eq!(
            LunarSolarConverter::get_lunar_intercalary_month(2051),
            None,
            "Year 2051 is out of range"
        );
    }
}
