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

//! Generated file from `ecs/components/AvatarAttach.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:PBAvatarAttach)
pub struct PBAvatarAttach {
    // message fields
    // @@protoc_insertion_point(field:PBAvatarAttach.avatar_id)
    pub avatar_id: ::std::string::String,
    // @@protoc_insertion_point(field:PBAvatarAttach.anchor_point_id)
    pub anchor_point_id: ::protobuf::EnumOrUnknown<AvatarAnchorPoint>,
    // special fields
    // @@protoc_insertion_point(special_field:PBAvatarAttach.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PBAvatarAttach {
    fn default() -> &'a PBAvatarAttach {
        <PBAvatarAttach as ::protobuf::Message>::default_instance()
    }
}

impl PBAvatarAttach {
    pub fn new() -> PBAvatarAttach {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &PBAvatarAttach| { &m.avatar_id },
            |m: &mut PBAvatarAttach| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "anchor_point_id",
            |m: &PBAvatarAttach| { &m.anchor_point_id },
            |m: &mut PBAvatarAttach| { &mut m.anchor_point_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PBAvatarAttach>(
            "PBAvatarAttach",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PBAvatarAttach {
    const NAME: &'static str = "PBAvatarAttach";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.avatar_id = is.read_string()?;
                },
                16 => {
                    self.anchor_point_id = is.read_enum_or_unknown()?;
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
        if !self.avatar_id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.avatar_id);
        }
        if self.anchor_point_id != ::protobuf::EnumOrUnknown::new(AvatarAnchorPoint::POSITION) {
            my_size += ::protobuf::rt::int32_size(2, self.anchor_point_id.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.avatar_id.is_empty() {
            os.write_string(1, &self.avatar_id)?;
        }
        if self.anchor_point_id != ::protobuf::EnumOrUnknown::new(AvatarAnchorPoint::POSITION) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.anchor_point_id))?;
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

    fn new() -> PBAvatarAttach {
        PBAvatarAttach::new()
    }

    fn clear(&mut self) {
        self.avatar_id.clear();
        self.anchor_point_id = ::protobuf::EnumOrUnknown::new(AvatarAnchorPoint::POSITION);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PBAvatarAttach {
        static instance: PBAvatarAttach = PBAvatarAttach {
            avatar_id: ::std::string::String::new(),
            anchor_point_id: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PBAvatarAttach {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PBAvatarAttach").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PBAvatarAttach {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PBAvatarAttach {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:AvatarAnchorPoint)
pub enum AvatarAnchorPoint {
    // @@protoc_insertion_point(enum_value:AvatarAnchorPoint.POSITION)
    POSITION = 0,
    // @@protoc_insertion_point(enum_value:AvatarAnchorPoint.NAME_TAG)
    NAME_TAG = 1,
    // @@protoc_insertion_point(enum_value:AvatarAnchorPoint.LEFT_HAND)
    LEFT_HAND = 2,
    // @@protoc_insertion_point(enum_value:AvatarAnchorPoint.RIGHT_HAND)
    RIGHT_HAND = 3,
}

impl ::protobuf::Enum for AvatarAnchorPoint {
    const NAME: &'static str = "AvatarAnchorPoint";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AvatarAnchorPoint> {
        match value {
            0 => ::std::option::Option::Some(AvatarAnchorPoint::POSITION),
            1 => ::std::option::Option::Some(AvatarAnchorPoint::NAME_TAG),
            2 => ::std::option::Option::Some(AvatarAnchorPoint::LEFT_HAND),
            3 => ::std::option::Option::Some(AvatarAnchorPoint::RIGHT_HAND),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [AvatarAnchorPoint] = &[
        AvatarAnchorPoint::POSITION,
        AvatarAnchorPoint::NAME_TAG,
        AvatarAnchorPoint::LEFT_HAND,
        AvatarAnchorPoint::RIGHT_HAND,
    ];
}

impl ::protobuf::EnumFull for AvatarAnchorPoint {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("AvatarAnchorPoint").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for AvatarAnchorPoint {
    fn default() -> Self {
        AvatarAnchorPoint::POSITION
    }
}

impl AvatarAnchorPoint {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<AvatarAnchorPoint>("AvatarAnchorPoint")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!ecs/components/AvatarAttach.proto\x1a\x0fcommon/id.proto\"i\n\x0ePBAv\
    atarAttach\x12\x1b\n\tavatar_id\x18\x01\x20\x01(\tR\x08avatarId\x12:\n\
    \x0fanchor_point_id\x18\x02\x20\x01(\x0e2\x12.AvatarAnchorPointR\ranchor\
    PointId*N\n\x11AvatarAnchorPoint\x12\x0c\n\x08POSITION\x10\0\x12\x0c\n\
    \x08NAME_TAG\x10\x01\x12\r\n\tLEFT_HAND\x10\x02\x12\x0e\n\nRIGHT_HAND\
    \x10\x03B\x05\x80\xb5\x18\xb1\x08J\xf6\x02\n\x06\x12\x04\0\0\x0f\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\0\x19\n\x08\n\
    \x01\x08\x12\x03\x03\0!\n\x0b\n\x04\x08\xd0\x86\x03\x12\x03\x03\0!\n\n\n\
    \x02\x05\0\x12\x04\x05\0\n\x01\n\n\n\x03\x05\0\x01\x12\x03\x05\x05\x16\n\
    \x0b\n\x04\x05\0\x02\0\x12\x03\x06\x02\x0f\n\x0c\n\x05\x05\0\x02\0\x01\
    \x12\x03\x06\x02\n\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x06\r\x0e\n\x0b\n\
    \x04\x05\0\x02\x01\x12\x03\x07\x02\x0f\n\x0c\n\x05\x05\0\x02\x01\x01\x12\
    \x03\x07\x02\n\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x07\r\x0e\n\x0b\n\
    \x04\x05\0\x02\x02\x12\x03\x08\x02\x10\n\x0c\n\x05\x05\0\x02\x02\x01\x12\
    \x03\x08\x02\x0b\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x08\x0e\x0f\n\x0b\
    \n\x04\x05\0\x02\x03\x12\x03\t\x02\x11\n\x0c\n\x05\x05\0\x02\x03\x01\x12\
    \x03\t\x02\x0c\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\t\x0f\x10\n\n\n\x02\
    \x04\0\x12\x04\x0c\0\x0f\x01\n\n\n\x03\x04\0\x01\x12\x03\x0c\x08\x16\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\r\x02\x17\n\x0c\n\x05\x04\0\x02\0\x05\x12\
    \x03\r\x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\r\t\x12\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\r\x15\x16\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0e\
    \x02(\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03\x0e\x02\x13\n\x0c\n\x05\x04\
    \0\x02\x01\x01\x12\x03\x0e\x14#\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\
    \x0e&'b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::id::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PBAvatarAttach::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(AvatarAnchorPoint::generated_enum_descriptor_data());
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
