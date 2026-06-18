use crate::seo::{
    faq_json_ld, organization_json_ld, product_json_ld, website_json_ld, JsonLd, SeoHead,
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Script, Stylesheet};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::use_params_map,
    ParamSegment, StaticSegment,
};

/* ==========================================
Data Types
========================================== */

/* ==========================================
Language & Localization (i18n) Context
========================================== */

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Language {
    En,
    Vi,
}

#[derive(Clone, Copy)]
pub struct LanguageContext {
    pub lang: RwSignal<Language>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: RwSignal<Theme>,
}

macro_rules! t {
    ($lang:expr, $en:expr, $vi:expr) => {
        move || match $lang.get() {
            Language::En => $en,
            Language::Vi => $vi,
        }
    };
}

/* ==========================================
Data Types
========================================== */

#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    pub id: String,
    pub name_en: String,
    pub name_vi: String,
    pub tag_en: String,
    pub tag_vi: String,
    pub image: String,
    pub images: Vec<String>,
    pub price: f64,
    pub desc_en: String,
    pub desc_vi: String,
    pub detailed_desc_en: String,
    pub detailed_desc_vi: String,
    pub shopee_url: String,
    pub github_url: Option<String>,
    pub specs_en: Vec<String>,
    pub specs_vi: Vec<String>,
}

impl Product {
    pub fn name(&self, lang: Language) -> String {
        match lang {
            Language::En => self.name_en.clone(),
            Language::Vi => self.name_vi.clone(),
        }
    }
    pub fn tag(&self, lang: Language) -> String {
        match lang {
            Language::En => self.tag_en.clone(),
            Language::Vi => self.tag_vi.clone(),
        }
    }
    pub fn desc(&self, lang: Language) -> String {
        match lang {
            Language::En => self.desc_en.clone(),
            Language::Vi => self.desc_vi.clone(),
        }
    }
    pub fn detailed_desc(&self, lang: Language) -> String {
        match lang {
            Language::En => self.detailed_desc_en.clone(),
            Language::Vi => self.detailed_desc_vi.clone(),
        }
    }
    pub fn specs(&self, lang: Language) -> Vec<String> {
        match lang {
            Language::En => self.specs_en.clone(),
            Language::Vi => self.specs_vi.clone(),
        }
    }
}

/* ==========================================
Static Products Data
========================================== */

