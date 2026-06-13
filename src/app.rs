use crate::seo::{
    faq_json_ld, organization_json_ld, product_json_ld, website_json_ld, JsonLd, SeoHead,
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Script, Stylesheet};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment,
};

/* ==========================================
Data Types
========================================== */

#[derive(Clone, Debug, PartialEq)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub image: String,
    pub price: f64,
    pub desc: String,
    pub detailed_desc: String,
    pub shopee_url: String,
    pub github_url: Option<String>,
    pub specs: Vec<String>,
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
                        <Route path=StaticSegment("about") view=AboutPage/>
                    </Routes>
                </main>
                <Footer/>
            </Router>
        </div>
    }
}

/* ==========================================
Navigation Component
========================================== */

#[component]
fn Navbar() -> impl IntoView {
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
                <span>"ssr"</span>
            </A>
            <ul class="nav-menu">
                <li><A href="/" attr:class="nav-link">"Home"</A></li>
                <li><A href="/shop" attr:class="nav-link">"Shop"</A></li>
                <li><A href="/about" attr:class="nav-link">"About"</A></li>
            </ul>
            <div class="nav-actions">
                <a href="https://shopee.vn/opendiy" target="_blank" rel="noopener noreferrer" class="btn btn-primary btn-sm">
                    "Shopee Store"
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
    view! {
        <SeoHead
            title="open-diy | Custom 3D Printed Keyboards".to_string()
            description="Shop ergonomic 3D printed keyboards with open-source designs, custom switches, and premium keycaps.".to_string()
            path="/".to_string()
            image_path="/images/dactyl.png".to_string()
        />
        <JsonLd json=faq_json_ld()/>
        <div class="hero">
            <span class="hero-tag">"Revolutionary Mechanical Keyboards"</span>
            <h1 class="hero-title">
                "Ergonomic "
                <br/>
                <span class="gradient-text">"3D Printed Keyboards"</span>
            </h1>
            <p class="hero-subtitle">
                "Open-source designs manufactured in-house with high-performance plastics. contoured for your comfort, posture, and typing acoustics."
            </p>
            <div class="hero-actions">
                <A href="/shop" attr:class="btn btn-primary">"Browse Products"</A>
                <A href="/about" attr:class="btn btn-secondary">"Our Philosophy"</A>
            </div>
            <div class="hero-visual">
                <img src="/images/dactyl.png" alt="Dactyl Manuform Custom Keyboard"/>
            </div>
        </div>

        <section class="features">
            <div class="section-header">
                <h2 class="section-title">"Why Choose " <span class="gradient-text">"open-diy"</span> "?"</h2>
                <p class="section-subtitle">"We merge advanced additive manufacturing with premium custom keyboard enthusiast specs."</p>
            </div>
            <div class="grid grid-cols-3" style="max-width: 1200px; margin: 0 auto;">
                <div class="glass-card feature-card">
                    <div class="feature-icon">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline>
                        </svg>
                    </div>
                    <h3>"Unrivaled Ergonomics"</h3>
                    <p>"Sculpted split designs like the Dactyl place keys in natural column curvature, preventing wrist strain during long typing sessions."</p>
                </div>
                <div class="glass-card feature-card">
                    <div class="feature-icon">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"></path>
                            <polyline points="3.27 6.96 12 12.01 20.73 6.96"></polyline>
                            <line x1="12" y1="22.08" x2="12" y2="12"></line>
                        </svg>
                    </div>
                    <h3>"Precision 3D Printing"</h3>
                    <p>"We use fine-tuned high-density print settings (0.2mm layer heights, gyroid infill) for superb acoustics and impact-resistant cases."</p>
                </div>
                <div class="glass-card feature-card">
                    <div class="feature-icon">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polygon points="12 2 2 7 12 12 22 7 12 2"></polygon>
                            <polyline points="2 17 12 22 22 17"></polyline>
                            <polyline points="2 12 12 17 22 12"></polyline>
                        </svg>
                    </div>
                    <h3>"Enthusiast Specs Standard"</h3>
                    <p>"Hot-swappable sockets let you replace switches without soldering. Pre-lubed stabs and acoustic foam are included in every build."</p>
                </div>
            </div>
        </section>

        <section class="faq-section" style="max-width: 1200px; margin: 0 auto; padding: 40px 0 20px;">
            <div class="section-header">
                <span class="hero-tag" style="margin-bottom: 12px;">"FAQ"</span>
                <h2 class="section-title">"Keyboard buying questions, answered."</h2>
                <p class="section-subtitle">"Helpful details for shoppers searching for ergonomic, open-source, and 3D printed keyboards."</p>
            </div>
            <div class="grid grid-cols-3" style="gap: 18px;">
                <details class="glass-card feature-card" open>
                    <summary style="cursor: pointer; font-weight: 700;">"What makes open-diy different?"</summary>
                    <p style="margin-top: 12px;">"We combine open-source keyboard design with in-house 3D printing, custom switch choices, and direct links to checkout safely on Shopee."</p>
                </details>
                <details class="glass-card feature-card">
                    <summary style="cursor: pointer; font-weight: 700;">"Can I build an ergonomic keyboard here?"</summary>
                    <p style="margin-top: 12px;">"Yes. The catalog highlights split and curved layouts such as Dactyl and Alice-inspired designs, all tuned for better comfort and desk positioning."</p>
                </details>
                <details class="glass-card feature-card">
                    <summary style="cursor: pointer; font-weight: 700;">"Do you support DIY builders?"</summary>
                    <p style="margin-top: 12px;">"Absolutely. The About page points to the open-source side of the project, and we list public GitHub repositories for builders who want to print and assemble by themselves."</p>
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
    let selected_product = RwSignal::<Option<Product>>::new(None);

    let products = vec![
        Product {
            id: String::from("dactyl"),
            name: String::from("Dactyl-ManuForm Split"),
            tag: String::from("Best Seller"),
            image: String::from("/images/dactyl.png"),
            price: 219.0,
            desc: String::from("The ultimate split ergonomic design. Hand-scaffolded curved keywells, separate left/right modules for optimal desk positioning."),
            detailed_desc: String::from("Dactyl-ManuForm is a split, contoured, ergonomic mechanical keyboard. By placing keys in a 3D curved keywell, it minimizes the distance fingers must travel, reducing strain. Separate left and right modules let you locate them exactly where your hands naturally rest, aligning with your shoulders to prevent slouching."),
            shopee_url: String::from("https://shopee.vn/opendiy"),
            github_url: Some(String::from("https://github.com/tshort/dactyl-keyboard")),
            specs: vec![
                String::from("3D sculpted curved split keywells"),
                String::from("Columnar layout to match finger lengths"),
                String::from("Fully hot-swappable switch sockets"),
                String::from("VIAL/QMK firmware custom keymap mapping"),
                String::from("High-grade PETG/ASA active-heated printed shell"),
            ],
        },
        Product {
            id: String::from("frosted"),
            name: String::from("Frosted 60% Neon"),
            tag: String::from("Minimalist"),
            image: String::from("/images/frosted.png"),
            price: 109.0,
            desc: String::from("Compact 60% layout with a semi-translucent case. Ideal for clean desk setups with vibrant customizable RGB underglow."),
            detailed_desc: String::from("The Frosted 60% Neon features a high-quality semi-translucent matte finish casing that disperses underglow LED lighting beautifully. Designed for keyboard enthusiasts who want a clean, minimalist workspace setup without sacrificing structural rigidity or premium acoustic signatures."),
            shopee_url: String::from("https://shopee.vn/opendiy"),
            github_url: None,
            specs: vec![
                String::from("Sleek unibody 60% compact layout"),
                String::from("Frosted acrylic-style translucent enclosure"),
                String::from("Customizable vibrant underglow RGB LEDs"),
                String::from("Hot-swappable keys with sound dampening foam"),
                String::from("Standard ANSI layout layout compatibility"),
            ],
        },
        Product {
            id: String::from("alice"),
            name: String::from("Alice Curved Forest"),
            tag: String::from("Ergonomic"),
            image: String::from("/images/alice.png"),
            price: 149.0,
            desc: String::from("Unibody ergonomic curved layout. Reduces wrist ulnar deviation without the learning curve of a fully split keyboard."),
            detailed_desc: String::from("The Alice Curved Forest brings ergonomic typing to a unibody form factor. By curving the alphabetic keys outward, it aligns with your wrist angles, reducing strain. This design is perfect for typing enthusiasts who want ergonomic benefits without adjusting to two separate modules."),
            shopee_url: String::from("https://shopee.vn/opendiy"),
            github_url: None,
            specs: vec![
                String::from("Ergonomic curved Alice unibody layout"),
                String::from("Reduces wrist ulnar deviation"),
                String::from("Hot-swappable switch sockets"),
                String::from("Forest green textured acoustic casing"),
                String::from("Simple standard keyboard learning curve"),
            ],
        },
        Product {
            id: String::from("corne"),
            name: String::from("Corne Cherry Split"),
            tag: String::from("Ultra-Compact"),
            image: String::from("/images/corne.png"),
            price: 129.0,
            desc: String::from("Ultra-compact 40% split layout. Highly optimized mapping, customized dual OLED displays, and brilliant customizable RGB underglow."),
            detailed_desc: String::from("Corne (Crkbd) is a legendary ultra-compact 40% split mechanical keyboard. By discarding the number and function rows, it keeps all keys within one unit of your home row. With customizable dual OLED status displays and full addressable RGB underglow, it is the ultimate keyboard for developers and minimalists."),
            shopee_url: String::from("https://shopee.vn/opendiy"),
            github_url: Some(String::from("https://github.com/foostan/crkbd")),
            specs: vec![
                String::from("Ultra-compact split 42-key layout"),
                String::from("Dual custom 0.91-inch OLED screens"),
                String::from("Individual per-key and underglow RGB"),
                String::from("VIAL software support for simple custom layer setup"),
                String::from("Premium slim acrylic plate sandwich casing"),
            ],
        },
        Product {
            id: String::from("wristrests"),
            name: String::from("Dactyl Walnut Wrist Rests"),
            tag: String::from("Accessories"),
            image: String::from("/images/wristrests.png"),
            price: 39.0,
            desc: String::from("Handcrafted premium American Walnut wood. Artfully sculpted and contoured to perfectly match Dactyl Manuform split casings."),
            detailed_desc: String::from("These premium wrist rests are handcrafted from solid American Walnut wood. Specifically sculpted to match the exact vertical and horizontal curves of Dactyl Manuform casings, they provide the necessary elevation and support for your wrists during long coding sessions."),
            shopee_url: String::from("https://shopee.vn/opendiy"),
            github_url: None,
            specs: vec![
                String::from("Solid American Walnut wood construction"),
                String::from("Custom contoured specifically for Dactyl curves"),
                String::from("Satin natural wood oil finish"),
                String::from("Non-slip silicone rubber feet embedded"),
                String::from("Reduces wrist extension and muscle tension"),
            ],
        },
    ];

    view! {
        <SeoHead
            title="open-diy Shop | Curated 3D Printed Keyboard Builds".to_string()
            description="Browse curated ergonomic keyboard builds including Dactyl, Frosted 60%, and Alice Curved layouts with 3D printed cases.".to_string()
            path="/shop".to_string()
            image_path="/images/frosted.png".to_string()
        />
        <JsonLd json=product_json_ld(
            "Dactyl-ManuForm Split",
            "Sculpted split ergonomic keyboard with curved keywells for comfortable all-day typing.",
            "/images/dactyl.png",
            "/shop",
            219.0,
        )/>
        <JsonLd json=product_json_ld(
            "Frosted 60% Neon",
            "Compact 60% keyboard with translucent case styling and vibrant underglow potential.",
            "/images/frosted.png",
            "/shop",
            109.0,
        )/>
        <JsonLd json=product_json_ld(
            "Alice Curved Forest",
            "Unibody ergonomic Alice layout that reduces wrist strain without the split-keyboard learning curve.",
            "/images/alice.png",
            "/shop",
            149.0,
        )/>
        <JsonLd json=product_json_ld(
            "Corne Cherry Split",
            "Ultra-compact 40% split ergonomic keyboard with customizable OLED screens and RGB underglow.",
            "/images/corne.png",
            "/shop",
            129.0,
        )/>
        <JsonLd json=product_json_ld(
            "Dactyl Walnut Wrist Rests",
            "Premium handcrafted walnut wood wrist rests sculpted to fit the curves of the Dactyl-Manuform.",
            "/images/wristrests.png",
            "/shop",
            39.0,
        )/>
        <section class="catalog-section">
            <div class="section-header">
                <span class="hero-tag" style="margin-bottom: 12px;">"Catalog"</span>
                <h1 class="section-title">"Our " <span class="gradient-text">"Curated Builds"</span></h1>
                <p class="section-subtitle">"Browse our professional ergonomic layouts and order securely on Shopee."</p>
            </div>
            <div class="glass-card" style="max-width: 1200px; margin: 0 auto 28px auto; padding: 20px 24px;">
                <p style="margin-bottom: 10px;">"Each build is designed for people searching for an ergonomic keyboard that feels custom without requiring a full DIY project from scratch."</p>
                <p>"Click on any product to view its detailed specifications, and open the Shopee link to customize options and place your order."</p>
            </div>

            <div class="grid grid-cols-3" style="max-width: 1200px; margin: 0 auto;">
                {products.into_iter().map(|p| {
                    let p1 = p.clone();
                    let p2 = p.clone();
                    let p3 = p.clone();
                    let p_img = p.image.clone();
                    let p_tag = p.tag.clone();
                    let p_name = p.name.clone();
                    let p_desc = p.desc.clone();
                    let p_price = p.price;
                    view! {
                        <div class="glass-card product-card">
                            <div class="product-image-wrapper" style="cursor: pointer;" on:click=move |_| selected_product.set(Some(p1.clone()))>
                                <span class="product-tag">{p_tag}</span>
                                <img src=p_img alt=p_name.clone() loading="lazy" decoding="async"/>
                            </div>
                            <div class="product-info">
                                <h3 class="product-title" style="cursor: pointer;" on:click=move |_| selected_product.set(Some(p2.clone()))>{p_name}</h3>
                                <p class="product-desc">{p_desc}</p>
                                <div class="product-meta">
                                    <span class="product-price">"From $" {format!("{:.2}", p_price)}</span>
                                    <button on:click=move |_| selected_product.set(Some(p3.clone())) class="btn btn-primary btn-sm">"View Details"</button>
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>

        <ProductDetailModal product=selected_product />
    }
}

/* ==========================================
Product Detail Slide-over Panel Component
========================================== */

#[component]
fn ProductDetailModal(
    product: RwSignal<Option<Product>>,
) -> impl IntoView {
    let show = move || product.get().is_some();
    let close = move |_| product.set(None);

    view! {
        <div class=move || if show() { "cart-drawer-backdrop open" } else { "cart-drawer-backdrop" }
             on:click=close></div>

        <div class=move || if show() { "cart-drawer open" } else { "cart-drawer" } style="padding: 28px; display: flex; flex-direction: column; overflow-y: auto;">
            {move || product.get().map(|p| {
                let name = p.name.clone();
                let img = p.image.clone();
                let desc = p.detailed_desc.clone();
                let price_str = format!("{:.2}", p.price);
                let shopee = p.shopee_url.clone();
                let github = p.github_url.clone();
                let specs = p.specs.clone();

                view! {
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; border-bottom: 1px solid var(--border-color); padding-bottom: 15px;">
                        <h2 class="gradient-text" style="font-size: 1.5rem; margin: 0; line-height: 1.25;">{name}</h2>
                        <button class="cart-close-btn" on:click=close style="background: none; border: none; color: var(--text-muted); cursor: pointer; padding: 5px;">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <line x1="18" y1="6" x2="6" y2="18"></line>
                                <line x1="6" y1="6" x2="18" y2="18"></line>
                            </svg>
                        </button>
                    </div>

                    <div style="text-align: center; background: #111827; border-radius: 12px; overflow: hidden; border: 1px solid var(--border-color); margin-bottom: 20px;">
                        <img src=img style="width: 100%; aspect-ratio: 16/10; object-fit: cover;" alt=p.name.clone()/>
                    </div>

                    <div style="font-family: var(--font-display); font-size: 1.6rem; font-weight: 700; color: #fff; margin-bottom: 15px;">"$" {price_str}</div>

                    <p style="color: var(--text-muted); line-height: 1.6; margin-bottom: 24px; font-size: 0.95rem;">{desc}</p>

                    <h4 style="font-size: 0.9rem; color: #fff; margin-bottom: 12px; text-transform: uppercase; letter-spacing: 0.05em; font-weight: 700;">"Key Specifications"</h4>
                    <ul style="list-style: none; margin-bottom: 30px; padding: 0;">
                        {specs.into_iter().map(|spec| view! {
                            <li style="display: flex; align-items: center; gap: 8px; color: var(--text-main); font-size: 0.9rem; margin-bottom: 10px; line-height: 1.4;">
                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="color: var(--secondary); flex-shrink: 0;">
                                    <polyline points="20 6 9 17 4 12"></polyline>
                                </svg>
                                {spec}
                            </li>
                        }).collect_view()}
                    </ul>

                    <div style="display: flex; flex-direction: column; gap: 12px; margin-top: auto; padding-top: 20px; border-top: 1px solid var(--border-color);">
                        <a href=shopee target="_blank" rel="noopener noreferrer" class="btn btn-primary" style="width: 100%;">
                            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-right: 8px;">
                                <path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z"></path>
                                <line x1="3" y1="6" x2="21" y2="6"></line>
                                <path d="M16 10a4 4 0 0 1-8 0"></path>
                            </svg>
                            "Buy on Shopee"
                        </a>

                        {move || github.clone().map(|url| view! {
                            <a href=url target="_blank" rel="noopener noreferrer" class="btn btn-secondary" style="width: 100%;">
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="margin-right: 8px;">
                                    <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                                </svg>
                                "DIY Source (GitHub)"
                            </a>
                        })}
                    </div>
                }
            })}
        </div>
    }
}

/* ==========================================
About Page Component
========================================== */

#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <SeoHead
            title="open-diy About | Open-Source Keyboard Manufacturing".to_string()
            description="Learn how open-diy combines open-source keyboard designs with precise 3D printing, ergonomic layouts, and DIY-friendly hardware support.".to_string()
            path="/about".to_string()
            image_path="/images/alice.png".to_string()
        />
        <div class="about-section">
            <div class="about-header">
                <span class="hero-tag">"Philosophy"</span>
                <h1 class="gradient-text">"Open-Source Keyboards"</h1>
                <p class="section-subtitle">"We believe hardware should be owned and customized, not locked behind IP walls."</p>
            </div>

            <img src="/images/alice.png" alt="Close-up of a 3D printed Alice keyboard case" class="about-image" loading="lazy" decoding="async"/>

            <div class="about-body">
                <p>
                    <strong>"open-diy"</strong> " was founded on the idea that standard, mass-produced rectangular keyboards don't fit human anatomy or satisfy our tactile expectations. The custom keyboard hobby has exploded, but premium cases remain expensive and difficult to source. We unlock this by utilizing high-precision industrial 3D printers."
                </p>

                <p>
                    "If you are looking for a 3D printed keyboard shop that still feels approachable, the goal here is to make the learning curve shorter without removing the enthusiast details people care about."
                </p>

                <div class="about-highlight-box">
                    <h3 style="margin-bottom: 12px; font-size: 1.2rem; color: #fff;">"Our Printing Philosophy"</h3>
                    <p>
                        "We run fine-tuned, speed-reduced 3D printer settings to produce sturdy shell walls. By selecting PETG and dense ASA filaments instead of cheap PLA, our cases are structurally stable and do not wrap or degrade. The internal layout utilizes a grid of hollow spaces that acts as a natural dampening chamber, converting harsh typewriter clacks into deep, satisfying acoustics."
                    </p>
                </div>

                <h2 style="font-size: 1.75rem; margin-top: 15px;">"Additive Manufacturing Specs"</h2>
                <ul class="specs-list">
                    <li>
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                        <span>"0.2mm high-precision layers"</span>
                    </li>
                    <li>
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                        <span>"5 outer perimeters for structural mass"</span>
                    </li>
                    <li>
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                        <span>"25% acoustic Gyroid infill pattern"</span>
                    </li>
                    <li>
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                        <span>"Industrial active-heated enclosures"</span>
                    </li>
                </ul>

                <p>
                    "Because our keyboards are open-source projects, we actively support the DIY community. If you own a 3D printer, you can download all casing files (STL/STEP) directly from public Github repositories. If you want us to handle the printing, soldering, lubing, and assembly - that is where our store comes in!"
                </p>

                <div class="glass-card" style="margin-top: 24px; padding: 20px;">
                    <h3 style="margin-bottom: 12px;">"Good fit for shoppers who want"</h3>
                    <ul class="specs-list">
                        <li><span>"An ergonomic keyboard that is fully customizable"</span></li>
                        <li><span>"A 3D printed keyboard case with premium finish options"</span></li>
                        <li><span>"A path from DIY open-source parts to a finished order"</span></li>
                    </ul>
                </div>

                <div style="text-align: center; margin-top: 30px;">
                    <a href="https://github.com/leptos-rs/start-axum" target="_blank" rel="noopener noreferrer" class="btn btn-secondary" style="display: inline-flex; align-items: center; gap: 8px;">
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                        </svg>
                        "Source Code on GitHub"
                    </a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <SeoHead
            title="open-diy | Page Not Found".to_string()
            description="The page you requested could not be found on open-diy.".to_string()
            path="/404".to_string()
            image_path="/images/dactyl.png".to_string()
            robots="noindex,follow".to_string()
        />
        <div class="checkout-page">
            <h1 class="gradient-text">"404 - Page Not Found"</h1>
            <p>"The keyboard you are looking for has been disassembled."</p>
            <div style="margin-top: 24px;">
                <A href="/" attr:class="btn btn-primary">"Back to Home"</A>
            </div>
        </div>
    }
}

/* ==========================================
Footer Component
========================================== */

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="footer-grid">
                <div class="footer-col">
                    <h4 class="gradient-text">"open-diy"</h4>
                    <p style="margin-bottom: 15px; max-width: 320px;">
                        "Enabling ergonomics and absolute hardware customizability through additive manufacturing and open source designs."
                    </p>
                </div>
                <div class="footer-col">
                    <h4>"Quick Links"</h4>
                    <ul class="footer-links">
                        <li><A href="/">"Home"</A></li>
                        <li><A href="/shop">"Catalog Shop"</A></li>
                        <li><A href="/about">"Philosophy"</A></li>
                        <li><a href="https://shopee.vn/opendiy" target="_blank" rel="noopener noreferrer">"Shopee Store"</a></li>
                    </ul>
                </div>
                <div class="footer-col">
                    <h4>"3D Print Hub"</h4>
                    <p style="margin-bottom: 10px;">"Prusa XL CoreXY Farms"</p>
                    <p style="margin-bottom: 10px;">"Polymaker PETG/ASA filaments"</p>
                    <p>"0.2mm layer resolution"</p>
                </div>
            </div>

            <div class="footer-bottom">
                <p>"© 2026 open-diy. Released under MIT Licence."</p>
                <div class="footer-badge">
                    <span></span>
                    "Print Farm: Online"
                </div>
            </div>
        </footer>
    }
}
