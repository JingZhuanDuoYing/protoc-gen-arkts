declare const $base64$: {
  encode(v: Uint8Array): string;
  decode(v: string): Uint8Array;
};

declare const $type_registry$: {
    get(qualified_name: string): typeof Message;
}


// message
declare abstract class Message {
    static type: string;
    toBinary(): Uint8Array;
    static fromBinary(v: Uint8Array): Message;
}

declare function $wkt_Message(): {
    new (): {
    };
};


// field_mask.ts
declare class $wkt_google_protobuf_FieldMask extends $wkt_Message() {
    paths: string[];
}

// any.ts
declare class $wkt_google_protobuf_Any extends $wkt_Message() {
    type_url: string;
    value: Uint8Array;
}


// duration.ts
declare class $wkt_google_protobuf_Duration extends $wkt_Message() {
    seconds: bigint;
    nanos: number;
}


// timestamp.ts
declare class $wkt_google_protobuf_Timestamp extends $wkt_Message() {
    seconds: bigint;
    nanos: number;
}


// struct.ts
declare enum $wkt_google_protobuf_NullValue {
    NULL_VALUE = 0,
}

declare class $wkt_google_protobuf_Struct extends $wkt_Message() {
    fields: Map<string, $wkt_google_protobuf_Value>;
}

declare class $wkt_google_protobuf_Value extends $wkt_Message() {
    null_value: $wkt_google_protobuf_NullValue;
    number_value: number;
    string_value: string;
    bool_value: boolean;
    struct_value: $wkt_google_protobuf_Struct;
    list_value: $wkt_google_protobuf_ListValue;
}

declare class $wkt_google_protobuf_ListValue extends $wkt_Message() {
    values: $wkt_google_protobuf_Value[];
}


// wrappers.ts
declare function $wkt_Value<WireType>(): {
    new (): {
        value: WireType;
    };
};
declare class $wkt_google_protobuf_DoubleValue extends $wkt_Value<number>() {}
declare class $wkt_google_protobuf_FloatValue extends $wkt_Value<number>() {}
declare class $wkt_google_protobuf_Int64Value extends $wkt_Value<bigint>() {}
declare class $wkt_google_protobuf_UInt64Value extends $wkt_Value<bigint>() {}
declare class $wkt_google_protobuf_Int32Value extends $wkt_Value<number>() {}
declare class $wkt_google_protobuf_UInt32Value extends $wkt_Value<number>() {}
declare class $wkt_google_protobuf_BoolValue extends $wkt_Value<boolean>() {}
declare class $wkt_google_protobuf_StringValue extends $wkt_Value<string>() {}
declare class $wkt_google_protobuf_BytesValue extends $wkt_Value<Uint8Array>() {}