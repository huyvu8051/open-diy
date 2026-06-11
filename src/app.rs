use crate::seo::{
    faq_json_ld, organization_json_ld, product_json_ld, website_json_ld, JsonLd, SeoHead,
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Script, Stylesheet};
use leptos_router::{
    components::{Route, Router, Routes, A},
    hooks::use_query_map,
    StaticSegment,
};

/* ==========================================
Data Types
========================================== */

#[derive(Clone, Debug, PartialEq)]
pub struct CartItem {
    pub id: String,
    pub name: String,
    pub image: String,
    pub price: f64,
    pub layout: String,
    pub color: String,
    pub switches: String,
    pub keycaps: String,
    pub quantity: u32,
}

#[derive(Clone, Copy)]
pub struct CartContext {
    pub items: RwSignal<Vec<CartItem>>,
    pub is_open: RwSignal<bool>,
}

impl CartContext {
    pub fn add_item(&self, item: CartItem) {
        self.items.update(|current| {
            if let Some(existing) = current.iter_mut().find(|i| {
                i.id == item.id
                    && i.layout == item.layout
                    && i.color == item.color
                    && i.switches == item.switches
                    && i.keycaps == item.keycaps
            }) {
                existing.quantity += item.quantity;
            } else {
                current.push(item);
            }
        });
        self.is_open.set(true);
    }

    pub fn remove_item(&self, index: usize) {
        self.items.update(|current| {
            if index < current.len() {
                current.remove(index);
            }
        });
    }

    pub fn clear(&self) {
        self.items.set(Vec::new());
    }

    pub fn total_quantity(&self) -> u32 {
        self.items.get().iter().map(|item| item.quantity).sum()
    }

    pub fn subtotal(&self) -> f64 {
        self.items
            .get()
            .iter()
            .map(|item| item.price * item.quantity as f64)
            .sum()
    }
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

    // Initialize Shopping Cart State
    let items = RwSignal::new(Vec::<CartItem>::new());
    let is_open = RwSignal::new(false);
    provide_context(CartContext { items, is_open });

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
                        <Route path=StaticSegment("builder") view=BuilderPage/>
                        <Route path=StaticSegment("about") view=AboutPage/>
                        <Route path=StaticSegment("checkout-success") view=CheckoutSuccessPage/>
                    </Routes>
                </main>
                <CartDrawer/>
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
    let cart = use_context::<CartContext>().expect("CartContext not found");
    let total_qty = move || cart.total_quantity();

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
                <li><A href="/builder" attr:class="nav-link">"Builder"</A></li>
                <li><A href="/about" attr:class="nav-link">"About"</A></li>
            </ul>
            <div class="nav-actions">
                <button class="cart-trigger" on:click=move |_| cart.is_open.set(true)>
                    <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-shopping-bag">
                        <path d="M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z"></path>
                        <line x1="3" y1="6" x2="21" y2="6"></line>
                        <path d="M16 10a4 4 0 0 1-8 0"></path>
                    </svg>
                    <span>"Cart"</span>
                    <span class="cart-count">{total_qty}</span>
                </button>
            </div>
        </nav>
    }
}

/* ==========================================
Shopping Cart Slide-out Drawer Component
========================================== */

