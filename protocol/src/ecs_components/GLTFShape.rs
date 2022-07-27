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

//! Generated file from `ecs/components/GLTFShape.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:PBGLTFShape)
pub struct PBGLTFShape {
    // message fields
    // @@protoc_insertion_point(field:PBGLTFShape.with_collisions)
    pub with_collisions: bool,
    // @@protoc_insertion_point(field:PBGLTFShape.is_pointer_blocker)
    pub is_pointer_blocker: bool,
    // @@protoc_insertion_point(field:PBGLTFShape.visible)
    pub visible: bool,
    // @@protoc_insertion_point(field:PBGLTFShape.src)
    pub src: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:PBGLTFShape.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PBGLTFShape {
    fn default() -> &'a PBGLTFShape {
        <PBGLTFShape as ::protobuf::Message>::default_instance()
    }
}

impl PBGLTFShape {
    pub fn new() -> PBGLTFShape {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "with_collisions",
            |m: &PBGLTFShape| { &m.with_collisions },
            |m: &mut PBGLTFShape| { &mut m.with_collisions },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_pointer_blocker",
            |m: &PBGLTFShape| { &m.is_pointer_blocker },
            |m: &mut PBGLTFShape| { &mut m.is_pointer_blocker },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "visible",
            |m: &PBGLTFShape| { &m.visible },
            |m: &mut PBGLTFShape| { &mut m.visible },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "src",
            |m: &PBGLTFShape| { &m.src },
            |m: &mut PBGLTFShape| { &mut m.src },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PBGLTFShape>(
            "PBGLTFShape",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PBGLTFShape {
    const NAME: &'static str = "PBGLTFShape";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.with_collisions = is.read_bool()?;
                },
                16 => {
                    self.is_pointer_blocker = is.read_bool()?;
                },
                24 => {
                    self.visible = is.read_bool()?;
                },
                34 => {
                    self.src = is.read_string()?;
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
        if self.with_collisions != false {
            my_size += 1 + 1;
        }
        if self.is_pointer_blocker != false {
            my_size += 1 + 1;
        }
        if self.visible != false {
            my_size += 1 + 1;
        }
        if !self.src.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.src);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.with_collisions != false {
            os.write_bool(1, self.with_collisions)?;
        }
        if self.is_pointer_blocker != false {
            os.write_bool(2, self.is_pointer_blocker)?;
        }
        if self.visible != false {
            os.write_bool(3, self.visible)?;
        }
        if !self.src.is_empty() {
            os.write_string(4, &self.src)?;
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

    fn new() -> PBGLTFShape {
        PBGLTFShape::new()
    }

    fn clear(&mut self) {
        self.with_collisions = false;
        self.is_pointer_blocker = false;
        self.visible = false;
        self.src.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PBGLTFShape {
        static instance: PBGLTFShape = PBGLTFShape {
            with_collisions: false,
            is_pointer_blocker: false,
            visible: false,
            src: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PBGLTFShape {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PBGLTFShape").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PBGLTFShape {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PBGLTFShape {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eecs/components/GLTFShape.proto\x1a\x0fcommon/id.proto\"\x90\x01\n\
    \x0bPBGLTFShape\x12'\n\x0fwith_collisions\x18\x01\x20\x01(\x08R\x0ewithC\
    ollisions\x12,\n\x12is_pointer_blocker\x18\x02\x20\x01(\x08R\x10isPointe\
    rBlocker\x12\x18\n\x07visible\x18\x03\x20\x01(\x08R\x07visible\x12\x10\n\
    \x03src\x18\x04\x20\x01(\tR\x03srcB\x05\x80\xb5\x18\x91\x08J\xa8\x02\n\
    \x06\x12\x04\0\x03\n\x01\n\x08\n\x01\x0c\x12\x03\0\x03\x15\n\t\n\x02\x03\
    \0\x12\x03\x02\0\x19\n\x08\n\x01\x08\x12\x03\x03\0!\n\x0b\n\x04\x08\xd0\
    \x86\x03\x12\x03\x03\0!\n\n\n\x02\x04\0\x12\x04\x05\0\n\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x05\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x02\
    \x1b\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x06\x02\x06\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x06\x07\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x06\
    \x19\x1a\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x07\x02\x1e\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03\x07\x02\x06\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x07\x07\x19\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x07\x1c\x1d\n\x0b\n\
    \x04\x04\0\x02\x02\x12\x03\x08\x02\x13\n\x0c\n\x05\x04\0\x02\x02\x05\x12\
    \x03\x08\x02\x06\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x08\x07\x0e\n\x0c\
    \n\x05\x04\0\x02\x02\x03\x12\x03\x08\x11\x12\n\x0b\n\x04\x04\0\x02\x03\
    \x12\x03\t\x02\x11\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\t\x02\x08\n\x0c\
    \n\x05\x04\0\x02\x03\x01\x12\x03\t\t\x0c\n\x0c\n\x05\x04\0\x02\x03\x03\
    \x12\x03\t\x0f\x10b\x06proto3\
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
            messages.push(PBGLTFShape::generated_message_descriptor_data());
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
