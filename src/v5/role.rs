use crate::{
    Error, Header, Result, Status,
    base::{FromParserBase, ParserBase, Storage},
    types::{
        IdentifiedQoS, MqttString, PacketIdentifier, QoS, TopicName, TopicNameError, VarByteInt,
        v5::{
            TopicReference,
            property::{
                ContentType, CorrelationData, MessageExpiryInterval, OnceHelper,
                PayloadFormatIndicator, PropertyType, ResponseTopic, TopicAlias, UserProperty,
            },
        },
    },
    v5::packet::{
        Auth, Connack, Connect, Disconnect, OmissibleReasonCode, Packet, Puback, Pubcomp, Publish,
        Pubrec, Pubrel, SubackReasonCode, UnsubackReasonCode, parse_generic_puback,
    },
};

pub trait Role {
    type PacketType: Into<u8>;

    type Connect;
    type Connack<B, UPS>;
    // type Publish;
    // type Puback;
    // type Pubrec;
    // type Pubrel;
    // type Pubcomp;
    type Subscribe;
    type Suback<B, UPS, RCS>;
    type Unsubscribe;
    type Unsuback<B, UPS, RCS>;
    type Pingreq;
    type Pingresp;
    // type Disconnect;
    // type Auth;

    type PublishSubscriptionIdentifierError: Into<Error>;
    type PublishSubscriptionIdentifier<B: ParserBase>: FromParserBase<B, Error = Self::PublishSubscriptionIdentifierError>;

    type PubackReasonCode<B: ParserBase>: FromParserBase<B> + OmissibleReasonCode
    where
        Error: From<<<Self as Role>::PubackReasonCode<B> as FromParserBase<B>>::Error>;

    type PubrecReasonCode<B: ParserBase>: FromParserBase<B> + OmissibleReasonCode
    where
        Error: From<<<Self as Role>::PubrecReasonCode<B> as FromParserBase<B>>::Error>;

    type DisconnectReasonCode<B: ParserBase>: FromParserBase<B>;
    type DisconnectSessionExpiryIntervalError;
    type DisconnectSessionExpiryInterval<B: ParserBase>: FromParserBase<B, Error = Self::DisconnectSessionExpiryIntervalError>;
    type DisconnectServerReferenceError;
    type DisconnectServerReference<B: ParserBase>: FromParserBase<B, Error = Self::DisconnectServerReferenceError>;

    type AuthReasonCode<B: ParserBase>: FromParserBase<B>;

    fn packet_type(first_byte: u8) -> core::result::Result<Self::PacketType, Error>;
    fn parse<B, UPS, SIS, SRCS, URCS>(
        data: B,
    ) -> crate::Result<Packet<Self, B, UPS, SIS, SRCS, URCS>>
    where
        B: ParserBase,
        UPS: Storage<Element = UserProperty<B>>,
        SIS: Storage<Element = Self::PublishSubscriptionIdentifier<B>>,
        SRCS: Storage<Element = SubackReasonCode>,
        URCS: Storage<Element = UnsubackReasonCode>;

