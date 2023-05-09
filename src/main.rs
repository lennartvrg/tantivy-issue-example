use std::env;

use tantivy::{
    doc,
    schema::{Schema, TextFieldIndexing, TextOptions, BytesOptions},
    IndexBuilder, IndexSettings, IndexSortByField,
};

fn with_text_field() {
    let mut builder = Schema::builder();
    builder.add_text_field(
        "id",
        TextOptions::default()
            .set_fast()
            .set_stored()
            .set_indexing_options(TextFieldIndexing::default().set_tokenizer("raw")),
    );
    builder.add_text_field("text", TextOptions::default().set_stored());

    let schema = builder.build();
    let id_field = schema.get_field("id").unwrap();
    let text_field = schema.get_field("text").unwrap();

    let index = IndexBuilder::default()
        .schema(schema)
        .settings(IndexSettings {
            sort_by_field: Some(IndexSortByField {
                field: "id".to_string(),
                order: tantivy::Order::Desc,
            }),
            ..Default::default()
        })
        .create_in_ram()
        .unwrap();

    let mut writer = index.writer(50_000_000).unwrap();
    writer
        .add_document(doc!(
            id_field => "01H00DQFNPSYS30M0J5ZPZDMNJ",
            text_field => "Some message"
        ))
        .unwrap();
    writer.commit().unwrap();
}

fn with_bytes_field() {
    let mut builder = Schema::builder();
    builder.add_bytes_field(
        "id",
        BytesOptions::default()
            .set_fast()
            .set_stored()
            .set_indexed(),
    );
    builder.add_text_field("text", TextOptions::default().set_stored());

    let schema = builder.build();
    let id_field = schema.get_field("id").unwrap();
    let text_field = schema.get_field("text").unwrap();

    let index = IndexBuilder::default()
        .schema(schema)
        .settings(IndexSettings {
            sort_by_field: Some(IndexSortByField {
                field: "id".to_string(),
                order: tantivy::Order::Desc,
            }),
            ..Default::default()
        })
        .create_in_ram()
        .unwrap();

    let mut writer = index.writer(50_000_000).unwrap();
    writer
        .add_document(doc!(
            id_field => "01H00DQFNPSYS30M0J5ZPZDMNJ".as_bytes(),
            text_field => "Some message"
        ))
        .unwrap();
    writer.commit().unwrap();
}

fn main() {
    match env::args().nth(1) {
        Some(val) if val == "text" => with_text_field(),
        Some(val) if val == "bytes" => with_bytes_field(),
        _ => (),
    };
}