#[component]
fn CartDrawer() -> impl IntoView {
    let cart = use_context::<CartContext>().expect("CartContext not found");
    let is_open = move || cart.is_open.get();
    let cart_items = move || cart.items.get();
    let subtotal = move || cart.subtotal();

    view! {
        <div class=move || if is_open() { "cart-drawer-backdrop open" } else { "cart-drawer-backdrop" }
             on:click=move |_| cart.is_open.set(false)></div>

        <div class=move || if is_open() { "cart-drawer open" } else { "cart-drawer" }>
            <div class="cart-header">
                <div class="cart-title">
                    <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-shopping-cart" style="color: #06b6d4;">
                        <circle cx="9" cy="21" r="1"></circle>
                        <circle cx="20" cy="21" r="1"></circle>
                        <path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"></path>
                    </svg>
                    "Your Cart"
                </div>
                <button class="cart-close-btn" on:click=move |_| cart.is_open.set(false)>
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <line x1="18" y1="6" x2="6" y2="18"></line>
                        <line x1="6" y1="6" x2="18" y2="18"></line>
                    </svg>
                </button>
            </div>

            <div class="cart-items-list">
                {move || if cart_items().is_empty() {
                    view! {
                        <div class="cart-empty-message">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                                <circle cx="12" cy="12" r="10"></circle>
                                <path d="M8 14s1.5 2 4 2 4-2 4-2"></path>
                                <line x1="9" y1="9" x2="9.01" y2="9"></line>
                                <line x1="15" y1="9" x2="15.01" y2="9"></line>
                            </svg>
                            <p>"Your cart is currently empty."</p>
                            <A href="/shop" attr:class="btn btn-secondary btn-sm" on:click=move |_| cart.is_open.set(false)>"Browse Keyboards"</A>
                        </div>
                    }.into_any()
                } else {
                    cart_items().into_iter().enumerate().map(|(idx, item)| {
                        let name = item.name.clone();
                        let img = item.image.clone();
                        let layout = item.layout.clone();
                        let color = item.color.clone();
                        let switches = item.switches.clone();
                        let keycaps = item.keycaps.clone();
                        let price_str = format!("{:.2}", item.price);
                        let qty = item.quantity;
                        view! {
                            <div class="cart-item">
                                <img src=img class="cart-item-img" alt=name.clone()/>
                                <div class="cart-item-details">
                                    <div class="cart-item-name">{name}</div>
                                    <div class="cart-item-specs">
                                        "Layout: " {layout} " | Color: " {color}
                                        <br/>
                                        "Switches: " {switches} " | Caps: " {keycaps}
                                    </div>
                                    <div class="cart-item-bottom">
                                        <div class="cart-item-price">"$" {price_str} " (x" {qty} ")"</div>
                                        <button class="cart-item-remove" on:click=move |_| cart.remove_item(idx)>"Remove"</button>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect_view().into_any()
                }}
            </div>

            {move || if !cart_items().is_empty() {
                view! {
                    <div class="cart-footer">
                        <div class="cart-total-row">
                            <span>"Subtotal"</span>
                            <span class="cart-total-price">"$" {format!("{:.2}", subtotal())}</span>
                        </div>
                        <A href="/checkout-success" attr:class="btn btn-primary cart-checkout-btn" on:click=move |_| {
                            cart.clear();
                            cart.is_open.set(false);
                        }>
                            "Checkout Order"
                        </A>
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </div>
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
            description="Shop and configure ergonomic 3D printed keyboards with open-source designs, custom switches, and premium keycaps.".to_string()
            path="/".to_string()
            image_path="/images/dactyl.png".to_string()
        />
        <JsonLd json=faq_json_ld()/>
        <div class="hero">
            <span class="hero-tag">"Revolutionary Mechanical Keyboards"</span>
            <h1 class="hero-title">
                "Fully Custom "
                <br/>
                <span class="gradient-text">"3D Printed Keyboards"</span>
            </h1>
            <p class="hero-subtitle">
                "Open-source designs manufactured in-house with high-performance plastics. Fully configured to your taste, sound profile, and ergonomics."
            </p>
            <div class="hero-actions">
                <A href="/builder" attr:class="btn btn-primary">"Configure Yours"</A>
                <A href="/shop" attr:class="btn btn-secondary">"Browse Catalog"</A>
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
                    <p style="margin-top: 12px;">"We combine open-source keyboard design with in-house 3D printing, custom switch choices, and a guided builder so buyers can get a keyboard that fits their typing style."</p>
                </details>
                <details class="glass-card feature-card">
                    <summary style="cursor: pointer; font-weight: 700;">"Can I build an ergonomic keyboard here?"</summary>
                    <p style="margin-top: 12px;">"Yes. The catalog highlights split and curved layouts such as Dactyl and Alice-inspired designs, all tuned for better comfort and desk positioning."</p>
                </details>
                <details class="glass-card feature-card">
                    <summary style="cursor: pointer; font-weight: 700;">"Do you support DIY builders?"</summary>
                    <p style="margin-top: 12px;">"Absolutely. The About page points to the open-source side of the project, while the builder lets shoppers configure a fully assembled board if they want a finished purchase."</p>
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
        <section class="catalog-section">
            <div class="section-header">
                <span class="hero-tag" style="margin-bottom: 12px;">"Catalog"</span>
                <h1 class="section-title">"Our " <span class="gradient-text">"Curated Builds"</span></h1>
                <p class="section-subtitle">"Choose a layout template and edit it directly in our visual configurator."</p>
            </div>
            <div class="glass-card" style="max-width: 1200px; margin: 0 auto 28px auto; padding: 20px 24px;">
                <p style="margin-bottom: 10px;">"Each build is designed for people searching for an ergonomic keyboard that feels custom without requiring a full DIY project from scratch."</p>
                <p>"Use the catalog to compare layouts, pricing, and aesthetics before opening the builder to fine-tune switches, keycaps, and case color."</p>
            </div>

            <div class="grid grid-cols-3" style="max-width: 1200px; margin: 0 auto;">
                // Product 1
                <div class="glass-card product-card">
                    <div class="product-image-wrapper">
                        <span class="product-tag">"Best Seller"</span>
                        <img src="/images/dactyl.png" alt="Dactyl Manuform ergonomic keyboard" loading="lazy" decoding="async"/>
                    </div>
                    <div class="product-info">
                        <h3 class="product-title">"Dactyl-ManuForm Split"</h3>
                        <p class="product-desc">"The ultimate split ergonomic design. Hand-scaffolded curved keywells, separate left/right modules for optimal desk positioning."</p>
                        <div class="product-meta">
                            <span class="product-price">"From $219.00"</span>
                            <A href="/builder?preset=dactyl" attr:class="btn btn-primary btn-sm">"Customize"</A>
                        </div>
                    </div>
                </div>

                // Product 2
                <div class="glass-card product-card">
                    <div class="product-image-wrapper">
                        <span class="product-tag">"Minimalist"</span>
                        <img src="/images/frosted.png" alt="Frosted 60 percent compact keyboard" loading="lazy" decoding="async"/>
                    </div>
                    <div class="product-info">
                        <h3 class="product-title">"Frosted 60% Neon"</h3>
                        <p class="product-desc">"Compact 60% layout with a semi-translucent case. Ideal for clean desk setups with vibrant customizable RGB underglow."</p>
                        <div class="product-meta">
                            <span class="product-price">"From $109.00"</span>
                            <A href="/builder?preset=frosted" attr:class="btn btn-primary btn-sm">"Customize"</A>
                        </div>
                    </div>
                </div>

                // Product 3
                <div class="glass-card product-card">
                    <div class="product-image-wrapper">
                        <span class="product-tag">"Ergonomic"</span>
                        <img src="/images/alice.png" alt="Alice curved ergonomic keyboard" loading="lazy" decoding="async"/>
                    </div>
                    <div class="product-info">
                        <h3 class="product-title">"Alice Curved Forest"</h3>
                        <p class="product-desc">"Unibody ergonomic curved layout. Reduces wrist ulnar deviation without the learning curve of a fully split keyboard."</p>
                        <div class="product-meta">
                            <span class="product-price">"From $149.00"</span>
                            <A href="/builder?preset=alice" attr:class="btn btn-primary btn-sm">"Customize"</A>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

/* ==========================================
Keyboard Customizer Page Component
========================================== */

#[component]
fn BuilderPage() -> impl IntoView {
    let cart = use_context::<CartContext>().expect("CartContext not found");

    // Read the query parameters reactive map
    let query_map = use_query_map();

    // Configurable signals
    let selected_layout = RwSignal::new(String::from("dactyl"));
    let selected_color = RwSignal::new(String::from("Matte Black"));
    let selected_switch = RwSignal::new(String::from("Linear Red"));
    let selected_keycaps = RwSignal::new(String::from("Retro Cream"));
    let quantity = RwSignal::new(1u32);

    // React to query parameter preset changes
    Effect::new(move |_| {
        let preset_val = query_map.read().get("preset").unwrap_or_default();
        if preset_val == "frosted" {
            selected_layout.set(String::from("frosted"));
            selected_color.set(String::from("Frosted Glass"));
            selected_keycaps.set(String::from("Pastel Gradient"));
        } else if preset_val == "alice" {
            selected_layout.set(String::from("alice"));
            selected_color.set(String::from("Forest Green"));
            selected_keycaps.set(String::from("Retro Cream"));
        } else if preset_val == "dactyl" {
            selected_layout.set(String::from("dactyl"));
            selected_color.set(String::from("Matte Black"));
            selected_keycaps.set(String::from("Retro Cream"));
        }
    });

    // Option base prices and addon costs
    let get_prices = move || {
        let base = match selected_layout.get().as_str() {
            "frosted" => 109.0,
            "alice" => 149.0,
            _ => 219.0, // dactyl
        };

        let color_addon = match selected_color.get().as_str() {
            "Frosted Glass" => 15.0,
            "Forest Green" => 10.0,
            "Marble White" => 10.0,
            _ => 0.0,
        };

        let switch_addon = match selected_switch.get().as_str() {
            "Tactile Brown" => 5.0,
            "Custom Silent" => 20.0,
            _ => 0.0,
        };

        let keycap_addon = match selected_keycaps.get().as_str() {
            "Retro Cream" => 20.0,
            "Pastel Gradient" => 25.0,
            "Classic Matte Black" => 10.0,
            _ => 0.0, // None
        };

        (
            base,
            color_addon,
            switch_addon,
            keycap_addon,
            base + color_addon + switch_addon + keycap_addon,
        )
    };

    // Calculate total price
    let total_price = move || {
        let (_, _, _, _, total) = get_prices();
        total * (quantity.get() as f64)
    };

    // Keyboard layouts list
    let layouts = vec![
        (
            "dactyl",
            "Dactyl-ManuForm Split",
            "Sculpted curved split layout, optimum comfort.",
            219.0,
        ),
        (
            "frosted",
            "Frosted 60% Compact",
            "Ultra compact unibody with gorgeous underglow.",
            109.0,
        ),
        (
            "alice",
            "Alice Curved Ergonomic",
            "Curved layout without separate modules.",
            149.0,
        ),
    ];

    // Colors list
    let colors = vec![
        ("Matte Black", "#111827", "rgba(255,255,255,0.2)"),
        (
            "Frosted Glass",
            "rgba(243,244,246,0.6)",
            "rgba(6,182,212,0.6)",
        ),
        ("Forest Green", "#1e3f20", "rgba(16,185,129,0.5)"),
        ("Marble White", "#f9fafb", "rgba(139,92,246,0.4)"),
    ];

    // Switches list
    let switches = vec![
        ("Linear Red", "Smooth & Quiet", 0.0),
        ("Tactile Brown", "Balanced tactile bump", 5.0),
        ("Clicky Blue", "Satisfying loud click", 0.0),
        ("Custom Silent", "Lubed & ultra-quiet linear", 20.0),
    ];

    // Keycaps list
    let keycaps = vec![
        ("Retro Cream", "PBT Vintage Cream & Slate", 20.0),
        ("Pastel Gradient", "Pastel rainbow gradient", 25.0),
        (
            "Classic Matte Black",
            "Minimalist matte black keycaps",
            10.0,
        ),
        ("Bring Your Own", "Shipped without keycaps", 0.0),
    ];

    // Add item handler
    let add_to_cart_action = move |_| {
        let name = match selected_layout.get().as_str() {
            "frosted" => "open-diy Frosted 60%",
            "alice" => "open-diy Alice Forest",
            _ => "open-diy Dactyl-ManuForm Split",
        };

        let image = match selected_layout.get().as_str() {
            "frosted" => "/images/frosted.png",
            "alice" => "/images/alice.png",
            _ => "/images/dactyl.png",
        };

        let (_, _, _, _, unit_price) = get_prices();

        let item = CartItem {
            id: selected_layout.get(),
            name: String::from(name),
            image: String::from(image),
            price: unit_price,
            layout: match selected_layout.get().as_str() {
                "frosted" => String::from("Frosted 60%"),
                "alice" => String::from("Alice Curved"),
                _ => String::from("Dactyl-ManuForm"),
            },
            color: selected_color.get(),
            switches: selected_switch.get(),
            keycaps: selected_keycaps.get(),
            quantity: quantity.get(),
        };

        cart.add_item(item);
    };

    view! {
        <SeoHead
            title="open-diy Builder | Configure an Ergonomic Keyboard".to_string()
            description="Customize your open-diy keyboard build by choosing layout, case color, switches, keycaps, and quantity in one place.".to_string()
            path="/builder".to_string()
            image_path="/images/dactyl.png".to_string()
        />
        <section class="builder-section">
            <div class="section-header">
                <span class="hero-tag" style="margin-bottom: 12px;">"Interactive Customizer"</span>
                <h1 class="section-title">"Assemble Your " <span class="gradient-text">"Dream Keyboard"</span></h1>
                <p class="section-subtitle">"Realtime custom selections with dynamic pricing. Built by hand, printed for you."</p>
                <p style="max-width: 840px; margin: 0 auto; color: var(--text-muted);">
                    "This configurator is for shoppers who want a custom keyboard with the ergonomics of a specialist build and the convenience of a guided checkout flow."
                </p>
            </div>

            <div class="builder-container" style="max-width: 1200px; margin: 0 auto;">
                <div class="builder-preview-pane">
                    <div class="preview-image-container">
                        {move || match selected_layout.get().as_str() {
                            "frosted" => view! { <img src="/images/frosted.png" alt="Frosted 60% keyboard preview" loading="lazy" decoding="async"/> }.into_view(),
                            "alice" => view! { <img src="/images/alice.png" alt="Alice curved keyboard preview" loading="lazy" decoding="async"/> }.into_view(),
                            _ => view! { <img src="/images/dactyl.png" alt="Dactyl ManuForm keyboard preview" loading="lazy" decoding="async"/> }.into_view(),
                        }}

                        <div class="preview-overlay-specs">
                            <span class="spec-pill">"Layout: " <strong>{selected_layout}</strong></span>
                            <span class="spec-pill">"Color: " <strong>{selected_color}</strong></span>
                            <span class="spec-pill">"Switches: " <strong>{selected_switch}</strong></span>
                            <span class="spec-pill">"Keycaps: " <strong>{selected_keycaps}</strong></span>
                        </div>
                    </div>

                    <div class="glass-card builder-summary-card">
                        <h3 style="font-size: 1.25rem; margin-bottom: 15px;">"Order Summary"</h3>
                        <div class="summary-row">
                            <span>"Base Keyboard Layout"</span>
                            <span>"$" {move || format!("{:.2}", get_prices().0)}</span>
                        </div>
                        <div class="summary-row">
                            <span>"Case Filament Color"</span>
                            <span>"+" {move || format!("${:.2}", get_prices().1)}</span>
                        </div>
                        <div class="summary-row">
                            <span>"Switch Option"</span>
                            <span>"+" {move || format!("${:.2}", get_prices().2)}</span>
                        </div>
                        <div class="summary-row">
                            <span>"Keycaps Style"</span>
                            <span>"+" {move || format!("${:.2}", get_prices().3)}</span>
                        </div>
                        <div class="summary-row total">
                            <span>"Total Cost"</span>
                            <span>"$" {move || format!("{:.2}", total_price())}</span>
                        </div>

                        <div style="display: flex; gap: 15px; align-items: center; margin-top: 20px;">
                            <div style="display: flex; align-items: center; background: rgba(255,255,255,0.05); border: 1px solid var(--border-color); border-radius: 8px; padding: 4px;">
                                <button class="btn btn-secondary btn-sm" style="padding: 6px 12px; border: none; background: none;" on:click=move |_| {
                                    if quantity.get() > 1 {
                                        quantity.set(quantity.get() - 1);
                                    }
                                }>"-"</button>
                                <span style="padding: 0 15px; font-weight: 600; font-family: var(--font-display);">{quantity}</span>
                                <button class="btn btn-secondary btn-sm" style="padding: 6px 12px; border: none; background: none;" on:click=move |_| {
                                    quantity.set(quantity.get() + 1);
                                } >"+"</button>
                            </div>
                            <button class="btn btn-primary builder-add-btn" on:click=add_to_cart_action>
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-plus-square" style="margin-right: 8px;">
                                    <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
                                    <line x1="12" y1="8" x2="12" y2="16"></line>
                                    <line x1="8" y1="12" x2="16" y2="12"></line>
                                </svg>
                                "Add Custom Build"
                            </button>
                        </div>
                    </div>
                </div>

                <div class="builder-controls-pane">
                    <div class="glass-card">
                        <div class="config-group-title">"1. Select Layout"</div>
                        <div class="config-options-grid" style="grid-template-columns: 1fr; gap: 15px;">
                            {layouts.into_iter().map(|(id, name, desc, base_price)| {
                                let is_selected = move || selected_layout.get() == id;
                                view! {
                                    <div class=move || if is_selected() { "config-option-btn selected" } else { "config-option-btn" }
                                         on:click=move |_| selected_layout.set(String::from(id))>
                                        <div style="display: flex; justify-content: space-between; align-items: center;">
                                            <div class="config-option-name">{name}</div>
                                            <div class="config-option-price">"$" {format!("{:.2}", base_price)}</div>
                                        </div>
                                        <div class="config-option-desc" style="margin-top: 5px;">{desc}</div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>

                    <div class="glass-card">
                        <div class="config-group-title">"2. Case Filament Color"</div>
                        <div class="color-options-row">
                            {colors.into_iter().map(|(name, hex, glow)| {
                                let is_selected = move || selected_color.get() == name;
                                view! {
                                    <div class=move || if is_selected() { "color-option-pill selected" } else { "color-option-pill" }
                                         style=format!("background: {}; --glow-color: {};", hex, glow)
                                         on:click=move |_| selected_color.set(String::from(name))
                                         title=name>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                        <p style="font-size: 0.85rem; color: var(--text-muted);">
                            "Selected: " <strong style="color: #fff;">{selected_color}</strong>
                        </p>
                    </div>

                    <div class="glass-card">
                        <div class="config-group-title">"3. Switch Option"</div>
                        <div class="config-options-grid">
                            {switches.into_iter().map(|(name, desc, addon)| {
                                let is_selected = move || selected_switch.get() == name;
                                view! {
                                    <div class=move || if is_selected() { "config-option-btn selected" } else { "config-option-btn" }
                                         on:click=move |_| selected_switch.set(String::from(name))>
                                        <div class="config-option-name">{name}</div>
                                        <div class="config-option-desc">{desc}</div>
                                        <div class="config-option-price">
                                            {if addon == 0.0 { String::from("Free") } else { format!("+${:.2}", addon) }}
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>

                    <div class="glass-card">
                        <div class="config-group-title">"4. Keycaps Profile"</div>
                        <div class="config-options-grid">
                            {keycaps.into_iter().map(|(name, desc, addon)| {
                                let is_selected = move || selected_keycaps.get() == name;
                                view! {
                                    <div class=move || if is_selected() { "config-option-btn selected" } else { "config-option-btn" }
                                         on:click=move |_| selected_keycaps.set(String::from(name))>
                                        <div class="config-option-name">{name}</div>
                                        <div class="config-option-desc">{desc}</div>
                                        <div class="config-option-price">
                                            {if addon == 0.0 { String::from("Free") } else { format!("+${:.2}", addon) }}
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
            </div>
        </section>
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

/* ==========================================
Checkout Success Page Component
========================================== */

#[component]
fn CheckoutSuccessPage() -> impl IntoView {
    // Generate a random order number
    let order_num = "OD-849204";

    view! {
        <SeoHead
            title="open-diy Checkout Success".to_string()
            description="Your open-diy order was received successfully.".to_string()
            path="/checkout-success".to_string()
            image_path="/images/dactyl.png".to_string()
            robots="noindex,follow".to_string()
        />
        <div class="checkout-page">
            <div class="success-badge">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                    <polyline points="22 4 12 14.01 9 11.01"></polyline>
                </svg>
            </div>

            <h1 class="gradient-text" style="font-size: 2.5rem; margin-bottom: 15px;">"Order Received!"</h1>
            <p style="font-size: 1.1rem; color: var(--text-muted); margin-bottom: 25px;">
                "Thank you for backing open-diy hardware. We have queued your keyboard for printing."
            </p>

            <div class="glass-card" style="max-width: 450px; margin: 0 auto 40px auto; padding: 24px;">
                <div style="display: flex; justify-content: space-between; margin-bottom: 12px;">
                    <span>"Order Number"</span>
                    <strong style="color: #fff;">{order_num}</strong>
                </div>
                <div style="display: flex; justify-content: space-between; margin-bottom: 12px;">
                    <span>"Estimated Print Time"</span>
                    <strong style="color: #06b6d4;">"24 - 48 Hours"</strong>
                </div>
                <div style="display: flex; justify-content: space-between;">
                    <span>"Status"</span>
                    <strong style="color: var(--success);">"In Filament Queue"</strong>
                </div>
            </div>

            <A href="/" attr:class="btn btn-primary">"Back to Home Page"</A>
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
                        <li><A href="/builder">"Configurator"</A></li>
                        <li><A href="/about">"Philosophy"</A></li>
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
