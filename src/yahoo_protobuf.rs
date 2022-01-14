// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `yahoo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct YahooFinanceQuote {
    // message fields
    id: ::protobuf::SingularField<::std::string::String>,
    price: ::std::option::Option<f32>,
    time: ::std::option::Option<i64>,
    currency: ::protobuf::SingularField<::std::string::String>,
    changePercent: ::std::option::Option<f32>,
    bid: ::std::option::Option<f32>,
    ask: ::std::option::Option<f32>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a YahooFinanceQuote {
    fn default() -> &'a YahooFinanceQuote {
        <YahooFinanceQuote as ::protobuf::Message>::default_instance()
    }
}

impl YahooFinanceQuote {
    pub fn new() -> YahooFinanceQuote {
        ::std::default::Default::default()
    }

    // optional string id = 1;


    pub fn get_id(&self) -> &str {
        match self.id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        }
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        self.id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional float price = 2;


    pub fn get_price(&self) -> f32 {
        self.price.unwrap_or(0.)
    }
    pub fn clear_price(&mut self) {
        self.price = ::std::option::Option::None;
    }

    pub fn has_price(&self) -> bool {
        self.price.is_some()
    }

    // Param is passed by value, moved
    pub fn set_price(&mut self, v: f32) {
        self.price = ::std::option::Option::Some(v);
    }

    // optional sint64 time = 3;


    pub fn get_time(&self) -> i64 {
        self.time.unwrap_or(0)
    }
    pub fn clear_time(&mut self) {
        self.time = ::std::option::Option::None;
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: i64) {
        self.time = ::std::option::Option::Some(v);
    }

    // optional string currency = 4;


