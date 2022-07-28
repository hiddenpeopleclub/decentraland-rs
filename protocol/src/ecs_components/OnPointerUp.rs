// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `ecs/components/OnPointerUp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:PBOnPointerUp)
pub struct PBOnPointerUp {
    // message fields
    // @@protoc_insertion_point(field:PBOnPointerUp.button)
    pub button: ::protobuf::EnumOrUnknown<super::ActionButton::ActionButton>,
    // @@protoc_insertion_point(field:PBOnPointerUp.hover_text)
    pub hover_text: ::std::string::String,
    // @@protoc_insertion_point(field:PBOnPointerUp.distance)
    pub distance: f32,
    // @@protoc_insertion_point(field:PBOnPointerUp.show_feedback)
    pub show_feedback: bool,
    // special fields
    // @@protoc_insertion_point(special_field:PBOnPointerUp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PBOnPointerUp {
    fn default() -> &'a PBOnPointerUp {
        <PBOnPointerUp as ::protobuf::Message>::default_instance()
    }
}

impl PBOnPointerUp {
    pub fn new() -> PBOnPointerUp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "button",
            |m: &PBOnPointerUp| { &m.button },
            |m: &mut PBOnPointerUp| { &mut m.button },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hover_text",
            |m: &PBOnPointerUp| { &m.hover_text },
            |m: &mut PBOnPointerUp| { &mut m.hover_text },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "distance",
            |m: &PBOnPointerUp| { &m.distance },
            |m: &mut PBOnPointerUp| { &mut m.distance },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "show_feedback",
            |m: &PBOnPointerUp| { &m.show_feedback },
            |m: &mut PBOnPointerUp| { &mut m.show_feedback },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PBOnPointerUp>(
            "PBOnPointerUp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PBOnPointerUp {
    const NAME: &'static str = "PBOnPointerUp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.button = is.read_enum_or_unknown()?;
                },
                18 => {
                    self.hover_text = is.read_string()?;
                },
                29 => {
                    self.distance = is.read_float()?;
                },
                32 => {
                    self.show_feedback = is.read_bool()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.button != ::protobuf::EnumOrUnknown::new(super::ActionButton::ActionButton::POINTER) {
            my_size += ::protobuf::rt::int32_size(1, self.button.value());
        }
        if !self.hover_text.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.hover_text);
        }
        if self.distance != 0. {
            my_size += 1 + 4;
        }
        if self.show_feedback != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.button != ::protobuf::EnumOrUnknown::new(super::ActionButton::ActionButton::POINTER) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.button))?;
        }
        if !self.hover_text.is_empty() {
            os.write_string(2, &self.hover_text)?;
        }
        if self.distance != 0. {
            os.write_float(3, self.distance)?;
        }
        if self.show_feedback != false {
            os.write_bool(4, self.show_feedback)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PBOnPointerUp {
        PBOnPointerUp::new()
    }

    fn clear(&mut self) {
        self.button = ::protobuf::EnumOrUnknown::new(super::ActionButton::ActionButton::POINTER);
        self.hover_text.clear();
        self.distance = 0.;
        self.show_feedback = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PBOnPointerUp {
        static instance: PBOnPointerUp = PBOnPointerUp {
            button: ::protobuf::EnumOrUnknown::from_i32(0),
            hover_text: ::std::string::String::new(),
            distance: 0.,
            show_feedback: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PBOnPointerUp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PBOnPointerUp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PBOnPointerUp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PBOnPointerUp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20ecs/components/OnPointerUp.proto\x1a\x0fcommon/id.proto\x1a\x19com\
    mon/ActionButton.proto\"\x96\x01\n\rPBOnPointerUp\x12%\n\x06button\x18\
    \x01\x20\x01(\x0e2\r.ActionButtonR\x06button\x12\x1d\n\nhover_text\x18\
    \x02\x20\x01(\tR\thoverText\x12\x1a\n\x08distance\x18\x03\x20\x01(\x02R\
    \x08distance\x12#\n\rshow_feedback\x18\x04\x20\x01(\x08R\x0cshowFeedback\
    B\x05\x80\xb5\x18\xa5\x08J\xb3\x02\n\x06\x12\x04\0\x03\x0b\x01\n\x08\n\
    \x01\x0c\x12\x03\0\x03\x15\n\t\n\x02\x03\0\x12\x03\x02\0\x19\n\x08\n\x01\
    \x08\x12\x03\x03\0!\n\x0b\n\x04\x08\xd0\x86\x03\x12\x03\x03\0!\n\t\n\x02\
    \x03\x01\x12\x03\x04\0#\n\n\n\x02\x04\0\x12\x04\x06\0\x0b\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x06\x08\x15\n\x0b\n\x04\x04\0\x02\0\x12\x03\x07\x02\
    \x1a\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x07\x02\x0e\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x07\x0f\x15\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x07\
    \x18\x19\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x08\x02\x18\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03\x08\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x08\t\x13\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x08\x16\x17\n\x0b\n\x04\
    \x04\0\x02\x02\x12\x03\t\x02\x15\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\t\
    \x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\t\x08\x10\n\x0c\n\x05\x04\
    \0\x02\x02\x03\x12\x03\t\x13\x14\n\x0b\n\x04\x04\0\x02\x03\x12\x03\n\x02\
    \x19\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\n\x02\x06\n\x0c\n\x05\x04\0\
    \x02\x03\x01\x12\x03\n\x07\x14\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\n\
    \x17\x18b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::id::file_descriptor().clone());
            deps.push(super::ActionButton::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PBOnPointerUp::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
