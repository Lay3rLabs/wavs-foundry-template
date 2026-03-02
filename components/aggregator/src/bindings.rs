#[rustfmt::skip]
wit_bindgen::generate!({
    world: "aggregator-world",
    path: "../../wit-aggregator",
    pub_export_macro: true,
    generate_all,
    features: ["tls"],
});
