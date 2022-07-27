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

//! Generated file from `kernel/apis/UserActionModule.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:RequestTeleportRequest)
pub struct RequestTeleportRequest {
    // message fields
    // @@protoc_insertion_point(field:RequestTeleportRequest.destination)
    pub destination: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:RequestTeleportRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RequestTeleportRequest {
    fn default() -> &'a RequestTeleportRequest {
        <RequestTeleportRequest as ::protobuf::Message>::default_instance()
    }
}

impl RequestTeleportRequest {
    pub fn new() -> RequestTeleportRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "destination",
            |m: &RequestTeleportRequest| { &m.destination },
            |m: &mut RequestTeleportRequest| { &mut m.destination },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RequestTeleportRequest>(
            "RequestTeleportRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RequestTeleportRequest {
    const NAME: &'static str = "RequestTeleportRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.destination = is.read_string()?;
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
        if !self.destination.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.destination);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.destination.is_empty() {
            os.write_string(1, &self.destination)?;
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

    fn new() -> RequestTeleportRequest {
        RequestTeleportRequest::new()
    }

    fn clear(&mut self) {
        self.destination.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RequestTeleportRequest {
        static instance: RequestTeleportRequest = RequestTeleportRequest {
            destination: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RequestTeleportRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RequestTeleportRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RequestTeleportRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestTeleportRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:RequestTeleportResponse)
pub struct RequestTeleportResponse {
    // special fields
    // @@protoc_insertion_point(special_field:RequestTeleportResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RequestTeleportResponse {
    fn default() -> &'a RequestTeleportResponse {
        <RequestTeleportResponse as ::protobuf::Message>::default_instance()
    }
}

impl RequestTeleportResponse {
    pub fn new() -> RequestTeleportResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RequestTeleportResponse>(
            "RequestTeleportResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RequestTeleportResponse {
    const NAME: &'static str = "RequestTeleportResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RequestTeleportResponse {
        RequestTeleportResponse::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RequestTeleportResponse {
        static instance: RequestTeleportResponse = RequestTeleportResponse {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RequestTeleportResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RequestTeleportResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RequestTeleportResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestTeleportResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"kernel/apis/UserActionModule.proto\":\n\x16RequestTeleportRequest\
    \x12\x20\n\x0bdestination\x18\x01\x20\x01(\tR\x0bdestination\"\x19\n\x17\
    RequestTeleportResponse2a\n\x17UserActionModuleService\x12F\n\x0fRequest\
    Teleport\x12\x17.RequestTeleportRequest\x1a\x18.RequestTeleportResponse\
    \"\0J\xc7\x01\n\x06\x12\x04\0\0\n\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\
    \n\n\x02\x04\0\x12\x04\x02\0\x04\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\
    \x1e\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x1b\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\x16\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x19\x1a\n\t\n\x02\x04\x01\x12\x03\
    \x06\0\"\n\n\n\x03\x04\x01\x01\x12\x03\x06\x08\x1f\n\n\n\x02\x06\0\x12\
    \x04\x08\0\n\x01\n\n\n\x03\x06\0\x01\x12\x03\x08\x08\x1f\n\x0b\n\x04\x06\
    \0\x02\0\x12\x03\t\x04T\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\t\x08\x17\n\
    \x0c\n\x05\x06\0\x02\0\x02\x12\x03\t\x18.\n\x0c\n\x05\x06\0\x02\0\x03\
    \x12\x03\t9Pb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(RequestTeleportRequest::generated_message_descriptor_data());
            messages.push(RequestTeleportResponse::generated_message_descriptor_data());
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