    fn parse_connect<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Connect>;
    fn parse_connack<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Connack<B, UPS>>;
    fn parse_subscribe<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Subscribe>;
    fn parse_suback<
        B: ParserBase,
        UPS: Storage<Element = UserProperty<B>>,
        RCS: Storage<Element = SubackReasonCode>,
    >(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Suback<B, UPS, RCS>>;
    fn parse_unsubscribe<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Unsubscribe>;
    fn parse_unsuback<
        B: ParserBase,
        UPS: Storage<Element = UserProperty<B>>,
        RCS: Storage<Element = UnsubackReasonCode>,
    >(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Unsuback<B, UPS, RCS>>;
    fn parse_pingreq<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Pingreq>;
    fn parse_pingresp<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Self::Pingresp>;

    fn parse_publish<
        B: ParserBase,
        SIS: Storage<Element = Self::PublishSubscriptionIdentifier<B>>,
        UPS: Storage<Element = UserProperty<B>>,
    >(
        header: Header<Self::PacketType>,
        mut data: B,
    ) -> Result<Publish<B, SIS, UPS>> {
        let len: usize = header
            .remaining_len
            .try_into()
            .map_err(|_| Error::PacketTooLarge)?;

        // qos, retain, dup
        let dup = header.flags >> 3 == 1;

        let qos = match header.flags & 0x06 {
            0x00 => QoS::AtMostOnce,
            0x02 => QoS::AtLeastOnce,
            0x04 => QoS::ExactlyOnce,
            _ => return Err(Error::InvalidFixedHeaderFlags),
        };
        let retain = header.flags & 0x01 == 1;

        let mut backing = match data.split_to_checked(len) {
            Some(d) => d,
            None => return Ok(Status::Partial),
        };

        if data.len() > 0 {
            return Err(todo!("more data than in remaining length field"));
        }

        let data = &mut backing;

        let topic_name = MqttString::parse(data)?;
        let topic_name = if topic_name.is_empty() {
            None
        } else {
            Some(TopicName::new(topic_name).map_err(|e| match e {
                TopicNameError::Empty => unreachable!(),
                TopicNameError::Wildcard => Error::InvalidTopicName,
            })?)
        };

        let identified_qos = match qos {
            QoS::AtMostOnce => IdentifiedQoS::AtMostOnce,
            QoS::AtLeastOnce => IdentifiedQoS::AtLeastOnce(PacketIdentifier::parse(data)?),
            QoS::ExactlyOnce => IdentifiedQoS::ExactlyOnce(PacketIdentifier::parse(data)?),
        };

        let properties_length: usize = VarByteInt::parse(data)?
            .try_into()
            .map_err(|_| Error::InvalidPropertyLength)?;
        let mut properties_data = data
            .split_to_checked(properties_length)
            .ok_or(Error::InvalidPropertyLength)?;
        let properties_data = &mut properties_data;

        let mut payload_format_indicator: Option<PayloadFormatIndicator> = None;
        let mut message_expiry_interval: Option<MessageExpiryInterval> = None;
        let mut topic_alias: Option<TopicAlias> = None;
        let mut response_topic: Option<ResponseTopic<B>> = None;
        let mut correlation_data: Option<CorrelationData<B>> = None;
        let mut user_properties = UPS::default();
        let mut subscription_identifiers = SIS::default();
        let mut content_type: Option<ContentType<B>> = None;

        // TODO convert RemainingLengthExceeded to PropertyLengthExceeded here
        while properties_data.len() > 0 {
            let property_type = PropertyType::parse(properties_data)?;

            match property_type {
                PropertyType::PayloadFormatIndicator => {
                    payload_format_indicator.set(properties_data)?;
                }
                PropertyType::MessageExpiryInterval => {
                    message_expiry_interval.set(properties_data)?;
                }
                PropertyType::TopicAlias => {
                    topic_alias.set(properties_data)?;
                }
                PropertyType::ResponseTopic => {
                    response_topic.set(properties_data)?;
                }
                PropertyType::CorrelationData => {
                    correlation_data.set(properties_data)?;
                }
                PropertyType::UserProperty => {
                    let user_property = UserProperty::parse(properties_data)?;
                    if let Some(limit) = UPS::LIMIT
                        && user_properties.len() == limit
                    {
                        // Storage is full
                    } else {
                        user_properties.extend(Some(user_property));
                    }
                }
                PropertyType::SubscriptionIdentifier => {
                    let subscription_identifier =
                        Self::PublishSubscriptionIdentifier::parse(properties_data)
                            .map_err(Into::into)?;
                    if let Some(limit) = SIS::LIMIT
                        && user_properties.len() == limit
                    {
                        // Storage is full
                    } else {
                        subscription_identifiers.extend(Some(subscription_identifier));
                    }
                }
                PropertyType::ContentType => {
                    content_type.set(properties_data)?;
                }
                p => return Err(Error::InvalidPropertyType),
            }
        }

        let topic = match (topic_name, topic_alias) {
            (None, None) => return Err(Error::InvalidTopicName),
            (None, Some(alias)) => TopicReference::Alias(alias.into()),
            (Some(name), None) => TopicReference::Name(name),
            (Some(name), Some(alias)) => TopicReference::Mapping(name, alias.into()),
        };

        let message = backing;

        Ok(Status::Complete(Publish {
            dup,
            identified_qos,
            retain,
            topic,
            payload_format_indicator,
            message_expiry_interval,
            response_topic,
            correlation_data,
            user_properties,
            subscription_identifiers,
            content_type,
            message,
        }))
    }
    fn parse_puback<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Puback<B, Self::PubackReasonCode<B>, UPS>> {
        parse_generic_puback(header, data)
    }
    fn parse_pubrec<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Pubrec<B, Self::PubrecReasonCode<B>, UPS>> {
        parse_generic_puback(header, data)
    }
    fn parse_pubrel<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Pubrel<B, UPS>> {
        parse_generic_puback(header, data)
    }
    fn parse_pubcomp<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Pubcomp<B, UPS>> {
        parse_generic_puback(header, data)
    }
    fn parse_disconnect<B: ParserBase, UPS: Storage<Element = UserProperty<B>>>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<
        Disconnect<
            B,
            Self::DisconnectReasonCode<B>,
            Self::DisconnectSessionExpiryInterval<B>,
            Self::DisconnectServerReference<B>,
            UPS,
        >,
    > {
        todo!()
    }
    fn parse_auth<B: ParserBase>(
        header: Header<Self::PacketType>,
        data: B,
    ) -> Result<Auth<Self::AuthReasonCode<B>>> {
        todo!()
    }
}
