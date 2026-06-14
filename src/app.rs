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
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
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

    view! {
        <Stylesheet id="leptos" href="/pkg/open-diy.css"/>
        <Script type_="application/ld+json">{organization_json_ld()}</Script>
        <Script type_="application/ld+json">{website_json_ld()}</Script>

        <div class="app-container">
            <Router>
                <Navbar/>
                <main class="main-content">
                    <Routes fallback=NotFoundPage>
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=StaticSegment("shop") view=CatalogPage/>
                        <Route path=(StaticSegment("product"), ParamSegment("id")) view=ProductDetailPage/>
                        <Route path=StaticSegment("about") view=AboutPage/>
                    </Routes>
                </main>
                <Footer/>

                // Floating Messenger Chat Widget
                <a 
                    href="https://m.me/1111759575360830" 
                    target="_blank" 
                    rel="noopener noreferrer" 
                    class="messenger-float-btn"
                    title="Chat with us on Messenger"
                >
                    <svg viewBox="0 0 24 24" width="28" height="28" fill="currentColor">
                        <path d="M12 2C6.48 2 2 6.14 2 11.25c0 2.91 1.45 5.51 3.75 7.15.19.14.31.36.31.6l-.02 1.89c-.01.48.51.81.93.57l2.12-1.22c.17-.1.38-.11.56-.05 1.44.43 2.97.66 4.55.66 5.52 0 10-4.14 10-9.25S17.52 2 12 2zm1.18 11.63l-2.02-2.15-3.92 2.15c-.41.22-.89-.24-.65-.65l2.02-3.48 2.02 2.15 3.92-2.15c.41-.22.89.24.65.65l-2.02 3.48z"/>
                    </svg>
                </a>
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

    let is_en = move || lang.get() == Language::En;
    let is_vi = move || lang.get() == Language::Vi;

    let toggle_en = move |_| lang.set(Language::En);
    let toggle_vi = move |_| lang.set(Language::Vi);

    view! {
        <nav class="navbar">
            <A href="/" attr:class="nav-brand">
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="feather feather-cpu" style="margin-right: 5px; color: #8b5cf6;">
                    <rect x="4" y="4" width="16" height="16" rx="2" ry="2"></rect>
                    <rect x="9" y="9" width="6" height="6"></rect>
                    <line x1="9" y1="1" x2="9" y2="4"></line>
                    <line x1="15" y1="1" x2="15" y2="4"></line>
                    <line x1="9" y1="20" x2="9" y2="23"></line>
                    <line x1="15" y1="20" x2="15" y2="23"></line>
                    <line x1="20" y1="9" x2="23" y2="9"></line>
                    <line x1="20" y1="15" x2="23" y2="15"></line>
                    <line x1="1" y1="9" x2="4" y2="9"></line>
                    <line x1="1" y1="15" x2="4" y2="15"></line>
                </svg>
                "open-diy"
            </A>
            <ul class="nav-menu">
                <li><A href="/" attr:class="nav-link">{t!(lang, "Home", "Trang chủ")}</A></li>
                <li><A href="/shop" attr:class="nav-link">{t!(lang, "Shop", "Cửa hàng")}</A></li>
                <li><A href="/about" attr:class="nav-link">{t!(lang, "About", "Giới thiệu")}</A></li>
            </ul>
            <div class="nav-actions" style="display: flex; align-items: center; gap: 16px;">
                <div style="display: flex; gap: 6px; font-size: 0.85rem; font-weight: 600;">
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
                <a href="https://shopee.vn/opendiy" target="_blank" rel="noopener noreferrer" class="btn btn-primary btn-sm">
                    {t!(lang, "Shopee Store", "Cửa hàng Shopee")}
                </a>
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
                <img src="/images/dactyl.png" alt="Dactyl Manuform Custom Keyboard"/>
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
                                    <img src=selected_img style="width: 100%; height: 100%; object-fit: cover;" alt=name.clone()/>
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
                                        <a href=shopee target="_blank" rel="noopener noreferrer" class="btn btn-primary" style="width: 100%;">
                                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-right: 8px;">
                                                <path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z"></path>
                                                <line x1="3" y1="6" x2="21" y2="6"></line>
                                                <path d="M16 10a4 4 0 0 1-8 0"></path>
                                            </svg>
                                            {t!(lang, "Customize & Buy on Shopee", "Tùy biến & Mua trên Shopee")}
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
                        <li><A href="/">{t!(lang, "Home", "Trang chủ")}</A></li>
                        <li><A href="/shop">{t!(lang, "Catalog Shop", "Danh mục sản phẩm")}</A></li>
                        <li><A href="/about">{t!(lang, "Philosophy", "Triết lý")}</A></li>
                        <li><a href="https://shopee.vn/opendiy" target="_blank" rel="noopener noreferrer">{t!(lang, "Shopee Store", "Cửa hàng Shopee")}</a></li>
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
