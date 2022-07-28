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

//! Generated file from `ecs/components/AudioStream.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:PBAudioStream)
pub struct PBAudioStream {
    // message fields
    // @@protoc_insertion_point(field:PBAudioStream.playing)
    pub playing: bool,
    // @@protoc_insertion_point(field:PBAudioStream.volume)
    pub volume: f32,
    // @@protoc_insertion_point(field:PBAudioStream.url)
    pub url: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:PBAudioStream.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PBAudioStream {
    fn default() -> &'a PBAudioStream {
        <PBAudioStream as ::protobuf::Message>::default_instance()
    }
}

impl PBAudioStream {
    pub fn new() -> PBAudioStream {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "playing",
            |m: &PBAudioStream| { &m.playing },
            |m: &mut PBAudioStream| { &mut m.playing },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "volume",
            |m: &PBAudioStream| { &m.volume },
            |m: &mut PBAudioStream| { &mut m.volume },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "url",
            |m: &PBAudioStream| { &m.url },
            |m: &mut PBAudioStream| { &mut m.url },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PBAudioStream>(
            "PBAudioStream",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PBAudioStream {
    const NAME: &'static str = "PBAudioStream";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.playing = is.read_bool()?;
                },
                21 => {
                    self.volume = is.read_float()?;
                },
                26 => {
                    self.url = is.read_string()?;
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
        if self.playing != false {
            my_size += 1 + 1;
        }
        if self.volume != 0. {
            my_size += 1 + 4;
        }
        if !self.url.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.url);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.playing != false {
            os.write_bool(1, self.playing)?;
        }
        if self.volume != 0. {
            os.write_float(2, self.volume)?;
        }
        if !self.url.is_empty() {
            os.write_string(3, &self.url)?;
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

    fn new() -> PBAudioStream {
        PBAudioStream::new()
    }

    fn clear(&mut self) {
        self.playing = false;
        self.volume = 0.;
        self.url.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PBAudioStream {
        static instance: PBAudioStream = PBAudioStream {
            playing: false,
            volume: 0.,
            url: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PBAudioStream {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PBAudioStream").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PBAudioStream {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PBAudioStream {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20ecs/components/AudioStream.proto\x1a\x0fcommon/id.proto\"S\n\rPBAu\
    dioStream\x12\x18\n\x07playing\x18\x01\x20\x01(\x08R\x07playing\x12\x16\
    \n\x06volume\x18\x02\x20\x01(\x02R\x06volume\x12\x10\n\x03url\x18\x03\
    \x20\x01(\tR\x03urlB\x05\x80\xb5\x18\xfd\x07J\xf1\x01\n\x06\x12\x04\0\
    \x03\t\x01\n\x08\n\x01\x0c\x12\x03\0\x03\x15\n\t\n\x02\x03\0\x12\x03\x02\
    \0\x19\n\x08\n\x01\x08\x12\x03\x03\0!\n\x0b\n\x04\x08\xd0\x86\x03\x12\
    \x03\x03\0!\n\n\n\x02\x04\0\x12\x04\x05\0\t\x01\n\n\n\x03\x04\0\x01\x12\
    \x03\x05\x08\x15\n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x02\x13\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x06\x02\x06\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x06\x07\x0e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x06\x11\x12\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\x07\x02\x13\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\
    \x07\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x07\x08\x0e\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\x07\x11\x12\n\x0b\n\x04\x04\0\x02\x02\x12\
    \x03\x08\x02\x11\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x08\x02\x08\n\x0c\
    \n\x05\x04\0\x02\x02\x01\x12\x03\x08\t\x0c\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x08\x0f\x10b\x06proto3\
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
            messages.push(PBAudioStream::generated_message_descriptor_data());
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