fn get_products() -> Vec<Product> {
    vec![
        Product {
            id: String::from("dactyl"),
            name_en: String::from("Dactyl-ManuForm Split"),
            name_vi: String::from("Bàn phím Ergonomic Dactyl-ManuForm"),
            tag_en: String::from("Best Seller"),
            tag_vi: String::from("Bán chạy nhất"),
            image: String::from("/images/dactyl.png"),
            images: vec![
                String::from("/images/dactyl.png"),
                String::from("/images/dactyl_side.png"),
                String::from("/images/dactyl_setup.png"),
            ],
            price: 219.0,
            desc_en: String::from("The ultimate split ergonomic design. Hand-scaffolded curved keywells, separate left/right modules for optimal desk positioning."),
            desc_vi: String::from("Thiết kế công thái học tách rời tối ưu. Các phím võng 3D được đo đạc thủ công, hai mô-đun trái/phải tách biệt giúp tối ưu hóa tư thế đặt tay."),
            detailed_desc_en: String::from("ABOUT THE BUILD:\nThe Dactyl-ManuForm is an ultra-premium split mechanical keyboard designed specifically to follow the natural contours of human hands. By dividing the layout into standalone left and right modules, it allows your shoulders to remain fully open, promoting a relaxed, upright posture that completely eliminates slouching.\n\nERGONOMIC PROFILE & DESIGN:\nThe defining characteristic of this board is its 3D curved keywells. Rather than forcing your fingers to stretch across a flat plane, the keys are sculpted in a bowl shape that matches the natural arc of your fingers as they curl. Combined with a column-staggered layout where keys are vertically aligned to match differing finger lengths, it reduces repetitive finger travel and wrist strain by over 65% compared to flat rectangular keyboards.\n\nPREMIUM CASING & ACOUSTICS:\nEach enclosure is printed in-house using high-temperature PETG or carbon-fiber reinforced ASA with dense perimeters and solid infill. The unique internal geometry of the 3D-printed shell behaves as an acoustic resonance chamber, creating deep, solid typing sound profiles without hollow echoes. Equipped with hot-swap sockets, pre-lubed stabilizers, and custom damping foam."),
            detailed_desc_vi: String::from("GIỚI THIỆU SẢN PHẨM:\nDactyl-ManuForm là dòng bàn phím cơ công thái học chia đôi cao cấp, được thiết kế mô phỏng chính xác cấu trúc tự nhiên của bàn tay. Việc tách rời hai nửa trái/phải giúp vai bạn luôn mở rộng, duy trì tư thế ngồi thẳng tự nhiên và loại bỏ hoàn toàn tình trạng gù vai.\n\nTHIẾT KẾ & HÌNH DÁNG CÔNG THÁI HỌC:\nĐiểm đặc trưng nhất của dòng phím này là khung phím cong 3D dạng lòng chảo. Thay vì bắt các ngón tay phải duỗi thẳng trên một mặt phẳng, các phím được uốn cong theo cung chuyển động tự nhiên của ngón tay. Kết hợp với thiết kế so le cột (column-staggered), khoảng cách di chuyển của ngón tay và áp lực lên cổ tay được giảm thiểu tới hơn 65% so với phím truyền thống.\n\nVỎ PHÍM & CHẤT ÂM CAO CẤP:\nTừng chiếc vỏ được in trực tiếp tại xưởng bằng nhựa PETG chịu nhiệt hoặc nhựa ASA gia cường carbon với thành vỏ dày và độ đặc cao. Cấu trúc rỗng đặc biệt bên trong đóng vai trò như một hộp cộng hưởng âm thanh, tạo ra tiếng gõ phím trầm ấm (thocky) và đầm chắc, loại bỏ hoàn toàn tiếng vang rỗng khó chịu."),
            shopee_url: String::from("https://shopee.vn/opendiy"),
            github_url: Some(String::from("https://github.com/tshort/dactyl-keyboard")),
            specs_en: vec![
                String::from("3D sculpted curved split keywells"),
                String::from("Columnar layout to match finger lengths"),
                String::from("Fully hot-swappable switch sockets"),
                String::from("VIAL/QMK firmware custom keymap mapping"),
                String::from("High-grade PETG/ASA active-heated printed shell"),
                String::from("Integrated thumb clusters with 5 or 6 keys for core modifiers"),
                String::from("RJ9/TRRS interconnect cables with custom coiled aesthetic options"),
            ],
            specs_vi: vec![
                String::from("Khung phím cong 3D dạng lòng chảo"),
                String::from("Thiết kế dạng cột phù hợp với chiều dài ngón tay"),
                String::from("Hotswap phím thay switch dễ dàng không cần hàn"),
                String::from("Hỗ trợ firmware VIAL/QMK dễ dàng gán phím"),
                String::from("Vỏ in ASA/PETG chất lượng cao từ buồng sấy chủ động"),
                String::from("Cụm phím ngón cái tích hợp 5-6 phím tiện lợi"),
                String::from("Cáp kết nối RJ9/TRRS bọc dù thời trang"),
            ],
        },
        Product {
            id: String::from("frosted"),
            name_en: String::from("Frosted 60% Neon"),
            name_vi: String::from("Bàn phím Frosted 60% Neon"),
            tag_en: String::from("Minimalist"),
            tag_vi: String::from("Tối giản"),
            image: String::from("/images/frosted.png"),
            images: vec![
                String::from("/images/frosted.png"),
                String::from("/images/frosted_side.png"),
                String::from("/images/frosted_setup.png"),
            ],
            price: 109.0,
            desc_en: String::from("Compact 60% layout with a semi-translucent case. Ideal for clean desk setups with vibrant customizable RGB underglow."),
            desc_vi: String::from("Bố cục 60% nhỏ gọn với vỏ bán trong suốt. Lý tưởng cho các góc làm việc gọn gàng với đèn gầm RGB rực rỡ."),
            detailed_desc_en: String::from("ABOUT THE BUILD:\nThe Frosted 60% Neon combines standard keyboard compatibility with stunning desk aesthetics. Removing the arrow cluster and number pad, this compact 60% form factor keeps your mouse closer to your center of gravity, lessening arm strain and leaving more open space on your desk.\n\nTHE FROSTED CASE EXPERIENCE:\nThe custom case is machined from semi-translucent frosted polycarbonate/acrylic material. Unlike fully clear cases that show dust and internal hot glue, the frosted finish diffuses light smoothly across the entire chassis. Internal LEDs create a seamless, vibrant ambient glow that reflects beautifully onto your desk mat.\n\nSOUND PROFILE & BUILD SPECIFICS:\nTo counter the high-pitched clacking common in acrylic keyboards, this edition features custom internal silicone dampening pads and thick PET plate materials. This gives the board a soft, slightly bouncy typing feel paired with a quiet, clean sound signature suitable for office environments."),
            detailed_desc_vi: String::from("GIỚI THIỆU SẢN PHẨM:\nFrosted 60% Neon là sự kết hợp hoàn hảo giữa độ tương thích phím tiêu chuẩn và tính thẩm mỹ tuyệt vời. Loại bỏ cụm mũi tên và hàng phím số phụ, form 60% giúp tay di chuột của bạn gần cơ thể hơn, giảm thiểu mỏi vai gáy và giải phóng diện tích bàn làm việc.\n\nTRẢI NGHIỆM VỎ NHÁM FROSTED:\nVỏ phím được gia công từ nhựa Polycarbonate/Acrylic nhám mờ bán trong suốt. Khác với vỏ trong suốt dễ lộ bụi bẩn và keo dán bên trong, bề mặt frosted khuếch tán ánh sáng cực kỳ mịn màng. Dàn LED gầm tạo ra quầng sáng dịu mắt phản chiếu xuống thảm trải bàn.\n\nCHẤT ÂM & CẤU TRÚC PHÍM:\nĐể khắc phục nhược điểm âm gõ đanh, chói của phím acrylic, phiên bản này được tích hợp đệm silicon giảm chấn nội bộ cùng plate nhựa PET dày dặn. Mang lại cảm giác gõ êm ái, hơi nhún nhẹ đi kèm âm thanh trầm, tĩnh lặng phù hợp cho cả môi trường văn phòng."),
            shopee_url: String::from("https://shopee.vn/opendiy"),
            github_url: None,
            specs_en: vec![
                String::from("Sleek unibody 60% compact layout"),
                String::from("Frosted acrylic-style translucent enclosure"),
                String::from("Customizable vibrant underglow RGB LEDs"),
                String::from("Hot-swappable keys with sound dampening foam"),
                String::from("Standard ANSI layout layout compatibility"),
                String::from("Tray-mount construction with customized sound-dampening sheets"),
                String::from("USB-C detachable custom coiled cable compatible"),
            ],
            specs_vi: vec![
                String::from("Thiết kế unibody 60% siêu nhỏ gọn"),
                String::from("Vỏ nhựa nhám mờ tản sáng cao cấp"),
                String::from("Tùy biến led gầm và led nền RGB rực rỡ"),
                String::from("Hotswap thay switch nhanh tích hợp foam tiêu âm"),
                String::from("Tương thích hoàn hảo layout ANSI tiêu chuẩn"),
                String::from("Cấu trúc Tray-mount đi kèm đệm silicon giảm chấn"),
                String::from("Kết nối USB-C tương thích tốt cáp xoắn thời trang"),
            ],
        },
        Product {
            id: String::from("alice"),
            name_en: String::from("Alice Curved Forest"),
            name_vi: String::from("Bàn phím Alice Curved Forest"),
            tag_en: String::from("Ergonomic"),
            tag_vi: String::from("Công thái học"),
            image: String::from("/images/alice.png"),
            images: vec![
                String::from("/images/alice.png"),
                String::from("/images/alice_side.png"),
                String::from("/images/alice_setup.png"),
            ],
            price: 149.0,
            desc_en: String::from("Unibody ergonomic curved layout. Reduces wrist ulnar deviation without the learning curve of a fully split keyboard."),
            desc_vi: String::from("Bố cục cong unibody công thái học. Giảm thiểu uốn cổ tay mà không cần mất thời gian làm quen với bàn phím tách rời hoàn toàn."),
            detailed_desc_en: String::from("ABOUT THE BUILD:\nThe Alice Curved Forest bridges the gap between conventional typing layouts and full split ergonomics. Perfect for writers, programmers, and office workers who want to protect their wrists but prefer the solidity and simplicity of a single, unified keyboard chassis on their desk.\n\nERGO-ANGLED ALIGNMENT:\nBy splitting the main character keys into two angled halves that curve outward, the Alice layout allows your wrists to align straight with your forearms. This reduces ulnar deviation (the outward twisting of the hands) which is the leading cause of carpal tunnel syndrome in typing professionals.\n\nUNMATCHED TEXTURED AESTHETICS:\nThe case features a gorgeous forest-green finish with a textured acoustic outer shell. Constructed with gasket mounts, the plate rests on soft rubber strips inside the case, allowing the entire mounting structure to yield slightly under typing pressure for a flexible, cushioned response."),
            detailed_desc_vi: String::from("GIỚI THIỆU SẢN PHẨM:\nAlice Curved Forest là cầu nối tuyệt vời giữa bàn phím truyền thống và phím chia đôi công thái học. Thích hợp cho nhà văn, lập trình viên và nhân viên văn phòng muốn bảo vệ cổ tay nhưng vẫn yêu thích sự vững chãi, gọn gàng của một chiếc bàn phím liền khối.\n\nTƯ THẾ TỰ NHIÊN:\nBằng cách chia các phím chữ làm hai nửa nghiêng đối xứng hướng ra ngoài, bố cục Alice giúp cổ tay bạn luôn thẳng hàng với cẳng tay. Điều này giảm thiểu tối đa hiện tượng uốn cổ tay ra ngoài - nguyên nhân chính gây hội chứng ống cổ tay ở dân văn phòng.\n\nTHẨM MỸ MÀU XANH RỪNG CUỐN HÚT:\nVỏ phím mang sắc xanh rừng già cực kỳ dịu mắt với lớp hoàn thiện nhám cách âm. Được xây dựng theo cấu trúc Gasket-mount, tấm plate đỡ switch được nâng đỡ bằng các dải cao su mềm trong vỏ phím, giúp hấp thụ lực gõ cực tốt và đem lại phản hồi gõ êm nhẹ, đàn hồi."),
            shopee_url: String::from("https://shopee.vn/opendiy"),
            github_url: None,
            specs_en: vec![
                String::from("Ergonomic curved Alice unibody layout"),
                String::from("Reduces wrist ulnar deviation"),
                String::from("Hot-swappable switch sockets"),
                String::from("Forest green textured acoustic casing"),
                String::from("Simple standard keyboard learning curve"),
                String::from("Premium Gasket-mounted plate structure for flexible typing feel"),
                String::from("Includes rotary encoder knob for smooth volume control"),
            ],
            specs_vi: vec![
                String::from("Bố cục Alice unibody uốn cong công thái học"),
                String::from("Giảm thiểu uốn cổ tay ra ngoài"),
                String::from("Hotswap thay switch dễ dàng"),
                String::from("Vỏ xanh rừng cây hoàn thiện nhám tiêu âm"),
                String::from("Thời gian làm quen cực kỳ ngắn so với phím split"),
                String::from("Cấu trúc Gasket-mount cho cảm giác gõ êm và đàn hồi"),
                String::from("Tích hợp núm xoay vô cực điều khiển âm lượng mượt mà"),
            ],
        },
        Product {
            id: String::from("corne"),
            name_en: String::from("Corne Cherry Split"),
            name_vi: String::from("Bàn phím Ergonomic Corne Cherry"),
            tag_en: String::from("Ultra-Compact"),
            tag_vi: String::from("Siêu nhỏ gọn"),
            image: String::from("/images/corne.png"),
            images: vec![
                String::from("/images/corne.png"),
                String::from("/images/corne_side.png"),
                String::from("/images/corne_setup.png"),
            ],
            price: 129.0,
            desc_en: String::from("Ultra-compact 40% split layout. Highly optimized mapping, customized dual OLED displays, and brilliant customizable RGB underglow."),
            desc_vi: String::from("Bố cục split 40% siêu nhỏ gọn. Sơ đồ phím tối ưu hóa cực cao, tích hợp màn hình OLED kép hiển thị thông tin và led nền RGB."),
            detailed_desc_en: String::from("ABOUT THE BUILD:\nFor keyboard purists and power-users, the Corne Split represents the pinnacle of compact input. It discards the number row, function keys, and navigation keys, keeping exactly 42 keys total so your hands never need to move from the home row.\n\nOPTIMIZED LAYER LOGIC:\nBy using QMK/VIAL firmware, the Corne utilizes modifier keys and custom layers to access missing keys. Holding the space bar can activate the number layer, while holding backspace triggers symbols, allowing faster and more efficient typing once the short layout is mastered.\n\nOLED STATUS SCREEN & RGB SHOWCASE:\nEach half of the keyboard is equipped with a custom 0.91-inch OLED screen that displays active layers, typing speed (WPM), lock states, and customized logos. Combined with vibrant per-key and underglow RGB, it is as visually striking as it is functional."),
            detailed_desc_vi: String::from("GIỚI THIỆU SẢN PHẨM:\nĐối với những người dùng chuyên sâu và tối giản, bàn phím Corne Split đại diện cho đỉnh cao của sự gọn nhẹ. Loại bỏ hoàn toàn hàng số, phím chức năng và phím điều hướng, phím chỉ có đúng 42 phím giúp tay bạn không bao giờ phải rời khỏi hàng phím cơ sở (home row).\n\nTỐI ƯU HÓA LỚP PHÍM (LAYERS):\nBằng cách sử dụng phần mềm gán phím VIAL/QMK, Corne tận dụng triệt để cơ chế gán phím theo tầng. Nhấn giữ phím Space có thể kích hoạt tầng gõ số, nhấn giữ Backspace để mở ký tự đặc biệt, giúp tốc độ gõ phím nhanh hơn nhiều khi đã làm quen.\n\nMÀN HÌNH OLED & LED RGB RỰC RỠ:\nMỗi nửa bàn phím được trang bị một màn hình OLED 0.91 inch hiển thị lớp phím đang hoạt động, tốc độ gõ (WPM), trạng thái khóa và logo cá nhân. Kết hợp với led nền từng phím và led gầm rực rỡ tạo nên hiệu ứng thị giác đỉnh cao."),
            shopee_url: String::from("https://shopee.vn/opendiy"),
            github_url: Some(String::from("https://github.com/foostan/crkbd")),
            specs_en: vec![
                String::from("Ultra-compact split 42-key layout"),
                String::from("Dual custom 0.91-inch OLED status screens"),
                String::from("Individual per-key and underglow RGB"),
                String::from("VIAL software support for simple custom layer setup"),
                String::from("Premium slim acrylic plate sandwich casing"),
                String::from("Column-staggered layout matching finger anatomy"),
                String::from("Ultra-low latency custom MCU controllers"),
            ],
            specs_vi: vec![
                String::from("Bố cục split 42 phím siêu tối giản"),
                String::from("Màn hình OLED kép 0.91 inch hiển thị thông số"),
                String::from("Đèn LED nền từng phím và led gầm RGB độc lập"),
                String::from("Hỗ trợ gán phím trực quan qua phần mềm VIAL"),
                String::from("Vỏ dạng sandwich mica acrylic mỏng nhẹ"),
                String::from("Bố cục so le cột chuẩn giải phẫu ngón tay"),
                String::from("Bộ vi điều khiển MCU độ trễ cực thấp"),
            ],
        },
        Product {
            id: String::from("wristrests"),
            name_en: String::from("Dactyl Walnut Wrist Rests"),
            name_vi: String::from("Kê tay Gỗ Walnut cho Dactyl"),
            tag_en: String::from("Accessories"),
            tag_vi: String::from("Phụ kiện"),
            image: String::from("/images/wristrests.png"),
            images: vec![
                String::from("/images/wristrests.png"),
                String::from("/images/wristrests_side.png"),
                String::from("/images/wristrests_setup.png"),
            ],
            price: 39.0,
            desc_en: String::from("Handcrafted premium American Walnut wood. Artfully sculpted and contoured to perfectly match Dactyl Manuform split casings."),
            desc_vi: String::from("Gỗ óc chó Mỹ tự nhiên chế tác thủ công. Uốn lượn nghệ thuật ôm sát hoàn hảo đường cong của vỏ Dactyl Manuform."),
            detailed_desc_en: String::from("ABOUT THE ACCESSORY:\nMost flat wrist rests fail to provide adequate support for split contoured keyboards. These Dactyl Walnut Wrist Rests are handcrafted from premium solid American Walnut wood, specifically carved to pair with our Dactyl-ManuForm models.\n\nSCULPTED INTEGRATION:\nEach rest is individual, allowing you to position them alongside the split keyboard modules. The wood is sculpted with matching vertical slopes and horizontal curves, providing a seamless landing pad for your palms that keeps your wrists perfectly straight and supported.\n\nNATURAL CRAFTSMANSHIP:\nWe hand-sand each block through multiple grits and finish them with multiple coats of natural satin oil. This brings out the deep, rich walnut grain patterns while protecting the wood from moisture and skin oils. Fitted with embedded non-slip silicone feet."),
            detailed_desc_vi: String::from("GIỚI THIỆU PHỤ KIỆN:\nHầu hết các loại kê tay phẳng thông thường không thể nâng đỡ cổ tay bạn khi sử dụng bàn phím cong Dactyl. Kê tay Dactyl Walnut được chế tác thủ công hoàn toàn từ gỗ óc chó nguyên khối cao cấp, được đẽo gọt chuẩn xác theo đường cong của phím.\n\nTỰ DO DI CHUYỂN:\nKê tay gồm hai nửa riêng biệt giúp bạn tự do bố trí cạnh phím tùy theo vị trí đặt tay. Khung gỗ được tính toán độ dốc thẳng đứng và chiều cong ngang để lòng bàn tay bạn luôn có điểm tựa vững chãi, giữ cổ tay thẳng tự nhiên.\n\nCHẾ TÁC TỰ NHIÊN:\nTừng khối gỗ được đánh bóng thủ công tỉ mỉ qua nhiều cấp độ nhám và phủ dầu bảo vệ gỗ tự nhiên. Lớp hoàn thiện làm nổi bật vân gỗ óc chó sâu, ấm áp đồng thời chống ẩm, mồ hôi hiệu quả. Đế kê tay có chân cao su chống trượt."),
            shopee_url: String::from("https://shopee.vn/opendiy"),
            github_url: None,
            specs_en: vec![
                String::from("Solid American Walnut wood construction"),
                String::from("Custom contoured specifically for Dactyl curves"),
                String::from("Satin natural wood oil finish"),
                String::from("Non-slip silicone rubber feet embedded"),
                String::from("Reduces wrist extension and muscle tension"),
                String::from("Handcrafted in-house with individual wood grain characters"),
            ],
            specs_vi: vec![
                String::from("Chế tác hoàn toàn từ gỗ óc chó Mỹ nguyên khối"),
                String::from("Uốn lượn ôm sát theo vỏ phím Dactyl"),
                String::from("Phủ dầu bảo vệ gỗ mờ tự nhiên cao cấp"),
                String::from("Đế chân cao su silicone chống xê dịch hiệu quả"),
                String::from("Giảm thiểu mỏi và căng cơ cổ tay"),
                String::from("Mỗi sản phẩm có vân gỗ tự nhiên độc nhất vô nhị"),
            ],
        },
        Product {
            id: String::from("cheese-pen-holder"),
            name_en: String::from("Cheese Cube Pen Holder"),
            name_vi: String::from("Giá Đựng Bút Phô Mai Swiss 3D In"),
            tag_en: String::from("Desk Accessory"),
            tag_vi: String::from("Phụ kiện bàn"),
            image: String::from("https://v3b.fal.media/files/b/0a9e5f48/03htpFTenjoLH9vmcKOot_lY4c5lB7.png"),
            images: vec![
                String::from("https://v3b.fal.media/files/b/0a9e5f48/03htpFTenjoLH9vmcKOot_lY4c5lB7.png"),
            ],
            price: 149.0,
            desc_en: String::from("Swiss cheese inspired 3D printed desk organizer. PETG, ~97g, yellow novelty pen holder for clean desk setups."),
            desc_vi: String::from("Giá đựng bút hình khối phô mai Swiss, 3D in bằng PETG vàng đặc trưng. Thiết kế độc đáo, nhiều lỗ tiện lợi, phù hợp desk setup và quà tặng."),
            detailed_desc_en: String::from("ABOUT THE PRODUCT:\nAdd a playful Swiss cheese twist to your workspace! Made of premium, durable yellow PETG, this ~97g pen holder keeps your pens and tools neatly organized. Perfect for desk setup lovers and keyboard enthusiasts seeking a unique, conversation-starting design.\n\n- Concept: Swiss cheese cube novelty design\n- Material: Durable yellow PETG\n- Weight: ~97g with a stable base\n- Target: Desk setup and keyboard enthusiasts\n- USPs: Distinct multi-hole design organizes pens & tools stylishly"),
            detailed_desc_vi: String::from("GIỚI THIỆU SẢN PHẨM:\nThêm chút cá tính cho góc làm việc với chiếc giá cắm bút hình khối phô mai Swiss độc đáo! Sản phẩm được in từ PETG bền bỉ với màu vàng bắt mắt và trọng lượng ~97g đầm chắc. Đây là lựa chọn hoàn hảo cho dân chơi bàn phím cơ và desk setup muốn làm mới không gian làm việc.\n\n- Concept: Khối phô mai Swiss độc đáo và sáng tạo\n- Chất liệu: Nhựa PETG vàng cao cấp, bền bỉ\n- Trọng lượng: ~97g đầm chắc, chống đổ\n- Đối tượng: Người đam mê desk setup & bàn phím cơ\n- Điểm nổi bật (USP): Nhiều lỗ cắm đa năng cho bút và dụng cụ học tập"),

            shopee_url: String::from("https://shopee.vn/opendiy"),
            github_url: None,
            specs_en: vec![
                String::from("Swiss cheese cube novelty design"),
                String::from("PETG 1.75mm, yellow premium finish"),
                String::from("~97g, stable weighted base"),
                String::from("Made in Vietnam by Open DIY"),
            ],
            specs_vi: vec![
                String::from("Concept khối phô mai Swiss độc đáo"),
                String::from("PETG 1.75mm, hoàn thiện vàng đặc trưng"),
                String::from("~97g, đế chắc không bị đổ khi cắm bút"),
                String::from("Sản xuất tại Việt Nam bởi Open DIY"),
            ],
        },
    ]
}

