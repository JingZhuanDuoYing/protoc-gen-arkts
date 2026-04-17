import { BinaryReader, BinaryWriter } from "google-protobuf";
import { Message } from "./message";

export class DoubleValue extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}

export class FloatValue extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}

export class Int64Value extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}

export class UInt64Value extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}

export class Int32Value extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}

export class UInt32Value extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}

export class BoolValue extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}

export class StringValue extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}

export class BytesValue extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}
