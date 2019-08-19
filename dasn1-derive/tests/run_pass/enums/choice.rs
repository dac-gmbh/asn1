use dasn1::identifier::*;
use dasn1_derive::*;

#[derive(AsnType)]
enum Choice {
    Foo(u8),
    Bar(String),
    Baz,
}

#[test]
fn adt_enum_identifier_is_automatic_context() {
    for (i, variant) in [Choice::Foo(0), Choice::Bar(String::new()), Choice::Baz].into_iter().enumerate() {
        assert_eq!(variant.identifier(), Identifier::new(Class::Context, i as u32));
    }
}


#[derive(AsnType)]
enum Nested {
    Inner(Choice),
}


#[test]
fn correct_choice_tag_encoding() {
    assert_eq!(Nested::Inner(Choice::Foo(0)).tag_encoding(), TagEncoding::Explicit);
}
