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

//! Generated file from `ecs/components/CylinderShape.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:PBCylinderShape)
pub struct PBCylinderShape {
    // message fields
    // @@protoc_insertion_point(field:PBCylinderShape.with_collisions)
    pub with_collisions: bool,
    // @@protoc_insertion_point(field:PBCylinderShape.is_pointer_blocker)
    pub is_pointer_blocker: bool,
    // @@protoc_insertion_point(field:PBCylinderShape.visible)
    pub visible: bool,
    // @@protoc_insertion_point(field:PBCylinderShape.radius_top)
    pub radius_top: f32,
    // @@protoc_insertion_point(field:PBCylinderShape.radius_bottom)
    pub radius_bottom: f32,
    // special fields
    // @@protoc_insertion_point(special_field:PBCylinderShape.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PBCylinderShape {
    fn default() -> &'a PBCylinderShape {
        <PBCylinderShape as ::protobuf::Message>::default_instance()
    }
}

impl PBCylinderShape {
    pub fn new() -> PBCylinderShape {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "with_collisions",
            |m: &PBCylinderShape| { &m.with_collisions },
            |m: &mut PBCylinderShape| { &mut m.with_collisions },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_pointer_blocker",
            |m: &PBCylinderShape| { &m.is_pointer_blocker },
            |m: &mut PBCylinderShape| { &mut m.is_pointer_blocker },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "visible",
            |m: &PBCylinderShape| { &m.visible },
            |m: &mut PBCylinderShape| { &mut m.visible },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "radius_top",
            |m: &PBCylinderShape| { &m.radius_top },
            |m: &mut PBCylinderShape| { &mut m.radius_top },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "radius_bottom",
            |m: &PBCylinderShape| { &m.radius_bottom },
            |m: &mut PBCylinderShape| { &mut m.radius_bottom },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PBCylinderShape>(
            "PBCylinderShape",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PBCylinderShape {
    const NAME: &'static str = "PBCylinderShape";

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
                37 => {
                    self.radius_top = is.read_float()?;
                },
                45 => {
                    self.radius_bottom = is.read_float()?;
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
        if self.radius_top != 0. {
            my_size += 1 + 4;
        }
        if self.radius_bottom != 0. {
            my_size += 1 + 4;
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
        if self.radius_top != 0. {
            os.write_float(4, self.radius_top)?;
        }
        if self.radius_bottom != 0. {
            os.write_float(5, self.radius_bottom)?;
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

    fn new() -> PBCylinderShape {
        PBCylinderShape::new()
    }

    fn clear(&mut self) {
        self.with_collisions = false;
        self.is_pointer_blocker = false;
        self.visible = false;
        self.radius_top = 0.;
        self.radius_bottom = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PBCylinderShape {
        static instance: PBCylinderShape = PBCylinderShape {
            with_collisions: false,
            is_pointer_blocker: false,
            visible: false,
            radius_top: 0.,
            radius_bottom: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PBCylinderShape {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PBCylinderShape").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PBCylinderShape {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PBCylinderShape {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"ecs/components/CylinderShape.proto\x1a\x0fcommon/id.proto\"\xc6\x01\
    \n\x0fPBCylinderShape\x12'\n\x0fwith_collisions\x18\x01\x20\x01(\x08R\
    \x0ewithCollisions\x12,\n\x12is_pointer_blocker\x18\x02\x20\x01(\x08R\
    \x10isPointerBlocker\x12\x18\n\x07visible\x18\x03\x20\x01(\x08R\x07visib\
    le\x12\x1d\n\nradius_top\x18\x04\x20\x01(\x02R\tradiusTop\x12#\n\rradius\
    _bottom\x18\x05\x20\x01(\x02R\x0cradiusBottomB\x05\x80\xb5\x18\xf8\x07J\
    \xdf\x02\n\x06\x12\x04\0\x03\x0b\x01\n\x08\n\x01\x0c\x12\x03\0\x03\x15\n\
    \t\n\x02\x03\0\x12\x03\x02\0\x19\n\x08\n\x01\x08\x12\x03\x03\0!\n\x0b\n\
    \x04\x08\xd0\x86\x03\x12\x03\x03\0!\n\n\n\x02\x04\0\x12\x04\x05\0\x0b\
    \x01\n\n\n\x03\x04\0\x01\x12\x03\x05\x08\x17\n\x0b\n\x04\x04\0\x02\0\x12\
    \x03\x06\x02\x1b\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x06\x02\x06\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x06\x07\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x06\x19\x1a\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x07\x02\x1e\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x03\x07\x02\x06\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x07\x07\x19\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x07\x1c\x1d\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\x08\x02\x13\n\x0c\n\x05\x04\0\x02\x02\
    \x05\x12\x03\x08\x02\x06\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x08\x07\
    \x0e\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x08\x11\x12\n\x0b\n\x04\x04\0\
    \x02\x03\x12\x03\t\x02\x17\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\t\x02\
    \x07\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\t\x08\x12\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03\t\x15\x16\n\x0b\n\x04\x04\0\x02\x04\x12\x03\n\x02\
    \x1a\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\n\x02\x07\n\x0c\n\x05\x04\0\
    \x02\x04\x01\x12\x03\n\x08\x15\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\n\
    \x18\x19b\x06proto3\
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
            messages.push(PBCylinderShape::generated_message_descriptor_data());
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
