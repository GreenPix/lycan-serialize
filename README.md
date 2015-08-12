This library intends to abstract all the serialization / deserialization parts of messages
transiting on the network

It has a slight inconsistency: when serializing, it prefixes the message with its size, but when
reading it it does not read this size first. This on the other hand allows the caller to ensure that
the message has entirely arrived before processing it.
