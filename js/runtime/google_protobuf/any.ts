import { BinaryReader, BinaryWriter } from "google-protobuf";
import { Message } from "./message";

export class Any extends Message {
  override serializeInternal(bw: BinaryWriter): void {
  }
  override mergeFrom(bytes: Uint8Array): this {
    return this;
  }
}
