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

//! Generated file from `kernel/apis/SocialController.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:InitRequest)
pub struct InitRequest {
    // special fields
    // @@protoc_insertion_point(special_field:InitRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a InitRequest {
    fn default() -> &'a InitRequest {
        <InitRequest as ::protobuf::Message>::default_instance()
    }
}

impl InitRequest {
    pub fn new() -> InitRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<InitRequest>(
            "InitRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for InitRequest {
    const NAME: &'static str = "InitRequest";

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

    fn new() -> InitRequest {
        InitRequest::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static InitRequest {
        static instance: InitRequest = InitRequest {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for InitRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("InitRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for InitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InitRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:SocialEvent)
pub struct SocialEvent {
    // message fields
    // @@protoc_insertion_point(field:SocialEvent.event)
    pub event: ::std::string::String,
    // @@protoc_insertion_point(field:SocialEvent.payload)
    pub payload: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:SocialEvent.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SocialEvent {
    fn default() -> &'a SocialEvent {
        <SocialEvent as ::protobuf::Message>::default_instance()
    }
}

impl SocialEvent {
    pub fn new() -> SocialEvent {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "event",
            |m: &SocialEvent| { &m.event },
            |m: &mut SocialEvent| { &mut m.event },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "payload",
            |m: &SocialEvent| { &m.payload },
            |m: &mut SocialEvent| { &mut m.payload },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SocialEvent>(
            "SocialEvent",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SocialEvent {
    const NAME: &'static str = "SocialEvent";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.event = is.read_string()?;
                },
                18 => {
                    self.payload = is.read_string()?;
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
        if !self.event.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.event);
        }
        if !self.payload.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.payload);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.event.is_empty() {
            os.write_string(1, &self.event)?;
        }
        if !self.payload.is_empty() {
            os.write_string(2, &self.payload)?;
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

    fn new() -> SocialEvent {
        SocialEvent::new()
    }

    fn clear(&mut self) {
        self.event.clear();
        self.payload.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SocialEvent {
        static instance: SocialEvent = SocialEvent {
            event: ::std::string::String::new(),
            payload: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SocialEvent {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SocialEvent").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SocialEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SocialEvent {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:GetAvatarEventsResponse)
pub struct GetAvatarEventsResponse {
    // message fields
    // @@protoc_insertion_point(field:GetAvatarEventsResponse.events)
    pub events: ::std::vec::Vec<SocialEvent>,
    // special fields
    // @@protoc_insertion_point(special_field:GetAvatarEventsResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetAvatarEventsResponse {
    fn default() -> &'a GetAvatarEventsResponse {
        <GetAvatarEventsResponse as ::protobuf::Message>::default_instance()
    }
}

impl GetAvatarEventsResponse {
    pub fn new() -> GetAvatarEventsResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "events",
            |m: &GetAvatarEventsResponse| { &m.events },
            |m: &mut GetAvatarEventsResponse| { &mut m.events },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetAvatarEventsResponse>(
            "GetAvatarEventsResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetAvatarEventsResponse {
    const NAME: &'static str = "GetAvatarEventsResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.events.push(is.read_message()?);
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
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.events {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetAvatarEventsResponse {
        GetAvatarEventsResponse::new()
    }

    fn clear(&mut self) {
        self.events.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetAvatarEventsResponse {
        static instance: GetAvatarEventsResponse = GetAvatarEventsResponse {
            events: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetAvatarEventsResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetAvatarEventsResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetAvatarEventsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetAvatarEventsResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"kernel/apis/SocialController.proto\"\r\n\x0bInitRequest\"=\n\x0bSoci\
    alEvent\x12\x14\n\x05event\x18\x01\x20\x01(\tR\x05event\x12\x18\n\x07pay\
    load\x18\x02\x20\x01(\tR\x07payload\"?\n\x17GetAvatarEventsResponse\x12$\
    \n\x06events\x18\x01\x20\x03(\x0b2\x0c.SocialEventR\x06events2W\n\x17Soc\
    ialControllerService\x12<\n\x10PullAvatarEvents\x12\x0c.InitRequest\x1a\
    \x18.GetAvatarEventsResponse\"\0J\xdb\x02\n\x06\x12\x04\0\0\x0e\x01\n\
    \x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x04\0\x12\x03\x02\0\x16\n\n\n\
    \x03\x04\0\x01\x12\x03\x02\x08\x13\n\n\n\x02\x04\x01\x12\x04\x03\0\x06\
    \x01\n\n\n\x03\x04\x01\x01\x12\x03\x03\x08\x13\n\x0b\n\x04\x04\x01\x02\0\
    \x12\x03\x04\x02\x13\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x04\x02\x08\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x04\t\x0e\n\x0c\n\x05\x04\x01\x02\0\
    \x03\x12\x03\x04\x11\x12\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x05\x02\x15\
    \n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x05\x02\x08\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x05\t\x10\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x05\x13\x14\n\n\n\x02\x04\x02\x12\x04\x08\0\n\x01\n\n\n\x03\x04\x02\x01\
    \x12\x03\x08\x08\x1f\n\x0b\n\x04\x04\x02\x02\0\x12\x03\t\x02\"\n\x0c\n\
    \x05\x04\x02\x02\0\x04\x12\x03\t\x02\n\n\x0c\n\x05\x04\x02\x02\0\x06\x12\
    \x03\t\x0b\x16\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\t\x17\x1d\n\x0c\n\
    \x05\x04\x02\x02\0\x03\x12\x03\t\x20!\n\n\n\x02\x06\0\x12\x04\x0c\0\x0e\
    \x01\n\n\n\x03\x06\0\x01\x12\x03\x0c\x08\x1f\n\x0b\n\x04\x06\0\x02\0\x12\
    \x03\r\x04J\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\r\x08\x18\n\x0c\n\x05\
    \x06\0\x02\0\x02\x12\x03\r\x19$\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\r/Fb\
    \x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(InitRequest::generated_message_descriptor_data());
            messages.push(SocialEvent::generated_message_descriptor_data());
            messages.push(GetAvatarEventsResponse::generated_message_descriptor_data());
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