    pub fn get_currency(&self) -> &str {
        match self.currency.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_currency(&mut self) {
        self.currency.clear();
    }

    pub fn has_currency(&self) -> bool {
        self.currency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currency(&mut self, v: ::std::string::String) {
        self.currency = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currency(&mut self) -> &mut ::std::string::String {
        if self.currency.is_none() {
            self.currency.set_default();
        }
        self.currency.as_mut().unwrap()
    }

    // Take field
    pub fn take_currency(&mut self) -> ::std::string::String {
        self.currency.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional float changePercent = 8;


    pub fn get_changePercent(&self) -> f32 {
        self.changePercent.unwrap_or(0.)
    }
    pub fn clear_changePercent(&mut self) {
        self.changePercent = ::std::option::Option::None;
    }

    pub fn has_changePercent(&self) -> bool {
        self.changePercent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_changePercent(&mut self, v: f32) {
        self.changePercent = ::std::option::Option::Some(v);
    }

    // optional float bid = 23;


    pub fn get_bid(&self) -> f32 {
        self.bid.unwrap_or(0.)
    }
    pub fn clear_bid(&mut self) {
        self.bid = ::std::option::Option::None;
    }

    pub fn has_bid(&self) -> bool {
        self.bid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bid(&mut self, v: f32) {
        self.bid = ::std::option::Option::Some(v);
    }

    // optional float ask = 25;


    pub fn get_ask(&self) -> f32 {
        self.ask.unwrap_or(0.)
    }
    pub fn clear_ask(&mut self) {
        self.ask = ::std::option::Option::None;
    }

    pub fn has_ask(&self) -> bool {
        self.ask.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ask(&mut self, v: f32) {
        self.ask = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for YahooFinanceQuote {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.price = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.time = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.currency)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.changePercent = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.bid = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.ask = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.id.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.price {
            my_size += 5;
        }
        if let Some(v) = self.time {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        }
        if let Some(ref v) = self.currency.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.changePercent {
            my_size += 5;
        }
        if let Some(v) = self.bid {
            my_size += 6;
        }
        if let Some(v) = self.ask {
            my_size += 6;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.id.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.price {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.time {
            os.write_sint64(3, v)?;
        }
        if let Some(ref v) = self.currency.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.changePercent {
            os.write_float(8, v)?;
        }
        if let Some(v) = self.bid {
            os.write_float(23, v)?;
        }
        if let Some(v) = self.ask {
            os.write_float(25, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> YahooFinanceQuote {
        YahooFinanceQuote::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &YahooFinanceQuote| { &m.id },
                |m: &mut YahooFinanceQuote| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "price",
                |m: &YahooFinanceQuote| { &m.price },
                |m: &mut YahooFinanceQuote| { &mut m.price },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                "time",
                |m: &YahooFinanceQuote| { &m.time },
                |m: &mut YahooFinanceQuote| { &mut m.time },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "currency",
                |m: &YahooFinanceQuote| { &m.currency },
                |m: &mut YahooFinanceQuote| { &mut m.currency },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "changePercent",
                |m: &YahooFinanceQuote| { &m.changePercent },
                |m: &mut YahooFinanceQuote| { &mut m.changePercent },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "bid",
                |m: &YahooFinanceQuote| { &m.bid },
                |m: &mut YahooFinanceQuote| { &mut m.bid },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                "ask",
                |m: &YahooFinanceQuote| { &m.ask },
                |m: &mut YahooFinanceQuote| { &mut m.ask },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<YahooFinanceQuote>(
                "YahooFinanceQuote",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static YahooFinanceQuote {
        static instance: ::protobuf::rt::LazyV2<YahooFinanceQuote> = ::protobuf::rt::LazyV2::INIT;
        instance.get(YahooFinanceQuote::new)
    }
}

impl ::protobuf::Clear for YahooFinanceQuote {
    fn clear(&mut self) {
        self.id.clear();
        self.price = ::std::option::Option::None;
        self.time = ::std::option::Option::None;
        self.currency.clear();
        self.changePercent = ::std::option::Option::None;
        self.bid = ::std::option::Option::None;
        self.ask = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for YahooFinanceQuote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for YahooFinanceQuote {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0byahoo.proto\"\xb3\x01\n\x11YahooFinanceQuote\x12\x0e\n\x02id\x18\
    \x01\x20\x01(\tR\x02id\x12\x14\n\x05price\x18\x02\x20\x01(\x02R\x05price\
    \x12\x12\n\x04time\x18\x03\x20\x01(\x12R\x04time\x12\x1a\n\x08currency\
    \x18\x04\x20\x01(\tR\x08currency\x12$\n\rchangePercent\x18\x08\x20\x01(\
    \x02R\rchangePercent\x12\x10\n\x03bid\x18\x17\x20\x01(\x02R\x03bid\x12\
    \x10\n\x03ask\x18\x19\x20\x01(\x02R\x03askJ\xfb\x05\n\x06\x12\x04\0\0\
    \x0b\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n/\n\x02\x04\0\x12\x04\x03\0\
    \x0b\x01\x1a#\x20Describes\x20a\x20complete\x20.proto\x20file.\n\n\n\n\
    \x03\x04\0\x01\x12\x03\x03\x08\x19\n9\n\x04\x04\0\x02\0\x12\x03\x04\x02\
    \x19\",\x20file\x20name,\x20relative\x20to\x20root\x20of\x20source\x20tr\
    ee\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x04\x02\n\n\x0c\n\x05\x04\0\x02\
    \0\x05\x12\x03\x04\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x04\x12\
    \x14\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x04\x17\x18\n*\n\x04\x04\0\x02\
    \x01\x12\x03\x05\x02\x1b\"\x1d\x20e.g.\x20\"foo\",\x20\"foo.bar\",\x20et\
    c.\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x05\x02\n\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03\x05\x0b\x10\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x05\x11\x16\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x05\x19\x1a\n*\n\x04\
    \x04\0\x02\x02\x12\x03\x06\x02\x1b\"\x1d\x20e.g.\x20\"foo\",\x20\"foo.ba\
    r\",\x20etc.\n\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x06\x02\n\n\x0c\n\
    \x05\x04\0\x02\x02\x05\x12\x03\x06\x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x01\
    \x12\x03\x06\x12\x16\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x06\x19\x1a\n\
    *\n\x04\x04\0\x02\x03\x12\x03\x07\x02\x1f\"\x1d\x20e.g.\x20\"foo\",\x20\
    \"foo.bar\",\x20etc.\n\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03\x07\x02\n\n\
    \x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x07\x0b\x11\n\x0c\n\x05\x04\0\x02\
    \x03\x01\x12\x03\x07\x12\x1a\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x07\
    \x1d\x1e\n*\n\x04\x04\0\x02\x04\x12\x03\x08\x02#\"\x1d\x20e.g.\x20\"foo\
    \",\x20\"foo.bar\",\x20etc.\n\n\x0c\n\x05\x04\0\x02\x04\x04\x12\x03\x08\
    \x02\n\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x08\x0b\x10\n\x0c\n\x05\x04\
    \0\x02\x04\x01\x12\x03\x08\x11\x1e\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\
    \x08!\"\n*\n\x04\x04\0\x02\x05\x12\x03\t\x02\x1a\"\x1d\x20e.g.\x20\"foo\
    \",\x20\"foo.bar\",\x20etc.\n\n\x0c\n\x05\x04\0\x02\x05\x04\x12\x03\t\
    \x02\n\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\t\x0b\x10\n\x0c\n\x05\x04\0\
    \x02\x05\x01\x12\x03\t\x11\x14\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\t\
    \x17\x19\n\x0b\n\x04\x04\0\x02\x06\x12\x03\n\x02\x1a\n\x0c\n\x05\x04\0\
    \x02\x06\x04\x12\x03\n\x02\n\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\n\x0b\
    \x10\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\n\x11\x14\n\x0c\n\x05\x04\0\
    \x02\x06\x03\x12\x03\n\x17\x19\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