/* ==========================================
Shell (Server Side Entry)
========================================== */

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="preconnect" href="https://fonts.googleapis.com" />
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="true" />
                <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;500;600;700;800&family=Plus+Jakarta+Sans:wght@300;400;500;600;700&display=swap" rel="stylesheet" />
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <script>
                    "if (localStorage.getItem('theme') === 'light' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: light)').matches)) {
                        document.documentElement.classList.add('light');
                    } else {
                        document.documentElement.classList.remove('light');
                    }"
                </script>
                <script src="/js/otel-sdk.min.js" defer="true" />
                <script src="https://unpkg.com/@grafana/faro-web-sdk@1.9.0/dist/bundle/faro-web-sdk.iife.js" defer="true" />
                <script>
                    "window.addEventListener('load', () => {
                        if (window.GrafanaFaroWebSdk) {
                            window.GrafanaFaroWebSdk.initializeFaro({
                                url: 'http://faro.opendiy.vn/collect',
                                app: {
                                    name: 'open-diy-frontend',
                                    version: '0.1.0',
                                    environment: 'production'
                                }
                            });
                        }
                    });"
                </script>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/* ==========================================
Main App Component
========================================== */

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();



    let lang = RwSignal::new(Language::Vi); // Default to Vietnamese
    provide_context(LanguageContext { lang });

    // Initialize theme signal (default to Dark to preserve existing aesthetics)
    let theme = RwSignal::new(Theme::Dark);

    // Hydrate theme from LocalStorage on the client side
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(local_storage)) = window.local_storage() {
                if let Ok(Some(theme_val)) = local_storage.get_item("theme") {
                    if theme_val == "light" {
                        theme.set(Theme::Light);
                    }
                } else {
                    // Fallback to system preference if no localStorage value exists
                    if let Ok(Some(media_query_list)) = window.match_media("(prefers-color-scheme: light)") {
                        if media_query_list.matches() {
                            theme.set(Theme::Light);
                        }
                    }
                }
            }
        }
    }

    provide_context(ThemeContext { theme });

    // Reactively update the class on <html> and save to LocalStorage
    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        {
            let current_theme = theme.get();
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(html) = document.document_element() {
                        match current_theme {
                            Theme::Light => {
                                let _ = html.class_list().add_1("light");
                                if let Ok(Some(local_storage)) = window.local_storage() {
                                    let _ = local_storage.set_item("theme", "light");
                                }
                            }
                            Theme::Dark => {
                                let _ = html.class_list().remove_1("light");
                                if let Ok(Some(local_storage)) = window.local_storage() {
                                    let _ = local_storage.set_item("theme", "dark");
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    view! {

        <Stylesheet id="leptos" href=format!("/pkg/open-diy.css?v={}", env!("GIT_HASH"))/>
        <Script type_="application/ld+json">{organization_json_ld()}</Script>
        <Script type_="application/ld+json">{website_json_ld()}</Script>
        <Script src="/js/audio.js" defer="true" />

        <div class="app-container">
            <Router>
                <Navbar/>
                <main class="main-content">
                    <Routes fallback=NotFoundPage>
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=StaticSegment("shop") view=CatalogPage/>
                        <Route path=(StaticSegment("product"), ParamSegment("id")) view=ProductDetailPage/>
                        <Route path=StaticSegment("about") view=AboutPage/>
                        <Route path=StaticSegment("otel-test") view=OtelTestPage/>
                    </Routes>
                </main>
                <Footer/>

                // Floating AI CSKH Assistant
                <AiAssistant/>
            </Router>
        </div>
    }
}

/* ==========================================
Navigation Component
========================================== */

#[component]
fn Navbar() -> impl IntoView {
    let lang = expect_context::<LanguageContext>().lang;
    let theme_ctx = expect_context::<ThemeContext>();
    let theme = theme_ctx.theme;

    let menu_open = RwSignal::new(false);

    let is_en = move || lang.get() == Language::En;
    let is_vi = move || lang.get() == Language::Vi;

    let toggle_en = move |_| lang.set(Language::En);
    let toggle_vi = move |_| lang.set(Language::Vi);

    let toggle_menu = move |_| menu_open.update(|v| *v = !*v);

    // Toggle theme callback
    let toggle_theme = move |_| {
        theme.update(|t| {
            *t = match t {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };
        });
    };

    view! {
        <nav class="navbar">
            <A href="/" attr:class="nav-brand">
                <img
                    src="/images/logo_light.png"
                    alt="Open-DIY Logo"
                    class="logo-light"
                />
                <img
                    src="/images/logo_dark.png"
                    alt="Open-DIY Logo"
                    class="logo-dark"
                />
            </A>
            <ul class="nav-menu desktop-only">
                <li><A href="/" attr:class="nav-link">{move || t!(lang, "Home", "Trang chủ")()}</A></li>
                <li><A href="/shop" attr:class="nav-link">{move || t!(lang, "Shop", "Cửa hàng")()}</A></li>
                <li><A href="/about" attr:class="nav-link">{move || t!(lang, "About", "Giới thiệu")()}</A></li>
            </ul>
            <div class="nav-actions" style="display: flex; align-items: center; gap: 16px;">
                // Theme Toggle Button
                <button
                    on:click=toggle_theme
                    type="button"
                    class="theme-toggle-btn"
                    aria-label="Toggle Theme"
                    style="background: none; border: none; color: var(--text-muted); cursor: pointer; display: flex; align-items: center; justify-content: center; padding: 4px; transition: var(--transition-fast);"
                >
                    <svg class="sun-icon" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="12" cy="12" r="5"></circle>
                        <line x1="12" y1="1" x2="12" y2="3"></line>
                        <line x1="12" y1="21" x2="12" y2="23"></line>
                        <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                        <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                        <line x1="1" y1="12" x2="3" y2="12"></line>
                        <line x1="21" y1="12" x2="23" y2="12"></line>
                        <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                        <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                    </svg>
                    <svg class="moon-icon" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                    </svg>
                </button>
                <div class="desktop-only" style="gap: 6px; font-size: 0.85rem; font-weight: 600;">
                    <button
                        on:click=toggle_vi
                        type="button"
                        style=move || if is_vi() {
                            "background: none; border: none; color: var(--secondary); cursor: pointer; padding: 2px 4px; border-bottom: 2px solid var(--secondary);"
                        } else {
                            "background: none; border: none; color: var(--text-muted); cursor: pointer; padding: 2px 4px;"
                        }
                    >
                        "VI"
                    </button>
                    <span style="color: var(--border-color);">"|"</span>
                    <button
                        on:click=toggle_en
                        type="button"
                        style=move || if is_en() {
                            "background: none; border: none; color: var(--secondary); cursor: pointer; padding: 2px 4px; border-bottom: 2px solid var(--secondary);"
                        } else {
                            "background: none; border: none; color: var(--text-muted); cursor: pointer; padding: 2px 4px;"
                        }
                    >
                        "EN"
                    </button>
                </div>
                <a href="https://shopee.vn/opendiy" target="_blank" rel="noopener noreferrer" class="btn btn-primary btn-sm desktop-only">
                    {move || t!(lang, "Shopee Store", "Cửa hàng Shopee")()}
                </a>

                // Burger Button (Mobile only)
                <button
                    on:click=toggle_menu
                    type="button"
                    class="burger-btn"
                    aria-label="Toggle Menu"
                >
                    <div class=move || if menu_open.get() { "burger-icon open" } else { "burger-icon" }>
                        <span></span>
                        <span></span>
                        <span></span>
                    </div>
                </button>
            </div>

            // Mobile Menu Overlay
            <div class=move || if menu_open.get() { "mobile-menu open" } else { "mobile-menu" }>
                <ul class="mobile-nav-links">
                    <li><A on:click=toggle_menu href="/" attr:class="mobile-nav-link">{move || t!(lang, "Home", "Trang chủ")()}</A></li>
                    <li><A on:click=toggle_menu href="/shop" attr:class="mobile-nav-link">{move || t!(lang, "Shop", "Cửa hàng")()}</A></li>
                    <li><A on:click=toggle_menu href="/about" attr:class="mobile-nav-link">{move || t!(lang, "About", "Giới thiệu")()}</A></li>
                    
                    // Mobile Language Selector
                    <li style="margin-top: 20px; border-top: 1px solid var(--border-color); padding-top: 20px;">
                        <div style="display: flex; justify-content: center; gap: 20px; font-size: 1rem; font-weight: 600;">
                            <button
                                on:click=move |_| {
                                    lang.set(Language::Vi);
                                    menu_open.set(false);
                                }
                                type="button"
                                style=move || if is_vi() {
                                    "background: none; border: none; color: var(--secondary); cursor: pointer; padding: 6px 12px; border-bottom: 2px solid var(--secondary);"
                                } else {
                                    "background: none; border: none; color: var(--text-muted); cursor: pointer; padding: 6px 12px;"
                                }
                            >
                                "Tiếng Việt"
                            </button>
                            <button
                                on:click=move |_| {
                                    lang.set(Language::En);
                                    menu_open.set(false);
                                }
                                type="button"
                                style=move || if is_en() {
                                    "background: none; border: none; color: var(--secondary); cursor: pointer; padding: 6px 12px; border-bottom: 2px solid var(--secondary);"
                                } else {
                                    "background: none; border: none; color: var(--text-muted); cursor: pointer; padding: 6px 12px;"
                                }
                            >
                                "English"
                            </button>
                        </div>
                    </li>

                    <li style="margin-top: 24px; width: 100%;">
                        <a href="https://shopee.vn/opendiy" target="_blank" rel="noopener noreferrer" class="btn btn-primary" style="width: 100%; text-align: center; display: block; padding: 12px; font-size: 1rem;">
                            {move || t!(lang, "Shopee Store", "Cửa hàng Shopee")()}
                        </a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}

/* ==========================================
Home Page Component
========================================== */

#[component]
fn HomePage() -> impl IntoView {
    let lang = expect_context::<LanguageContext>().lang;

    view! {
        <SeoHead
            title="open-diy | Custom 3D Printed Keyboards".to_string()
            description="Shop ergonomic 3D printed keyboards with open-source designs, custom switches, and premium keycaps.".to_string()
            path="/".to_string()
            image_path="/images/dactyl.png".to_string()
        />
        <JsonLd json=faq_json_ld()/>
        <div class="hero">
            <span class="hero-tag">{t!(lang, "Revolutionary Mechanical Keyboards", "Bàn Phím Cơ Đột Phá")}</span>
            <h1 class="hero-title">
                {t!(lang, "Ergonomic ", "Bàn Phím ")}
                <br/>
                <span class="gradient-text">{t!(lang, "3D Printed Keyboards", "In 3D Công Thái Học")}</span>
            </h1>
            <p class="hero-subtitle">
                {t!(lang, 
                    "Open-source designs manufactured in-house with high-performance plastics. contoured for your comfort, posture, and typing acoustics.", 
                    "Thiết kế nguồn mở được sản xuất trực tiếp bằng nhựa hiệu năng cao. Thiết kế ôm sát cổ tay tối ưu hóa công thái học và âm học."
                )}
            </p>
            <div class="hero-actions">
                <A href="/shop" attr:class="btn btn-primary">{t!(lang, "Browse Products", "Xem sản phẩm")}</A>
                <A href="/about" attr:class="btn btn-secondary">{t!(lang, "Our Philosophy", "Triết lý thiết kế")}</A>
            </div>
            <div class="hero-visual">
                <img src="/images/dactyl.png" alt="Dactyl Manuform Custom Keyboard" fetchpriority="high"/>
            </div>
        </div>

        <section class="features">
            <div class="section-header">
                <h2 class="section-title">
                    {t!(lang, "Why Choose ", "Tại sao chọn ")}
                    <span class="gradient-text">"open-diy"</span> "?"
                </h2>
                <p class="section-subtitle">
                    {t!(lang, 
                        "We merge advanced additive manufacturing with premium custom keyboard enthusiast specs.", 
                        "Chúng tôi kết hợp công nghệ in 3D tiên tiến với các thông số phím cơ custom cao cấp nhất."
                    )}
                </p>
            </div>
            <div class="grid grid-cols-3" style="max-width: 1200px; margin: 0 auto;">
                <div class="glass-card feature-card">
                    <div class="feature-icon">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline>
                        </svg>
                    </div>
                    <h3>{t!(lang, "Unrivaled Ergonomics", "Công thái học vượt trội")}</h3>
                    <p>
                        {t!(lang, 
                            "Sculpted split designs like the Dactyl place keys in natural column curvature, preventing wrist strain during long typing sessions.", 
                            "Thiết kế chia đôi dạng lòng chảo như Dactyl đặt các phím theo độ cong tự nhiên của ngón tay, chống mỏi cổ tay khi gõ lâu."
                        )}
                    </p>
                </div>
                <div class="glass-card feature-card">
                    <div class="feature-icon">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
                            <polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline>
                            <line x1="12" y1="22.08" x2="12" y2="12"></line>
                        </svg>
                    </div>
                    <h3>{t!(lang, "Precision 3D Printing", "In 3D siêu chuẩn xác")}</h3>
                    <p>
                        {t!(lang, 
                            "We use fine-tuned high-density print settings (0.2mm layer heights, gyroid infill) for superb acoustics and impact-resistant cases.", 
                            "Sử dụng cài đặt in mật độ cao (lớp in 0.2mm, họa tiết xương Gyroid) cho chất âm trầm đầm tay và độ bền vỏ vượt trội."
                        )}
                    </p>
                </div>
                <div class="glass-card feature-card">
                    <div class="feature-icon">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polygon points="12 2 2 7 12 12 22 7 12 2"></polygon>
                            <polyline points="2 17 12 22 22 17"></polyline>
                            <polyline points="2 12 12 17 22 12"></polyline>
                        </svg>
                    </div>
                    <h3>{t!(lang, "Enthusiast Specs Standard", "Chuẩn phím cơ Custom")}</h3>
                    <p>
                        {t!(lang, 
                            "Hot-swappable sockets let you replace switches without soldering. Pre-lubed stabs and acoustic foam are included in every build.", 
                            "Mạch hotswap thay switch nhanh không cần hàn. Stabilizer đã được cân wire, lube sẵn cùng foam tiêu âm đi kèm trong mỗi bản build."
                        )}
                    </p>
                </div>
            </div>
        </section>

        <section class="faq-section" style="max-width: 1200px; margin: 0 auto; padding: 40px 0 20px;">
            <div class="section-header">
                <span class="hero-tag" style="margin-bottom: 12px;">"FAQ"</span>
                <h2 class="section-title">{t!(lang, "Keyboard buying questions, answered.", "Giải đáp thắc mắc về bàn phím")}</h2>
                <p class="section-subtitle">
                    {t!(lang, 
                        "Helpful details for shoppers searching for ergonomic, open-source, and 3D printed keyboards.", 
                        "Thông tin hữu ích giúp bạn hiểu rõ hơn về các dòng bàn phím in 3D công thái học nguồn mở."
                    )}
                </p>
            </div>
            <div class="grid grid-cols-3" style="gap: 18px;">
                <details class="glass-card feature-card" open>
                    <summary style="cursor: pointer; font-weight: 700;">{t!(lang, "What makes open-diy different?", "Sự khác biệt của open-diy là gì?")}</summary>
                    <p style="margin-top: 12px;">
                        {t!(lang, 
                            "We combine open-source keyboard design with in-house 3D printing, custom switch choices, and direct links to checkout safely on Shopee.", 
                            "Chúng tôi kết hợp thiết kế bàn phím nguồn mở với việc in 3D tại chỗ, tùy chọn switch custom và liên kết trực tiếp mua hàng an toàn trên Shopee."
                        )}
                    </p>
                </details>
                <details class="glass-card feature-card">
                    <summary style="cursor: pointer; font-weight: 700;">{t!(lang, "Can I build an ergonomic keyboard here?", "Tôi có thể mua bàn phím công thái học ở đây không?")}</summary>
                    <p style="margin-top: 12px;">
                        {t!(lang, 
                            "Yes. The catalog highlights split and curved layouts such as Dactyl and Alice-inspired designs, all tuned for better comfort and desk positioning.", 
                            "Có. Danh mục sản phẩm nổi bật với các bố cục chia đôi và uốn cong như Dactyl và Alice, được tối ưu hóa cho sự thoải mái và tư thế gõ."
                        )}
                    </p>
                </details>
                <details class="glass-card feature-card">
                    <summary style="cursor: pointer; font-weight: 700;">{t!(lang, "Do you support DIY builders?", "Các bạn có hỗ trợ người tự build (DIY) không?")}</summary>
                    <p style="margin-top: 12px;">
                        {t!(lang, 
                            "Absolutely. The About page points to the open-source side of the project, and we list public GitHub repositories for builders who want to print and assemble by themselves.", 
                            "Hoàn toàn có. Trang giới thiệu hướng tới khía cạnh nguồn mở của dự án, và chúng tôi liệt kê các kho lưu trữ GitHub công khai cho những ai muốn tự in và lắp ráp."
                        )}
                    </p>
                </details>
            </div>
        </section>
    }
}

/* ==========================================
Catalog Page Component
========================================== */

#[component]
fn CatalogPage() -> impl IntoView {
    let lang = expect_context::<LanguageContext>().lang;
    let products = get_products();

    view! {
        <SeoHead
            title="open-diy Shop | Curated 3D Printed Keyboard Builds".to_string()
            description="Browse curated ergonomic keyboard builds including Dactyl, Frosted 60%, and Alice Curved layouts with 3D printed cases.".to_string()
            path="/shop".to_string()
            image_path="/images/frosted.png".to_string()
        />
        <section class="catalog-section">
            <div class="section-header">
                <span class="hero-tag" style="margin-bottom: 12px;">{t!(lang, "Catalog", "Danh mục")}</span>
                <h1 class="section-title">
                    {t!(lang, "Our ", "Các mẫu phím ")}
                    <span class="gradient-text">{t!(lang, "Curated Builds", "Custom Tuyển Chọn")}</span>
                </h1>
                <p class="section-subtitle">
                    {t!(lang, "Browse our professional ergonomic layouts and order securely on Shopee.", "Xem các bố cục công thái học chuyên nghiệp của chúng tôi và đặt hàng an toàn qua Shopee.")}
                </p>
            </div>
            <div class="glass-card" style="max-width: 1200px; margin: 0 auto 28px auto; padding: 20px 24px;">
                <p style="margin-bottom: 10px;">
                    {t!(lang, 
                        "Each build is designed for people searching for an ergonomic keyboard that feels custom without requiring a full DIY project from scratch.", 
                        "Mỗi bản build được thiết kế chuyên biệt cho những ai tìm kiếm bàn phím công thái học custom nhưng không muốn tự lắp ráp từ đầu."
                    )}
                </p>
                <p>
                    {t!(lang, 
                        "Click on any product to view its detailed specifications, and open the Shopee link to customize options and place your order.", 
                        "Bấm vào bất kỳ sản phẩm nào để xem thông số kỹ thuật chi tiết, và mở liên kết Shopee để lựa chọn tùy biến và đặt hàng."
                    )}
                </p>
            </div>

            <div class="grid grid-cols-3" style="max-width: 1200px; margin: 0 auto;">
                {move || {
                    let current_lang = lang.get();
                    products.iter().map(|p| {
                        let p_id = p.id.clone();
                        let p_img = p.image.clone();
                        let p_tag = p.tag(current_lang);
                        let p_name = p.name(current_lang);
                        let p_name_alt = p_name.clone();
                        let p_desc = p.desc(current_lang);
                        let p_price = p.price;
                        let detail_url_1 = format!("/product/{}", p_id);
                        let detail_url_2 = detail_url_1.clone();
                        let detail_url_3 = detail_url_1.clone();
                        view! {
                            <div class="glass-card product-card">
                                <A href=detail_url_1 attr:style="display: block; position: relative;">
                                    <div class="product-image-wrapper">
                                        <span class="product-tag">{p_tag}</span>
                                        <img src=p_img alt=p_name_alt loading="lazy" decoding="async"/>
                                    </div>
                                </A>
                                <div class="product-info">
                                    <A href=detail_url_2 attr:style="text-decoration: none; color: inherit;">
                                        <h3 class="product-title">{p_name}</h3>
                                    </A>
                                    <p class="product-desc">{p_desc}</p>
                                    <div class="product-meta">
                                        <span class="product-price">
                                            {t!(lang, "From $", "Từ $")()}{format!("{:.2}", p_price)}
                                        </span>
                                        <A href=detail_url_3 attr:class="btn btn-primary btn-sm">{t!(lang, "View Details", "Chi tiết")}</A>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </section>
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ActiveTab {
    Customizer,
    Estimator,
    SoundTest,
}

#[component]
fn KeyboardCustomizer(lang: RwSignal<Language>, #[prop(into)] product_name: String) -> impl IntoView {
    let (case_color, set_case_color) = signal("#18181b".to_string());
    let (keycap_color, set_keycap_color) = signal("#4b5563".to_string());
    let (cable_color, set_cable_color) = signal("#09090b".to_string());
    let prod_name = product_name.clone();
    let order_click = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            let get_case_name = move |hex: &str| -> String {
                match hex {
                    "#18181b" => "Stealth Black".to_string(),
                    "#14532d" => "Forest Green".to_string(),
                    "#db2777" => "Neon Pink".to_string(),
                    "#f4f4f5" => "Frosted White".to_string(),
                    "#0284c7" => "Cyber Blue".to_string(),
                    _ => hex.to_string()
                }
            };

            let get_keycap_name = move |hex: &str| -> String {
                match hex {
                    "#4b5563" => "Charcoal".to_string(),
                    "#e4e4e7" => "Classic Chalk".to_string(),
                    "#0891b2" => "Neon Cyan".to_string(),
                    "#7c3aed" => "Lavender".to_string(),
                    "#ea580c" => "Pastel Orange".to_string(),
                    _ => hex.to_string()
                }
            };

            let get_cable_name = move |hex: &str| -> String {
                match hex {
                    "#09090b" => "Stealth Black".to_string(),
                    "#06b6d4" => "Cyan Coil".to_string(),
                    "#8b5cf6" => "Violet Coil".to_string(),
                    "#e11d48" => "Ruby Coil".to_string(),
                    _ => hex.to_string()
                }
            };

            let name = prod_name.clone();
            let case = get_case_name(&case_color.get());
            let keycap = get_keycap_name(&keycap_color.get());
            let cable = get_cable_name(&cable_color.get());
            let eval_str = format!(
                "window.orderConfiguration('{}', '{}', '{}', '{}')", 
                name.replace('\'', "\\'"), 
                case.replace('\'', "\\'"), 
                keycap.replace('\'', "\\'"), 
                cable.replace('\'', "\\'")
            );
            let _ = js_sys::eval(&eval_str);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = prod_name;
            let _ = case_color;
            let _ = keycap_color;
            let _ = cable_color;
        }
    };

    view! {
        <div class="customizer-container glass-card" style="display: grid; grid-template-columns: 1fr; gap: 30px; padding: 24px; border-radius: 16px;">
            // Left Column: Interactive SVG Preview
            <div class="customizer-preview" style="display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 15px;">
                <h3 style="font-size: 1.15rem; color: #fff; margin: 0; text-transform: uppercase; letter-spacing: 0.05em; font-weight: 700; color: var(--secondary);">
                    {t!(lang, "Live Color Preview", "Xem trước màu sắc trực tiếp")}
                </h3>

                <svg width="100%" height="280" viewBox="0 0 600 300" style="background: rgba(0,0,0,0.4); border-radius: 12px; border: 1px solid var(--border-color); box-shadow: inset 0 0 40px rgba(0,0,0,0.6);">
                    // Coiled Cable
                    <path d="M 220 150 C 230 110, 240 110, 250 150 C 260 190, 270 190, 280 150 C 290 110, 300 110, 310 150 C 320 190, 330 190, 340 150 C 350 110, 360 110, 380 150" fill="none" stroke=move || cable_color.get() stroke-width="8" stroke-linecap="round" stroke-linejoin="round" style="transition: stroke 0.3s ease;"/>
                    
                    // Left Hand Case (Curved split keyboard half)
                    <path d="M 50 100 Q 130 70 210 100 Q 230 150 210 220 Q 130 250 50 220 Q 30 150 50 100 Z" fill=move || case_color.get() stroke="rgba(255,255,255,0.15)" stroke-width="3" style="transition: fill 0.3s ease;"/>
                    // Left Thumb cluster
                    <path d="M 180 180 Q 220 190 220 230 Q 180 250 150 230 Z" fill=move || case_color.get() stroke="rgba(255,255,255,0.15)" stroke-width="2" style="transition: fill 0.3s ease;"/>
                    
                    // Right Hand Case
                    <path d="M 550 100 Q 470 70 390 100 Q 370 150 390 220 Q 470 250 550 220 Q 570 150 550 100 Z" fill=move || case_color.get() stroke="rgba(255,255,255,0.15)" stroke-width="3" style="transition: fill 0.3s ease;"/>
                    // Right Thumb cluster
                    <path d="M 420 180 Q 380 190 380 230 Q 420 250 450 230 Z" fill=move || case_color.get() stroke="rgba(255,255,255,0.15)" stroke-width="2" style="transition: fill 0.3s ease;"/>
                    
                    // Left keycaps grid
                    <rect x="75" y="115" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="75" y="138" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="75" y="161" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="97" y="110" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="97" y="133" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="97" y="156" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="119" y="105" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="119" y="128" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="119" y="151" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="141" y="110" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="141" y="133" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="141" y="156" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="163" y="115" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="163" y="138" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="163" y="161" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="175" y="195" width="16" height="24" rx="3" fill=move || keycap_color.get() transform="rotate(-15 175 195)" style="transition: fill 0.3s ease;"/>
                    <rect x="195" y="200" width="16" height="24" rx="3" fill=move || keycap_color.get() transform="rotate(-15 195 200)" style="transition: fill 0.3s ease;"/>

                    // Right keycaps grid
                    <rect x="509" y="115" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="509" y="138" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="509" y="161" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="487" y="110" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="487" y="133" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="487" y="156" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="465" y="105" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="465" y="128" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="465" y="151" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="443" y="110" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="443" y="133" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="443" y="156" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="421" y="115" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="421" y="138" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="421" y="161" width="16" height="16" rx="3" fill=move || keycap_color.get() style="transition: fill 0.3s ease;"/>
                    <rect x="409" y="195" width="16" height="24" rx="3" fill=move || keycap_color.get() transform="rotate(15 409 195)" style="transition: fill 0.3s ease;"/>
                    <rect x="389" y="200" width="16" height="24" rx="3" fill=move || keycap_color.get() transform="rotate(15 389 200)" style="transition: fill 0.3s ease;"/>
                </svg>
            </div>

            // Right Column: Controls
            <div class="customizer-controls" style="display: flex; flex-direction: column; gap: 20px;">
                // Case Color Selection
                <div>
                    <h4 style="font-size: 0.9rem; text-transform: uppercase; color: var(--text-muted); margin-bottom: 8px;">
                        {t!(lang, "Case Color", "Màu Vỏ Phím")}
                    </h4>
                    <div style="display: flex; gap: 10px; flex-wrap: wrap;">
                        {[
                            ("#18181b", "Stealth Black"),
                            ("#14532d", "Forest Green"),
                            ("#db2777", "Neon Pink"),
                            ("#f4f4f5", "Frosted White"),
                            ("#0284c7", "Cyber Blue"),
                        ].into_iter().map(|(val, name)| {
                            let is_active = move || case_color.get() == val;
                            view! {
                                <button 
                                    on:click=move |_| set_case_color.set(val.to_string())
                                    type="button"
                                    class=move || if is_active() { "btn btn-primary btn-sm" } else { "btn btn-secondary btn-sm" }
                                    style="padding: 6px 12px; font-size: 0.8rem;"
                                >
                                    {name}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </div>

                // Keycap Color Selection
                <div>
                    <h4 style="font-size: 0.9rem; text-transform: uppercase; color: var(--text-muted); margin-bottom: 8px;">
                        {t!(lang, "Keycap Theme", "Màu Nút (Keycaps)")}
                    </h4>
                    <div style="display: flex; gap: 10px; flex-wrap: wrap;">
                        {[
                            ("#4b5563", "Charcoal"),
                            ("#e4e4e7", "Classic Chalk"),
                            ("#0891b2", "Neon Cyan"),
                            ("#7c3aed", "Lavender"),
                            ("#ea580c", "Pastel Orange"),
                        ].into_iter().map(|(val, name)| {
                            let is_active = move || keycap_color.get() == val;
                            view! {
                                <button 
                                    on:click=move |_| set_keycap_color.set(val.to_string())
                                    type="button"
                                    class=move || if is_active() { "btn btn-primary btn-sm" } else { "btn btn-secondary btn-sm" }
                                    style="padding: 6px 12px; font-size: 0.8rem;"
                                >
                                    {name}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </div>

                // Cable Selection
                <div>
                    <h4 style="font-size: 0.9rem; text-transform: uppercase; color: var(--text-muted); margin-bottom: 8px;">
                        {t!(lang, "USB-C Coiled Cable", "Cáp Xoắn Kết Nối")}
                    </h4>
                    <div style="display: flex; gap: 10px; flex-wrap: wrap;">
                        {[
                            ("#09090b", "Stealth Black"),
                            ("#06b6d4", "Cyan Coil"),
                            ("#8b5cf6", "Violet Coil"),
                            ("#e11d48", "Ruby Coil"),
                        ].into_iter().map(|(val, name)| {
                            let is_active = move || cable_color.get() == val;
                            view! {
                                <button 
                                    on:click=move |_| set_cable_color.set(val.to_string())
                                    type="button"
                                    class=move || if is_active() { "btn btn-primary btn-sm" } else { "btn btn-secondary btn-sm" }
                                    style="padding: 6px 12px; font-size: 0.8rem;"
                                >
                                    {name}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </div>

                // Pre-order configuration action
                <div style="border-top: 1px solid var(--border-color); padding-top: 20px; margin-top: 10px;">
                    <button 
                        on:click=order_click
                        type="button"
                        class="btn btn-primary"
                        style="width: 100%; display: flex; align-items: center; justify-content: center; gap: 8px; background: linear-gradient(135deg, #0084FF, #00C6FF); border: none;"
                    >
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M12 2C6.48 2 2 6.14 2 11.25c0 2.91 1.45 5.51 3.75 7.15.19.14.31.36.31.6l-.02 1.89c-.01.48.51.81.93.57l2.12-1.22c.17-.1.38-.11.56-.05 1.44.43 2.97.66 4.55.66 5.52 0 10-4.14 10-9.25S17.52 2 12 2zm1.18 11.63l-2.02-2.15-3.92 2.15c-.41.22-.89-.24-.65-.65l2.02-3.48 2.02 2.15 3.92-2.15c.41-.22.89.24.65.65l-2.02 3.48z"/>
                        </svg>
                        {t!(lang, "Pre-order this Configuration", "Đặt mua cấu hình này")}
                    </button>
                    <p style="font-size: 0.75rem; color: var(--text-muted); margin-top: 8px; text-align: center; line-height: 1.4;">
                        {t!(lang, "* The configuration will be copied to your clipboard.", "* Cấu hình chi tiết sẽ được tự động sao chép vào bộ nhớ tạm.")}
                    </p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn HandEstimator(lang: RwSignal<Language>) -> impl IntoView {
    let (finger_len, set_finger_len) = signal(7.5);
    let (palm_width, set_palm_width) = signal(8.5);

    let recommendation = move || {
        let fl = finger_len.get();
        let pw = palm_width.get();
        let current_lang = lang.get();
        if fl < 6.8 || pw < 7.5 {
            (
                match current_lang {
                    Language::En => "Corne Cherry Split (42 keys)",
                    Language::Vi => "Bàn phím Corne Cherry Split (42 phím)",
                },
                match current_lang {
                    Language::En => "With smaller hands, a 40% split layout keeps keys close and eliminates finger stretching. This minimizes muscle fatigue.",
                    Language::Vi => "Với kích thước bàn tay nhỏ, layout split 40% giúp phím luôn trong tầm tay, loại bỏ việc với ngón gây căng cơ.",
                },
                "Hand Size: Small / Fit: Ultra-Compact"
            )
        } else if fl > 8.5 || pw > 9.5 {
            (
                match current_lang {
                    Language::En => "Dactyl-ManuForm 5x6 (Extended Cluster)",
                    Language::Vi => "Bàn phím Dactyl-ManuForm 5x6 (Bản mở rộng)",
                },
                match current_lang {
                    Language::En => "Larger hands are perfectly supported by the sculptured 3D keywells of the Dactyl 5x6. The deep hand-scaffolding provides natural curvature.",
                    Language::Vi => "Bàn tay lớn được nâng đỡ hoàn hảo bởi các lòng chảo 3D của dòng Dactyl 5x6. Độ võng sâu giúp các ngón tay duỗi cong tự nhiên.",
                },
                "Hand Size: Large / Fit: Extended Comfort"
            )
        } else {
            (
                match current_lang {
                    Language::En => "Dactyl-ManuForm 5x6 or Alice Curved",
                    Language::Vi => "Dactyl-ManuForm 5x6 hoặc Alice Curved",
                },
                match current_lang {
                    Language::En => "Your standard hand dimensions are highly versatile. The Dactyl 5x6 offers ultimate ergonomics, while Alice Curved offers a shorter learning curve.",
                    Language::Vi => "Số đo tay tiêu chuẩn của bạn cực kỳ linh hoạt. Dactyl 5x6 mang lại công thái học tối đa, còn Alice Curved là lựa chọn dễ làm quen nhất.",
                },
                "Hand Size: Medium / Fit: Ergonomic Standard"
            )
        }
    };

    view! {
        <div class="estimator-container glass-card" style="padding: 24px; border-radius: 16px; display: grid; grid-template-columns: 1fr; gap: 30px;">
            // Controls Input Panel
            <div style="display: flex; flex-direction: column; gap: 20px;">
                <h3 style="font-size: 1.15rem; color: #fff; margin: 0; text-transform: uppercase; letter-spacing: 0.05em; font-weight: 700; color: var(--secondary);">
                    {t!(lang, "Enter Hand Dimensions", "Nhập kích thước bàn tay")}
                </h3>
                
                // Slide 1: Middle Finger Length
                <div class="slider-group" style="display: flex; flex-direction: column; gap: 8px;">
                    <div style="display: flex; justify-content: space-between; font-size: 0.9rem;">
                        <span style="color: var(--text-muted);">{t!(lang, "Middle Finger Length", "Chiều dài ngón giữa")}</span>
                        <span style="color: #fff; font-weight: 600;">{move || format!("{:.1} cm", finger_len.get())}</span>
                    </div>
                    <input 
                        type="range" 
                        min="5.0" 
                        max="11.0" 
                        step="0.1" 
                        value=move || finger_len.get().to_string()
                        on:input=move |ev| {
                            if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                set_finger_len.set(val);
                            }
                        }
                        style="width: 100%; accent-color: var(--secondary); cursor: pointer;"
                    />
                </div>

                // Slide 2: Palm Width
                <div class="slider-group" style="display: flex; flex-direction: column; gap: 8px;">
                    <div style="display: flex; justify-content: space-between; font-size: 0.9rem;">
                        <span style="color: var(--text-muted);">{t!(lang, "Palm Width", "Chiều rộng lòng bàn tay")}</span>
                        <span style="color: #fff; font-weight: 600;">{move || format!("{:.1} cm", palm_width.get())}</span>
                    </div>
                    <input 
                        type="range" 
                        min="6.0" 
                        max="12.0" 
                        step="0.1" 
                        value=move || palm_width.get().to_string()
                        on:input=move |ev| {
                            if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                set_palm_width.set(val);
                            }
                        }
                        style="width: 100%; accent-color: var(--secondary); cursor: pointer;"
                    />
                </div>
            </div>

            // Recommendation Display Panel
            <div class="estimator-result" style="background: rgba(0,0,0,0.25); border: 1px solid var(--border-color); border-radius: 12px; padding: 20px; display: flex; flex-direction: column; gap: 12px;">
                <span class="hero-tag" style="align-self: flex-start; margin-bottom: 0; font-size: 0.75rem; padding: 4px 10px;">
                    {move || recommendation().2}
                </span>
                <h4 style="font-size: 1.4rem; color: #fff; margin: 0; line-height: 1.2;">
                    {move || recommendation().0}
                </h4>
                <p style="color: var(--text-muted); font-size: 0.9rem; line-height: 1.6; text-align: justify; margin: 0;">
                    {move || recommendation().1}
                </p>
                <div style="border-top: 1px solid var(--border-color); padding-top: 10px; margin-top: 5px; font-size: 0.8rem; color: var(--secondary); display: flex; align-items: center; gap: 6px;">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="12" cy="12" r="10"></circle>
                        <line x1="12" y1="16" x2="12" y2="12"></line>
                        <line x1="12" y1="8" x2="12.01" y2="8"></line>
                    </svg>
                    <span>{t!(lang, "Optimized for anatomical alignment", "Được tối ưu hóa theo cấu trúc sinh học cổ tay")}</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SoundTestPlayer(lang: RwSignal<Language>) -> impl IntoView {
    let play_sound = move |sound_type: &'static str| {
        #[cfg(target_arch = "wasm32")]
        {
            let eval_str = format!("window.playSwitchSound('{}')", sound_type);
            let _ = js_sys::eval(&eval_str);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = sound_type;
        }
    };

    view! {
        <div class="sound-test-container glass-card" style="padding: 24px; border-radius: 16px; display: flex; flex-direction: column; gap: 20px;">
            <h3 style="font-size: 1.15rem; color: #fff; margin: 0; text-transform: uppercase; letter-spacing: 0.05em; font-weight: 700; color: var(--secondary);">
                {t!(lang, "Switch Sound Acoustics", "Thử nghiệm âm thanh gõ Switch")}
            </h3>
            <p style="font-size: 0.88rem; color: var(--text-muted); margin: 0; line-height: 1.5;">
                {t!(lang, 
                   "Click any switch to simulate its mechanical acoustic sound test synthesized live using Web Audio API.", 
                   "Click vào từng loại switch để nghe thử âm thanh gõ cơ học được giả lập trực tiếp qua Web Audio API."
                )}
            </p>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin-top: 10px;">
                // Yellow Linear Switch
                <button 
                    on:click=move |_| play_sound("linear")
                    type="button"
                    class="sound-card-btn"
                    style="background: rgba(234, 179, 8, 0.05); border: 1px solid rgba(234, 179, 8, 0.2); border-radius: 12px; padding: 18px; text-align: left; cursor: pointer; transition: all 0.2s;"
                >
                    <div style="display: flex; align-items: center; gap: 10px; margin-bottom: 8px;">
                        <span style="width: 14px; height: 14px; background: #eab308; border-radius: 4px; box-shadow: 0 0 8px #eab308;"></span>
                        <h4 style="margin: 0; font-size: 1rem; color: #fff;">"Linear (Yellow)"</h4>
                    </div>
                    <p style="margin: 0; font-size: 0.8rem; color: var(--text-muted); line-height: 1.4;">
                        {t!(lang, "Deep, silent, thocky tone. Smooth keypress travel.", "Âm thanh trầm ấm (Thocky), gõ êm mượt không có khấc cản.")}
                    </p>
                </button>

                // Brown Tactile Switch
                <button 
                    on:click=move |_| play_sound("tactile")
                    type="button"
                    class="sound-card-btn"
                    style="background: rgba(139, 92, 26, 0.05); border: 1px solid rgba(139, 92, 26, 0.2); border-radius: 12px; padding: 18px; text-align: left; cursor: pointer; transition: all 0.2s;"
                >
                    <div style="display: flex; align-items: center; gap: 10px; margin-bottom: 8px;">
                        <span style="width: 14px; height: 14px; background: #a16207; border-radius: 4px; box-shadow: 0 0 8px #a16207;"></span>
                        <h4 style="margin: 0; font-size: 1rem; color: #fff;">"Tactile (Brown)"</h4>
                    </div>
                    <p style="margin: 0; font-size: 0.8rem; color: var(--text-muted); line-height: 1.4;">
                        {t!(lang, "Moderate bump, round clack. Crisp tactile typing feedback.", "Cực kỳ cơ học với khấc cản nhẹ. Âm thanh clack tròn trịa.")}
                    </p>
                </button>

                // Blue Clicky Switch
                <button 
                    on:click=move |_| play_sound("clicky")
                    type="button"
                    class="sound-card-btn"
                    style="background: rgba(6, 182, 212, 0.05); border: 1px solid rgba(6, 182, 212, 0.2); border-radius: 12px; padding: 18px; text-align: left; cursor: pointer; transition: all 0.2s;"
                >
                    <div style="display: flex; align-items: center; gap: 10px; margin-bottom: 8px;">
                        <span style="width: 14px; height: 14px; background: #06b6d4; border-radius: 4px; box-shadow: 0 0 8px #06b6d4;"></span>
                        <h4 style="margin: 0; font-size: 1rem; color: #fff;">"Clicky (Blue)"</h4>
                    </div>
                    <p style="margin: 0; font-size: 0.8rem; color: var(--text-muted); line-height: 1.4;">
                        {t!(lang, "Loud double-click snap. High pitched typewriter tone.", "Âm thanh click đanh giòn như tiếng máy đánh chữ truyền thống.")}
                    </p>
                </button>
            </div>
        </div>
    }
}

/* ==========================================
Product Detail Page Component
========================================== */

#[component]
fn ProductDetailPage() -> impl IntoView {
    let params = use_params_map();
    let lang = expect_context::<LanguageContext>().lang;
    
    let product_id = move || params.with(|p| p.get("id").unwrap_or_default());
    
    let product = move || {
        let id = product_id();
        get_products().into_iter().find(|p| p.id == id)
    };

    let (active_tab, set_active_tab) = signal(ActiveTab::Customizer);

    view! {
        {move || {
            let current_lang = lang.get();
            product().map(|p| {
                let (selected_img, set_selected_img) = signal(p.image.clone());
                let images = p.images.clone();
                let images_gallery = images.clone();
                let images_detail = images.clone();
                let name = p.name(current_lang);
                let img = p.image.clone();
                let desc = p.detailed_desc(current_lang);
                let price_str = format!("{:.2}", p.price);
                let shopee = p.shopee_url.clone();
                let github = p.github_url.clone();
                let specs = p.specs(current_lang);
                let page_path = format!("/product/{}", p.id);
                let page_path_alt = page_path.clone();
                let name_alt = name.clone();
                let desc_alt = p.desc(current_lang);
                let img_alt = img.clone();
                let price_alt = p.price;

                view! {
                    <SeoHead
                        title=format!("open-diy | {}", name)
                        description=desc_alt.clone()
                        path=page_path
                        image_path=img.clone()
                    />
                    <JsonLd json=product_json_ld(&name_alt, &desc_alt, &img_alt, &page_path_alt, price_alt)/>
                    
                    <section class="builder-section" style="max-width: 1200px; margin: 0 auto; padding: 60px 4% 100px 4%;">
                        <div style="margin-bottom: 24px;">
                            <A href="/shop" attr:style="color: var(--secondary); text-decoration: none; font-weight: 500; display: inline-flex; align-items: center; gap: 6px;">
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                                    <line x1="19" y1="12" x2="5" y2="12"></line>
                                    <polyline points="12 19 5 12 12 5"></polyline>
                                </svg>
                                {t!(lang, "Back to Catalog", "Quay lại danh mục")}
                            </A>
                        </div>

                        <div class="builder-container">
                            // Left Column - Image Gallery (Only gallery, no specs card to keep it clean)
                            <div class="builder-preview-pane">
                                <div class="preview-image-container" style="aspect-ratio: 16/10; overflow: hidden; border-radius: var(--radius-lg); border: 1px solid var(--border-color); background: rgba(255,255,255,0.02);">
                                    <img src=selected_img style="width: 100%; height: 100%; object-fit: cover;" alt=name.clone() fetchpriority="high"/>
                                </div>
                                
                                // Thumbnail list
                                <div class="preview-gallery" style="display: flex; gap: 12px; margin-top: 14px; margin-bottom: 24px;">
                                    {images_gallery.into_iter().map(|img_url| {
                                        let img_url_clone1 = img_url.clone();
                                        let img_url_clone2 = img_url.clone();
                                        let is_active = move || selected_img.get() == img_url_clone1;
                                        view! {
                                            <button 
                                                on:click=move |_| set_selected_img.set(img_url_clone2.clone())
                                                type="button"
                                                style=move || if is_active() {
                                                    "width: 80px; height: 55px; border-radius: var(--radius-md); overflow: hidden; border: 2px solid var(--secondary); background: none; padding: 0; cursor: pointer; transition: all 0.2s; opacity: 1;"
                                                } else {
                                                    "width: 80px; height: 55px; border-radius: var(--radius-md); overflow: hidden; border: 1px solid var(--border-color); background: none; padding: 0; cursor: pointer; transition: all 0.2s; opacity: 0.5;"
                                                }
                                            >
                                                <img src=img_url style="width: 100%; height: 100%; object-fit: cover;" alt="Product thumbnail"/>
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>

                            // Right Column - Buying Info & Specs
                            <div class="builder-controls-pane">
                                <div class="glass-card" style="display: flex; flex-direction: column; gap: 20px; padding: 28px;">
                                    <div>
                                        <span class="hero-tag" style="margin-bottom: 10px;">{p.tag(current_lang)}</span>
                                        <h1 class="gradient-text" style="font-size: 2.3rem; margin-top: 5px; line-height: 1.2;">{name.clone()}</h1>
                                        <div style="font-family: var(--font-display); font-size: 1.8rem; font-weight: 700; color: #fff; margin-top: 10px; margin-bottom: 12px;">
                                            {t!(lang, "From $", "Từ $")()}{price_str.clone()}
                                        </div>
                                        <p style="color: var(--text-muted); line-height: 1.6; font-size: 0.92rem; margin: 0; text-align: justify;">
                                            {p.desc(current_lang)}
                                        </p>
                                    </div>

                                    <div style="display: flex; flex-direction: column; gap: 12px; border-top: 1px solid var(--border-color); border-bottom: 1px solid var(--border-color); padding: 20px 0;">
                                        <a href=shopee target="_blank" rel="noopener noreferrer" class="btn btn-secondary" style="width: 100%; opacity: 0.85; display: flex; align-items: center; justify-content: center;">
                                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-right: 8px;">
                                                <path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z"></path>
                                                <line x1="3" y1="6" x2="21" y2="6"></line>
                                                <path d="M16 10a4 4 0 0 1-8 0"></path>
                                            </svg>
                                            {t!(lang, "Customize & Buy on Shopee (Coming Soon)", "Tùy biến & Mua trên Shopee (Sắp mở)")}
                                        </a>

                                        <a 
                                            href="https://m.me/1111759575360830" 
                                            target="_blank" 
                                            rel="noopener noreferrer" 
                                            class="btn btn-primary" 
                                            style="width: 100%; background: linear-gradient(135deg, #0084FF, #00C6FF); border: none; display: flex; align-items: center; justify-content: center;"
                                        >
                                            <svg width="18" height="18" viewBox="0 0 24 24" fill="currentColor" style="margin-right: 8px;">
                                                <path d="M12 2C6.48 2 2 6.14 2 11.25c0 2.91 1.45 5.51 3.75 7.15.19.14.31.36.31.6l-.02 1.89c-.01.48.51.81.93.57l2.12-1.22c.17-.1.38-.11.56-.05 1.44.43 2.97.66 4.55.66 5.52 0 10-4.14 10-9.25S17.52 2 12 2zm1.18 11.63l-2.02-2.15-3.92 2.15c-.41.22-.89-.24-.65-.65l2.02-3.48 2.02 2.15 3.92-2.15c.41-.22.89.24.65.65l-2.02 3.48z"/>
                                            </svg>
                                            {t!(lang, "Order Direct via Messenger (Pre-order)", "Đặt hàng trực tiếp qua Messenger (Pre-order)")}
                                        </a>
                                        
                                        {move || github.clone().map(|url| view! {
                                            <a href=url target="_blank" rel="noopener noreferrer" class="btn btn-secondary" style="width: 100%;">
                                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-right: 8px;">
                                                    <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                                                </svg>
                                                {t!(lang, "DIY Files (GitHub)", "Tài liệu DIY (GitHub)")}
                                            </a>
                                        })}
                                    </div>

                                    <div style="margin-top: 5px;">
                                        <h4 style="font-size: 0.95rem; color: #fff; margin-bottom: 12px; text-transform: uppercase; letter-spacing: 0.05em; font-weight: 700; color: var(--secondary);">
                                            {t!(lang, "Specifications", "Thông số kỹ thuật")}
                                        </h4>
                                        <ul style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 8px;">
                                            {specs.into_iter().map(|spec| view! {
                                                <li style="display: flex; align-items: flex-start; gap: 8px; color: var(--text-main); font-size: 0.88rem; line-height: 1.4;">
                                                    <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" style="color: var(--secondary); flex-shrink: 0; margin-top: 2px;">
                                                        <polyline points="20 6 9 17 4 12"></polyline>
                                                    </svg>
                                                    <span>{spec}</span>
                                                </li>
                                            }).collect_view()}
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Interactive Experience Section (Tabs)
                        <div class="interactive-experience-section" style="margin-top: 60px; margin-bottom: 20px;">
                            <div class="section-header">
                                <span class="hero-tag" style="margin-bottom: 12px;">{t!(lang, "Hands-On", "Tương tác")}</span>
                                <h2 class="section-title">
                                    {t!(lang, "Interactive ", "Trải nghiệm ")}
                                    <span class="gradient-text">{t!(lang, "Studio", "Trực quan")}</span>
                                </h2>
                                <p class="section-subtitle">
                                    {t!(lang, "Choose colors, calculate hand ergonomics, and listen to switch click acoustics.", "Chọn màu sắc, ước tính kích thước tay công thái học và nghe thử âm thanh switch.")}
                                </p>
                            </div>

                            // Tab Buttons
                            <div class="tabs-nav" style="display: flex; justify-content: center; gap: 15px; margin-bottom: 30px; flex-wrap: wrap;">
                                <button 
                                    on:click=move |_| set_active_tab.set(ActiveTab::Customizer)
                                    type="button"
                                    class=move || if active_tab.get() == ActiveTab::Customizer { "btn btn-primary" } else { "btn btn-secondary" }
                                >
                                    {t!(lang, "Color Customizer", "Bộ phối màu phím")}
                                </button>
                                <button 
                                    on:click=move |_| set_active_tab.set(ActiveTab::Estimator)
                                    type="button"
                                    class=move || if active_tab.get() == ActiveTab::Estimator { "btn btn-primary" } else { "btn btn-secondary" }
                                >
                                    {t!(lang, "Hand Size Estimator", "Đo kích thước tay")}
                                </button>
                                <button 
                                    on:click=move |_| set_active_tab.set(ActiveTab::SoundTest)
                                    type="button"
                                    class=move || if active_tab.get() == ActiveTab::SoundTest { "btn btn-primary" } else { "btn btn-secondary" }
                                >
                                    {t!(lang, "Switch Sound Acoustics", "Thử âm thanh switch")}
                                </button>
                            </div>

                            // Active Tab Content
                            <div class="tab-content" style="max-width: 800px; margin: 0 auto;">
                                {move || match active_tab.get() {
                                    ActiveTab::Customizer => view! { <KeyboardCustomizer lang=lang product_name=name.clone()/> }.into_any(),
                                    ActiveTab::Estimator => view! { <HandEstimator lang=lang/> }.into_any(),
                                    ActiveTab::SoundTest => view! { <SoundTestPlayer lang=lang/> }.into_any(),
                                }}
                            </div>
                        </div>

                        // Product Details Section (Full Width, below the builder container grid)
                        <div class="product-details-container">
                            <h3 style="font-size: 1.5rem; color: #fff; margin-bottom: 20px; border-bottom: 1px solid var(--border-color); padding-bottom: 10px; font-weight: 700; text-align: center; letter-spacing: 0.05em; text-transform: uppercase;">
                                {t!(lang, "Product Details & Deep Dive", "Chi tiết sản phẩm & Đi sâu tìm hiểu")}
                            </h3>
                            
                            {desc.split("\n\n").enumerate().map(|(idx, block)| {
                                let block_img = images_detail.get(idx).or_else(|| images_detail.first()).cloned().unwrap_or_default();
                                let is_even = idx % 2 == 0;
                                
                                let (title, content) = if let Some((t, c)) = block.split_once('\n') {
                                    if t.ends_with(':') {
                                        (t.trim_end_matches(':').to_string(), c.to_string())
                                    } else {
                                        (String::new(), block.to_string())
                                    }
                                } else {
                                    (String::new(), block.to_string())
                                };
                                
                                let row_class = if is_even {
                                    "glass-card product-details-row"
                                } else {
                                    "glass-card product-details-row reverse"
                                };

                                view! {
                                    <div class=row_class>
                                        // Image Pane
                                        <div class="product-details-image-pane">
                                            <img src=block_img alt="Detail showcase image" loading="lazy"/>
                                        </div>
                                        
                                        // Content Pane
                                        <div class="product-details-content-pane">
                                            {if !title.is_empty() {
                                                let title_str = title.clone();
                                                view! {
                                                    <h4 style="font-size: 1.2rem; color: var(--secondary); font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em; margin: 0;">
                                                        {title_str}
                                                    </h4>
                                                }.into_any()
                                            } else {
                                                view! {}.into_any()
                                            }}
                                            <p style="color: var(--text-muted); line-height: 1.8; font-size: 0.95rem; text-align: justify; margin: 0;">
                                                {content.clone()}
                                            </p>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </section>
                }
            })
        }}
    }
}

/* ==========================================
About Page Component
========================================== */

#[component]
fn AboutPage() -> impl IntoView {
    let lang = expect_context::<LanguageContext>().lang;
    view! {
        <SeoHead
            title="open-diy About | Open-Source Keyboard Manufacturing".to_string()
            description="Learn how open-diy combines open-source keyboard designs with precise 3D printing, ergonomic layouts, and DIY-friendly hardware support.".to_string()
            path="/about".to_string()
            image_path="/images/alice.png".to_string()
        />
        <div class="about-section">
            <div class="about-header">
                <span class="hero-tag">{t!(lang, "Philosophy", "Triết lý")}</span>
                <h1 class="gradient-text">{t!(lang, "Open-Source Keyboards", "Bàn phím Nguồn mở")}</h1>
                <p class="section-subtitle">
                    {t!(lang, 
                        "We believe hardware should be owned and customized, not locked behind IP walls.", 
                        "Chúng tôi tin rằng phần cứng cần được sở hữu và tùy biến bởi người dùng, chứ không bị khóa sau bức tường bản quyền."
                    )}
                </p>
            </div>

            <img src="/images/alice.png" alt="Close-up of a 3D printed Alice keyboard case" class="about-image" loading="lazy" decoding="async"/>

            <div class="about-body">
                <p>
                    <strong>"open-diy"</strong>
                    {t!(lang, 
                        " was founded on the idea that standard, mass-produced rectangular keyboards don't fit human anatomy or satisfy our tactile expectations. The custom keyboard hobby has exploded, but premium cases remain expensive and difficult to source. We unlock this by utilizing high-precision industrial 3D printers.", 
                        " được thành lập dựa trên ý tưởng rằng các bàn phím hình chữ nhật sản xuất hàng loạt thông thường không phù hợp với giải phẫu học của tay người hoặc đáp ứng kỳ vọng về xúc giác. Thú chơi bàn phím cơ custom đã bùng nổ, nhưng các bộ vỏ premium vẫn đắt đỏ và khó tiếp cận. Chúng tôi giải quyết điều đó bằng cách sử dụng máy in 3D công nghiệp độ chính xác cao."
                    )}
                </p>

                <p>
                    {t!(lang, 
                        "If you are looking for a 3D printed keyboard shop that still feels approachable, the goal here is to make the learning curve shorter without removing the enthusiast details people care about.", 
                        "Nếu bạn đang tìm kiếm một cửa hàng bán phím in 3D dễ tiếp cận, mục tiêu ở đây là làm giảm thời gian tìm hiểu của bạn nhưng vẫn giữ nguyên các chi tiết custom mà những người đam mê phím cơ mong đợi."
                    )}
                </p>

                <div class="about-highlight-box">
                    <h3 style="margin-bottom: 12px; font-size: 1.2rem; color: #fff;">{t!(lang, "Our Printing Philosophy", "Triết lý in 3D của chúng tôi")}</h3>
                    <p>
                        {t!(lang, 
                            "We run fine-tuned, speed-reduced 3D printer settings to produce sturdy shell walls. By selecting PETG and dense ASA filaments instead of cheap PLA, our cases are structurally stable and do not wrap or degrade. The internal layout utilizes a grid of hollow spaces that acts as a natural dampening chamber, converting harsh typewriter clacks into deep, satisfying acoustics.", 
                            "Chúng tôi vận hành máy in với tốc độ được tinh chỉnh chậm hơn để tạo ra các lớp thành vỏ cực kỳ chắc chắn. Bằng cách chọn sợi nhựa PETG và ASA mật độ cao thay vì nhựa PLA rẻ tiền, vỏ phím của chúng tôi có kết cấu bền vững, không bị cong vênh hay lão hóa theo thời gian. Bố cục bên trong sử dụng lưới các khoảng trống hoạt động như một buồng tiêu âm tự nhiên, biến tiếng lách cách đanh tai thành âm gõ phím trầm ấm, dễ chịu."
                        )}
                    </p>
                </div>

                <h2 style="font-size: 1.75rem; margin-top: 15px;">{t!(lang, "Additive Manufacturing Specs", "Thông số sản xuất bồi đắp")}</h2>
                <ul class="specs-list">
                    <li>
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                        <span>{t!(lang, "0.2mm high-precision layers", "Lớp in độ chính xác cao 0.2mm")}</span>
                    </li>
                    <li>
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                        <span>{t!(lang, "5 outer perimeters for structural mass", "5 lớp thành ngoài để gia tăng khối lượng cấu trúc")}</span>
                    </li>
                    <li>
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                        <span>{t!(lang, "25% acoustic Gyroid infill pattern", "Họa tiết xương Gyroid mật độ 25% tối ưu âm học")}</span>
                    </li>
                    <li>
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                        <span>{t!(lang, "Industrial active-heated enclosures", "Buồng sấy in chủ động công nghiệp")}</span>
                    </li>
                </ul>

                <p>
                    {t!(lang, 
                        "Because our keyboards are open-source projects, we actively support the DIY community. If you own a 3D printer, you can download all casing files (STL/STEP) directly from public Github repositories. If you want us to handle the printing, soldering, lubing, and assembly - that is where our store comes in!", 
                        "Vì bàn phím của chúng tôi là dự án nguồn mở, chúng tôi tích cực hỗ trợ cộng đồng DIY. Nếu bạn sở hữu máy in 3D, bạn có thể tải toàn bộ tệp thiết kế vỏ (STL/STEP) trực tiếp từ các kho Github công khai. Còn nếu bạn muốn chúng tôi lo phần in ấn, hàn mạch, lube switch và lắp ráp - đó chính là lý do cửa hàng ra đời!"
                    )}
                </p>

                <div class="glass-card" style="margin-top: 24px; padding: 20px;">
                    <h3 style="margin-bottom: 12px;">{t!(lang, "Good fit for shoppers who want", "Phù hợp cho những khách hàng tìm kiếm")}</h3>
                    <ul class="specs-list">
                        <li><span>{t!(lang, "An ergonomic keyboard that is fully customizable", "Một bàn phím công thái học có khả năng tùy biến hoàn toàn")}</span></li>
                        <li><span>{t!(lang, "A 3D printed keyboard case with premium finish options", "Vỏ bàn phím in 3D với các tùy chọn bề mặt hoàn thiện cao cấp")}</span></li>
                        <li><span>{t!(lang, "A path from DIY open-source parts to a finished order", "Một con đường từ các linh kiện nguồn mở DIY đến sản phẩm hoàn thiện")}</span></li>
                    </ul>
                </div>

                <div style="text-align: center; margin-top: 30px;">
                    <a href="https://github.com/leptos-rs/start-axum" target="_blank" rel="noopener noreferrer" class="btn btn-secondary" style="display: inline-flex; align-items: center; gap: 8px;">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                        </svg>
                        {t!(lang, "Source Code on GitHub", "Mã nguồn trên GitHub")}
                    </a>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChatMessage {
    pub is_ai: bool,
    pub text: String,
}

#[component]
fn AiAssistant() -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let (input, set_input) = signal(String::new());
    
    // Initial greeting
    let initial_msg = ChatMessage {
        is_ai: true,
        text: "Xin chào! Tôi là Trợ lý AI của Open-DIY. Tôi có thể giúp bạn tìm hiểu về bàn phím Corne, Dactyl, tư vấn kích thước đo tay hoặc hướng dẫn đặt cọc. Bạn cần hỗ trợ gì hôm nay?".to_string(),
    };
    let (messages, set_messages) = signal(vec![initial_msg]);

    let handle_send = move |text_val: String| {
        if text_val.trim().is_empty() { return; }
        
        // Add user message
        set_messages.update(|msgs| msgs.push(ChatMessage {
            is_ai: false,
            text: text_val.clone(),
        }));
        
        set_input.set(String::new());
        
        // Find best match response
        let text_lower = text_val.to_lowercase();
        let response_text = if text_lower.contains("corne") {
            "Bàn phím Corne Cherry Split là dòng phím tách đôi 40% (42 phím), có màn hình OLED hiển thị thông số và hệ thống LED RGB rực rỡ. Rất nhỏ gọn và phù hợp cho người có cỡ tay nhỏ đến trung bình. Giá từ $129.".to_string()
        } else if text_lower.contains("dactyl") || text_lower.contains("manuform") {
            "Bàn phím Dactyl-Manuform có thiết kế dạng lòng chảo cong 3D ôm sát theo hướng ngón tay, giúp giảm tối đa mỏi vai gáy và cổ tay. Phù hợp cho lập trình viên và người gõ phím cường độ cao. Giá từ $219.".to_string()
        } else if text_lower.contains("bảo hành") || text_lower.contains("hỏng") || text_lower.contains("sửa") {
            "Tất cả phím chính hãng Open-DIY đều được bảo hành phần cứng (mạch PCB, mối hàn, IC điều khiển) trong 12 tháng. Hỗ trợ sửa chữa trọn đời sản phẩm.".to_string()
        } else if text_lower.contains("đo tay") || text_lower.contains("cỡ tay") || text_lower.contains("kích thước") {
            "Bạn có thể dùng tính năng 'Đo kích thước tay' ngay tại trang sản phẩm trên website. Đo chiều dài ngón giữa và bề ngang lòng bàn tay để hệ thống tự động gợi ý phím Corne hay Dactyl phù hợp!".to_string()
        } else if text_lower.contains("sound") || text_lower.contains("âm thanh") || text_lower.contains("gõ thử") {
            "Chúng tôi có bộ phát thử âm thanh switch (Linear, Tactile, Clicky) tích hợp sẵn trong trang chi tiết sản phẩm. Bạn hãy ghé xem sản phẩm và bấm tab 'Thử âm thanh switch' để nghe nhé!".to_string()
        } else if text_lower.contains("cọc") || text_lower.contains("mua") || text_lower.contains("đặt") || text_lower.contains("thanh toán") {
            "Quy trình đặt mua của chúng tôi: Bạn chọn màu sắc vỏ/keycap trên trang web, copy cấu hình gửi qua Messenger và chuyển khoản cọc 50%. Shop sẽ tiến hành lắp ráp thủ công trong 3-5 ngày và ship COD phần còn lại.".to_string()
        } else {
            "Tôi chưa hiểu rõ câu hỏi của bạn. Bạn có thể hỏi về: Corne, Dactyl, Đo tay, Sound test, Bảo hành, hoặc nhấn 'Trò chuyện trực tiếp' để gửi tin nhắn đến Messenger của shop nhé!".to_string()
        };

        // Add AI response immediately
        set_messages.update(|msgs| msgs.push(ChatMessage {
            is_ai: true,
            text: response_text,
        }));
    };
    
    let send_suggestion = move |text: &'static str| {
        let t = text.to_string();
        handle_send(t);
    };

    view! {
        // AI Float Button
        <button 
            on:click=move |_| set_is_open.update(|open| *open = !*open)
            type="button"
            class="messenger-float-btn"
            style="border: none; background: linear-gradient(135deg, #8b5cf6, #06b6d4); box-shadow: 0 4px 16px rgba(139, 92, 246, 0.4); display: flex; align-items: center; justify-content: center;"
            title="Chat with Open-DIY AI Support"
        >
            {move || if is_open.get() {
                view! {
                    <svg viewBox="0 0 24 24" width="28" height="28" fill="currentColor">
                        <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/>
                    </svg>
                }.into_any()
            } else {
                view! {
                    <svg viewBox="0 0 24 24" width="28" height="28" fill="currentColor">
                        <path d="M12 2a2 2 0 012 2v1h4a2 2 0 012 2v10a2 2 0 01-2 2h-4v1a2 2 0 01-4 0v-1H6a2 2 0 01-2-2V7a2 2 0 012-2h4V4a2 2 0 012-2zm6 5H6v10h12V7zm-9 2a1.5 1.5 0 110 3 1.5 1.5 0 010-3zm6 0a1.5 1.5 0 110 3 1.5 1.5 0 010-3zm-6 5h6v1.5H9V14z"/>
                    </svg>
                }.into_any()
            }}
        </button>

        // AI Chat Overlay
        {move || if is_open.get() {
            view! {
                <div class="glass-card ai-chat-window">
                    // Header
                    <div style="padding: 16px; background: linear-gradient(135deg, rgba(139,92,246,0.2), rgba(6,182,212,0.2)); border-bottom: 1px solid var(--border-color); display: flex; align-items: center; gap: 10px; text-align: left;">
                        <span style="width: 10px; height: 10px; background: #22c55e; border-radius: 50%; box-shadow: 0 0 8px #22c55e;"></span>
                        <div>
                            <h4 style="margin: 0; font-size: 0.95rem; font-weight: 600; color: var(--text-contrast);">"Trợ lý AI Open-DIY"</h4>
                            <span style="font-size: 0.72rem; color: var(--text-muted);">"CSKH Hỗ trợ 24/7"</span>
                        </div>
                    </div>

                    // Messages list
                    <div style="flex: 1; padding: 16px; overflow-y: auto; display: flex; flex-direction: column; gap: 12px; text-align: left;" id="ai-chat-messages">
                        {move || messages.get().into_iter().map(|msg| {
                            let (bg, align, text_color, rounded) = if msg.is_ai {
                                ("rgba(255,255,255,0.05)", "flex-start", "#fff", "12px 12px 12px 2px")
                            } else {
                                ("linear-gradient(135deg, #8b5cf6, #7c3aed)", "flex-end", "#fff", "12px 12px 2px 12px")
                            };
                            view! {
                                <div style=format!("align-self: {}; max-width: 80%; background: {}; color: {}; padding: 10px 14px; border-radius: {}; font-size: 0.85rem; line-height: 1.4; border: {}", align, bg, text_color, rounded, if msg.is_ai { "1px solid var(--border-color)" } else { "none" })>
                                    {msg.text}
                                </div>
                            }
                        }).collect_view()}
                    </div>

                    // Suggestion Chips
                    <div style="padding: 8px 16px; display: flex; gap: 6px; flex-wrap: wrap; border-top: 1px solid rgba(255,255,255,0.03); justify-content: flex-start;">
                        <button 
                            on:click=move |_| send_suggestion("Tìm hiểu phím Corne Split")
                            type="button"
                            style="background: rgba(255,255,255,0.03); border: 1px solid var(--border-color); border-radius: 20px; padding: 4px 10px; font-size: 0.72rem; color: var(--secondary); cursor: pointer; transition: all 0.2s;"
                        >
                            "⌨️ Phím Corne"
                        </button>
                        <button 
                            on:click=move |_| send_suggestion("Tìm hiểu phím Dactyl-Manuform")
                            type="button"
                            style="background: rgba(255,255,255,0.03); border: 1px solid var(--border-color); border-radius: 20px; padding: 4px 10px; font-size: 0.72rem; color: var(--secondary); cursor: pointer; transition: all 0.2s;"
                        >
                            "👑 Phím Dactyl"
                        </button>
                        <button 
                            on:click=move |_| send_suggestion("Làm sao để đo kích thước tay?")
                            type="button"
                            style="background: rgba(255,255,255,0.03); border: 1px solid var(--border-color); border-radius: 20px; padding: 4px 10px; font-size: 0.72rem; color: var(--secondary); cursor: pointer; transition: all 0.2s;"
                        >
                            "📏 Đo tay"
                        </button>
                        <button 
                            on:click=move |_| send_suggestion("Chính sách đặt cọc và mua hàng")
                            type="button"
                            style="background: rgba(255,255,255,0.03); border: 1px solid var(--border-color); border-radius: 20px; padding: 4px 10px; font-size: 0.72rem; color: var(--secondary); cursor: pointer; transition: all 0.2s;"
                        >
                            "💳 Đặt cọc"
                        </button>
                    </div>

                    // Footer Input
                    <form 
                        on:submit=move |ev| {
                            ev.prevent_default();
                            let val = input.get();
                            if !val.trim().is_empty() {
                                handle_send(val);
                            }
                        }
                        style="padding: 12px 16px; border-top: 1px solid var(--border-color); display: flex; gap: 8px; background: rgba(0,0,0,0.2);"
                    >
                        <input 
                            type="text"
                            placeholder="Nhập câu hỏi..."
                            prop:value=input
                            on:input=move |ev| set_input.set(event_target_value(&ev))
                            style="flex: 1; background: rgba(255,255,255,0.04); border: 1px solid var(--border-color); border-radius: 8px; padding: 8px 12px; font-size: 0.85rem; color: #fff; outline: none; transition: border 0.2s;"
                        />
                        <button 
                            type="submit"
                            class="btn btn-primary btn-sm"
                            style="padding: 0 14px; border-radius: 8px; background: linear-gradient(135deg, #8b5cf6, #06b6d4); border: none; display: flex; align-items: center; justify-content: center;"
                        >
                            <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                            </svg>
                        </button>
                    </form>

                    // Direct Messenger Link
                    <div style="padding: 8px; text-align: center; background: rgba(0,84,255,0.1); border-top: 1px solid rgba(255,255,255,0.03);">
                        <a 
                            href="https://m.me/1111759575360830" 
                            target="_blank" 
                            rel="noopener noreferrer" 
                            style="font-size: 0.72rem; color: #0084ff; text-decoration: none; font-weight: 500; display: inline-flex; align-items: center; gap: 4px;"
                        >
                            <svg viewBox="0 0 24 24" width="12" height="12" fill="currentColor">
                                <path d="M12 2C6.48 2 2 6.14 2 11.25c0 2.91 1.45 5.51 3.75 7.15.19.14.31.36.31.6l-.02 1.89c-.01.48.51.81.93.57l2.12-1.22c.17-.1.38-.11.56-.05 1.44.43 2.97.66 4.55.66 5.52 0 10-4.14 10-9.25S17.52 2 12 2zm1.18 11.63l-2.02-2.15-3.92 2.15c-.41.22-.89-.24-.65-.65l2.02-3.48 2.02 2.15 3.92-2.15c.41-.22.89.24.65.65l-2.02 3.48z"/>
                            </svg>
                            "Hoặc trò chuyện trực tiếp với chủ shop"
                        </a>
                    </div>
                </div>
            }.into_any()
        } else {
            view! { <div/> }.into_any()
        }}
    }
}

#[component]
fn NotFoundPage() -> impl IntoView {
    let lang = expect_context::<LanguageContext>().lang;
    view! {
        <SeoHead
            title="open-diy | Page Not Found".to_string()
            description="The page you requested could not be found on open-diy.".to_string()
            path="/404".to_string()
            image_path="/images/dactyl.png".to_string()
            robots="noindex,follow".to_string()
        />
        <div class="checkout-page">
            <h1 class="gradient-text">{t!(lang, "404 - Page Not Found", "404 - Không tìm thấy trang")}</h1>
            <p>{t!(lang, "The keyboard you are looking for has been disassembled.", "Bàn phím bạn đang tìm kiếm đã bị rã linh kiện.")}</p>
            <div style="margin-top: 24px;">
                <A href="/" attr:class="btn btn-primary">{t!(lang, "Back to Home", "Quay lại Trang chủ")}</A>
            </div>
        </div>
    }
}

/* ==========================================
Footer Component
========================================== */

#[component]
fn Footer() -> impl IntoView {
    let lang = expect_context::<LanguageContext>().lang;
    view! {
        <footer>
            <div class="footer-grid">
                <div class="footer-col">
                    <h4 class="gradient-text">"open-diy"</h4>
                    <p style="margin-bottom: 15px; max-width: 320px;">
                        {t!(lang, 
                            "Enabling ergonomics and absolute hardware customizability through additive manufacturing and open source designs.", 
                            "Mang lại thiết kế công thái học và khả năng tùy biến phần cứng tuyệt đối thông qua sản xuất in 3D và các thiết kế nguồn mở."
                        )}
                    </p>
                </div>
                <div class="footer-col">
                    <h4>{t!(lang, "Quick Links", "Liên kết nhanh")}</h4>
                    <ul class="footer-links">
                        <li><A href="/">{move || t!(lang, "Home", "Trang chủ")()}</A></li>
                        <li><A href="/shop">{t!(lang, "Catalog Shop", "Danh mục sản phẩm")}</A></li>
                        <li><A href="/about">{t!(lang, "Philosophy", "Triết lý")}</A></li>
                        <li><a href="https://shopee.vn/opendiy" target="_blank" rel="noopener noreferrer">{move || t!(lang, "Shopee Store", "Cửa hàng Shopee")()}</a></li>
                    </ul>
                </div>
                <div class="footer-col">
                    <h4>{t!(lang, "Social Media", "Mạng xã hội")}</h4>
                    <ul class="footer-links">
                        <li>
                            <a href="https://www.facebook.com/1111759575360830" target="_blank" rel="noopener noreferrer" style="display: inline-flex; align-items: center; gap: 8px;">
                                <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="color: var(--secondary);">
                                    <path d="M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z"></path>
                                </svg>
                                "Facebook Page"
                            </a>
                        </li>
                        <li>
                            <a href="https://www.youtube.com/@opendiyvn" target="_blank" rel="noopener noreferrer" style="display: inline-flex; align-items: center; gap: 8px;">
                                <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="color: var(--secondary);">
                                    <path d="M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"></path>
                                    <polygon points="9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02"></polygon>
                                </svg>
                                "YouTube Channel"
                            </a>
                        </li>
                    </ul>
                </div>
                <div class="footer-col">
                    <h4>{t!(lang, "3D Print Hub", "Trung tâm in 3D")}</h4>
                    <p style="margin-bottom: 10px;">{t!(lang, "Prusa XL CoreXY Farms", "Trang trại máy in Prusa XL CoreXY")}</p>
                    <p style="margin-bottom: 10px;">{t!(lang, "Polymaker PETG/ASA filaments", "Sợi nhựa Polymaker PETG/ASA")}</p>
                    <p>{t!(lang, "0.2mm layer resolution", "Độ phân giải lớp in 0.2mm")}</p>
                </div>
            </div>

            <div class="footer-bottom">
                <p>{t!(lang, "© 2026 open-diy. Released under MIT Licence.", "© 2026 open-diy. Phát hành dưới Giấy phép MIT.")}</p>
                <div class="footer-badge">
                    <span></span>
                    {t!(lang, "Print Farm: Online", "Trang trại in: Đang hoạt động")}
                </div>
            </div>
        </footer>
    }
}

#[component]
fn OtelTestPage() -> impl IntoView {
    let lang = expect_context::<LanguageContext>().lang;

    // Actions triggering browser JS / OTel SDK helper methods using wasm_bindgen js_sys under hydrate
    let trigger_custom_span = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = ["window", "otelHelper"], js_name = triggerCustomSpan)]
                fn trigger_custom_span_js(span_name: &str);
            }
            trigger_custom_span_js("manual_otel_test_span");
        }
    };

    let trigger_exception = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = ["window", "otelHelper"], js_name = triggerException)]
                fn trigger_exception_js(span_name: &str, err_msg: &str);
            }
            trigger_exception_js("manual_error_span", "Simulated exception error from test page");
        }
    };

    let trigger_fetch = move |_| {
        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = web_sys::window() {
                let _ = window.fetch_with_str("/api/otel-test");
            }
        }
    };

    let trigger_faro_log = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = ["window", "GrafanaFaroWebSdk", "faro", "api"], js_name = pushLog)]
                fn faro_push_log(args: &js_sys::Array);
            }
            let array = js_sys::Array::new();
            array.push(&JsValue::from_str("Manual test log triggered from open-diy Faro test page"));
            faro_push_log(&array);
        }
    };

    let trigger_faro_error = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = ["window", "GrafanaFaroWebSdk", "faro", "api"], js_name = pushError)]
                fn faro_push_error(err: &JsValue);
            }
            let err = js_sys::Error::new("Simulated exception error from open-diy Faro test page");
            faro_push_error(&err);
        }
    };


    view! {
        <div class="otel-test-container" style="max-width: 800px; margin: 40px auto; padding: 20px; font-family: sans-serif;">
            <h1 class="gradient-text" style="font-size: 2.5rem; margin-bottom: 20px;">
                {t!(lang, "OpenTelemetry Web SDK Test Suite", "Bộ Thử Nghiệm OpenTelemetry Web SDK")}
            </h1>
            
            <p style="margin-bottom: 30px; font-size: 1.1rem; line-height: 1.6; color: var(--text);">
                {t!(
                    lang,
                    "Use this page to manually trigger and test OTel Web SDK features. All events, spans, and traces generated here are automatically captured and pushed to the Tempo gateway at otel.opendiy.vn.",
                    "Sử dụng trang này để kích hoạt và thử nghiệm các tính năng của OTel Web SDK. Tất cả các sự kiện, span và trace được tạo ở đây sẽ tự động được thu thập và gửi đến cổng Tempo tại otel.opendiy.vn."
                )}
            </p>

            <div style="display: grid; grid-template-columns: 1fr; gap: 20px;">
                // 1. Document Load Test Card
                <div style="border: 1px solid var(--border); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); backdrop-filter: blur(10px);">
                    <h3 style="margin-top: 0; display: flex; align-items: center; gap: 8px;">
                        <span style="display: inline-block; width: 10px; height: 10px; border-radius: 50%; background-color: #10B981;"></span>
                        {t!(lang, "Document Load & Performance Metrics", "Tải trang & Chỉ số Hiệu năng")}
                    </h3>
                    <p style="font-size: 0.9rem; color: var(--text-muted); line-height: 1.5;">
                        {t!(
                            lang,
                            "Traces are automatically generated when this page finishes loading. It records browser performance timings (DNS lookup, TCP connect, DOM content loaded, etc.).",
                            "Trace được tạo tự động khi trang này tải xong. Nó ghi lại thời gian hiệu năng của trình duyệt (tra cứu DNS, kết nối TCP, DOM content loaded, v.v.)."
                        )}
                    </p>
                </div>

                // 2. Custom Tracer Span Card
                <div style="border: 1px solid var(--border); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); backdrop-filter: blur(10px);">
                    <h3 style="margin-top: 0; display: flex; align-items: center; gap: 8px;">
                        <span style="display: inline-block; width: 10px; height: 10px; border-radius: 50%; background-color: #3B82F6;"></span>
                        {t!(lang, "Custom Span Generation", "Tạo Custom Span")}
                    </h3>
                    <p style="font-size: 0.9rem; color: var(--text-muted); line-height: 1.5; margin-bottom: 15px;">
                        {t!(
                            lang,
                            "Manually start and end a tracer span inside the browser environment. This tests client-side manual instrumentation using the web SDK.",
                            "Tự động tạo và kết thúc một span trong môi trường trình duyệt. Thử nghiệm việc định cấu hình trace thủ công phía client bằng SDK web."
                        )}
                    </p>
                    <button 
                        on:click=trigger_custom_span
                        class="btn btn-primary"
                        style="padding: 10px 20px; font-weight: 600; cursor: pointer; border-radius: 8px; border: none; background: linear-gradient(135deg, #3B82F6 0%, #1D4ED8 100%); color: #fff;"
                    >
                        {t!(lang, "Trigger Custom Span", "Kích Hoạt Custom Span")}
                    </button>
                </div>

                // 3. Exception Recording Card
                <div style="border: 1px solid var(--border); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); backdrop-filter: blur(10px);">
                    <h3 style="margin-top: 0; display: flex; align-items: center; gap: 8px;">
                        <span style="display: inline-block; width: 10px; height: 10px; border-radius: 50%; background-color: #EF4444;"></span>
                        {t!(lang, "Error & Exception Recording", "Ghi Nhận Lỗi & Ngoại Lệ")}
                    </h3>
                    <p style="font-size: 0.9rem; color: var(--text-muted); line-height: 1.5; margin-bottom: 15px;">
                        {t!(
                            lang,
                            "Generate a span that records an simulated JavaScript Error object. This tests how exceptions are categorized and sent to the collector.",
                            "Tạo một span để ghi nhận một đối tượng Lỗi JavaScript mô phỏng. Thử nghiệm cách phân loại ngoại lệ và gửi tới collector."
                        )}
                    </p>
                    <button 
                        on:click=trigger_exception
                        class="btn btn-danger"
                        style="padding: 10px 20px; font-weight: 600; cursor: pointer; border-radius: 8px; border: none; background: linear-gradient(135deg, #EF4444 0%, #B91C1C 100%); color: #fff;"
                    >
                        {t!(lang, "Trigger Exception Span", "Kích Hoạt Exception Span")}
                    </button>
                </div>

                // 4. Fetch Auto-Instrumentation Card
                <div style="border: 1px solid var(--border); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); backdrop-filter: blur(10px);">
                    <h3 style="margin-top: 0; display: flex; align-items: center; gap: 8px;">
                        <span style="display: inline-block; width: 10px; height: 10px; border-radius: 50%; background-color: #F59E0B;"></span>
                        {t!(lang, "Fetch Request Instrumentation", "Tự động Trace Yêu Cầu Fetch")}
                    </h3>
                    <p style="font-size: 0.9rem; color: var(--text-muted); line-height: 1.5; margin-bottom: 15px;">
                        {t!(
                            lang,
                            "Triggers an HTTP GET request using fetch() to backend API (/api/otel-test). The FetchInstrumentation should intercept this and generate a tracer span for this HTTP request.",
                            "Kích hoạt một yêu cầu HTTP GET bằng fetch() tới API hệ thống (/api/otel-test). FetchInstrumentation sẽ bắt sự kiện này và tạo span cho yêu cầu HTTP."
                        )}
                    </p>
                    <button 
                        on:click=trigger_fetch
                        class="btn btn-secondary"
                        style="padding: 10px 20px; font-weight: 600; cursor: pointer; border-radius: 8px; border: none; background: linear-gradient(135deg, #F59E0B 0%, #D97706 100%); color: #fff;"
                    >
                        {t!(lang, "Trigger fetch() Request", "Kích Hoạt Yêu Cầu fetch()")}
                    </button>
                </div>

                // 5. Grafana Faro RUM Card
                <div style="border: 1px solid var(--border); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); backdrop-filter: blur(10px);">
                    <h3 style="margin-top: 0; display: flex; align-items: center; gap: 8px;">
                        <span style="display: inline-block; width: 10px; height: 10px; border-radius: 50%; background-color: #EC4899;"></span>
                        {t!(lang, "Grafana Faro RUM Instrumentation", "Thử Nghiệm Grafana Faro RUM")}
                    </h3>
                    <p style="font-size: 0.9rem; color: var(--text-muted); line-height: 1.5; margin-bottom: 15px;">
                        {t!(
                            lang,
                            "Trigger custom logs and exceptions to be captured and sent to the Faro receiver endpoint (faro.opendiy.vn).",
                            "Gửi log và lỗi mô phỏng đến máy thu thập Faro (faro.opendiy.vn)."
                        )}
                    </p>
                    <div style="display: flex; gap: 10px;">
                        <button 
                            on:click=trigger_faro_log
                            class="btn btn-primary"
                            style="padding: 10px 20px; font-weight: 600; cursor: pointer; border-radius: 8px; border: none; background: linear-gradient(135deg, #EC4899 0%, #BE185D 100%); color: #fff;"
                        >
                            {t!(lang, "Trigger Faro Log", "Gửi Faro Log")}
                        </button>
                        <button 
                            on:click=trigger_faro_error
                            class="btn btn-danger"
                            style="padding: 10px 20px; font-weight: 600; cursor: pointer; border-radius: 8px; border: none; background: linear-gradient(135deg, #EF4444 0%, #B91C1C 100%); color: #fff;"
                        >
                            {t!(lang, "Trigger Faro Exception", "Gửi Faro Exception")}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

