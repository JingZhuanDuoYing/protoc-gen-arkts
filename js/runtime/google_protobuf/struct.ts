import { BinaryReader, BinaryWriter } from "google-protobuf";
import { Message } from "./message";

export class Struct extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}

export class ListValue extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}

export class Value extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}
