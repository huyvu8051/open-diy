use leptos::prelude::*;
use leptos_meta::{Link, Meta, Script, Title};
use std::env;

pub const SITE_NAME: &str = "open-diy";
const DEFAULT_SITE_URL: &str = "https://shop.opendiy.vn";

pub fn site_origin() -> String {
    env::var("SITE_URL")
        .or_else(|_| env::var("OPEN_DIY_SITE_URL"))
        .unwrap_or_else(|_| DEFAULT_SITE_URL.to_string())
        .trim_end_matches('/')
        .to_string()
}

pub fn absolute_url(path: &str) -> String {
    let trimmed = path.trim_start_matches('/');
    if trimmed.is_empty() {
        site_origin()
    } else {
        format!("{}/{}", site_origin(), trimmed)
    }
}

pub fn robots_txt() -> String {
    format!(
        "User-agent: *\nAllow: /\nDisallow: /checkout-success\nSitemap: {}/sitemap.xml\n",
        site_origin()
    )
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "")
}

pub fn sitemap_xml() -> String {
    let urls = [
        ("/", "1.0", "weekly"),
        ("/shop", "0.9", "weekly"),
        ("/about", "0.7", "monthly"),
    ];

    let entries = urls
        .iter()
        .map(|(path, priority, changefreq)| {
            format!(
                "  <url><loc>{}</loc><changefreq>{}</changefreq><priority>{}</priority></url>\n",
                absolute_url(path),
                changefreq,
                priority
            )
        })
        .collect::<String>();

    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{}</urlset>\n",
        entries
    )
}

pub fn organization_json_ld() -> String {
    format!(
        r#"{{"@context":"https://schema.org","@type":"Organization","name":"{}","url":"{}","logo":"{}","image":"{}","sameAs":["https://www.facebook.com/1111759575360830","https://www.youtube.com/@opendiyvn"]}}"#,
        SITE_NAME,
        site_origin(),
        absolute_url("/favicon.ico"),
        absolute_url("/images/dactyl.png")
    )
}

pub fn website_json_ld() -> String {
    format!(
        r#"{{"@context":"https://schema.org","@type":"WebSite","name":"{}","url":"{}","potentialAction":{{"@type":"SearchAction","target":"{}?q={{search_term_string}}","query-input":"required name=search_term_string"}}}}"#,
        SITE_NAME,
        site_origin(),
        absolute_url("/")
    )
}

pub fn faq_json_ld() -> String {
    let questions = [
        (
            "What is open-diy?",
            "open-diy is a small shop for ergonomic 3D printed keyboards, open-source case designs, and configurable keyboard builds.",
        ),
        (
            "Can I customize switches and keycaps?",
            "Yes. The builder lets you choose layout, case color, switches, keycaps, and quantity before adding a build to the cart.",
        ),
        (
            "Are the keyboards open-source?",
            "Yes. The project highlights open-source keyboard designs and encourages the DIY community to print their own cases when they want to build at home.",
        ),
    ];

    let entries = questions
        .iter()
        .map(|(question, answer)| {
            format!(
                r#"{{"@type":"Question","name":"{}","acceptedAnswer":{{"@type":"Answer","text":"{}"}}}}"#,
                escape_json(question),
                escape_json(answer)
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(
        r#"{{"@context":"https://schema.org","@type":"FAQPage","mainEntity":[{}]}}"#,
        entries
    )
}

pub fn product_json_ld(
    name: &str,
    description: &str,
    image_path: &str,
    path: &str,
    price: f64,
) -> String {
    let product_url = absolute_url(path);
    format!(
        r#"{{"@context":"https://schema.org","@type":"Product","name":"{}","description":"{}","image":"{}","brand":{{"@type":"Brand","name":"{}"}},"url":"{}","offers":{{"@type":"Offer","priceCurrency":"USD","price":"{:.2}","availability":"https://schema.org/InStock","url":"{}"}}}}"#,
        escape_json(name),
        escape_json(description),
        absolute_url(image_path),
        SITE_NAME,
        product_url,
        price,
        product_url
    )
}

#[component]
pub fn SeoHead(
    title: String,
    description: String,
    path: String,
    image_path: String,
    #[prop(optional)] robots: Option<String>,
) -> impl IntoView {
    let canonical = absolute_url(&path);
    let image = absolute_url(&image_path);
    let robots = robots.unwrap_or_else(|| "index,follow,max-image-preview:large".to_string());

    view! {
        <Title text=title.clone()/>
        <Meta name="description" content=description.clone()/>
        <Meta name="robots" content=robots/>
        <Meta name="theme-color" content="#0f172a"/>
        <Meta property="og:site_name" content=SITE_NAME/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:title" content=title.clone()/>
        <Meta property="og:description" content=description.clone()/>
        <Meta property="og:url" content=canonical.clone()/>
        <Meta property="og:image" content=image.clone()/>
        <Meta property="og:image:alt" content=title.clone()/>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:title" content=title.clone()/>
        <Meta name="twitter:description" content=description.clone()/>
        <Meta name="twitter:image" content=image.clone()/>
        <Link rel="canonical" href=canonical/>
    }
}

#[component]
pub fn JsonLd(json: String) -> impl IntoView {
    view! {
        <Script type_="application/ld+json">
            {json}
        </Script>
    }
}
