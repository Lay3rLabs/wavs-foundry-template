#[rustfmt::skip]
wit_bindgen::generate!({
    world: "wavs-world",
    path: "../../wit",
    pub_export_macro: true,
    generate_all,
    features: ["tls"],
});
